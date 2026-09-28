# Fidelity Checkpoint and Object-Order Audit

Date: 2026-09-27  
Scope: read-only checkpoint of the WAV, ZIP retained-work, and shared compact-patch changes. This audit did not run Cargo or native tests.

## Actionable Findings

### P1: WAV mutations accept pad bytes that the encoder cannot serialize

`WavSnapshot` exposes `fmtPadByte`, `dataPadByte`, and each retained `RiffChunk.padByte` as unconstrained `u8` values in `…/wav/…/schema/snapshot/🦀️.rs` (lines 57–109). `SetSnapshot`, `SetFmt`, `SetData`, and `SetOtherChunks` reduce directly to the corresponding diff without a RIFF representability check in `…/schema/mutations/🦀️.rs` (lines 99–106).

`append_chunk` in `…/wav/…/🚪️io/🦀️.rs` writes its supplied pad byte only when the payload length is odd (lines 219–225). Therefore a typed mutation can set a nonzero pad byte for an even-length primary or retained chunk, save successfully, then reopen with that field decoded as zero. The existing TypeScript mutation fixture already constructs nonzero `fmtPadByte` and `dataPadByte` values for 16-byte `fmt ` and 8-byte `data` payloads; it only exercises schema parsing, not save/reopen.

Reject those states before emitting the mutation/diff, or make padding absent unless its paired serialized payload is odd. Add a native edit → encode → decode assertion for even primary chunks and even retained chunks.

### P1: WAV extensible-format length can overflow its on-wire `cbSize`

`encode_fmt_chunk` casts `fmt.ext.len()` to `u16` for `cbSize` and then appends the complete unbounded vector (lines 118–129 of `…/wav/…/🚪️io/🦀️.rs`). Neither the snapshot schema nor typed `SetFmt`/`SetSnapshot` bounds `ext` to 65,535 bytes.

For an `ext` length of 65,536, the stream declares `cbSize = 0` while carrying all 65,536 bytes. The decoder consumes the declared zero extension and the saved state cannot reopen coherently. Enforce the `u16` bound prior to mutation persistence and add an exact-boundary/one-over-boundary save-reopen law.

### P1: WAV FourCC accepts Unicode character length but emits four bytes

`RiffChunk.fourcc` is a `String` (snapshot lines 57–63). When encoding retained chunks, the codec converts it to UTF-8 bytes, pads only short values, then takes `fourcc[0..4]` (io lines 265–278). A four-character non-ASCII value passes a character-count schema constraint yet has more than four UTF-8 bytes; it is truncated, potentially in the middle of a code point, and decodes differently.

Require exactly four permitted wire bytes (normally ASCII) at the snapshot/mutation boundary, or represent FourCC as a fixed four-byte value. Add a rejected-unicode-FourCC mutation test.

### P1: One `InsertAt` patch has incompatible ordered-object behavior across TypeScript and Rust

TypeScript applies positioned object insertion through `Object.entries`, `splice`, and `Object.fromEntries`; it rejects the patch if `Object.keys(next)[index]` is not the inserted key (lines 79–85 of `…/editing/🩹️patch/🟦️.ts`). ECMAScript enumerates canonical integer-index property keys in numeric order.

The recorded TypeScript reproduction inserts `"1"` at position 1 into `{ "2": "two", "10": "ten", "z": "zed" }`. It rejects with `the object cannot preserve the requested key position at /metadata/1`, because the result enumerates `"1"` before `"2"`. See `../🗑️generated/fidelity-checkpoint-patch-numeric-keys.log`.

The Rust patch accepts the same object parent and delegates `InsertAt` to the target representation (lines 125–150 of `…/editing/🩹️patch/🦀️.rs`). `DslValue::Object` has an explicit entry sequence, so it can retain `["2", "1", "10", "z"]`. This produces a wire-level behavioral divergence for a valid portable patch.

The proposed contract correction is sound: exact positioned order must be asserted only for explicit ordered representations such as `DslValue::Object` and authored JSON members. Canonical JavaScript integer-key order and intrinsic map order must retain their container semantics. The TypeScript postcondition must not reject a canonicalized result solely because the requested sequence is unattainable. Add forward-and-inverse laws for integer keys and a two-edit rename, comparing canonical key order and exact values in each runtime.

### P1: Rust `HashMap` currently treats `InsertAt.index` nondeterministically

`HashMap` documents that its iteration order is unspecified in `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs` (lines 1064–1070). Its edit implementation handles `ValueEdit::Insert` and `ValueEdit::InsertAt` together, discarding the insertion index (lines 1149–1154), while `value_key_at_path` derives an index from `self.keys().nth(index)` (lines 1091–1105).

The generic Rust patch postcondition then accepts or rejects a positioned insertion depending on that unspecified order (editing patch lines 146–150). Its inverse also derives an `InsertAt` index from that order (lines 347–376). This is a concrete nondeterministic outcome for typed `HashMap` snapshots.

Remove the generic key-at-index postcondition for intrinsic-order/unordered containers as proposed. For `HashMap`, `InsertAt` must not promise a sequence position; forward/inverse tests should prove key/value restoration under the map's native semantics. Preserve exact-order assertions for `DslValue::Object` only.

## ZIP Retained Checkpoint Assessment

No actionable defect was found in the inspected ZIP resume path. The actual `BoundedNativeEditToolJobFactory` gates restoration through `NATIVE_CHECKPOINT_RESUME`; the retained-command shell binds checkpoints to its workspace identity and context digest. The artifact-owned context digest includes the canonical base revision. ZIP's restored worker independently refuses an absent context before binding its cursor, and the native test targets the registered factory with a changed-revision rejection.

This conclusion is static plus TypeScript coverage, not native proof. The native lane was intentionally not run for this audit.

## Executed Checks

- `bun nx test @semio-tech/stdio-zip --skip-nx-cache`: 10 passing TypeScript tests. Log: `../🗑️generated/fidelity-checkpoint-zip-typescript.log`.
- `bun nx test @semio-tech/stdio-snapshot-editing-js --skip-nx-cache`: 81 passing TypeScript tests. Log: `../🗑️generated/fidelity-checkpoint-patch-typescript.log`.
- Focused `bun` numeric-key reproduction: expected TypeScript rejection recorded in `../🗑️generated/fidelity-checkpoint-patch-numeric-keys.log`.

The existing reports reviewed were `📓️wav-chunk-fidelity-and-png-dci4k-2026-09-27.md`, `🔍️research/🧬zip-checkpoint-contract-2026-09-27.md`, and `🔍️research/🧬compact-object-order-2026-09-27.md`. Passing TypeScript checks do not exercise the WAV encoder/decoder or native checkpoint factory, so they do not verify those native claims.
