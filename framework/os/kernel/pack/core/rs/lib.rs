//! 📦 `pack_core` — foundation of the `pack` binary document container family: stable ids,
//! the shared error type, corruption-hardening resource limits, LEB128/zigzag varints, a
//! bounds-checked byte reader/writer, an in-crate CRC-32C table, the `PackSource`/`PackSink`
//! random-access traits, and the `CompressionCodec` trait. Every other `pack_*` crate path-deps
//! on this one; nothing here may allocate before validating a length against `PackLimits`.

//#region 🔖Ids
/// @emoji 🔑 A blake3 content hash (32 bytes), formatted as lowercase hex via `Display`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentHash(pub [u8; 32]);

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContentHash({self})")
    }
}

/// @emoji 🧩 Identity of a chunk within a pack file's chunk table.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkId(pub u32);

/// @emoji 🏷️ The one-byte kind tag stamped on every segment; see `KIND_*` constants below.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SegmentKind(pub u8);

/// @emoji 🗜️ The one-byte compression codec identifier stamped on segment flags.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CodecId(pub u8);

/// @emoji 📏 An absolute byte offset paired with a length, used for spans into a pack file.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ByteRange {
    pub offset: u64,
    pub len: u64,
}
//#endregion 🔖Ids

//#region 🔖SegmentKinds
/// @emoji 🔚 Marks the end of the segment stream.
pub const KIND_END: u8 = 0x00;
/// @emoji 🗺️ The manifest segment: spans + counts describing the rest of the file.
pub const KIND_MANIFEST: u8 = 0x01;
/// @emoji 🧬 An embedded schema description segment.
pub const KIND_SCHEMA: u8 = 0x02;
/// @emoji 🔤 The interned string table segment.
pub const KIND_SYMBOLS: u8 = 0x03;
/// @emoji 📄 The encoded document body segment.
pub const KIND_DOCUMENT: u8 = 0x04;
/// @emoji 🧱 One chunk of blob data, framed like any other segment.
pub const KIND_CHUNK: u8 = 0x05;
/// @emoji 📇 The chunk table segment: offset/len/crc/hash per chunk.
pub const KIND_CHUNK_TABLE: u8 = 0x06;
/// @emoji 📸 A snapshot segment.
pub const KIND_SNAPSHOT: u8 = 0x07;
/// @emoji 🔎 A field index segment.
pub const KIND_FIELD_INDEX: u8 = 0x08;
/// @emoji ⬜ Padding, skipped on read.
pub const KIND_PADDING: u8 = 0x7F;
//#endregion 🔖SegmentKinds

