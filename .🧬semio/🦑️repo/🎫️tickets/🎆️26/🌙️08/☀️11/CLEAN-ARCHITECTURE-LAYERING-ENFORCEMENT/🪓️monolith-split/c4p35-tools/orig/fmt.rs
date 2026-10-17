//! 📦️ `pack_format` — the `SPK` binary document container: exact byte-level header/footer/
//! segment/manifest/symbols/chunk-table layout, a `PackWriter`/`PackFile` pair for building and
//! random-access reading packs of any size, three verification levels trading speed for
//! integrity guarantees, forward-scan recovery for footer-less/truncated files, and an optional
//! deflate codec. Every length is validated against `crate::PackLimits` before allocation.
//!
//! Layout notes: see the `## pack_format` section of the wave-0 contract at
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md`. One deliberate
//! deviation is documented in `🔖️Footer` below: the contract's prose arithmetic for footer size
//! (`"= 80 bytes exactly"`) undercounts its own `footer_crc32` field by 4 bytes; this crate
//! implements the mathematically self-consistent 84-byte footer and exports `FOOTER_SIZE` so
//! downstream crates never have to hardcode the number themselves.

use semio_framework_pack_error::PackRefusal;
use semio_framework_replication_codec::{crc32c, read_varint_u64, write_varint_u64, ByteRange, ChunkId, CodecId, CompressionCodec, ContentHash, NoCompression, PackLimits};
use semio_framework_pack_source::{PackSink, PackSource};
use semio_framework_value::{ValueRefusalKind, ValueError};
use std::mem::size_of;
use semio_framework_value::native_decoding::NativeDecodeControl;

#[path="🛫️encoding/🦀️.rs"]
mod controlled_encoding;
pub use controlled_encoding::ControlledPackWriter;

fn check_allocation_layout<T>(count:usize,limit:&'static str)->Result<(),PackRefusal>{
    count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit})?;
    Ok(())
}

fn check_controlled_allocation(control:&NativeDecodeControl<'_>,limits:&PackLimits,bytes:usize)->Result<(),PackRefusal>{
    let next=control.owned_bytes().checked_add(bytes).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled allocation overflow"})?;
    if next>control.maximum_bytes()||next as u64>limits.max_total_alloc{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled allocation exceeds caller limits"});}
    Ok(())
}

fn controlled_vec<T>(control:&mut NativeDecodeControl<'_>,limits:&PackLimits,count:usize)->Result<Vec<T>,PackRefusal>{
    check_controlled_allocation(control,limits,count.checked_mul(size_of::<T>()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled collection size overflow"})?)?;
    control.allocate_vec(count).map_err(PackRefusal::ValueRefusal)
}

#[cfg(feature="deflate")]
struct OwnedControlledInflater(semio_framework_replication_codec::DeflateRetainedCursor);
#[cfg(feature="deflate")]
impl Drop for OwnedControlledInflater{fn drop(&mut self){while self.0.close_step(256,65536)!=semio_framework_replication_codec::RetainedInflateCloseStep::Complete{}}}

async fn controlled_crc<S:PackSource>(source:&S,offset:u64,length:u64,control:&mut NativeDecodeControl<'_>)->Result<u32,S::Error>{
    let total=usize::try_from(length).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled CRC range exceeds address space"})?;
    control.begin_stage(total).map_err(PackRefusal::ValueRefusal)?;
    let mut crc=semio_framework_replication_codec::Crc32cCursor::new();let mut page=[0;4096];let mut position=0;
    while position<total{let count=(total-position).min(page.len());source.read_exact_at(offset+position as u64,&mut page[..count]).await?;crc.update_page(&page[..count]);position+=count;control.advance(count).map_err(PackRefusal::ValueRefusal)?;}
    Ok(crc.finish())
}

async fn append_controlled_payload<S:PackSource>(source:&S,offset:u64,stored_len:u64,raw_len:u64,codec:CodecId,output:&mut Vec<u8>,limits:&PackLimits,control:&mut NativeDecodeControl<'_>)->Result<(),S::Error>{
    let expected=usize::try_from(raw_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled payload exceeds address space"})?;
    if expected>output.capacity()-output.len(){return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled payload exceeds admitted capacity"}).into());}
    control.begin_stage(expected).map_err(PackRefusal::ValueRefusal)?;
    let mut page=[0;4096];let start=output.len();
    match codec.0{
        0=>{
            if stored_len!=raw_len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"identity",offset,detail:"stored and decoded lengths differ".into()}).into());}
            let mut position=0;while position<expected{let count=(expected-position).min(page.len());source.read_exact_at(offset+position as u64,&mut page[..count]).await?;output.extend_from_slice(&page[..count]);position+=count;control.advance(count).map_err(PackRefusal::ValueRefusal)?;}
        },
        1=>{
            #[cfg(feature="deflate")]
            {
                let workload=expected.checked_add(usize::try_from(stored_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"compressed input exceeds address space"})?).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"inflation work overflow"})?;
                control.begin_stage(workload).map_err(PackRefusal::ValueRefusal)?;
                let maximum=control.maximum_bytes().min(usize::try_from(limits.max_total_alloc).unwrap_or(usize::MAX)).checked_sub(control.owned_bytes()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled inflater allowance exhausted"})?;
                let mut inflater=OwnedControlledInflater(semio_framework_replication_codec::DeflateRetainedCursor::try_new(raw_len,limits.max_segment_len,maximum)?);
                let mut stored_position=0;let mut page_position=0;let mut page_len=0;
                loop{
                    if let Some(bytes)=inflater.0.next_allocation_bytes(){check_controlled_allocation(control,limits,bytes)?;control.charge(bytes).map_err(PackRefusal::ValueRefusal)?;inflater.0.reserve_allocation(bytes).map_err(|error|PackRefusal::RetainedAllocation{kind:error.kind,allocated_bytes:error.allocated_bytes,what:"controlled inflater allocation",offset,detail:error.reason})?;continue;}
                    match inflater.0.grant(stored_position==stored_len)?{
                        semio_framework_replication_codec::DeflateRetainedStep::Byte(byte)=>{output.push(byte);control.step().map_err(PackRefusal::ValueRefusal)?;},
                        semio_framework_replication_codec::DeflateRetainedStep::NeedInput=>{
                            if inflater.0.next_allocation_bytes().is_some(){continue;}
                            if !inflater.0.can_admit(){control.checkpoint().map_err(PackRefusal::ValueRefusal)?;continue;}
                            if stored_position==stored_len{return Err((PackRefusal::Truncated(offset+stored_position)).into());}
                            if page_position==page_len{page_len=usize::try_from((stored_len-stored_position).min(page.len() as u64)).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled inflater page length"})?;source.read_exact_at(offset+stored_position,&mut page[..page_len]).await?;page_position=0;}
                            inflater.0.admit_byte(page[page_position]).map_err(|_|PackRefusal::Malformed{kind:ValueRefusalKind::InvariantViolated,what:"deflate",offset:offset+stored_position,detail:"inflater rejected available input".into()})?;page_position+=1;stored_position+=1;control.step().map_err(PackRefusal::ValueRefusal)?;
                        },
                        semio_framework_replication_codec::DeflateRetainedStep::Complete=>{if stored_position!=stored_len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"deflate",offset:offset+stored_position,detail:"trailing compressed input".into()}).into());}break;},
                    }
                }
            }
            #[cfg(not(feature="deflate"))]
            return Err((PackRefusal::UnsupportedCodec(1)).into());
        },
        other=>return Err((PackRefusal::UnsupportedCodec(other)).into()),
    }
    if output.len()-start!=expected{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"payload",offset,detail:"decoded payload length mismatch".into()}).into());}
    Ok(())
}

async fn append_controlled_segment<S:PackSource>(source:&S,offset:u64,output:&mut Vec<u8>,maximum_output:Option<usize>,limits:&PackLimits,verification:VerificationLevel,control:&mut NativeDecodeControl<'_>)->Result<(u8,u64),S::Error>{
    control.checkpoint().map_err(PackRefusal::ValueRefusal)?;
    let total=source.len().await;let kind=read_u8_at(source,offset).await?;let flags=read_u8_at(source,offset.checked_add(1).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment offset overflow"})?).await?;
    let (stored_len,n1)=read_varint_u64_at(source,offset.checked_add(2).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment offset overflow"})?).await?;
    let mut payload_offset=offset.checked_add(2+n1).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment header overflow"})?;
    let raw_len=if flags&1!=0{let(length,n2)=read_varint_u64_at(source,payload_offset).await?;payload_offset=payload_offset.checked_add(n2).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment header overflow"})?;length}else{stored_len};
    if stored_len>limits.max_segment_len||raw_len>limits.max_segment_len{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled segment exceeds max_segment_len"}).into());}
    let crc_offset=payload_offset.checked_add(stored_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment range overflow"})?;let end=crc_offset.checked_add(4).filter(|end|*end<=total).ok_or(PackRefusal::Truncated(crc_offset))?;
    let length=usize::try_from(raw_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment exceeds address space"})?;
    if let Some(maximum)=maximum_output{if output.len().checked_add(length).filter(|length|*length<=maximum).is_none(){return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"segment exceeds declared output allocation"}).into());}}
    if length>output.capacity()-output.len(){
        check_controlled_allocation(control,limits,output.len().checked_add(length).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment allocation overflow"})?)?;
        control.charge(output.len()+length).map_err(PackRefusal::ValueRefusal)?;check_allocation_layout::<u8>(output.len()+length,"controlled segment allocation failed")?;output.try_reserve_exact(length).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"controlled segment allocation failed"})?;
    }
    if verification.checks_crc(){let mut bytes=[0;4];source.read_exact_at(crc_offset,&mut bytes).await?;if controlled_crc(source,offset,crc_offset-offset,control).await?!=u32::from_le_bytes(bytes){return Err((PackRefusal::ChecksumMismatch{segment:"segment",offset:crc_offset}).into());}}
    let codec=if flags&1!=0{CodecId((flags>>1)&7)}else{CodecId(0)};
    append_controlled_payload(source,payload_offset,stored_len,raw_len,codec,output,limits,control).await?;
    Ok((kind,end-offset))
}

//#region 🔖️Header
/// 🧲️ The 8-byte magic every `.spk` pack file begins with.
pub const MAGIC: [u8; 8] = [0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
/// 📏️ Fixed wire size of the header, in bytes.
pub const HEADER_SIZE: usize = 32;
/// 🔢️ The container format version this crate writes and reads.
pub const FORMAT_VERSION_MAJOR: u16 = 1;
/// 🔢️ The container format minor version this crate writes.
pub const FORMAT_VERSION_MINOR: u16 = 0;

/// 🗜️ Required flag bit: at least one segment/chunk in this file uses compression.
pub const REQUIRED_COMPRESSED: u32 = 1 << 0;
/// 🧱️ Required flag bit: this file contains a chunk table.
pub const REQUIRED_CHUNKED: u32 = 1 << 1;
/// 🔒️ Required flag bit: reserved for encryption, never set by this crate.
pub const REQUIRED_ENCRYPTED: u32 = 1 << 2;
/// ⛓️ Required flag bit: reserved for footer chaining.
pub const REQUIRED_FOOTER_CHAIN: u32 = 1 << 3;
pub const REQUIRED_KNOWN_MASK: u32 = REQUIRED_COMPRESSED | REQUIRED_CHUNKED | REQUIRED_ENCRYPTED | REQUIRED_FOOTER_CHAIN;

/// 🧮️ Optional flag bit: the document body was encoded in canonical form.
pub const OPTIONAL_CANONICAL: u32 = 1 << 0;
/// 🌊️ Optional flag bit: the file was produced by a streaming writer.
pub const OPTIONAL_STREAMED: u32 = 1 << 1;
/// 🧬️ Optional flag bit: a schema segment is present.
pub const OPTIONAL_HAS_SCHEMA: u32 = 1 << 2;

/// 🪪️ The fixed 32-byte superblock header: magic, version, feature flags, self-CRC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub version_major: u16,
    pub version_minor: u16,
    pub required_flags: u32,
    pub optional_flags: u32,
}

impl Header {
    /// ✍️ Serializes to the exact 32-byte wire form, computing `header_crc32` over
    /// bytes `0..20` and zeroing the 8 reserved bytes.
    // 🧮️ Every helper this fn calls (`crc32c`, `PackSink::write_all`) is a first-party `async fn`
    // one hop away in `semio-framework-replication`, which this packet does not own; R9 rule 3
    // ("if every consumer can become async, make it async instead") applies, not R9 rule 2 — see
    // 📓️terra-pack-finish-report.md §"pure-computation-made-async: the recipe".
    async fn write_bytes(&self) -> [u8; HEADER_SIZE] {
        self.write_bytes_sync()
    }

    fn write_bytes_sync(&self)->[u8;HEADER_SIZE]{
        let mut buf = [0u8; HEADER_SIZE];
        buf[0..8].copy_from_slice(&MAGIC);
        buf[8..10].copy_from_slice(&self.version_major.to_le_bytes());
        buf[10..12].copy_from_slice(&self.version_minor.to_le_bytes());
        buf[12..16].copy_from_slice(&self.required_flags.to_le_bytes());
        buf[16..20].copy_from_slice(&self.optional_flags.to_le_bytes());
        let crc = crc32c(&buf[0..20]);
        buf[20..24].copy_from_slice(&crc.to_le_bytes());
        buf
    }

    /// 📖️ Parses and validates a 32-byte header: magic, self-CRC, and that
    /// `required_flags` sets no bit outside the known `0..=3` range.
    async fn parse(bytes: &[u8]) -> Result<Self, PackRefusal> {
        if bytes.len() < HEADER_SIZE {
            return Err(PackRefusal::Truncated(bytes.len() as u64));
        }
        if bytes[0..8] != MAGIC {
            return Err(PackRefusal::BadMagic);
        }
        let version_major = u16::from_le_bytes([bytes[8], bytes[9]]);
        let version_minor = u16::from_le_bytes([bytes[10], bytes[11]]);
        let required_flags = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let optional_flags = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        let stored_crc = u32::from_le_bytes(bytes[20..24].try_into().unwrap());
        let computed_crc = crc32c(&bytes[0..20]);
        if stored_crc != computed_crc {
            return Err(PackRefusal::ChecksumMismatch { segment: "header", offset: 20 });
        }
        let unknown = required_flags & !REQUIRED_KNOWN_MASK;
        if unknown != 0 {
            return Err(PackRefusal::UnknownRequiredFlags(unknown));
        }
        if version_major != FORMAT_VERSION_MAJOR {
            return Err(PackRefusal::UnsupportedVersion { major: version_major, minor: version_minor });
        }
        Ok(Self { version_major, version_minor, required_flags, optional_flags })
    }
}
//#endregion 🔖️Header

//#region 🔖️Footer
/// 🧲️ The 8-byte magic the footer begins with.
pub const FOOTER_MAGIC: [u8; 8] = *b"SPKFOOT1";
/// 📏️ Fixed wire size of the footer, in bytes, at the end of the file.
/// See the module doc for why this is 84, not the contract prose's arithmetically-inconsistent
/// "80" (the prose sum omits the trailing `footer_crc32` field's own 4 bytes).
pub const FOOTER_SIZE: usize = 84;

/// 🪶️ The fixed-size trailer every pack file ends with — the single root of trust a
/// reader locates by seeking to `file_len - FOOTER_SIZE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Footer {
    pub version_major: u16,
    pub version_minor: u16,
    pub required_flags: u32,
    pub manifest_offset: u64,
    pub manifest_len: u64,
    pub file_len: u64,
    pub content_hash: ContentHash,
    pub prev_footer_offset: u64,
}

