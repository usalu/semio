# Terra P4e Puzzle3D Constructor, Spatial, Checkpoint, and Preview Packet — 2026-08-24

## Disposition

**GREEN-ready implementation packet.** P4e is not yet accepted: the live Puzzle3D path still performs whole-input planner construction, reconfiguration, collision-index rebuilding, and producer-state cloning synchronously. The previously accepted P4d route remains exclusive and must not be widened or replaced. There are no external blockers and no P4d repair required.

This is a static, read-only census of current source and the existing P4 status/gap reports. No Cargo, Nx, Wasm, browser, runtime, or network command was run.

## Current Production Census

| Area | Current mounted/production residual | Required P4e disposition |
| --- | --- | --- |
| Initial planner construction | `Puzzle3dCollision::rebuild_queue` in `✏️editor/⏳️precompute/🦀️component.rs` allocates an identity, enumerates every fixture target, clones catalogs and fixture, constructs `FillBuilder::new`, then calls `configure` with whole config roots. | Replace with a generation-owned, resumable preparation job; bounded work per worker grant. |
| Soft replan and refresh | `soft_replan_fill_tail` resets fixture, calls `rebuild_collision_index`, and clears a rebuild map. `refresh_fill_job` clones scene/meshes, calls `refresh_meshes` or restart, then `configure`. | Use the same resumable preparation/reconfiguration state machine; no synchronous alternate route. |
| Builder construction | `FillBuilder::new` in `🪣️fill/🦀️component.rs` clones the fixture/catalogs/mesh bodies, scans all base objects, materializes all collision entries, calls spatial `upsert` per entry, and builds lookup state. | Convert to zero/small seed plus cursorized construction phases. |
| Rebuild | `refresh_meshes` clones every mesh body then invokes `rebuild_collision_index`; that method clears index/lookup and scans all fixture objects while upserting them. | Cursorize mesh copying, entry bounds, index insertion, and lookup installation. |
| Producer clones | `progress` clones appended objects, attractions, sequence, and preview. `publish_preview` clones the full accepted prefix. Commit also clones `self.sequence`. | Publish bounded diagnostic/ghost projection only; remove whole producer-state cloning. |
| Checkpoint codec | `checkpoint_bytes`, `restore_checkpoint`, and `restore_checkpoint_for_fixture` serialize/replace full `FillBuilder` state through `FillJobCheckpoint`; `restore_checkpoint` rebuilds the index. Current live P4d persistence returns/stores a registry token and has no direct production caller of this codec. | Delete the dormant codec and its JSON/serde restoration path. P4d registry-token persistence remains the only live resume mechanism. |
| Spatial index | `CollisionSpatialIndex` uses dynamic `Vec<String>` buckets. `new`/rebuild/commit directly call `upsert`; `upsert` calls `remove` and allocates/scans covered-cell vectors. | Replace with bounded fixed storage and resumable insert/remove/query operations. |
| Narrowing | `CollisionSpatialIndex::query` is unused in production. Live preview broad phase instead scans `self.placed` and calls `entry_intersects`; `covered_cells` materializes a `Vec`. | Make production collision/preview use the resumed spatial query and prove narrowing. |
| Renderer diagnostics | `world_fill_preview_json` returns `None` when there is no ghost, serializes a ghost and a nested `fillBuildPreview`; `World3dHost` parses only ghost fields and renders only `BrushPreviewGhost`. | Publish and render an explicit bounded fill diagnostic overlay independently of ghost availability. |

## Exact Residual Sites

### Constructor/configure/rebuild/checkpoint/restore

- `✏️editor/⏳️precompute/🦀️component.rs`
  - `Puzzle3dCollision::rebuild_queue`: synchronous all-target enumeration, `FillBuilder::new(scene.fixture.clone(), ...)`, and `fill.configure(...)`.
  - `soft_replan_fill_tail`: fixture reset, `rebuild_collision_index()`, and `candidate_cache.clear_for_rebuild_residual()`.
  - `refresh_fill_job`: whole scene/mesh cloning, `refresh_meshes`/restart, and `configure`.
- `✏️editor/⏳️precompute/🪣️fill/🦀️component.rs`
  - `FillBuilder::new`, `configure`, `refresh_meshes`, `rebuild_collision_index`.
  - `checkpoint_bytes`, `restore_checkpoint`, `restore_checkpoint_for_fixture`, and `FillJobCheckpoint`.
  - `progress`, `publish_preview`, and commit-time `accepted_prefix = self.sequence.clone()`.
