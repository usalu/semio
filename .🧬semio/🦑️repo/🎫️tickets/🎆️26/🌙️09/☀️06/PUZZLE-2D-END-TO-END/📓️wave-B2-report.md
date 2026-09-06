# Wave B2 — hard Works (`forceLayout`, `redrawHandles`, the fill-session family)

Ticket `26/09/06/PUZZLE-2D-END-TO-END`. All line numbers are as of the end of this wave; the editor
file was being edited concurrently by B1 throughout, so re-grep before acting on any of them.

**Nothing in this report was compiled.** The main session owns cargo; every claim below is derived
from reading the traits and the pre-existing code that the new code mirrors, plus `rustfmt --check`
(clean on every file I touched, with the repo's own `rustfmt.toml`). §7 lists exactly what must be
compile-verified.

---

## 0. Headline

| Item | Before | After |
|---|---|---|
| `forceLayout` on Nakagin (180 nodes) | `extent() == None` → preflight fault, action structurally dead | admitted, **1,112 declared steps** = the steps actually taken |
| `forceLayout` declared vs real work | declared ≤ 576, real ~846,720 `step()` calls at the permitted 64-node max (**×1,470**) | declared == taken, worst case over the whole admissible space **3,388 ≤ 4,096** |
| `PUZZLE2D_FORCE_MAX_HANDLES` | enforced only mid-run (`Fault` at the `Handles` stage) | checked in `extent()`; a wide document is refused at preflight |
| `PUZZLE2D_FORCE_ITERATIONS` | fixed 420 regardless of size | size-derived (`420` for small graphs, **122** for Nakagin, `24` floor at 512 nodes) |
| `redrawHandles` | one unbounded whole-fixture `apply_edge_handle_snap_to_fixture_v1_json` call, no size guard, no yield | `Puzzle2dRedrawHandlesWork`, 3 chunked passes, extent **10** on Nakagin / **100** at the ceiling |
| fill family (8 ids) | process-global `AtomicPtr` slot registry + `WorkerPool` + `MountedWorkerJobSession` + store lease + self-dispatched `Effect::DispatchAction` ping-pong; **unreachable from a retained work** | `Puzzle2dFillSessionWork`, session owned by the work, extent **1,028** for a search verb / **4** for a control verb |
| `set-fill-count/🦀️.rs` | 2,401 lines | 1,946 lines, registry-free |

---

## 1. `forceLayout` (task 1)

### 1.1 Why it was dead, measured

`Puzzle2dForceLayoutWork::extent()` gated `nodes ≤ PUZZLE2D_FORCE_MAX_NODES = 64`. Nakagin
(`📚️examples/🏗️nakagin-capsule-tower`) is 180 nodes / 179 edges / 358 handles, so `extent()` returned
`None` and `🎮️commands/🧵️retained/🦀️.rs:545` faulted the job with
`"puzzle command exceeds fixed semantic work capacity"`. The action was dead on the only non-trivial
example the artifact ships.

The second, quieter problem: even at the *permitted* 64 nodes the declared extent was
`nodes + edges ≤ 576` while `step()` ran `PUZZLE2D_FORCE_ITERATIONS = 420` iterations of
`n(n−1)/2 = 2,016` repulsion pairs plus a springs pass — **one `step()` call per pair**, i.e. ~846,720
retained steps, each one a checkpoint boundary. Nothing faults on that (the `Work` phase at
`🧵️retained/🦀️.rs:555-574` never checks `work_cursor ≤ work_extent`), so the miscount was silent.

### 1.2 What I changed

Editor `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

- **Constants + two helpers** (`const PUZZLE2D_FORCE_MAX_NODES` … `fn puzzle2d_force_iterations`).
  Ceilings raised to `MAX_NODES = 512`, `MAX_EDGES = 4_096`, `MAX_HANDLES = 4_096`. `512` was not
  invented here — the retained-jobs fixture already carried `forceLayoutMax nodes:512` /
  `forceLayoutMaxPlusOne nodes:513` vectors, so the fixture and the code now agree.
- **Iteration schedule.** `puzzle2d_force_iterations(nodes, edges)` =
  `clamp(PUZZLE2D_FORCE_WORK_BUDGET / (pairs + edges), 24, 420)` with `WORK_BUDGET = 2_000_000`.
  Rule, documented in the source: *a spring embedder's cost per iteration is `n(n−1)/2` repulsion
  pairs plus one pass per edge, so hold total force work constant instead of iteration count.*
  Derived from the **raw** document counts (not the visible subset) so `extent()` and `step()` can
  never disagree — visibility filtering only makes the real run shorter than the declared budget.
  **Answer to the brief's question: no, 420 iterations on 180 nodes is not acceptable** — it is
  6.84 M force evaluations. Nakagin now gets 122 iterations ≈ 1.99 M evaluations.
- **Chunking.** Every stage body became a `*_one` helper on the inherent impl (`scan_node_one`,
  `scan_handle_one`, `scan_edge_one`, `seed_one`, `center_one`, `reset_one`, `repel_one`,
  `spring_one`, `integrate_one`, `emit_one`, plus `cool`), and `step()` drains one chunk per call:
  `PUZZLE2D_FORCE_UNITS_PER_STEP = 8_192` force units, `PUZZLE2D_FORCE_NODES_PER_STEP = 512` node
  units, `PUZZLE2D_FORCE_SCAN_PER_STEP = 256` scan units. 8,192 pair evaluations is a few hundred µs
  against the 7,500 µs `PUZZLE_COMMAND_STEP_MICROS`.
- **`extent()`** now counts those chunks:
  `ceil((N+H+E)/256) + 3·ceil(N/512) + 8 + iterations·(ceil(pairs/8192) + ceil(E/8192) + 2·ceil(N/512) + 4)`,
  and refuses outright when `N > MAX_NODES ∨ E > MAX_EDGES ∨ H > MAX_HANDLES`. **`MAX_HANDLES` is now
  a preflight gate**, closing the "≤64 nodes but >512 handles faults mid-run" hole the shared-infra
  exploration flagged.
- **`tool_id` is now a field** with `Puzzle2dForceLayoutWork::new(tool_id)` (and `Default` = `"forceLayout"`),
  because `reorganize` runs the same simulation and the retained job's decode phase rejects a payload
  whose `action_id()` ≠ `work.tool_id()`. B1 already uses `Puzzle2dForceLayoutWork::new("reorganize")`.

Stage names are unchanged (`Nodes`/`Handles`/`Edges`/`Seed`/`Center`/`Reset`/`Repel`/`Springs`/
`Integrate`/`Emit`/`Complete`/`Closing`), so the fixture's `semanticCursors.forceLayout` and its four
`Puzzle2dForceStage::*` hostile mutations still hold verbatim.

### 1.3 Extent table (computed, `🗑️generated/b2` script)

| case | N | E | H | iterations | `forceLayout` extent | `redrawHandles` extent |
|---|---|---|---|---|---|---|
| **nakagin-capsule-tower** | 180 | 179 | 358 | 122 | **1,112** | **10** |
| **concrete-forest** | 1 | 0 | 11 | 1 | **18** | **6** |
| empty board (boot default) | 0 | 0 | 0 | 1 | 17 | 4 |
| force ceiling | 512 | 4,096 | 4,096 | 24 | 597 | 54 |
| old permitted max | 64 | 512 | 512 | 420 | 3,376 | 11 |
| dense small graph | 98 | 4,096 | 4,096 | 226 | 1,852 | 53 |
| redraw ceiling | 4,096 | 4,096 | 8,192 | — | refused (N > 512) | 100 |

Worst case over the entire admissible space (swept N ∈ [0,512] × E ∈ {0,1,N,2N,1024,4096}):
**3,388** at `N=2, E=4096` — under `PUZZLE_COMMAND_WORK_ITEMS = 4_096`, and anything that would
exceed it returns `None` (fail-closed) rather than lying.

---

## 2. `redrawHandles` (task 2)

`🎮️commands/🔄️redraw-handles/🦀️.rs` called
`crate::editor::puzzle2d::engine::apply_edge_handle_snap_to_fixture_v1_json(&ctx.scene.fixture.to_string())`
— it serialised the whole fixture to a `String`, re-parsed it, walked every node/handle/edge, and
re-serialised. No size guard, no yield point.

New `Puzzle2dRedrawHandlesWork` (editor `🦀️.rs`, immediately after the force-layout work), three
chunked passes at `PUZZLE2D_REDRAW_UNITS_PER_STEP = 256` units per step:

1. `Nodes`/`Handles` — index each visible handle id → `(node index, handle index)` and each visible
   node → its snap geometry (`Puzzle2dRedrawShape`, mirroring the engine's private `NodeShapeSnap`:
   a node with no finite centre, a circle with no radius, or a rectangle with no extents has no rim
   and is skipped exactly as the engine skips it).
2. `Edges` — per visible edge, resolve both endpoint angles into a `BTreeMap<(usize,usize), f64>`;
   **last edge wins on a shared handle**, which is what the engine's `angle_by_loc` map does.
3. `Emit` — drain the map in deterministic key order, parse the original `Puzzle2dHandle`, set
   `angle`, and publish `replace_node_handle` only when the handle actually changed. That is
   precisely the delta `puzzle2d_snapshot_mutations` derived from the legacy whole-fixture rewrite
   (`🧬️schema/🧬️mutations/🦀️.rs:176-181`), so the two paths agree value-for-value.

The angle math is **not** reimplemented: it calls the framework's own
`circle_handle_angle_toward` / `rectangle_handle_angle_toward` / `distance_between` /
`board_json_visible_or_true` / `fixture_edge_handle_ids_from_object`, all reachable through
`crate::editor::puzzle2d::engine::*` (the same re-export path `🔗️linking/🦀️.rs:16` already uses for
`distance_between`/`handle_position_on_circle`).

Ceilings: `MAX_NODES 4_096`, `MAX_EDGES 4_096`, `MAX_HANDLES 8_192`; a `retained_bytes` accumulator
keeps indexed ids under `PUZZLE_COMMAND_OUTPUT_BYTES`. Extent = `ceil((N+H)/256) + ceil(E/256) +
ceil(H/256) + 4` — the brief asked for "edges (capped)"; I count all three passes because the index
and publish passes are the ones that actually dominate (Nakagin: 3 + 1 + 2 + 4 = 10).

**Dependency**: the emitted verb is `replace-node-handle`, whose diff has the no-op-guard bug the
master plan assigns to **wave A1** (`🧬️mutations/🔌replace-node-handle/🔺️diff/🦀️.rs:13-21`). Until A1
lands, `redrawHandles` will publish correct mutations that apply as no-ops. This is the same bug the
legacy path had (it produced the same mutation kind through the delta), so it is not a regression —
but `redrawHandles` cannot be *runtime-verified* before A1.

---

## 3. Fill-session family, 8 ids (task 3)

### 3.1 The bridging question, answered

**No — `Puzzle2dConfigMutation::Fill { runtime: Puzzle2dFillRuntime }` does *not* give 2d the
property that made puzzle3d's `fillBuildTick` migratable, and this is the decisive finding.**

- Puzzle3d's `Puzzle3dConfig` carries `pub fill_checkpoint: Vec<u8>`
  (`🧊️3d/…/✏️editor/🎚️config/🦀️.rs:155`) — the **serialized precompute state**. The checkpoint in
  `Config` *is* the session, which is why `with_puzzle3d_app_for(config, …)` can rebuild a fresh
  `Puzzle3dPlayApp::default()` on every call and lose nothing.
- Puzzle2d's `Puzzle2dFillRuntime` (`◻️2d/…/✏️editor/🎚️config/🦀️.rs:129-141`) carries only scalars:
  8 counters, a lifecycle enum, and two 64-byte fixed texts. It is **telemetry about** a session,
  not the session. The session itself lived in `static FILL_SESSION_SLOTS: [FillRegistrySlot; 8]`
  (`AtomicPtr<FillSessionNode>`), holding a `MountedWorkerJobSession<ArtifactBoardFillJob>`, a
  `BoardFillCheckpoint`, and a `store::SnapshotRead` lease.
- The registry key was `(app_instance_id, operation_id, generation)`, and `app_instance_id` comes
  only from `AppOperationContext`, i.e. only from an `ArtifactView`. **A `PuzzleCommandWork` never
  sees an `ArtifactView`** — `puzzle2d_board_events_reduce` sets `operation: None` explicitly. So
  every fill verb routed through a retained work with the old code would have hit
  `action_authority(ctx) == None` and returned `puzzle2d-fill-operation-authority` (or silently
  no-op'd for `step`/`adopt`/`cancel`/`discard`). **A half-migration here is not merely dishonest,
  it is guaranteed-broken**, which is why the registry had to go rather than being wrapped.
- Serializing `BoardFillJobState` into a config field (the 3d shape) is not available either: it is
  a private struct of ~35 fields including several `BoardFillFixedPages<…>` in
  `🧰️framework/…/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:873-911`, with `Drop` assertions
  and no encode surface. That would be a framework change in a module shared with other plugins.

### 3.2 What replaced it

The retained job **already owns a live object across steps**: `RetainedPuzzleCommandJob<A>` holds
`work: Box<dyn PuzzleCommandWork<A>>` for the whole job, checkpointed by the shared
`PuzzleCommandCheckpointState`, and it calls `payload.work.bind_operation(operation)`
(`🧵️retained/🦀️.rs:374`). That is the correct home for the session. So:

`Puzzle2dFillSessionWork` (`🎮️commands/🧮️set-fill-count/🦀️.rs`, region `//#region 🪣️Session`) owns
the capture cursor + `BoardFillSnapshotIngress`, the `infinite_canvas::BoardFillJob`, the
`BoardFillCheckpoint`, the `FillPlacementApplyCursor`, and the retained `StepOutcome`, and drives
them itself. Stages:

| stage | what one `step()` call does | chunk / ceiling |
|---|---|---|
| `Control` | applies the verb's runtime transition via the single shared `fill_session_control` | `PUZZLE2D_FILL_CONTROL_CHUNKS = 4` |
| `Capture` | streams the document into the fixed ingress, one field (one byte for text) at a time | `2_048` units × `256` chunks |
| `Search` | `BoardFillJob::step` under a locally built `StepContext` | `512` units × `512` chunks |
| `Apply` | drains one checkpoint's pending placement into `Puzzle2dMutation`s, then hands the checkpoint back to the search | `4_096` units × `256` chunks |

The bridge into the engine job is
`semio_framework_job::StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), puzzle2d_fill_monotonic_zero, &mut self.preview_sequence)` —
all of which are public. Two deliberate choices:

- **fuel = 1**, matching the `BatchDriveConfig { fuel_per_step: 1, … }` the removed
  `MountedWorkerJobSession` used, so the search granularity is unchanged.
- **no wall clock** (`deadline_us = u64::MAX`, `now_us` returns `Some(0)`). Two reasons: the retained
  job already enforces the real 7,500 µs deadline before it calls `work.step()`, and the retained
  checkpoint **replay** (`restore_target` / `restore_has_passed_target`,
  `🧵️retained/🦀️.rs:600-612`) requires the work to be deterministic — a wall-clock-dependent inner
  budget would make a replay diverge and fault with
  `"puzzle command checkpoint replay diverged"`.

Cancellation is the framework's: `step_inner` checks `cx.is_cancelled()` before every `work.step()`,
and `begin_close`/`close_step`/`terminal_is_empty` release every engine owner one at a time
(checkpoint handed back to the search first, then the search job, then the ingress — the order the
engine's own `Drop` assertions require). Progress is `PuzzleCommandWorkStep::Progress` per chunk with
EN/DE labels.

**Seed determinism**: `begin_search` rewrites `self.operation` to
`Operation::new(same op, same revision, same generation, requested seed)` so the requested seed
reaches `BoardFillJob::with_operation` while the operation/generation the job validates stay
identical.

### 3.3 The eight verbs, as explicit stages

| verb | meaning now | lanes actually emitted |
|---|---|---|
| `setFillCount` | set the count, activate the fill tool (`Effect::SetActiveTool`), run a fill | `Artifact` + `Config` |
| `brushFillSessionBegin` | run a fill at an explicit `maxCount`/`seed` | `Artifact` + `Config` |
| `brushFillSessionRetry` | re-run with the stored count and seed | `Artifact` + `Config` |
| `brushFillSessionStep` | **real resumption**: run the *remaining* count (`fill_count − accepted`) with the stored seed when the lifecycle is resumable; otherwise settle the lifecycle and publish | `Artifact` + `Config` |
| `brushFillSessionAdopt` | lifecycle → `Completed` (or `Faulted` if a code is carried) | **`Config` only** |
| `brushFillSessionCancel` | lifecycle → `Cancelled`, clear the fault | **`Config` only** |
| `brushFillSessionDiscard` | zero the job counters, lifecycle → `Discarded`, keep the count | **`Config` only** |
| `brushFillSessionClear` | discard **and** zero `fill_count` | **`Config` only** |

`brushFillSessionStep` is not a poll any more: the placements a previous session accepted are already
committed to the document, so *remaining count + stored seed* is the complete continuation state.
That is the checkpoint-carried state the brief asked for — `Puzzle2dFillRuntime` **plus the document**.

A budget exhaustion in `Search` is **not** a fault: the work publishes what it has with lifecycle
`CheckpointReady`, and `brushFillSessionStep` continues. `Capture`/`Apply` exhaustion faults
(`puzzle2d-fill-capture-budget` / `-apply-budget`) because a partially captured document or a
half-applied placement is not a resumable state. A fault is published as a `Faulted` lifecycle with
its code, not as a job fault, so the panel can still offer retry/discard.

### 3.4 What was deleted

From `🧮️set-fill-count/🦀️.rs`: `FILL_SESSION_CAPACITY`, `FILL_SESSION_LOCKED`, `FillSessionNode`
(+ `Drop`), `FillSessionBacking` (+ `Drop`, the raw `alloc`/`MaybeUninit` dance), `FillRegistrySlot`,
`static FILL_SESSION_SLOTS`, `FillSessionGuard`, `FillSessionReservation`, `reserve_session_slot`,
`publish_session`, `take_matching_session`, `take_snapshot_pending_session`,
`pump_abandoned_session`, `registry_has_sessions`, `fill_worker_pool`, `FillWork`,
`ArtifactBoardFillJob` (+ its `InteractiveJob`/`Drop` impls — its capture body survives as
`Puzzle2dFillSessionWork`'s `//#region 🔬️Capture` methods, retargeted from
`store::SnapshotRead<Puzzle2dPlaySnapshot>` to a borrowed `&Value`), `prepare_snapshot_read`,
`reconcile_snapshot_read`, `base_revision`, `action_authority`, `is_fresh`, `fill_action_effect`,
`queue_fill_action`/`_step`/`_adopt`/`_discard`, `close_session_work_one`, `apply_checkpoint_step`,
`pump_fill_worker`, `begin_fill_job`, `step_fill_job`, `adopt_fill_job`, `cancel_fill_job`,
`discard_fill_job`, `retry_fill_job`, `set_fill_count`, `reject_fill_request`,
`Puzzle2dFillActionCtx`, and the `use std::sync::atomic::{AtomicPtr, Ordering}`.

Kept verbatim (they were correct and are still the only fixed-capacity path into/out of the engine):
the capture stage/field enums and `ArtifactFillCaptureCursor`, every `capture_*_one`, the whole
placement machinery (`FillPlacementApplyCursor`, the three fixed owners, `copy_fill_text_one`,
`FillPlacementPublishView`, `publish_fixed_placement`, `publish_commit_candidate`) and
`capture_fault_code`.

### 3.5 `handle()` and `setActiveUtility`

All eight ids are `Migrated`, so `validate_ui_dispatch_classification` routes them to the tool job and
`ArtifactApp::handle` is unreachable for them in production. Per the brief's "either both call one
shared fn or the legacy branch is removed", **the branch is removed**: `handle()` now reads

```rust
if set_fill_count::is_fill_session_action(action) {
    return Err(Fault::from("puzzle2d-fill-requires-retained-job"));
}
```

with `is_fill_session_action`/`dispatch_fill_session_action` deleted from the editor, and the two
`ArtifactEditor` overrides the removed store lease needed (`mounted_job_prepare_snapshot_read`,
`pending_effects`) dropped back to their trait defaults
(`🧰️framework/…/🔌️plugin/🦀️.rs:26663` and `:26875`).

`setActiveUtility`'s inline coupling still works and got simpler: `🎮️commands/🧰️set-active-utility/🦀️.rs`
now calls `set_fill_count::discard_fill_session(&mut fill_runtime)` — abandoning a fill *is* discarding
the runtime, because the runtime is the only thing a later verb resumes from. It no longer needs an
`AppOperationContext`, a generation, or a boundary-fault channel.

The three fill command leaves keep real content rather than becoming dead taxonomy nodes:
`🏁️fill-session-begin` owns the `maxCount`/`seed` admission, `👣️fill-session-step` owns the resumption
rule, `🧹️fill-session-clear` owns discard-and-zero; `fill_session_control` dispatches into them.

---

## 4. Hostile vectors (task 4)

`◻️2d/…/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json` — appended only (the oracle at
`🧵️retained/🦀️.rs:940` asserts `vector_ids.starts_with(VECTOR_IDS)` and compares the first 16
fingerprints, so ordering is preserved):

- `evidenceToolIds` +10: `reorganize`, `redrawHandles` and the eight fill ids (the oracle requires
  every `toolId`/`toolIds` in a vector to be an evidence id).
- `semanticCursors` +10 routes: `reorganize` (same cursor list as `forceLayout`), `redrawHandles`
  (`node`/`handle`/`edge`/`handlePublish`/`closeOwner`), and the fill verbs
  (`control`/`capture`/`search`/`apply`/`publish`/`closeOwner`, trimmed to `control`/`publish`/
  `closeOwner` for the four control-only verbs).
- `hostileSourceMutations` 9 → 26: un-chunking the repulsion loop, restoring a fixed iteration count,
  removing the handle gate from `extent`, lowering `MAX_NODES` back to 64, replacing the redraw work
  with `BoundedFirstStepCommandWork`, removing each redraw cursor, removing the redraw handle
  ceiling, reviving the slot registry / the worker pool / the store lease, removing the fill capture
  and apply cursors, removing the search budget guard, reviving the `handle()` dispatch branch, and
  swapping the deterministic inner clock for `default_now_us`.
- `vectors` 32 → 72: `forceLayoutNakagin`, handle/edge ceilings, iteration-scaling replay determinism,
  `reorganize` sharing the force work + a wrong-tool-authority probe; `redrawHandles` zero /
  malformed / Nakagin / max / max+1 / shared-handle-last-edge-wins / hidden-node-skipped /
  stale-generation / cancel-at-every-boundary / fault-at-every-boundary; `setFillCount` zero /
  missing-argument / malformed / max / max+1 / non-finite / negative / stale-generation /
  cancel-and-fault at every boundary / replay determinism; `brushFillSessionBegin` missing-seed /
  missing-count / max+1 / seed determinism; `brushFillSessionStep` without-session / resumes-remainder /
  stale-generation; control-only probes for adopt/cancel/discard/clear; a `brushFillSessionClear`
  malformed probe; and a three-verb `fillCohortStaleRetryCloseReplay`.

Validated mechanically: the file parses, all 72 vector ids are unique, every vector carries
`expected`, and every vector satisfies the oracle's `control | authority | closeGrant | toolId∈evidence
| toolIds⊆evidence` predicate.

---

## 5. Every edit site

| file | region |
|---|---|
| `◻️2d/…/✏️editor/🦀️.rs` | force-layout constants + `puzzle2d_force_pairs`/`puzzle2d_force_iterations`; `Puzzle2dForceLayoutWork` `tool_id` field, `new()`, the ten `*_one` helpers + `cool`, and its `extent`/`step`; the whole new `Puzzle2dRedrawHandlesWork` block before `//#endregion 🧵️RetainedCommands`; deletion of `is_fill_session_action`/`dispatch_fill_session_action`; deletion of `mounted_job_prepare_snapshot_read`/`pending_effects`; `handle()`'s fill branch; two `build_tool_job` arms appended at the bottom of the match; `use …commands::{…}` (dropped 3 now-unused leaves); test `mounted_fill_dispatch_contract` → `fill_session_retained_only_contract` + `mounted_fill_dispatch_revivals_are_rejected`; removed the two fill entries from `view_actions_emit_no_ops_through_the_registry` |
| `…/🎮️commands/🧮️set-fill-count/🦀️.rs` | rewritten (2,401 → 1,946): registry/pool/lease removed, `Puzzle2dFillSessionWork` + `fill_session_control` + `discard_fill_session` added, source-contract tests rewritten |
| `…/🎮️commands/🏁️fill-session-begin/🦀️.rs` | rewritten — owns `maxCount`/`seed` admission |
| `…/🎮️commands/👣️fill-session-step/🦀️.rs` | rewritten — owns the resumption rule |
| `…/🎮️commands/🧹️fill-session-clear/🦀️.rs` | rewritten — discard + zero count |
| `…/🎮️commands/🧰️set-active-utility/🦀️.rs` | rewritten — runtime discard, no ctx/operation/generation |
| `…/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json` | evidence ids, cursors, hostile mutations, vectors |

`…/✳️any/🎚️config/🦀️.rs` was **not** touched: the design needs no new config field (§3.1 explains why
adding a `fill_checkpoint: Vec<u8>` would not have worked anyway).

`rustfmt --edition 2021 --config-path rustfmt.toml --check` is clean on every file above. The editor
file still reports 6 hunks (lines 30, 246, 261, 1835, 1924, 3699) — all pre-existing or peer-owned
(`Puzzle2dActiveExampleWork`, the import block, `io()`), byte-identical to the count at `HEAD`.

---

## 6. What B1 must know

1. **Already done by B1, and correct** — all 10 of my ids are in `PUZZLE2D_RETAINED_TOOL_IDS`, the
   proofs `tools:` list, `PUBLICATION_CONTRACTS` and `.action_interactive_job(…, Migrated)`, and
   `reorganize` already routes to `Puzzle2dForceLayoutWork::new("reorganize")`. Nothing to add.
2. **Lane correction, non-blocking.** The gate at `🧰️framework/…/🔌️plugin/🦀️.rs:22940` rejects an
   *undeclared* lane, never an over-declared one, so `Artifact + Config` on all eight fill ids is
   safe. But `brushFillSessionAdopt` / `Cancel` / `Discard` / `Clear` only ever publish `Config`;
   tightening those four to `&[ArtifactToolPublicationLane::Config]` would make the contract honest.
   `redrawHandles` (`Artifact`) and `reorganize` (`Artifact`) are exactly right as declared.
3. **Do not re-add** `is_fill_session_action` / `dispatch_fill_session_action` to the editor, or the
   `mounted_job_prepare_snapshot_read` / `pending_effects` overrides — the fixture now carries
   hostile mutations (`fillSessionHandleBranchRevived`, `fillSessionStoreLeaseRevived`) and an editor
   source contract (`fill_session_retained_only_contract`) that reject them. The shared fn a caller
   should reach for is `set_fill_count::fill_session_control` (private) via
   `set_fill_count::Puzzle2dFillSessionWork`, or `set_fill_count::discard_fill_session` for the
   `setActiveUtility` coupling.
4. **`puzzle2d_dispatch_emit` still has a `"redrawHandles" => redraw_handles::redraw_handles(ctx)`
   arm.** It is unreachable in production (the id is `Migrated`) and it is *semantically* identical to
   the new work (same engine snap → same `replace_node_handle` deltas), so it does not violate the
   "must not differ" rule — but it is the last unbounded whole-fixture call in the file and should be
   deleted along with the other dead legacy arms, together with the `🔄️redraw-handles` leaf and its
   crate-root `mod`. I left it because `puzzle2d_dispatch_emit` is B1's region.
5. **`🗄️retained-jobs/🔣️.json` `toolIds`** — B1 filled it and it must stay set-equal *and order-equal*
   to `PUZZLE2D_RETAINED_TOOL_IDS` (`assert_eq!(actual.tool_ids, expected.tool_ids)`,
   `🧵️retained/🦀️.rs:929`). Note the same assertion is **red for puzzle3d today**: its fixture lists 4
   `toolIds` against a 62-entry `PUZZLE3D_RETAINED_TOOL_IDS`. Not mine to fix, but it will surface in
   the same test run.

---

## 7. What the main session must compile-verify

None of this is compiled. In rough risk order:

1. **`🧮️set-fill-count/🦀️.rs` borrow-checking.** The riskiest constructs, all deliberately written to
   avoid the known traps: `search_step_one` destructures `Self { search, preview_sequence, operation, .. }`
   for disjoint field borrows; `absorb_outcome` binds `self.search.as_mut().and_then(…)` to a `let`
   *before* matching (a match scrutinee would extend the `&mut self.search` temporary across arms that
   call `self.runtime_mut()`); `apply_one` scopes its destructure inside a block; `drain_outcome`
   re-reads `self.outcome` each iteration instead of holding a loop-carried `&mut`. `close_one` copies
   the exact shape the deleted `FillSessionNode::close_step` used and which compiled.
2. **Engine re-export reachability** for `circle_handle_angle_toward`, `rectangle_handle_angle_toward`,
   `board_json_visible_or_true`, `fixture_edge_handle_ids_from_object`, `Point` through
   `crate::editor::puzzle2d::engine::*`. `distance_between` and `handle_position_on_circle` from the
   same `pub use` lines are already used by `🔗️linking/🦀️.rs` and `🌉️wasm/🦀️.rs`, so the path is
   proven; these five come from the same re-exports but I did not confirm each individually.
3. **`Operation` field visibility** — I construct
   `Operation::new(self.operation.operation, self.operation.base_revision, self.operation.generation, seed)`.
   The deleted code read all three of those fields, so they are public; `Operation: Copy` is assumed
   from `bind_operation`'s by-value signature.
4. **`Drop`-assert correctness at runtime.** `BoardFillCheckpoint`, `BoardFillPlacement`,
   `FillPlacementApplyCursor` and `BoardFillJob` all `assert!` terminal-emptiness on `Drop` — a
   mistake in `close_one`'s ordering aborts the wasm component rather than failing a test. This is
   the one thing a compile cannot catch: it needs `cargo test -p semio-s-plugin-puzzle` plus a real
   fill dispatch in `dev 2d`.
5. **`Puzzle2dConfig: Default`** (used by the new `fill_control_verbs_are_pure_runtime_transitions`
   test) and `protocol::InteractionState: Default` (used by `fill_session_extent_is_the_enforced_budget`).
6. The two new `build_tool_job` arms sit **below** B1's `generic if …` / `host_only if …` guard arms;
   `redrawHandles` is in neither list, so no arm is shadowed — worth a glance if B1 moves things.

---

## 8. Not finished / not attempted

- **Runtime verification of anything.** No cargo, per the wave brief.
- **`redrawHandles` end-to-end** is blocked on wave A1's `replace-node-handle` diff fix (§2).
- **The `redraw_handles` legacy leaf** was left in place because `puzzle2d_dispatch_emit` is B1's
  region (§6.4).
- **`PUZZLE2D_FILL_COUNT_MAX = 1_000` at the search ceiling.** 512 × 512 = 262,144 engine steps is
  comfortable for a few hundred placements but may not finish 1,000 in one job; by design that
  publishes a `CheckpointReady` runtime and `brushFillSessionStep` resumes. Whether the panel
  actually dispatches that continuation is a UI question I could not verify without a boot.
- **`🗄️retained-jobs` vector *semantics*** are declarative only — the fixture oracle checks shape and
  ids, not behaviour. Turning the new vectors into executed cases would need a driver that does not
  exist for puzzle2d (the same gap 3d has).
