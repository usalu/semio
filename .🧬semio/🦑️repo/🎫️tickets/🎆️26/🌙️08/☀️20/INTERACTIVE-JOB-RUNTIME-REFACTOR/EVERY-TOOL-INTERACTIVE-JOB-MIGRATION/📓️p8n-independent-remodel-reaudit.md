# P8n Independent Remodel Re-Audit

## Verdict

**SOURCE REPAIR READY FOR INDEPENDENT RE-AUDIT — the P0/P1/P2 source findings are repaired. Runtime, Wasm, clippy, generated-descriptor, and timing gates remain deferred.**

This source/static repair pass completed on 2026-08-22. No Cargo command, generated-descriptor step, manifest/status/JSON edit, cache/target edit, Git command, or ticket-status mutation ran. The repository MCP remained unavailable; work stayed in the already-existing P8 ticket.

## Repaired Findings

### P0 — compressed ingestion no longer joins or duplicates whole input

- `FrameIngestion` owns `CompressedChunkRope`, whose leaves are independently shared `Arc<[u8]>` values capped at 4,096 bytes and whose aggregate length is checked against the MIME envelope (`images/🦀️component.rs:210-260`, `run-reconstruction/🦀️component.rs:282-303`).
- `BoundedStillDecoder` has no `Joining`, `Vec<Vec<u8>>`, `Cursor<Vec<u8>>`, `concat`, or flatten path. PNG retains a persistent rope reader and advances one decoded scanline per continuation. JPEG advances a 4-KiB header probe cursor, then the fixed 131,072-byte/262,144-pixel baseline decoder reads the same rope through `JpgByteSource`; it does not create a second whole compressed vector (`images/🦀️component.rs:266-407`, stdio JPG `io/🦀️component.rs:797-1085`).
- Source regressions assert every rope leaf is at most 4 KiB, the whole-input materialization counter does not move for PNG or JPEG, oversized/malformed envelopes reject, and cancellation during streaming releases the retained rope before decode/publication (`images/🦀️component.rs:1260-1410`, `run-reconstruction/🦀️component.rs:1521-1549`).

### P0 — durable asset and mesh admission is aggregate- and semantics-bounded

- Asset staging selects an exact `Sparse` or `Raster` content kind on its first chunk; caps aggregate bytes/chunks at 6,144/2 or 1,114,112/272; caps active staging blobs at 32; rejects non-contiguous indices, content-kind changes, malformed base64, oversized chunks, aggregate overflow, and checked arithmetic overflow before retention (`remodel/🦀️component.rs:211-470`).
- Mesh staging caps aggregate content at 87,582 bytes/30 chunks and 32 active blobs. It enforces monotonically ordered fields, four-byte component shapes, per-field semantic cardinalities, UTF-8/texture bounds, and rejects the 513th vertex or triangle before retaining the offending chunk. Commit rechecks exact count, bytes, digest length/content id, field envelope, valid triangle indices, and the 512-vertex/512-triangle resolution limit (`remodel/🦀️component.rs:577-809`).
- Cancellation, supersession, stale checkpoints, terminal failures, and explicit user cancellation discard the bounded private staging associated with the removed session (`run-reconstruction/🦀️component.rs:346-457`, `803-912`).
- Public source coverage now includes multi-chunk aggregate overflow, kind mismatch, malformed field order/component count, 513th vertex/triangle, Busy admission, and cleanup-to-zero assertions (`run-reconstruction/🦀️component.rs:1344-1439`).

### P0 — every active handler turn emits exactly one durable mutation

- `TerminalPreparation` no longer contains `prepared_mutations`. Sparse, mesh, and raster staging turns each emit one compact chunk/verification mutation. Progress-only turns emit one job mutation. `TerminalPhase::Commit` emits one typed `CommitReconstruction` containing compact handles and bounded metrics; continuation scheduling stays in ephemeral `Effect::DispatchAction` (`run-reconstruction/🦀️component.rs:60-84`, `724-740`, `916-932`).
- `CommitReconstruction` applies one atomic document diff for job, sparse cloud, trajectory, mesh, geo products, QC, and named assets, while commit-time code rechecks staged asset/mesh count, digest, content kind/envelope, and mesh semantics (`commit-reconstruction/🦠️mutation/🦀️component.rs:12-45`, `🔺️diff/🦀️component.rs:6-36`).
- The public ActionBus/worker regression asserts `result.mutations.len() == 1` on every active turn from start through terminal completion (`run-reconstruction/🦀️component.rs:1156-1170`). Cancel emits exactly one job mutation; stale delivery emits none.

### P1 — accounting and cursor math is checked

- Content and mesh preparation propagate retryable typed `Fault` on digest-length, chunk-count, field, verification, and serialization cursor overflow. Shared staging returns typed `RemodelStagingFault::{Busy, Invalid}` and removes invalid partial blobs. Raster PNG returns `RasterPngProgress::Failed` on digest-length, chunk-count, row/range, DEFLATE-length, or PNG-chunk-length overflow.
- Hash-lane multiplication remains intentionally wrapping as fixed-width digest mixing; all lengths, counts, and relevant content/mesh/raster indices use checked arithmetic. Direct source tests seed `u64::MAX` digest/count state and assert failure rather than wrap (`run-reconstruction/🦀️component.rs:1404-1439`, engine `🦀️component.rs:454-470`).

### P2 — decorative async removed from builders

- `preview_job`, `reconstruction_stage`, `RasterAssetPreparation::advance`, all content/mesh chunk builders, `replace_job`, `replace_sparse`, `replace_mesh_result`, `replace_qc`, `replace_geo_products`, `replace_trajectory`, `create_asset`, packed-value constructors/accessors, and `next_remodel_id` are synchronous.
- Async remains only at framework/protocol boundaries whose traits require futures or in tests that genuinely await framework operations. Static scans find none of the rejected no-await builder signatures or stale `.await` call sites.

## Preserved Proof

The prior public replay proof remains: all three public starts traverse `ArtifactEditor::command_from_action`, typed dispatch, ActionBus, worker continuations, terminal completion, typed `OpText` genesis replay, total process-state clearing, mesh/input reacquisition, checkpoint pack, a second state clear, and restore. Concurrent two-document cancellation/stale/ABA coverage is retained.

## Source and Static Validation

- `rustfmt --edition 2021` completed on the repaired artifact, engine, command, stdio JPEG, and mutation Rust sources.
- Static scans found no old whole-input join shapes, `prepared_mutations`, rejected decorative-async builders, stale sync-builder awaits, unchecked digest/chunk increments, or multi-mutation active Remodel emits.
- `bun ./📜️script.ts verify interactivity tool-jobs` exited 0 on 2026-08-22: 775 production rows, 775 bounded rows, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch path, and 773 unique rows.

These are source/static results only. They do not claim compilation, runtime behavior, allocation timing, native/wasm type-checking, or clippy success.

## Deferred Executable Gates

1. `cargo test -p semio-s-plugin-remodel --lib` in debug mode.
2. `cargo test --release -p semio-s-plugin-remodel --lib` for maximum-envelope/timing coverage.
3. `cargo check -p semio-s-plugin-remodel --target wasm32-wasip2`.
4. `cargo clippy -p semio-s-plugin-remodel --target wasm32-wasip2 -- -D warnings`.
5. Sanctioned descriptor regeneration/comparison for `cancelReconstruction` and `CommitReconstruction`.
6. The master Phase 8 ActionBus/tool-job quick suite after the Rust lanes are available.
