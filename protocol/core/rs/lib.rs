//! 🎞️ Protocol command/collaboration-semantics foundation: stable identifiers, hybrid-logical
//! timestamps, the shared error/limit ceilings, the on-disk record-kind/flag vocabulary,
//! lossless-by-construction scalar codecs, a deterministic dictionary builder/reader, and the
//! crypto trait seam every other `protocol_*` crate path-deps on. Frozen contract:
//! `.repo/🎫/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_core` + its
//! `## Amendment` section).

//#region 🔖Errors
/// @emoji 🚨 The one error type every `protocol_*` public fn returns; never leaks `std::io::Error`.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ProtocolError {
    #[error(transparent)]
    Pack(#[from] pack_core::PackError),
    #[error("chain mismatch at commit {commit_seq}")]
    ChainMismatch { commit_seq: u64 },
    #[error("torn tail at offset {0}")]
    TornTail(u64),
    #[error("unknown critical record kind {0:#x}")]
    UnknownCriticalRecord(u8),
    #[error("dictionary index out of range: {0}")]
    DictMiss(u32),
    #[error("dictionary out of order: expected base_count {expected}, got {actual}")]
    DictOutOfOrder { expected: u32, actual: u32 },
    #[error("signature verification required but no verifier supplied")]
    VerifierRequired,
    #[error("signature invalid for commit {commit_seq}")]
    SignatureInvalid { commit_seq: u64 },
    #[error("frame back_len mismatch at offset {0}")]
    FrameFraming(u64),
    #[error("limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("malformed {what} at offset {offset}: {detail}")]
    Malformed { what: &'static str, offset: u64, detail: String },
    #[error("io error: {0}")]
    Io(String),
}
//#endregion 🔖Errors

//#region 🔖Limits
/// @emoji 🛡️ Corruption-hardening ceilings every decoder in this crate family must validate
/// against BEFORE allocating — mirrors `pack_core::PackLimits`'s stated invariant.
#[derive(Clone, Debug)]
pub struct ProtocolLimits {
    pub max_file_len: u64,
    pub max_frame_len: u64,
    pub max_record_count: u64,
    pub max_dict_entries: u32,
    pub max_op_count_per_edit: u32,
    pub max_total_alloc: u64,
}

impl Default for ProtocolLimits {
    fn default() -> Self {
        Self {
            max_file_len: 64 * 1024 * 1024 * 1024,
            max_frame_len: 2 * 1024 * 1024 * 1024,
            max_record_count: 256_000_000,
            max_dict_entries: 1_000_000,
            max_op_count_per_edit: 100_000,
            max_total_alloc: 4 * 1024 * 1024 * 1024,
        }
    }
}
//#endregion 🔖Limits

//#region 🔖RecordKinds
// Plain `pub const` u8s (mirrors pack_core::SegmentKind convention but this family uses bare
// u8 kind bytes directly in the frame, no wrapper newtype — simpler, and every downstream crate
// matches on the byte).
/// @emoji 🔚 Marks the end of the record stream.
pub const REC_END: u8 = 0x00;
/// @emoji 📄 The document-identity record: doc id + schema id.
pub const REC_DOC: u8 = 0x01;
/// @emoji 🎭 A delta into the actor-id dictionary.
pub const REC_ACTOR_DICT: u8 = 0x02;
/// @emoji 🔤 A delta into the general string dictionary.
pub const REC_STR_DICT: u8 = 0x03;
/// @emoji ✏️ One edit (a batch of forward ops, optionally backwards ops + explicit meta).
pub const REC_EDIT: u8 = 0x04;
/// @emoji 💾 One named change (a save point referencing edits).
pub const REC_CHANGE: u8 = 0x05;
/// @emoji 🚩 One checkpoint (a durable milestone referencing changes).
pub const REC_CHECKPOINT: u8 = 0x06;
/// @emoji 🌿 One named alternative (branch) referencing checkpoints.
pub const REC_ALTERNATIVE: u8 = 0x07;
/// @emoji 🎯 Marks the currently-active alternative.
pub const REC_ACTIVE: u8 = 0x08;
/// @emoji 🏔️ A frontier summary snapshot.
pub const REC_FRONTIER: u8 = 0x09;
/// @emoji 📸 A materialized projection body (opaque to this crate family).
pub const REC_PROJECTION: u8 = 0x0A;
/// @emoji 🔎 An advisory, rebuildable offset index.
pub const REC_INDEX: u8 = 0x0B;
/// @emoji ⛓️ The commit frame: hash-chains everything written since the previous commit.
pub const REC_COMMIT: u8 = 0x0C;
/// @emoji ✍️ A detached signature over a commit's chain hash.
pub const REC_SIGNATURE: u8 = 0x0D;
/// @emoji 🕳️ A tombstone recording a redaction; the original bytes are physically gone.
pub const REC_REDACTION: u8 = 0x0E;
/// @emoji ⬆️ Records a schema-version upcast applied during a rewrite.
pub const REC_UPCAST: u8 = 0x0F;
/// @emoji 👻 Ephemeral/preview-lane data, dropped freely by compaction.
pub const REC_EPHEMERAL: u8 = 0x10;
/// @emoji 🔏 Marks a range of the file as sealed (immutable, already compacted).
pub const REC_SEALED: u8 = 0x11;
/// @emoji ♻️ A compaction batch: sealed, replayable inner record frames.
pub const REC_COMPACTION: u8 = 0x12;
/// @emoji ⬜ Padding, always safely skippable.
pub const REC_PADDING: u8 = 0x7F;
// Extension range 0x40..=0x7E is caller-defined, never critical unless the frame's critical bit is set.

/// @emoji ❗ True iff an unrecognized `kind` byte with this value must abort the reader rather
/// than being skipped (see `protocol_format`'s skip-unknown rule).
pub fn is_critical_kind(kind: u8) -> bool {
    matches!(
        kind,
        REC_DOC | REC_EDIT | REC_CHANGE | REC_CHECKPOINT | REC_ALTERNATIVE | REC_ACTIVE | REC_COMMIT | REC_ACTOR_DICT | REC_STR_DICT
    )
}
//#endregion 🔖RecordKinds

//#region 🔖Flags
// Header required/optional flags (32-byte header, see protocol_format).
/// @emoji ⛓️ Required flag bit: every commit frame's `chain_hash` must verify.
pub const REQUIRED_HASH_CHAIN: u32 = 1 << 0;
/// @emoji ✍️ Required flag bit: every commit frame must carry a valid `REC_SIGNATURE`.
pub const REQUIRED_SIGNED: u32 = 1 << 1;
/// @emoji 🔒 Required flag bit: reserved for encryption, never set by this crate family.
pub const REQUIRED_ENCRYPTED: u32 = 1 << 2;
/// @emoji 🧮 Optional flag bit: the document body was encoded in canonical form.
pub const OPTIONAL_CANONICAL: u32 = 1 << 0;
/// @emoji 📸 Optional flag bit: this file contains at least one `REC_PROJECTION`.
pub const OPTIONAL_HAS_PROJECTIONS: u32 = 1 << 1;
/// @emoji 🔎 Optional flag bit: this file contains at least one `REC_INDEX`.
pub const OPTIONAL_HAS_INDEX: u32 = 1 << 2;
/// @emoji 🕳️ Optional flag bit: this file contains at least one `REC_REDACTION`.
pub const OPTIONAL_REDACTED: u32 = 1 << 3;
// Frame flags byte (per-record, not header): bit0 compressed, bit1 critical, bits2..4 = codec id (0..=7).
/// @emoji 🗜️ Frame flags bit: the payload is compressed (see `frame_codec_id`).
pub const FRAME_FLAG_COMPRESSED: u8 = 1 << 0;
/// @emoji ❗ Frame flags bit: an unrecognized `kind` carrying this bit aborts the reader.
pub const FRAME_FLAG_CRITICAL: u8 = 1 << 1;

/// @emoji 🗜️ Extracts the 3-bit codec id (bits 2..4) from a frame flags byte.
pub fn frame_codec_id(flags: u8) -> u8 {
    (flags >> 2) & 0b111
}

/// @emoji 🏗️ Assembles a frame flags byte from its three logical fields.
pub fn frame_flags(compressed: bool, critical: bool, codec: u8) -> u8 {
    (compressed as u8) | ((critical as u8) << 1) | ((codec & 0b111) << 2)
}
//#endregion 🔖Flags

//#region 🔖Scalars
/// @emoji 🧮 Tagged, lossless-by-construction scalar codecs shared by `protocol_history`'s
/// payload codecs. `out`/`input` follow `pack_core::ByteWriter`/`ByteReader` conventions exactly
/// (they take `&mut pack_core::ByteWriter` / `&mut pack_core::ByteReader<'_>` directly — no
/// reimplementation of the varint/byte primitives).
pub mod scalar {
    use pack_core::{ByteReader, ByteWriter, PackError};

    //#region 🔖Timestamp
    // Timestamp tag: 0 = raw string (len varint + utf8), 1 = epoch-ms varint (iff reprint is
    // byte-exact vs source), 2 = zigzag-varint delta-ms vs previous tag-1/2 timestamp in stream.
    //
    // 🎯 Design choice (contract leaves the round-trip heuristic to the implementer): a timestamp
    // only ever gets tag 1/2 when `format_rfc3339_ms(parse_rfc3339_ms(raw)) == raw` byte-for-byte;
    // any input that isn't already exactly `YYYY-MM-DDTHH:MM:SS[.fff]Z` (fraction present iff its
    // ms component is nonzero) safely falls back to tag 0 raw text — correctness never depends on
    // the parser/formatter being calendar-complete, only on the equality check.

    /// @emoji ⏱️ Writes `raw` using the most compact of the three timestamp tags that reproduces
    /// it byte-exact. Returns `Some(epoch_ms)` iff tag 1/2 was written (thread this back in as
    /// `prev_epoch_ms` on the next call to keep deltas short); `None` iff tag 0 (raw) was written.
    pub fn write_timestamp(out: &mut ByteWriter, raw: &str, prev_epoch_ms: Option<i64>) -> Option<i64> {
        let Some(epoch_ms) = round_trip_epoch_ms(raw) else {
            write_raw_timestamp(out, raw);
            return None;
        };
        match prev_epoch_ms {
            Some(prev) => {
                out.write_u8(2);
                out.write_varint_i64(epoch_ms - prev);
                Some(epoch_ms)
            }
            None if epoch_ms >= 0 => {
                out.write_u8(1);
                out.write_varint_u64(epoch_ms as u64);
                Some(epoch_ms)
            }
            None => {
                // Tag 1 stores an unsigned varint; a pre-1970 timestamp with no prior delta base
                // has no lossless absolute encoding here, so it falls back to raw text.
                write_raw_timestamp(out, raw);
                None
            }
        }
    }

    /// @emoji ⏱️ Reads one tagged timestamp, returning the reconstructed string and, iff tag 1/2,
    /// the `epoch_ms` to feed back in as `prev_epoch_ms` for the next call.
    pub fn read_timestamp(input: &mut ByteReader<'_>, prev_epoch_ms: Option<i64>) -> Result<(String, Option<i64>), PackError> {
        let tag = input.read_u8()?;
        match tag {
            0 => {
                let len = input.read_varint_u64()? as usize;
                let bytes = input.read_bytes(len)?;
                Ok((utf8(bytes, input.position() as u64)?.to_string(), None))
            }
            1 => {
                let epoch_ms = input.read_varint_u64()? as i64;
                Ok((format_rfc3339_ms(epoch_ms), Some(epoch_ms)))
            }
            2 => {
                let prev = prev_epoch_ms.ok_or_else(|| malformed("timestamp", input.position() as u64, "tag 2 delta with no previous timestamp in scope"))?;
                let delta = input.read_varint_i64()?;
                let epoch_ms = prev
                    .checked_add(delta)
                    .ok_or_else(|| malformed("timestamp", input.position() as u64, "epoch_ms overflow"))?;
                Ok((format_rfc3339_ms(epoch_ms), Some(epoch_ms)))
            }
            other => Err(malformed("timestamp tag", input.position() as u64, &format!("unknown timestamp tag {other:#x}"))),
        }
    }

    fn write_raw_timestamp(out: &mut ByteWriter, raw: &str) {
        out.write_u8(0);
        out.write_varint_u64(raw.len() as u64);
        out.write_bytes(raw.as_bytes());
    }

    fn round_trip_epoch_ms(raw: &str) -> Option<i64> {
        let epoch_ms = parse_rfc3339_ms(raw)?;
        (format_rfc3339_ms(epoch_ms) == raw).then_some(epoch_ms)
    }

    /// @emoji 📆 Parses a UTC RFC-3339 timestamp (`Z` or numeric `±HH:MM` offset, optional
    /// fractional seconds truncated to milliseconds) into milliseconds since the Unix epoch.
    /// `None` on any deviation from the grammar — callers fall back to raw text, never panic.
    fn parse_rfc3339_ms(s: &str) -> Option<i64> {
        let bytes = s.as_bytes();
        if bytes.len() < 20 {
            return None;
        }
        let digit = |i: usize| -> Option<i64> {
            let b = *bytes.get(i)?;
            b.is_ascii_digit().then_some((b - b'0') as i64)
        };
        let two = |i: usize| -> Option<i64> { Some(digit(i)? * 10 + digit(i + 1)?) };
        let year = digit(0)? * 1000 + digit(1)? * 100 + digit(2)? * 10 + digit(3)?;
        if bytes[4] != b'-' {
            return None;
        }
        let month = two(5)?;
        if bytes[7] != b'-' {
            return None;
        }
        let day = two(8)?;
        if bytes[10] != b'T' {
            return None;
        }
        let hour = two(11)?;
        if bytes[13] != b':' {
            return None;
        }
        let minute = two(14)?;
        if bytes[16] != b':' {
            return None;
        }
        let second = two(17)?;
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || minute > 59 || second > 60 {
            return None;
        }
        let mut pos = 19usize;
        let mut ms = 0i64;
        if bytes.get(pos) == Some(&b'.') {
            pos += 1;
            let frac_start = pos;
            while bytes.get(pos).is_some_and(u8::is_ascii_digit) {
                pos += 1;
            }
            if pos == frac_start {
                return None;
            }
            let frac = &s[frac_start..pos];
            let mut ms_digits = [b'0'; 3];
            for (i, slot) in ms_digits.iter_mut().enumerate() {
                if let Some(&b) = frac.as_bytes().get(i) {
                    *slot = b;
                }
            }
            ms = std::str::from_utf8(&ms_digits).ok()?.parse::<i64>().ok()?;
        }
        let offset_minutes = match bytes.get(pos) {
            Some(b'Z') | Some(b'z') => {
                pos += 1;
                0i64
            }
            Some(b'+') | Some(b'-') => {
                let sign = if bytes[pos] == b'+' { 1 } else { -1 };
                pos += 1;
                let oh = two(pos)?;
                pos += 2;
                if bytes.get(pos) != Some(&b':') {
                    return None;
                }
                pos += 1;
                let om = two(pos)?;
                pos += 2;
                sign * (oh * 60 + om)
            }
            _ => return None,
        };
        if pos != bytes.len() {
            return None;
        }
        let days = days_from_civil(year, month, day);
        let total_seconds = days * 86_400 + hour * 3_600 + minute * 60 + second - offset_minutes * 60;
        Some(total_seconds * 1_000 + ms)
    }

    /// @emoji 📆 Canonical UTC formatter: `YYYY-MM-DDTHH:MM:SSZ`, with `.fffZ` appended iff the
    /// millisecond component is nonzero. The single source of truth for the tag-1/2 round trip.
    fn format_rfc3339_ms(epoch_ms: i64) -> String {
        let days = epoch_ms.div_euclid(86_400_000);
        let rem_ms = epoch_ms.rem_euclid(86_400_000);
        let (year, month, day) = civil_from_days(days);
        let hour = rem_ms / 3_600_000;
        let rem = rem_ms % 3_600_000;
        let minute = rem / 60_000;
        let rem = rem % 60_000;
        let second = rem / 1_000;
        let ms = rem % 1_000;
        if ms == 0 {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
        } else {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{ms:03}Z")
        }
    }

    /// @emoji 🌍 Howard Hinnant's `days_from_civil`: proleptic-Gregorian date to days-since-epoch.
    /// <https://howardhinnant.github.io/date_algorithms.html>
    fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
        let y = if m <= 2 { y - 1 } else { y };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let mp = if m > 2 { m - 3 } else { m + 9 };
        let doy = (153 * mp + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    /// @emoji 🌍 Inverse of `days_from_civil`.
    fn civil_from_days(z: i64) -> (i64, i64, i64) {
        let z = z + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        (y, m, d)
    }
    //#endregion 🔖Timestamp

    //#region 🔖Id
    // Id tag: 0 = raw string, 1 = dictref (varint index into a string dictionary — caller supplies
    // the resolve/intern closures), 2 = prefix_dictref + [u8;16] (iff id is "<prefix>-<uuid>"),
    // 3 = edit-ordinal varint (only valid where the referent is a previously-seen edit).
    //
    // 🎯 Design choice: `write_id` tries edit-ordinal first (cheapest — a single small varint,
    // when the id names an edit already in scope), then the prefix+uuid split (16 raw bytes beats
    // a 36-byte string even after dictionary interning), then falls back to a plain dictref. Tag 0
    // (raw) is never emitted by this writer — it exists purely so `read_id` stays forward-
    // compatible with ids written directly by another layer that chooses to skip interning.

    /// @emoji 🪪 Writes `id` using the most compact of the four id tags that preserves it exactly.
    pub fn write_id(out: &mut ByteWriter, id: &str, mut intern: impl FnMut(&str) -> u32, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<(), PackError> {
        if let Some(ordinal) = edit_ordinal_of(id) {
            out.write_u8(3);
            out.write_varint_u64(ordinal);
            return Ok(());
        }
        if let Some((prefix, uuid_bytes)) = split_prefix_uuid(id) {
            out.write_u8(2);
            out.write_varint_u64(intern(prefix) as u64);
            out.write_bytes(&uuid_bytes);
            return Ok(());
        }
        out.write_u8(1);
        out.write_varint_u64(intern(id) as u64);
        Ok(())
    }

    /// @emoji 🪪 Reads one tagged id, resolving dictrefs/ordinals through the supplied closures.
    pub fn read_id<'r>(input: &mut ByteReader<'_>, resolve: impl Fn(u32) -> Result<&'r str, PackError>, ordinal_to_id: impl Fn(u64) -> Result<&'r str, PackError>) -> Result<String, PackError> {
        let tag = input.read_u8()?;
        match tag {
            0 => {
                let len = input.read_varint_u64()? as usize;
                let bytes = input.read_bytes(len)?;
                Ok(utf8(bytes, input.position() as u64)?.to_string())
            }
            1 => {
                let idx = input.read_varint_u64()? as u32;
                Ok(resolve(idx)?.to_string())
            }
            2 => {
                let idx = input.read_varint_u64()? as u32;
                let prefix = resolve(idx)?;
                let uuid_bytes = input.read_bytes(16)?;
                let mut array = [0u8; 16];
                array.copy_from_slice(uuid_bytes);
                Ok(format!("{prefix}-{}", format_uuid(&array)))
            }
            3 => {
                let ordinal = input.read_varint_u64()?;
                Ok(ordinal_to_id(ordinal)?.to_string())
            }
            other => Err(malformed("id tag", input.position() as u64, &format!("unknown id tag {other:#x}"))),
        }
    }

    /// @emoji 🔪 Splits `"<prefix>-<uuid>"` into `(prefix, 16 raw uuid bytes)`, requiring the
    /// trailing 36 bytes to be a canonical lowercase-hex-with-dashes UUID and a non-empty prefix
    /// — so the round trip through `format_uuid` reproduces the original text exactly.
    fn split_prefix_uuid(id: &str) -> Option<(&str, [u8; 16])> {
        let len = id.len();
        if len < 38 {
            return None;
        }
        let uuid_start = len - 36;
        let dash_idx = len - 37;
        if !id.is_char_boundary(dash_idx) || !id.is_char_boundary(uuid_start) {
            return None;
        }
        if id.as_bytes()[dash_idx] != b'-' {
            return None;
        }
        let prefix = &id[..dash_idx];
        if prefix.is_empty() {
            return None;
        }
        let uuid_str = &id[uuid_start..];
        let uuid_bytes = uuid_str.as_bytes();
        if uuid_bytes[8] != b'-' || uuid_bytes[13] != b'-' || uuid_bytes[18] != b'-' || uuid_bytes[23] != b'-' {
            return None;
        }
        let hex_ranges = [0..8, 9..13, 14..18, 19..23, 24..36];
        let mut out = [0u8; 16];
        let mut out_pos = 0usize;
        for range in hex_ranges {
            let group = uuid_str.get(range)?.as_bytes();
            let mut gi = 0usize;
            while gi + 1 < group.len() + 1 && gi < group.len() {
                let hi = lower_hex_val(group[gi])?;
                let lo = lower_hex_val(group[gi + 1])?;
                out[out_pos] = (hi << 4) | lo;
                out_pos += 1;
                gi += 2;
            }
        }
        Some((prefix, out))
    }

    fn lower_hex_val(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            _ => None,
        }
    }

    /// @emoji 🎨 Formats 16 raw bytes as a canonical lowercase `8-4-4-4-12` UUID string.
    fn format_uuid(bytes: &[u8; 16]) -> String {
        let mut s = String::with_capacity(36);
        for (i, b) in bytes.iter().enumerate() {
            if matches!(i, 4 | 6 | 8 | 10) {
                s.push('-');
            }
            s.push_str(&format!("{b:02x}"));
        }
        s
    }
    //#endregion 🔖Id

    //#region 🔖Shared
    fn utf8(bytes: &[u8], offset: u64) -> Result<&str, PackError> {
        std::str::from_utf8(bytes).map_err(|_| malformed("utf8", offset, "invalid utf-8"))
    }

    fn malformed(what: &'static str, offset: u64, detail: &str) -> PackError {
        PackError::Malformed { what, offset, detail: detail.to_string() }
    }
    //#endregion 🔖Shared

    // Id list: count varint, entries* (each via write_id/read_id).
    // Minimal-varint enforcement reuses pack_core::is_minimal_varint at Full verification.
}
//#endregion 🔖Scalars

//#region 🔖Dictionary
/// @emoji 📚 In-memory dictionary builder — deterministic first-use interning order — shared by
/// `protocol_history`'s `REC_ACTOR_DICT`/`REC_STR_DICT` codec and `protocol_format`'s dict-aware
/// frame helpers.
#[derive(Clone, Debug, Default)]
pub struct DictBuilder {
    entries: Vec<String>,
    index: std::collections::HashMap<String, u32>,
}

impl DictBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕ Returns `s`'s existing index, or appends it and returns the new index.
    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&idx) = self.index.get(s) {
            return idx;
        }
        let idx = self.entries.len() as u32;
        self.entries.push(s.to_string());
        self.index.insert(s.to_string(), idx);
        idx
    }

    pub fn len(&self) -> u32 {
        self.entries.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// @emoji ✂️ The entries appended since `base_count` — the delta a `REC_*_DICT` record stores.
    pub fn entries_since(&self, base_count: u32) -> &[String] {
        &self.entries[base_count as usize..]
    }
}

