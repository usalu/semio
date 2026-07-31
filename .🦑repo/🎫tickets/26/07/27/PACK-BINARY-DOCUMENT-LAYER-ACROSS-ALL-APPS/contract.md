# Pack Crate Family — Shared API Contract (Wave 0)

Every wave-0 agent implements exactly one crate. This contract is the binding cross-crate
interface — deviate only where your crate-specific section says "your choice", otherwise an
upstream/downstream crate written by a different agent will fail to compile against yours.

Full design rationale: `/Users/ueli/.claude/plans/introduce-a-new-layer-valiant-diffie.md`.
Read it first for context, then implement strictly against the signatures below.

Repo conventions (non-negotiable): single-file `lib.rs` at `pack/<part>/rs/lib.rs` with
`[lib] path = "lib.rs"`; `edition = "2021"`, `rust-version = "1.88"`; `[lints] workspace = true`;
code in `//#region 🔖Name` / `//#endregion 🔖Name` blocks; every doc comment starts with an
emoji (`/// 📦 ...` or `//! 📦 ...`); tests inline in `//#region 🧪Tests` at the bottom of
`lib.rs`, `mod tests { ... mod quick { } mod long { } mod exhaustive { } }` (only add level
submodules if you actually have slow/exhaustive tests — plain `#[test]` fns for everything else);
no `unsafe`; no `std::io::Error` in any public signature (must stay wasm32-clean in
pack_core/format/value/index; pack_io/async/http may use `std::io`/tokio internally behind
`cfg(not(target_arch = "wasm32"))` or a cargo feature, but their *public trait* signatures must
still avoid leaking `std::io::Error` — wrap it into `PackError::Io(String)`).

Package names are `pack_core`, `pack_format`, `pack_value`, `pack_io`, `pack_async`,
`pack_http`, `pack_index`, `pack` (facade), `pack_testkit`, `pack_cli`. Path deps between them
are plain relative paths, e.g. `pack_core = { path = "../../core/rs" }`.

---

## pack_core (`pack/core/rs`, package `pack_core`)

Deps: `thiserror`. No other deps. Must compile for `wasm32-unknown-unknown`.

