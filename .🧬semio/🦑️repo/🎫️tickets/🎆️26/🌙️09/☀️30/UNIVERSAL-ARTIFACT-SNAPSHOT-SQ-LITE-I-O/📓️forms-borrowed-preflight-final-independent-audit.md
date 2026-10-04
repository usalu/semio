# Forms Borrowed Preflight Final Independent Audit

Read-only source review on 2026-10-03. No production or test edits, Cargo invocation, new native run, or fresh passing claim. Earlier run outcomes supplied by the parent are historical evidence, not independently rerun here.

## Mounted Authority

The actual Forms SQLite trait now delegates `preflight_sqlite_snapshot_encoding` to `native_pack::preflight`; the Pack root mounts `📏️bound/🦀️.rs`. The bound first executes the actual SQLite admission census, then constructs `NativeEncodingBound` using the same mutable `SqliteSnapshotControl`. This is an owner override, not the previous default UnsupportedOwner path.

## Allocation and Domain Review

The forecast owns borrowed reference frontiers and borrowed identity slots only. Full replacement buffers are reserved through the shared physical allocation control before references move. No snapshot clone, flat record materialization, string copy, output buffer, or recursive domain ownership occurs in this preflight. Reference-vector destruction does not recursively destroy caller-owned values or conditions. Cumulative physical allowance remains consumed after scratch destruction; bound arithmetic does not charge fictitious output allocation.

All nine intrinsic value variants and all six expression variants are covered by the reused census. Flat native values retain floating-point raw bits in `Option<u64>`; NaN payloads and infinity do not require native float literal conversion here. Optional scalars and text, absent or empty optional lists, bytes, arrays, object members, and expression links receive conservative allowances. Borrowed text length times six covers JSON-style escaped UTF-8 worst cases without scanning or copying text. Existing byte counts conservatively cover byte/base64 output and scalar decimal words. The fixed field allowance is 58 bytes and record allowance 16; the counted records include 4 step fields, 26 question fields (actual record has fewer), 2 option fields, 3 vector fields, 4 response fields, 4 answer fields, 9 value fields, 7 condition fields, and 2 member fields. Array and condition links receive an extra 21 bytes each. Root and child syntax receives 24 field and 7 record allowances plus the declared component-specific native envelope prefix.

Overflow-sensitive input counts, aggregate additions, and repeated multiplication are checked. Both DSL and Pack use the same deliberately conservative ceiling, with the correct component envelope prefix. `max_value_bytes` and `max_file_bytes` are output/admission ceilings; `max_allocation_bytes` pays only actual scratch here. A conservative bound can refuse some payloads that would actually fit; this is not an exact output-size predictor.

## Actionable Cancellation Finding

In `🪶️sqlite/📏️admission/🦀️.rs`, question validation evaluates `q.kind.trim().is_empty()` before counting that text. An arbitrarily large borrowed kind consisting of whitespace, or whitespace followed by a non-whitespace character, requires a full Unicode whitespace scan without any progress or cancellation checkpoint. This violates the requested bounded control for every expensive borrowed loop. Replace this scan with a controlled Unicode whitespace check that preserves `str::trim().is_empty()` semantics, and add a targeted long-kind cancellation law. This affects the reused admission owner and therefore both the newly mounted preflight and actual encoding census.

Other reviewed expensive traversals tick the control every 256 work units; identity string comparisons advance in at most 65,536-byte slices. Length-only escaped-output multiplication requires constant work and does not itself need a byte traversal.

## Test Independence and Remaining Evidence Limits

The new law measures actual System allocation requests independently from the physical control ledger, checks exact scratch allowance and minus one, compares empty-title and 140,000-byte-title requests, exercises both native encodings, separates value/file/allocation/row ceiling roles, checks entry cancellation and an interior EncodeNative callback, and derives a file ceiling from a real encoded payload. This is useful independent backing evidence rather than comparing two counters from the same implementation.

The current interior-cancellation assertion can be satisfied by the identity stage; it does not prove the long whitespace-kind scan is controlled. The law also does not exhaustively prove every record syntax and intrinsic domain reaches the conservative output bound. Source review supports the arithmetic, but no independent runtime execution was performed by this audit agent. Parent-owned native execution remains authoritative for the current 12-law outcome.
