# Remodel Resumable Reconstruction

## Outcome

The Remodel rejection repair is ready for an independent source re-audit. `runReconstruction`, `runStage`, and `retryStage` share the registered `advanceReconstruction` continuation and retain the requested stage in every generation-tagged checkpoint. `cancelReconstruction` is a user-addressable registered action. Production defaults use fixed empty/box child handles, reconstructed meshes expose only bounded whole resolution or individual durable chunks, and the arbitrary whole-mesh mint/resolve helpers are test-only.

No Cargo command ran. Every `<8 ms` assertion described below is authored source coverage, not a claimed execution result; debug and release proof remains deliberately deferred to the Cargo owner.

## Second Re-Audit Repair Addendum

The 2026-08-22 P8n rejection findings were repaired in source:

- Compressed frame input is now a persistent `CompressedChunkRope` of at-most-4-KiB `Arc<[u8]>` leaves. PNG owns a rope reader; the shared baseline JPEG decoder reads the same rope through `JpgByteSource`. The old retained `Vec<Vec<u8>>`, `Joining`, whole-input `Vec`, and `Cursor<Vec<u8>>` shapes are gone. PNG/JPEG source regressions assert actual sequential/random rope-reader metrics, and cancellation drops a partially ingested rope before decoding.
- Durable sparse/raster staging now selects an exact content kind and enforces checked aggregate byte/chunk caps plus a 32-blob Busy limit. Mesh staging enforces its exact aggregate cap, monotone field order, component shapes, field cardinalities, 512 vertices/512 triangles, and bounded texture content before retention. Commit repeats count/digest/envelope validation; cancellation, supersession, stale checkpoints, and terminal failures clean private staging.
- Terminal work no longer accumulates a mutation vector. Every active handler turn emits exactly one durable mutation, and the final turn emits one typed `CommitReconstruction` carrying compact handles and bounded metrics; scheduling remains ephemeral. The public ActionBus drive asserts one mutation on every active turn through terminal completion.
- Digest lengths, chunk counts, serialization/verification cursors, raster rows/ranges, and staging indices now use checked arithmetic with typed `Fault`, `RemodelStagingFault::{Busy, Invalid}`, or `RasterPngProgress::Failed` propagation. Direct `u64::MAX` source tests cover content, mesh, and raster accounting.
- The rejected no-await helpers/builders are synchronous, including preview/stage mapping, result mutation constructors, packed-value conversions, and terminal content/mesh/raster preparation.

`rustfmt --edition 2021` completed on the repaired Rust sources, and `bun ./📜️script.ts verify interactivity tool-jobs` again exited 0 with 775/775 bounded production rows and zero batch-only/forbidden/deleted rows. This is source/static evidence only; Cargo, runtime, Wasm, clippy, descriptor generation, and timing execution remain deferred.

## Worker Ownership, Freshness, and Persistence

- Sessions live in a process-wide `Mutex<BTreeMap<generation, ReconstructionSession>>`, not thread-local state. Admission is capped at 32 active generations and never evicts an active worker-owned session.
- Every continuation validates document job id, generation admission, requested stage, phase, stream/frame cursor, terminal cursor, tick, and cancellation before work or publication. Supersession and explicit cancellation discard private staging and stale continuations emit nothing.
- Mesh payloads are encoded field-by-field into at most 4,096-byte chunks. Each chunk is a durable `CreateAsset` event-log row. The incremental content id carries four independent 64-bit digest lanes plus byte length; equal-id reuse verifies one chunk per continuation before a compact commit.
- Committed handles identify snapshot-owned content-addressed leaves and resolve only after the exact chunk count passes the 512-vertex/512-triangle field envelope. The public cold-restart source regression records typed `OpText`, clears all private asset/mesh staging, live sessions, admissions, and generation state, replays one typed row per step from genesis, and compares every input image plus the exact terminal mesh handle/value. It clears process state again before checkpoint-pack restore and repeats those assertions.
- DSM/DTM PNG content uses the same staged/verified/content-addressed event-log boundary. Named raster assets and the fixed-cardinality compact result mutations remain private until terminal commit, so cancellation cannot expose partial authoritative reconstruction results.
- Shared durable asset and mesh admission rejects malformed base64, decoded payloads above 4,096 bytes, non-contiguous indices, and overflowing index/count relationships. Terminal raster PNG row emission is also calculated against a 4,096-byte framed chunk ceiling.