```rust
//#region 🔖Ids
pub struct ContentHash(pub [u8; 32]);        // impl Display as lowercase hex, impl Debug, Clone, Copy, PartialEq, Eq, Hash
pub struct ChunkId(pub u32);                 // Clone, Copy, PartialEq, Eq, Hash, Debug
pub struct SegmentKind(pub u8);              // Clone, Copy, PartialEq, Eq, Debug — see 🔖SegmentKinds below
pub struct CodecId(pub u8);                  // Clone, Copy, PartialEq, Eq, Debug — 0 = none, 1 = deflate, 2/3 reserved
pub struct ByteRange { pub offset: u64, pub len: u64 }  // Clone, Copy, PartialEq, Eq, Debug
//#endregion

//#region 🔖SegmentKinds
// Associated consts on SegmentKind (or a plain module of `pub const` u8s re-exported as SegmentKind) — pick one, but name them exactly:
// KIND_END = 0x00, KIND_MANIFEST = 0x01, KIND_SCHEMA = 0x02, KIND_SYMBOLS = 0x03,
// KIND_DOCUMENT = 0x04, KIND_CHUNK = 0x05, KIND_CHUNK_TABLE = 0x06, KIND_SNAPSHOT = 0x07,
// KIND_FIELD_INDEX = 0x08, KIND_PADDING = 0x7F. Extension range 0x40..=0x7E is caller-defined.
//#endregion

//#region 🔖Errors
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum PackError {
    #[error("bad magic")] BadMagic,
    #[error("unsupported version {major}.{minor}")] UnsupportedVersion { major: u16, minor: u16 },
    #[error("unknown required feature bits {0:#x}")] UnknownRequiredFlags(u32),
    #[error("truncated at offset {0}")] Truncated(u64),
    #[error("checksum mismatch in {segment} at offset {offset}")] ChecksumMismatch { segment: &'static str, offset: u64 },
    #[error("content hash mismatch")] ContentHashMismatch,
    #[error("limit exceeded: {0}")] LimitExceeded(&'static str),
    #[error("malformed {what} at offset {offset}: {detail}")] Malformed { what: &'static str, offset: u64, detail: String },
    #[error("non-canonical encoding: {0}")] NonCanonical(&'static str),
    #[error("unsupported codec {0}")] UnsupportedCodec(u8),
    #[error("schema error: {0}")] Schema(String),
    #[error("io error: {0}")] Io(String),
}
//#endregion

//#region 🔖Limits
#[derive(Clone, Debug)]
pub struct PackLimits {
    pub max_file_len: u64,       // default 16 * 1024 * 1024 * 1024 (16 GiB)
    pub max_segment_len: u64,    // default 256 * 1024 * 1024 (256 MiB)
    pub max_symbols: u32,        // default 1_000_000
    pub max_depth: u16,          // default 64
    pub max_items: u64,          // default 64_000_000
    pub max_total_alloc: u64,    // default 4 * 1024 * 1024 * 1024 (4 GiB)
}
impl Default for PackLimits { fn default() -> Self { /* the values above */ } }
//#endregion

//#region 🔖Varint
// LEB128 unsigned varint (u64) and zigzag-encoded signed varint (i64). Max 10 bytes.
pub fn write_varint_u64(out: &mut Vec<u8>, value: u64);
pub fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, PackError>;   // errors Malformed on overlong (>10 bytes) or truncated input
pub fn write_varint_i64(out: &mut Vec<u8>, value: i64);   // zigzag then write_varint_u64
pub fn read_varint_i64(bytes: &[u8], pos: &mut usize) -> Result<i64, PackError>;
// Canonical/minimal-form check: re-encoding the decoded value must reproduce the exact same bytes.
pub fn is_minimal_varint(bytes: &[u8]) -> bool;
//#endregion

//#region 🔖Bytes
pub struct ByteReader<'a> { /* wraps &'a [u8] + cursor */ }
impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self;
    pub fn remaining(&self) -> usize;
    pub fn position(&self) -> usize;
    pub fn read_u8(&mut self) -> Result<u8, PackError>;
    pub fn read_u16_le(&mut self) -> Result<u16, PackError>;
    pub fn read_u32_le(&mut self) -> Result<u32, PackError>;
    pub fn read_u64_le(&mut self) -> Result<u64, PackError>;
    pub fn read_f64_le(&mut self) -> Result<f64, PackError>;
    pub fn read_varint_u64(&mut self) -> Result<u64, PackError>;
    pub fn read_varint_i64(&mut self) -> Result<i64, PackError>;
    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], PackError>;   // bounds-checked, never panics
    pub fn read_array32(&mut self) -> Result<[u8; 32], PackError>;
}
pub struct ByteWriter { /* wraps Vec<u8> */ }
impl ByteWriter {
    pub fn new() -> Self;
    pub fn into_bytes(self) -> Vec<u8>;
    pub fn write_u8(&mut self, v: u8);
    pub fn write_u16_le(&mut self, v: u16);
    pub fn write_u32_le(&mut self, v: u32);
    pub fn write_u64_le(&mut self, v: u64);
    pub fn write_f64_le(&mut self, v: f64);
    pub fn write_varint_u64(&mut self, v: u64);
    pub fn write_varint_i64(&mut self, v: i64);
    pub fn write_bytes(&mut self, bytes: &[u8]);
}
//#endregion

//#region 🔖Crc
pub fn crc32c(bytes: &[u8]) -> u32;   // Castagnoli polynomial, in-crate 256-entry table, no dependency
//#endregion

//#region 🔖Source
pub trait PackSource {
    fn len(&self) -> u64;
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError>;
    fn read_exact_at(&self, offset: u64, buf: &mut [u8]) -> Result<(), PackError> {
        // default impl: loop read_at until buf is full or error/short-read -> Truncated
    }
}
impl PackSource for &[u8] { /* offset/len bounds-checked slice reads */ }
impl PackSource for Vec<u8> { /* delegates to &[u8] */ }

pub trait PackSink {
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), PackError>;
    fn position(&self) -> u64;
    fn flush(&mut self) -> Result<(), PackError> { Ok(()) }
}
impl PackSink for Vec<u8> { /* position = self.len() as u64, write_all = self.extend_from_slice */ }
//#endregion

//#region 🔖Codec
pub trait CompressionCodec {
    fn id(&self) -> CodecId;
    fn compress(&self, raw: &[u8]) -> Result<Vec<u8>, PackError>;
    fn decompress(&self, stored: &[u8], raw_len: u64, limit: u64) -> Result<Vec<u8>, PackError>;  // must reject if raw_len > limit BEFORE allocating
}
pub struct NoCompression;
impl CompressionCodec for NoCompression { /* id() = CodecId(0); compress/decompress = identity (respecting limit) */ }
//#endregion
```

Your crate is the foundation everyone else path-deps on. Every public fn must validate lengths
against `PackLimits` **before** allocating (`Vec::with_capacity`, etc.) — this is load-bearing
for the corruption-hardening tests other crates will write against your primitives.

---

## pack_format (`pack/format/rs`, package `pack_format`)

Deps: `pack_core = { path = "../../core/rs" }`, `blake3`, `miniz_oxide` (behind cargo feature
`deflate`, **default-on** in `[features] default = ["deflate"]`).

Container byte layout — implement EXACTLY this (byte-for-byte; other crates and the CLI will
round-trip against these constants):

- **Magic**: `[0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A]` (8 bytes).
- **Footer magic**: `b"SPKFOOT1"` (8 bytes).
- **Header (32 bytes)**: offset 0 magic(8), 8 version_major u16 LE, 10 version_minor u16 LE,
  12 required_flags u32 LE, 16 optional_flags u32 LE, 20 header_crc32 u32 LE (CRC-32C over
  bytes 0..20), 24 reserved 8 bytes (must be zero on write, ignored on read).
- **Required flag bits** (u32): `bit 0 = COMPRESSED`, `bit 1 = CHUNKED`, `bit 2 = ENCRYPTED`
  (reserved, never set by this crate), `bit 3 = FOOTER_CHAIN` (reserved). Reader MUST return
  `PackError::UnknownRequiredFlags` if any bit outside 0..=3 is set in `required_flags`.
