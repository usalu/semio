# Protocol Crate Family — Shared API Contract (Wave 0)

Every wave-0 agent implements exactly one crate. This is the binding cross-crate interface —
deviate only where your crate's section says "your choice"; an upstream/downstream crate written
by a different agent will fail to compile against yours otherwise.

Full design rationale: `/Users/ueli/.claude/plans/we-want-to-create-refactored-harbor.md`. Read it
first for context, then implement strictly against the signatures below. Also skim
`/Users/ueli/Documents/semio/.repo/🎫️/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md`
and the actual merged `pack/core/rs/lib.rs` + `pack/format/rs/lib.rs` — this format reuses those
primitives directly (path dependency, not reimplementation).

Repo conventions (non-negotiable, identical to the pack rollout): single-file `lib.rs` at
`protocol/<part>/rs/lib.rs` with `[lib] path = "lib.rs"`; `edition = "2021"`,
`rust-version = "1.88"`; `[lints] workspace = true`; code in `//#region 🔖️Name` /
`//#endregion 🔖️Name` blocks; every doc comment starts with an emoji (`/// 🎞️ ...` or
`//! 🎞️ ...` — use 🎞️ film-strip as this family's emoji, distinct from pack's 📦️); tests inline
in `//#region 🧪️Tests` at the bottom of `lib.rs`, `mod tests { ... mod quick { } mod long { }
mod exhaustive { } }` (only add level submodules if you actually have slow/exhaustive tests —
plain `#[test]` fns for everything else); no `unsafe`; no `std::io::Error` in any public signature
(must stay wasm32-clean in protocol_core/format/history/materialize; protocol_io may use
`std::io`/tokio internally behind `cfg(not(target_arch = "wasm32"))`, but its public trait
signatures must still avoid leaking `std::io::Error` — wrap into `ProtocolError::Io(String)`).

Package names are `protocol_core`, `protocol_format`, `protocol_history`, `protocol_materialize`,
`protocol_io`, `protocol` (facade), `protocol_testkit`, `protocol_cli`. Path deps between them are
plain relative paths, e.g. `protocol_core = { path = "../../core/rs" }`. Path deps on the `pack`
family are relative into `../../../pack/...`, e.g. `pack_core = { path = "../../../pack/core/rs" }`.

**Hard dependency rule**: protocol crates NEVER depend on `vcs`, `semio-framework-core`, or
`pack_value`. Op payloads and projection bodies are opaque validated bytes to every crate in this
family — you store, hash, frame, and index them; you never interpret them. `vcs` will depend on
`protocol` (a later wave), never the reverse — creating a cycle is a hard build failure you must
never introduce.

---

## protocol_core (`protocol/core/rs`, package `protocol_core`)

Deps: `pack_core = { path = "../../../pack/core/rs" }`, `thiserror`. Must compile for
`wasm32-unknown-unknown`.

```rust
//#region 🔖️Errors
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ProtocolError {
    #[error(transparent)] Pack(#[from] pack_core::PackError),
    #[error("chain mismatch at commit {commit_seq}")] ChainMismatch { commit_seq: u64 },
    #[error("torn tail at offset {0}")] TornTail(u64),
    #[error("unknown critical record kind {0:#x}")] UnknownCriticalRecord(u8),
    #[error("dictionary index out of range: {0}")] DictMiss(u32),
    #[error("dictionary out of order: expected base_count {expected}, got {actual}")] DictOutOfOrder { expected: u32, actual: u32 },
    #[error("signature verification required but no verifier supplied")] VerifierRequired,
    #[error("signature invalid for commit {commit_seq}")] SignatureInvalid { commit_seq: u64 },
    #[error("frame back_len mismatch at offset {0}")] FrameFraming(u64),
    #[error("limit exceeded: {0}")] LimitExceeded(&'static str),
    #[error("malformed {what} at offset {offset}: {detail}")] Malformed { what: &'static str, offset: u64, detail: String },
    #[error("io error: {0}")] Io(String),
}
//#endregion

//#region 🔖️Limits
#[derive(Clone, Debug)]
pub struct ProtocolLimits {
    pub max_file_len: u64,        // default 64 * 1024*1024*1024 (64 GiB — history can outlive the doc)
    pub max_frame_len: u64,       // default 2 * 1024*1024*1024 (2 GiB, fits u32 back_len)
    pub max_record_count: u64,    // default 256_000_000
    pub max_dict_entries: u32,    // default 1_000_000
    pub max_op_count_per_edit: u32, // default 100_000
    pub max_total_alloc: u64,     // default 4 * 1024*1024*1024 (4 GiB)
}
impl Default for ProtocolLimits { fn default() -> Self { /* values above */ } }
//#endregion

//#region 🔖️RecordKinds
// Plain `pub const` u8s (mirrors pack_core::SegmentKind convention but this family uses bare
// u8 kind bytes directly in the frame, no wrapper newtype — simpler, and every downstream crate
// matches on the byte).
pub const REC_END: u8 = 0x00;
pub const REC_DOC: u8 = 0x01;
pub const REC_ACTOR_DICT: u8 = 0x02;
pub const REC_STR_DICT: u8 = 0x03;
pub const REC_EDIT: u8 = 0x04;
pub const REC_CHANGE: u8 = 0x05;
pub const REC_CHECKPOINT: u8 = 0x06;
pub const REC_ALTERNATIVE: u8 = 0x07;
pub const REC_ACTIVE: u8 = 0x08;
pub const REC_FRONTIER: u8 = 0x09;
pub const REC_PROJECTION: u8 = 0x0A;
pub const REC_INDEX: u8 = 0x0B;
pub const REC_COMMIT: u8 = 0x0C;
pub const REC_SIGNATURE: u8 = 0x0D;
pub const REC_REDACTION: u8 = 0x0E;
pub const REC_UPCAST: u8 = 0x0F;
pub const REC_EPHEMERAL: u8 = 0x10;
pub const REC_SEALED: u8 = 0x11;
pub const REC_COMPACTION: u8 = 0x12;
pub const REC_PADDING: u8 = 0x7F;
// Extension range 0x40..=0x7E is caller-defined, never critical unless the frame's critical bit is set.
pub fn is_critical_kind(kind: u8) -> bool {
    matches!(kind, REC_DOC | REC_EDIT | REC_CHANGE | REC_CHECKPOINT | REC_ALTERNATIVE | REC_ACTIVE
        | REC_COMMIT | REC_ACTOR_DICT | REC_STR_DICT)
}
//#endregion

//#region 🔖️Flags
// Header required/optional flags (32-byte header, see protocol_format).
pub const REQUIRED_HASH_CHAIN: u32 = 1 << 0;
pub const REQUIRED_SIGNED: u32 = 1 << 1;
pub const REQUIRED_ENCRYPTED: u32 = 1 << 2;   // reserved, never set by this crate family
pub const OPTIONAL_CANONICAL: u32 = 1 << 0;
pub const OPTIONAL_HAS_PROJECTIONS: u32 = 1 << 1;
pub const OPTIONAL_HAS_INDEX: u32 = 1 << 2;
pub const OPTIONAL_REDACTED: u32 = 1 << 3;
// Frame flags byte (per-record, not header): bit0 compressed, bit1 critical, bits2..4 = codec id (0..=7).
pub const FRAME_FLAG_COMPRESSED: u8 = 1 << 0;
pub const FRAME_FLAG_CRITICAL: u8 = 1 << 1;
pub fn frame_codec_id(flags: u8) -> u8 { (flags >> 2) & 0b111 }
pub fn frame_flags(compressed: bool, critical: bool, codec: u8) -> u8 {
    (compressed as u8) | ((critical as u8) << 1) | ((codec & 0b111) << 2)
}
//#endregion

//#region 🔖️Scalars
// Tagged, lossless-by-construction scalar codecs shared by protocol_history's payload codecs.
// `out`/`bytes,pos` follow pack_core::ByteWriter/ByteReader conventions exactly (in fact take
// `&mut pack_core::ByteWriter` / `&mut pack_core::ByteReader<'a>` directly — no reimplementation).
pub mod scalar {
    use pack_core::{ByteReader, ByteWriter, PackError};

