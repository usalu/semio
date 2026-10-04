# Small Stdio Current API Readback Audit

Read-only source inspection on 2026-10-03. No Cargo/build/test execution, production edits, Git mutations or worktrees. Counts are authored candidates, never passing-law claims.

## Current Candidate Counts and Mounts

| Owner | Ordinary test attributes | Async test attributes | Total SQLite native candidates |
| --- | ---: | ---: | ---: |
| Binary raw/* | 6 | 2 | 8 |
| CSV rfc4180/* | 7 | 2 | 9 |
| TSV iana/* | 7 | 2 | 9 |
| BMP v3/* | 6 | 3 | 9 |
| Deflate rfc1950/* | 6 | 2 | 8 |
| Total | 32 | 11 | 43 |

Async attributes are `#[semio_framework_async_macros::async_test]`, not Tokio attributes. Each snapshot root actually mounts its SQLite implementation and SQLite tests. All five individual `📸️snapshot/🚦️native/🦀️.rs` drafts exist; none is mounted or bound to native hooks. Reports correctly describe staged, unexecuted Native work and existing strict-default hooks. Source receipts in the staging reports are historical receipts, not a new verification performed by this audit.

## Compile Prerequisites

BMP snapshot root `🦀️.rs:131` still invokes `TextError::new(e, TextSpan::at(1, 1))` with two arguments. This is a concrete current production compile prerequisite after the three-argument diagnostic API change; use the actual error category with message and span. The adjacent hexadecimal constructor at line128 already supplies three arguments.

All five draft imports of `semio_framework_value::native_decoding::NativeDecodeProgress` and `native_encoding::NativeEncodeProgress` resolve through public modules in `🧰️framework/🔨️modules/🌱️value/🦀️.rs:25–30`; they are not private-import blockers. Each draft explicitly maps controlled ValueError through `ValueError::into_message` at its existing String-return owner boundary. No blanket String compatibility conversion is needed. `semio_framework_os_kernel::io` is a public alias, its ArtifactDialect/Dialect/StandardId/SubsetId exports are public, and its io_mechanism is public.

Deflate draft imports `semio_framework_deflate` directly. The actual owning `🗜️deflate/📦️packages/🦀️rust/Cargo.toml` does not list that dependency. Author the first-party edge when mounting after authentic baseline; current unmounted presence does not typecheck the draft.

## Literal File Contract and External Wire

The full typed SQLite laws use arbitrary owned fixture state, exact database equality and both SnapshotEncoding metadata modes, rejecting native encode/decode phases. Canonical actual-factory external laws and explicit external normalization laws are separate. This distinction is correct: external CSV quoting/schema/header topology, TSV separators/empty-record topology, BMP extended header/palette/alpha details, Deflate reserved nibble/schema/dictionary fields cannot all be transported by the canonical external carrier. Binary Binary remains genuine raw bytes rather than a private envelope.

Neutral expected domain rows are Binary1025, CSV1030, TSV1030, BMP1027. Actual Native candidates directly sum the typed database rows against fixture expectedRows. Deflate has its own full literal typed law and reserved method/window15, dictionary u32MAX state. BMP reconstruction narrows integers using checked u8/u16/u32/i32 conversions; u32 fields are represented exactly within SQLite signed64. These five snapshots have no persisted u64 literal or IEEE float field. Therefore they supply no evidence for general u64MAX/NaN-bit/signed-zero fidelity elsewhere; that evidence must come from owners with those literal fields.

## Interior Control and Retirement Gaps

Every actual SQLite owner still reconstructs schema with `document.text(1)?.into()`. Projection clones the entire schema. Binary projection line12, CSV line28, TSV line29, BMP document row and Deflate document row need the same owned controlled text-copy primitive; callbacks measured in payload/row units do not cover a large one-row literal schema copy. CSV field values also clone whole Strings on projection (line40) and reconstruct uncontrolled at line81. TSV fields have the same issue at lines41/82. The authored long-schema cancellation laws deliberately expose this pending semantic repair.

Drafts visibly scan/census and admit ownership before arrays, use cumulative Native controls for bounded copy/emission, and avoid invoking their ordinary native codec. This is source evidence only. Deflate additionally uses explicit retained Inflater history, checksum/count pass before exact output ownership, reset and materialization. History Drop drains `close_retained_step(256, usize::MAX)` and asserts empty terminal storage, but it has no cancellation/progress callback for the drain; this is bounded per step yet synchronous end-to-end retirement. It needs owning runtime cancellation/error-path evidence, rather than shared engine historical receipts. Ordinary candidate test scope does not separately demonstrate every retained-history retirement path.

Deflate controlled output calls first-party `deflate_controlled`, whereas its ordinary artifact codec has a separate fixed-Huffman producer. The authored output law requires exact existing Binary/Text bytes and dev-only miniz inflation. A valid alternative zlib stream is insufficient. Byte parity must be genuinely measured in the root Cargo lane.

## Actionable Owner Paths

All owner paths below are under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`:

- `🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:131`: stale constructor prerequisite.
- Each of `💾️binary@raw`, `📊️csv@rfc4180`, `📑️tsv@iana`, `🪟️bmp@v3`, `🗜️deflate@rfc1950` snapshot `🪶️sqlite/🦀️.rs`: real controlled literal schema projection/reconstruction; CSV/TSV literal fields as well.
- Each snapshot `🚦️native/🦀️.rs` and snapshot root: mount/bind only after authentic owner law baseline, then compile and measure interior behavior and exact external bytes.
- `🗜️deflate/📦️packages/🦀️rust/Cargo.toml`: missing direct controlled-engine dependency at eventual mount.

No Native GREEN, compiler success, cancellation runtime behavior or exact compression parity is claimed by this audit.
