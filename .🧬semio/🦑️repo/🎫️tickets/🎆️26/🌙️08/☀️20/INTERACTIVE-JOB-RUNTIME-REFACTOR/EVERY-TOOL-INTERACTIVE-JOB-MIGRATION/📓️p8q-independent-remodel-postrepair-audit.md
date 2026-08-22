# P8q Independent Remodel Post-Repair Audit

## Verdict

**REJECT — current source still has production decorative-async builders (including two direct
future/value type mismatches), retains a process-global whole-asset facade, and the claimed
no-join allocation tests have inert instrumentation.** This is a read-only source/diff audit on
2026-08-22; no Cargo, build, test, cache/target, generated-descriptor, ticket-status, or
Git-mutating command ran.

## Scope And Method

- Read `AGENTS.md` and the complete P8e, P8l, and P8n reports, including P8e's repair addendum.
- Inspected the current Remodel artifact, bounded-image decoder, reconstruction continuation,
  staged asset/mesh admission, terminal commit diff, direct stdio JPEG reader, public replay
  tests, and the scoped working-tree diff.
- Read-only `rg`, `sed`, `nl`, and `git diff` were used. The worktree contains extensive
  concurrent unrelated changes; no conclusion below depends on them.

## Rejection Findings

### P0 — Decorative Async Persists In Production Builders And Produces Direct Type Mismatches

`placeholder_result` is a no-await, pure value builder but remains `async` at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️reset-placeholder-mesh/🦀️component.rs:12-14`.
Its production handler passes `Box::new(placeholder_result())` to `replace_mesh_result` at line
43. `replace_mesh_result` requires `Box<RemodelMesh>`, not a future, at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🦠️mutation/🦀️component.rs:14-21`.

The identical error occurs for no-await `empty_result` at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-result/🦀️component.rs:12-14`,
which its production handler passes as `Box::new(empty_result())` at line 44. These are source
type mismatches; native compilation is deliberately unrun, but the required signatures are
visible directly in source.

The issue is transitive rather than isolated: pure production helpers also remain async in the IO
and import path, including `mesh_data_to_semio_mesh` at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs:42-50`,
`remodel_mesh_from_document` at lines 94-101, `batch_stream_id` at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-frame-payload/🦀️component.rs:20-26`,
and the no-await local blur-gate helpers at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-frame-payload/🦀️component.rs:30-90`.
This fails the required full-transitive decorative-async removal.

### P0 — A Process-Global Whole-Asset Facade Remains On The Production Ingestion Path

`REMODEL_ASSET_BLOBS` is a process-global `OnceLock<Mutex<HashMap<String,
Arc<ImageAsset>>>>` at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:258-287`.
It stores whole `ImageAsset` values (including the complete base64 `data` payload), and public
`mint_and_stash_asset` clones that whole value into the map at lines 289-308. There is no
per-asset byte cap or chunk-only representation at this facade.

The continuation actually consumes that global payload through `shared_remodel_asset` when it
constructs `FrameIngestion` at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs:1021-1038`.
It is therefore not a test-only compatibility helper. This contradicts the requested absence of a
process-global whole-asset facade; the bounded `CompressedChunkRope` only starts after that facade
has supplied the entire encoded asset.

### P1 — The Claimed No-Join Allocation Regression Is Vacuous

`WHOLE_COMPRESSED_INPUT_MATERIALIZATIONS` is declared and only loaded at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs:200-206`;
a scoped source scan found no increment/write. The PNG and JPEG tests compare this permanently
unchanged counter at lines 1264-1284 and 1350-1366. Thus those assertions cannot detect a joined
or duplicated compressed input allocation. They are authored, but do not provide the requested
no-join allocation proof.

### P1 — `CommitReconstruction` Has Non-Transactional Global Staging Side Effects

The atomic document diff calls mutable `commit_staged_remodel_asset_kind` for sparse content at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🔺️diff/🦀️component.rs:6-13`,
then mutably commits each raster at lines 15-24, then mutably commits mesh staging at lines 30-40.
Each commit removes/reinserts the global staging blob at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:458-477` and `:715-735` before
later validation can fail. A later invalid raster/mesh therefore returns an error after an earlier
private blob has already become committed; there is no validate-all-then-apply/rollback path. This
does not meet a strict atomic terminal staging/cleanup interpretation.

## Confirmed Source Evidence That Does Not Clear The Rejection

- `CompressedChunkRope` retains `Arc<[u8]>` leaves and rejects leaves above 4 KiB with checked
  aggregate arithmetic (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs:208-262`);
  PNG owns a rope reader and JPEG calls `decode_jpg_source(&rope)` at `:264-405`. The stdio JPEG trait is random access and
  `decode_jpg_source` takes that trait (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/🦀️component.rs:795-843`).
- Asset staging selects `Sparse`/`Raster` before retention and checks raw chunk, sequence,
  aggregate byte/chunk, and digest arithmetic (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:334-420`); mesh
  staging checks order, component width, counts, limits, and overflow at `:593-686`, with final
  envelope checks at `:715-735` and `:784-807`.