- **Optional flag bits**: `bit 0 = CANONICAL`, `bit 1 = STREAMED`, `bit 2 = HAS_SCHEMA_SEGMENT`.
  Unknown optional bits are ignored.
- **Segment framing**: `kind: u8, flags: u8 (bit0=compressed, bits1..3=codec id), seg_len: varint u64,
  [raw_len: varint u64 present iff flags.bit0], payload: seg_len bytes, crc32: u32 LE` (CRC-32C
  over `kind..payload` inclusive, i.e. everything except the trailing crc32 field itself).
  Validate `seg_len`/`raw_len` against `PackLimits::max_segment_len` and against remaining input
  length BEFORE allocating a buffer for payload/decompression.
- **Footer (80 bytes, at end of file)**: `magic(8)="SPKFOOT1", version_major u16, version_minor u16,
  required_flags u32 (copy), manifest_offset u64, manifest_len u64, file_len u64,
  content_hash [u8;32] (blake3), prev_footer_offset u64 (0 = none), footer_crc32 u32` — that's
  8+2+2+4+8+8+8+32+8 = 80 bytes exactly; footer_crc32 is CRC-32C over the preceding 76 bytes.
- **Manifest segment (kind KIND_MANIFEST)**: `schema_name: symref varint, schema_hash: [u8;32],
  doc_span: (offset varint, len varint), doc_frame_count: varint, symbols_span: (offset varint,
  len varint), chunk_table_span: (offset varint, len varint) (0,0 if absent), field_index_span:
  (offset varint, len varint) (0,0 if absent), uncompressed_body_len: varint, field_count: varint,
  chunk_count: varint, symbol_count: varint`. Trailing bytes after these fields are ignored on
  read (additive-evolution slot) — do not error on extra bytes.
- **Symbols segment (kind KIND_SYMBOLS)**: `count: varint`, then `count ×
  (len: varint, utf8 bytes)`. A `symref` elsewhere is `varint` = index into this table.
- **Chunk table (kind KIND_CHUNK_TABLE)**: `count: varint`, then `count ×
  (offset: varint (absolute file offset of the Chunk segment's payload start),
  stored_len: varint, raw_len: varint, crc32: u32 LE, blake3: [u8;32])`.
- **Chunk segment (kind KIND_CHUNK)**: payload is raw (optionally compressed per its own
  segment flags) blob bytes — framed exactly like any other segment.

```rust
//#region 🔖Header
pub struct Header { pub version_major: u16, pub version_minor: u16, pub required_flags: u32, pub optional_flags: u32 }
pub const REQUIRED_COMPRESSED: u32 = 1 << 0;
pub const REQUIRED_CHUNKED: u32 = 1 << 1;
pub const REQUIRED_ENCRYPTED: u32 = 1 << 2;
pub const REQUIRED_FOOTER_CHAIN: u32 = 1 << 3;
pub const OPTIONAL_CANONICAL: u32 = 1 << 0;
pub const OPTIONAL_STREAMED: u32 = 1 << 1;
pub const OPTIONAL_HAS_SCHEMA: u32 = 1 << 2;
//#endregion

//#region 🔖Footer
pub struct Footer { pub version_major: u16, pub version_minor: u16, pub required_flags: u32,
    pub manifest_offset: u64, pub manifest_len: u64, pub file_len: u64,
    pub content_hash: pack_core::ContentHash, pub prev_footer_offset: u64 }
//#endregion

//#region 🔖Manifest
pub struct Manifest { pub schema_name: String, pub schema_hash: [u8; 32],
    pub doc_span: pack_core::ByteRange, pub doc_frame_count: u64,
    pub symbols_span: pack_core::ByteRange, pub chunk_table_span: pack_core::ByteRange,
    pub field_index_span: pack_core::ByteRange, pub uncompressed_body_len: u64,
    pub field_count: u64, pub chunk_count: u64, pub symbol_count: u64 }
//#endregion

//#region 🔖Verify
pub enum VerificationLevel { Trusted, Standard, Full }   // impl Default -> Standard
//#endregion

//#region 🔖Writer
pub struct WriteOptions { pub required_flags: u32, pub optional_flags: u32, pub codec: pack_core::CodecId }
pub struct PackWriter<S: pack_core::PackSink> { /* fields private */ }
impl<S: pack_core::PackSink> PackWriter<S> {
    pub fn begin(sink: S, options: &WriteOptions) -> Result<Self, pack_core::PackError>;   // writes Header
    pub fn write_segment(&mut self, kind: u8, payload: &[u8]) -> Result<(), pack_core::PackError>;   // frames + compresses per options.codec + crc's + writes
    pub fn write_chunk(&mut self, payload: &[u8]) -> Result<pack_core::ChunkId, pack_core::PackError>;  // writes a Chunk segment, records offset/len/crc/blake3 internally for the chunk table
    pub fn finish(self, manifest: &Manifest) -> Result<S, pack_core::PackError>;   // writes ChunkTable (if any chunks), Manifest, End, Footer; returns the sink
}
//#endregion

//#region 🔖Reader
pub struct Superblock { pub header: Header, pub footer: Footer }
pub struct PackFile<S: pack_core::PackSource> { /* fields private */ }
impl<S: pack_core::PackSource> PackFile<S> {
    pub fn open_superblock(source: S, limits: &pack_core::PackLimits) -> Result<Self, pack_core::PackError>;   // open level 1: header + footer only
    pub fn open_manifest(source: S, limits: &pack_core::PackLimits, verification: VerificationLevel) -> Result<Self, pack_core::PackError>;  // level 2: + manifest + symbols
    pub fn superblock(&self) -> &Superblock;
    pub fn manifest(&self) -> Option<&Manifest>;
    pub fn symbol(&self, symref: u64) -> Result<&str, pack_core::PackError>;
    pub fn chunk_count(&self) -> u64;
    pub fn chunk_range(&self, id: pack_core::ChunkId) -> Result<pack_core::ByteRange, pack_core::PackError>;
    pub fn read_chunk(&self, id: pack_core::ChunkId, verification: VerificationLevel) -> Result<Vec<u8>, pack_core::PackError>;
    pub fn body_bytes(&self, verification: VerificationLevel) -> Result<Vec<u8>, pack_core::PackError>;   // level 3: concatenated uncompressed Document segment payload(s)
    pub fn content_hash(&self) -> pack_core::ContentHash;   // from footer, no decode needed
}
// Standalone helper used by pack::content_hash() — reads only the last 80 bytes.
pub fn read_footer_only<S: pack_core::PackSource>(source: &S) -> Result<Footer, pack_core::PackError>;
//#endregion

//#region 🔖Recover
// Forward-scans from byte 32 (after header) reading valid crc'd segments until the first
// invalid/truncated segment; used when the footer fails validation. Returns what was recoverable.
pub struct RecoveryReport { pub segments_recovered: u64, pub bytes_recovered: u64, pub manifest: Option<Manifest> }
pub fn recover<S: pack_core::PackSource>(source: &S, limits: &pack_core::PackLimits) -> Result<RecoveryReport, pack_core::PackError>;
//#endregion

//#region 🔖Deflate
#[cfg(feature = "deflate")]
pub struct DeflateCodec;
#[cfg(feature = "deflate")]
impl pack_core::CompressionCodec for DeflateCodec { /* id() = CodecId(1); miniz_oxide::deflate::compress_to_vec / inflate::decompress_to_vec_with_limit */ }
//#endregion
```

