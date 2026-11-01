#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeflateError { BadBlockType, BadStoredLength, BadHuffmanCode, BadDistance, UnexpectedEnd, OutputLimitExceeded }
pub fn inflate(stored: &[u8], max: usize) -> Result<Vec<u8>, DeflateError> {
    miniz_oxide::inflate::decompress_to_vec_with_limit(stored, max).map_err(|_| DeflateError::UnexpectedEnd)
}