impl Footer {
    /// ✍️ Serializes to the exact 84-byte wire form, computing `footer_crc32` over the
    /// preceding 80 bytes.
    // 🧮️ Same R9-rule-3 reasoning as `Header::write_bytes` above — `crc32c` is a pure but
    // externally-owned `async fn`, so this fn propagates `async` rather than fighting it.
    async fn write_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(FOOTER_SIZE);
        buf.extend_from_slice(&FOOTER_MAGIC);
        buf.extend_from_slice(&self.version_major.to_le_bytes());
        buf.extend_from_slice(&self.version_minor.to_le_bytes());
        buf.extend_from_slice(&self.required_flags.to_le_bytes());
        buf.extend_from_slice(&self.manifest_offset.to_le_bytes());
        buf.extend_from_slice(&self.manifest_len.to_le_bytes());
        buf.extend_from_slice(&self.file_len.to_le_bytes());
        buf.extend_from_slice(&self.content_hash.0);
        buf.extend_from_slice(&self.prev_footer_offset.to_le_bytes());
        let crc = crc32c(&buf);
        buf.extend_from_slice(&crc.to_le_bytes());
        buf
    }

    /// 📖️ Parses and validates an 84-byte footer: magic and self-CRC over the first 80
    /// bytes. Does not cross-check `file_len` against an actual source — callers that have one
    /// should do so themselves (see `PackFile::open_superblock`).
    async fn parse(bytes: &[u8]) -> Result<Self, PackRefusal> {
        if bytes.len() < FOOTER_SIZE {
            return Err(PackRefusal::Truncated(bytes.len() as u64));
        }
        if bytes[0..8] != FOOTER_MAGIC {
            return Err(PackRefusal::BadMagic);
        }
        let version_major = u16::from_le_bytes(bytes[8..10].try_into().unwrap());
        let version_minor = u16::from_le_bytes(bytes[10..12].try_into().unwrap());
        let required_flags = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let manifest_offset = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let manifest_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
        let file_len = u64::from_le_bytes(bytes[32..40].try_into().unwrap());
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes[40..72]);
        let prev_footer_offset = u64::from_le_bytes(bytes[72..80].try_into().unwrap());
        let stored_crc = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
        let computed_crc = crc32c(&bytes[0..80]);
        if stored_crc != computed_crc {
            return Err(PackRefusal::ChecksumMismatch { segment: "footer", offset: 80 });
        }
        Ok(Self { version_major, version_minor, required_flags, manifest_offset, manifest_len, file_len, content_hash: ContentHash(hash), prev_footer_offset })
    }
}
//#endregion 🔖️Footer

//#region 🔖️Segment
/// 📦️ The fully-encoded wire bytes of one framed segment, plus the byte offsets within
/// them a writer needs to record chunk-table/manifest-span metadata without re-parsing.
struct EncodedSegment {
    bytes: Vec<u8>,
    #[cfg(test)]
    header_len: usize,
    #[cfg(test)]
    stored_len: usize,
}

fn retained_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
    let mut len = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output[len] = byte;
        len += 1;
        if value == 0 {
            return &output[..len];
        }
    }
}

fn retained_varint_len(value: u64) -> usize {
    let mut output = [0u8; 10];
    retained_varint(value, &mut output).len()
}

/// 🧵️ Resolves `CodecId` to this crate's codec implementations for compression.
async fn codec_compress(codec: CodecId, raw: &[u8]) -> Result<Vec<u8>, PackRefusal> {
    match codec.0 {
        0 => Ok(raw.to_vec()),
        1 => semio_framework_replication_codec::deflate_compress(raw),
        other => Err(PackRefusal::UnsupportedCodec(other)),
    }
}