/// @emoji 📖 Read-side twin of `DictBuilder`: replays `REC_*_DICT` deltas in file order.
#[derive(Clone, Debug, Default)]
pub struct DictReader {
    entries: Vec<String>,
}

impl DictReader {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕ Appends a dictionary delta. `base_count` must equal the reader's current length
    /// — a mismatch means the stream's dictionary deltas arrived out of order.
    pub fn extend(&mut self, base_count: u32, new_entries: impl IntoIterator<Item = String>) -> Result<(), ProtocolError> {
        let expected = self.entries.len() as u32;
        if base_count != expected {
            return Err(ProtocolError::DictOutOfOrder { expected, actual: base_count });
        }
        self.entries.extend(new_entries);
        Ok(())
    }

    pub fn resolve(&self, index: u32) -> Result<&str, ProtocolError> {
        self.entries.get(index as usize).map(String::as_str).ok_or(ProtocolError::DictMiss(index))
    }

    pub fn len(&self) -> u32 {
        self.entries.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
//#endregion 🔖Dictionary

//#region 🔖Crypto
// Trait-only — no algorithm ships in protocol_core (repo rule: external libs behind an
// interface). protocol_format provides a Blake3Hasher impl of RecordHasher (it already owns the
// blake3 dep); Signer/SignatureVerifier have zero impls in this family — supplied by the
// integration layer or protocol_cli's optional feature-gated tooling.

/// @emoji 🔗 Content-hashes raw bytes into a 32-byte digest (the commit chain's hash primitive).
pub trait RecordHasher {
    fn hash(&self, bytes: &[u8]) -> [u8; 32];
}

/// @emoji ✍️ Produces a detached signature over a 32-byte message (a commit's `chain_hash`).
pub trait Signer {
    fn scheme(&self) -> &str;
    fn key_id(&self) -> &str;
    fn sign(&self, message: &[u8; 32]) -> Result<Vec<u8>, ProtocolError>;
}

/// @emoji ✅ Verifies a detached signature produced by some `Signer`.
pub trait SignatureVerifier {
    fn verify(&self, scheme: &str, key_id: &str, message: &[u8; 32], signature: &[u8]) -> Result<bool, ProtocolError>;
}
//#endregion 🔖Crypto

//#region 🔖Identifiers
// Moved from framework/core/rs/lib.rs 🔖Identifiers (L5768-5838). Serde-transparent newtypes,
// shapes unchanged from their framework-core originals.

/// @emoji 🆔 A stable identifier for one operation instance (an `Edit`'s forward/backward op).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct OperationId(pub String);

/// @emoji 🧑 A stable identifier for one collaborating actor.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActorId(pub String);

/// @emoji 📄 A stable identifier for one document.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DocumentId(pub String);

/// @emoji 🔢 A monotone document version counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DocumentVersion(pub u64);

/// @emoji 🧬 A stable identifier for one document/operation schema.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SchemaId(pub String);

/// @emoji 🔢 A schema's version number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub u32);

/// @emoji #️⃣ A blake3 content hash over an operation/projection payload.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PayloadHash(pub [u8; 32]);
//#endregion 🔖Identifiers

//#region 🔖HybridLogicalTimestamp
// Moved from framework/core (L5840-5881). FIX vs the original: cmp_key gains `actor` as a total-
// order tiebreak (the original omitted it, so two ticks with equal physical_ms/logical from
// different actors compared Equal — a real ordering bug). Real `Ord`/`PartialOrd` now derive from
// cmp_key, not from field declaration order.

/// @emoji ⏰ A hybrid logical clock tick: physical time plus a logical tiebreak plus the
/// originating actor (the third tiebreak — see the module note on the ordering fix above).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HybridLogicalTimestamp {
    pub actor: u64,
    pub physical_ms: u64,
    pub logical: u64,
}

impl HybridLogicalTimestamp {
    pub fn new(actor: u64, physical_ms: u64) -> Self {
        Self { actor, physical_ms, logical: 0 }
    }

