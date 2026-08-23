# Sol Read-only Phase 4 Puzzle 3D Vertical-slice Status/Gap Audit — 2026-08-23

## Decision

**RED.** The current tree has a real isolated-job route, persistent fill/collision cursors, deterministic generation checks, and a renderer-reachable candidate ghost. It does not yet have fixed planner/index ownership, exact cancellation/close, or a bounded worker-envelope path. In particular, both live `fillBuildTick` routes call `enqueue_fill_job`, which clones and serializes the complete worker state before emitting `Effect::SpawnJob`; the post-encode ceiling is 4 MiB. The nested collision/search preview is carried in JSON but is not consumed by `World3dHost`, so only the ghost is visibly implemented.

This is a read-only source audit. No source, test, verifier, ticket checklist, runtime, or lifecycle state was changed. Cargo, Nx, Wasm, browser, and runtime gates were not invoked.

## Read Set

- `📓️p4-baseline-and-packet-map.md`
- `📓️p4a-fill-job.md`
- `📓️p4b-collision-state-machine.md`
- `📓️p4c-preview-action-integration.md`
- `📓️p4-closure-audit-20260822.md`
- `📓️p4-closure-blocker-repair-20260822.md`
- `📌️important.md`
- live Puzzle 3D fill action, precompute bridge, `FillBuilder`, collision geometry/index, Puzzle registration, editor render projection, and `World3dHost` consumer.

## Exact Production Caller Census

All counts below exclude code below each file's first `#[cfg(test)]` boundary.

| Authority or route | Definition | Exact production callers/reachability | Test-only census |
| --- | --- | --- | --- |
| `FillStep` | **No type or value exists.** | Zero. The only match is the stale prose reference in `⏳️precompute/🦀️component.rs:110`. | Zero. |
| `FillBuilder` | `⏳️precompute/🪣️fill/🦀️component.rs:196`; `InteractiveJob` implementation at `:1221`. | Exactly two `FillBuilder::new` sites: scene rebuild at `⏳️precompute/🦀️component.rs:191` and worker-state restoration at `:869`. `InteractiveJob::step` is reached through the one `drive_step` site at `:557`. | Seven `FillBuilder::new` call sites: two in the fill module and five in the precompute module. |
| Fill action entry | `fill_build_tick` at `🎮️commands/🪣️fill-build-tick/🦀️component.rs:18`; cached variant at `:35`. | Exactly two dispatches: ordinary action dispatch at editor `🦀️component.rs:2326`, cached `ArtifactApp::handle` route at `:2419`. Both call `poll_fill_job` and `enqueue_fill_job`; both can serialize the whole worker envelope synchronously. | Focused action assertions begin at editor `🦀️component.rs:3873`; the isolated-spawn assertion is at `:3905`. |
| Worker bridge | `enqueue_fill_job` at `⏳️precompute/🦀️component.rs:895`, `poll_fill_job` at `:909`, `drive_fill_job` at `:917`, `fill_job` at `:1001`. | `enqueue_fill_job` and `poll_fill_job` each have exactly the two action callers above. `drive_fill_job` has one production caller inside `fill_job` at `:1017`. `fill_job` has exactly one plugin registration at `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs:67`. | One test helper calls `drive_fill_job`; focused cold-reopen/ABA/determinism/worker-count/preview tests are in the precompute test module. |
| `CollisionOverlapState` | `⏳️precompute/📐️geometry/🦀️component.rs:561`. | Exactly two constructor call sites: fill collision at `🪣️fill/🦀️component.rs:929` and brush-suggestion collision at precompute `🦀️component.rs:391`. The latter is UI/product reachable through suggestion refresh. | Eight textual constructor call sites in geometry tests, including deterministic batch, checkpoint, cancellation, early reject, and watchdog coverage. |
| `CollisionSpatialIndex` | `⏳️precompute/📐️geometry/🦀️component.rs:429`. | Owned only by `FillBuilder`. It is constructed in `FillBuilder::new` and `rebuild_collision_index`; live upserts occur during initial/rebuild scans and acceptance. Live broad phase calls only `entry_intersects` once per entry at fill `:894`; `query` has zero production callers. | Two direct index constructors exercise exact ordering and adversarial cell spans. |
| Fill preview producer | `world_fill_preview_json` at main-window `🦀️component.rs:405`. | Exactly one caller at `:469`, from `render`; editor app render calls the window at editor `🦀️component.rs:2547`. The result is passed as `world3d.brushPreviewJson`. | Typed preview/checkpoint source tests exist in the fill module; editor render/action tests cover ghost/reveal behavior. |
| Renderer consumer | `WorldBrushPreviewRecord` at `World3dHost/🟦️component.tsx:277`; `BrushPreviewGhost` at `:2740`. | The ghost is rendered at `:4927`. The consumer schema has no `fillBuildPreview`, broad-phase, pair, collision-id, rejection, or sample fields; repository-wide search finds `fillBuildPreview` only at its Rust producer. | Existing renderer tests cover brush-preview mesh URL/fallback, not Puzzle fill collision overlays. |