- The active terminal handler emits one mutation via `yield_terminal` (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs:727-744`) and emits one typed `CommitReconstruction` mutation at `:919-935`.
- Authored tests cover multi-chunk aggregate overflow, kind mismatch, malformed field order and
  component width, the 513th vertex/triangle, and accounting overflow at
  `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs:1370-1435`;
  public ActionBus replay/freshness/two-document/cancel/stale/ABA coverage is at `:1250-1347` and
  cancellation cleanup at `:1510-1557`. None was run.

## Required Unrun Gates

All executable gates remain **UNRUN** because this audit was explicitly prohibited from running
Cargo/build/tests due to critical disk space:

1. Native debug: `cargo test -p semio-s-plugin-remodel --lib`.
2. Native release/timing: `cargo test --release -p semio-s-plugin-remodel --lib`.
3. Wasm compile: `cargo check -p semio-s-plugin-remodel --target wasm32-wasip2`.
4. Wasm lint: `cargo clippy -p semio-s-plugin-remodel --target wasm32-wasip2 -- -D warnings`.
5. Runtime ActionBus public replay/cold-restart/two-document/cancel/stale/ABA/max/malformed suite.
6. Runtime PNG/JPEG maximum-envelope, malformed-entropy, bounded-progress, and real allocation
   instrumentation suite in debug and release.
7. Sanctioned descriptor regeneration/comparison for `cancelReconstruction` and
   `CommitReconstruction`.

The P0 source findings must be repaired before these gates can establish an accept decision.

## Repair Disposition — 2026-08-22

All four rejection findings were repaired in source:

- **P0 decorative async:** the two direct `Box<Future>` mismatches and the transitive production helper chain are synchronous, including all live mutation diff/inverse/builders, result replacements, placeholder/empty results, mesh/result conversion, import blur/batch helpers, stage display, payload parsing, and packed queries. No stale await or `block_on` was introduced.
- **P0 authority:** `REMODEL_ASSET_BLOBS` and whole `ImageAsset`/production `MeshData` committed facades were deleted. Durable snapshot state owns compact content handles and base64 leaves whose decoded size is capped at 4 KiB; cold replay and checkpoint reopen resolve exclusively through the snapshot-owned store. Private bounded staging uses document/app/operation/generation identity and is discarded on terminal commit, cancellation, or supersession.
- **P1 no-join proof:** the inert materialization counter was removed. `CompressedChunkRope` now records actual sequential reads/bytes/largest read and JPEG random-access reads; PNG and JPEG source regressions assert those exercised paths.
- **P1 atomic staging:** `CommitReconstruction` materializes and validates all sparse/raster/mesh durable values first, then one transaction validates all staged entries under both locks before removing any. There is no fallible reconstruction/publication step after apply begins.

Public source coverage includes two-document isolation, cancel/supersede cleanup, stale/ABA rejection, exact maximum/+1/malformed/overflow admission, leaf-size/no-hidden-whole-payload checks, total process-state clear followed by typed-row replay, and serialize/checkpoint reopen. Rope PNG/JPEG decoding, 512 vertex/triangle limits, checked arithmetic, and one `CommitReconstruction` durable event per terminal turn are preserved.

Native debug/release runtime tests, Wasm check/clippy, timing suites, and sanctioned descriptor regeneration remain explicitly **UNRUN** because Cargo/build/test and generated-output gates were prohibited for disk safety. This disposition is source/static evidence, not a runtime acceptance claim.