/// 🧵️ Resolves `CodecId` to this crate's codec implementations for decompression.
async fn codec_decompress(codec: CodecId, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackRefusal> {
    match codec.0 {
        0 => NoCompression.decompress(stored, raw_len, limit),
        1 => semio_framework_replication_codec::deflate_decompress(stored, raw_len, limit),
        other => Err(PackRefusal::UnsupportedCodec(other)),
    }
}

/// 🖇️ Frames `payload` as a segment per the contract's byte layout: `kind, flags,
/// seg_len, [raw_len], payload, crc32`. Compresses first when `codec` is non-identity.
async fn encode_segment(kind: u8, codec: CodecId, payload: &[u8]) -> Result<EncodedSegment, PackRefusal> {
    let compressed = codec.0 != 0;
    let stored = if compressed { codec_compress(codec, payload).await? } else { payload.to_vec() };
    let flags: u8 = if compressed { 1 | (codec.0 << 1) } else { 0 };
    let mut buf = Vec::with_capacity(stored.len() + 24);
    buf.push(kind);
    buf.push(flags);
    write_varint_u64(&mut buf, stored.len() as u64);
    if compressed {
        write_varint_u64(&mut buf, payload.len() as u64);
    }
    #[cfg(test)]
    let header_len = buf.len();
    buf.extend_from_slice(&stored);
    let crc = crc32c(&buf);
    buf.extend_from_slice(&crc.to_le_bytes());
    Ok(EncodedSegment { bytes: buf, #[cfg(test)] header_len, #[cfg(test)] stored_len: stored.len() })
}

/// 👓️ A decoded, CRC-checked, decompressed segment plus enough position bookkeeping for
/// callers walking a sequence of segments.
struct DecodedSegment {
    kind: u8,
    payload: Vec<u8>,
    /// ➡️ Total wire bytes consumed by this segment (frame header + stored payload + crc).
    consumed: u64,
}

/// 1⃣ Bounds-checked single-byte read at an absolute file offset.
async fn read_u8_at<S: PackSource>(source: &S, offset: u64) -> Result<u8, S::Error> {
    let mut buf = [0u8; 1];
    source.read_exact_at(offset, &mut buf).await?;
    Ok(buf[0])
}

/// 🔢️ Reads one LEB128 varint starting at an absolute file offset, one byte at a time so
/// it never over-reads past a legitimately short remaining file. Returns `(value, bytes_consumed)`.
async fn read_varint_u64_at<S: PackSource>(source: &S, offset: u64) -> Result<(u64, u64), S::Error> {
    let mut tmp = [0u8; 10];
    let mut i = 0u64;
    loop {
        let byte = read_u8_at(source, offset + i).await?;
        tmp[i as usize] = byte;
        i += 1;
        if byte & 0x80 == 0 {
            break;
        }
        if i >= 10 {
            return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"varint", offset, detail: "overlong varint".to_string() }).into());
        }
    }
    let mut pos = 0usize;
    let value = read_varint_u64(&tmp[..i as usize], &mut pos)?;
    Ok((value, i))
}

/// 🚪️ Decodes one framed segment starting at an absolute file offset: reads and
/// bounds-checks `kind, flags, seg_len, [raw_len]`, validates lengths against
/// `limits.max_segment_len` **before** allocating the payload buffer, then optionally verifies
/// the frame's CRC-32C and decompresses. Unknown `kind` values are decoded and returned as-is —
/// callers that only want known kinds are responsible for skipping/rejecting them, this function
/// never errors on an unrecognized kind.
async fn decode_segment_at<S: PackSource>(source: &S, offset: u64, limits: &PackLimits, verify_crc: bool) -> Result<DecodedSegment, S::Error> {
    let total_len = source.len().await;
    if offset >= total_len {
        return Err((PackRefusal::Truncated(offset)).into());
    }
    let kind = read_u8_at(source, offset).await?;
    let flags = read_u8_at(source, offset + 1).await?;
    let compressed = flags & 0x01 != 0;
    let codec = CodecId((flags >> 1) & 0x07);
    let (stored_len, n1) = read_varint_u64_at(source, offset + 2).await?;
    let mut cursor = offset + 2 + n1;
    let raw_len = if compressed {
        let (v, n2) = read_varint_u64_at(source, cursor).await?;
        cursor += n2;
        v
    } else {
        stored_len
    };
    if stored_len > limits.max_segment_len || raw_len > limits.max_segment_len {
        return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"segment length exceeds max_segment_len"}).into());
    }
    let payload_offset = cursor;
    let payload_end = payload_offset.checked_add(stored_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment payload offset overflow"})?;
    if payload_end > total_len {
        return Err((PackRefusal::Truncated(payload_offset)).into());
    }
    let frame_len=(payload_offset-offset).checked_add(stored_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"frame allocation overflow"})?;
    let allocation=frame_len.checked_add(raw_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"decoded frame allocation overflow"})?;
    if allocation>limits.max_total_alloc{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"decoded frame exceeds max_total_alloc"}).into());}
    let header_len = usize::try_from(payload_offset-offset).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"frame header exceeds address space"})?;
    let frame_len=usize::try_from(frame_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"frame exceeds address space"})?;
    let mut frame=Vec::new();check_allocation_layout::<u8>(frame_len,"frame allocation failed")?;frame.try_reserve_exact(frame_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"frame allocation failed"})?;frame.resize(frame_len,0);
    source.read_exact_at(offset, &mut frame).await?;
    let crc_offset = payload_end;
    if crc_offset + 4 > total_len {
        return Err((PackRefusal::Truncated(crc_offset)).into());
    }
    let mut crc_bytes = [0u8; 4];
    source.read_exact_at(crc_offset, &mut crc_bytes).await?;
    if verify_crc {
        let stored_crc = u32::from_le_bytes(crc_bytes);
        let computed_crc = crc32c(&frame);
        if stored_crc != computed_crc {
            return Err((PackRefusal::ChecksumMismatch { segment: "segment", offset: crc_offset }).into());
        }
    }
    let stored_payload = &frame[header_len..];
    let payload = if compressed { codec_decompress(codec, stored_payload, raw_len, limits.max_segment_len).await? } else { stored_payload.to_vec() };
    let consumed = (crc_offset + 4) - offset;
    Ok(DecodedSegment { kind, payload, consumed })
}
//#endregion 🔖️Segment

//#region 🔖️Symbols
/// ✍️ Serializes a symbol table: `count varint, then count × (len varint, utf8 bytes)`.
/// Exposed so callers (e.g. `pack_value`) can build a `KIND_SYMBOLS` segment payload for
/// `PackWriter::write_segment` without re-implementing this crate's wire format.
pub async fn encode_symbols(symbols: &[String]) -> Vec<u8> {
    let mut buf = Vec::new();
    write_varint_u64(&mut buf, symbols.len() as u64);
    for symbol in symbols {
        let bytes = symbol.as_bytes();
        write_varint_u64(&mut buf, bytes.len() as u64);
        buf.extend_from_slice(bytes);
    }
    buf
}

/// 📖️ Parses a symbol table, rejecting a count over `limits.max_symbols` before
/// allocating the output `Vec`.
async fn decode_symbols(payload: &[u8], limits: &PackLimits) -> Result<Vec<String>, PackRefusal> {
    let mut pos = 0usize;
    let count = read_varint_u64(payload, &mut pos)?;
    if count > limits.max_symbols as u64 {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"symbol count exceeds max_symbols"});
    }
    let slots = count.checked_mul(size_of::<String>() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol allocation overflow"})?;
    if slots > limits.max_total_alloc || count > payload.len() as u64 {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol storage exceeds max_total_alloc"});
    }
    let mut owned_bytes = slots;
    let mut out = Vec::new();
    let count=usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol count exceeds address space"})?;
    check_allocation_layout::<String>(count,"symbol allocation")?;
    out.try_reserve_exact(count).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"symbol allocation"})?;
    for _ in 0..count {
        let len = read_varint_u64(payload, &mut pos)?;
        owned_bytes = owned_bytes.checked_add(len).filter(|bytes| *bytes <= limits.max_total_alloc).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol storage exceeds max_total_alloc"})?;
        let len = usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol length exceeds address space"})?;
        let end = pos.checked_add(len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol length overflow"})?;
        if end > payload.len() {
            return Err(PackRefusal::Truncated(pos as u64));
        }
        let borrowed = std::str::from_utf8(&payload[pos..end]).map_err(|_| PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symbol", offset: pos as u64, detail: "invalid utf8".to_string() })?;
        let mut text = String::new();
        check_allocation_layout::<u8>(len,"symbol text allocation")?;
        text.try_reserve_exact(len).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"symbol text allocation"})?;
        text.push_str(borrowed);
        pos = end;
        out.push(text);
    }
    Ok(out)
}
//#endregion 🔖️Symbols

//#region 🔖️ChunkTable
/// 🧱️ One row of the chunk table: where a chunk's (possibly compressed) payload lives
/// and how to verify it. `crc32` covers the stored (on-disk) bytes; `blake3` covers the raw
/// (decompressed) content — the former is cheap and always checked at `Standard`+, the latter is
/// the content-identity hash only checked at `Full`.
#[derive(Clone, Debug)]
struct ChunkTableEntry {
    offset: u64,
    stored_len: u64,
    raw_len: u64,
    crc32: u32,
    blake3: [u8; 32],
}