`content_hash` in the footer = blake3 over the **uncompressed concatenated Document segment
payload bytes** (not the whole file, not compressed bytes). Compression choice must never
change a document's identity.

---

## pack_value (`pack/value/rs`, package `pack_value`)

Deps: `pack_core = { path = "../../core/rs" }`, `pack_format = { path = "../../format/rs" }`,
`dsl_schema = { path = "../../../dsl/schema/rs" }`.

Read `/Users/ueli/Documents/semio/dsl/schema/rs/lib.rs` first for the exact current shape of
`Shape`, `FieldSpec`, `RecordSpec`, `FieldValue`, `RecordValue`, `DslValue`, `WireValue` — use
only these stable, already-merged types (do not reference anything you find mid-edit/uncommitted).

Wire tags (1 byte each, self-describing so unknown fields still decode):

```
0x00 Absent          0x0B Tuple   (arity varint, values*)
0x01 False            0x0C List    (len varint, values*)
0x02 True              0x0D Record  (field_count varint, (field_id varint, value)*)
0x03 Int   (zigzag varint)     0x0E Block   (inner value)
0x04 UInt  (varint)            0x0F Statements (count varint, (keyword symref, Record-payload)*)
0x05 F64   (8 bytes LE)        0x10 Map     (count varint, (key, value)*)
0x06 Str   (symref varint)     0x11 Value   (DslValue, recursively these same tags; objects as 0x10 with inline-string keys)
0x07 StrInline (len varint, utf8 bytes)   0x12 Null
0x08 Bytes (len varint, raw bytes)         0x13 Wire (presence bitmask u8, from{...}, to{...}?, props Value)
0x09 BytesChunked (count varint, chunk_id varint *count)   0x14 TableSoA (see below)
0x0A Enum  (ordinal varint)                0x15 PackedF64 (count varint, count*8 bytes LE)
                                            0x16 PackedVarint (count varint, count zigzag varints back to back)
```

Map to `Shape`/`FieldValue` variants (read the actual enum definitions and match every variant
— this list is illustrative, not exhaustive): `Bool`→0x01/0x02, `Int`→0x03, `UInt`→0x04,
`Float`→0x05, `Text`→0x06 (interned) or 0x07 (inline, per canonical rule below), `Bytes64`→0x08
or 0x09 (chunked when `len >= chunk_threshold`, default 256 KiB, via the `PackWriter` you were
handed), `Enum`→0x0A using the ordinal already carried by `FieldValue::Enum`, homogeneous
numeric `Tuple`/`List`→0x15/0x16 packed forms when eligible else 0x0B/0x0C, `Record`→0x0D keyed
by `FieldSpec.id`, `Block`→0x0E, `Statements`→0x0F, `Map`→0x10, `Value`→0x11, `Wire`→0x13,
`Table`→0x14 (TableSoA, below).

