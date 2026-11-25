# Deflate restart custody

Status: not reproducible on the current tree; the invariant is correct and kept.

## Callers
`RetainedDeflateEncoder::restart` has exactly one caller: `BorrowedPackDocumentCursor::start_segment`
(`🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs`), only in the measuring pass.
Every call site (segment 0 to 1, 1 to 2, each next frame in `FrameCatalog`, `ChunkBirth` to segment 2) is reached
only through `finish_segment` or `FrameCatalog`, both entered only once `deflate.is_complete()` (phase Complete, no pending bits)
and after the lent `compressed` byte was taken. The emit pass never restarts. So custody is complete at every legitimate restart.

## Root cause
No legitimate sequence restarts with output in custody. The boot fault came from a transient state of the concurrently edited
document/deflate code (see plugin-tests-06.md); it is not reproduced now. A leaked `[DEBUG]` diagnostic (Box::leak) left in
`start_segment` by an earlier investigation was the only deviation.

## Fix
Removed the debug residue: `start_segment` is back to `self.deflate.restart()?`. The invariant stays.

## Test
Existing law `borrowed_pack_document_every_frame_split_and_body_length_restarts_with_complete_custody` (frames 16..200, lengths 0..400,
single and batched, compared against the canonical oracle) passes: `cargo test --lib -j 2 restarts_with_complete_custody` in the
borrowed cursor crate. If the boot fault recurs, the first failing message now names the plain invariant; rebuild the hub and recheck.

## Changed files
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs`
