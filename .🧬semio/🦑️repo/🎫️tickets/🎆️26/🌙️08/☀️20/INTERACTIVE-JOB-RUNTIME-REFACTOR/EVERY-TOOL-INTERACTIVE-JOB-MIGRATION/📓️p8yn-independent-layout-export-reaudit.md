# P8yn Independent Layout Export Re-Audit

## Verdict

**REJECT.** The repair closes the former public synchronous Wasm and `layout:out` paths, but the Layout export worker still performs legal, multi-kilobyte serialization, parsing, copying, and allocation in a single job unit. Therefore its one-work-unit/2 ms factory contract cannot establish the 8 ms ceiling.

## Accepted Source Evidence

- The four Wasm methods now return `LayoutExportOperation`; its single `step`, `progress`, `cancel`, and terminal take methods use `WorkerJobSession` and the process-wide worker-pool accessor, rather than `run_to_completion` (wasm bridge lines 243-359).
- The four command reducers fail closed. `LayoutPlayApp` registers concrete compiler-owned command and `layout:out` factories, constructs their request from framework operation/revision/generation material, and direct `layout:out` reducer access is `NotImplemented` (editor lines 210-245 and 260-274).
- The framework media route submits one pending step, returns `Running` on an empty first poll, resubmits only after an outcome, checks cancellation/staleness at terminal, and validates the O(1) producer credit against the public structural payload (framework plugin lines 14069-14235 and 11377-11435).
- PNG row/rectangle, PDF xref, ZIP entry/central-directory, base64, checkpoint, and final-copy cursors are materially improved. The only production `run_to_completion` is in the crate-private export module's named headless adapter; scene re-exports are `#[cfg(test)]` (export lines 1569-1629; scene lines 381-384).
- Static checks run in this audit passed: `rustfmt --edition 2021 --check` on the three modified Layout Rust sources, and `git diff --check` over Layout plus the relevant framework runtime files. No Cargo command was run.

## Blocking Defects

1. **Package serialization is not persistently cursor-bounded.** `json_fragment` serializes an entire value to a fresh `Vec` and `append_json_item` then feeds all of it to `ChunkRope::append` in one `advance_one` (export lines 1055-1066). `ChunkRope::append` iterates/hashes every supplied byte before returning. The collection stages clone and serialize whole `ParagraphStyle`, `CharacterStyle`, `TextStory`, `ImageLink`, `ParentPage`, `Spread`, and `Page` values (lines 1111-1117), while tail and missing-link paths do the same (1120-1123 and 1154-1171). Legal strings/fragments are up to 8 KiB/64 KiB, so neither the work nor allocation has a per-unit ceiling.

2. **Validation parses and walks whole legal JSON values in one worker step.** `validate_bounded_json` accepts up to `MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES` then calls `serde_json::from_str` into a whole `Value` and drains its stack in one call (lines 1466-1493). It is invoked as an individual `Validate` state advance for data fields and preflight (lines 750-756), while child/link values are wholly serialized during validation (761-765, 695 and 708). A byte/node cap is not an incremental-work bound.

3. **The terminal commit still performs an unbounded allocation.** On the first `PackageCommit` unit, `try_reserve_exact(self.active_output().len)` reserves the entire final candidate, up to 32 MiB, in one call (1299-1304). The later 4 KiB copy loop does not make that allocation bounded. A legal allocation/relocation can exceed the 8 ms ceiling.

4. **PNG's claimed 4 KiB granularity is not actual.** A legal 2,048-pixel RGBA row is 8,193 bytes. One `encode_png_one` unit updates Adler over the whole row, allocates/builds the full stored-DEFLATE block, updates CRC/appends it, and only then returns (923-1006, especially 985-995). Rectangle fill is tiled at 256 pixels, but final row encoding is not; `OUTPUT_CHUNK_BYTES` is only internal rope chunking and never yields mid-call.

These defects are source-visible at maximum legal envelopes. They are sufficient for rejection irrespective of the unrun watchdog gates.

## Required Repair

Replace the whole-value helpers with owned incremental codecs:

1. Store a durable package/JSON cursor `(section, collection index, record field, string byte offset, escape state, array/object stack)` and write at most a small fixed output credit per `advance_one`. Borrow from the immutable `Arc<LayoutSnapshot>` only for that turn; do not clone records. Dynamic JSON/preflight validation must be a resumable lexical/structural scanner with persistent byte/depth/node state, and supplied preflight bytes must still be copied byte-exactly in bounded slices.
2. Make page/story/style/link/parent/spread and child/link serialization field-by-field and string-byte-chunked. Add adversarial maximum-string/maximum-node/maximal-record tests that prove a single step cannot encode, validate, or append more than the declared credit.
3. Remove the full `try_reserve_exact` terminal allocation. Extend the job/commit boundary to transfer owned bounded chunks (or another genuinely incremental final representation) so completion does not flatten, reserve, or relocate a 32 MiB candidate in one job unit.
4. Split PNG Adler/DEFLATE/CRC output into bounded row-segment state; a row may remain stored in memory, but no step may scan or append the complete 8,193-byte row.

## Still Unrun After Repair

After the source repair, run serialized native debug/release and both Wasm targets; real 1/2/4/default process-pool factory dispatch; first-poll pending, duplicate submission, cancellation/drop, stale commit, per-instance head-of-line isolation; PNG/SVG/PDF/ZIP conformance and byte determinism; checkpoint/replay; all input/output/item max and max+1 envelopes; and watchdog measurements proving every worker slice is at or below 8 ms. Those gates were intentionally not run here because the workspace has insufficient Cargo disk headroom.