**TableSoA (0x14)**, encodes a `List` of `Record` values under `Shape::Table`:
```
row_count: varint, col_count: varint,
per column: field_id: varint, presence: u8 (0=dense, 1=bitmap follows: ceil(row_count/8) bytes),
            elem_tag: u8, packed column payload (F64: rows*8 bytes; Int/UInt/Enum: back-to-back
            varints; Str: back-to-back symrefs; Bool: bitmap; else: back-to-back self-describing
            values as fallback)
```
Columns sorted by field_id ascending in canonical mode.

```rust
//#region 🔖Options
pub struct EncodeOptions { pub canonical: bool, pub codec: pack_core::CodecId,
    pub chunk_threshold: u64, pub chunk_size: u64, pub frame_size: u64,
    pub preserve_unknown: bool, pub limits: pack_core::PackLimits }
impl Default for EncodeOptions { /* canonical: true, codec: CodecId(1) /*deflate*/, chunk_threshold: 256*1024,
    chunk_size: 1024*1024, frame_size: 1024*1024, preserve_unknown: true, limits: default */ }

pub struct DecodeOptions { pub verification: pack_format::VerificationLevel, pub preserve_unknown: bool,
    pub limits: pack_core::PackLimits }
impl Default for DecodeOptions { /* verification: Standard, preserve_unknown: true, limits: default */ }
//#endregion

//#region 🔖SchemaHash
pub fn schema_hash(spec: &dsl_schema::RecordSpec) -> [u8; 32];  // blake3 over a canonical serialization of the spec's (field id, key, shape-tag) tuples sorted by id
//#endregion

//#region 🔖DecodeReport
pub struct DecodeReport { pub unknown_field_ids: Vec<u16>, pub unknown_segments: Vec<u8>,
    pub schema_drift: bool, pub verified: pack_format::VerificationLevel }
//#endregion

//#region 🔖Document
// The single entry point pack_core/pack_format users and the facade call.
pub fn encode_document(spec: &dsl_schema::RecordSpec, record: &dsl_schema::RecordValue, options: &EncodeOptions)
    -> Result<Vec<u8>, pack_core::PackError>;
pub fn decode_document(bytes: &[u8], spec: &dsl_schema::RecordSpec, options: &DecodeOptions)
    -> Result<(dsl_schema::RecordValue, DecodeReport), pack_core::PackError>;
//#endregion
```

**Canonical mode rules** (when `options.canonical == true`, the default): record fields sorted
ascending by `field_id`; `FieldValue::Absent`/absent optionals omitted entirely (never encoded);
map/object entries sorted by key bytes; minimal-length varints only; f64 always 8-byte LE with
`-0.0` normalized to `0.0` and any NaN normalized to bit pattern `0x7ff8_0000_0000_0000`; a
string is interned into Symbols iff `len <= 128` OR it occurs `>= 2` times in the document,
otherwise encoded inline (0x07) — this is a deterministic function of the document, compute it
in a pre-pass; numeric-homogeneous Tuple/List always use the packed forms (0x15/0x16) when
every element is eligible. **LAW**: `encode_document` must be a pure function of
`(spec, record)` — byte-identical output across repeated calls regardless of `HashMap` iteration
order inside `RecordValue.fields` (sort before you iterate, always).

**Unknown-field preservation**: `decode_document` must return every field found in the wire
data, including field ids absent from `spec` (decoded generically via the self-describing tag),
inside the returned `RecordValue`; report them in `DecodeReport.unknown_field_ids`. Re-encoding
that `RecordValue` must reproduce them.

Float formatting must match `dsl_core`'s canonical number formatting (read
`/Users/ueli/Documents/semio/dsl/core/rs/lib.rs` for its `format_f64`/parse routines) closely
enough that `decode_pack(encode_pack(p)) == parse_dsl(print_dsl(p))` holds — this is the
cross-crate law `vcs`/`dsl_derive` will rely on in wave 1; do not diverge on NaN/-0.0 handling.

---

## pack_io (`pack/io/rs`, package `pack_io`)

Deps: `pack_core = { path = "../../core/rs" }`, `pack_format = { path = "../../format/rs" }`.
All public items behind `#[cfg(not(target_arch = "wasm32"))]` internals are fine, but the crate
itself must still compile (even if inert) for wasm32 — gate the whole native-only module with
`#[cfg(not(target_arch = "wasm32"))] mod native { ... } #[cfg(not(target_arch = "wasm32"))]
pub use native::*;` rather than failing the wasm32 build.