/// 📖️ Parses the chunk table, rejecting a count over `limits.max_items` or an entry
/// length over `limits.max_segment_len` before allocating.
async fn decode_chunk_table(payload: &[u8], limits: &PackLimits) -> Result<Vec<ChunkTableEntry>, PackRefusal> {
    let mut pos = 0usize;
    let count = read_varint_u64(payload, &mut pos)?;
    if count > limits.max_items {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"chunk_table count exceeds max_items"});
    }
    let slots = count.checked_mul(size_of::<ChunkTableEntry>() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk table allocation overflow"})?;
    if slots > limits.max_total_alloc || count > payload.len() as u64 / 39 {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk table storage exceeds max_total_alloc"});
    }
    let mut out = Vec::new();
    let count=usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk count exceeds address space"})?;
    check_allocation_layout::<ChunkTableEntry>(count,"chunk table allocation")?;
    out.try_reserve_exact(count).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"chunk table allocation"})?;
    for _ in 0..count {
        let offset = read_varint_u64(payload, &mut pos)?;
        let stored_len = read_varint_u64(payload, &mut pos)?;
        let raw_len = read_varint_u64(payload, &mut pos)?;
        if stored_len > limits.max_segment_len || raw_len > limits.max_segment_len {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"chunk table entry length exceeds max_segment_len"});
        }
        if pos + 4 > payload.len() {
            return Err(PackRefusal::Truncated(pos as u64));
        }
        let crc32 = u32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap());
        pos += 4;
        if pos + 32 > payload.len() {
            return Err(PackRefusal::Truncated(pos as u64));
        }
        let mut blake3 = [0u8; 32];
        blake3.copy_from_slice(&payload[pos..pos + 32]);
        pos += 32;
        out.push(ChunkTableEntry { offset, stored_len, raw_len, crc32, blake3 });
    }
    Ok(out)
}
//#endregion 🔖️ChunkTable

//#region 🔖️Manifest
/// 🗺️ The manifest: spans and counts describing every other segment in the file.
/// `schema_name` round-trips through the symbol table as a symref on the wire (see
/// `PackWriter::finish`/`PackFile::open_manifest`) but is resolved to a plain `String` here for
/// callers' convenience.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    pub schema_name: String,
    pub schema_hash: [u8; 32],
    pub doc_span: ByteRange,
    pub doc_frame_count: u64,
    pub symbols_span: ByteRange,
    pub chunk_table_span: ByteRange,
    pub field_index_span: ByteRange,
    pub uncompressed_body_len: u64,
    pub field_count: u64,
    pub chunk_count: u64,
    pub symbol_count: u64,
}

/// 🗺️ The wire-level manifest fields before `schema_symref` has been resolved against a
/// symbol table (on decode) or after it has been resolved to a symref (on encode).
pub struct RawManifest {
    pub schema_symref: u64,
    pub schema_hash: [u8; 32],
    pub doc_span: ByteRange,
    pub doc_frame_count: u64,
    pub symbols_span: ByteRange,
    pub chunk_table_span: ByteRange,
    pub field_index_span: ByteRange,
    pub uncompressed_body_len: u64,
    pub field_count: u64,
    pub chunk_count: u64,
    pub symbol_count: u64,
}

async fn write_span(buf: &mut Vec<u8>, span: ByteRange) {
    write_varint_u64(buf, span.offset);
    write_varint_u64(buf, span.len);
}

async fn read_span(payload: &[u8], pos: &mut usize) -> Result<ByteRange, PackRefusal> {
    let offset = read_varint_u64(payload, pos)?;
    let len = read_varint_u64(payload, pos)?;
    Ok(ByteRange { offset, len })
}

/// ✍️ Serializes the manifest segment payload per the contract's field order.
async fn encode_manifest_bytes(schema_symref: u64, manifest: &Manifest) -> Vec<u8> {
    let mut buf = Vec::new();
    write_varint_u64(&mut buf, schema_symref);
    buf.extend_from_slice(&manifest.schema_hash);
    write_span(&mut buf, manifest.doc_span).await;
    write_varint_u64(&mut buf, manifest.doc_frame_count);
    write_span(&mut buf, manifest.symbols_span).await;
    write_span(&mut buf, manifest.chunk_table_span).await;
    write_span(&mut buf, manifest.field_index_span).await;
    write_varint_u64(&mut buf, manifest.uncompressed_body_len);
    write_varint_u64(&mut buf, manifest.field_count);
    write_varint_u64(&mut buf, manifest.chunk_count);
    write_varint_u64(&mut buf, manifest.symbol_count);
    buf
}

/// 📖️ Parses the manifest segment payload. Trailing bytes beyond the known fields are
/// silently ignored (additive-evolution slot), never an error.
async fn parse_raw_manifest(payload: &[u8]) -> Result<RawManifest, PackRefusal> {
    let mut pos = 0usize;
    let schema_symref = read_varint_u64(payload, &mut pos)?;
    if pos + 32 > payload.len() {
        return Err(PackRefusal::Truncated(pos as u64));
    }
    let mut schema_hash = [0u8; 32];
    schema_hash.copy_from_slice(&payload[pos..pos + 32]);
    pos += 32;
    let doc_span = read_span(payload, &mut pos).await?;
    let doc_frame_count = read_varint_u64(payload, &mut pos)?;
    let symbols_span = read_span(payload, &mut pos).await?;
    let chunk_table_span = read_span(payload, &mut pos).await?;
    let field_index_span = read_span(payload, &mut pos).await?;
    let uncompressed_body_len = read_varint_u64(payload, &mut pos)?;
    let field_count = read_varint_u64(payload, &mut pos)?;
    let chunk_count = read_varint_u64(payload, &mut pos)?;
    let symbol_count = read_varint_u64(payload, &mut pos)?;
    Ok(RawManifest { schema_symref, schema_hash, doc_span, doc_frame_count, symbols_span, chunk_table_span, field_index_span, uncompressed_body_len, field_count, chunk_count, symbol_count })
}

// 🚫️async: E1 pure struct-field resolution — no I/O, no call into any async fn (unlike its
// siblings in this region it never touches `crc32c`/`read_varint_u64`), so nothing forces async.
fn resolve_manifest(raw: &RawManifest, symbols: &[String]) -> Result<Manifest, PackRefusal> {
    let schema_name =
        if symbols.is_empty() && raw.schema_symref == 0 { String::new() } else { symbols.get(raw.schema_symref as usize).cloned().ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest", offset: 0, detail: "schema symref out of range".to_string() })? };
    Ok(Manifest {
        schema_name,
        schema_hash: raw.schema_hash,
        doc_span: raw.doc_span,
        doc_frame_count: raw.doc_frame_count,
        symbols_span: raw.symbols_span,
        chunk_table_span: raw.chunk_table_span,
        field_index_span: raw.field_index_span,
        uncompressed_body_len: raw.uncompressed_body_len,
        field_count: raw.field_count,
        chunk_count: raw.chunk_count,
        symbol_count: raw.symbol_count,
    })
}
//#endregion 🔖️Manifest

//#region 🔖️Verify
/// 🛡️ The exact shared Pack verification policy controls CRC and content hash checks.
pub use semio_framework_replication_codec::PackVerificationLevel as VerificationLevel;
//#endregion 🔖️Verify

//#region 🔖️Writer
/// ⚙️ Header flags and the codec to compress every segment/chunk with when building a
/// pack file. `REQUIRED_COMPRESSED` is set automatically in the written header whenever `codec`
/// is non-identity, so callers only need to set it themselves for other reasons (e.g. signaling
/// intent before any segment is written).
#[derive(Clone, Copy, Debug)]
pub struct WriteOptions {
    pub required_flags: u32,
    pub optional_flags: u32,
    pub codec: CodecId,
}

/// ✒️ Sequential pack file builder. Write segments/chunks in any order, then `finish`
/// with a `Manifest` — `finish` fills in `symbols_span`/`chunk_table_span`/`chunk_count`/
/// `symbol_count` authoritatively from what was actually written (the caller-supplied values in
/// those fields are ignored), and resolves `manifest.schema_name` to a symref against the last
/// `write_segment(KIND_SYMBOLS, ...)` call, so a symbols segment must be written (containing
/// `schema_name`, unless it's empty) before `finish` is called.
pub struct PackWriter<S: PackSink> {
    sink: S,
    options: WriteOptions,
    chunks: Vec<ChunkTableEntry>,
    symbols: Vec<String>,
    symbols_span: Option<ByteRange>,
    document_hasher: semio_framework_hash::Hasher,
}

pub struct PackIdentitySegment<'a, S: PackSink> {
    owner: &'a mut PackWriter<S>,
    kind: u8,
    payload_len: usize,
    written: usize,
    crc: semio_framework_replication_codec::Crc32cCursor,
}

pub struct PackIdentityChunk<'a, S: PackSink> {
    owner: &'a mut PackWriter<S>,
    payload_offset: u64,
    payload_len: usize,
    written: usize,
    segment_crc: semio_framework_replication_codec::Crc32cCursor,
    payload_crc: semio_framework_replication_codec::Crc32cCursor,
    hash: semio_framework_hash::Hasher,
}

impl<S: PackSink> PackIdentityChunk<'_, S> {
    pub async fn write_fragment(&mut self, fragment: &[u8]) -> Result<(), S::Error> {
        self.written = self.written.checked_add(fragment.len()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk payload length overflow"})?;
        if self.written > self.payload_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvariantViolated,limit:"chunk exceeded retained payload reservation"}).into());
        }
        self.segment_crc.update_page(fragment);
        self.payload_crc.update_page(fragment);
        self.hash.update(fragment);
        self.owner.sink.write_all(fragment).await
    }

    pub async fn finish(self) -> Result<ChunkId, S::Error> {
        if self.written != self.payload_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvariantViolated,limit:"chunk ended before retained payload reservation"}).into());
        }
        self.owner.sink.write_all(&self.segment_crc.finish().to_le_bytes()).await?;
        let id = ChunkId(self.owner.chunks.len() as u32);
        self.owner.chunks.push(ChunkTableEntry { offset: self.payload_offset, stored_len: self.payload_len as u64, raw_len: self.payload_len as u64, crc32: self.payload_crc.finish(), blake3: *self.hash.finalize().as_bytes() });
        Ok(id)
    }

    pub fn close(self) {}
}

