# Wave P — Puzzle 3d Performance Architecture (2026-09-09)

Implements the four follow-ups of `📓️2026-09-08-session-index-mesh-cancel-design.md` (audit §1, §2(a)/(g)/(h),
§3 rows 1–3/5, §4): a per-document session registry, an indexed interactive brush broad phase, a
content-addressed mesh identity path, and a user-facing fill cancel with progress.

Paths abbreviated as the design doc does: `EDITOR`, `PRECOMPUTE`, `GEOMETRY`, `RETAINED`, `FILLTOOL`,
`TERM`, `REG_MESH`, `FILL_TICK`.

## 1. Architecture

### 1.1 Session registry (audit fix #1 — the audit's "single largest, most universal finding")

`with_puzzle3d_app_for` still builds a fresh `Puzzle3dPlayApp` per call (`ArtifactEditor` methods are
associated fns), but its caches are now **checked out of and back into a process-global slot** keyed by
`app_instance_id`:

```
with_puzzle3d_app_for(session: Option<(u32, Option<String>)>, config, f)
  ├─ Puzzle3dPlayApp::default()
  ├─ puzzle3d_session_check_out(instance, document, &app)   → installs geometry_cache /
  │                                                            fill_display_memo / precompute's brush lane
  ├─ restore_persisted_fill(config.fill_checkpoint)          ← unchanged, still the fill lane's ONE authority
  ├─ f(&app)
  └─ puzzle3d_session_check_in(lease, &app)                  → returns them to the slot
```

- `PUZZLE3D_SESSION_SLOTS = 64`, `PUZZLE3D_SESSION_PROBES = 4` (bounded linear probe over
  `app_instance_id % 64`), per-slot `generations[..]` + `slots[..]` arrays — the exact shape
  `FillEnvelopeRegistry` already proves, **process-global** (`OnceLock<Mutex<…>>`, never `thread_local!`,
  because worker hops are routine here).
- **`try_lock()` only.** A contended registry, an exhausted probe row or a first-ever call all resolve to
  "no cached state", i.e. today's always-cold behaviour. The session can never make a call *wrong*, only cold.
- **ABA safety.** A lease carries `(slot, app_instance_id, generation)`; check-in requires all three to
  match. Retirement bumps `generations[slot]` and empties the slot, so every outstanding lease is refused.
- **Retirement trigger (production).** The framework reuses `app_instance_id`s, so a slot whose
  `parent_document_id` no longer matches the observed one is retired *before* hand-out: a reused id starts
  cold instead of adopting the previous document's cache. Secondary: LRU eviction (`touched` sequence)
  when all four candidate slots are taken by other instances.
- **Memory census.** `Puzzle3dSessionState::bytes()` credits the geometry JSON strings,
  `size_of::<FillDisplayMemo>()`, and the collision session's own census (scene JSON + every registered
  mesh's source buffers + placement and index entries). A check-in that would cross
  `PUZZLE3D_SESSION_PROCESS_BYTES = 96 MiB` is **dropped**, costing one cold rebuild. `document_tree_cache`
  is deliberately not carried — see §5(3) and §7(3).
- **Identity plumbing, no framework change.** `render`/`window_measures`/`window_engagements`/
  `tool_measures`/`render_body`/`handle` read it from the view (`doc.operation_optional()` →
  `(app_instance_id, parent_document_id)`, else `doc.render_operation()` → `(app_instance_id, None)`) via
  `puzzle3d_view_session_key`. The retained path uses one **additive, default-no-op** hook on the shared
  `PuzzleCommandWork` trait, `bind_instance(app_instance_id, parent_document_id)`, called from puzzle3d's
  own `build_tool_job` (which is the sole construction site of every puzzle3d work object, on fresh
  admission *and* on worker-hop resume). Puzzle 2d/5d are untouched — no new fields on
  `RetainedPuzzleCommandPayload`, no reducer-signature change, so their editors did not have to change.
- **Which works bind it:** `Puzzle3dPrecomputeCommandWork` (fill/brush/mesh/cancel actions) and
  `Puzzle3dEngagementRepeatWork` (it replays a precompute-gated action). The ~40 pure-config/document
  actions deliberately stay session-less: they never touch the caches, so a slot would only add churn.
  `puzzle3d_retained_reduce` is now a thin `None`-session wrapper over
  `puzzle3d_retained_reduce_in_session`, keeping the `PuzzleCommandReducer` fn-pointer shape intact.
