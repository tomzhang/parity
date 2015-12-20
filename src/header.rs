use util::hash::*;
use util::bytes::*;
use util::uint::*;
use util::rlp::*;

pub static ZERO_ADDRESS: Address = Address([0x00; 20]);
pub static ZERO_H256: H256 = H256([0x00; 32]);
pub static ZERO_LOGBLOOM: LogBloom = H2048([0x00; 256]);

pub type LogBloom = H2048;

#[derive(Debug)]
pub struct Header {
	parent_hash: H256,
	timestamp: U256,
	number: U256,
	author: Address,

	transactions_root: H256,
	uncles_hash: H256,
	extra_data: Bytes,

	state_root: H256,
	receipts_root: H256,
	log_bloom: LogBloom,
	gas_used: U256,
	gas_limit: U256,

	difficulty: U256,
	seal: Vec<Bytes>,
}

impl Header {
	pub fn new() -> Header {
		Header {
			parent_hash: ZERO_H256.clone(),
			timestamp: BAD_U256.clone(),
			number: ZERO_U256.clone(),
			author: ZERO_ADDRESS.clone(),

			transactions_root: ZERO_H256.clone(),
			uncles_hash: ZERO_H256.clone(),
			extra_data: vec![],

			state_root: ZERO_H256.clone(),
			receipts_root: ZERO_H256.clone(),
			log_bloom: ZERO_LOGBLOOM.clone(),
			gas_used: ZERO_U256.clone(),
			gas_limit: ZERO_U256.clone(),

			difficulty: ZERO_U256.clone(),
			seal: vec![],
		}
	}
}

impl Decodable for Header {
	fn decode<D>(decoder: &D) -> Result<Self, DecoderError> where D: Decoder {
		let d = try!(decoder.as_list());

		let mut blockheader = Header {
			parent_hash: try!(Decodable::decode(&d[0])),
			uncles_hash: try!(Decodable::decode(&d[1])),
			author: try!(Decodable::decode(&d[2])),
			state_root: try!(Decodable::decode(&d[3])),
			transactions_root: try!(Decodable::decode(&d[4])),
			receipts_root: try!(Decodable::decode(&d[5])),
			log_bloom: try!(Decodable::decode(&d[6])),
			difficulty: try!(Decodable::decode(&d[7])),
			number: try!(Decodable::decode(&d[8])),
			gas_limit: try!(Decodable::decode(&d[9])),
			gas_used: try!(Decodable::decode(&d[10])),
			timestamp: try!(Decodable::decode(&d[11])),
			extra_data: try!(Decodable::decode(&d[12])),
			seal: vec![],
		};

		for i in 13..d.len() {
			blockheader.seal.push(try!(Decodable::decode(&d[i])));
		}

		Ok(blockheader)
	}
}

impl Encodable for Header {
	fn encode<E>(&self, encoder: &mut E) where E: Encoder {
		encoder.emit_list(| e | {
			self.parent_hash.encode(e);
			self.uncles_hash.encode(e);
			self.author.encode(e);
			self.state_root.encode(e);
			self.transactions_root.encode(e);
			self.receipts_root.encode(e);
			self.log_bloom.encode(e);
			self.difficulty.encode(e);
			self.number.encode(e);
			self.gas_limit.encode(e);
			self.gas_used.encode(e);
			self.timestamp.encode(e);
			self.extra_data.encode(e);
		
			for b in self.seal.iter() {
				b.encode(e);
			}
		})
	}
}

#[cfg(test)]
mod tests {
}