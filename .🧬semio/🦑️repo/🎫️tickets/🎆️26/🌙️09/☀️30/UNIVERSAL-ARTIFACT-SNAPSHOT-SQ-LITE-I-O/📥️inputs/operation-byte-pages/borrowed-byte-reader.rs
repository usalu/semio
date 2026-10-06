#[derive(Clone, Copy)]
enum ByteStorage<'a> { Slice(&'a [u8]), Paged(&'a crate::mutation::bytes::OwnedOperationBytes) }

/// 📏️ A checked finite range retains the exact borrowed source owner.
#[derive(Clone, Copy)]
pub struct ByteSpan<'a> { source: ByteStorage<'a>, offset: usize, length: usize }
impl<'a> ByteSpan<'a> {
    pub fn from_slice(bytes: &'a [u8]) -> Self { Self { source: ByteStorage::Slice(bytes), offset: 0, length: bytes.len() } }
    pub fn from_source(source: &'a crate::mutation::bytes::OwnedOperationBytes) -> Self { Self { source: ByteStorage::Paged(source), offset: 0, length: source.len() } }
    pub fn len(self) -> usize { self.length }
    pub fn is_empty(self) -> bool { self.length == 0 }
    pub fn get(self, offset: usize) -> Option<&'a u8> {
        if offset >= self.length { return None; }
        match self.source { ByteStorage::Slice(bytes) => bytes.get(self.offset + offset), ByteStorage::Paged(source) => source.byte_ref(self.offset + offset) }
    }
    pub fn slice(self, offset: usize, length: usize) -> Result<Self, PackRefusal> {
        if offset.checked_add(length).is_none_or(|end| end > self.length) { return Err(PackRefusal::Truncated(offset as u64)); }
        Ok(Self { source: self.source, offset: self.offset + offset, length })
    }
    pub fn contiguous(self) -> Option<&'a [u8]> { match self.source { ByteStorage::Slice(bytes) => Some(&bytes[self.offset..self.offset + self.length]), ByteStorage::Paged(_) => None } }
    pub fn iter(self) -> impl Iterator<Item = u8> + ExactSizeIterator + 'a { (0..self.length).map(move |index| *self.get(index).expect("checked borrowed operation byte range")) }
}
impl std::ops::Index<usize> for ByteSpan<'_> {
    type Output = u8;
    fn index(&self, index: usize) -> &u8 { self.get(index).expect("checked borrowed operation byte offset") }
}

/// 👓️ One bounds-checked cursor borrows either a slice or the genuine paged operation owner.
pub struct ByteReader<'a> { bytes: ByteSpan<'a>, pos: usize }
impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self { Self::from_span(ByteSpan::from_slice(bytes)) }
    pub fn from_source(bytes: &'a crate::mutation::bytes::OwnedOperationBytes) -> Self { Self::from_span(ByteSpan::from_source(bytes)) }
    pub fn from_span(bytes: ByteSpan<'a>) -> Self { Self { bytes, pos: 0 } }
    pub fn fork(&self) -> Self { Self { bytes: self.bytes, pos: self.pos } }
    pub fn remaining(&self) -> usize { self.bytes.len() - self.pos }
    pub fn position(&self) -> usize { self.pos }
    pub fn read_span(&mut self, length: usize) -> Result<ByteSpan<'a>, PackRefusal> {
        if length > self.remaining() { return Err(PackRefusal::Truncated(self.pos as u64)); }
        let span = self.bytes.slice(self.pos, length)?;
        self.pos += length;
        Ok(span)
    }
    pub fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], PackRefusal> {
        if length > self.remaining() { return Err(PackRefusal::Truncated(self.pos as u64)); }
        let span = self.bytes.slice(self.pos, length)?;
        let bytes = span.contiguous().ok_or(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::UnsupportedOwner, what: "operation byte source", offset: self.pos as u64, detail: "contiguous read requires a slice source; use the borrowed byte range" })?;
        self.pos += length;
        Ok(bytes)
    }
    pub fn read_u8(&mut self) -> Result<u8, PackRefusal> { Ok(self.read_span(1)?[0]) }
    fn array<const N: usize>(&mut self) -> Result<[u8; N], PackRefusal> {
        let span = self.read_span(N)?;
        let mut bytes = [0; N];
        for (output, byte) in bytes.iter_mut().zip(span.iter()) { *output = byte; }
        Ok(bytes)
    }
    pub fn read_u16_le(&mut self) -> Result<u16, PackRefusal> { Ok(u16::from_le_bytes(self.array()?)) }
    pub fn read_u32_le(&mut self) -> Result<u32, PackRefusal> { Ok(u32::from_le_bytes(self.array()?)) }
    pub fn read_u64_le(&mut self) -> Result<u64, PackRefusal> { Ok(u64::from_le_bytes(self.array()?)) }
    pub fn read_f64_le(&mut self) -> Result<f64, PackRefusal> { Ok(f64::from_le_bytes(self.array()?)) }
    pub fn read_array32(&mut self) -> Result<[u8; 32], PackRefusal> { self.array() }
    pub fn read_varint_u64(&mut self) -> Result<u64, PackRefusal> {
        let start = self.pos;
        let mut result = 0;
        for index in 0..10 {
            let byte = self.read_u8()?;
            let payload = u64::from(byte & 127);
            if index == 9 && (byte & 128 != 0 || payload > 1) { return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "varint", offset: start as u64, detail: "overlong varint (exceeds 10 bytes / 64 bits)".into() }); }
            result |= payload << (index * 7);
            if byte & 128 == 0 { return Ok(result); }
        }
        Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "varint", offset: start as u64, detail: "overlong varint (exceeds 10 bytes)".into() })
    }
    pub fn read_varint_i64(&mut self) -> Result<i64, PackRefusal> { self.read_varint_u64().map(zigzag_decode) }
}