The live chain is therefore:

`fillBuildTick` dispatch → `poll_fill_job` → `enqueue_fill_job` → whole `fill_worker_checkpoint_bytes` → isolated `Effect::SpawnJob` → registered `fill_job` → full restore/decode → `JobCtx::tick().await` → `drive_fill_job` → `precompute_step_lane(Fill, 1)` → `drive_step(FillBuilder)`.

## RED/GREEN Gate Matrix

| Gate | Status | Live evidence |
| --- | --- | --- |
| P4a persistent fill state machine | **GREEN** | `FillJobStage` and the target/candidate/accept subphases retain cursors; `FillBuilder` implements `InteractiveJob`. |
| Operation/generation freshness before fill-step mutation | **GREEN** | `FillBuilder::step` rejects cancellation and operation/generation mismatch at `:1223-1229`; the worker request is checked against live operation/generation at precompute `:917-927`; restore rejects mismatched admitted/restored requests. |
| Deterministic sequencing | **GREEN** | Persistent RNG, ordered target/candidate cursors, `BTreeMap`/`BTreeSet`, sorted worker meshes, stable placed-vector order, checkpoint/replay tests. `HashSet` uses are membership-only on the decision path. |
| One live fill transition opportunity per `drive_fill_job` call | **GREEN** | The production worker calls `precompute_step_lane(Fill, 1)` at precompute `:928`; only one outer lane iteration can execute. |
| One bounded semantic operation per fill grant | **RED** | Several stages do whole work: initial `FillBuilder::new`, full fixture/index rebuild, `construct_preview` scans host/catalog/volumes and all collision-body parts, `Accept::Commit` can upsert 4,096 cells and clones the accepted prefix, preview/checkpoint/complete clone and serialize whole dynamic state. |
| Fixed fill item/byte admission before allocation | **RED** | `FILL_COUNT_MAX=1000` caps accepted placements only. Scene objects, attractions, vortices, catalog rows, IDs, target/candidate working sets, index entries/members, checkpoint descendants, and preview IDs/prefix have no local item/byte preflight. |
| Worker input/checkpoint byte ceiling | **RED** | A 4 MiB ceiling exists, but `fill_worker_checkpoint_bytes` first calls `serde_json::to_vec` and only then tests `bytes.len()` at precompute `:846-848`; it does not prevent allocation/copy and runs on both UI action routes. Restore runs full `serde_json::from_slice`. |
| Exact cancellation/fault owner retirement | **RED** | Cancellation produces `StepOutcome::Cancelled`; no public retained close cursor drains `FillBuilder`, fixture, meshes, candidate/index owners, preview, checkpoint, or terminal result one owner per grant. Ordinary future/session drop remains the terminal owner path. |
| P4b persistent collision phase/RNG cursor | **GREEN** | `CollisionOverlapState` retains broad-phase, part-pair, sample, RNG, inside-count, result, and early-reject state. |
| Collision cancellation/yield checks | **GREEN** | Checked before stage work and inside the sample batch. |
| Collision work bounded independently of mesh cardinality | **RED** | One live fill collision call may run eight samples in a `while`; every sample calls `point_inside_body` for both bodies, which scans every part. `CollisionAabb::from_body`/`world_bounds` also scans all parts. No part cursor or part cap is admitted. |
| Spatial index deterministic exact order | **GREEN** | Ordered maps/sets and binary insertion provide stable order; focused tests assert exact query order. |
| Spatial index narrows the live fill query | **RED** | `CollisionSpatialIndex::query` is test-only. `FillBuilder::query_broad_phase` scans the complete `placed` vector across resumptions and asks `entry_intersects` per entry. It is resumable but still a full scan. |
| Spatial index fixed ownership/one-cell step | **RED** | Entries/cells/member IDs are dynamic. `covered_cells` materializes up to 4,096 cells; `upsert` and `remove` iterate all of them in one call and clone the ID per cell. No global cell/member/id byte cap or rejected-owner handback exists. |
| P4c action does not execute solver transition inline | **GREEN** | Both action variants poll/enqueue and emit `JobPlacement::Isolated`; direct `precompute_step_lane(Fill, ...)` is absent from the action. |
| P4c action is interaction-bounded | **RED** | `enqueue_fill_job` synchronously clones scene/mesh/checkpoint state, sorts meshes, builds `FillWorkerState`, and encodes it before returning the spawn effect. |
| Candidate ghost reaches a real renderer | **GREEN** | Rust window projection passes it as `brushPreviewJson`; `World3dHost` parses it and renders `BrushPreviewGhost`. |
| Broad-phase/pair/collision/rejection overlay reaches renderer | **RED** | The producer nests typed `fillBuildPreview`, but `WorldBrushPreviewRecord` and `BrushPreviewGhost` ignore it. No renderer consumer exists for the extra fields. This is a live placeholder transport, not a rendered overlay. |
| Preview collection cap | **RED** | Collision sample history is manually limited to 32, but uses dynamic `Vec` plus `remove(0)`. Broad-phase IDs, colliding IDs, accepted prefix, strings, and serialized preview bytes are uncapped. |
| Source tests cover checkpoint/cancel/stale/determinism/watchdogs | **GREEN** | Six focused fill tests, ten focused overlap/index tests, worker reopen/ABA, drive-budget replay, actual worker-count parity, first-preview, and action enqueue tests are present. This audit did not execute them. |
| Cap/+1, bytes/+1, saturation, exact handback, terminal-close tests | **RED** | No discriminating planner/index/preview admission or terminal-ownership fixtures were found. Existing adversarial index coverage tests a large span fallback, not global ownership/capacity. |
| Permanent P4 verifier predicate/mutations | **RED** | Root `📜️script.ts` contains no P4/fill/collision structural rule. The broad interactivity verifier is clean but does not reject the whole-encode, dynamic planner/index, full-scan, ignored-overlay, or ordinary-drop paths above. |
| Scoped rustfmt check | **GREEN** | `rustfmt --edition 2021 --check` exited 0 for fill action, precompute bridge, fill builder, collision geometry, Puzzle registration, and main-window projection. |
| Broad interactivity self-test and plain DENY | **GREEN** | Both commands exited 0 and reported `DENY mode — clean`; one mutation finding was counted by the self-test/plain reporting corpus, with no live denial. |
| Current runtime/build matrix | **RED** | Not rerun by instruction. Historical final closure evidence is retained in the prior report, but no current Cargo/Nx/Wasm/browser/runtime result is claimed by this audit. |