- `✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`
  - `FixedOwnerMap::clear_for_rebuild_residual`, `FixedOwnerMap::cloned_btree`, and `FixedOwnerSet::cloned_btree`.
  - `CollisionSpatialIndex::{upsert,remove,query,covered_cells}`.

### Spatial caller split

| Symbol | Live caller status | P4e action |
| --- | --- | --- |
| `CollisionSpatialIndex::upsert` | Production: builder construction, `rebuild_collision_index`, accepted commit. | Only call via an owner/cursor turn. |
| `CollisionSpatialIndex::remove` | Internal to `upsert`; no independent production caller. | Fold into resumable replacement operation. |
| `CollisionSpatialIndex::query` | Tests only. | Promote to the sole production broad-phase narrowing API. |
| `CollisionSpatialIndex::entry_intersects` | Production: preview broad-phase scan over all `self.placed`. | Remove from production broad-phase route; retain only if bounded helper proof needs it. |
| `CollisionSpatialIndex::covered_cells` | Used by index methods and returns an unbounded materialized vector (cell-cap bounded but non-resumable). | Replace with a fixed span plus cell cursor. |

The geometry collision sampler also computes body bounds and point-inside checks by scanning body parts. Its entry/bounds and part cursors must be included in the same owner job; spatial-index completion alone is insufficient.

## Smallest Sol-High Implementation Packet

### 1. One generation-owned cooperative preparation path

Change `✏️editor/⏳️precompute/🦀️component.rs` and `✏️editor/⏳️precompute/🪣️fill/🦀️component.rs`.

Create a preparation/reconfiguration state held by the admitted fill owner. It starts from immutable scene/catalog/mesh roots and uses explicit cursors for target enumeration, mesh/body copy or immutable resolution, base-object collision-entry bounds, spatial installation, and lookup installation. A grant performs one bounded unit, persists its exact cursor, and returns. `rebuild_queue`, soft replan, and refresh only enqueue/advance that state; none may construct or configure a complete builder synchronously.

The builder’s executable search stages may begin only after the preparation state reports complete. Supersession, cancellation, failure, and close must use the existing P4d single-owner/registry lifecycle: a replaced generation may not mutate a newer owner and a failed preparation remains reclaimable/closeable.

**Acceptance predicates**

- A fixture/mesh/catalog at the cap and cap plus one needs respectively bounded turns and permanent, attributable refusal; neither blocks a frame/worker grant.
- Initial, soft-replan, and refresh paths all traverse the same resumable owner phases.
- A generation change during any preparation cursor stops old mutation before it can install a new generation’s entries.
- No mounted path directly invokes whole `FillBuilder::new`, `configure`, `refresh_meshes`, or `rebuild_collision_index`.

### 2. Make the live spatial index bounded and actually queried

Change `✏️editor/⏳️precompute/📐️geometry/🦀️component.rs` and the fill call sites.

Replace per-cell `Vec<String>` membership with fixed, explicitly capped owner storage (or a fixed-page equivalent owned wholly by the job). Replace the materialized covered-cell vector with a validated fixed cell span and cursor. Give replacement/upsert, removal, and query begin/step/finalize operations explicit owner identity and persisted cursors. Preflight all fixed capacity before mutation so capacity failure leaves the prior index exact and leaves the owner faulted/retryable according to its phase contract.

Route production preview/collision broad phase through `query`/its stepped successor. Do not scan all `self.placed` with `entry_intersects`. Return a bounded candidate page with truncation/continuation diagnostic rather than an unbounded list. Cursorize body bounds/point-inside part scans in the geometry collision state as well.

**Acceptance predicates**

- Sparse fixtures prove only intersecting occupied cells/members are examined; a distant `self.placed` population is not linearly visited.
- One-cell, multi-cell, oversized, removal, replacement, capacity-edge, and cap-plus-one fixtures each preserve exact old state on refusal and make forward progress on acceptance.
- No dynamic spatial bucket, `Vec` cell coverage, direct `upsert`/`remove`, or production full-placed scan survives.
- A stale/cancelled owner cannot finish a partial removal or insertion after the current generation advances.

### 3. Delete the dormant whole checkpoint and clone escape hatches

Change `✏️editor/⏳️precompute/🪣️fill/🦀️component.rs` and `✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`.