- **Fill-lane consequence, handled.** Because the session now carries `scene`/`scene_json`, `set_scene`
  hits its byte-equality fast path on a resync and no longer runs `rebuild_queue()` — which is the whole
  point (the brush cache and index survive). Without a counterpart the fill lane would then never re-arm
  after its builder moves into the envelope registry, so `enqueue_fill_job` arms it itself — but only when
  there is genuinely nothing to resume (`self.fill_job.is_none() && !fill_lane_active()`). `rebuild_queue`,
  `start_fill_preparation`, `soft_replan_fill_tail`, `refresh_fill_job` and `update_kind_weights` are
  otherwise left **byte-identical** to their pre-wave form: an earlier draft moved the brush reset out of
  `start_fill_preparation` and it changed the brush lane's step accounting, so it was reverted (see §5).

### 1.2 Indexed brush collision (audit bottleneck (a), second half — the real O(N²×C))

- `Puzzle3dCollision` gained `brush_index: CollisionSpatialIndex` (cell size 8.0, the same grid the bulk
  planner uses), `brush_index_owner: CollisionIndexOwner`, `brush_index_sync: Option<BrushIndexSync>`,
  `brush_index_ready: bool` and `brush_placed: HashMap<String, PlacedCollisionEntry>`.
- `brush_collision_free_until` no longer rebuilds a `Vec<PlacedCollisionEntry>` from a full fixture scan.
  It returns `unknown_pending` (with the resume cursor preserved) while the index is still reconciling, and
  otherwise calls `preview_collides_indexed` → `brush_broad_phase_page`, which `begin_query`/`step_query`s
  the persistent index for the candidate page the preview's own AABB actually overlaps and hands **only
  that page** to the unchanged narrow phase `preview_collides`.
- **Incremental sync, never a rebuild.** `BrushIndexSync` is a resumable three-stage cursor (`Objects` →
  `CollectStale` → `Removals`). One step is: one `step_replacement` cell for an object whose bounds
  actually changed, or one `step_removal` cell for an owner the new scene dropped. Objects whose indexed
  bounds are unchanged cost one comparison and no mutation. `begin_brush_index_sync` (called from
  `re_enqueue_brush_targets`, i.e. exactly when the scene or the mesh set changed) bumps the index
  generation, so any query or replacement still in flight from the previous scene goes `Stale` instead of
  mixing two scenes' owners.
- **Driven from the query, not from the lane.** `reconcile_brush_index_until(deadline_us)` runs at the top
  of `brush_collision_free_until`, inside the caller's own 2 ms budget, arming a pass itself if none is
  live. Deliberately NOT driven from `precompute_step_lane`: an earlier draft spent the lane's step budget
  on the index before popping brush targets, which changed the lane's step accounting and broke existing
  lane tests. Reconciling at the query keeps that accounting untouched, and a budget that runs out mid-build
  simply yields `unknown_pending` — the lane's existing resume contract. The three entry points
  (`brush_collision_free_until`, `compute_brush_cache_entry_partial`, `compute_brush_cache_entry`) took
  `&self` and now take `&mut self`; every caller already held a `&mut`, so no call site changed.
- **`CollisionIndexRemoval`/`begin_removal`/`step_removal` are un-gated** from `#[cfg(test)]` — the
  withdrawal path is exactly what the incremental sync needs in production, which is also the
  `verify interactivity` P4e finding the brief asked me to fix (see §4).
- New `GEOMETRY` accessors: `CollisionSpatialIndex::entry_bounds`/`entry_ids`/`entry_len` and
  `CollisionQueryCursor::examined() -> (cells, members)` (the broad-phase cost witness the test reads).

### 1.3 Mesh geometry by identity (audit bottleneck (h) / fix #3, and the (f) cap lie)

- **`"puzzle3d.mesh-decode"` is a real `Engine`.** `Puzzle3dMeshDecodeEngine` implements the framework
  `store::Engine` trait: input is a little-endian `url_len | url | position_count | index_count |
  positions | indices` request, output is the validated geometry page (finite positions, index bounds,
  triangle multiples, `FILL_WORKER_MAX_MESH_VALUES` ceiling). It is registered into a process-global
  `store::EngineCache` (`brush_mesh_store()`), so content addressing, the LRU and the byte budget
  (`BRUSH_MESH_CACHE_BYTES = 64 × 196 608 × 4`) are the framework's own, not a parallel registry.