impl<'a, S: PackSink> PackIdentitySegment<'a, S> {
    pub async fn write_fragment(&mut self, fragment: &[u8]) -> Result<(), S::Error> {
        self.written = self.written.checked_add(fragment.len()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment payload length overflow"})?;
        if self.written > self.payload_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvariantViolated,limit:"segment exceeded retained payload reservation"}).into());
        }
        self.crc.update_page(fragment);
        if self.kind == semio_framework_replication_codec::KIND_DOCUMENT {
            self.owner.document_hasher.update(fragment);
        }
        self.owner.sink.write_all(fragment).await
    }

    pub async fn finish(self) -> Result<(), S::Error> {
        if self.written != self.payload_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvariantViolated,limit:"segment ended before retained payload reservation"}).into());
        }
        self.owner.sink.write_all(&self.crc.finish().to_le_bytes()).await
    }
}

impl<S: PackSink> PackWriter<S> {
    /// 🚀️ Writes the 32-byte header and returns a writer positioned right after it.
    pub async fn begin(mut sink: S, options: &WriteOptions) -> Result<Self, S::Error> {
        let mut required_flags = options.required_flags;
        if options.codec.0 != 0 {
            required_flags |= REQUIRED_COMPRESSED;
        }
        let unknown = required_flags & !REQUIRED_KNOWN_MASK;
        if unknown != 0 {
            return Err((PackRefusal::UnknownRequiredFlags(unknown)).into());
        }
        let header = Header { version_major: FORMAT_VERSION_MAJOR, version_minor: FORMAT_VERSION_MINOR, required_flags, optional_flags: options.optional_flags };
        sink.write_all(&header.write_bytes().await).await?;
        Ok(Self { sink, options: WriteOptions { required_flags, optional_flags: options.optional_flags, codec: options.codec }, chunks: Vec::new(), symbols: Vec::new(), symbols_span: None, document_hasher: semio_framework_hash::Hasher::new() })
    }

    /// 📍️ Current absolute write position — the offset the next segment/chunk will start
    /// at. Callers building a `Manifest` (e.g. `doc_span`/`field_index_span`) call this before
    /// and after their own `write_segment` calls to record spans this writer doesn't track
    /// automatically.
    pub async fn position(&self) -> u64 {
        self.sink.position().await
    }

    pub async fn begin_identity_segment(&mut self, kind: u8, payload_len: usize) -> Result<PackIdentitySegment<'_, S>, S::Error> {
        let mut length = [0u8; 10];
        let mut remaining = payload_len as u64;
        let mut count = 0;
        loop {
            let mut byte = (remaining & 0x7f) as u8;
            remaining >>= 7;
            if remaining != 0 {
                byte |= 0x80;
            }
            length[count] = byte;
            count += 1;
            if remaining == 0 {
                break;
            }
        }
        let fixed = [kind, 0];
        let mut crc = semio_framework_replication_codec::Crc32cCursor::new();
        crc.update_page(&fixed);
        crc.update_page(&length[..count]);
        self.sink.write_all(&fixed).await?;
        self.sink.write_all(&length[..count]).await?;
        Ok(PackIdentitySegment { owner: self, kind, payload_len, written: 0, crc })
    }

    /// 🧱️ Opens one `KIND_CHUNK` segment written RAW, whatever codec the pack as a whole
    /// uses. A chunk is framed with its own `flags = 0`, so a reader takes its codec from the
    /// segment, not from the pack — and the chunk table indexes each chunk by absolute payload
    /// offset plus a content hash over the stored bytes, which only lines up while those bytes are
    /// the payload. Refusing here instead made every large `Bytes64` field unencodable under the
    /// DEFAULT `EncodeOptions` (`codec: CodecId(1)`): `large_bytes_field_is_chunked_and_round_trips`
    /// failed at `encode: UnsupportedCodec(1)`, i.e. no document with a blob past
    /// `chunk_threshold` could be written at all.
    pub async fn begin_identity_chunk(&mut self, payload_len: usize) -> Result<PackIdentityChunk<'_, S>, S::Error> {
        let base = self.sink.position().await;
        let mut length = [0u8; 10];
        let mut remaining = payload_len as u64;
        let mut count = 0usize;
        loop {
            let mut byte = (remaining & 0x7f) as u8;
            remaining >>= 7;
            if remaining != 0 {
                byte |= 0x80;
            }
            length[count] = byte;
            count += 1;
            if remaining == 0 {
                break;
            }
        }
        let fixed = [semio_framework_replication_codec::KIND_CHUNK, 0];
        let mut segment_crc = semio_framework_replication_codec::Crc32cCursor::new();
        segment_crc.update_page(&fixed);
        segment_crc.update_page(&length[..count]);
        self.sink.write_all(&fixed).await?;
        self.sink.write_all(&length[..count]).await?;
        Ok(PackIdentityChunk { owner: self, payload_offset: base + fixed.len() as u64 + count as u64, payload_len, written: 0, segment_crc, payload_crc: semio_framework_replication_codec::Crc32cCursor::new(), hash: semio_framework_hash::Hasher::new() })
    }

    /// 🖇️ Frames, compresses (per `options.codec`), CRCs, and writes one segment. A
    /// `KIND_SYMBOLS` segment is parsed and remembered for `schema_name` resolution in `finish`;
    /// a `KIND_DOCUMENT` segment's raw bytes are folded into the running content-hash used for
    /// the footer.
    pub async fn write_segment(&mut self, kind: u8, payload: &[u8]) -> Result<(), S::Error> {
        if self.options.codec.0 == 0 && kind != semio_framework_replication_codec::KIND_SYMBOLS {
            let mut segment = self.begin_identity_segment(kind, payload.len()).await?;
            segment.write_fragment(payload).await?;
            return segment.finish().await;
        }
        let base = self.sink.position().await;
        let encoded = encode_segment(kind, self.options.codec, payload).await?;
        self.sink.write_all(&encoded.bytes).await?;
        if kind == semio_framework_replication_codec::KIND_SYMBOLS {
            self.symbols = decode_symbols(payload, &PackLimits::default()).await?;
            self.symbols_span = Some(ByteRange { offset: base, len: encoded.bytes.len() as u64 });
        }
        if kind == semio_framework_replication_codec::KIND_DOCUMENT {
            self.document_hasher.update(payload);
        }
        Ok(())
    }

    /// 🧱️ Writes a `KIND_CHUNK` segment and records its offset/lengths/hashes for the
    /// chunk table `finish` will emit.
    #[cfg(test)]
    pub async fn write_chunk(&mut self, payload: &[u8]) -> Result<ChunkId, S::Error> {
        if self.options.codec.0 == 0 {
            let mut chunk = self.begin_identity_chunk(payload.len()).await?;
            chunk.write_fragment(payload).await?;
            return chunk.finish().await;
        }
        let base = self.sink.position().await;
        let encoded = encode_segment(semio_framework_replication_codec::KIND_CHUNK, self.options.codec, payload).await?;
        let payload_offset = base + encoded.header_len as u64;
        let stored_bytes = &encoded.bytes[encoded.header_len..encoded.header_len + encoded.stored_len];
        let stored_crc = crc32c(stored_bytes);
        let raw_hash = semio_framework_hash::hash(payload);
        self.sink.write_all(&encoded.bytes).await?;
        let id = ChunkId(self.chunks.len() as u32);
        self.chunks.push(ChunkTableEntry { offset: payload_offset, stored_len: encoded.stored_len as u64, raw_len: payload.len() as u64, crc32: stored_crc, blake3: *raw_hash.as_bytes() });
        Ok(id)
    }

    /// 🏁️ Writes the chunk table (if any chunks were written), the manifest, an `End`
    /// segment, then the footer — and returns the underlying sink.
    pub async fn finish(mut self, manifest: &Manifest) -> Result<S, S::Error> {
        let chunk_count = self.chunks.len() as u64;
        let mut chunk_table_span = ByteRange { offset: 0, len: 0 };
        if !self.chunks.is_empty() {
            let base = self.sink.position().await;
            let chunks = std::mem::take(&mut self.chunks);
            let payload_len = chunks.iter().try_fold(retained_varint_len(chunks.len() as u64), |length, entry| {
                length
                    .checked_add(retained_varint_len(entry.offset))
                    .and_then(|length| length.checked_add(retained_varint_len(entry.stored_len)))
                    .and_then(|length| length.checked_add(retained_varint_len(entry.raw_len)))
                    .and_then(|length| length.checked_add(4 + 32))
                    .ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk table retained length overflow"})
            })?;
            let mut segment = self.begin_identity_segment(semio_framework_replication_codec::KIND_CHUNK_TABLE, payload_len).await?;
            let mut varint = [0u8; 10];
            segment.write_fragment(retained_varint(chunks.len() as u64, &mut varint)).await?;
            for entry in chunks {
                segment.write_fragment(retained_varint(entry.offset, &mut varint)).await?;
                segment.write_fragment(retained_varint(entry.stored_len, &mut varint)).await?;
                segment.write_fragment(retained_varint(entry.raw_len, &mut varint)).await?;
                segment.write_fragment(&entry.crc32.to_le_bytes()).await?;
                segment.write_fragment(&entry.blake3).await?;
            }
            segment.finish().await?;
            chunk_table_span = ByteRange { offset: base, len: self.sink.position().await - base };
        }
        let schema_symref = if manifest.schema_name.is_empty() {
            0u64
        } else {
            self.symbols.iter().position(|symbol| symbol == &manifest.schema_name).ok_or_else(|| PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:0,detail:format!("schema_name {:?} not found in written symbols table",manifest.schema_name)})? as u64
        };
        let final_manifest = Manifest {
            schema_name: manifest.schema_name.clone(),
            schema_hash: manifest.schema_hash,
            doc_span: manifest.doc_span,
            doc_frame_count: manifest.doc_frame_count,
            symbols_span: self.symbols_span.unwrap_or(ByteRange { offset: 0, len: 0 }),
            chunk_table_span,
            field_index_span: manifest.field_index_span,
            uncompressed_body_len: manifest.uncompressed_body_len,
            field_count: manifest.field_count,
            chunk_count,
            symbol_count: self.symbols.len() as u64,
        };
        let manifest_bytes = encode_manifest_bytes(schema_symref, &final_manifest).await;
        let manifest_base = self.sink.position().await;
        self.write_segment(semio_framework_replication_codec::KIND_MANIFEST, &manifest_bytes).await?;
        let manifest_span = ByteRange { offset: manifest_base, len: self.sink.position().await - manifest_base };

        let mut end = self.begin_identity_segment(semio_framework_replication_codec::KIND_END, 0).await?;
        end.write_fragment(&[]).await?;
        end.finish().await?;

        let content_hash = ContentHash(*self.document_hasher.finalize().as_bytes());
        let file_len = self.sink.position().await + FOOTER_SIZE as u64;
        let footer = Footer {
            version_major: FORMAT_VERSION_MAJOR,
            version_minor: FORMAT_VERSION_MINOR,
            required_flags: self.options.required_flags,
            manifest_offset: manifest_span.offset,
            manifest_len: manifest_span.len,
            file_len,
            content_hash,
            prev_footer_offset: 0,
        };
        self.sink.write_all(&footer.write_bytes().await).await?;
        self.sink.flush().await?;
        Ok(self.sink)
    }
}
//#endregion 🔖️Writer