## Blocking, Dynamic, Full-scan, and Placeholder Findings

### 1. UI-reachable whole worker-envelope serialization

Both action functions call `enqueue_fill_job` while holding the mutable precompute session. That method calls `fill_worker_checkpoint_bytes`, whose path:

1. clones the request, scene, mesh sources, fill checkpoint, observation, and last checkpoint;
2. collects and sorts all mesh records;
3. `serde_json::to_vec`s the complete state;
4. rejects only after the allocation if the resulting `Vec<u8>` exceeds 4 MiB.

This is not a syscall/blocking-executor bridge, and the production forbidden-executor scan is correctly zero. It is nevertheless synchronous document-size-dependent CPU/allocation on both UI action paths and contradicts the report's “two bounded UI-safe operations” claim.

### 2. Full dynamic checkpoint/progress materialization inside the worker turn

`FillBuilder::checkpoint_bytes` clones every dynamic planner collection before whole JSON encoding. `publish_preview` clones the complete accepted sequence and whole-encodes the preview. `complete` constructs both a full checkpoint and full progress output. `drive_fill_job` can additionally make another checkpoint and clone it into `last_emitted_fill_checkpoint`. These are not retained field/item/page cursors and are not governed by `StepContext` fuel.

### 3. Construction, restoration, and index rebuild are full scans