```rust
//#region 🔖File
pub struct FilePackSource { /* wraps std::fs::File, uses read_at via FileExt on unix / seek+read fallback elsewhere */ }
impl FilePackSource { pub fn open(path: &std::path::Path) -> Result<Self, pack_core::PackError>; }
impl pack_core::PackSource for FilePackSource { /* ... */ }

pub struct FilePackSink { /* wraps std::fs::File opened for write */ }
impl pack_core::PackSink for FilePackSink { /* ... */ }
//#endregion

//#region 🔖Atomic
// Write to `<path>.tmp-<random-ish suffix, NOT rand crate — use process id + a monotonic counter>`,
// fsync, then rename into place. No partial file is ever visible at `path`.
pub fn write_atomic(path: &std::path::Path, bytes: &[u8]) -> Result<(), pack_core::PackError>;
//#endregion

//#region 🔖Stream
// Incremental writer: flush segments to disk as produced rather than buffering the whole file.
pub struct StreamingPackWriter { /* wraps pack_format::PackWriter<FilePackSink> */ }
impl StreamingPackWriter {
    pub fn create(path: &std::path::Path, options: &pack_format::WriteOptions) -> Result<Self, pack_core::PackError>;
    pub fn write_segment(&mut self, kind: u8, payload: &[u8]) -> Result<(), pack_core::PackError>;
    pub fn write_chunk(&mut self, payload: &[u8]) -> Result<pack_core::ChunkId, pack_core::PackError>;
    pub fn finish(self, manifest: &pack_format::Manifest) -> Result<(), pack_core::PackError>;
}
//#endregion

//#region 🔖Recover
pub fn recover_file(path: &std::path::Path, limits: &pack_core::PackLimits) -> Result<pack_format::RecoveryReport, pack_core::PackError>;
//#endregion
```

---

## pack_async (`pack/async/rs`, package `pack_async`)

Deps: `pack_core = { path = "../../core/rs" }`, `pack_format = { path = "../../format/rs" }`.
Runtime-neutral: do not hard-require tokio in the trait definitions. Put a tokio-based adapter
behind `[features] tokio = ["dep:tokio"]`.

```rust
//#region 🔖AsyncSource
#[async_trait::async_trait]   // add async-trait as a dep; acceptable since it's a thin proc-macro, not a runtime
pub trait AsyncPackSource: Send + Sync {
    fn len(&self) -> u64;
    async fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, pack_core::PackError>;
}
pub struct CancellationToken { /* Arc<AtomicBool>, clone-cheap */ }
impl CancellationToken { pub fn new() -> Self; pub fn cancel(&self); pub fn is_cancelled(&self) -> bool; }
//#endregion

//#region 🔖Scheduler
pub enum LoadPriority { Critical, Visible, Requested, Prefetch, Background }
pub struct ReadRequest { pub range: pack_core::ByteRange, pub priority: LoadPriority }
pub struct ReadScheduler<S: AsyncPackSource> { /* coalesces adjacent/overlapping ranges, dedups identical in-flight requests, priority queue */ }
impl<S: AsyncPackSource> ReadScheduler<S> {
    pub fn new(source: S) -> Self;
    pub async fn read(&self, request: ReadRequest, cancel: &CancellationToken) -> Result<Vec<u8>, pack_core::PackError>;
}
//#endregion

//#region 🔖Backpressure
pub struct BoundedDemand { /* semaphore-like counter capping concurrent in-flight reads, no tokio dependency required for the non-tokio-feature path — implement with std::sync primitives + a simple spin/park or a minimal futures-based waiter */ }
//#endregion
```

---

## pack_http (`pack/http/rs`, package `pack_http`)

Deps: `pack_core`, `pack_format`, `pack_async` (all path deps). Do NOT depend on a concrete HTTP
client crate in the public API — inject one via a trait so wasm (browser fetch) and native
(e.g. `ureq`, behind a feature) can both provide it, per CLAUDE.md's "external libraries behind
an interface" rule.

```rust
//#region 🔖Transport
pub struct RangeRequest { pub url: String, pub range: pack_core::ByteRange, pub if_range_etag: Option<String> }
pub struct RangeResponse { pub bytes: Vec<u8>, pub etag: Option<String>, pub total_len: Option<u64>, pub range_satisfied: bool }
#[async_trait::async_trait]
pub trait RangeTransport: Send + Sync {
    async fn fetch_range(&self, request: RangeRequest) -> Result<RangeResponse, pack_core::PackError>;
}
//#endregion

//#region 🔖Source
pub struct HttpPackSource<T: RangeTransport> { /* url, transport, cached etag, retry policy */ }
impl<T: RangeTransport> HttpPackSource<T> {
    pub fn new(url: String, transport: T) -> Self;
}
impl<T: RangeTransport> pack_async::AsyncPackSource for HttpPackSource<T> { /* retry+backoff on transient failure, validates etag when re-fetching, coalesces via an internal pack_async::ReadScheduler */ }
//#endregion

//#region 🔖Cache
pub struct ChunkLruCache { /* bounded-size in-memory LRU keyed by pack_core::ContentHash or (url, range) */ }
impl ChunkLruCache {
    pub fn new(capacity_bytes: u64) -> Self;
    pub fn get(&self, key: &pack_core::ContentHash) -> Option<Vec<u8>>;
    pub fn put(&self, key: pack_core::ContentHash, bytes: Vec<u8>);
}
//#endregion
```