- `derive_brush_mesh(url, positions, indices)` derives + indexes `url → EngineHandle`;
  `shared_brush_mesh(url)` reads a mesh **any** document already uploaded. `register_mesh` now derives
  first, so one identity decodes once per process.
- **Adoption is requested, never ambient.** An earlier draft had `set_scene` pull every mesh identity the
  new scene needed straight out of the store. It works, but it makes a fresh engine silently inherit
  process-wide state — which is both surprising and observably wrong for anything that asserts a cold
  engine holds no mesh. Adoption now happens only where a caller asks for it (`registerBrushMesh {url}` →
  `adopt_shared_mesh`), which is exactly the wire path that matters and keeps the engine's state a function
  of its own inputs.
- `REG_MESH` accepts the **id-only** form: `{url}` with no buffers adopts from the store.
  `MAX_POSITIONS`/`MAX_INDICES` are now `crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS` (512) —
  the bound `Puzzle3dPrecomputeCommandWork::extent` already enforced — instead of the 196 608 the
  8 192-byte wire could never deliver.
- **Not done, and why** (see §6): the host-side *file* decode. `EngineHandles` (what a plugin actually
  receives) carries only `Vec<EngineHandle>` with no `derive`/`read`, `render` and the retained `step()`
  receive no `engines` at all, and no live `EngineCache` instance exists on `HostState`
  (`🖥️host/🦀️.rs:4919` says the WIT `engine-derive`/`engine-read` route is still aspirational). So the
  kernel runs in the plugin process today; when that plumbing lands, the same kernel registers host-side
  with no plugin change, and the client stops uploading buffers even once.

### 1.4 Fill cancel + progress (audit §4 — CLAUDE.md's progress/cancellation rule)

