use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub id: u64,
    pub sum: u64,
}

pub fn record(id: u64) -> Record {
    Record { id, sum: p_leaf::checksum(id) }
}