//#region 🔖Errors
/// @emoji 🚨 The one error type every `pack_*` public fn returns; never leaks `std::io::Error`.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum PackError {
    #[error("bad magic")]
    BadMagic,
    #[error("unsupported version {major}.{minor}")]
    UnsupportedVersion { major: u16, minor: u16 },
    #[error("unknown required feature bits {0:#x}")]
    UnknownRequiredFlags(u32),
    #[error("truncated at offset {0}")]
    Truncated(u64),
    #[error("checksum mismatch in {segment} at offset {offset}")]
    ChecksumMismatch { segment: &'static str, offset: u64 },
    #[error("content hash mismatch")]
    ContentHashMismatch,
    #[error("limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("malformed {what} at offset {offset}: {detail}")]
    Malformed { what: &'static str, offset: u64, detail: String },
    #[error("non-canonical encoding: {0}")]
    NonCanonical(&'static str),
    #[error("unsupported codec {0}")]
    UnsupportedCodec(u8),
    #[error("schema error: {0}")]
    Schema(String),
    #[error("io error: {0}")]
    Io(String),
}
//#endregion 🔖Errors

//#region 🔖Limits
/// @emoji 🛡️ Corruption-hardening ceilings every decoder must validate against before
/// allocating — load-bearing for every other `pack_*` crate's fuzz/corruption tests.
#[derive(Clone, Debug)]
pub struct PackLimits {
    pub max_file_len: u64,
    pub max_segment_len: u64,
    pub max_symbols: u32,
    pub max_depth: u16,
    pub max_items: u64,
    pub max_total_alloc: u64,
}

impl Default for PackLimits {
    fn default() -> Self {
        Self {
            max_file_len: 16 * 1024 * 1024 * 1024,
            max_segment_len: 256 * 1024 * 1024,
            max_symbols: 1_000_000,
            max_depth: 64,
            max_items: 64_000_000,
            max_total_alloc: 4 * 1024 * 1024 * 1024,
        }
    }
}
//#endregion 🔖Limits

//#region 🔖Varint
/// @emoji ➡️ Zigzag-encodes an `i64` into the `u64` domain: small magnitudes stay small
/// regardless of sign. See <https://protobuf.dev/__KEEP_pluginming__-guides/encoding/#signed-ints>.
fn zigzag_encode(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}

/// @emoji ⬅️ Inverse of `zigzag_encode`.
fn zigzag_decode(value: u64) -> i64 {
    ((value >> 1) as i64) ^ -((value & 1) as i64)
}

/// @emoji ✏️ Writes `value` as an unsigned LEB128 varint (minimal length, max 10 bytes).
pub fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
    let mut remaining = value;
    loop {
        let byte = (remaining & 0x7F) as u8;
        remaining >>= 7;
        if remaining == 0 {
            out.push(byte);
            break;
        }
        out.push(byte | 0x80);
    }
}

/// @emoji 📖 Reads an unsigned LEB128 varint starting at `*pos`, advancing `*pos` past it.
/// Errors `Malformed` on a >10-byte (overlong) encoding, `Truncated` on running out of bytes.
pub fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, PackError> {
    let start = *pos;
    let mut result: u64 = 0;
    for i in 0..10usize {
        let idx = *pos;
        if idx >= bytes.len() {
            return Err(PackError::Truncated(idx as u64));
        }
        let byte = bytes[idx];
        *pos += 1;
        let more = byte & 0x80 != 0;
        let payload = (byte & 0x7F) as u64;
        if i == 9 && (more || payload > 1) {
            return Err(PackError::Malformed {
                what: "varint",
                offset: start as u64,
                detail: "overlong varint (exceeds 10 bytes / 64 bits)".to_string(),
            });
        }
        result |= payload << (i as u32 * 7);
        if !more {
            return Ok(result);
        }
    }
    Err(PackError::Malformed {
        what: "varint",
        offset: start as u64,
        detail: "overlong varint (exceeds 10 bytes)".to_string(),
    })
}

/// @emoji ✏️ Writes `value` as a zigzag-encoded signed varint.
pub fn write_varint_i64(out: &mut Vec<u8>, value: i64) {
    write_varint_u64(out, zigzag_encode(value));
}

/// @emoji 📖 Reads a zigzag-encoded signed varint starting at `*pos`.
pub fn read_varint_i64(bytes: &[u8], pos: &mut usize) -> Result<i64, PackError> {
    let raw = read_varint_u64(bytes, pos)?;
    Ok(zigzag_decode(raw))
}

/// @emoji ✅ True iff `bytes` is exactly one minimal-length varint (decoding then re-encoding
/// reproduces the input byte-for-byte, with nothing left over).
pub fn is_minimal_varint(bytes: &[u8]) -> bool {
    let mut pos = 0usize;
    let value = match read_varint_u64(bytes, &mut pos) {
        Ok(value) => value,
        Err(_) => return false,
    };
    if pos != bytes.len() {
        return false;
    }
    let mut reencoded = Vec::new();
    write_varint_u64(&mut reencoded, value);
    reencoded == bytes
}
//#endregion 🔖Varint

//#region 🔖Bytes
/// @emoji 👓 A bounds-checked cursor over a borrowed byte slice — every read either succeeds
/// or returns `PackError` (`Truncated`), it never panics or reads out of bounds.
pub struct ByteReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn read_u8(&mut self) -> Result<u8, PackError> {
        Ok(self.read_bytes(1)?[0])
    }

    pub fn read_u16_le(&mut self) -> Result<u16, PackError> {
        let bytes = self.read_bytes(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_u32_le(&mut self) -> Result<u32, PackError> {
        let bytes = self.read_bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_u64_le(&mut self) -> Result<u64, PackError> {
        let bytes = self.read_bytes(8)?;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(u64::from_le_bytes(array))
    }

    pub fn read_f64_le(&mut self) -> Result<f64, PackError> {
        let bytes = self.read_bytes(8)?;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(f64::from_le_bytes(array))
    }

    pub fn read_varint_u64(&mut self) -> Result<u64, PackError> {
        read_varint_u64(self.bytes, &mut self.pos)
    }

    pub fn read_varint_i64(&mut self) -> Result<i64, PackError> {
        read_varint_i64(self.bytes, &mut self.pos)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], PackError> {
        if len > self.remaining() {
            return Err(PackError::Truncated(self.pos as u64));
        }
        let slice = &self.bytes[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    pub fn read_array32(&mut self) -> Result<[u8; 32], PackError> {
        let slice = self.read_bytes(32)?;
        let mut array = [0u8; 32];
        array.copy_from_slice(slice);
        Ok(array)
    }
}

/// @emoji ✍️ An append-only byte buffer with typed little-endian and varint writers.
pub struct ByteWriter {
    buf: Vec<u8>,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    pub fn write_u16_le(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u32_le(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u64_le(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_f64_le(&mut self, v: f64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_varint_u64(&mut self, v: u64) {
        write_varint_u64(&mut self.buf, v);
    }

    pub fn write_varint_i64(&mut self, v: i64) {
        write_varint_i64(&mut self.buf, v);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }
}

impl Default for ByteWriter {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖Bytes

//#region 🔖Crc
/// @emoji 🌀 CRC-32C (Castagnoli) polynomial, reflected form, as used by iSCSI/ext4/pack.
const CRC32C_POLY: u32 = 0x82F6_3B78;

/// @emoji 📐 Builds the 256-entry CRC-32C lookup table at compile time — no runtime init, no
/// dependency on the `crc`/`crc32c` crates.
const fn build_crc32c_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ CRC32C_POLY } else { crc >> 1 };
            bit += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

const CRC32C_TABLE: [u32; 256] = build_crc32c_table();

/// @emoji 🧮 Computes the CRC-32C (Castagnoli) checksum of `bytes`.
pub fn crc32c(bytes: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in bytes {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32C_TABLE[index];
    }
    crc ^ 0xFFFF_FFFF
}
//#endregion 🔖Crc

//#region 🔖Source
/// @emoji 📥 Random-access read source a pack file is decoded from — implementable over an
/// in-memory slice, a file (see `pack_io`), or (via `pack_async`) a network range-fetcher.
pub trait PackSource {
    fn len(&self) -> u64;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError>;

    fn read_exact_at(&self, offset: u64, buf: &mut [u8]) -> Result<(), PackError> {
        let mut filled = 0usize;
        while filled < buf.len() {
            let read = self.read_at(offset + filled as u64, &mut buf[filled..])?;
            if read == 0 {
                return Err(PackError::Truncated(offset + filled as u64));
            }
            filled += read;
        }
        Ok(())
    }
}

impl PackSource for &[u8] {
    fn len(&self) -> u64 {
        (*self).len() as u64
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError> {
        let slice: &[u8] = self;
        let total = slice.len() as u64;
        if offset > total {
            return Err(PackError::Truncated(offset));
        }
        let available = &slice[offset as usize..];
        let n = available.len().min(buf.len());
        buf[..n].copy_from_slice(&available[..n]);
        Ok(n)
    }
}

impl PackSource for Vec<u8> {
    fn len(&self) -> u64 {
        self.as_slice().len() as u64
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError> {
        self.as_slice().read_at(offset, buf)
    }
}

/// @emoji 📤 Append-only write sink a pack file is encoded into — implementable over a
/// `Vec<u8>`, a file (see `pack_io`), or any other ordered byte destination.
pub trait PackSink {
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), PackError>;

    fn position(&self) -> u64;

    fn flush(&mut self) -> Result<(), PackError> {
        Ok(())
    }
}

impl PackSink for Vec<u8> {
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), PackError> {
        self.extend_from_slice(bytes);
        Ok(())
    }

    fn position(&self) -> u64 {
        self.len() as u64
    }
}
//#endregion 🔖Source

//#region 🔖Codec
/// @emoji 🗜️ A pluggable segment payload compressor/decompressor, identified by `CodecId`.
/// `decompress` must validate `raw_len` against `limit` BEFORE allocating the output buffer.
pub trait CompressionCodec {
    fn id(&self) -> CodecId;

    fn compress(&self, raw: &[u8]) -> Result<Vec<u8>, PackError>;

    fn decompress(&self, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError>;
}

/// @emoji 🚫 The identity codec (`CodecId(0)`) — no compression, used as the default and as
/// the fallback when a segment's compressed flag bit is unset.
pub struct NoCompression;

impl CompressionCodec for NoCompression {
    fn id(&self) -> CodecId {
        CodecId(0)
    }

    fn compress(&self, raw: &[u8]) -> Result<Vec<u8>, PackError> {
        Ok(raw.to_vec())
    }

    fn decompress(&self, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError> {
        if raw_len > limit {
            return Err(PackError::LimitExceeded("NoCompression::decompress raw_len exceeds limit"));
        }
        if stored.len() as u64 != raw_len {
            return Err(PackError::Malformed {
                what: "codec",
                offset: 0,
                detail: "identity codec stored length does not match raw_len".to_string(),
            });
        }
        Ok(stored.to_vec())
    }
}
//#endregion 🔖Codec

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖Ids
    #[test]
    fn content_hash_display_is_lowercase_hex() {
        let mut bytes = [0u8; 32];
        bytes[0] = 0xAB;
        bytes[31] = 0x0F;
        let hash = ContentHash(bytes);
        let text = hash.to_string();
        assert_eq!(text.len(), 64);
        assert!(text.starts_with("ab"));
        assert!(text.ends_with("0f"));
        assert_eq!(text, text.to_lowercase());
    }

    #[test]
    fn segment_kind_constants_match_contract() {
        assert_eq!(KIND_END, 0x00);
        assert_eq!(KIND_MANIFEST, 0x01);
        assert_eq!(KIND_SCHEMA, 0x02);
        assert_eq!(KIND_SYMBOLS, 0x03);
        assert_eq!(KIND_DOCUMENT, 0x04);
        assert_eq!(KIND_CHUNK, 0x05);
        assert_eq!(KIND_CHUNK_TABLE, 0x06);
        assert_eq!(KIND_SNAPSHOT, 0x07);
        assert_eq!(KIND_FIELD_INDEX, 0x08);
        assert_eq!(KIND_PADDING, 0x7F);
    }
    //#endregion 🔖Ids

    //#region 🔖Limits
    #[test]
    fn pack_limits_default_matches_contract() {
        let limits = PackLimits::default();
        assert_eq!(limits.max_file_len, 16 * 1024 * 1024 * 1024);
        assert_eq!(limits.max_segment_len, 256 * 1024 * 1024);
        assert_eq!(limits.max_symbols, 1_000_000);
        assert_eq!(limits.max_depth, 64);
        assert_eq!(limits.max_items, 64_000_000);
        assert_eq!(limits.max_total_alloc, 4 * 1024 * 1024 * 1024);
    }
    //#endregion 🔖Limits

    //#region 🔖Varint
    #[test]
    fn varint_u64_round_trips_boundary_values() {
        let values: &[u64] = &[
            0,
            1,
            0x7F,
            0x80,
            0x3FFF,
            0x4000,
            0x1F_FFFF,
            0x20_0000,
            u32::MAX as u64,
            u32::MAX as u64 + 1,
            u64::MAX / 2,
            u64::MAX - 1,
            u64::MAX,
        ];
        for &value in values {
            let mut out = Vec::new();
            write_varint_u64(&mut out, value);
            assert!(out.len() <= 10, "varint for {value} exceeded 10 bytes");
            let mut pos = 0usize;
            let decoded = read_varint_u64(&out, &mut pos).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(pos, out.len());
            assert!(is_minimal_varint(&out), "encoding of {value} should be minimal");
        }
    }

    #[test]
    fn varint_u64_max_value_uses_ten_bytes() {
        let mut out = Vec::new();
        write_varint_u64(&mut out, u64::MAX);
        assert_eq!(out.len(), 10);
    }

    #[test]
    fn varint_i64_round_trips_boundary_values() {
        let values: &[i64] = &[0, 1, -1, 63, -64, 64, -65, i32::MIN as i64, i32::MAX as i64, i64::MIN, i64::MAX];
        for &value in values {
            let mut out = Vec::new();
            write_varint_i64(&mut out, value);
            let mut pos = 0usize;
            let decoded = read_varint_i64(&out, &mut pos).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(pos, out.len());
        }
    }

    #[test]
    fn varint_multi_byte_sequence_reads_each_value_in_order() {
        let mut buf = Vec::new();
        write_varint_u64(&mut buf, 300);
        write_varint_u64(&mut buf, 1);
        write_varint_u64(&mut buf, 0x4000);
        let mut pos = 0usize;
        assert_eq!(read_varint_u64(&buf, &mut pos).unwrap(), 300);
        assert_eq!(read_varint_u64(&buf, &mut pos).unwrap(), 1);
        assert_eq!(read_varint_u64(&buf, &mut pos).unwrap(), 0x4000);
        assert_eq!(pos, buf.len());
    }

    #[test]
    fn varint_read_truncated_input_errors_never_panics() {
        let mut pos = 0usize;
        assert_eq!(read_varint_u64(&[], &mut pos), Err(PackError::Truncated(0)));
        pos = 0;
        assert_eq!(read_varint_u64(&[0x80], &mut pos), Err(PackError::Truncated(1)));
        pos = 0;
        assert_eq!(read_varint_u64(&[0x80, 0x80, 0x80], &mut pos), Err(PackError::Truncated(3)));
    }

    #[test]
    fn varint_read_overlong_eleven_bytes_is_malformed() {
        let overlong = [0x80u8; 11];
        let mut pos = 0usize;
        let result = read_varint_u64(&overlong, &mut pos);
        assert!(matches!(result, Err(PackError::Malformed { .. })));
    }

    #[test]
    fn varint_read_tenth_byte_with_extra_bits_is_malformed() {
        let mut bytes = vec![0x80u8; 9];
        bytes.push(0x02);
        let mut pos = 0usize;
        let result = read_varint_u64(&bytes, &mut pos);
        assert!(matches!(result, Err(PackError::Malformed { .. })));
    }

    #[test]
    fn is_minimal_varint_rejects_non_minimal_encoding_of_zero() {
        assert!(is_minimal_varint(&[0x00]));
        assert!(!is_minimal_varint(&[0x80, 0x00]));
    }

    #[test]
    fn is_minimal_varint_rejects_trailing_garbage() {
        let mut out = Vec::new();
        write_varint_u64(&mut out, 5);
        out.push(0xFF);
        assert!(!is_minimal_varint(&out));
    }
    //#endregion 🔖Varint

    //#region 🔖Bytes
    #[test]
    fn byte_reader_writer_round_trip_all_types() {
        let mut writer = ByteWriter::new();
        writer.write_u8(0x42);
        writer.write_u16_le(0x1234);
        writer.write_u32_le(0xDEAD_BEEF);
        writer.write_u64_le(0x0123_4567_89AB_CDEF);
        writer.write_f64_le(std::f64::consts::PI);
        writer.write_varint_u64(300);
        writer.write_varint_i64(-42);
        writer.write_bytes(&[1, 2, 3, 4]);
        let array32: [u8; 32] = [7u8; 32];
        writer.write_bytes(&array32);
        let bytes = writer.into_bytes();

        let mut reader = ByteReader::new(&bytes);
        assert_eq!(reader.position(), 0);
        assert_eq!(reader.read_u8().unwrap(), 0x42);
        assert_eq!(reader.read_u16_le().unwrap(), 0x1234);
        assert_eq!(reader.read_u32_le().unwrap(), 0xDEAD_BEEF);
        assert_eq!(reader.read_u64_le().unwrap(), 0x0123_4567_89AB_CDEF);
        assert_eq!(reader.read_f64_le().unwrap(), std::f64::consts::PI);
        assert_eq!(reader.read_varint_u64().unwrap(), 300);
        assert_eq!(reader.read_varint_i64().unwrap(), -42);
        assert_eq!(reader.read_bytes(4).unwrap(), &[1, 2, 3, 4]);
        assert_eq!(reader.read_array32().unwrap(), array32);
        assert_eq!(reader.remaining(), 0);
        assert_eq!(reader.position(), bytes.len());
    }

    #[test]
    fn byte_reader_bounds_checked_reads_never_panic_on_truncated_input() {
        let bytes = [1u8, 2, 3];
        let mut reader = ByteReader::new(&bytes);
        assert_eq!(reader.read_u32_le(), Err(PackError::Truncated(0)));
        assert_eq!(reader.position(), 0);
        assert!(reader.read_bytes(1).is_ok());
        assert_eq!(reader.read_u64_le(), Err(PackError::Truncated(1)));
        assert_eq!(reader.read_array32(), Err(PackError::Truncated(1)));
        let empty: [u8; 0] = [];
        let mut empty_reader = ByteReader::new(&empty);
        assert_eq!(empty_reader.read_u8(), Err(PackError::Truncated(0)));
        assert_eq!(empty_reader.read_bytes(0).unwrap(), &empty[..]);
    }
    //#endregion 🔖Bytes

    //#region 🔖Crc
    #[test]
    fn crc32c_matches_known_test_vector() {
        assert_eq!(crc32c(b"123456789"), 0xE306_9283);
    }

    #[test]
    fn crc32c_empty_input_is_zero() {
        assert_eq!(crc32c(b""), 0);
    }

    #[test]
    fn crc32c_differs_for_different_inputs() {
        assert_ne!(crc32c(b"abc"), crc32c(b"abd"));
    }
    //#endregion 🔖Crc

    //#region 🔖Source
    #[test]
    fn pack_source_over_slice_reads_at_offset() {
        let data: &[u8] = b"hello world";
        let mut buf = [0u8; 5];
        let n = data.read_at(6, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"world");
        assert_eq!(PackSource::len(&data), 11);
        assert!(!data.is_empty());
    }

    #[test]
    fn pack_source_over_slice_short_read_at_end_never_panics() {
        let data: &[u8] = b"hi";
        let mut buf = [0u8; 10];
        let n = data.read_at(0, &mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf[..2], b"hi");
    }

    #[test]
    fn pack_source_read_at_offset_past_end_errors_never_panics() {
        let data: &[u8] = b"hi";
        let mut buf = [0u8; 4];
        let result = data.read_at(100, &mut buf);
        assert_eq!(result, Err(PackError::Truncated(100)));
    }

    #[test]
    fn pack_source_read_exact_at_errors_on_truncated_input() {
        let data: &[u8] = b"hi";
        let mut buf = [0u8; 5];
        let result = data.read_exact_at(0, &mut buf);
        assert!(matches!(result, Err(PackError::Truncated(_))));
    }

    #[test]
    fn pack_source_over_vec_matches_slice_behavior() {
        let data: Vec<u8> = b"hello world".to_vec();
        let mut buf = [0u8; 5];
        let n = data.read_at(0, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");
        assert_eq!(PackSource::len(&data), 11);
    }

    #[test]
    fn pack_sink_over_vec_appends_and_tracks_position() {
        let mut sink: Vec<u8> = Vec::new();
        assert_eq!(sink.position(), 0);
        sink.write_all(b"abc").unwrap();
        assert_eq!(sink.position(), 3);
        sink.write_all(b"def").unwrap();
        assert_eq!(sink.position(), 6);
        assert_eq!(sink.as_slice(), b"abcdef");
        assert!(sink.flush().is_ok());
    }
    //#endregion 🔖Source

    //#region 🔖Codec
    #[test]
    fn no_compression_round_trips_identity() {
        let codec = NoCompression;
        assert_eq!(codec.id(), CodecId(0));
        let raw = b"the quick brown fox";
        let compressed = codec.compress(raw).unwrap();
        assert_eq!(compressed, raw);
        let decompressed = codec.decompress(&compressed, raw.len() as u64, 1_000_000).unwrap();
        assert_eq!(decompressed, raw);
    }

    #[test]
    fn no_compression_decompress_rejects_raw_len_over_limit_before_allocating() {
        let codec = NoCompression;
        let stored = vec![0u8; 16];
        let result = codec.decompress(&stored, 1_000_000_000, 1_000);
        assert!(matches!(result, Err(PackError::LimitExceeded(_))));
    }

    #[test]
    fn no_compression_decompress_rejects_stored_len_mismatch() {
        let codec = NoCompression;
        let stored = vec![0u8; 4];
        let result = codec.decompress(&stored, 5, 1_000);
        assert!(matches!(result, Err(PackError::Malformed { .. })));
    }
    //#endregion 🔖Codec
}
//#endregion 🧪Tests