Remove `FillJobCheckpoint`, `checkpoint_bytes`, `restore_checkpoint`, `restore_checkpoint_for_fixture`, the JSON/serde whole-state route, `cloned_btree`, and `clear_for_rebuild_residual`. Do not retain a compatibility adapter: the only live restore is the P4d registry token returned from `Puzzle3dCollision::fill_checkpoint_bytes` and restored by `restore_persisted_fill`.

Replace `FillBuilder::progress` and preview publication with a small schema-defined diagnostic snapshot: stage, generation/token-safe identity, completed/total bounded counters, current target/candidate reference, collision/rejection summary, bounded sample/page, truncation flag, and optional ghost transform. It must not clone sequences, accepted prefixes, fixture objects, attractions, or arbitrary collision-id collections.

**Acceptance predicates**

- Search source has no `FillJobCheckpoint`, fill JSON checkpoint/restore API, `cloned_btree`, `try_from_btree`, or rebuild-clear residual.
- Process/page zero during every preparation/search/close checkpoint requires no serialised builder state; token restore resumes once through P4d only.
- A preview at cap and cap plus one publishes fixed-size diagnostics with truncation and never clones the producer collection.

### 4. Complete the preview-to-renderer diagnostic contract

Change the canonical fill preview schema in `🧬schema/🦀️component.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs`, and `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx`.

Make the transport emit the bounded fill diagnostic whenever the admitted fill is active, including rejection/no-candidate states. Ghost rendering remains conditional on an optional ghost field; diagnostic rendering is independent. Teach `World3dHost` to parse the nested fill diagnostic and render a compact, renderer-consumed overlay for stage/progress, current target/candidate, collision/rejection, and truncation. It must discard absent/stale identity and never infer diagnostic state from a ghost alone.

**Acceptance predicates**

- Candidate ghost state renders the ghost and matching diagnostic.
- Rejected/no-ghost state still renders its rejection diagnostic.
- Terminal, cancelled, replaced, malformed, or stale-token payload renders neither stale ghost nor stale overlay.
- Renderer source consumes each canonical diagnostic field it claims to transport; no hidden `fillBuildPreview` payload remains ignored.

### 5. Permanently enforce P4e without widening P4d

Change root `📜️script.ts`, extending the existing interactivity verifier and its self-test fixtures.

Add P4e source predicates and faithful mutations for: direct mounted whole builder/configure/rebuild calls; full checkpoint codec/restore and clone helpers; dynamic spatial buckets/materialized coverage/direct spatial mutation; unused query plus production `self.placed` scan; producer-state preview clones; ghost-gated diagnostic transport; and ignored renderer diagnostics. Retain the R7 single-worker exclusivity and R8 lost-handle partial-Closing predicates unchanged. The verifier must prove the scope boundary: P4e may not reintroduce a whole payload worker route or touch P4d registry ownership semantics.

## Required Mutation Matrix

| Fixture/mutation | Must fail the verifier or acceptance harness because |
| --- | --- |
| Restore `FillBuilder::new(...fixture.clone...)` or direct `configure` in `rebuild_queue` | Mounted whole-input construction/reconfiguration returned. |
| Reintroduce direct `rebuild_collision_index` from soft replan/refresh | Rebuild bypasses owner turns. |
| Reintroduce `FillJobCheckpoint` JSON bytes/restore | Whole-state persistence escaped the registry token lifecycle. |
| Reintroduce `cloned_btree` or rebuild clear | Whole copy/clear rebuild residual returned. |
| Replace fixed bucket/page with `Vec<String>` or `covered_cells -> Vec` | Spatial work/allocation became unbounded per call. |
| Keep `CollisionSpatialIndex::query` unused and scan `self.placed` | Spatial narrowing is decorative rather than production. |
| Reintroduce direct `upsert` at construction/commit | Index mutation is no longer one-owner resumable work. |
| Clone `sequence`/accepted prefix from `progress` or `publish_preview` | Preview publication copies producer state. |
| Return `None` without a ghost or omit renderer overlay consumption | Rejection/no-candidate diagnostics are invisible. |
| Render a supplied stale diagnostic after supersession | Generation identity is not enforced at the renderer edge. |
| Cap and cap-plus-one constructor/index/query/preview fixtures | Allocation/exhaustion lacks permanent, exact refusal and forward progress proof. |

## Handoff Boundary

Implement P4e in the order above: cooperative preparation first, spatial representation/query second, codec/clone removal and canonical bounded preview third, renderer/verification last. Do not start P5b and do not fold renderer cosmetics or unrelated collision algorithm changes into this packet.