    // Timestamp tag: 0 = raw string (len varint + utf8), 1 = epoch-ms varint (iff reprint is
    // byte-exact vs source), 2 = zigzag-varint delta-ms vs previous tag-1/2 timestamp in stream.
    pub fn write_timestamp(out: &mut ByteWriter, raw: &str, prev_epoch_ms: Option<i64>) -> Option<i64>;
    // Returns the decoded string AND, if tag 1/2, the epoch_ms to feed as `prev_epoch_ms` next call.
    pub fn read_timestamp(input: &mut ByteReader<'_>, prev_epoch_ms: Option<i64>) -> Result<(String, Option<i64>), PackError>;

    // Id tag: 0 = raw string, 1 = dictref (varint index into a string dictionary — caller supplies
    // the resolve/intern closures), 2 = prefix_dictref + [u8;16] (iff id is "<prefix>-<uuid>"),
    // 3 = edit-ordinal varint (only valid where the referent is a previously-seen edit).
    pub fn write_id(out: &mut ByteWriter, id: &str, intern: impl FnMut(&str) -> u32, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<(), PackError>;
    pub fn read_id(input: &mut ByteReader<'_>, resolve: impl Fn(u32) -> Result<&str, PackError>, ordinal_to_id: impl Fn(u64) -> Result<&str, PackError>) -> Result<String, PackError>;

    // Id list: count varint, entries* (each via write_id/read_id).
    // Minimal-varint enforcement reuses pack_core::is_minimal_varint at Full verification.
}
//#endregion

//#region 🔖️Dictionary
// In-memory dictionary builder/reader shared by protocol_history's REC_ACTOR_DICT/REC_STR_DICT
// codec and protocol_format's dict-aware frame helpers. Deterministic first-use interning order.
pub struct DictBuilder { /* Vec<String> + HashMap<String, u32> for O(1) intern */ }
impl DictBuilder {
    pub fn new() -> Self;
    pub fn intern(&mut self, s: &str) -> u32;   // returns existing index or appends + returns new index
    pub fn len(&self) -> u32;
    pub fn is_empty(&self) -> bool;
    pub fn entries_since(&self, base_count: u32) -> &[String];  // for delta-record encoding
}
pub struct DictReader { entries: Vec<String> }
impl DictReader {
    pub fn new() -> Self;
    pub fn extend(&mut self, base_count: u32, new_entries: impl IntoIterator<Item = String>) -> Result<(), ProtocolError>;  // errors DictOutOfOrder if base_count != self.entries.len()
    pub fn resolve(&self, index: u32) -> Result<&str, ProtocolError>;
    pub fn len(&self) -> u32;
}
//#endregion

//#region 🔖️Crypto
// Trait-only — no algorithm ships in protocol_core (repo rule: external libs behind an
// interface). protocol_format provides a Blake3Hasher impl of RecordHasher (it already
// owns the blake3 dep); Signer/SignatureVerifier have zero impls in this family — supplied
// by the integration layer or protocol_cli's optional feature-gated tooling.
pub trait RecordHasher { fn hash(&self, bytes: &[u8]) -> [u8; 32]; }
pub trait Signer {
    fn scheme(&self) -> &str;
    fn key_id(&self) -> &str;
    fn sign(&self, message: &[u8; 32]) -> Result<Vec<u8>, ProtocolError>;
}
pub trait SignatureVerifier {
    fn verify(&self, scheme: &str, key_id: &str, message: &[u8; 32], signature: &[u8]) -> Result<bool, ProtocolError>;
}
//#endregion
```

Your crate is the foundation everyone else path-deps on. Validate every length against
`ProtocolLimits` **before allocating**, exactly like `pack_core`'s stated invariant.

---

## protocol_format (`protocol/format/rs`, package `protocol_format`)

Deps: `protocol_core = { path = "../../core/rs" }`, `pack_core`, `pack_format = { path =
"../../../pack/format/rs" }` (only for `DeflateCodec`, feature `deflate` default-on), `blake3`.

Container byte layout — implement EXACTLY this (byte-for-byte; other crates and the CLI round-trip
against these constants).

- **Magic**: `[0x89, b'S', b'P', b'R', 0x0D, 0x0A, 0x1A, 0x0A]` (8 bytes). Own magic, NOT pack's —
  a `.spr` file is append-only and live; its EOF can be torn at any moment, so it cannot share
  pack's write-once-footer-as-root-of-trust model. Extension `.spr`.
- **Header (32 bytes)**: offset 0 magic(8), 8 version_major u16 LE (=1), 10 version_minor u16 LE
  (=0), 12 required_flags u32 LE, 16 optional_flags u32 LE, 20 header_crc32 u32 LE (CRC-32C over
  bytes 0..20, reuse `pack_core::crc32c`), 24 reserved 8 bytes (zero on write, ignored on read).
- **Required flags** (`protocol_core::REQUIRED_*`): reader MUST return
  `ProtocolError::Pack(PackError::UnknownRequiredFlags)`-shaped error (wrap manually — construct
  your own via a `Malformed`/dedicated variant if `PackError::UnknownRequiredFlags` isn't directly
  constructible outside pack_core; if it isn't, add a `ProtocolError::UnknownRequiredFlags(u32)`
  variant to protocol_core in this crate's PR since you own the header parser — deviation allowed
  here, note it in your PR summary) if any bit outside 0..=2 is set.
- **Record frame** (the single uniform unit, used for EVERY record including REC_COMMIT):
  ```
  body_len:  varint u64        — length of kind + flags + [raw_len] + payload
  kind:      u8
  flags:     u8                — bit0 compressed, bit1 critical, bits2..4 codec id (protocol_core::frame_flags helpers)
  [raw_len:  varint u64]       — iff flags.bit0; validate vs ProtocolLimits::max_frame_len BEFORE allocating decompress buffer
  payload:   body_len - (1 + 1 + [raw_len varint width]) bytes
  crc32c:    u32 LE            — CRC-32C over kind..payload inclusive (pack_core::crc32c)
  back_len:  u32 LE            — TOTAL frame length in bytes INCLUDING the body_len varint, crc32c, and back_len itself
  ```
  Skip-unknown: unknown `kind` with critical bit clear → skip `body_len` bytes + trailing 8
  (crc32c + back_len); critical bit set → `ProtocolError::UnknownCriticalRecord(kind)`.
  Reverse scan: read trailing 4 bytes as `back_len`, jump back `back_len` bytes, forward-parse,
  require the reproduced end offset to equal the original position and crc to validate; else
  `ProtocolError::FrameFraming(offset)`. Cap one frame at `< 4 GiB` (u32 `back_len`), enforced via
  `ProtocolLimits::max_frame_len` (default 2 GiB, leaving headroom).
- **Commit frame** (`REC_COMMIT`, always critical, fixed 64-byte payload, so total frame is
  exactly `1(body_len varint, single byte since 65 < 128) + 1(kind) + 1(flags) + 64(payload) +
  4(crc32c) + 4(back_len) = 75 bytes`):
  ```
  offset 0  commit_seq: u64 LE            — monotone from 1
  offset 8  prev_commit_offset: u64 LE    — absolute file offset of the previous REC_COMMIT frame's FIRST byte (0 = none, only for commit_seq == 1)
  offset 16 records_len: u64 LE           — bytes of all frames between the previous commit frame's end (or offset 32 for the first) and this frame's start
  offset 24 record_count: u32 LE          — count of those frames
  offset 28 reserved: u32 LE              — zero on write
  offset 32 chain_hash: [u8; 32]          — blake3, see chain algorithm below
  ```
  **Chain algorithm**: `digest_i = blake3(full frame bytes of record i)` for every non-commit
  frame since the previous commit (in file order). `chain_0 = blake3(header 32 bytes)`;
  `chain_n = blake3(chain_{n-1} || digest_1 || .. || digest_k)` where `chain_n` is this commit's
  `chain_hash` and `chain_{n-1}` is the previous commit's `chain_hash` (or `chain_0` for the
  first). A redacted record (see REC_REDACTION in protocol_history) has its original frame bytes
  physically replaced by a padding tombstone ONLY during compaction; the tombstone carries the
  original `digest_i` so `chain_n` recomputes identically — chain verification never needs the
  redacted bytes.
- **`FrameCursor`/`ReverseFrameCursor`** — zero-copy, no self-referential types:
  ```rust
  pub struct RecordFrame<'a> { pub kind: u8, pub flags: u8, pub offset: u64, pub stored: &'a [u8], pub raw_len: Option<u64> }
  impl<'a> RecordFrame<'a> {
      pub fn payload(&self) -> &'a [u8];              // if !compressed, == stored; caller decompresses via CompressionCodec otherwise
      pub fn frame_len(&self) -> u64;                  // total on-disk bytes for this frame
  }
  pub struct FrameCursor<'a> { /* bytes: &'a [u8], pos: usize */ }
  impl<'a> FrameCursor<'a> {
      pub fn new(bytes: &'a [u8], start_offset: u64) -> Self;
      pub fn next_frame(&mut self) -> Result<Option<RecordFrame<'a>>, ProtocolError>;
  }
  pub struct ReverseFrameCursor<'a> { /* bytes: &'a [u8], pos: usize (end-exclusive) */ }
  impl<'a> ReverseFrameCursor<'a> {
      pub fn at_end(bytes: &'a [u8]) -> Self;
      pub fn prev_frame(&mut self) -> Result<Option<RecordFrame<'a>>, ProtocolError>;   // O(1) step via back_len
  }
  ```
- **Sealed batch** (`REC_SEALED`, written only by compaction, never by the live appender):
  payload = `codec: u8, raw_len: varint, compressed_bytes` where `compressed_bytes` decompress
  (via `pack_format::DeflateCodec` when codec==1) to a concatenation of ordinary inner record
  frames (same grammar, parseable by a fresh `FrameCursor` over the inflated buffer).
- **Writer**:
  ```rust
  pub struct WriteOptions { pub required_flags: u32, pub optional_flags: u32 }
  pub struct SprWriter<S: pack_core::PackSink> { /* fields private; tracks running chain_hash, pending record count/bytes since last commit */ }
  impl<S: pack_core::PackSink> SprWriter<S> {
      pub fn begin(sink: S, options: &WriteOptions) -> Result<Self, ProtocolError>;      // writes 32-byte header
      pub fn position(&self) -> u64;
      pub fn write_record(&mut self, kind: u8, critical: bool, payload: &[u8], codec: pack_core::CodecId) -> Result<u64, ProtocolError>;  // returns this record's start offset; frames + compresses + crc's + writes; updates running chain state
      pub fn commit(&mut self) -> Result<u64, ProtocolError>;   // writes a REC_COMMIT frame covering everything since the last commit (or header); returns the commit frame's start offset
      pub fn into_sink(self) -> S;
  }
  ```
  `write_record` must use ONE reusable internal scratch `Vec<u8>` for the frame prefix (body_len
  varint + kind + flags + raw_len) — never buffer the whole payload again if it's already a
  caller-owned slice; stream it through `PackSink::write_all` in at most two calls (prefix, then
  payload, then trailer).
- **Reader / recovery**:
  ```rust
  pub struct RecoveryReport { pub records_recovered: u64, pub bytes_recovered: u64, pub last_commit_seq: u64, pub last_commit_offset: u64, pub torn_tail_bytes: u64 }
  pub enum RecoveryMode { LastCommit, LastValidRecord }   // default LastCommit
  pub enum VerificationLevel { Trusted, Standard, Full }  // impl Default -> Standard; Full recomputes the full chain
  pub fn recover<S: pack_core::PackSource>(source: &S, limits: &protocol_core::ProtocolLimits, mode: RecoveryMode) -> Result<RecoveryReport, ProtocolError>;
  // Algorithm (implement exactly): (1) validate header; (2) fast path — reverse-probe EOF for a
  // crc-valid REC_COMMIT frame, walk prev_commit_offset backwards confirming each link is itself
  // a crc-valid commit frame (O(commits), no record scan); (3) on fast-path failure, forward-scan
  // frames from offset 32, validating body_len/limits/crc/back_len-echo per frame, tracking
  // last_valid_end and last_commit_end; stop at first invalid/truncated frame; (4) LastCommit mode
  // trusts only [0, last_commit_end); LastValidRecord trusts through last_valid_end.
  ```

Own the `blake3` dependency here (protocol_core stays dep-free). `Blake3Hasher` implements
`protocol_core::RecordHasher`.

---

## protocol_history (`protocol/history/rs`, package `protocol_history`)

Deps: `protocol_core`, `protocol_format`, `pack_core`, `dsl_core = { path =
"../../../dsl/core/rs" }`, `dsl_schema = { path = "../../../dsl/schema/rs" }`.

This is the typed record layer: the `HistoryLog` pivot model, per-kind payload codecs, and the
`.ops` text grammar twin. Read `/Users/ueli/Documents/semio/vcs/rs/lib.rs` lines 855-1218 (regions
`🔖️TextFormat` + `🔖️OpsHeaderGrammar`) FIRST for the exact `OpsHeaderLine` grammar you are
building a binary-format-independent, `vcs`-independent twin of. You cannot depend on `vcs` (it
depends on `DslOps`, which emits `impl ::vcs::OpText` — a `vcs`-owned trait), so build your own
parser/printer directly against `dsl_schema`'s `RecordSpec`/grammar primitives — read
`/Users/ueli/Documents/semio/dsl/schema/rs/lib.rs` and `/Users/ueli/Documents/semio/dsl/core/rs/lib.rs`
for the engine you're building on (the same one `dsl_derive`'s `DslOps` macro drives).

```rust
//#region 🔖️Model
// Every field of vcs::OpsHeaderLine (Doc/Edit/Change/Checkpoint/Alternative/Active) has exactly
// one slot below. Op lines are opaque exact `print_op` strings (one per line, no '\n' inside).
// Derived data (backwards, sequence_number, unless explicitly captured via `meta`) is excluded.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryLog {
    pub doc_id: String,
    pub schema: String,
    pub edits: Vec<HistoryEdit>,
    pub changes: Vec<HistoryChange>,
    pub checkpoints: Vec<HistoryCheckpoint>,
    pub alternatives: Vec<HistoryAlternative>,
    pub active_alternative_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryEdit {
    pub id: String,
    pub actor: Option<String>,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub coalesce_key: Option<String>,
    pub description: Option<String>,
    pub ops: Vec<OpPayload>,
    /// Present iff the caller supplied it (vcs hot-path appender has these in memory); absent
    /// for text-compiled/imported logs. NOT required for round-trip; a decoder recomputing
    /// backwards/meta from a fresh replay never touches this field.
    pub meta: Option<Vec<HistoryOpMeta>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpPayload { pub text: String, pub binary: Option<Vec<u8>> }  // binary: reserved seam, v1 always None
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryOpMeta {
    pub op_id: Option<String>, pub dependencies: Vec<String>, pub base_version: u64,
    pub author_id: Option<String>, pub hlt: Option<(u64, i64, u64)>,  // (actor, physical_ms, logical)
    pub undo_policy: u8, pub payload_hash: Option<[u8; 32]>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryChange { pub id: String, pub saved_at: String, pub edit_ids: Vec<String>, pub description: Option<String> }
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryCheckpoint { pub id: String, pub timestamp: String, pub change_ids: Vec<String>, pub parent_id: Option<String>, pub authors: Vec<HistoryAuthor>, pub message: Option<String> }
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryAuthor { pub id: String, pub name: String }
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryAlternative { pub id: String, pub name: String, pub checkpoint_ids: Vec<String> }
//#endregion

//#region 🔖️TextGrammar
// Your own twin of vcs::OpsHeaderLine's grammar (do NOT import vcs). Build against dsl_schema
// directly. Must round-trip exactly like vcs's existing text format for the 6 line kinds.
pub fn parse_ops_text(ops: &str) -> Result<HistoryLog, ProtocolError>;
pub fn print_ops_text(log: &HistoryLog) -> String;
// LAW: parse_ops_text(&print_ops_text(log)) == log for every log producible by this crate's
// own generator (protocol_testkit). Comments/blank lines normalize away on first parse.
//#endregion

//#region 🔖️Payloads
// Binary codec for each record kind, using protocol_core::scalar + protocol_format's frame
// writer/reader. Every payload starts `format: u8` (=1); trailing bytes ignored on read
// (additive-evolution slot) except where a critical record demands `format <= known`.
pub fn encode_doc(doc_id: &str, schema: &str, dict: &mut protocol_core::DictBuilder) -> Vec<u8>;
pub fn decode_doc(payload: &[u8], dict: &protocol_core::DictReader) -> Result<(String, String), ProtocolError>;
pub fn encode_edit(edit: &HistoryEdit, dict: &mut protocol_core::DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError>;
pub fn decode_edit(payload: &[u8], dict: &protocol_core::DictReader, ordinal_to_id: impl Fn(u64) -> Result<&str, ProtocolError>) -> Result<HistoryEdit, ProtocolError>;
// ... encode_change/decode_change, encode_checkpoint/decode_checkpoint,
//     encode_alternative/decode_alternative, encode_active/decode_active — same shape.
// REC_EDIT layout: format u8, presence u8 (bit0 actor, bit1 finished, bit2 key, bit3 description,
// bit4 explicit_meta, bit5 has_backwards_section), id, started(ts), [actor(dictref)],
// [finished(ts)], [key(str)], [description(str)], op_count varint, op_count x (op_tag u8
// [bit0 has_text=1 required in v1, bit1 has_binary reserved] + text_len varint + utf8),
// [explicit_meta iff bit4: op_count x (op_id(id?), dep_count varint + deps(id)*, base_version
// varint, author(dictref?), hlt(3 varints)?, undo_policy u8, payload_hash tag u8 + [u8;32]?)],
// [backwards section iff bit5: op_count varint (backward op count, may differ from forward)
// + same op-line encoding as forwards + reuses explicit_meta encoding for operation_meta].
//#endregion

//#region 🔖️Codec
// Whole-file compile: HistoryLog <-> .spr bytes, using protocol_format::SprWriter/FrameCursor.
// Dictionaries are built in one pre-pass (deterministic first-use order over the whole log) then
// flushed as REC_ACTOR_DICT/REC_STR_DICT delta records before the records that reference them —
// OR flushed incrementally as they grow (your choice; must stay canonical-stable either way).
pub struct EncodeOptions { pub canonical: bool, pub write_backwards_section: bool, pub limits: protocol_core::ProtocolLimits }
impl Default for EncodeOptions { /* canonical: true, write_backwards_section: false, limits: default */ }
pub struct DecodeOptions { pub verification: protocol_format::VerificationLevel, pub limits: protocol_core::ProtocolLimits }
impl Default for DecodeOptions { /* verification: Standard, limits: default */ }

pub fn encode_history(log: &HistoryLog, options: &EncodeOptions) -> Result<Vec<u8>, ProtocolError>;
pub fn decode_history(bytes: &[u8], options: &DecodeOptions) -> Result<HistoryLog, ProtocolError>;
// LAW (protocol_testkit asserts this): decode_history(&encode_history(log, _)) == log
// (modulo `meta`/backwards presence exactly matching what write_backwards_section requested).
//#endregion

//#region 🔖️Append
// Streaming append API over protocol_format::SprWriter — the hot path. One edit -> one REC_EDIT
// frame, O(new edit) allocation.
pub struct HistoryAppender<S: pack_core::PackSink> { /* wraps SprWriter<S> + live DictBuilder state */ }
impl<S: pack_core::PackSink> HistoryAppender<S> {
    pub fn begin(sink: S, doc_id: &str, schema: &str, options: &protocol_format::WriteOptions) -> Result<Self, ProtocolError>;
    pub fn append_edit(&mut self, edit: &HistoryEdit) -> Result<u64, ProtocolError>;   // returns frame offset
    pub fn append_change(&mut self, change: &HistoryChange) -> Result<u64, ProtocolError>;
    pub fn append_checkpoint(&mut self, checkpoint: &HistoryCheckpoint) -> Result<u64, ProtocolError>;
    pub fn append_alternative(&mut self, alternative: &HistoryAlternative) -> Result<u64, ProtocolError>;
    pub fn set_active(&mut self, alternative_id: Option<&str>) -> Result<u64, ProtocolError>;
    pub fn commit(&mut self) -> Result<u64, ProtocolError>;   // forwards to SprWriter::commit
    pub fn into_sink(self) -> S;
}
//#endregion

//#region 🔖️Scan
// Read-side over a byte buffer (or anything PackSource-shaped, via protocol_format cursors).
pub struct HistoryReader<'a> { /* bytes: &'a [u8] */ }
impl<'a> HistoryReader<'a> {
    pub fn open(bytes: &'a [u8], options: &DecodeOptions) -> Result<Self, ProtocolError>;
    pub fn log(&self) -> Result<HistoryLog, ProtocolError>;              // full decode (FullHistory materialize path)
    pub fn edits(&self) -> EditIter<'a>;                                  // forward, lazy per-record decode
    pub fn edits_rev(&self, limit: usize) -> RevEditIter<'a>;             // O(1)-step reverse via ReverseFrameCursor
}
pub struct EditIter<'a> { /* ... */ }
impl<'a> Iterator for EditIter<'a> { type Item = Result<HistoryEdit, ProtocolError>; /* ... */ }
pub struct RevEditIter<'a> { /* ... */ }
impl<'a> Iterator for RevEditIter<'a> { type Item = Result<HistoryEdit, ProtocolError>; /* ... */ }
//#endregion

//#region 🔖️Frontier
#[derive(Clone, Debug, PartialEq)]
pub struct AlternativeHead { pub alternative_id: String, pub checkpoint_id: String, pub head_edit_ordinal: u64 }
#[derive(Clone, Debug, PartialEq)]
pub struct FrontierSummary { pub document_id: String, pub head_edit_ordinal: u64, pub head_edit_id: String, pub alternatives: Vec<AlternativeHead>, pub last_commit_seq: u64, pub chain_hash: [u8; 32] }
pub enum FrontierComparison { Equal, Ahead, Behind, Diverged { common_edit_count: u64 } }
pub fn frontier_delta(local: &FrontierSummary, remote: &FrontierSummary) -> FrontierComparison;
//#endregion

//#region 🔖️Index
// Advisory REC_INDEX payload sections. Rebuildable from scan; never authoritative.
pub const SEC_EDIT_OFFSETS: u8 = 0x01;
pub const SEC_CHECKPOINT_OFFSETS: u8 = 0x02;
pub const SEC_DICT_OFFSETS: u8 = 0x03;
pub const SEC_PROJECTION_OFFSETS: u8 = 0x04;
pub const SEC_SEALED_OFFSETS: u8 = 0x05;
pub struct IndexBuilder { /* accumulates offsets as records are appended */ }
impl IndexBuilder {
    pub fn new() -> Self;
    pub fn record_edit(&mut self, ordinal: u64, offset: u64);
    pub fn record_checkpoint(&mut self, id: &str, offset: u64, edit_ordinal: u64);
    // ... record_dict, record_projection, record_sealed
    pub fn build(&self) -> Vec<u8>;   // REC_INDEX payload
}
pub struct IndexReader<'a> { /* borrows the raw REC_INDEX payload */ }
impl<'a> IndexReader<'a> {
    pub fn open(payload: &'a [u8]) -> Result<Self, ProtocolError>;
    pub fn edit_offset_at_or_before(&self, ordinal: u64) -> Option<u64>;
    pub fn checkpoint_offset(&self, checkpoint_id: &str) -> Option<(u64, u64)>;  // (offset, edit_ordinal)
    pub fn latest_projection_offset_at_or_before(&self, ordinal: u64) -> Option<u64>;
}
//#endregion
```

---

## protocol_materialize (`protocol/materialize/rs`, package `protocol_materialize`)

Deps: `protocol_core`, `protocol_format`, `protocol_history`.

```rust
//#region 🔖️Projection
// REC_PROJECTION payload: format u8, anchor tag u8 (0 = checkpoint id follows, 1 = raw edit
// ordinal follows), [checkpoint_id: id], edit_ordinal: varint, body_kind u8 (0 = embedded pack
// bytes, 1 = sidecar by content hash, 2 = embedded dsl text), body_hash [u8;32] (blake3 of body
// bytes), [body_len varint + body bytes iff embedded]. The body is ALWAYS opaque to this crate —
// a complete `.spk` (or dsl text) produced upstream; you store/hash/frame it, never decode it.
pub enum ProjectionBodyKind { EmbeddedPack, SidecarPack, EmbeddedDsl }
pub struct ProjectionRecord { pub anchor_checkpoint_id: Option<String>, pub edit_ordinal: u64, pub body_kind: ProjectionBodyKind, pub body_hash: [u8; 32], pub body: Option<Vec<u8>> }
pub fn encode_projection(record: &ProjectionRecord) -> Vec<u8>;
pub fn decode_projection(payload: &[u8]) -> Result<ProjectionRecord, ProtocolError>;
//#endregion

//#region 🔖️Policy
pub struct CheckpointPolicy { pub every_edits: u64, pub every_bytes: u64, pub on_checkpoint_commit: bool, pub embed_below: u64 }
impl Default for CheckpointPolicy { fn default() -> Self { Self { every_edits: 512, every_bytes: 4 * 1024 * 1024, on_checkpoint_commit: true, embed_below: 1024 * 1024 } } }
//#endregion

//#region 🔖️Plan
pub enum BaseBytes<'a> { Borrowed(&'a [u8]), Sidecar { expected_hash: [u8; 32] } }
pub struct BaseProjection<'a> { pub bytes: BaseBytes<'a>, pub applied_edits: u64 }
pub struct MaterializePlan<'a> { pub base: BaseProjection<'a>, pub tail_start_offset: u64, pub target_edit_ordinal: Option<u64> }
pub enum MaterializeTarget { LatestOnActive, AtCheckpoint(String), AtEditOrdinal(u64) }

pub fn resolve_plan<'a>(protocol_bytes: &'a [u8], initial_pack: &'a [u8], target: MaterializeTarget, limits: &protocol_core::ProtocolLimits) -> Result<MaterializePlan<'a>, ProtocolError>;
// Steps: open via protocol_format::recover fast path (O(1)-ish) -> load latest REC_INDEX + dicts
// -> pick newest REC_PROJECTION with edit_ordinal <= target via SEC_PROJECTION_OFFSETS (fallback:
// reverse frame scan) -> verify body_hash at Standard/Full -> on corrupt/missing, fall back to
// the next-older projection, ultimately to `initial_pack` at edit_ordinal 0.
//#endregion

//#region 🔖️Drive
// Closure-generic replay driver — protocol never knows P or the op-application semantics.
pub struct MaterializeReport { pub snapshot_used: Option<(Option<String>, u64)>, pub snapshots_skipped_corrupt: u32, pub edits_replayed: u64, pub bytes_read: u64, pub genesis_replay: bool }
pub fn materialize_with<P, E>(
    plan: MaterializePlan<'_>,
    protocol_bytes: &[u8],
    decode_base: impl FnOnce(&[u8]) -> Result<P, E>,
    mut apply_edit: impl FnMut(&mut P, &protocol_history::HistoryEdit) -> Result<(), E>,
) -> Result<(P, MaterializeReport), E>
where E: From<ProtocolError>;
//#endregion
```

---

## protocol_io (`protocol/io/rs`, package `protocol_io`)

Deps: `protocol_core`, `protocol_format`, `protocol_history`, `pack_core`, `pack_io = { path =
"../../../pack/io/rs" }`. Whole native-only module gated `#[cfg(not(target_arch = "wasm32"))]`,
mirroring `pack_io`'s pattern exactly (crate compiles inert for wasm32, never fails the build).

```rust
#[cfg(not(target_arch = "wasm32"))]
mod native {
//#region 🔖️File
pub struct ResumeState { pub end_offset: u64, pub last_commit_seq: u64, pub chain_hash: [u8; 32] }
pub struct HistoryFile { /* wraps pack_io::FilePackSource + FilePackSink-like write handle */ }
impl HistoryFile {
    pub fn create(path: &std::path::Path, doc_id: &str, schema: &str, options: &protocol_format::WriteOptions) -> Result<Self, protocol_core::ProtocolError>;
    /// Runs protocol_format::recover, truncates to the recovery point, rebuilds ResumeState.
    pub fn open_append(path: &std::path::Path, limits: &protocol_core::ProtocolLimits) -> Result<Self, protocol_core::ProtocolError>;
    pub fn open_read_only(path: &std::path::Path, limits: &protocol_core::ProtocolLimits) -> Result<Self, protocol_core::ProtocolError>;
    pub fn resume_state(&self) -> &ResumeState;
    pub fn appender(&mut self) -> &mut protocol_history::HistoryAppender<pack_io::FilePackSink>;
}
//#endregion

//#region 🔖️Sidecar
// .sprc sidecar checkpoint bodies: `<stem>.<hex8-of-body-hash>.sprc`, each a complete ordinary
// .spk pack file, written via pack_io::write_atomic.
pub fn sidecar_path(protocol_path: &std::path::Path, body_hash: &[u8; 32]) -> std::path::PathBuf;
pub fn write_sidecar(protocol_path: &std::path::Path, body_hash: &[u8; 32], pack_bytes: &[u8]) -> Result<(), protocol_core::ProtocolError>;
pub fn read_sidecar(protocol_path: &std::path::Path, body_hash: &[u8; 32]) -> Result<Vec<u8>, protocol_core::ProtocolError>;
//#endregion

//#region 🔖️Recover
pub fn recover_file(path: &std::path::Path, limits: &protocol_core::ProtocolLimits, mode: protocol_format::RecoveryMode) -> Result<protocol_format::RecoveryReport, protocol_core::ProtocolError>;
//#endregion

//#region 🔖️Sync
// Poll-based live tailing, runtime-neutral (no tokio dependency in the type itself).
pub struct TailFollower { /* path, last known end_offset, last_seq */ }
impl TailFollower {
    pub fn open(path: &std::path::Path, from_edit_ordinal: u64) -> Result<Self, protocol_core::ProtocolError>;
    pub fn poll(&mut self) -> Result<Vec<protocol_history::HistoryEdit>, protocol_core::ProtocolError>;
    pub fn last_edit_ordinal(&self) -> u64;
}
//#endregion

//#region 🔖️Compact
pub struct CompactOptions { pub drop_ephemeral: bool, pub keep_snapshots: KeepSnapshots }
pub enum KeepSnapshots { All, LatestPerAlternative, LatestN(u32) }
/// Atomic rewrite via pack_io::write_atomic semantics; physical only, identity-preserving (the
/// multiset of (edit_id, record payload bytes) for non-superseded records + all structural
/// records is unchanged); restarts the commit chain with REC_COMPACTION provenance.
pub fn compact(path: &std::path::Path, options: &CompactOptions, limits: &protocol_core::ProtocolLimits) -> Result<(), protocol_core::ProtocolError>;
//#endregion
}
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
```

---

## protocol (facade) (`protocol/rs`, package `protocol`)

Deps: `protocol_core`, `protocol_format`, `protocol_history`, `protocol_materialize`, `protocol_io`
(all path deps, all re-exported; `protocol_io` re-export is `#[cfg(not(target_arch = "wasm32"))]`).

```rust
//#region 🔖️Reexports
pub use protocol_core::{ProtocolError, ProtocolLimits, RecordHasher, Signer, SignatureVerifier};
pub use protocol_format::{WriteOptions, SprWriter, RecoveryMode, RecoveryReport, VerificationLevel, RecordFrame, FrameCursor, ReverseFrameCursor};
pub use protocol_history::{HistoryLog, HistoryEdit, OpPayload, HistoryOpMeta, HistoryChange, HistoryCheckpoint, HistoryAuthor, HistoryAlternative, EncodeOptions, DecodeOptions, HistoryAppender, HistoryReader, FrontierSummary, AlternativeHead, FrontierComparison, frontier_delta};
pub use protocol_materialize::{ProjectionRecord, ProjectionBodyKind, CheckpointPolicy, MaterializePlan, MaterializeTarget, MaterializeReport, resolve_plan, materialize_with};
#[cfg(not(target_arch = "wasm32"))] pub use protocol_io::{HistoryFile, ResumeState, TailFollower, CompactOptions, KeepSnapshots, compact, recover_file};
//#endregion

//#region 🔖️Compile
pub fn compile_ops(ops: &str, options: &protocol_history::EncodeOptions) -> Result<Vec<u8>, ProtocolError> {
    protocol_history::encode_history(&protocol_history::parse_ops_text(ops)?, options)
}
pub fn decompile_ops(bytes: &[u8], options: &protocol_history::DecodeOptions) -> Result<String, ProtocolError> {
    Ok(protocol_history::print_ops_text(&protocol_history::decode_history(bytes, options)?))
}
//#endregion

//#region 🔖️Sync
/// Zero-copy: one contiguous borrowed byte span of whole record frames covering an edit-ordinal
/// range — itself a valid record stream, shippable as-is in a binary backbone/hub frame.
pub struct RecordSlice<'a> { pub bytes: &'a [u8], pub first_edit_ordinal: u64, pub last_edit_ordinal: u64, pub count: u64 }
pub fn extract_range<'a>(protocol_bytes: &'a [u8], ordinals: std::ops::Range<u64>) -> Result<RecordSlice<'a>, ProtocolError>;
pub fn verify_slice(slice: &[u8], expected_chain: &[u8; 32]) -> Result<(), ProtocolError>;
pub fn content_frontier(protocol_bytes: &[u8]) -> Result<protocol_history::FrontierSummary, ProtocolError>;
//#endregion
```

---

## protocol_testkit (`protocol/testkit/rs`, package `protocol_testkit`)

Deps: `protocol = { path = "../../rs" }`, `pack_testkit = { path = "../../../pack/testkit/rs" }`.
Dev-deps: `criterion = { version = "0.5", default-features = false, features = ["html_reports"] }`.
`[[bench]] name = "protocol" harness = false` → `protocol/testkit/rs/benches/protocol.rs`. Follow
`/Users/ueli/Documents/semio/kernel/3d/brep/rs/benches/kernel.rs` for the criterion harness
pattern (parameterized groups via `benchmark_group` + `BenchmarkId`, `//#region` organized).

```rust
//#region 🔖️Gen
// Inline splitmix64 PRNG (NOT arbitrary/quickcheck/proptest — repo precedent, pack_testkit's
// RecordValueGen). Profiles cover: tiny/typical/adversarial (unicode ids, huge descriptions,
// empty edits, non-canonical timestamp strings that must fall back to tag-0 raw).
pub struct GenProfile { pub edit_count: usize, pub max_ops_per_edit: usize, pub checkpoint_every: usize, pub adversarial: bool }
pub struct HistoryLogGen { state: u64 }
impl HistoryLogGen {
    pub fn new(seed: u64) -> Self;
    pub fn generate(&mut self, profile: &GenProfile) -> protocol::HistoryLog;
}
//#endregion

//#region 🔖️Laws
pub fn assert_history_encode_decode_identity(log: &protocol::HistoryLog);
pub fn assert_history_canonical_stable(log: &protocol::HistoryLog);          // encode twice, byte-identical
pub fn assert_ops_protocol_bidirectional(ops_text: &str);                     // parse_ops_text -> encode -> decode -> print_ops_text is a fixpoint
pub fn assert_streamed_equals_buffered(log: &protocol::HistoryLog);           // HistoryAppender-per-record == encode_history(whole log)
pub fn assert_zero_copy(bytes: &[u8]);                                        // sweep: every RecordFrame::payload() pointer-range lies within `bytes`
pub fn assert_chain_detects_tamper(bytes: &[u8]);                             // flip one byte anywhere before the last commit -> Full verification must error
pub fn assert_recovery_truncates_to_commit(bytes: &[u8], level: pack_testkit::CorruptionLevel);  // truncate at every byte (quick: sampled, exhaustive: all) -> recover -> valid prefix ends at a real commit
pub fn assert_compaction_identity(bytes: &[u8]);                              // decode before/after compact: same non-superseded (edit_id, payload) multiset + all structural records
//#endregion

//#region 🔖️Corrupt
// Reuse pack_testkit::{fuzz_truncation, fuzz_bit_flips, CorruptionLevel, CorruptionReport} —
// closure-generic, zero duplication. Law: cases_panicked must always be empty.
//#endregion

//#region 🔖️Golden
pub use pack_testkit::golden_hash_hex;
//#endregion
```

---

## protocol_cli (`protocol/cli/rs`, package `protocol_cli`)

Deps: `protocol = { path = "../../rs" }`. `[[bin]] name = "protocol" path = "lib.rs"` alongside
`[lib] path = "lib.rs"` — identical single-file-serves-both-targets trick as `pack_cli`. Put
`pub fn main_impl(args: &[String]) -> i32` in `lib.rs` plus a tiny `fn main()` at the bottom,
`#[cfg(not(test))]`-reachable only from the bin target. No app-crate dependencies — schema-free by
construction (record payloads are opaque op-text; no dsl_schema RecordSpec needed for these
subcommands, unlike pack_cli's `to-dsl`/`from-dsl`).

```
protocol inspect <file>                       # header, commit chain (all generations), record
                                                # counts by kind, dictionaries, projections, indexes
protocol verify <file> [--level=trusted|standard|full]
                                                # full: + index-rebuild equality + chain recompute
protocol hash <file>                           # (commit_seq, chain_hash) identity
protocol log <file> [--limit N] [--actor ID] [--alternative ID] [--reverse]
                                                # timeline text output: checkpoint lanes, actor
                                                # column, amend-supersession markers
protocol compile <doc.ops> [--out doc.spr]     # ops text -> binary (the bidirectional law, schema-free)
protocol decompile <doc.spr> [--out doc.ops]   # binary -> ops text
protocol diff <a.spr> <b.spr>                  # record-level: common prefix, only-in-a/only-in-b by (ordinal, id), payload-hash mismatches; exit 1 if differ
protocol compact <file> [--out FIXED]          # calls protocol_io::compact
protocol repair <file> [--truncate-torn-tail] [--rebuild-indexes]
protocol upgrade <file>                        # RecordUpcaster-driven rewrite (v1: no-op passthrough, hook exists)
```

Exit codes 0/1/2; never panics on corrupt input (catch malformed input as `Err`, print to stderr,
return 2). Hand-rolled `parse_args`, no external CLI-parsing crate.

---

## Amendment (INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING, 🗄️) — semantics + wire crates

This ticket's format-only scope (`protocol_core/format/history/materialize/io` + facade/testkit/cli)
is extended, not replaced, by a sibling campaign that needs `protocol` to also be the command and
collaboration **semantics** layer (traits, causal metadata, conflict/CRDT rules) extracted from
`vcs`/`framework-core`, plus a hub wire-frame crate. Four more crates join the family below. The
package list becomes: `protocol_core`, `protocol_command`, `protocol_causal`, `protocol_crdt`,
`protocol_format`, `protocol_history`, `protocol_materialize`, `protocol_io`, `protocol_wire`,
`protocol` (facade), `protocol_testkit`, `protocol_cli` — 12 total. Same repo conventions apply
verbatim (single-file lib.rs, regions, 🎞️ emoji docstrings, no `std::io::Error` leakage, wasm32-clean
except `protocol_io`/`protocol_wire`'s native halves).

**Naming**: keep the trait names `Operation`/`OperationDiff`/`OpText` unchanged when they move here
from `vcs` — do not rename to `Command*` (framework/core already has a distinct UI-invocation
`Command` layer; a second meaning would create a repo-wide homonym). The spec's "CommandDescriptor"
is `OperationDescriptor` here.

**Revised hard dependency rule**: no protocol crate depends on `vcs`, `semio-framework-core`,
`semio-framework-plugin`, `ui_wgpu`, or `pack_value` — unchanged in spirit, now explicit about
`framework-plugin`/`ui_wgpu` because `protocol_core`/`protocol_causal` absorb types that today live
in `framework/core` (which itself depends on `ui_wgpu`). This is *why* HLC/OpDag/OperationEnvelope
move down into protocol rather than staying in framework-core: a headless `db` server consuming
causal primitives must never pull in the UI type surface.

---

### protocol_core — additions (`protocol/core/rs`)

Same crate as above, same deps (`pack_core`, `thiserror`), extended with:

```rust
//#region 🔖️Identifiers
// Moved from framework/core/rs/lib.rs 🔖️Identifiers (L5768-5838). Serde-transparent newtypes,
// shapes unchanged from their framework-core originals.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OperationId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ActorId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DocumentId(pub String);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct DocumentVersion(pub u64);
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SchemaId(pub String);
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SchemaVersion(pub u32);
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PayloadHash(pub [u8; 32]);
//#endregion

//#region 🔖️HybridLogicalTimestamp
// Moved from framework/core (L5840-5881). FIX vs the original: cmp_key gains `actor` as a total-
// order tiebreak (the original omitted it, so two ticks with equal physical_ms/logical from
// different actors compared Equal — a real ordering bug). Real `Ord`/`PartialOrd` now derive from
// cmp_key, not from field declaration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HybridLogicalTimestamp { pub actor: u64, pub physical_ms: u64, pub logical: u64 }
impl HybridLogicalTimestamp {
    pub fn new(actor: u64, physical_ms: u64) -> Self { Self { actor, physical_ms, logical: 0 } }
    pub fn tick(&mut self, physical_ms: u64) { /* advance-or-increment, unchanged algorithm */ }
    pub fn merge(&mut self, other: &Self) { /* max-then-+1, unchanged algorithm */ }
    pub fn cmp_key(&self) -> (u64, u64, u64) { (self.physical_ms, self.logical, self.actor) }
}
impl Ord for HybridLogicalTimestamp { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.cmp_key().cmp(&other.cmp_key()) } }
impl PartialOrd for HybridLogicalTimestamp { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
//#endregion

//#region 🔖️Policies
// UndoPolicy moved from vcs/rs (unchanged variants); MergeStrategyKind + DocumentKind moved from
// framework/core L6636-6668 (unchanged variants). New: ConflictRule, the per-operation conflict
// declaration surface today's code lacks (vcs::merge_concurrent_diffs collapses everything to
// absorb regardless of declared strategy — protocol_crdt fixes this using ConflictRule).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UndoPolicy { ExactBaseOnly, TransformAgainstConcurrent, SemanticUndo, CompensatingAction }
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MergeStrategyKind { LwwRegister, OrderedSequence, TextSequence, TombstonedGraphSet, ContentAddressedBlob }
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConflictRule { Commutes, Transform, Merge(MergeStrategyKind), Crdt(MergeStrategyKind) }
//#endregion

//#region 🔖️StateClass
// New: the explicit persistent/shared-ui/local-ui/preview/effect separation the db spec requires.
// Carried on OperationDescriptor (protocol_command) and on wire envelopes (protocol_wire).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StateClass { Persistent, SharedUi, LocalUi, Preview, Effect }
//#endregion
```

---

### protocol_command (`protocol/command/rs`, package `protocol_command`)

Deps: `protocol_core`, `dsl_core = { path = "../../../dsl/core/rs" }` (for `TextError` only —
NOT `dsl_schema`, ops stay schema-opaque here exactly like protocol_history), `serde`, `serde_json`,
`blake3` (fingerprints). Must compile for `wasm32-unknown-unknown`.

```rust
//#region 🔖️Operation
// Moved from vcs/rs/lib.rs L606-668, generic parameter and method set UNCHANGED except the two new
// defaulted methods and reconcile's return type. Every existing app `impl` recompiles with only an
// import-path change (`vcs::Operation` -> `protocol::Operation`) once this lands.
pub trait OperationDiff<P>: Clone + Default + serde::Serialize + serde::de::DeserializeOwned {
    fn apply(&self, base: &P) -> P;
    fn absorb(&mut self, other: Self);
}
pub trait Operation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    type Diff: OperationDiff<P>;
    fn diff(&self, base: &P) -> Self::Diff;
    fn backwards(&self, base: &P) -> Vec<Self>;
    fn operation_id(&self) -> Option<protocol_core::OperationId> { None }
    fn dependencies(&self) -> Vec<protocol_core::OperationId> { Vec::new() }
    fn base_version(&self) -> Option<protocol_core::DocumentVersion> { None }
    fn author_id(&self) -> Option<protocol_core::ActorId> { None }
    fn timestamp(&self) -> Option<protocol_core::HybridLogicalTimestamp> { None }
    fn undo_policy(&self) -> protocol_core::UndoPolicy { protocol_core::UndoPolicy::ExactBaseOnly }
    fn merge_strategy(&self) -> protocol_core::MergeStrategyKind { protocol_core::MergeStrategyKind::LwwRegister }
    // NEW, both defaulted so every existing impl compiles unchanged:
    fn conflict_rule(&self) -> protocol_core::ConflictRule { protocol_core::ConflictRule::Merge(self.merge_strategy()) }
    fn state_class(&self) -> protocol_core::StateClass { protocol_core::StateClass::Persistent }
    // reconcile's return type changes from vcs::StudioConflict (which drags studio types down)
    // to this crate's own ReconcileReport; vcs maps ReconcileReport -> StudioConflict at its edge.
    fn reconcile(&self, projection: P) -> (P, Vec<ReconcileReport>) { (projection, Vec::new()) }
    fn validate(&self, _projection: &P) -> Result<(), String> { Ok(()) }
}
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReconcileReport { pub id: String, pub message: String, pub severity: ReconcileSeverity }
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReconcileSeverity { Info, Warning, Blocking }
//#endregion

//#region 🔖️OpText
// Moved verbatim from vcs/rs L236. dsl_derive's DslOps macro emits `impl ::protocol::OpText`
// instead of `impl ::vcs::OpText` after the kernel cut-over wave — this crate's shape is otherwise
// identical to vcs's today, so the flip is a pure re-target.
pub trait OpText: Sized {
    fn print_op(&self) -> String;                          // one line, no '\n'
    fn parse_op(line: &str) -> Result<Self, dsl_core::TextError>;
}
//#endregion

//#region 🔖️Meta
// Moved from vcs/rs L59 (OperationMeta) and L73 (Edit<Operation>). Field shapes unchanged except
// `timestamp: protocol_core::HybridLogicalTimestamp` (was the framework-core type at the same path).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperationMeta {
    pub operation_id: Option<protocol_core::OperationId>,
    pub dependencies: Vec<protocol_core::OperationId>,
    pub base_version: u64,
    pub author_id: Option<protocol_core::ActorId>,
    pub timestamp: protocol_core::HybridLogicalTimestamp,
    pub undo_policy: protocol_core::UndoPolicy,
    pub payload_hash: Option<protocol_core::PayloadHash>,
}
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Edit<Op> {
    pub id: String, pub actor: Option<String>,
    pub forwards: Vec<Op>, pub backwards: Vec<Op>,
    pub operation_meta: Vec<OperationMeta>,
    pub description: Option<String>, pub coalesce_key: Option<String>,
    pub sequence_number: i32, pub started_at: String, pub finished_at: Option<String>,
}
//#endregion

//#region 🔖️Collection
// Moved verbatim from vcs/rs L447-604: Identified<TId>, Patchable<TPatch>, ItemPatch, CollectionDiff,
// CollectionOperation<TId,TItem,TPatch> {Add,Remove,Move,Patch}, apply_collection_operation,
// invert_collection_operation, collection_diff_from_operation. No behavior change.
pub trait Identified<TId> { fn id(&self) -> &TId; }
pub trait Patchable<TPatch> { fn apply_patch(&mut self, patch: &TPatch); fn diff_patch(&self, other: &Self) -> Option<TPatch>; }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CollectionOperation<TId, TItem, TPatch> { Add { id: TId, item: TItem, at: usize }, Remove { id: TId }, Move { id: TId, to: usize }, Patch { id: TId, patch: TPatch } }
// ... apply_collection_operation / invert_collection_operation / collection_diff_from_operation /
//     CollectionDiff / ItemPatch signatures identical to vcs's today.
//#endregion

//#region 🔖️Descriptor
// New: runtime type-erased registry, mirrors vcs::CodecRegistry's OnceLock<RwLock<HashMap>> pattern.
pub struct OperationDescriptor {
    pub id: protocol_core::SchemaId, pub schema_version: protocol_core::SchemaVersion,
    pub state_class: protocol_core::StateClass, pub conflict_rule: protocol_core::ConflictRule,
    pub fingerprint: [u8; 32],   // blake3 over the descriptor's canonical bytes
}
pub fn register_operation_descriptor(descriptor: OperationDescriptor);
pub fn operation_descriptor(schema: &str) -> Option<OperationDescriptor>;
//#endregion

//#region 🔖️Upcast
pub trait OperationUpcaster<Op> { fn upcast(&self, from_version: protocol_core::SchemaVersion, op: Op) -> Op; }
// LAW: upcast(upcast(x)) == upcast(x) — idempotence at the target version.
//#endregion

//#region 🔖️Events
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperationEvent { pub operation_id: protocol_core::OperationId, pub state_class: protocol_core::StateClass, pub payload: serde_json::Value }
//#endregion

//#region 🔖️Outcome
// The five-channel separation framework/core's InvocationResult later maps onto.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CommandOutcome<Diff> { pub persistent: Vec<Diff>, pub shared_ui: Vec<Diff>, pub local_ui: Vec<Diff>, pub preview: Vec<Diff>, pub effects: Vec<OperationEvent> }
//#endregion
```

---

### protocol_causal (`protocol/causal/rs`, package `protocol_causal`)

Deps: `protocol_core`, `protocol_command`, `serde`, `serde_json`.

```rust
//#region 🔖️Envelope
// Moved from framework/core L6246 (OperationEnvelope), L6121 (DocumentDiff), L6137 (InverseOperation).
// Fields unchanged; diff/inverse stay schema-erased (serde_json::Value payload).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperationEnvelope { pub operation_id: protocol_core::OperationId, pub document_id: protocol_core::DocumentId, pub actor: protocol_core::ActorId, pub dependencies: Vec<protocol_core::OperationId>, pub diff: DocumentDiff, pub inverse: InverseOperation, pub timestamp: protocol_core::HybridLogicalTimestamp }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentDiff { pub schema: String, pub payload: serde_json::Value }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InverseOperation { pub schema: String, pub inverse_diff: serde_json::Value }
//#endregion

//#region 🔖️OpDag
// Moved verbatim from framework/core L6266-6379 including its existing unit tests (L6488-6572).
// No behavior change: insert -> InsertResult{Applied,Pending,AlreadyApplied}, ready(),
// drain_applied_envelopes() (causal order), seed_applied(), OpDagError::Duplicate.
pub struct OpDag { /* unchanged internals */ }
pub enum InsertResult { Applied, Pending, AlreadyApplied }
pub enum OpDagError { Duplicate }
impl OpDag {
    pub fn new() -> Self;
    pub fn insert(&mut self, envelope: OperationEnvelope) -> Result<InsertResult, OpDagError>;
    pub fn ready(&self) -> Vec<protocol_core::OperationId>;
    pub fn drain_applied_envelopes(&mut self) -> Vec<OperationEnvelope>;
    pub fn seed_applied(&mut self, operation_id: protocol_core::OperationId);
}
//#endregion

//#region 🔖️Frontier
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrontierSummary { pub document_id: protocol_core::DocumentId, pub head_edit_ordinal: u64, pub head_edit_id: String, pub last_commit_seq: u64, pub chain_hash: [u8; 32] }
pub enum FrontierComparison { Equal, Ahead, Behind, Diverged { common_edit_count: u64 } }
pub fn frontier_delta(local: &FrontierSummary, remote: &FrontierSummary) -> FrontierComparison;
// Same shape as protocol_history::FrontierSummary/frontier_delta (that crate's version is the
// on-disk-log-derived twin; this one is the runtime/wire twin used by db and framework/sync without
// needing a full history-log decode). Both crates may exist; do not attempt to unify them — they
// serve different layers (durable log vs live runtime state).
//#endregion

//#region 🔖️Transform
pub enum TransformOutcome<Op> { Unchanged(Op), Transformed(Op), Conflict(String) }
pub trait OperationTransform<P>: protocol_command::Operation<P> {
    fn transform(&self, against: &Self) -> TransformOutcome<Self> where Self: Sized;
}
//#endregion

//#region 🔖️Bridge
// Moved from vcs/rs (was operation_envelope_from_edit); generic over Op: Operation<P> + OpText.
pub fn operation_envelope_from_edit<P, Op: protocol_command::Operation<P>>(edit: &protocol_command::Edit<Op>, document_id: &protocol_core::DocumentId) -> Vec<OperationEnvelope>;
// Addendum (double-delivery fix, PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS): extracted the id half
// of operation_envelope_from_edit's per-op fallback chain (operation_meta[i] field, else the Op
// trait method, else `{edit.id}#{i}`) so `store::merge_remote_snapshot` can recognize a
// snapshot-carried edit as one it already ingested via a prior `BackboneMessage::Operations`
// message under that edit's wire id, without paying for encode_op/backwards.
pub fn operation_ids_for_edit<P, Op: protocol_command::Operation<P>>(edit: &protocol_command::Edit<Op>) -> Vec<protocol_core::OperationId>;
//#endregion
```

---

### protocol_crdt (`protocol/crdt/rs`, package `protocol_crdt`)

Deps: `protocol_core`, `protocol_command`.

```rust
//#region 🔖️Merge
// Replaces vcs::merge_concurrent_diffs (vcs/rs L680), which today collapses all five
// MergeStrategyKind variants to absorb() regardless of declared strategy — the known
// inconsistency this crate fixes with real per-strategy behavior.
pub fn merge_concurrent_diffs<P, D: protocol_command::OperationDiff<P>>(strategy: protocol_core::MergeStrategyKind, existing: D, incoming: D, existing_meta: &protocol_command::OperationMeta, incoming_meta: &protocol_command::OperationMeta) -> D;
//#endregion
//#region 🔖️Lww
// HLC-arbitrated register: the diff whose OperationMeta.timestamp is greater (via
// HybridLogicalTimestamp::Ord, actor-tiebroken) wins; ties are impossible post-fix.
//#endregion
//#region 🔖️OrderedSequence
// Stable-anchor sequence merge: AnchorId dense-order keys (fractional-index style), concurrent
// inserts at the same anchor are ordered by (timestamp, actor) deterministically on both replicas.
pub struct AnchorId(pub Vec<u8>); // dense order key, comparable via Ord
//#endregion
//#region 🔖️TextSequence
// Character/grapheme-range merge over two concurrent text diffs; non-overlapping ranges compose,
// overlapping ranges fall back to Lww on the overlapping span only.
//#endregion
//#region 🔖️TombstonedGraphSet
// Node/edge add-wins, remove leaves a tombstone that outranks a concurrent add of the same id only
// if the tombstone's timestamp is greater (else the add resurrects it) — declared, testable law.
//#endregion
//#region 🔖️ContentAddressedBlob
// Two concurrent blob-extent writes: last-writer-wins by content hash equality short-circuit (equal
// hashes are not a conflict at all), else Lww by timestamp.
//#endregion
// LAWS (protocol_testkit): commutativity merge(a,b) == merge(b,a) on concurrent metas per strategy;
// idempotence merge(a,a) == a; LWW arbitration strictly follows HybridLogicalTimestamp::Ord.
```

---

### protocol_wire (`protocol/wire/rs`, package `protocol_wire`)

Deps: `protocol_core`, `protocol_causal`, `protocol_command`, `serde`. Native halves (if any
transport helpers are added later) go behind `#[cfg(not(target_arch = "wasm32"))]`; the frame types
themselves must stay wasm32-clean since the browser client encodes/decodes them directly.

```rust
//#region 🔖️Lane
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane { Command = 0, Preview = 1 }
//#endregion

//#region 🔖️ClientFrame
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClientFrame {
    Hello { wire_version: u32, protocol_version: u32, schema: String, pack_schema_hash: [u8; 32], actor: protocol_core::ActorId, token: Option<String>, resume_token: Option<String>, frontier: Option<protocol_causal::FrontierSummary> },
    Commands { batch_id: u64, envelopes: Vec<protocol_causal::OperationEnvelope> },
    FrontierAdvertise { frontier: protocol_causal::FrontierSummary },
    PreviewPublish { key: String, seq: u64, payload: Vec<u8> },
    Presence { peer: serde_json::Value },
    CreditGrant { n: u32 },
    Bye,
}
//#endregion

//#region 🔖️ServerFrame
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Bootstrap { None, Snapshot { pack_hash: [u8; 32], inline: Option<Vec<u8>> }, Tail }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ApplyOutcome { Accepted, Transformed { envelope: protocol_causal::OperationEnvelope }, Rejected { reason: String } }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AckStage { Received, Persisted, Applied { outcome: ApplyOutcome } }
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServerFrame {
    Welcome { session_id: String, resume_token: String, server_frontier: protocol_causal::FrontierSummary, bootstrap: Bootstrap },
    SnapshotChunk { seq: u32, bytes: Vec<u8> },
    SnapshotDone { seq_count: u32 },
    Commands { envelopes: Vec<protocol_causal::OperationEnvelope>, origin: protocol_core::ActorId, frontier: protocol_causal::FrontierSummary },
    Ack { batch_id: u64, stages: Vec<AckStage>, frontier: protocol_causal::FrontierSummary },
    Preview { actor: protocol_core::ActorId, key: String, seq: u64, payload: Vec<u8> },
    Presence { peers: Vec<serde_json::Value> },
    CreditGrant { n: u32 },
    Error { code: String, message: String },
}
//#endregion

//#region 🔖️Codec
// Binary frame encode/decode built on protocol_core::ByteWriter/ByteReader (via pack_core) — a
// lane byte followed by a serde-encoded frame body (bincode-shaped, hand-rolled, no serde_json on
// the wire). v1 may implement this as `lane: u8` + varint-len-prefixed serde_json bytes if a full
// hand-rolled binary encoding is deferred — record the deviation in your PR summary if so; the
// frame *types* above are the frozen contract, the byte encoding has one degree of freedom.
pub fn encode_client_frame(frame: &ClientFrame, lane: Lane) -> Vec<u8>;
pub fn decode_client_frame(bytes: &[u8]) -> Result<(Lane, ClientFrame), protocol_core::ProtocolError>;
pub fn encode_server_frame(frame: &ServerFrame, lane: Lane) -> Vec<u8>;
pub fn decode_server_frame(bytes: &[u8]) -> Result<(Lane, ServerFrame), protocol_core::ProtocolError>;
// LAW: decode(encode(frame)) == frame for every frame variant (protocol_testkit).
//#endregion
```

---

### protocol (facade) — additional re-exports

```rust
pub use protocol_core::{OperationId, ActorId, DocumentId, DocumentVersion, SchemaId, SchemaVersion, PayloadHash, HybridLogicalTimestamp, UndoPolicy, MergeStrategyKind, ConflictRule, StateClass};
pub use protocol_command::{Operation, OperationDiff, OpText, OperationMeta, Edit, Identified, Patchable, CollectionOperation, CollectionDiff, ItemPatch, OperationDescriptor, register_operation_descriptor, operation_descriptor, OperationUpcaster, OperationEvent, CommandOutcome, ReconcileReport, ReconcileSeverity};
pub use protocol_causal::{OperationEnvelope, DocumentDiff, InverseOperation, OpDag, InsertResult, OpDagError, FrontierSummary as RuntimeFrontierSummary, FrontierComparison as RuntimeFrontierComparison, frontier_delta as runtime_frontier_delta, TransformOutcome, OperationTransform, operation_envelope_from_edit, operation_ids_for_edit};
pub use protocol_crdt::merge_concurrent_diffs;
pub use protocol_wire::{Lane, ClientFrame, ServerFrame, Bootstrap, ApplyOutcome, AckStage, encode_client_frame, decode_client_frame, encode_server_frame, decode_server_frame};
```

(`RuntimeFrontierSummary`/`runtime_frontier_delta` avoid a name collision with
`protocol_history::FrontierSummary`/`frontier_delta` re-exported earlier — both stay reachable at
distinct facade names since they serve different layers per the causal crate's note above.)

---

### Additional workspace-wide notes for this amendment

- Same scaffolding rule: per-crate agents write only their own `Cargo.toml` + `lib.rs`; a single
  closing agent adds all 12 crates to root `Cargo.toml` workspace members, `.vscode/launch.json`,
  and each crate's `project.json`/`script.ts` in one pass.
- `protocol_command`/`protocol_causal`/`protocol_crdt`/`protocol_wire` must each add inline
  `//#region 🧪️Tests` covering: OpText round-trip on a hand-built dummy `(P, Op)` pair;
  `Edit`/`OperationMeta` serde round-trip; `OperationDescriptor` fingerprint stability (golden hex
  pin, `pack_testkit::golden_hash_hex`); `OpDag` permutation-convergence (quick: a handful of
  hand-built 3-4-node DAGs in all topological orders; exhaustive, in `protocol_testkit`: larger
  random DAGs); CRDT commutativity/idempotence per strategy; wire frame `decode(encode(f)) == f`
  for one instance of every enum variant.
- `protocol_testkit` (this ticket's original scope) additionally gets law fns:
  `assert_op_text_round_trip`, `assert_op_dag_convergence`, `assert_crdt_commutative`,
  `assert_crdt_idempotent`, `assert_wire_frame_round_trip` — same file, new `//#region 🔖️Laws`
  entries alongside the existing history/format laws.

---

## Workspace-wide requirements

- Every crate's `Cargo.toml` starts with the standard header (copy from
  `/Users/ueli/Documents/semio/pack/rs/Cargo.toml`): `version = "0.1.0"`, `edition = "2021"`,
  `rust-version = "1.88"`, one-line `description`, blank line, `[lints] workspace = true`, then
  `[lib]`, then `[dependencies]`.
- Do NOT add yourself to the root `Cargo.toml` `[workspace] members` list, do NOT create your
  crate's `project.json`/`script.ts`, do NOT touch `.vscode/launch.json` — a separate closing
  agent scaffolds those for all eight crates in one pass (pack precedent, avoids file conflicts).
  Create exactly your own `Cargo.toml` + `lib.rs` (+ `benches/protocol.rs` for protocol_testkit
  only) and nothing else.
- Write scratch/progress notes only inside this ticket folder
  (`.repo/🎫️/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/`), as `.txt` files (gitignored `*.log` is
  silently dropped by `ticket_close` — use `.txt`).
- Include inline `//#region 🧪️Tests` unit tests proving your crate's own laws in isolation
  (hand-built byte literals for protocol_format: header/frame round-trip, commit-chain
  verification on a small hand-built file, skip-unknown, truncation-at-every-byte on a tiny
  fixture). Do not write a workspace test runner — that's the closing step.
- The rename of the existing Blockly-like `protocol` technology to `playbook` MUST have already
  landed before you start (it frees the `protocol/` directory and crate name) — if `protocol/rs`
  still contains the old Blockly-editor code when you begin, STOP and report back rather than
  overwriting it.