`[features] ureq = ["dep:ureq"]` optionally providing a native `UreqRangeTransport: RangeTransport`
impl; keep it feature-gated and off by default so wasm builds of the facade stay lean.

---

## pack_index (`pack/index/rs`, package `pack_index`)

Deps: `pack_core`, `pack_format`, `pack_value` (all path deps).

```rust
//#region 🔖FieldIndex
// Segment kind KIND_FIELD_INDEX (0x08). Maps a field-id path (from the document root) to the
// ByteRange of that field's encoded value within the Document segment(s), for sub-document
// random access without full decode. Root-eager (small header), leaf pages lazy.
pub struct FieldPath(pub Vec<u16>);   // sequence of field ids from the record root
pub struct FieldIndexEntry { pub path: FieldPath, pub range: pack_core::ByteRange }

pub struct FieldIndexBuilder { /* accumulates entries during pack_value::encode_document */ }
impl FieldIndexBuilder {
    pub fn new() -> Self;
    pub fn record(&mut self, path: FieldPath, range: pack_core::ByteRange);
    pub fn build(self) -> Vec<u8>;   // serialized FieldIndex segment payload: count varint, then (path len varint, path field ids varint*, offset varint, len varint)*, sorted by path
}
pub struct FieldIndexReader<'a> { /* borrows the raw FieldIndex segment payload */ }
impl<'a> FieldIndexReader<'a> {
    pub fn open(payload: &'a [u8]) -> Result<Self, pack_core::PackError>;
    pub fn lookup(&self, path: &FieldPath) -> Result<Option<pack_core::ByteRange>, pack_core::PackError>;
}
//#endregion
```

This crate does not itself hook into `pack_value::encode_document`/`decode_document` (those stay
index-agnostic in v1) — it is a standalone segment reader/writer that a future caller can wire
in. Do not modify `pack_value`'s signatures to accommodate it.

---

## pack (facade) (`pack/rs`, package `pack`)

Deps: `pack_core`, `pack_format`, `pack_value`, `pack_io`, `pack_async`, `pack_http`,
`pack_index` (all path deps, all re-exported).

```rust
//#region 🔖Reexports
pub use pack_core::{PackError, PackLimits, ContentHash, ChunkId, ByteRange, CodecId, PackSource, PackSink, CompressionCodec};
pub use pack_format::{VerificationLevel, Header, Footer, Manifest, Superblock, PackFile, PackWriter, WriteOptions, RecoveryReport};
pub use pack_value::{EncodeOptions, DecodeOptions, DecodeReport};
#[cfg(not(target_arch = "wasm32"))] pub use pack_io::{FilePackSource, FilePackSink, StreamingPackWriter, write_atomic};
pub use pack_async::{AsyncPackSource, CancellationToken, LoadPriority, ReadScheduler};
pub use pack_http::{RangeTransport, RangeRequest, RangeResponse, HttpPackSource, ChunkLruCache};
pub use pack_index::{FieldPath, FieldIndexEntry, FieldIndexBuilder, FieldIndexReader};
//#endregion

//#region 🔖Encode
pub fn encode_document(spec: &dsl_schema::RecordSpec, record: &dsl_schema::RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    pack_value::encode_document(spec, record, options)
}
pub fn decode_document(bytes: &[u8], spec: &dsl_schema::RecordSpec, options: &DecodeOptions) -> Result<(dsl_schema::RecordValue, DecodeReport), PackError> {
    pack_value::decode_document(bytes, spec, options)
}
pub fn content_hash(bytes: &[u8]) -> Result<ContentHash, PackError> {
    pack_format::read_footer_only(&bytes).map(|footer| footer.content_hash)
}
//#endregion
```

Note: the facade re-exports `dsl_schema` types by reference in its function signatures without
re-exporting the `dsl_schema` crate itself as `pack::dsl_schema` — callers pass in
`&dsl_schema::RecordSpec`/`&dsl_schema::RecordValue` they already have (from `vcs`/`dsl` in
wave 1). Do not add a `dsl_schema` re-export module to the facade.

---

## pack_testkit (`pack/testkit/rs`, package `pack_testkit`)

Deps: `pack = { path = "../../rs" }`, `dsl_schema = { path = "../../../dsl/schema/rs" }`,
`dsl_core = { path = "../../../dsl/core/rs" }`.