//#region 🔖️Reader
/// 🪪️ The two fixed-size, always-present anchors of a pack file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Superblock {
    pub header: Header,
    pub footer: Footer,
}

/// 📂️ Random-access pack file reader with three progressively-deeper open levels:
/// `open_superblock` (header+footer only), `open_manifest` (+manifest+symbols+chunk table),
/// and `body_bytes`/`read_chunk` (full content, decompressed and optionally content-hash
/// verified).
pub struct PackFile<S: PackSource> {
    source: S,
    limits: PackLimits,
    superblock: Superblock,
    manifest: Option<Manifest>,
    symbols: Vec<String>,
    chunk_table: Vec<ChunkTableEntry>,
}

/// 🧩️ Retained identity-chunk reader that advances by one caller-owned fragment.
pub struct PackIdentityChunkCursor<'file, S: PackSource> {
    source: &'file S,
    entry: ChunkTableEntry,
    verification: VerificationLevel,
    offset: u64,
    crc: semio_framework_replication_codec::Crc32cCursor,
    hash: semio_framework_hash::Hasher,
    terminal: bool,
}

impl<'file, S: PackSource> PackIdentityChunkCursor<'file, S> {
    pub fn len(&self) -> u64 {
        self.entry.raw_len
    }

    pub fn is_empty(&self) -> bool {
        self.entry.raw_len == 0
    }

    pub fn remaining(&self) -> u64 {
        self.entry.raw_len.saturating_sub(self.offset)
    }

