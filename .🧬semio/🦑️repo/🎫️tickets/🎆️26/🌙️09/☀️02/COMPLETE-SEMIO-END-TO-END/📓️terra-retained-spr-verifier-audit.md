# Retained SPR Fixed-Buffer Verifier Audit

## Scope and verdict

Audited current staged, deliberately unmounted sources only:

- `🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🦀️.rs`
- `…/🧫️fixture/🔣️.json` and `…/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:39-169`

The core fixed-buffer scanner is a promising **source-only** framing verifier: it checks header CRC/reserved bytes, frame length/CRC/back-length, canonical length varints, chained commit sequence/previous offset/covered byte and record count, and commit BLAKE3. It returns only a scalar `VerifiedSprSpan`; it publishes no record, dictionary, document, or input owner (`verification/🦀️.rs:1-3,33-51,137-239`). A torn or uncommitted suffix leaves the last full verified commit available, while a fully framed CRC-valid but semantically invalid commit is sticky rejection. Those are correct source properties.

It is **not yet runtime-qualified**. The module is unmounted, no production caller retains an input/root through verification, and the registered native selector cannot currently run. The reported Node/AJV source oracle is coordinator evidence, not an independently executed native result.

## Current-byte reread — compressed/profile correction

The former compressed-grammar and header-profile REDs are **source-closed** in the current staged bytes; the historical sections below are retained for their initial-red rationale. Coordinator-observed source gate `43033` exited 0 with 2 commits, 224 prefix recoveries, 26 general denials, and 10 compressed grammar cases. That is source/oracle evidence only, not a native result.

- `verification/🦀️.rs:1-7,155-166` now defines a strict retained profile: v1.0, `REQUIRED_HASH_CHAIN` only, no signed/encrypted requirement, canonical LEB128, frame flags confined to bits `0..=4`, and compression iff the three-bit codec id is nonzero. `REC_COMMIT` still requires precisely `FRAME_FLAG_CRITICAL`.
- `verification/🦀️.rs:281-312,349` incorporates all compressed vectors into the selected hostile native law at grants 1/7/4096. It validates production retained-writer framing for the positive Deflate row, while the scanner deliberately neither inflates nor grants raw allocation credit.
- The neutral fixture/schema bind the selected profile and ten compressed vectors: committed Deflate, missing/nonminimal/overflow/unterminated raw length, ten continuation bytes, compressed commit, reserved high flags, compressed identity codec, and codec without compression (`🧫️fixture/🔣️.json:1-60`, `🧬️schema/🔣️.json:3-37`).
- The independent Node/AJV oracle validates that schema, parses the same canonical raw-length grammar, uses `@webassemblyjs/leb128`, and independently inflates the accepted Deflate vector with `node:zlib` before reconstructing its committed full-frame BLAKE3 chain (`📜️script.ts:80-160`). The script exact-selects two native laws and requires a `pub mod retained;` mount. The parent is still unmounted, so these assertions are present but have not been accepted as native evidence.

No raw-output ceiling is implied by this structural verifier: a legal large `raw_len` is not allocated or exposed here. The first semantic decompressor must impose an output limit before allocation. Likewise, a sequence-zero header prefix is only a structural prefix; a future document consumer must require the committed semantic records it needs.

## Current source evidence

| Boundary | Current source result | Evidence |
|---|---|---|
| Header / fixed memory | PASS, source-only | `RetainedSprVerification` retains only header `[u8;32]`, trailer `[u8;8]`, commit `[u8;64]` plus hash/CRC state; `new` rejects file/frame/record capacity violations before traversal (`verification/🦀️.rs:57-101`). |
| Fuel / cancellation | PASS, source-only | `push` consumes at most supplied fuel and counts every byte; `cancel` stores a sticky closed diagnostic and further push/finish do not consume more (`:104-135`). The Rust law checks zero fuel and cancel at every fixture boundary (`:290-339`). |
| Torn versus invalid | PASS, source-only | Incomplete length/body/trailer enters `Torn` and `finish` returns the prior commit prefix; any complete frame with bad CRC/back length or a complete malformed commit returns sticky `Frame`/`Commit` (`:143-174,202-239`). |
| Commit coverage | PASS, source-only | Non-commit full-frame digests feed `pending_hash`; `REC_COMMIT` validates exact 66-byte body/75-byte frame, sequence, previous offset, byte count, record count, reserved bytes and BLAKE3 (`:202-239`). This mirrors the writer's full-frame digest / commit payload construction (`📐️format/🦀️.rs:487-527`). |
| No semantic publication | PASS, source-only | The verifier exposes only offsets/counts/chain. It does not decode record kinds or invoke history/document construction. This is intentionally structural, as base format is kind-agnostic (`📐️format/🦀️.rs:9-15`). |
| One-use / retained ownership | RED | `VerifiedSprSpan` is an ordinary copy-free value with no attached input authority; no caller, input retainer, cancellation generation, or close/abort owner exists because this module is unmounted. A future consumer must retain the original bytes and accept exactly one span from its own operation; this verifier alone cannot prove that lifecycle. |