`FillBuilder::new` scans all base objects, resolves meshes, constructs `placed`, and upserts every placed body. `restore_checkpoint` decodes a complete dynamic checkpoint, rebuilds the fixture with cloned append rows, then calls `rebuild_collision_index`, which clears and reconstructs all placed/index state in one call. The worker performs those operations before its first `JobCtx::tick` grant.

### 4. Spatial index exists but live fill uses a retained full scan

The implementation's deterministic `query` function has zero production callers. Fill broad phase walks every `placed` entry, one entry per fill step, and uses the index only as a map lookup plus AABB intersection. This avoids a monolithic scan but does not deliver the planned spatial narrowing. Oversized index entries also fall back to the full ordered entry set in `query`.

### 5. Narrow phase retains the outer cursor but not inner part scans

Part-pair testing is one pair per `CollisionOverlapState::step`, but sample work is a batch of up to eight. Each sample invokes two `point_inside_body` calls that scan all parts without a cursor/yield/cancellation check. AABB initialization similarly scans all parts. Mesh cardinality is bounded at transport values, not as an admitted collision-part authority, so the per-grant ceiling is not structural.

### 6. Dynamic/fallible collection owners remain pervasive

`FillBuilder` and its checkpoint retain dynamic `Vec`, `BTreeMap`, `BTreeSet`, `HashMap`, and `HashSet` owners for base/working fixtures, placement outputs, targets, weights/Fenwick trees, candidates, IDs, preview, meshes, and spatial state. Collection growth uses ordinary `push`/`insert` and reset uses whole `clear`/replacement. There is no exact rejected owner return or one-owner terminal close.

The few explicit limits are incomplete:

- 1,000 accepted placements;
- 4 MiB encoded worker envelope, checked after allocation;
- 64 meshes, 196,608 values per positions/indices vector, 393,216 aggregate mesh values, 4 KiB URL;
- 4,096 covered cells per index entry;
- 32 visible collision samples, implemented as a dynamic vector with front removal.

No local caps cover scene object/attraction/volume counts, per-object vortex rows, catalogs, compatibility rows, target/candidate work sets, collision parts, IDs/nested strings, index entries/cells/member bytes, broad-phase/colliding ID lists, accepted preview prefix, or simultaneously retained clone bytes.

### 7. Collision/search overlay is a placeholder at the final edge

The Rust producer includes `fillBuildPreview` in the ghost JSON. The renderer's `WorldBrushPreviewRecord` contains only ghost placement/material fields and its component renders only the ghost mesh. No consumer reads broad-phase IDs, current pair, collision IDs/samples, rejection reason, or counters. The candidate ghost is real; the diagnostic overlay promised by P4c is not.

### 8. Verifier coverage is structurally blind to the remaining gaps

The broad verifier rejects known blocking/runtime patterns and is green. Root script search found no `FillBuilder`, fill-job, Puzzle 3D collision/index, or P4 predicate. Reintroducing direct solver stepping may be caught only if it matches a broad token; whole encode/decode, clone-before-cap, full scan, unused spatial query, ignored renderer payload, and missing terminal close currently have no permanent mutations.

## Tests Present but Not Sufficient for Closure

Present source fixtures include:

- fill checkpoint byte identity, cancellation-before-next-transition, typed preview checkpoint, stale-generation no-progress, empty watchdog, and 1,024-entry resumable broad-phase watchdog;
- collision disjoint/coincident/touching/early-reject, 1/7/64 sample-batch determinism, exact checkpoint/RNG resume, cancellation/yield, ordered spatial queries, adversarial cell spans, and a sample watchdog;
- worker cold reopen, cross-operation ABA rejection, checkpoint/resume, drive-budget determinism, actual 1/2/4/default worker parity, first substantive preview, isolated action spawn, and fill/reveal behavior.