## Bounded Production Units

### Input and codecs

- Base64 decoding consumes at most 4,096 source characters per handler turn. In-session compressed input is admitted up to 1,114,112 bytes and never reserves the whole joined buffer.
- PNG admission is at most 262,144 pixels and 4,096 pixels per scanline. The reader advances one scanline per step and grows the output by that row; it performs no full-output reserve or terminal flatten/copy.
- JPEG admission is at most 131,072 compressed bytes and 262,144 pixels. The shared baseline decoder is retained as a fixed-envelope unit because it exposes no MCU checkpoint. Maximum accepted, oversized, and malformed-entropy timing fixtures are authored for execution in both build profiles.

### Reconstruction engine

- Image count is capped at 64; each RGBA frame is capped at 262,144 pixels / 1,048,576 bytes. Admission, fixed buffer reservations, and malformed shape rejection have maximum-envelope timing fixtures.
- Feature work is 4,096 luma/detection pixels or 16 descriptors per turn. Matching is 4,096 descriptor comparisons. Track construction is 4,096 observations or 64 groups.
- SfM admits 64 seed correspondences and 32 seed hypotheses; registration admits 64 PnP correspondences, one RANSAC hypothesis per turn, and at most 64 attempts; triangulation and bundle residual work admit eight observations; at most 512 tracks are scanned. The engine calls seed, registration, and bundle continuation APIs with fuel `1`, so one retained fixed solve/track unit is the whole scheduler turn.
- The former whole SfM driver (`run_all`, `init_pair`, registration/two-view fallbacks, whole BA/prune/retriangulate helpers, and whole reconstruction snapshot facade) is test-only. The whole raster encoder, non-interactive mesh constructors, arbitrary `mint_and_stash_mesh`, and full-cloning `remodel_mesh_workspace` are also test-only; production can construct only the bounded path.
- PatchMatch allocates inside the fixed 512x512 frame envelope, then advances one pixel per engine turn. Dense luma copies 2,048 pixels. TSDF integration advances 256 ray samples. Fusion advances 256 comparisons and retains at most 512 output points; its constructor cannot reserve an input-sized cloud.
- Meshing copies 2,048 points or 4,096 texture bytes per setup turn. The production mesh pipeline is `new_bounded`, admits at most 512 vertices / 512 triangles, caps texture atlases at 64x64 and Taubin iterations at four, and cursorizes extraction, clean, repair, orientation, hole fill, validation, smoothing, simplification, unwrap, texture bake, PNG publication, and interchange.

### Terminal path and public handler boundary

- Sparse output copies 64 cameras and 256 points per turn; quality reduces 256 observations; geo bounds/binning advances 256 points. DSM/DTM raster allocation extends 256 cells per raster per turn, and stored-DEFLATE PNG publication scans/emits at most a 4,096-cell window.
- Mesh serialization/hash/verification emits at most one 4,096-byte work event per turn. Final `ReplaceMeshResult` contains only the replayable handle and compact metadata.
- The public source regression enters through `ArtifactEditor::command_from_action` and `VcsArtifactApp::dispatch_typed`, so start commands and every actual `DispatchAction` continuation cross the ActionBus/worker boundary. It drives admission, input decode, features, matches, SfM, dense, mesh, and terminal commit for all three public start actions without seeding a terminal session or calling `handle_advance` directly.

## Authored Adversarial Coverage