```rust
//#region 🔖Arbitrary
// Deterministic seeded generator — NOT the `arbitrary` or `quickcheck` crates (avoid new deps);
// implement a small xorshift/splitmix64 PRNG inline seeded by a u64.
pub struct RecordValueGen { /* seed state */ }
impl RecordValueGen {
    pub fn new(seed: u64) -> Self;
    pub fn generate(&mut self, spec: &dsl_schema::RecordSpec, max_depth: u16) -> dsl_schema::RecordValue;
}
//#endregion

//#region 🔖Laws
pub fn assert_encode_decode_identity(spec: &dsl_schema::RecordSpec, record: &dsl_schema::RecordValue);
    // encode_document then decode_document, asserts equal RecordValue (ignoring pure-Absent noise)
pub fn assert_canonical_stable(spec: &dsl_schema::RecordSpec, record: &dsl_schema::RecordValue);
    // encodes twice (with default options), asserts byte-identical output
pub fn assert_unknown_field_preserved(spec: &dsl_schema::RecordSpec, record_with_extra_fields: &dsl_schema::RecordValue, extra_ids: &[u16]);
pub fn assert_streamed_equals_buffered(spec: &dsl_schema::RecordSpec, record: &dsl_schema::RecordValue);
pub fn assert_dsl_pack_bidirectional<P>(
    parse_dsl: impl Fn(&str) -> P, print_dsl: impl Fn(&P) -> String,
    encode_pack: impl Fn(&P) -> Vec<u8>, decode_pack: impl Fn(&[u8]) -> P,
    sample: &P,
) where P: PartialEq + std::fmt::Debug;
    // decode_pack(encode_pack(sample)) == parse_dsl(&print_dsl(sample)) == *sample
    // (kept generic over closures so this crate need not depend on vcs/dsl_derive; vcs's own
    // test_support wraps this in wave 1 with concrete P: DocumentDsl + DocumentPack bounds)
//#endregion

//#region 🔖Corrupt
pub enum CorruptionLevel { Quick, Long, Exhaustive }
pub struct CorruptionReport { pub cases_run: u64, pub cases_errored: u64, pub cases_panicked: Vec<String> }
// Must never observe a panic (catch_unwind internally) — every corrupted input must produce Err, never Ok-with-garbage or a panic/abort.
pub fn fuzz_truncation(valid_pack: &[u8], level: CorruptionLevel, decode: impl Fn(&[u8]) -> Result<(), String>) -> CorruptionReport;
pub fn fuzz_bit_flips(valid_pack: &[u8], level: CorruptionLevel, decode: impl Fn(&[u8]) -> Result<(), String>) -> CorruptionReport;
//#endregion

//#region 🔖Golden
pub fn golden_hash_hex(bytes: &[u8]) -> String;   // hex(blake3(bytes)) — for committing as a text constant in a test
//#endregion
```

---

## pack_cli (`pack/cli/rs`, package `pack_cli`)

Deps: `pack = { path = "../../rs" }`. This crate has a `[[bin]] name = "pack" path = "lib.rs"`
in addition to being a library — put a `pub fn main_impl(args: &[String]) -> i32` in `lib.rs`
plus a tiny `fn main() { std::process::exit(main_impl(&std::env::args().skip(1).collect::<Vec<_>>())); }`
at the bottom (still inside the single `lib.rs` file — do not create a separate `main.rs`;
instead set `[[bin]] name = "pack" path = "lib.rs"` in Cargo.toml alongside `[lib] path = "lib.rs"`,
both pointing at the same file, which is fine since `main_impl`/`main` are only reachable from
the bin target when `#[cfg(not(test))]`-guarded appropriately — verify this compiles for both
`cargo build --lib` and `cargo build --bin pack`).

Subcommands (schema-less where possible — self-describing decode powers `inspect`/`verify`/`hash`
without needing a `RecordSpec`; `to-dsl`/`from-dsl` need a spec, so accept `--schema <name>`
resolved against a small built-in registry you seed with 2-3 example specs from `dsl_schema`
tests, or leave a clear `TODO` if no spec is reachable without depending on 49 app crates — do
NOT add a dependency from pack_cli to any app crate):
```
pack inspect <file>            # prints header/footer/manifest/segment list as text
pack verify <file> [--level=trusted|standard|full]
pack hash <file>                # prints content_hash hex
pack to-dsl <file> --schema <name>     # best-effort; documents the schema-registry limitation if unresolved
pack from-dsl <file> --schema <name> --out <file>
pack diff <file-a> <file-b>     # structural diff of decoded RecordValue trees where a schema is available, else raw segment diff
```

---

## Workspace-wide requirements

- Every crate's `Cargo.toml` starts with the standard header (copy from
  `/Users/ueli/Documents/semio/dsl/core/rs/Cargo.toml`): `version = "0.1.0"`, `edition = "2021"`,
  `rust-version = "1.88"`, one-line `description`, blank line, `[lints] workspace = true`, then
  `[lib]`, then `[dependencies]`.
- Do NOT add yourself to the root `Cargo.toml` `[workspace] members` list, do NOT create your
  crate's `project.json`/`script.ts`, and do NOT touch `.vscode/launch.json` — a separate
  closing agent scaffolds those for all ten crates in one pass to avoid file conflicts. Assume
  your crate directory and `Cargo.toml` already exist with the correct path deps when you start
  (the closing/foundation agent creates the skeleton first); if they don't yet exist when you
  run, create exactly your own `Cargo.toml` + `lib.rs` and nothing else.
- Write scratch/progress notes only inside this ticket folder
  (`.repo/🎫/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/`), as `.txt` files
  (gitignored `*.log` is silently dropped by ticket_close — use `.txt`).
- Include inline `//#region 🧪Tests` unit tests proving your crate's own laws in isolation
  (e.g. pack_format: header/footer round-trip on hand-built byte literals, segment
  skip-unknown, truncation-at-every-byte on a small hand-built file). Do not write a workspace
  test runner — that's wave 0's closing step.