Missing discriminators include:

- scene/item/id/nested-byte cap and +1;
- worker-envelope exact preflight and encode page boundary;
- action enqueue with maximum admitted state;
- index entry/cell/member/item/byte cap and +1;
- one-cell upsert/remove/query, occupied/full saturation, exact rejected ID/bounds handback;
- collision body part cap and one-part containment/AABB cursor;
- cancel/fault during envelope construction, checkpoint encode, index update, and terminal cleanup;
- checked-out/drop/close exact ownership and terminal-empty;
- renderer assertions for broad phase, current pair, samples, collision IDs, rejection, stale generation, and payload cap;
- faithful permanent mutations for every item above.

## Smallest File-disjoint Source Packet Next

### P4d — Retained fixed-admission fill-worker envelope

**Choose this before renderer or collision expansion.** It closes the UI-reachable bulk operation without touching currently active P3 renderer files or unrelated Puzzle/UI areas.

Owned files:

- `✏️s/🔌️plugins/🧩️puzzle/🗢️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs`
- the strictly necessary checkpoint-facing region of `⏳️precompute/🪣️fill/🦀️component.rs`
- one distinct permanent P4 predicate/mutation region in the existing root `📜️script.ts`.

Required packet:

1. Replace action-time `fill_worker_checkpoint_bytes` with a generation-keyed retained preparation authority. `fillBuildTick` may only reserve/observe/enqueue a lightweight handle; no full scene/checkpoint/mesh clone or encode on the action call.
2. Define credible fixed item and byte ledgers for scene roots, IDs/strings, meshes/pages, checkpoint fields, preview/progress, and simultaneous source/derived owners. Preflight before ownership transfer; use at most 16 KiB encoded pages.
3. Materialize/encode/decode one field, item, or page per accepted worker grant. Preserve source owner on cap/saturation; no whole `serde_json::{to_vec,from_slice}` on the live route.
4. Keep operation/base-revision/generation validation before page mutation/publication; cancel/stale/fault moves exact retained roots into public terminal ownership.
5. Provide take/resume and one-owner/page close with truthful terminal-empty; no ordinary future/session drop of deep owners.
6. Preserve current action single-flight, isolated job registration, deterministic checkpoint bytes, replay, worker-count parity, and ghost publication.
7. Add cap/+1 and bytes/+1, nested identifier bytes, quiet saturation, cancel/stale/ABA at every cursor phase, exact rejected owner handback, terminal take/resume/drop-handback/close, and clone-before-admission/whole-serde verifier mutations.

This packet intentionally does **not** touch `World3dHost`, other renderer files, collision/index semantics, atlas/icon/glyph/surface/Vello paths, or broader Puzzle actions. A later P4e can own fixed collision/index cursors; a later renderer-disjoint packet can make the diagnostic overlay real.

## Commands and Results

Read-only source census:

```text
rg --files / rg symbol and pattern censuses over the P4 ticket, Puzzle 3D source, root verifier, and World3dHost
result: caller census above; zero production forbidden executor matches after cfg(test) truncation
```

Formatting:

```text
rustfmt --edition 2021 --check <six owned Rust files>
exit: 0
```

Interactivity verifier:

```text
bun 📜️script.ts verify interactivity --self-test --format json
exit: 0; DENY mode clean

bun 📜️script.ts verify interactivity --format json
exit: 0; DENY mode clean
```

No Cargo, Nx, Wasm, browser, runtime, network, root lint, or modifying Git command was run.

## Final Status

Phase 4 remains **RED/open** in source terms. The old inline-solver action and monolithic outer fill/collision loops were materially improved, but the current tree still lacks the fixed admission, page/item cursoring, exact terminal ownership, real spatial narrowing, and renderer-consumed collision overlay needed for a truthful bounded vertical slice. The first bounded follow-up should be P4d above; no Phase 4 acceptance or checklist completion is claimed.
