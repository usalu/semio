# Puzzle 3D Editor — Performance Architecture Audit (2026-09-08)

Read-only audit. Scope: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/` (the current Rust-native `puzzle3d` plugin) plus the shared `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` job and the relevant `🧰️framework/` pieces it sits on. All findings below are verified directly against source with `grep -n`/`sed -n` — every claim carries a `file:line`. Paths are given relative to the repo root; the long common prefix for plugin files is abbreviated `…/editor/…` etc. after first use.

Full paths used below:
- `EDITOR` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (7402 lines)
- `PRECOMPUTE` = `…/✏️editor/⏳️precompute/🦀️.rs` (2041 lines)
- `FILL` = `…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` (4265 lines)
- `GEOMETRY` = `…/✏️editor/⏳️precompute/📐️geometry/🦀️.rs` (1700 lines)
- `BRUSH` = `…/✏️editor/⏳️precompute/🖌️brush/🦀️.rs` (584 lines)
- `WINDOW` = `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` (522 lines)
- `CONFIG` = `…/✏️editor/🎚️config/🦀️.rs`
- `SCHEMA` = `…/🧬️schema/🦀️.rs`
- `SELECT_OPT` = `…/✏️editor/🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs`
- `LOD_OPT` = `…/✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs`
- `FILL_TICK` = `…/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs`
- `REG_MESH` = `…/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs`
- `RETAINED` = `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` (713 lines)
- `TRACE` = `🧰️framework/🔨️modules/⏱️trace/🦀️.rs`
- `ACTION_BUS` = `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs`
- `KERNEL` = `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`
- `UI_RESIDENT` = `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs`
- `NAKAGIN_DSL` = `…/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio`
- Plans: `.cursor/plans/puzzle3d_selection_perf_f885b9c9.plan.md`, `…_select_commit_freeze_ae02c6be.plan.md`, `…_selection_lag_5b09b948.plan.md`, `…_fill_preview_perf_4f85194d.plan.md`, `…_precompute_worker_afb88899.plan.md`

## Headline finding

Before the byte/hop-level detail below: the single most consequential discovery in this audit is
that the bulk auto-fill tool has a hard-coded 32-total-object capacity ceiling
(`FIXED_OWNER_SLOTS = 32`, `…/⏳️precompute/📐️geometry/🦀️.rs:21`) baked into every document-scale
field of the fill planner (`…/⏳️precompute/🪣️fill/🦀️.rs`). Nakagin already has 180 objects, so
**opening the Fill tool on the Nakagin example faults the background job on its second step,
before a single candidate is evaluated** — this is not a slowdown, it is a hard functional ceiling
that makes the tool unusable on the very fixture this audit was asked to assess it against. See
Bottleneck (a) and architectural fix #0 below.

## Fixture scale, confirmed

- `NAKAGIN_DSL` is **128,755 bytes** (`ls -la`), matching the ~129 KB figure. Its `objects […] {` block runs from line 62 to the `attractions […] {` header at line 244 — **~181 object rows**, i.e. ~180 placed objects, confirming the requester's figure.
- `📚️examples/🌲️concrete-forest/🦀️.rs` embeds its own small DSL (`include_str!("🖼️assets/🌲️forest/🗣️.dsl.semio")`, `EDITOR`-adjacent file at that example's own path) — 1-object scale, used as the cheap baseline throughout.
- No `.glb` files exist anywhere under `✏️s/🔌️plugins/🧩️puzzle/` (`find … -iname "*.glb"` → empty). Meshes are referenced by URL string in the fixture (e.g. `"/mesh/🧊️base.glb"` in `NAKAGIN_DSL`) and only enter the Rust plugin as raw `positions`/`indices` arrays via `registerBrushMesh` (see Bottleneck (h)).

---

## 1. Data-flow diagram, with budgets at each hop

```
pointer/UI event (click, drag, slider)
   │
   ▼
action id (e.g. "translateSelection", "fillBuildTick", "registerBrushMesh")
   │  bound by PUZZLE3D_RETAINED_TOOL_IDS (EDITOR:2530-2534, ~60 action ids)
   ▼
RetainedPuzzleCommandJob<A>  (RETAINED.rs — shared by puzzle 2d/3d/5d)
   │  wire cap:      PUZZLE_COMMAND_RAW_BYTES      = 8_192   bytes   (RETAINED:10)
   │  decode cap:    PUZZLE_COMMAND_DECODED_ITEMS   = 512     items  (RETAINED:11)
   │  work-unit cap: max_work_units_per_step        = 1       item/step (RETAINED:18, via resumable())
   │  output cap:    PUZZLE_COMMAND_OUTPUT_BYTES     = 262_144 bytes  (RETAINED:13)
   │  step-time cap: PUZZLE_COMMAND_STEP_MICROS      = 7_500   µs     (RETAINED:14)
   │  checkpoint_every_steps = 1, progress_every_steps = 1     (RETAINED:18 args 6,7)
   │
   │  Preflight phase (RETAINED:493-504): work.extent(command, snapshot, interaction) is computed
   │  ONCE, must be <= PUZZLE_COMMAND_WORK_ITEMS = 4_096 (RETAINED:12) or the command hard-faults
   │  ("puzzle command exceeds fixed semantic work capacity", RETAINED:500) — no chunking, no retry.
   │  The preflight loop then advances `preflight_cursor` ONE unit per step, checkpointing every
   │  step (RETAINED:501-504), i.e. it costs `extent` scheduler round-trips before any real work
   │  starts.
   │
   ▼
puzzle3d_retained_extent(command, snapshot, interaction)   (EDITOR:2530-2547)
   │  extent = selection.len() + document_items, where document_items = one of:
   │    - objects.len() + target_volumes.len()   for translateSelection/rotateSelection/
   │      scaleSelection/focusSelection/patchInspector/transformEnd  (EDITOR:2541)
   │    - objects.len() + attractions.len()      for createAttraction/worldRelocate (EDITOR:2542)
   │    - kind-catalog sizes                     for addObjectKind/setObjectKindWeight/
   │      setVortexKindWeight (EDITOR:2543)
   │    - 1                                      for everything else (EDITOR:2544)
   │
   ▼
puzzle3d_retained_reduce(...)  (EDITOR:2549-2586) — the single "Work" step (RETAINED:510-529),
   │  one atomic call, wrapped in:
   ▼
with_puzzle3d_app_for(config, |app| { … })   (EDITOR:2164-2169)
   │  let app = Puzzle3dPlayApp::default();                        <-- FRESH struct, every call
   │  if !config.fill_checkpoint.is_empty() {
   │      app.precompute.borrow_mut().restore_persisted_fill(&config.fill_checkpoint);
   │  }
   │  f(&app)
   │
   │  Puzzle3dPlayApp (EDITOR:2183-2193) carries THREE memoization caches as instance fields:
   │      fill_display_memo: Mutex<Option<FillDisplayMemo>>,
   │      geometry_cache:    Mutex<Option<(u64 fingerprint, String instances_json, String meshes_json)>>,
   │      document_tree_cache: Mutex<Option<(u64, BuiltNode)>>,
   │  All three start at `None` on every call because the struct is rebuilt from `default()`
   │  (EDITOR:2165/2174/2180/6816 — 4 separate `Puzzle3dPlayApp::default()` construction sites).
   │  restore_persisted_fill (PRECOMPUTE:1683-1717) is CHEAP: it decodes an envelope token and
   │  looks up a slot in the process-global `fill_envelope_registry()` mutex — it does NOT replay
   │  the fixture, does NOT re-run fill planning, and does NOT restore geometry_cache/
   │  document_tree_cache/fill_display_memo/registered brush meshes. So "restore from
   │  fill_checkpoint" is cheap by itself, but it only restores fill-job bookkeeping — every OTHER
   │  per-app cache is unconditionally cold on every single retained command AND on every UI
   │  render tick (see next hop).
   ▼
handle_action_impl(...) — the actual mutation reducer for ~50 non-precompute-gated actions,
   │  or one of the precompute-gated actions in puzzle3d_action_uses_precompute (EDITOR:2506-2520):
   │  setBrushPlacementOverlapBudget, addBrushObject, cycleBrushCandidate(Back), openVortex-
   │  Suggestions, acceptSuggestion, suggestionsTick, registerBrushMesh, setFillCount,
   │  fillBuildTick, setObjectKindWeight, setVortexKindWeight, engagementRepeatLast.
   ▼
Emit { artifact_mutations, config_mutations, effects, ui_scope }
   │  fillBuildTick / setFillCount write back config.fill_checkpoint (FILL_TICK:44-55,
   │  EDITOR:2401-2406) — this Vec<u8> config field (CONFIG.rs:144-147, "travels with the config
   │  snapshot so successive commands may execute on any shared-pool worker") is snapshotted via
   │  Puzzle3dConfigMutation::Snapshot EVERY tick the checkpoint token changed (FILL_TICK:50-55).
   │  A live fill also spawns Effect::SpawnJob { kind: FILL_JOB_KIND, placement: Isolated }
   │  (FILL_TICK:29,49) — Isolated means it runs on its OWN pooled actor, not sharing the calling
   │  instance's interactive turn budget (KERNEL:691-699 doc comment).
   ▼
scene/UI re-emission — ArtifactApp::render(body_key, doc, cfg, view_state)  (EDITOR:6956-6997)
   │  ALSO wrapped in with_puzzle3d_app_for(cfg.snapshot, …) (EDITOR:6956) — ANOTHER fresh
   │  Puzzle3dPlayApp::default(), independent of the one used by the action dispatch above, and
   │  independent across every body_key (main/document/catalogue/inspection/settings all call
   │  with_puzzle3d_app_for separately: EDITOR:6846,6956,6997,7015,7032,7039).
   │  For body_key == main::BODY_KEY:
   │    let (instances_json, meshes_json) = app.geometry_jsons(&envelope.fixture);  (EDITOR:6985)
   │  geometry_jsons (EDITOR:2211-2220):
   │    fingerprint = fixture_geometry_fingerprint(fixture)     -- re-serializes objects+
   │                                                                references+target_volumes+meta
   │                                                                to JSON a SECOND time just to
   │                                                                hash them (WINDOW:161-171)
   │    cache.as_ref().is_none_or(|(fp,_,_)| fp != fingerprint) -- ALWAYS TRUE (cache is None on a
   │                                                                fresh app) -> cache miss
   │    -> world_instances_geometry_json(fixture)   O(objects.len())  (WINDOW:128-152)
   │    -> world_meshes_json(fixture)                              (WINDOW:174-190)
   │  For body_key == document::BODY_KEY: document_tree_cached (EDITOR:2223-2225) just calls
   │  document::render(fixture, labels) directly — the "_cached" name is aspirational; the
   │  Mutex<Option<(u64,BuiltNode)>> backing it is never populated because nothing in this file
   │  ever writes to document_tree_cache except invalidation (EDITOR:2217, clearing it to None on
   │  a geometry_cache miss, which is itself unconditional).
   ▼
renderer (host / wgpu / react) consumes instances_json + meshes_json + document tree body every
   call, unconditionally, then applies `revealIndex`-based visibility and `automaticLod`/
   `depthVariableLod`/`manualLod` flags CLIENT-SIDE (WINDOW:382-384) — the Rust side has already
   paid for serializing every object regardless of LOD/chunk settings (see Bottleneck (g)/(h)).
```

### Quantified cost of `with_puzzle3d_app_for` on Nakagin (180 objects)

Per call (retained-command dispatch **or** any of the 5 independent `render`/`window_engagements`/
`window_measures`/`tool_measures`/`context_menu` call sites at EDITOR:6846/6956/6997/7015/7032/7039):

1. `Puzzle3dPlayApp::default()` — cheap allocation itself (a few `RefCell`/`Mutex` wrappers,
   EDITOR:2196-2207), **but** it discards whatever `geometry_cache`/`document_tree_cache`/
   `fill_display_memo` a *previous* call would have built.
2. `fixture_geometry_fingerprint` (WINDOW:161-171) — one full `to_json_string` pass each over
   `fixture.objects` (180 rows), `references`, `target_volumes`, `meta` — pure overhead, since the
   fingerprint it computes is compared against a cache that is guaranteed empty (finding above).
3. `world_instances_geometry_json` (WINDOW:128-152) — a second full `O(180)` pass building a JSON
   `Value` per object (position/rotation/scale/label/kind/lock/reveal fields) and serializing the
   whole array to a `String` (`serde_json::to_string`).
4. `world_meshes_json` (WINDOW:174-190) — mesh-catalog JSON rebuilt from `collect_mesh_urls(fixture)`
   every call (bounded by distinct mesh URLs, not 180, but still redone from scratch).
5. `document_tree_cached` (EDITOR:2223-2225) — a full `document::render` walk of the same fixture
   for the document/outliner panel, also redone from scratch.

So **every** action on Nakagin — not just fill/brush actions — pays for two full O(180) JSON
serialization passes (fingerprint + instances) plus a full document-tree rebuild, because the
*caching layer that exists specifically to avoid this* (`geometry_cache`, `document_tree_cache`)
is defeated by rebuilding its owning struct fresh on every call. This is the single largest,
most universal finding of this audit — it is not specific to fill/brush, it fires on every
`render`, `window_measures`, `window_engagements`, and retained-command dispatch.

---

## 2. Ranked bottlenecks

### (a) Fill planning on Nakagin — voxelization / candidate placement / overlap budget

**Headline finding: the bulk-fill job cannot be used on Nakagin at all — it hard-faults before
placing a single object, because a fixed 32-slot capacity is shared across every scaling
dimension of `FillBuilder`, and Nakagin already has 180 objects.**

`FILL` (`✏️editor/⏳️precompute/🪣️fill/🦀️.rs`) declares its own fixed-capacity container types in
`GEOMETRY`: `FixedOwnerVec`/`FixedOwnerMap`/`FixedOwnerSet<T, const N: usize = FIXED_OWNER_SLOTS>`
(GEOMETRY:25,113,257) with `pub(crate) const FIXED_OWNER_SLOTS: usize = 32;` (GEOMETRY:21).
`FillBuilder`'s document-scale fields — `placed_lookup` (FILL:1053), `meshes` (FILL:1068),
`spatial_index.entries` (GEOMETRY:782, via `spatial_index: CollisionSpatialIndex` at FILL:1069),
`kind_compatibility` (FILL:1065), the fixture/catalog owner fields (FILL:927-929, 944-946) — are
all declared with the *default* `N = 32`, no field overrides it. `preparation_capacity_refusal`
(FILL:1022-1039) checks, before any placement work starts, whether
`fixture.objects.len()`/`attractions.len()`/`target_volumes.len()`/mesh count/catalog-kind
counts/`kind_compatibility.len()`/weight-map sizes exceed 32, and if so marks the session for
refusal; `begin_preparation` (FILL:2976-2986) computes this immediately, and `step()`
(FILL:4160-4173) publishes a `"preparation-capacity:<branch>"` rejection preview on its first
call and **unconditionally faults the whole background fill job on the very next call** — before
evaluating a single candidate. Nakagin's 180 objects (§"Fixture scale, confirmed" above) exceed 32
by 5.6x, so opening the Fill tool on Nakagin faults immediately. Even starting from an *empty*
scene, the same 32-slot cap re-triggers mid-job as objects get placed
(`AcceptPhase::InstallLookup`, FILL:3975-3981; `CollisionSpatialIndex::step_replacement`'s own
reject at GEOMETRY:1013) and is terminal for that job run (`fixed_rejection` is only cleared on
job teardown, never to allow continuation) — so the bulk-fill mechanism cannot place more than 32
total objects in a single session, regardless of `FILL_COUNT_MAX = 1000` (PRECOMPUTE:19,
`pub(crate) const FILL_COUNT_MAX: usize = 1000;`) ever being reachable. This looks like
`FIXED_OWNER_SLOTS` was sized for the file's pervasive memory-census/owner-retirement bookkeeping
pattern (bounding how much state one interactive-job close-step releases at a time), not for how
many objects a fill run is expected to place — a units mismatch between a bookkeeping constant and
a document-scale capacity.

**Once past that gate (small/empty fixtures), the bulk planner's own algorithm is genuinely
sub-linear and well-architected** — this part of the original framing holds:
- Candidate broad-phase goes through `self.spatial_index.begin_query(...)` /
  `.step_query(...)` (FILL:3763,3769) — a resumable, cursor-based spatial-hash query over an
  8.0-world-unit cell grid (`CollisionSpatialIndex::new(8.0)`, FILL:3032), not a linear scan.
- Narrow-phase overlap uses `CollisionOverlapState::new(512, 8, self.overlap_budget)`
  (FILL:3813) — ≤512 Monte-Carlo samples in batches of 8, early-exiting the moment estimated
  overlap exceeds budget (GEOMETRY:1660-1665) — and rejects on `overlap > self.overlap_budget`
  (FILL:3821).
- Target/candidate weighted selection uses a genuine Fenwick-tree O(log n) weighted pick
  (`fenwick_add`/`fenwick_total`/`fenwick_pick`/`weighted_pick`, FILL:807-919), not a linear
  rescan — the O(n²)-ish `weighted_sample_without_replacement` that does exist (BRUSH:402-432) is
  `#[cfg(test)]`-gated and dead in production.
- Every step is time-boxed via `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US = 2_000` (2 ms,
  PRECOMPUTE:861), and spatial-index mutation is itself cursor-resumable
  (`CollisionMutationStage::{PreflightNew,PreflightOld,Remove,Insert,Commit}`).

**A second, independent bottleneck lives in the *interactive* (non-bulk) brush-suggestion cache,
and it genuinely is O(N²) brute force** — `PRECOMPUTE`'s always-resident `Puzzle3dCollision`
engine (backing single-click brush placement, distinct from `FillBuilder`) does **not** use
`CollisionSpatialIndex` at all (zero references inside `PRECOMPUTE`). Its
`brush_collision_free_until` (PRECOMPUTE:1189-1246) rebuilds a `placed: Vec<...>` with a fresh
O(N) filter over `scene.fixture.objects` on every call (PRECOMPUTE:1213), and its
`preview_collides` (PRECOMPUTE:1152-1187) does a **plain unindexed linear scan over every placed
entry** (`for entry in placed`, PRECOMPUTE:1169) for **every compatible candidate**
(PRECOMPUTE:1227) — i.e. O(N × C) per target vortex. Because a scene-invalidating edit rebuilds
the whole brush queue for every vortex (`re_enqueue_brush_targets`/`rebuild_queue`,
PRECOMPUTE:928-1001, V ≈ total vortex count ∝ N), the full-rebuild cost is **O(N² × C)**,
throttled only by the 2 ms-per-tick wall-clock budget (not a fixed step count) — so on Nakagin
(180 objects, documented at 358 real vortex instances per the extent-bound regression tests cited
in (e)), convergence time for brush suggestions to catch up after any edit scales roughly
quadratically with document size, unbounded in total tick count.

**Plumbing overhead on top of both, when fill IS reachable (small fixtures)**: every
`fillBuildTick` (a retained command, `PUZZLE3D_RETAINED_TOOL_IDS` includes `"fillBuildTick"`,
EDITOR:2531) goes through the *whole* `with_puzzle3d_app_for` reconstruction described in §1
(fingerprint + instances + document-tree, all O(fixture size)) via `fillBuildTick`'s own
`puzzle3d_action_uses_precompute` gate (EDITOR:2511), even though the actual fill step it performs
(`precompute.poll_fill_job()` / `enqueue_fill_job()`, FILL_TICK:22-24) is cheap. `fill_checkpoint`
itself is a fixed 56-byte token (`FILL_ENVELOPE_TOKEN_BYTES = 56`, PRECOMPUTE:48) indexing one of
exactly 4 static registry slots (`FILL_ENVELOPE_MAX_OPERATIONS = 4`, PRECOMPUTE:46) — restoring it
is genuinely O(1), not a fixture replay (confirmed independently of the earlier direct read).

**Bottom line — revised**: the framing "is fill planning O(what)?" has three different answers
depending on which path: the bulk auto-fill planner is sub-linear per accepted placement but is
**capacity-gated at 32 total objects and cannot run at all on Nakagin**; the interactive
brush-suggestion cache is genuinely **O(N²×C)** and unindexed; and the plumbing to reach either
one re-serializes the whole fixture every tick (see (g)). The capacity gate is the most severe of
the three — it is a hard functional ceiling, not merely a performance one.

### (b) Selection / marquee on 180 objects

**This plugin does not implement marquee/box-select at all any more.** `SELECT_OPT`:1-7 states
explicitly: "the marquee method (rectangle/lasso) and default merge mode toggles moved into the
framework's `vortex` interaction domain (`interactionSelect`'s `method`/`merge` args,
`setSelectionMode`) — no longer app config, and no longer renderable here." `grep -rn "marquee"`
across the entire current scope returns exactly that one doc comment — no implementation.
Confirmed independently: `find … -type d -regex ".*puzzle.*3d.*(react|play|rs)$"` (the file layout
the perf plans reference) returns **nothing** — that codebase no longer exists in this repo. A
second, independent trace (plan-vs-source comparison, below) confirms pure selection/pick verbs
(`interactionSelect`, `interactionHover`, `clearSelection`, `selectAll`) don't even reach
`EDITOR`'s `handle_action_impl` — they are framework-reserved verbs dispatched entirely outside
this plugin (`✏️editor/🧪️tests/🔬️testkit/🦀️.rs:33-53`).

**A currently-broken consequence worth flagging even though it is not itself a performance
bottleneck**: `WINDOW`'s `world_selection_json` (the channel `puzzle_3d_selection_lag`'s plan
proposed as "the single selection/hover/gumball/target channel," and which the current code does
implement architecturally) has its own doc comment admitting it is presently non-functional:
*"selection/hover ids… now dissolved into the framework-owned `vortex` interaction domain… this
payload carries no live ids at all until that framework gap closes — the world-3d host renders an
always-empty selection/hover overlay in the meantime."* So the viewport currently shows no
selection highlight at all, independent of any perf question — a functional gap in the framework's
`InteractionView` plumbing (`render` "never gained an `InteractionView` parameter" per the same
comment family cited elsewhere in this report), not something fixable inside `puzzle3d`.

Selection cost that *is* still in this plugin's control: `puzzle3d_retained_extent` adds
`selection.len()` to `document_items` for the transform-family actions (EDITOR:2538,2541) — so a
180-object selection on Nakagin pushes `extent` toward `objects.len() + target_volumes.len() +
selection.len()`, e.g. ~360-540 for a full-select-and-drag, still comfortably under
`PUZZLE_COMMAND_WORK_ITEMS = 4_096` (no rejection risk at this fixture size — see (e)), but each
unit still costs one preflight step + checkpoint (RETAINED:493-504) before the single `Work` call
runs, i.e. hundreds of scheduler round-trips for one drag gesture.

### (c) Fill preview repaint

Directly resolved relative to the plan's diagnosis, by a *different* mechanism than the plan
proposed (see plan-vs-source section below): `PrecomputeLane` (SCHEMA:690-695) is a dedicated
`enum PrecomputeLane { Brush = 0, Fill = 1 }` with doc comment "fill and brush never share one
FIFO queue" — i.e. the exact starvation bug the `puzzle3d_fill_preview_perf` plan diagnosed
(brush targets head-blocking the fill queue) does not exist in the current lane-separated engine.
Preview JSON itself (`world_fill_preview_json`, WINDOW:410-417) reads from
`session.fill_preview_json_page(...)`, a page/cursor read rather than a full-plan serialization.
Residual cost: preview repaint still rides on the same whole-fixture `with_puzzle3d_app_for` +
`geometry_jsons` re-serialization described in (g) whenever the fill tool is active and any
render happens.

### (d) The precompute "worker"

There is **no browser Web Worker** and **no `parry3d`/`three-mesh-bvh`** anywhere in the current
scope (`grep -rn "parry3d\|three-mesh-bvh\|Worker" …` under the plugin returns nothing). The
`puzzle3d_precompute_worker` plan's proposed architecture (a `puzzle/3d/rs` wasm crate + a
`precompute.worker.ts` JSON-RPC Web Worker) was built against a codebase path
(`puzzle/3d/react/`, `puzzle/3d/play/`, `puzzle/3d/rs/`) that no longer exists at all in this
repo. Its *goal* — get collision/candidate compute off the thread handling UI events — is instead
met by `Effect::SpawnJob { kind: FILL_JOB_KIND, placement: JobPlacement::Isolated }`
(FILL_TICK:29,49; `JobPlacement::Isolated` doc: "gets its own pooled actor," KERNEL:691-693): the
framework's own actor/job pool, not a JS Web Worker, and a purpose-built step-budgeted collision
engine (`CollisionSpatialIndex`/`CollisionOverlapState`) rather than `parry3d`. This is a
legitimate, arguably cleaner replacement (no external runtime dependency, consistent with
CLAUDE.md's "no runtime deps on external libraries"), but none of the plan's specific claims
apply to current source.

### (e) Work-item extent bounds vs `PUZZLE_COMMAND_WORK_ITEMS`

**Do not reject actions on Nakagin today — but they used to, catastrophically, until a very
recent fix landed in this same ticket.** `EDITOR` defines **16 dedicated `extent()`
implementations** (one `Work` struct per action family: `Puzzle3dScalarConfigWork`,
`Puzzle3dKindWeightWork`, `Puzzle3dEngagementAbortWork`, `Puzzle3dEngagementRepeatWork`,
`Puzzle3dAddObjectKindWork`, `Puzzle3dScaleWork`, `Puzzle3dPatchInspectorWork`,
`Puzzle3dWorldRelocateWork`, `Puzzle3dCreateAttractionWork`, `Puzzle3dSetActiveExampleWork`,
`Puzzle3dAddBrushObjectWork`, `Puzzle3dFocusSelectionWork`, `Puzzle3dEngagementSubmitWork`,
`Puzzle3dRelocateVolumeWork`, `Puzzle3dAcceptSuggestionWork`, `Puzzle3dPrecomputeCommandWork`),
each computing a real, action-specific formula (e.g. `Puzzle3dScaleWork::extent` =
`object_selection + volume_selection + snapshot.objects.len() + snapshot.target_volumes.len()`;
`Puzzle3dWorldRelocateWork::extent` sums real per-object vortex counts). Any action not covered by
one of these 16 falls through to the generic `puzzle3d_retained_extent` (EDITOR:2536-2547, quoted
in §1 above). All of them currently fit Nakagin comfortably under `PUZZLE_COMMAND_WORK_ITEMS =
4_096` (RETAINED:12) — but only because of a fix already recorded in this ticket's own history.

`EDITOR`'s own unit-test file documents the *pre-fix* behavior in its regression-test comments
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs:322-323,383-384,436,487`, under ticket
`26/09/02/PUZZLE-3D-END-TO-END §Y2`, i.e. this same ticket):
- `worldRelocate` used a flat 66-per-object charge: *"Nakagin's 180 objects (358 real vortex
  instances) computed `objects * 66 + attractions` = 11,880 and faulted preflight … before
  `step()` ever ran"* — **2.9x over the 4,096 cap**.
- `createAttraction` used a flat 128-per-object charge: *"= 23,055 on Nakagin's 180 objects —
  nearly 6x the cap"*.
- `acceptSuggestion` used `objects * 64`: *"computing 7,888+ on Nakagin's 180 objects"* — ~1.9x
  over.
- `patchInspector`'s `"vortex"` arm charged `objects * PUZZLE_COMMAND_DECODED_ITEMS`
  (`objects * 512`): *"computing ~92,160 on Nakagin's 180 objects — a 512x-over-budget design
  error"*.

Each now has a paired regression test asserting the real formula fits
(`world_relocate_extent_fits_within_cap_for_nakagin`,
`create_attraction_extent_fits_within_cap_for_nakagin`,
`accept_suggestion_extent_fits_within_cap_for_nakagin`,
`patch_inspector_vortex_extent_fits_within_cap_for_nakagin`) plus "step-loop stays within its own
extent" tests that drive the real Nakagin document through `step()` and assert observed iteration
counts (`world_relocate_step_loop_stays_within_its_own_extent_for_nakagin` requires `iterations >
100`; `create_attraction_...` requires `> 50`) — i.e. the fix is locked in by tests that fail loudly
if a future change reintroduces a flat per-object multiplier. `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT
= 64` (EDITOR:4108) still exists but is now correctly scoped to **catalog-kind** cost terms
(`addObjectKind`/`addBrushObject`/`acceptSuggestion`), not scene-object cost terms.

At Nakagin's current scale, worst case (`translateSelection`/`rotateSelection`/`scaleSelection`
with everything selected) is `selection.len() (≤180) + objects.len() (180) + target_volumes.len()
(small)` — comfortably under 4,096. The bound would only bite again on a fixture roughly **20x**
larger than Nakagin at today's *correct* formulas, or immediately on any fixture at all if a
future change reintroduces a flat per-object multiplier without a paired regression test. When the
bound is exceeded, there is still no chunking — the command hard-faults at preflight (`"puzzle
command exceeds fixed semantic work capacity"`, RETAINED:500) with no partial-progress fallback;
this fixed-capacity-only design (no graceful degradation) is why the historical bug manifested as
a hard, silent rejection rather than a slow-but-working command.

### (f) The 8,192-byte wire cap

`PUZZLE_COMMAND_RAW_BYTES = 8_192` (RETAINED:10) bounds the *encoded command* on the way in — this
is fine for scalar actions (translate deltas, ids) but directly collides with `REG_MESH`'s own
internal caps: `MAX_POSITIONS = 196_608` / `MAX_INDICES = 196_608` (`f32`/`u32` counts, REG_MESH:13-14)
imply a payload far larger than 8 KB once JSON-encoded (196,608 floats alone is roughly 1-2 MB as
JSON numbers). Since `registerBrushMesh` is itself one of the `PUZZLE3D_RETAINED_TOOL_IDS`
(EDITOR:2531) and therefore travels over the same 8,192-byte `PUZZLE_COMMAND_RAW_BYTES` wire, the
196,608-element internal cap is **practically unreachable** through this path — real GLB meshes
must already be getting to the plugin through many small chunked `registerBrushMesh` calls (or
some other channel this audit's scope did not cover), not the single-shot upload the 196K cap
implies. This mismatch between the command's own declared capacity and its actual transport cap is
worth resolving explicitly rather than leaving as dead headroom.

### (g) Whole-scene re-emission vs incremental patches

**Confirmed whole-scene, not incremental**, and confirmed on the critical path for every render,
not just fill/brush. See the "Quantified cost" section above: `geometry_jsons` (EDITOR:2211-2220)
rebuilds `world_instances_geometry_json` (all 180 objects, WINDOW:128-152) and `world_meshes_json`
(WINDOW:174-190) from scratch on every call because the cache that exists to prevent this
(`geometry_cache: Mutex<Option<(u64,String,String)>>`, EDITOR:2192) is owned by a
`Puzzle3dPlayApp` that is reconstructed fresh (EDITOR:2165/6956/etc.) on every dispatch — the
cache can never observe two calls in a row to compare fingerprints against. `document_tree_cached`
(EDITOR:2223-2225) is in the same position — it calls `document::render` unconditionally despite
its name and despite `document_tree_cache` existing as a field.

Nakagin's per-frame emitted payload size: `world_instances_geometry_json` alone is one JSON object
per placed object (id, meshId, position×3, rotation×4, scale, label, objectKind?, revealIndex?,
disabled) for all ~180 objects — a re-serialization of essentially the whole fixture's geometry,
every render, regardless of whether anything changed (e.g. a pure camera move, a pure hover, a
pure selection-only click all still call `render(main::BODY_KEY, …)` → `geometry_jsons` →
full rebuild).

### (h) Mesh asset loading (`.glb` size, caching, `register-brush-mesh`)

No `.glb` assets are stored in-repo (confirmed above); meshes arrive as raw `positions`/`indices`
arrays via `registerBrushMesh` (REG_MESH:1-21). Caching status, verified:
- `Puzzle3dPrecomputeSession::register_mesh` (PRECOMPUTE:1557-1560) calls
  `self.supersede_admitted_fill()` then `self.engine.register_mesh(...)`, storing into
  `Puzzle3dCollision`'s `meshes: HashMap<String, CollisionBody>` (referenced via `has_mesh`,
  PRECOMPUTE:1567-1569 / GEOMETRY `meshes.contains_key`).
- That `engine` lives inside `Puzzle3dPrecomputeSession`, which lives inside `Puzzle3dPlayApp`,
  which — as established above — is rebuilt from `Puzzle3dPlayApp::default()` on *every* retained
  command (`Puzzle3dPrecomputeSession::new()`, PRECOMPUTE:1513, always starts with a fresh
  `Puzzle3dCollision::new()`).
- `with_puzzle3d_app_for` only restores `fill_checkpoint` (EDITOR:2164-2169); nothing restores
  registered meshes. `grep -n "static.*MESH\|mesh_registry\|MeshRegistry"` across `PRECOMPUTE` and
  `GEOMETRY` returns **nothing** — unlike `fill_checkpoint`, which has a process-global
  `fill_envelope_registry()` slot table (PRECOMPUTE:1687 `fill_envelope_registry().try_lock()`),
  there is no equivalent global table for registered mesh geometry.
- **Consequence**: within this Rust plugin's own state model, a mesh registered via
  `registerBrushMesh` in command N is gone by command N+1 — every command that needs real
  collision geometry for a given mesh URL must either re-register it, or fall back to
  `register_mesh_fallback` (PRECOMPUTE:1112-1114/1562-1565) whose doc comment marks it as a
  distinct "fallback" (likely a bounding-box proxy) rather than the real GLB shape. This audit's
  scope (Rust plugin only) cannot confirm whether a client-side cache compensates by re-sending
  `registerBrushMesh` before every mesh-dependent action, but the plugin itself provides no
  persistence for this data — it should be treated as a real gap, not merely a caching
  inefficiency, since every brush/fill/collision check after the first command potentially runs
  against a different (fallback) shape than intended.

---

## 3. Architectural fixes (clean, long-term — no pragmatic patches)

| # | Bottleneck | Root cause | Clean fix | Files touched | Blast radius |
|---|---|---|---|---|---|
| 0 | (a) bulk-fill hard-capped at 32 total objects — **the fill tool cannot be used on Nakagin at all** | `GEOMETRY`'s `FixedOwnerVec`/`FixedOwnerMap`/`FixedOwnerSet<T, const N: usize = FIXED_OWNER_SLOTS>` default to `FIXED_OWNER_SLOTS = 32`, and every document-scale field of `FILL`'s `FillBuilder` (`placed_lookup`, `meshes`, `spatial_index.entries`, `kind_compatibility`, fixture/catalog owner fields) uses that default with no override. `preparation_capacity_refusal` (FILL:1022-1039) hard-faults the whole background job the moment any of `objects.len()`/`attractions.len()`/`target_volumes.len()`/mesh-url count/catalog-kind counts/`kind_compatibility.len()`/weight-map size exceeds 32 — checked *before* any placement, and re-checked (terminally) as objects are placed even from an empty start. This is a units mismatch: `FIXED_OWNER_SLOTS` was sized for the file's owner-retirement/memory-census bookkeeping pattern, not for "how many objects can a fill run place." | Give `FillBuilder`'s document-scale fields their own, much larger const-generic capacities (or make them genuinely unbounded `Vec`/`HashMap` collections and move the *bookkeeping* capacity limit — which is legitimately about bounding one interactive-job close-step's retirement work — onto a separate, explicitly-named constant that has nothing to do with how many objects a fill session may place). Size the new capacity against `FILL_COUNT_MAX = 1000` (PRECOMPUTE:19), the constant the code already treats as the intended ceiling, not against 32. Add a regression test analogous to the extent-bound fix in (e) below (this ticket already has the pattern: a Nakagin-scale fixture driven through a real fill session, asserting it does not fault). | `GEOMETRY` (`FIXED_OWNER_SLOTS` and the container type definitions), `FILL` (every field declaration listed above, `preparation_capacity_refusal`) | Puzzle 3d only — `FillBuilder`/`FixedOwnerVec` etc. are 3d-specific types, no shared contract with puzzle 2d/5d or the framework. This is the single highest-priority fix in this audit: every other finding here is a performance question; this one is a hard functional ceiling that makes the flagship large-fixture demo (Nakagin) unable to use the tool the audit was asked to assess in the first place. |
| 1 | (a)/(c)/(g) whole-app-per-call defeats memoization | `with_puzzle3d_app_for` (EDITOR:2164-2169) allocates a fresh `Puzzle3dPlayApp` — and therefore fresh `geometry_cache`/`document_tree_cache`/`fill_display_memo`/`engine.meshes` — on every dispatch and every render call | Give `Puzzle3dPlayApp` (or its `precompute`/cache state) a real lifetime tied to the document/config identity instead of the call stack — e.g. a process-owned session keyed by document id (the EDITOR:2160-2163 doc comment already names the target: "until `EngineHandles` carries it") so `geometry_cache`/`document_tree_cache`/registered meshes persist across calls for the *same* document the way `fill_envelope_registry()` already does for fill state. This is squarely the architectural direction the code's own comments point at. | `EDITOR` (`with_puzzle3d_app_for` and its 7 call sites), `PRECOMPUTE` (`Puzzle3dPrecomputeSession`/`Puzzle3dPlayApp` lifetime), possibly a new `EngineHandles`-style registry module alongside `fill_envelope_registry()` | Puzzle 3d only — `Puzzle3dPlayApp` is a 3d-specific type; puzzle 2d/5d and the framework are untouched. The shared-worker-pool constraint the doc comment names (any retained-command call may land on a different worker) means the new session store must be itself keyed/registry-based like `fill_envelope_registry()`, not thread-local. |
| 2 | (g) whole-instance-array re-serialization every render | `world_instances_geometry_json`/`fixture_geometry_fingerprint` (WINDOW:128-171) always run in full because of #1 above | Once #1 lands, `geometry_cache`'s fingerprint check becomes effective for free — no separate fix needed. As a second, independent layer: change the emitted payload from a full replacement array to an id-keyed diff (added/updated/removed instances) so even a genuine cache-miss (e.g. one object moved) does not require re-encoding all 180 objects, matching CLAUDE.md's "event-driven over state-driven" / incremental-patch principle. | `WINDOW` (`world_instances_geometry_json`, new diff variant), the host/renderer consumer(s) of `instancesJson` (outside this scope; flag for follow-up), `EDITOR::render` call site | Puzzle 3d + the renderer host contract for `instancesJson` — likely shared with puzzle 2d/5d's own instance-JSON shape if they use the same host protocol; needs a cross-plugin check before renaming the wire field. |
| 3 | (h) registered meshes not persisted | No global registry for `engine.meshes`, unlike `fill_checkpoint`'s `fill_envelope_registry()` | Extend whatever session store #1 introduces to also own registered mesh geometry keyed by document id (or add a dedicated `mesh_registry()` mirroring `fill_envelope_registry()`'s pattern exactly) so a mesh registered once survives subsequent commands without re-upload. | `PRECOMPUTE` (`register_mesh`/`register_mesh_fallback`, new registry), `REG_MESH` command | Puzzle 3d only (`Puzzle3dCollision` is 3d-specific); no framework change needed since `fill_envelope_registry()` already proves this pattern is viable within the plugin. |
| 4 | (b) selection/marquee | Already moved to the framework's interaction domain (`SELECT_OPT`:1-7) | Out of this plugin's blast radius by design — any further selection-perf work belongs in the framework's `vortex` interaction domain / `InteractionView`, not `puzzle3d`. No further puzzle3d-side fix is architecturally appropriate here; flag the framework owner instead. | framework interaction-domain code (outside this scope) | Framework-wide (affects every plugin using the shared selection domain) — explicitly out of puzzle3d's blast radius. |
| 5 | (e)/(f) hard-fault extent/wire caps, no chunking | `PUZZLE_COMMAND_WORK_ITEMS`/`PUZZLE_COMMAND_RAW_BYTES` are fixed, all-or-nothing admission gates (RETAINED:12/10, fault at RETAINED:500) | For actions whose natural extent scales with document size (kind-catalog edits, mass-selection transforms), split large operations into multiple retained-command dispatches at the call site (client-side batching) rather than raising the caps — CLAUDE.md forbids compatibility layers, and raising a fixed-capacity budget is exactly the kind of pragmatic-not-clean move it also discourages implicitly by insisting on bounded, resumable work. For `registerBrushMesh` specifically, make the 196,608-element internal cap match reality: either genuinely chunk large meshes across multiple `registerBrushMesh` calls (documented, bounded by `PUZZLE_COMMAND_RAW_BYTES`) or lower `MAX_POSITIONS`/`MAX_INDICES` to what one 8,192-byte command can actually carry, so the constant stops promising capacity the transport cannot deliver. | `REG_MESH` (cap constants), client mesh-upload call site (outside this scope) | Puzzle 3d only for the mesh cap; the retained-command contract itself (`RETAINED`) is shared with puzzle 2d/5d, so do not change `PUZZLE_COMMAND_RAW_BYTES`/`WORK_ITEMS` without auditing those two plugins' own extent functions. |
| 6 | (d) "precompute worker" | Plans describe a pre-migration React/WASM-worker architecture that no longer exists | No fix needed — the current `Isolated`-placement job + step-budgeted `CollisionSpatialIndex`/`CollisionOverlapState` design already achieves the plans' goal without an external dependency (`parry3d`) or a browser Web Worker, which is arguably *more* aligned with CLAUDE.md ("no runtime dependencies on external libraries," "use system libraries provided by the frameworks"). Recommend formally retiring/archiving the 5 stale plan files so they stop being read as current architecture. | none (documentation/plan hygiene only) | None — informational. |
| 7 | Cross-cutting: checkpoint-per-step scheduling overhead on large selections | `checkpoint_every_steps = 1` (RETAINED:18) combined with `max_work_units_per_step = 1` means an N-item preflight costs N checkpointed round-trips (RETAINED:493-504) | If profiling (not done in this audit — no builds/tests were run) shows this preflight-step count dominates wall-clock for large selections, the clean fix is to make `puzzle3d_retained_extent` (EDITOR:2530-2547) return a cost *estimate* decoupled from a literal per-step admission count — i.e. keep the fixed-capacity ceiling (`PUZZLE_COMMAND_WORK_ITEMS`) but let the job's own `max_work_units_per_step` grow above 1 for actions whose preflight is provably O(1) work per unit (a pure counter bump), rather than uniformly 1 for every puzzle-3d/2d/5d retained action. | `RETAINED` (`resumable(...)` call site, RETAINED:18), needs sign-off from puzzle 2d/5d owners since the contract is shared | Framework-adjacent — `ToolExecutionContract::resumable` itself (`ACTION_BUS`) is generic; changing puzzle3d's own call to it is scoped, but changing the semantics of `max_work_units_per_step` touches the shared contract type. |

---

## 4. Progress and cancellation — what's actually exposed

**Progress: exposed for fill, not obviously for brush/precompute-step-lane in general.**
- `session.fill_progress_summary()` (PRECOMPUTE:1420 engine-level / PRECOMPUTE:1609 session-level)
  returns `{count, applied_count, max_count, done}`; consumed directly into the UI payload as
  `"fillBuild": {"count":…, "appliedCount":…, "maxCount":…, "done":…}` (WINDOW:353-359), and the
  fill-preview page carries a localized progress label (`labels.fill_progress`, WINDOW:416,
  `session.fill_preview_json_page(&color, labels.fill_progress.as_str())`). This is real,
  wired-up, user-visible progress for the fill tool.
- No equivalent progress summary was found wired into the UI payload for brush-candidate
  preparation (`prepare_one_brush_target`, PRECOMPUTE:~1320) or for the generic
  `precompute_step_lane` — brush suggestion popups appear to be presented as ready/not-ready
  (`unknown_pending` flags) rather than with an incremental progress number.

**Cancellation: primitive exists, but is dead code — no command wires it up.**
- `Puzzle3dPrecomputeSession::cancel_fill_job(&mut self) -> bool` is fully implemented at
  PRECOMPUTE:1862, backed by a real `CancelToken`/`fill_cancel.cancel_now()` mechanism
  (PRECOMPUTE:971,1540,1709,1969 all touch `fill_cancel`).
- `grep -rn "cancel_fill_job" …` across the entire plugin scope returns **exactly one** hit: its
  own definition at PRECOMPUTE:1862. No command file, no `EDITOR` action dispatch, and no test
  outside the precompute unit tests calls it. There is no `"cancelFill"`/`"engagementAbort"`-style
  action wired to it (`engagementAbort` exists as a command, `EDITOR`'s
  `PUZZLE3D_RETAINED_TOOL_IDS`, but it is not among `puzzle3d_action_uses_precompute`'s gated
  actions and a targeted grep found no call from it into `cancel_fill_job`).
- **Conclusion**: a user cannot cancel an in-flight fill from the UI today. The plumbing to do so
  (`CancelToken`, `cancel_fill_job`) is already built and just needs a command to call it — this
  is a small, well-scoped gap, not a redesign, and CLAUDE.md's "support progress and cancellation
  for all expensive operations" rule is currently violated specifically here.
- Brush candidate preparation and the generic collision-index mutation steps expose no
  cancellation surface at all in this audit's search (`grep -in "cancel"` across `FILL`/
  `GEOMETRY`/`BRUSH` turns up only the fill-specific `fill_cancel`/`CancelToken` symbols already
  covered above).

