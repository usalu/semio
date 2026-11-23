use smallvec::SmallVec;

pub fn digits(value: u64) -> SmallVec<[u8; 24]> {
    let mut buffer = itoa::Buffer::new();
    buffer.format(value).bytes().collect()
}

pub fn checksum(value: u64) -> u64 {
    digits(value).iter().fold(0u64, |sum, byte| sum * 37 + u64::from(*byte))
}