## Material blockers and repairs

### RED 1 — native registered gate currently names an unmounted module

`RetainedVerificationScript` intentionally refuses a native run unless parent `📐️format/🦀️.rs` contains `pub mod retained;`, then selects `format::retained::tests::{…}` (`📜️script.ts:137-143`). Current parent has no child module declaration (`📐️format/🦀️.rs:1-19`), while the staged file physically lives at `🔎️verification/🦀️.rs`.

This is honest while staged, but no acceptance may cite `retained-verification-check` as a native gate yet. Mount it with the exact script-facing alias (for example a path-mounted `pub mod retained`), or change both selector and declaration together; do not create a second copy. Then run the exact two laws selected at `📜️script.ts:138-143` in an isolated target. The Node/AJV run cannot substitute for this compilation/execution.

### Historical RED 2 — compressed raw-length grammar initially lacked neutral or independent parity coverage (source-closed)

At the initial audit, the Rust machine had a compressed branch but all neutral records were fixed to `flags: 2`, and the independent `inspect` implementation did not parse compressed raw-length grammar. It therefore could not validate canonical LEB128, exact body-boundary, or commit-chain behavior for compressed frames.

The retained verifier's `varint_byte` rejects a nonminimal terminal zero for both frame and compressed raw lengths, while the shared `codec::read_varint_u64` used by ordinary `decode_frame_in_slice` accepts a nonminimal but in-range LEB128 (`⚙️codec/🦀️.rs:104-127`; `📐️format/🦀️.rs:212-236`). Current documentation and fixture now correctly define this as a strict retained profile, not parser parity or a generic-reader replacement.

Required minimal repair is schema/corpus, then both implementations:

1. Add one full-frame compressed non-commit vector with a canonical raw length and a nonzero codec-id flag; include it in a committed range so its full-frame digest participates in the next commit chain.
2. Add hostile rows for missing raw length in an otherwise CRC-valid complete body, nonminimal raw length, ten-byte overflow/unterminated raw length, and a compressed `REC_COMMIT` (must be `Commit`, not a torn prefix).
3. Teach the Node oracle to parse exactly the same raw-length grammar before it hashes/accepts the frame. It need not decompress: decompression and the raw allocation cap belong to the semantic codec consumer (`⚙️codec/🦀️.rs:346-352,399-408`), but the structural grammar must be identical here.

The initial implementation also accepted high reserved bits and compressed/codec-zero on non-commit frames. Current `byte` explicitly denies both, as well as a codec id without compression, and corpus rows cover each (`verification/🦀️.rs:160-166`; `fixture/🔣️.json:58-60`).

Current source implements that exact strict-profile repair: all ten rows now exercise the scanner and the independent oracle; the accepted Deflate row is decompressed only by the Node oracle to prove the fixture's stored bytes and raw-length grammar. `RetainedSprLimits` still deliberately limits only stored file/body/record counts (`verification/🦀️.rs:9-19`). That is allocation-safe for the verifier itself, but it is not a semantic decompression limit. Keep that nonclaim explicit until the retained semantic consumer applies its own raw-output budget before allocation.

### Historical RED 3 — retained header admission initially lacked an explicit profile contract (source-closed)

The staged verifier requires minor version `0` and `required_flags == REQUIRED_HASH_CHAIN` exactly (`verification/🦀️.rs:181-192`). Base format accepts any v1 minor and any *known* required flag combination (`📐️format/🦀️.rs:50-79`), including signed/encrypted flags which this structural verifier cannot satisfy.

Failing closed is safe, and current source now states **v1.0, hash-chain-only, unencrypted/unsigned retained framing**. The schema binds that profile and the fixture denies minor-version drift, unknown required flags, and signed/encrypted requirements (`verification/🦀️.rs:181-192`; `fixture/🔣️.json:21-29`). Optional header flags remain structurally ignorable as in the generic formatter; that is appropriate until a semantic record consumer needs one.

### RED 4 — no language-neutral cancellation / no-publication lifecycle oracle

The native source law probes cancellation at every byte boundary and asserts no later input consumption (`verification/🦀️.rs:330-339`), but the JSON schema has no cancellation, span-consumption, retained-input, or abort/close trace and the Node oracle has no cancellation state (`fixture schema`, `📜️script.ts:79-136`). Because input ownership correctly sits outside the verifier, this should be a separate neutral caller-operation fixture once a caller exists. It must prove cancel before first byte, during header, within length/body/trailer, after a verified commit, and after `finish`: no typed decode/publication, one retained input owner, bounded close, and no second span handoff.

## Acceptance boundary

Do not integrate the verifier into retained Flow or history hydration yet. Its safe present use is only an unmounted source packet. The framing/profile packet is source-closed, but acceptance still requires: (1) a single path-mounted native module matching the registered exact FQNs, (2) native execution of both selected laws, and (3) a separate caller-owned retained operation proving one-use span adoption, cancellation, close and zero semantic publication before verified EOF.

No Cargo/Nx command was run for this audit.