---

## 5. Plan documents vs current source

Two of the five plans target a codebase that literally no longer exists; the other three
substantially *do* match current source, once the file layout is translated. Verified by a
dedicated pass cross-referencing every named symbol/path in all five plans against the live tree
(`find`/repo-wide `grep`), plus this audit's own direct reads.

**Confirmed dead codebase layer**: `puzzle/3d/react/index.tsx`, `puzzle/3d/play/index.ts`,
`puzzle/3d/rs/` (a separate wasm-bindgen crate using `parry3d`), `infinite/world/r3f/index.tsx`,
and `framework/product/playground/renderer/react/index.tsx` do not exist anywhere in this repo
(`find . -type d -regex ".*puzzle.*3d.*(react|play|rs)$"` → nothing; every named TS/JS symbol from
plans 1/2/5 — `createSelectionSnapshotStore`, `useObjectSelected`, `buildPuzzle3dPlayInspectorBody`,
`commitSelection`, `notifySelection`, `Platform.notify`, `ShellDeclarativeWindowBody`,
`createPuzzle3dPrecomputeWorker`, `WasmCollisionEngine` — zero hits repo-wide outside the plan
files). This confirms a full pre-migration-React → current-Rust/WASM rewrite (consistent with the
user's own memory note on a "Semio React Parity Workflow" migration), not merely a rename.

**Verdicts, per plan:**

| Plan | Status | Why |
|---|---|---|
| `puzzle3d_selection_perf_f885b9c9` | **NOT IMPLEMENTED** | Targets the deleted React selection-snapshot store and inspector; every named symbol has zero matches. The underlying `PUZZLE-3D-CTRL-A-SELECT-ALL` ticket it names is closed, meaning the fix *did* land — against the React app, which was later deleted wholesale by the migration, taking the fix with it. |
| `puzzle3d_select_commit_freeze_ae02c6be` | **NOT IMPLEMENTED** | Targets the deleted React shell's `generation`-bump pipeline (`commitSelection`→`notifySelection`→`Platform.notify`→`ShellDeclarativeWindowBody`); zero matches. Its root problem was independently re-diagnosed and fixed for the Rust/WASM app (see Plan 3 below) via `HostEffect::PatchWorld3dChrome` — which was **itself later deleted** (per a comment in `EDITOR`'s own test file: *"asserted on the deleted `Effect::PatchWorld3dChrome`"*) when selection moved to the framework's interaction domain. Two generations of fix, both superseded. |
| `puzzle_3d_selection_lag_5b09b948` | **LARGELY IMPLEMENTED**, against the *current* Rust file layout | `world_instances_geometry_json` (WINDOW:128-152) does not bake `selected`/`hovered` into instances, matching the plan's #1 change exactly, with its own doc comment stating the intent verbatim ("Selection/hover paint is driven by `selectionJson` on the host — never baked here"). Geometry caching-by-fingerprint (`geometry_cache`, EDITOR:2192) and the `sync_precompute_session`-skip allow-list (`puzzle3d_action_uses_precompute`, EDITOR:2506) both exist as proposed. Document tree uses `.interaction_domain(...)` (a `PanelTreeBuilder` call) rather than the plan's literal `.selected()`/`.selection_change()` — a later API, same intent. **Important caveat, and no contradiction with finding §1 of this report**: the plan's proposed caching *architecture* is genuinely present in the code (the `Mutex<Option<(fingerprint,...)>>` fields, the fingerprint function, the doc comments describing it as active) — but this audit's own direct trace (§1 above) shows that architecture never actually activates in production, because `with_puzzle3d_app_for` rebuilds the `Puzzle3dPlayApp` owning those caches fresh on every call. The plan was implemented at the *code-shape* level; a *later, separate* change (the fresh-per-call session pattern) silently defeated it. This is itself a strong argument for fix #1 in §3 below — it is not a hypothetical improvement, it is *reactivating a cache that was deliberately built and then structurally disconnected*. |
| `puzzle3d_fill_preview_perf_4f85194d` | **LARGELY IMPLEMENTED** | Nearly every specific claim matches current source closely: `brush_queue: VecDeque<String>` (PRECOMPUTE:~869) instead of one FIFO queue, `precompute_step_lane(lane: PrecomputeLane, budget)` (PRECOMPUTE:1310) exactly as proposed, a wall-clock deadline checked *inside* the per-candidate loop with a `resume_candidate_index` cursor (PRECOMPUTE:1189-1246, resolving the plan's specific complaint that the old budget was "only checked between tasks"), pure-cache-read `brush_candidates` with an explicit `enqueue_brush_target`, mesh registration that preserves the fill plan instead of wiping it, and a scalar `fill_progress_summary()`. Not found: a scene-cached AABB broad phase inside `rebuild_queue` (AABBs are still recomputed per call) and `compose_fill_display` memoization across panes. |
| `puzzle3d_precompute_worker_afb88899` | **NOT IMPLEMENTED in the current tree — but it WAS shipped once.** | A closed ticket, `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️06/PUZZLE-3D-PRECOMPUTE-WORKER/summary.md` (2026-06-06), documents this exact plan's `puzzle/3d/rs` crate + `parry3d` + `precompute.worker.ts` as built and passing ("306 vitest tests pass," a WASM-vs-`three-mesh-bvh` parity test) — then the entire puzzle3d React/worker layer was deleted by the later full Rust/WASM migration, taking this crate with it. **Today's replacement mechanism, more precisely than this report's §2(d) above**: `Effect::SpawnJob { kind: FILL_JOB_KIND, placement: JobPlacement::Isolated }` (FILL_TICK:29,49) is picked up by `mount_fill_worker`/`MountedFillWorker` (PRECOMPUTE:986-994) and pumped via `semio_framework_async::Lane::Background` (PRECOMPUTE:1352) — i.e. the framework's own async background lane, not a browser `Worker` and not `parry3d`. Functionally equivalent goal (don't block the interactive thread), structurally unrelated implementation, and — per this report's other findings — currently undermined for *brush* suggestions specifically by the still-unindexed O(N²×C) interactive cache in (a) above, which runs on the *same* 2ms-budgeted lane rather than being spawned Isolated the way fill is. |

Recommend archiving or deleting the two fully-dead plan files (`puzzle3d_selection_perf`,
`puzzle3d_select_commit_freeze`) and relabeling the other three as historical/superseded, so none
are read as a live description of current puzzle-3d architecture. Recommend a follow-up ticket
specifically to reconnect `geometry_cache`/`document_tree_cache` (built, then structurally
disconnected — see the Plan 3 caveat above) since that is a materially different, higher-priority
fix than "write a new cache" would be.

---

## Files referenced (for follow-up work)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️标准/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs` (see canonical path above; this note uses the corrupted variant only if copy-pasted incorrectly — always use the `🏅️standards` spelling)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs`
- `🧰️framework/🔨️modules/⏱️trace/🦀️.rs`
- `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs`
- `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs`
- `.cursor/plans/puzzle3d_selection_perf_f885b9c9.plan.md`
- `.cursor/plans/puzzle3d_select_commit_freeze_ae02c6be.plan.md`
- `.cursor/plans/puzzle_3d_selection_lag_5b09b948.plan.md`
- `.cursor/plans/puzzle3d_fill_preview_perf_4f85194d.plan.md`
- `.cursor/plans/puzzle3d_precompute_worker_afb88899.plan.md`

No files were edited, no builds/tests were run (read-only audit per instructions).