    pub async fn read_fragment(&mut self, target: &mut [u8]) -> Result<usize, S::Error> {
        if self.terminal {
            return Ok(0);
        }
        let count = usize::try_from(self.remaining().min(target.len() as u64)).map_err(|_| PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"identity chunk fragment length"})?;
        if count == 0 {
            if self.verification.checks_crc() && self.crc.finish() != self.entry.crc32 {
                return Err((PackRefusal::ChecksumMismatch { segment: "chunk", offset: self.entry.offset }).into());
            }
            if self.verification.checks_content_hash() && self.hash.finalize().as_bytes() != &self.entry.blake3 {
                return Err((PackRefusal::ContentHashMismatch).into());
            }
            self.terminal = true;
            return Ok(0);
        }
        self.source.read_exact_at(self.entry.offset + self.offset, &mut target[..count]).await?;
        self.crc.update_page(&target[..count]);
        self.hash.update(&target[..count]);
        self.offset += count as u64;
        Ok(count)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl<S: PackSource> PackFile<S> {
    /// 🛬️ Reads owned manifest/catalog state with interior source, checksum, inflation and copy control.
    pub async fn open_manifest_controlled(source:S,limits:&PackLimits,verification:VerificationLevel,control:&mut NativeDecodeControl<'_>)->Result<Self,S::Error>{
        control.checkpoint().map_err(PackRefusal::ValueRefusal)?;let len=source.len().await;
        if len>limits.max_file_len{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"source length exceeds max_file_len"}).into());}
        if len<(HEADER_SIZE+FOOTER_SIZE) as u64{return Err((PackRefusal::Truncated(len)).into());}
        let mut header_bytes=[0;HEADER_SIZE];let mut footer_bytes=[0;FOOTER_SIZE];source.read_exact_at(0,&mut header_bytes).await?;source.read_exact_at(len-FOOTER_SIZE as u64,&mut footer_bytes).await?;
        let header=Header::parse(&header_bytes).await?;let footer=Footer::parse(&footer_bytes).await?;
        if footer.file_len!=len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"footer",offset:len-FOOTER_SIZE as u64,detail:"file_len differs from source".into()}).into());}
        let mut this=Self{source,limits:limits.clone(),superblock:Superblock{header,footer},manifest:None,symbols:Vec::new(),chunk_table:Vec::new()};
        let mut manifest_bytes=Vec::new();let(kind,consumed)=append_controlled_segment(&this.source,this.superblock.footer.manifest_offset,&mut manifest_bytes,None,limits,verification,control).await?;
        if kind!=semio_framework_replication_codec::KIND_MANIFEST||consumed!=this.superblock.footer.manifest_len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:this.superblock.footer.manifest_offset,detail:"manifest framing differs from footer".into()}).into());}
        let raw=parse_raw_manifest(&manifest_bytes).await?;
        if raw.symbols_span.len>0{
            let mut bytes=Vec::new();let(kind,consumed)=append_controlled_segment(&this.source,raw.symbols_span.offset,&mut bytes,None,limits,verification,control).await?;
            if kind!=semio_framework_replication_codec::KIND_SYMBOLS||consumed!=raw.symbols_span.len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symbols",offset:raw.symbols_span.offset,detail:"symbol framing differs from manifest".into()}).into());}
            let mut position=0;let count=read_varint_u64(&bytes,&mut position)?;
            if count>u64::from(limits.max_symbols){return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled symbol count exceeds source/limits"}).into());}
            if count>bytes.len() as u64{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"controlled symbol count exceeds source/limits"}).into());}
            this.symbols=controlled_vec(control,limits,usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol count exceeds address space"})?)?;control.begin_stage(count as usize).map_err(PackRefusal::ValueRefusal)?;
            for _ in 0..count{
                control.step().map_err(PackRefusal::ValueRefusal)?;let length=usize::try_from(read_varint_u64(&bytes,&mut position)?).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"symbol length exceeds address space"})?;
                let end=position.checked_add(length).filter(|end|*end<=bytes.len()).ok_or(PackRefusal::Truncated(position as u64))?;check_controlled_allocation(control,limits,length)?;
                let text=control.borrow_text(&bytes[position..end]).map_err(PackRefusal::ValueRefusal)?;this.symbols.push(control.copy_text(text).map_err(PackRefusal::ValueRefusal)?);position=end;
            }
            if position!=bytes.len(){return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symbols",offset:position as u64,detail:"trailing symbol data".into()}).into());}
        }
        if raw.chunk_table_span.len>0{
            let mut bytes=Vec::new();let(kind,consumed)=append_controlled_segment(&this.source,raw.chunk_table_span.offset,&mut bytes,None,limits,verification,control).await?;
            if kind!=semio_framework_replication_codec::KIND_CHUNK_TABLE||consumed!=raw.chunk_table_span.len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_table",offset:raw.chunk_table_span.offset,detail:"chunk framing differs from manifest".into()}).into());}
            let mut position=0;let count=read_varint_u64(&bytes,&mut position)?;
            if count>limits.max_items{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled chunk table exceeds source/limits"}).into());}
            if count>bytes.len() as u64/39{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"controlled chunk table exceeds source/limits"}).into());}
            this.chunk_table=controlled_vec(control,limits,usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk count exceeds address space"})?)?;control.begin_stage(count as usize).map_err(PackRefusal::ValueRefusal)?;
            for _ in 0..count{
                control.step().map_err(PackRefusal::ValueRefusal)?;let offset=read_varint_u64(&bytes,&mut position)?;let stored_len=read_varint_u64(&bytes,&mut position)?;let raw_len=read_varint_u64(&bytes,&mut position)?;
                if stored_len>limits.max_segment_len||raw_len>limits.max_segment_len{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled chunk length exceeds max_segment_len"}).into());}
                let end=position.checked_add(36).filter(|end|*end<=bytes.len()).ok_or(PackRefusal::Truncated(position as u64))?;let crc32=u32::from_le_bytes(bytes[position..position+4].try_into().unwrap());let mut blake3=[0;32];blake3.copy_from_slice(&bytes[position+4..end]);position=end;
                this.chunk_table.push(ChunkTableEntry{offset,stored_len,raw_len,crc32,blake3});
            }
            if position!=bytes.len(){return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_table",offset:position as u64,detail:"trailing chunk data".into()}).into());}
        }
        if raw.symbol_count!=this.symbols.len() as u64||raw.chunk_count!=this.chunk_table.len() as u64{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:0,detail:"declared catalog counts differ from contents".into()}).into());}
        let mut manifest=Manifest{schema_name:String::new(),schema_hash:raw.schema_hash,doc_span:raw.doc_span,doc_frame_count:raw.doc_frame_count,symbols_span:raw.symbols_span,chunk_table_span:raw.chunk_table_span,field_index_span:raw.field_index_span,uncompressed_body_len:raw.uncompressed_body_len,field_count:raw.field_count,chunk_count:raw.chunk_count,symbol_count:raw.symbol_count};
        if !this.symbols.is_empty(){let index=usize::try_from(raw.schema_symref).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"schema symbol exceeds address space"})?;let text=this.symbols.get(index).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:0,detail:"schema symbol out of range".into()})?;check_controlled_allocation(control,limits,text.len())?;manifest.schema_name=control.copy_text(text).map_err(PackRefusal::ValueRefusal)?;}
        else if raw.schema_symref!=0{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest",offset:0,detail:"schema symbol out of range".into()}).into());}
        this.manifest=Some(manifest);Ok(this)
    }

    /// 📄️ Concatenates directly into one admitted document allocation and controls each frame interior.
    pub async fn body_bytes_controlled(&self,verification:VerificationLevel,control:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,S::Error>{
        let manifest=self.manifest.as_ref().ok_or_else(||PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvariantViolated,what:"manifest",offset:0,detail:"controlled manifest not loaded"})?;
        let length=usize::try_from(manifest.uncompressed_body_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document exceeds address space"})?;
        let mut output=controlled_vec::<u8>(control,&self.limits,length)?;
        if manifest.doc_span.len>0{
            let frames=manifest.doc_frame_count.max(1);if frames>self.limits.max_items{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"document frame count exceeds source/limits"}).into());}
            if frames>manifest.doc_span.len/7{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"document frame count exceeds source/limits"}).into());}
            let mut offset=manifest.doc_span.offset;let end=offset.checked_add(manifest.doc_span.len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document span overflow"})?;
            for _ in 0..frames{let(kind,consumed)=append_controlled_segment(&self.source,offset,&mut output,Some(length),&self.limits,verification,control).await?;if kind!=semio_framework_replication_codec::KIND_DOCUMENT{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document",offset,detail:"unexpected document segment".into()}).into());}offset=offset.checked_add(consumed).filter(|offset|*offset<=end).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"document frame exceeds manifest span"})?;}
            if offset!=end{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document",offset,detail:"document frames differ from declared span".into()}).into());}
        }
        if output.len()!=length{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document",offset:manifest.doc_span.offset,detail:"decoded document differs from declared length".into()}).into());}
        if verification.checks_content_hash(){control.begin_stage(output.len()).map_err(PackRefusal::ValueRefusal)?;let mut hash=semio_framework_hash::Hasher::new();for page in output.chunks(4096){hash.update(page);control.advance(page.len()).map_err(PackRefusal::ValueRefusal)?;}if hash.finalize().as_bytes()!=&self.superblock.footer.content_hash.0{return Err((PackRefusal::ContentHashMismatch).into());}}
        Ok(output)
    }

    /// 1⃣ Level 1: parses and CRC-validates the header and footer only, and cross-checks
    /// the footer's `file_len` against the actual source length.
    pub async fn open_superblock(source: S, limits: &PackLimits) -> Result<Self, S::Error> {
        let len = source.len().await;
        if len > limits.max_file_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"source length exceeds max_file_len"}).into());
        }
        if len < FOOTER_SIZE as u64 {
            return Err((PackRefusal::Truncated(len)).into());
        }
        let mut header_bytes = [0u8; HEADER_SIZE];
        source.read_exact_at(0, &mut header_bytes).await?;
        let header = Header::parse(&header_bytes).await?;
        let mut footer_bytes = vec![0u8; FOOTER_SIZE];
        source.read_exact_at(len - FOOTER_SIZE as u64, &mut footer_bytes).await?;
        let footer = Footer::parse(&footer_bytes).await?;
        if footer.file_len != len {
            return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"footer", offset: len - FOOTER_SIZE as u64, detail: "file_len does not match actual source length".to_string() }).into());
        }
        Ok(Self { source, limits: limits.clone(), superblock: Superblock { header, footer }, manifest: None, symbols: Vec::new(), chunk_table: Vec::new() })
    }

    /// 2⃣ Level 2: `open_superblock` plus decoding the manifest, its symbol table (used
    /// to resolve `manifest().schema_name`), and the chunk table (if present).
    pub async fn open_manifest(source: S, limits: &PackLimits, verification: VerificationLevel) -> Result<Self, S::Error> {
        let mut this = Self::open_superblock(source, limits).await?;
        let verify_crc = verification.checks_crc();
        let manifest_seg = decode_segment_at(&this.source, this.superblock.footer.manifest_offset, &this.limits, verify_crc).await?;
        if manifest_seg.kind != semio_framework_replication_codec::KIND_MANIFEST {
            return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest", offset: this.superblock.footer.manifest_offset, detail: "expected KIND_MANIFEST segment".to_string() }).into());
        }
        if manifest_seg.consumed != this.superblock.footer.manifest_len {
            return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"manifest", offset: this.superblock.footer.manifest_offset, detail: "manifest_len mismatch".to_string() }).into());
        }
        let raw = parse_raw_manifest(&manifest_seg.payload).await?;
        let symbols = if raw.symbols_span.len > 0 {
            let seg = decode_segment_at(&this.source, raw.symbols_span.offset, &this.limits, verify_crc).await?;
            if seg.kind != semio_framework_replication_codec::KIND_SYMBOLS {
                return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symbols", offset: raw.symbols_span.offset, detail: "expected KIND_SYMBOLS segment".to_string() }).into());
            }
            decode_symbols(&seg.payload, &this.limits).await?
        } else {
            Vec::new()
        };
        let chunk_table = if raw.chunk_table_span.len > 0 {
            let seg = decode_segment_at(&this.source, raw.chunk_table_span.offset, &this.limits, verify_crc).await?;
            if seg.kind != semio_framework_replication_codec::KIND_CHUNK_TABLE {
                return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_table", offset: raw.chunk_table_span.offset, detail: "expected KIND_CHUNK_TABLE segment".to_string() }).into());
            }
            decode_chunk_table(&seg.payload, &this.limits).await?
        } else {
            Vec::new()
        };
        let manifest = resolve_manifest(&raw, &symbols)?;
        this.manifest = Some(manifest);
        this.symbols = symbols;
        this.chunk_table = chunk_table;
        Ok(this)
    }

    pub fn superblock(&self) -> &Superblock {
        &self.superblock
    }

    pub fn manifest(&self) -> Option<&Manifest> {
        self.manifest.as_ref()
    }

    /// 🧮️ Counts retained catalog slots and UTF-8 bytes before constructing document values.
    pub fn owned_catalog_bytes(&self) -> Result<u64, PackRefusal> {
        let symbol_slots = (self.symbols.capacity() as u64).checked_mul(size_of::<String>() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog symbol allocation overflow"})?;
        let chunk_slots = (self.chunk_table.capacity() as u64).checked_mul(size_of::<ChunkTableEntry>() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog chunk allocation overflow"})?;
        let mut bytes = symbol_slots.checked_add(chunk_slots).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog allocation overflow"})?;
        for symbol in &self.symbols {
            bytes = bytes.checked_add(symbol.capacity() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog text allocation overflow"})?;
        }
        if let Some(manifest) = &self.manifest {
            bytes = bytes.checked_add(manifest.schema_name.capacity() as u64).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog schema allocation overflow"})?;
        }
        Ok(bytes)
    }

    /// 🔤️ Resolves a symref (index into the symbol table loaded by `open_manifest`).
    pub fn symbol(&self, symref: u64) -> Result<&str, PackRefusal> {
        self.symbols.get(symref as usize).map(String::as_str).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"symref", offset: symref, detail: "symref out of range".to_string() })
    }

    pub fn chunk_count(&self) -> u64 {
        self.chunk_table.len() as u64
    }

    /// 📏️ The `(offset, stored_len)` range of a chunk's on-disk (possibly compressed)
    /// payload bytes — suitable for a range-fetch (see `pack_http`) without decoding.
    pub fn chunk_range(&self, id: ChunkId) -> Result<ByteRange, PackRefusal> {
        self.chunk_table.get(id.0 as usize).map(|entry| ByteRange { offset: entry.offset, len: entry.stored_len }).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })
    }

    /// 📏️ Declared decoded bytes let callers admit ownership before reading or decompressing a chunk.
    pub fn chunk_decoded_len(&self,id:ChunkId)->Result<u64,PackRefusal>{self.chunk_table.get(id.0 as usize).map(|entry|entry.raw_len).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id",offset:id.0 as u64,detail:"unknown chunk id".into()})}

    /// 🧩️ Opens an identity chunk without allocating or materializing its payload.
    pub fn identity_chunk_cursor(&self, id: ChunkId, verification: VerificationLevel) -> Result<PackIdentityChunkCursor<'_, S>, S::Error> {
        let entry = self.chunk_table.get(id.0 as usize).cloned().ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })?;
        if entry.stored_len != entry.raw_len {
            return Err((PackRefusal::Malformed { kind:ValueRefusalKind::UnsupportedOwner, what: "chunk", offset: entry.offset, detail: "retained fragment cursor requires an identity chunk".to_string() }).into());
        }
        if entry.raw_len > self.limits.max_segment_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"chunk length exceeds max_segment_len"}).into());
        }
        Ok(PackIdentityChunkCursor { source: &self.source, entry, verification, offset: 0, crc: semio_framework_replication_codec::Crc32cCursor::new(), hash: semio_framework_hash::Hasher::new(), terminal: false })
    }

    /// 🧩️ Appends one verified chunk into a previously admitted output buffer with interior source and inflation control.
    pub async fn append_chunk_controlled(&self,id:ChunkId,verification:VerificationLevel,output:&mut Vec<u8>,control:&mut NativeDecodeControl<'_>)->Result<(),S::Error>{
        control.checkpoint().map_err(PackRefusal::ValueRefusal)?;let entry=self.chunk_table.get(id.0 as usize).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id",offset:id.0 as u64,detail:"unknown chunk id".into()})?;
        if entry.stored_len>self.limits.max_segment_len||entry.raw_len>self.limits.max_segment_len{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled chunk exceeds max_segment_len"}).into());}let end=entry.offset.checked_add(entry.stored_len).filter(|end|*end<=self.superblock.footer.file_len).ok_or(PackRefusal::Truncated(entry.offset))?;if end>self.source.len().await{return Err((PackRefusal::Truncated(entry.offset)).into());}
        if verification.checks_crc()&&controlled_crc(&self.source,entry.offset,entry.stored_len,control).await?!=entry.crc32{return Err((PackRefusal::ChecksumMismatch{segment:"chunk",offset:entry.offset}).into());}
        let start=output.len();let codec=if entry.stored_len==entry.raw_len{CodecId(0)}else if self.superblock.header.required_flags&REQUIRED_COMPRESSED!=0{CodecId(1)}else{CodecId(0)};append_controlled_payload(&self.source,entry.offset,entry.stored_len,entry.raw_len,codec,output,&self.limits,control).await?;
        if verification.checks_content_hash(){let bytes=&output[start..];control.begin_stage(bytes.len()).map_err(PackRefusal::ValueRefusal)?;let mut hash=semio_framework_hash::Hasher::new();for page in bytes.chunks(4096){hash.update(page);control.advance(page.len()).map_err(PackRefusal::ValueRefusal)?;}if hash.finalize().as_bytes()!=&entry.blake3{return Err((PackRefusal::ContentHashMismatch).into());}}
        Ok(())
    }

    /// 3⃣ Level 3: reads, optionally CRC-verifies (`Standard`+) and decompresses one
    /// chunk; at `Full` also verifies its blake3 content hash.
    pub async fn read_chunk(&self, id: ChunkId, verification: VerificationLevel) -> Result<Vec<u8>, S::Error> {
        let entry = self.chunk_table.get(id.0 as usize).ok_or(PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"chunk_id", offset: id.0 as u64, detail: "unknown chunk id".to_string() })?;
        if entry.stored_len > self.limits.max_segment_len || entry.raw_len > self.limits.max_segment_len {
            return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"chunk length exceeds max_segment_len"}).into());
        }
        let total_len = self.source.len().await;
        let end = entry.offset.checked_add(entry.stored_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"chunk range overflow"})?;
        if end > total_len {
            return Err((PackRefusal::Truncated(entry.offset)).into());
        }
        let mut stored = vec![0u8; entry.stored_len as usize];
        self.source.read_exact_at(entry.offset, &mut stored).await?;
        if verification.checks_crc() {
            let computed = crc32c(&stored);
            if computed != entry.crc32 {
                return Err((PackRefusal::ChecksumMismatch { segment: "chunk", offset: entry.offset }).into());
            }
        }
        let raw = if entry.stored_len == entry.raw_len {
            stored
        } else {
            let codec = if self.superblock.header.required_flags & REQUIRED_COMPRESSED != 0 { CodecId(1) } else { CodecId(0) };
            codec_decompress(codec, &stored, entry.raw_len, self.limits.max_segment_len).await?
        };
        if verification.checks_content_hash() {
            let hash = semio_framework_hash::hash(&raw);
            if hash.as_bytes() != &entry.blake3 {
                return Err((PackRefusal::ContentHashMismatch).into());
            }
        }
        Ok(raw)
    }

    /// 📄️ Level 3: reads and concatenates the `doc_frame_count` `KIND_DOCUMENT` segments
    /// starting at `manifest().doc_span.offset`; at `Full` also verifies the result's blake3
    /// hash against the footer's `content_hash`.
    pub async fn body_bytes(&self, verification: VerificationLevel) -> Result<Vec<u8>, S::Error> {
        let manifest = self.manifest.as_ref().ok_or_else(|| PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvariantViolated,what:"manifest",offset:0,detail:"manifest not loaded; call open_manifest first"})?;
        let catalog=self.owned_catalog_bytes()?;
        let remaining=self.limits.max_total_alloc.checked_sub(catalog).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"catalog exceeds max_total_alloc"})?;
        if manifest.uncompressed_body_len>remaining{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document body exceeds max_total_alloc"}).into());}
        let body_len=usize::try_from(manifest.uncompressed_body_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document body exceeds address space"})?;
        let mut out = Vec::new();
        check_allocation_layout::<u8>(body_len,"document body allocation failed")?;
        out.try_reserve_exact(body_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"document body allocation failed"})?;
        let mut frame_limits=self.limits.clone();frame_limits.max_total_alloc=remaining-manifest.uncompressed_body_len;
        if manifest.doc_span.len > 0 {
            let mut offset = manifest.doc_span.offset;
            let frames = manifest.doc_frame_count.max(1);
            for _ in 0..frames {
                let seg = decode_segment_at(&self.source, offset, &frame_limits, verification.checks_crc()).await?;
                if seg.kind != semio_framework_replication_codec::KIND_DOCUMENT {
                    return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document", offset, detail: "expected KIND_DOCUMENT segment".to_string() }).into());
                }
                if seg.payload.len()>body_len-out.len(){return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"document frames exceed declared body length"}).into());}
                out.extend_from_slice(&seg.payload);
                offset=offset.checked_add(seg.consumed).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"document frame offset overflow"})?;
            }
        }
        if out.len()!=body_len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"document",offset:manifest.doc_span.offset,detail:"decoded body length differs from manifest".into()}).into());}
        if verification.checks_content_hash() {
            let hash = semio_framework_hash::hash(&out);
            if hash.as_bytes() != &self.superblock.footer.content_hash.0 {
                return Err((PackRefusal::ContentHashMismatch).into());
            }
        }
        Ok(out)
    }

    /// #⃣ The footer's content hash — no decode needed.
    pub fn content_hash(&self) -> ContentHash {
        self.superblock.footer.content_hash
    }
}