- distinct requested-stage dependency prefixes and stage-preserving checkpoints;
- stale generation, explicit cancellation, cancellation cleanup, 33rd-session backpressure, and two-document isolation;
- public ActionBus/worker traversal for all three starts, every reconstruction stage through `Done`, typed-row genesis replay, input-image reacquisition, exact terminal-handle recovery, and cold checkpoint restore after a second total process-map clear;
- two concurrently live documents, explicit cancel, stale continuation rejection, and generation/job ABA non-reuse through public actions;
- exact 4 KiB asset/mesh admission plus oversized, malformed-base64, and overflowing-index rejection;
- maximum/malformed PNG and JPEG decoding, maximum image admission, feature/match/track steps;
- 64-correspondence/32-hypothesis seed solve, degenerate seed geometry, 64-correspondence PnP, failed PnP, and eight-observation triangulation;
- maximum PatchMatch fixed reservations and one-pixel cost work, 12-view fusion/256 comparisons, 256-sample TSDF integration, and malformed TSDF parameters;
- accepted and rejected TSDF surface extraction, every interactive mesh postprocess stage, 64x64 texture bake/PNG publication, and 512x512 terminal raster PNG;
- test-only large mesh chunking across worker threads, retained as lower-level chunk-codec coverage rather than the public restart proof.

## Source and Static Validation

- `rustfmt --edition 2021` completed on the touched Remodel engine, command, artifact, and mesh-diff Rust sources.
- Static source scan finds no `async fn` in the Remodel engine tree.
- Static source scan confirms the legacy whole SfM/raster/non-interactive mesh facades are `#[cfg(test)]` and the production engine passes fuel `1` to seed, registration, bundle, and mesh continuation APIs.
- `bun ./📜️script.ts verify interactivity tool-jobs` exited 0 on 2026-08-22: 775 production rows, 775 bounded rows, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch path, and 773 unique rows.

The static verifier proves catalog/source classification only. It does not substitute for compiling or executing the timing regressions.

## Deferred Cargo and Generated Gates

1. `cargo test -p semio-s-plugin-remodel --lib` in debug mode, including every authored `<8 ms` assertion.
2. `cargo test --release -p semio-s-plugin-remodel --lib`, especially fixed-envelope JPEG, PNG, SfM, PatchMatch, fusion, TSDF, mesh, raster, and public-handler timing tests.
3. `cargo check -p semio-s-plugin-remodel --target wasm32-wasip2`.
4. `cargo clippy -p semio-s-plugin-remodel --target wasm32-wasip2 -- -D warnings`.
5. Regenerate and compare plugin descriptor assets through the sanctioned existing generator so `cancelReconstruction` is reflected in generated descriptor output; no generated file was hand-edited.
6. Run the master Phase 8 ActionBus/tool-job quick suite after the Cargo/timing lane is released.

## P8q Final Repair Addendum

- `RemodelSnapshot` now owns a serializable content-addressed artifact store whose only payload leaves decode to at most 4 KiB. Input images, terminal rasters, sparse values, and mesh fields reconstruct from that document-owned store after all process state is cleared; production no longer has `REMODEL_ASSET_BLOBS`, a whole-`ImageAsset` registry, or a committed whole-mesh/payload map.
- Private staging is keyed by document, app, operation, generation, and artifact identity. The terminal mutation first validates and materializes every durable value, then performs one validate-all/apply cleanup transaction under both staging locks. A late raster or mesh error cannot publish or consume an earlier staging item.
- The inert no-join counter was replaced by rope reader instrumentation for sequential calls/bytes/largest read and JPEG random-byte reads. PNG/JPEG regressions now exercise the counters on the actual rope read paths.
- Pure Remodel production value builders, mutation diff/inverse constructors, replacement mutations, placeholder/empty mesh builders, import helpers, preview/stage mapping, mesh conversion, and packed-value queries are synchronous. Async remains at framework handler/trait and genuine suspension boundaries; no `block_on` was added.
- Public replay coverage now inspects every durable leaf, clears all Remodel process state, replays serialized typed rows from genesis, and restores a checkpoint before resolving mesh, sparse data, and input assets from document state. The existing two-document, cancel/supersede cleanup, stale/ABA, exact-cap/+1, overflow, and malformed cases remain source-authored.

Runtime, native debug/release, Wasm check/clippy, descriptor regeneration, and timing gates remain explicitly **UNRUN** under the disk-space prohibition. This addendum records source/static disposition only.

## P8r Final Repair Addendum

The 2026-08-22 P8r final-audit rejections are repaired in source:

- Every pure `args_bridge` parser and its private command builder is synchronous. The framework-required outer `ArtifactEditor::command_from_action` remains async and returns the synchronous bridge value directly; no stale parser await or future/value mismatch remains.
- Generic `CreateAsset` rejects reconstruction content handles, its legacy magic commit-key parser/path and standalone asset commit helpers are deleted, and `ReplaceMeshResult` rejects private `mesh-stage:` handles. The only arbitrary mesh commit helper is `cfg(test)`; production promotion remains exclusively in `CommitReconstruction`'s validate-all/apply transaction.
- `RemodelAssetChunkSource` decodes each snapshot-owned durable leaf independently to `Arc<[u8]>` with a checked aggregate cap and a 4-KiB leaf ceiling while carrying identity, MIME, and dimensions separately. Active `FrameIngestion` constructs `CompressedChunkRope` directly from those leaves and no longer owns an `ImageAsset`, base64 cursor, process-content id, or process-content cursor. `remodel_asset` is absent from active reconstruction and remains a bounded export/UI/inverse facade only.
- Rope regressions now bound total sequential calls/bytes, total JPEG random-byte accesses below two input passes, largest sequential reads to 4 KiB, and random access to one byte. A production-constructor regression exercises snapshot durable storage through `frame_ingestion` and the real PNG decoder without whole compressed reassembly.

`rustfmt --edition 2021` completed on the repaired sources. `bun ./📜️script.ts verify interactivity tool-jobs` exited 0 with 775/775 bounded production rows, zero batch-only/forbidden/deleted rows, one factory/registration/dispatch path, and 773 unique rows. Scoped diff/debug/decorative-async/global-registry/legacy-promotion/join scans found no new production blocker; diagnostic prints and unrelated `Vec<Vec<u8>>` matches are test/video-codec fixtures, and the arbitrary direct mesh commit is test-only.

Cargo tests/build/check/clippy, native debug/release runtime and timing suites, Wasm gates, cache/target operations, generated descriptor regeneration, and modifying Git commands remain explicitly **UNRUN** under the disk-space prohibition.

## P8s Final Repair Addendum

The 2026-08-22 P8s final-audit rejection and obsolete-surface challenge are repaired in source:

- JPEG evidence now uses checked multiplication and a strict predicate requiring nonzero one-byte random accesses below 2 * input_len; zero input, one-byte input, multiplication overflow, a simulated second complete input pass, and an invalid multi-byte unit are explicit negative/edge fixtures. The real encoded 64×64 JPEG fixture remains the positive source path.
- The positive JPEG fixture also asserts a tighter checked ceiling derived from the owned decoder's monotone no-restart source pass plus Remodel's bounded SOF0 safety probe, and proves that ceiling itself is below a duplicate full pass. Metrics remain cumulative across admission and decode; no access is hidden and no counter is reset.
- The inert process-state deduplication protocol is deleted: no asset/mesh verification keys, verification mutation effects, staged-chunk comparison helpers, process content-exists checks, reconstruction verification cursors, verified-against state, or committed staging state remains. Raster and mesh terminal preparation proceed directly from bounded staging to the existing single CommitReconstruction.
- The terminal mutation still materializes snapshot-owned durable leaves before the existing locked validate-all/apply cleanup transaction. The lower-level cross-thread mesh fixture now resolves from a RemodelDurableArtifactStore rather than a process committed insertion; arbitrary test meshes use an isolated test-only registry.

rustfmt --edition 2021 completed on the five touched Rust sources. bun ./📜️script.ts verify interactivity tool-jobs exited 0 with 775/775 bounded production rows, zero batch-only/forbidden/deleted rows, one factory/registration/dispatch path, and 773 unique rows. Scoped static, obsolete-protocol, reassembly, cumulative-metrics, diff, and global-state scans found no P8s production blocker.

Cargo/build/tests, native debug/release runtime and timing gates, Wasm check/clippy, descriptor generation/comparison, cache/target operations, ticket metadata operations, and modifying Git commands remain explicitly **UNRUN**. This addendum records source/static evidence only.