    /// @emoji ⏩ Advances to `physical_ms` if it's newer, else bumps the logical counter.
    pub fn tick(&mut self, physical_ms: u64) {
        if physical_ms > self.physical_ms {
            self.physical_ms = physical_ms;
            self.logical = 0;
        } else {
            self.logical = self.logical.saturating_add(1);
        }
    }

    /// @emoji 🔀 Merges in a remote tick: adopts the greater `(physical_ms, logical)`, then bumps.
    pub fn merge(&mut self, other: &Self) {
        if other.physical_ms > self.physical_ms {
            self.physical_ms = other.physical_ms;
            self.logical = other.logical;
        } else if other.physical_ms == self.physical_ms && other.logical > self.logical {
            self.logical = other.logical;
        }
        self.logical = self.logical.saturating_add(1);
    }

    pub fn cmp_key(&self) -> (u64, u64, u64) {
        (self.physical_ms, self.logical, self.actor)
    }
}

impl Ord for HybridLogicalTimestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_key().cmp(&other.cmp_key())
    }
}

impl PartialOrd for HybridLogicalTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
//#endregion 🔖HybridLogicalTimestamp

//#region 🔖Policies
// UndoPolicy moved from vcs/rs (unchanged variants); MergeStrategyKind moved from framework/core
// L6636-6668 (unchanged variants). New: ConflictRule, the per-operation conflict declaration
// surface today's code lacks (store::merge_concurrent_diffs collapses everything to absorb
// regardless of declared strategy — protocol_crdt fixes this using ConflictRule).
//
// Note: the contract's prose also mentions `DocumentKind` moving alongside `MergeStrategyKind`,
// but the frozen signature block below only defines `UndoPolicy`/`MergeStrategyKind`/
// `ConflictRule` — `DocumentKind` is not redefined here (it stays in `framework/core` until a
// later wave's contract actually specifies its new home).