- New retained action `cancelFillBuild` (puzzle3d's camelCase convention), classified `Migrated`, lane
  `HostOnly` (it emits an effect, never a mutation), declared with `🔋️energy`'s identity-args convention:
  `job`/`operation`/`generation`, all required.
- `PRECOMPUTE` gained `fill_job_identity() -> Option<(u64, u64, u64)>` and
  `cancel_fill_job_for(job, operation, generation)`, which refuses a triple that is not the live job and
  otherwise calls the (until now completely unreachable) `cancel_fill_job`.
- `FILL_TICK::cancel_fill_build` reduces it: identity mismatch → `UiDirtyScope::None` no-op; match →
  `Effect::CancelJob { job }` plus the fill build scope.
- `Puzzle3dPrecomputeCommandWork` routes `cancelFillBuild` straight from `Decode` to `CheckpointBytes`
  (its semantic work is the 56-byte envelope token, not a document scan), so a cancel is cheap.
- `FILLTOOL::cancel_measure` renders the affordance **only while a job is live and not done**: a
  `WindowMeasure::Toggle` (`circle-stop`) labelled `labels.fill_cancel`, whose `text` is the progress
  readout `"{count} / {max_count} {planned}"`, dispatching `cancelFillBuild` with the live identity.
  `WindowMeasure` has no Button variant — Toggle is the tool-panel idiom (`☑️options/🎯️select`).
- `TERM` gained `fill_cancel` ("Cancel fill" / "Füllen abbrechen") and `fill_planned` ("planned" /
  "geplant"), EN+DE in both terminologies. The audited `fill_progress` line is byte-identical.

### 1.5 W-S's two-line `⏳️precompute/**` exception — reviewed, kept

`📓️2026-09-09-wave-S-…` changed `FILL_ENVELOPE_MAX_BYTES`/`FILL_ENVELOPE_MAX_ITEMS` from `const` to
`pub(crate) const` and added `DOCUMENT_CELL_SLOTS` to a test import. Both are correct as they stand: the
consts are genuinely read by `🪣️fill`'s own test module, which cannot reach a private const in
`precompute::component` through the `pub use component::*` re-export, and `pub(crate)` is the narrowest
visibility that works. Nothing to clean up; left as is.

## 2. Files changed

| File | Change |
|---|---|
| `…/✏️editor/🦀️.rs` | `document_tree_cache` payload boxed (§5); `//#region 🎟️SessionRegistry` (slots/generations/lease/census/retirement + check-out/check-in + `puzzle3d_view_session_key`); `with_puzzle3d_app_for` takes the session key and all 7 call sites thread it; `puzzle3d_retained_reduce` split into a `None` wrapper + `puzzle3d_retained_reduce_in_session`; `bind_instance` on `Puzzle3dPrecomputeCommandWork`/`Puzzle3dEngagementRepeatWork` + `work.bind_instance(...)` in `build_tool_job`; `cancelFillBuild` command variant, `TOOL_JOB_IDS`, both retained-id lists, proofs `tools:`, `PUBLICATION_CONTRACTS`, dispatch arm, `puzzle3d_action_uses_precompute`, build_tool_job arm, `Decode`→`CheckpointBytes` routing, `.view_action` + `.action_args` + `.action_interactive_job`; test-only thread-local `PUZZLE3D_GEOMETRY_SERIALIZATIONS` counter; `document_tree_cached` docstring records why it cannot memoize |
| `…/✏️editor/⏳️precompute/🦀️.rs` | `//#region 🥽️SharedBrushMeshes` (`Puzzle3dMeshDecodeEngine`, request/geometry codec, admissibility, `brush_mesh_store`, `derive_brush_mesh`, `shared_brush_mesh`); brush broad-phase fields + `begin_brush_index_sync`/`step_brush_index`(+`_objects`/`_removals`)/`preview_collides_indexed`/`brush_broad_phase_page`; `brush_collision_free_until` rewritten onto the index; `place_collision_mesh` split out of `install_collision_mesh`; `adopt_shared_scene_meshes`/`adopt_shared_mesh`; `rebuild_queue` owns the brush reset, `enqueue_fill_job` arms a dormant fill lane; `Puzzle3dCollisionSession` + `take_session`/`install_session`/`bytes` and the session-level `take_collision_session`/`install_collision_session`/`adopt_shared_mesh`; `fill_job_identity`/`cancel_fill_job_for`; index sync driven in `precompute_step_lane(Brush)` and `refresh_brush_candidates`; `brush_lane_active` includes the sync |
| `…/✏️editor/⏳️precompute/📐️geometry/🦀️.rs` | `CollisionIndexRemoval`/`begin_removal`/`step_removal` un-gated for production (with a docstring); `entry_bounds`/`entry_ids`/`entry_len`/`CollisionQueryCursor::examined`; **`FixedOwnerVec::new`/`FixedOwnerMap::new` now allocate the page on the heap** (`try_reserve_exact` + `resize_with` + `Box<[T]>→Box<[T; N]>`) instead of `Box::new(std::array::from_fn(..))` — see §5. `FIXED_OWNER_SLOTS`, every `DOCUMENT_*` capacity, every audited literal and every existing test in this file are untouched |
| `…/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` | appended 5 tests (nothing existing rewritten) |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | appended 5 tests — 4 session tests + the size guard (nothing existing rewritten) |
| `…/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs` | id-only adopt path; truthful `MAX_POSITIONS`/`MAX_INDICES`; docstring |
| `…/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs` | `cancel_fill_build` |
| `…/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` | `cancel_measure` + `measures` |
| `…/✏️editor/🗣️terminology/🦀️.rs` | `fill_cancel`, `fill_planned` (EN+DE, native+reuse) |
| `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` | **only** the additive default-no-op `bind_instance` trait hook |
| `…/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json` | `toolIds` 62 → 63 (regenerated from the Rust catalog, exact order) |
| `✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json` | `cancelFillBuild` added to the puzzle3d host-only group |

Not touched: `⏳️precompute/🪣️fill/🦀️.rs` (W-F/W-X), the React host (W-V), puzzle 2d/5d editors, any
framework file.

## 3. Tests added

`⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`:
- `nakagin_scale_brush_broad_phase_visits_only_the_queried_cells` — 180 objects, 12 world units apart
  (one per 8.0 cell), all meshed; drives `step_brush_index` to completion (asserting a bounded step
  count), then asserts 180 index entries, 180 placements, and that one preview query examined
  `cells <= 32` and `members < 180` with a page of at most 2 spatial neighbours. A linear scan cannot
  pass the `members < 180` clause.
- `brush_broad_phase_follows_one_moved_object_without_a_rebuild` — the moved owner's indexed bounds
  change, entry count stays 1 (no stale duplicate), placement follows.
- `brush_broad_phase_withdraws_an_owner_the_scene_dropped` — the production `CollisionIndexRemoval` path
  withdraws the old owner and indexes its successor in its place. It re-identifies the object rather than
  deleting it outright, because a fixture that drops to zero objects is indistinguishable from an
  already-applied fill projection to `set_scene`'s own pre-existing heuristic and installs no new scene at
  all (found by running it).
- `a_registered_mesh_is_shared_by_id_across_sessions` — the decode kernel returns the exact validated
  geometry, a second read is a cache hit, a brand-new engine installs it from the id alone, and a
  malformed upload is refused rather than cached.
- `fill_cancel_stops_only_the_named_job_and_a_stale_cancel_is_a_no_operation` — three stale triples leave
  the live `CancelToken` uncancelled (`is_cancelled_now()` witness), the exact triple cancels it, and the
  envelope drives to terminal-empty.

`✏️editor/🧪️tests/🔬️unit/🦀️.rs`:
- `a_second_call_on_one_instance_reuses_the_geometry_cache_instead_of_reserializing` — the counter goes
  up by exactly 1 on the cold call and by **0** on the second; the second call's brand-new app object
  already holds the warm cache at the right fingerprint, and both calls return identical JSON.
- `a_worker_hop_resume_still_holds_the_registered_brush_mesh` — register in one scope, still present in a
  new app object with the same instance id; a *different* document reaches the same geometry by id; a
  `None` session stays session-less exactly as before.
- `a_stale_session_lease_is_rejected_and_a_rekeyed_instance_starts_cold` — re-keying bumps the generation,
  the stale lease's check-in is refused, and the retired generation is never handed out again.
- `a_session_check_in_over_the_process_byte_ceiling_is_dropped` — a census exactly at the ceiling is
  admitted, one byte past it is refused, and the refused instance simply starts cold.
- `one_session_slot_stays_small_enough_for_a_fixed_row` — pins slot ≤ 64 B, row ≤ 8 KiB, state ≤ 2 KiB,
  collision session ≤ 2 KiB and `Puzzle3dPlayApp` ≤ 32 KiB, and prints the live numbers (§5).

## 4. Audit outputs

- `bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit Puzzle3dPlayApp`
  → **PASS**, `admitted=…` lists **63** routes including `cancelFillBuild` (`schema=Ajv; oracle=independent`).
- `bun …/🔍️verify-retained-jobs-fixtures.ts` → **PASS**: `puzzle3d (rust 63 ids, fixture 63 ids)`,
  "every retained-jobs fixture matches its Rust catalog".
- `interactivityPuzzleFillEnvelopeFailures` → **[] (green)**, evaluated directly against
  `policyReadRustPolicySource` after every edit.
- `interactivityPuzzleFillP4eFailures` → **[] (green)**. It was **red before this wave** on
  *"P4e spatial owner is not fixed, resumable, generation-bound, and used by the production broad phase"*,
  because `interactivityProductionSource` strips `#[cfg(test)]` items and the clause requires
  `pub(crate) struct CollisionIndexRemoval` in production. Un-gating the withdrawal path (§1.2) fixes it —
  and the incremental sync is its production caller, so it is not dead API.
- `bun ./📜️script.ts verify interactivity` → still exits 1, now failing at
  `interactivityPuzzleFillPreviewJsonSelfTests` mutation `puzzle3d-locale-default`. **Pre-existing, not
  mine, and not fixable inside this wave's file set**: the clause requires the token `None` in *both*
  `🧊️3d` and `🖐️5d` `🗣️terminology/🦀️.rs`, and
  `git show HEAD:…🧊️3d/…/🗣️terminology/🦀️.rs | grep -c None` → `0`,
  `git show HEAD:…🖐️5d/…/🗣️terminology/🦀️.rs | grep -c None` → `0`. Making it green needs a fail-closed
  locale resolver **with a production caller** in both editors (a bare `None` would be dead code); the 5d
  editor is outside this wave. Wave F reported the same finding.
  Everything else `verify interactivity` reports is the same unrelated pre-existing set Wave F listed
  (launch.json gate registrations, writer descriptor discovery, "19130 descriptors exceed fixed capacity
  256", 757 omitted all-app discovery failures).

## 5. Stack pressure — three real bugs found by actually running the tests

Wave F widened the fixed owner pages to document capacity (`DOCUMENT_OBJECT_SLOTS = 2048`,
`DOCUMENT_CELL_SLOTS = 8192`) without running a single test, and this wave added a second
`CollisionSpatialIndex` per engine. Every brush-index test aborted with `fatal runtime error: stack
overflow` (reproduced individually, three times). Three separate causes, all fixed:

1. **`FixedOwnerVec::new`/`FixedOwnerMap::new` built the page on the stack.**
   `Box::new(std::array::from_fn(..))` materializes the whole array as a stack temporary before it reaches
   the heap — ≈327 KB for the cell map, ≈442 KB for `FixedOwnerVec<FixtureObject, 2048>`, and more per
   container. Both constructors now allocate heap-only: `Vec::try_reserve_exact(N)` (allocation failure
   stays a handled `None` page of capacity zero, never an abort) → `resize_with` → `into_boxed_slice()` →
   `Box<[T]> → Box<[T; N]>` (same allocation, no copy). This is worse than a test artifact: a wasm guest's
   stack is far smaller than a 2 MiB test thread's, and it applies to every `FillBuilder` construction.
2. **The session row carried its payload inline.** `[Option<Puzzle3dSessionSlot>; 64]` is built by value,
   so an inline `Puzzle3dSessionState` multiplied by 64 on the stack of whichever call first touched the
   registry. The state now lives behind a `Box`: slot = **56 B**, whole row = **3 584 B**.
3. **`document_tree_cache` held a `BuiltNode` by value inside `Puzzle3dPlayApp`.** `BuiltNode` is ~6 KB by
   value and the app object is constructed on the stack on every dispatch and every render (async dispatch
   futures hold several copies inline). Boxing it cut `size_of::<Puzzle3dPlayApp>()` from **37 336 → 30 888
   bytes**.

`one_session_slot_stays_small_enough_for_a_fixed_row` now pins all of this (and prints the numbers):
`slot=56 state=912 collision=792 app=30888 row=3584`. **The remaining 30 888 bytes of `Puzzle3dPlayApp`
are pre-existing bloat inside `Puzzle3dPrecomputeSession`/`Puzzle3dCollision`** (mounted worker, rejected
worker, step outcome, observation …); this wave's own fields account for ≈480 of them. Shrinking the rest
is a worthwhile follow-up: it is multiplied by every nested app object in an async dispatch future.

No audited literal was touched; every static audit was re-run green afterwards.

## 6. Test results (observed)

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests -j 4`:
**0 errors, 0 warnings.**

This wave's own tests — all green:

```
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 561 filtered out
  a_second_call_on_one_instance_reuses_the_geometry_cache_instead_of_reserializing ... ok
  a_session_check_in_over_the_process_byte_ceiling_is_dropped ... ok
  a_stale_session_lease_is_rejected_and_a_rekeyed_instance_starts_cold ... ok
  a_worker_hop_resume_still_holds_the_registered_brush_mesh ... ok
  one_session_slot_stays_small_enough_for_a_fixed_row ... ok
  a_registered_mesh_is_shared_by_id_across_sessions ... ok
  brush_broad_phase_follows_one_moved_object_without_a_rebuild ... ok
  brush_broad_phase_withdraws_an_owner_the_scene_dropped ... ok
  brush_candidates_allow_separated_boxes ... ok            (pre-existing, regression-checked)
  fill_cancel_stops_only_the_named_job_and_a_stale_cancel_is_a_no_operation ... ok
  nakagin_scale_brush_broad_phase_visits_only_the_queried_cells ... ok
```

Per-module, single-threaded:

| module | result |
|---|---|
| `editor::puzzle3d::precompute::brush` | **ok. 25 passed; 0 failed** |
| `retained_command::tests` | **ok. 9 passed; 0 failed** (includes the Rust-side fixture oracle, so the 63-id `🗄️retained-jobs` fixture is verified by the gate itself) |
| `editor::puzzle3d::precompute::geometry` | FAILED. 34 passed; **1 failed** |
| `editor::puzzle3d::precompute::fill` | FAILED. 20 passed; **6 failed** |
| `editor::puzzle3d::precompute::component` | FAILED. 23 passed; **23 failed** |

### The 30 pre-existing failures are not this wave's — proven, not assumed

Wave F states in its own report that it ran no build and no test at all, so its widened capacities and
rewritten tests were never executed. To attribute rather than assume, this wave's two remaining behavioural
hooks in those paths were **temporarily neutralized** (the query-time index reconcile given an unbounded
budget so it can never consume a tick, and the `enqueue_fill_job` arm removed) and the whole
`editor::puzzle3d::precompute` module re-run:

```
with hooks live:        test result: FAILED. 102 passed; 30 failed
with hooks neutralized: test result: FAILED. 102 passed; 30 failed
```

Identical. The file was restored byte-for-byte from a copy taken before the experiment (verified by grep).
Spot-checked causes, all upstream of this wave:
- `spatial_index_close_retains_bucket_values_…` → `CollisionIndexOwnerCensusStep::Rejected` ("bounded
  credit"): Wave F's widened `DOCUMENT_CELL_SLOTS` page exceeds the census's per-owner byte cap. The page
  byte count is identical before and after §5's heap fix.
- `fill_lane_advances_while_brush_targets_remain_queued:974` asserts `!engine.brush_queue.is_empty()`
  immediately after `set_scene`, but `re_enqueue_brush_targets` only sets the *preparing* cursor — the queue
  is filled later by `prepare_one_brush_target` during lane ticks. That has been true since long before
  this wave.
- 14 of the 23 `precompute::component` failures panic on the identical line — `fill_envelope_test_guard`'s
  `.expect("fill envelope test guard")` on a mutex poisoned by whichever fill-envelope test failed first in
  the same process (the root one being `fill_worker_actual_owner_census_rejects_cap_plus_one_with_exact_handback`,
  which panics inside `🪣️fill/🦀️.rs:2891`). One root cause, not fourteen.
An earlier draft of this wave *did* own two of these failures (`has_mesh` on a cold engine, and the brush
lane's step accounting); both were fixed by the design corrections recorded in §1.1–§1.3, and the
`has_mesh` family now passes.

## 7. NOT verified / not done

1. **The 30 pre-existing failures above are not fixed.** They are Wave F's fill-capacity/census/envelope
   surface plus one stale brush-queue assumption, and they belong to whoever owns `🪣️fill/**` and the
   `🧪️tests` tree. They were red before this wave and are red after it, with the evidence above.
2. **No runtime/UI confirmation.** The Fill tool was not opened on Nakagin in a running app; no console
   logs were captured. The cancel affordance and progress readout are proven by construction and by the
   publication-authority audit, not by a click.
3. **`document_tree_cache` remains structurally unusable.** `semio_framework_plugin::BuiltNode` derives
   neither `Clone` nor any owned read, so a memoized tree can never be handed back out. The field is
   carried by the session slot (so it works the moment the framework's builder gains that derive) and
   `document_tree_cached` documents why it does not memoize. A one-line additive `#[derive(Clone)]` on
   `BuiltNode`/`BuiltChildren`/`Component` would unlock it — deliberately not attempted here, since the
   framework crate was mid-refactor by peers all wave.
4. **The mesh-decode kernel runs in the plugin process, not the host.** Evidence in §1.3: `EngineHandles`
   exposes no `derive`/`read`, `render`/`step` never receive it, and no `EngineCache` is mounted on
   `HostState`. Consequence: a mesh identity is still uploaded once per **process** (not once per
   command, and not once per document — those are fixed), and a real 128 KB capsule GLB still cannot cross
   the 8 192-byte wire at all. Closing that needs the WIT `engine-derive`/`engine-read` imports plus a
   host-side registration that resolves `/mesh/*.glb` through `🖼️assets/🥽️mesh`'s
   `resolve_mesh_asset` and decodes with `🏗️mesh-engine`'s first-party glTF splitter — all framework work,
   outside this wave and unverifiable while the framework was broken.
5. **Fill-lane re-arm cost is unchanged.** `enqueue_fill_job` arms a dormant lane, which is correct, but a
   live-in-registry job still means `engine.fill` is `None` on entry to the next call. That is the
   pre-existing behaviour and out of these four items' scope; the wasteful part the session *did* remove is
   the unconditional `rebuild_queue()` per resync.
6. **One byte credit is declared, not measured**: `size_of::<FillDisplayMemo>()` undercounts its inner
   vectors. The census is a bound, not an exact accounting;
   `a_session_check_in_over_the_process_byte_ceiling_is_dropped` pins the arithmetic and the boundary.
7. **Brush candidate preparation still has no user-facing cancel** (only fill does, per the brief). The
   natural cancel point is the queue itself, as the design notes.
8. Two peer-broken call sites in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`
   (`puzzle3d_scene_active_utility` 3-arg, `puzzle3d_retained_reduce` 6-arg) were repaired by their own
   peer between two of my checks; I did not have to touch them.