/// 🔎️ Standalone helper (used by `crate::content_hash`) that reads and parses only the
/// last `FOOTER_SIZE` bytes of `source`, without touching the header or any segment.
pub async fn read_footer_only<S: PackSource>(source: &S) -> Result<Footer, S::Error> {
    let len = source.len().await;
    if len < FOOTER_SIZE as u64 {
        return Err((PackRefusal::Truncated(len)).into());
    }
    let mut buf = vec![0u8; FOOTER_SIZE];
    source.read_exact_at(len - FOOTER_SIZE as u64, &mut buf).await?;
    Footer::parse(&buf).await.map_err(S::Error::from)
}

//#endregion 🔖️Reader

//#region 🔖️Recover
/// 🩹️ What a forward-scan recovery pass managed to salvage.
#[derive(Clone, Debug)]
pub struct RecoveryReport {
    pub segments_recovered: u64,
    pub bytes_recovered: u64,
    pub manifest: Option<Manifest>,
}

/// 🩺️ Forward-scans from byte `HEADER_SIZE` (right after the header), CRC-validating and
/// accumulating one segment at a time until the first invalid/truncated segment, a `KIND_END`
/// segment, or end of file — whichever comes first. Unrecognized segment kinds are accumulated,
/// not rejected. If a `KIND_MANIFEST` and (when its `schema_name` is non-empty) a matching
/// `KIND_SYMBOLS` segment were both recovered, the manifest is resolved and returned too. Used
/// when the footer itself fails to parse/validate.
pub async fn recover<S: PackSource>(source: &S, limits: &PackLimits) -> Result<RecoveryReport, S::Error> {
    let len = source.len().await;
    if len < HEADER_SIZE as u64 {
        return Err((PackRefusal::Truncated(len)).into());
    }
    let mut header_bytes = [0u8; HEADER_SIZE];
    source.read_exact_at(0, &mut header_bytes).await?;
    Header::parse(&header_bytes).await?;

    let mut offset = HEADER_SIZE as u64;
    let mut segments_recovered = 0u64;
    let mut bytes_recovered = 0u64;
    let mut found: Vec<(u8, Vec<u8>)> = Vec::new();
    while offset < len {
        match decode_segment_at(source, offset, limits, true).await {
            Ok(seg) => {
                segments_recovered += 1;
                bytes_recovered += seg.consumed;
                let is_end = seg.kind == semio_framework_replication_codec::KIND_END;
                offset += seg.consumed;
                found.push((seg.kind, seg.payload));
                if is_end {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    let mut symbols = Vec::new();
    if let Some((_, payload)) = found.iter().find(|(kind, _)| *kind == semio_framework_replication_codec::KIND_SYMBOLS) {
        if let Ok(decoded) = decode_symbols(payload, limits).await {
            symbols = decoded;
        }
    }
    let mut manifest = None;
    if let Some((_, payload)) = found.iter().find(|(kind, _)| *kind == semio_framework_replication_codec::KIND_MANIFEST) {
        if let Ok(raw) = parse_raw_manifest(payload).await {
            manifest = resolve_manifest(&raw, &symbols).ok();
        }
    }

    Ok(RecoveryReport { segments_recovered, bytes_recovered, manifest })
}
//#endregion 🔖️Recover

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔣️absolute-varint/🦀️.rs"]
mod absolute_varint_tests;