/// @emoji ↩️ How an undo of this operation kind should be computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UndoPolicy {
    ExactBaseOnly,
    TransformAgainstConcurrent,
    SemanticUndo,
    CompensatingAction,
}

/// @emoji 🧩 Which CRDT-style merge algorithm reconciles concurrent diffs of this kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MergeStrategyKind {
    LwwRegister,
    OrderedSequence,
    TextSequence,
    TombstonedGraphSet,
    ContentAddressedBlob,
}

/// @emoji ⚖️ The per-operation conflict declaration surface: how two concurrent instances of the
/// same operation kind resolve. `Commutes`/`Transform` need no merge strategy; `Merge`/`Crdt`
/// carry the `MergeStrategyKind` that arbitrates them (see `protocol_crdt`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConflictRule {
    Commutes,
    Transform,
    Merge(MergeStrategyKind),
    Crdt(MergeStrategyKind),
}
//#endregion 🔖Policies

//#region 🔖StateClass
// New: the explicit persistent/shared-ui/local-ui/preview/effect separation the db spec requires.
// Carried on OperationDescriptor (protocol_command) and on wire envelopes (protocol_wire).

/// @emoji 🗂️ Which durability/visibility class an operation's diffs belong to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StateClass {
    Persistent,
    SharedUi,
    LocalUi,
    Preview,
    Effect,
}
//#endregion 🔖StateClass

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖Errors
    #[test]
    fn pack_error_converts_into_protocol_error_via_from() {
        let pack_err = pack_core::PackError::Truncated(7);
        let protocol_err: ProtocolError = pack_err.clone().into();
        assert_eq!(protocol_err, ProtocolError::Pack(pack_err));
    }
    //#endregion 🔖Errors

    //#region 🔖Limits
    #[test]
    fn protocol_limits_default_matches_contract() {
        let limits = ProtocolLimits::default();
        assert_eq!(limits.max_file_len, 64 * 1024 * 1024 * 1024);
        assert_eq!(limits.max_frame_len, 2 * 1024 * 1024 * 1024);
        assert_eq!(limits.max_record_count, 256_000_000);
        assert_eq!(limits.max_dict_entries, 1_000_000);
        assert_eq!(limits.max_op_count_per_edit, 100_000);
        assert_eq!(limits.max_total_alloc, 4 * 1024 * 1024 * 1024);
    }
    //#endregion 🔖Limits

    //#region 🔖RecordKinds
    #[test]
    fn record_kind_constants_match_contract() {
        assert_eq!(REC_END, 0x00);
        assert_eq!(REC_DOC, 0x01);
        assert_eq!(REC_ACTOR_DICT, 0x02);
        assert_eq!(REC_STR_DICT, 0x03);
        assert_eq!(REC_EDIT, 0x04);
        assert_eq!(REC_CHANGE, 0x05);
        assert_eq!(REC_CHECKPOINT, 0x06);
        assert_eq!(REC_ALTERNATIVE, 0x07);
        assert_eq!(REC_ACTIVE, 0x08);
        assert_eq!(REC_FRONTIER, 0x09);
        assert_eq!(REC_PROJECTION, 0x0A);
        assert_eq!(REC_INDEX, 0x0B);
        assert_eq!(REC_COMMIT, 0x0C);
        assert_eq!(REC_SIGNATURE, 0x0D);
        assert_eq!(REC_REDACTION, 0x0E);
        assert_eq!(REC_UPCAST, 0x0F);
        assert_eq!(REC_EPHEMERAL, 0x10);
        assert_eq!(REC_SEALED, 0x11);
        assert_eq!(REC_COMPACTION, 0x12);
        assert_eq!(REC_PADDING, 0x7F);
    }

    #[test]
    fn is_critical_kind_matches_contract_set() {
        for kind in [REC_DOC, REC_EDIT, REC_CHANGE, REC_CHECKPOINT, REC_ALTERNATIVE, REC_ACTIVE, REC_COMMIT, REC_ACTOR_DICT, REC_STR_DICT] {
            assert!(is_critical_kind(kind), "{kind:#x} should be critical");
        }
        for kind in [REC_END, REC_FRONTIER, REC_PROJECTION, REC_INDEX, REC_SIGNATURE, REC_REDACTION, REC_UPCAST, REC_EPHEMERAL, REC_SEALED, REC_COMPACTION, REC_PADDING, 0x50] {
            assert!(!is_critical_kind(kind), "{kind:#x} should not be critical");
        }
    }
    //#endregion 🔖RecordKinds

    //#region 🔖Flags
    #[test]
    fn frame_flags_round_trips_codec_id() {
        for codec in 0u8..=7 {
            for compressed in [false, true] {
                for critical in [false, true] {
                    let flags = frame_flags(compressed, critical, codec);
                    assert_eq!(flags & FRAME_FLAG_COMPRESSED != 0, compressed);
                    assert_eq!(flags & FRAME_FLAG_CRITICAL != 0, critical);
                    assert_eq!(frame_codec_id(flags), codec);
                }
            }
        }
    }

    #[test]
    fn frame_codec_id_masks_to_three_bits() {
        assert_eq!(frame_codec_id(0b1111_1100), 0b111);
    }
    //#endregion 🔖Flags

    //#region 🔖Scalars
    mod scalars {
        use super::super::scalar::{read_id, read_timestamp, write_id, write_timestamp};
        use pack_core::{ByteReader, ByteWriter};

        #[test]
        fn timestamp_round_trips_canonical_utc_no_fraction() {
            let raw = "2024-01-15T10:30:00Z";
            let mut out = ByteWriter::new();
            let epoch = write_timestamp(&mut out, raw, None);
            assert!(epoch.is_some());
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (decoded, epoch_back) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
            assert_eq!(epoch_back, epoch);
        }

        #[test]
        fn timestamp_round_trips_canonical_utc_with_fraction() {
            let raw = "2024-01-15T10:30:00.123Z";
            let mut out = ByteWriter::new();
            write_timestamp(&mut out, raw, None);
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
        }

        #[test]
        fn timestamp_falls_back_to_raw_for_non_canonical_text() {
            let raw = "not-a-timestamp";
            let mut out = ByteWriter::new();
            let epoch = write_timestamp(&mut out, raw, None);
            assert_eq!(epoch, None);
            let bytes = out.into_bytes();
            assert_eq!(bytes[0], 0, "tag byte must be 0 (raw)");
            let mut reader = ByteReader::new(&bytes);
            let (decoded, epoch_back) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
            assert_eq!(epoch_back, None);
        }

        #[test]
        fn timestamp_falls_back_to_raw_for_non_utc_offset() {
            let raw = "2024-01-15T10:30:00+02:00";
            let mut out = ByteWriter::new();
            let epoch = write_timestamp(&mut out, raw, None);
            assert_eq!(epoch, None);
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
        }

        #[test]
        fn timestamp_chain_uses_delta_tag_after_first_absolute() {
            let mut out = ByteWriter::new();
            let e1 = write_timestamp(&mut out, "2024-01-15T10:30:00Z", None).unwrap();
            let e2 = write_timestamp(&mut out, "2024-01-15T10:30:05Z", Some(e1)).unwrap();
            assert_eq!(e2 - e1, 5_000);
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (d1, p1) = read_timestamp(&mut reader, None).unwrap();
            let (d2, p2) = read_timestamp(&mut reader, p1).unwrap();
            assert_eq!(d1, "2024-01-15T10:30:00Z");
            assert_eq!(d2, "2024-01-15T10:30:05Z");
            assert_eq!(p2, Some(e2));
        }

        #[test]
        fn timestamp_epoch_zero_round_trips() {
            let raw = "1970-01-01T00:00:00Z";
            let mut out = ByteWriter::new();
            let epoch = write_timestamp(&mut out, raw, None);
            assert_eq!(epoch, Some(0));
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
            assert_eq!(decoded, raw);
        }

        #[test]
        fn id_round_trips_via_edit_ordinal_tag() {
            let mut out = ByteWriter::new();
            write_id(&mut out, "edit-7", |_| unreachable!("must not intern"), |id| (id == "edit-7").then_some(7)).unwrap();
            let bytes = out.into_bytes();
            assert_eq!(bytes[0], 3, "tag byte must be 3 (edit-ordinal)");
            let mut reader = ByteReader::new(&bytes);
            let decoded = read_id(&mut reader, |_| unreachable!("must not resolve"), |ordinal| if ordinal == 7 { Ok("edit-7") } else { Err(pack_core::PackError::Truncated(0)) }).unwrap();
            assert_eq!(decoded, "edit-7");
        }

        #[test]
        fn id_round_trips_via_prefix_uuid_tag() {
            let id = "actor-3fa85f64-5717-4562-b3fc-2c963f66afa6";
            let mut out = ByteWriter::new();
            write_id(&mut out, id, |s| { assert_eq!(s, "actor"); 0 }, |_| None).unwrap();
            let bytes = out.into_bytes();
            assert_eq!(bytes[0], 2, "tag byte must be 2 (prefix+uuid)");
            let mut reader = ByteReader::new(&bytes);
            let decoded = read_id(&mut reader, |idx| if idx == 0 { Ok("actor") } else { Err(pack_core::PackError::Truncated(0)) }, |_| unreachable!("must not resolve ordinal")).unwrap();
            assert_eq!(decoded, id);
        }

        #[test]
        fn id_falls_back_to_dictref_tag_for_plain_strings() {
            let mut out = ByteWriter::new();
            write_id(&mut out, "hello-world", |s| { assert_eq!(s, "hello-world"); 42 }, |_| None).unwrap();
            let bytes = out.into_bytes();
            assert_eq!(bytes[0], 1, "tag byte must be 1 (dictref)");
            let mut reader = ByteReader::new(&bytes);
            let decoded = read_id(&mut reader, |idx| if idx == 42 { Ok("hello-world") } else { Err(pack_core::PackError::Truncated(0)) }, |_| unreachable!("must not resolve ordinal")).unwrap();
            assert_eq!(decoded, "hello-world");
        }

        #[test]
        fn id_raw_tag_is_readable_even_though_writer_never_emits_it() {
            let mut out = ByteWriter::new();
            out.write_u8(0);
            out.write_varint_u64(5);
            out.write_bytes(b"hello");
            let bytes = out.into_bytes();
            let mut reader = ByteReader::new(&bytes);
            let decoded = read_id(&mut reader, |_| unreachable!(), |_| unreachable!()).unwrap();
            assert_eq!(decoded, "hello");
        }

        #[test]
        fn id_dictref_dedupes_repeated_ids_through_intern_closure() {
            let mut dict: Vec<String> = Vec::new();
            let mut out = ByteWriter::new();
            {
                let mut intern = |s: &str| -> u32 {
                    if let Some(pos) = dict.iter().position(|e| e == s) {
                        pos as u32
                    } else {
                        dict.push(s.to_string());
                        (dict.len() - 1) as u32
                    }
                };
                write_id(&mut out, "same-id", &mut intern, |_| None).unwrap();
                write_id(&mut out, "same-id", &mut intern, |_| None).unwrap();
            }
            assert_eq!(dict.len(), 1, "second write must reuse the same dictionary slot");
        }
    }
    //#endregion 🔖Scalars

    //#region 🔖Dictionary
    #[test]
    fn dict_builder_interns_deterministically_and_dedupes() {
        let mut builder = DictBuilder::new();
        assert!(builder.is_empty());
        assert_eq!(builder.intern("a"), 0);
        assert_eq!(builder.intern("b"), 1);
        assert_eq!(builder.intern("a"), 0);
        assert_eq!(builder.len(), 2);
        assert_eq!(builder.entries_since(0), &["a".to_string(), "b".to_string()]);
        assert_eq!(builder.entries_since(1), &["b".to_string()]);
    }

    #[test]
    fn dict_reader_round_trips_builder_deltas_in_order() {
        let mut builder = DictBuilder::new();
        builder.intern("x");
        builder.intern("y");
        let mut reader = DictReader::new();
        reader.extend(0, builder.entries_since(0).to_vec()).unwrap();
        assert_eq!(reader.resolve(0).unwrap(), "x");
        assert_eq!(reader.resolve(1).unwrap(), "y");
        assert_eq!(reader.len(), 2);

        builder.intern("z");
        reader.extend(2, builder.entries_since(2).to_vec()).unwrap();
        assert_eq!(reader.resolve(2).unwrap(), "z");
    }

    #[test]
    fn dict_reader_rejects_out_of_order_deltas() {
        let mut reader = DictReader::new();
        let err = reader.extend(5, vec!["late".to_string()]).unwrap_err();
        assert_eq!(err, ProtocolError::DictOutOfOrder { expected: 0, actual: 5 });
    }

    #[test]
    fn dict_reader_reports_miss_past_the_end() {
        let reader = DictReader::new();
        assert_eq!(reader.resolve(0).unwrap_err(), ProtocolError::DictMiss(0));
    }
    //#endregion 🔖Dictionary

    //#region 🔖Crypto
    struct FixedHasher;
    impl RecordHasher for FixedHasher {
        fn hash(&self, bytes: &[u8]) -> [u8; 32] {
            let mut out = [0u8; 32];
            out[0] = bytes.len() as u8;
            out
        }
    }

    #[test]
    fn record_hasher_trait_is_object_usable() {
        let hasher = FixedHasher;
        assert_eq!(hasher.hash(b"abc")[0], 3);
    }
    //#endregion 🔖Crypto

    //#region 🔖HybridLogicalTimestamp
    #[test]
    fn hlc_tick_advances_on_newer_physical_time() {
        let mut hlc = HybridLogicalTimestamp::new(1, 100);
        hlc.tick(200);
        assert_eq!(hlc.physical_ms, 200);
        assert_eq!(hlc.logical, 0);
    }

    #[test]
    fn hlc_tick_bumps_logical_on_equal_or_older_physical_time() {
        let mut hlc = HybridLogicalTimestamp::new(1, 100);
        hlc.tick(100);
        assert_eq!(hlc.logical, 1);
        hlc.tick(50);
        assert_eq!(hlc.logical, 2);
    }

    #[test]
    fn hlc_merge_adopts_the_greater_remote_tick_then_bumps() {
        let mut local = HybridLogicalTimestamp::new(1, 100);
        let remote = HybridLogicalTimestamp { actor: 2, physical_ms: 150, logical: 3 };
        local.merge(&remote);
        assert_eq!(local.physical_ms, 150);
        assert_eq!(local.logical, 4);
    }

    #[test]
    fn hlc_ordering_uses_actor_as_final_tiebreak() {
        let a = HybridLogicalTimestamp { actor: 1, physical_ms: 100, logical: 5 };
        let b = HybridLogicalTimestamp { actor: 2, physical_ms: 100, logical: 5 };
        assert!(a < b, "equal physical_ms/logical must tiebreak by actor, not compare Equal");
        assert_ne!(a.cmp_key(), b.cmp_key());
    }

    #[test]
    fn hlc_ordering_prioritizes_physical_then_logical_then_actor() {
        let older = HybridLogicalTimestamp { actor: 9, physical_ms: 100, logical: 0 };
        let newer_physical = HybridLogicalTimestamp { actor: 0, physical_ms: 101, logical: 0 };
        let newer_logical = HybridLogicalTimestamp { actor: 0, physical_ms: 100, logical: 1 };
        assert!(older < newer_physical);
        assert!(older < newer_logical);
        assert!(newer_logical < newer_physical);
    }
    //#endregion 🔖HybridLogicalTimestamp

    //#region 🔖Identifiers
    #[test]
    fn identifier_newtypes_serde_round_trip_transparently() {
        let op = OperationId("op-1".to_string());
        let json = serde_json::to_string(&op).unwrap();
        assert_eq!(json, "\"op-1\"");
        assert_eq!(serde_json::from_str::<OperationId>(&json).unwrap(), op);

        let version = DocumentVersion(42);
        let json = serde_json::to_string(&version).unwrap();
        assert_eq!(json, "42");
        assert_eq!(serde_json::from_str::<DocumentVersion>(&json).unwrap(), version);
    }

    #[test]
    fn document_version_orders_numerically() {
        assert!(DocumentVersion(1) < DocumentVersion(2));
    }
    //#endregion 🔖Identifiers

    //#region 🔖Policies
    #[test]
    fn conflict_rule_carries_merge_strategy_variants() {
        let commutes = ConflictRule::Commutes;
        let merge = ConflictRule::Merge(MergeStrategyKind::LwwRegister);
        let crdt = ConflictRule::Crdt(MergeStrategyKind::TextSequence);
        assert_ne!(commutes, merge);
        assert_ne!(merge, crdt);
        assert_eq!(ConflictRule::Merge(MergeStrategyKind::LwwRegister), merge);
    }

    #[test]
    fn undo_policy_and_state_class_serde_round_trip() {
        for policy in [UndoPolicy::ExactBaseOnly, UndoPolicy::TransformAgainstConcurrent, UndoPolicy::SemanticUndo, UndoPolicy::CompensatingAction] {
            let json = serde_json::to_string(&policy).unwrap();
            assert_eq!(serde_json::from_str::<UndoPolicy>(&json).unwrap(), policy);
        }
        for class in [StateClass::Persistent, StateClass::SharedUi, StateClass::LocalUi, StateClass::Preview, StateClass::Effect] {
            let json = serde_json::to_string(&class).unwrap();
            assert_eq!(serde_json::from_str::<StateClass>(&json).unwrap(), class);
        }
    }
    //#endregion 🔖Policies
}
//#endregion 🧪Tests
