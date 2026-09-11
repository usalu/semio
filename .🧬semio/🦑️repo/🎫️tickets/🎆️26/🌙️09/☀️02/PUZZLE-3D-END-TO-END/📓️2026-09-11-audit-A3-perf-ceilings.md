# Audit A3 — Performance ceilings still in force on Nakagin (2026-09-11)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Read-only audit, no source edited, no git write, no build.
HEAD at start: `46c3cb9de0` (2026-09-11 12:39, auto-commit) — matches the ground truth in
`📓️2026-09-11-claude-coordination.md`. Served release wasm is #43 (`dist/release`, 2026-09-10 23:23),
which post-dates every wave cited below except this audit itself, so every guest-Rust fix listed as
"live in source" is very likely also live in the browser today — not independently confirmed by a
browser run in this audit (read-only, no server/browser tool used).

Method: read the nine cited prior-session reports plus the master-plan's W-P4/W-R/W-F4/W-F5/W-S2/W-F6
sections (`📓️2026-09-08-performance-architecture-audit.md`, `📓️2026-09-09-wave-P-performance.md`,
`…-wave-P2-precompute-step-budget.md`, `…-wave-P3-command-prologue.md`, `📓️2026-09-10-wave-R-intake-
delta-cost.md`, `…-wave-S2-scene-latency.md`, `…-wave-F6-command-pages.md`, `…-wave-F5-fill-budget-
cancel.md`, `📓️2026-09-09-wave-F-fill-oom.md` tail, `📓️2026-09-10-fill-build-host-tick.md` §8.9,
`…-wave-P5-lanes-render.md`, `📋️master-plan-2026-09-08.md`), then verified every constant/formula the
reports named against the CURRENT tree with `grep -a`/`sed` (BSD grep classifies these emoji-bearing
sources as binary — every read used `-a`). Nine of the twelve candidates in the brief are FIXED at
current HEAD (confirmed by reading the live constant/function, not by trusting the report); three are
still real ceilings; one brief item (`worldRelocate` extent, checklist §26 item 1) is independently
reverified as a STALE claim, not a live defect.

---

## Ranked ceilings still in force

### 1. The generic 4 096-continuation host↔guest settle loop — the only ceiling that still THROWS on Nakagin

- **Where**: `PLUGIN_UI_CONTINUATION_LIMIT = 4_096` and `settlePluginTurn` —
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1341-1410`.
  The loop drives `submitPluginTurn` once per continuation until every required surface has published or
  the guest quiesces; at 4 096 it throws
  `"PluginRuntime: actor ${actorId} did not publish its requested UI surfaces within 4096 continuations"`.
- **Measured**: `🗑️generated/w-ab-41-nakagin-brush4.txt` and `w-ab-41-nakagin-seq.txt` (this ticket,
  2026-09-10, i.e. captured against a wasm close to #43) both show this exact fault firing on
  `interactionSelect`/`interactionHover` on the Nakagin document — the newest recorded evidence in the
  ticket folder, newer than the W-S2 fix that reduced ONE cause of it. It is not hypothetical: it is the
  most recent reproduced fault against Nakagin in this ticket.
- **Root cause, and why W-S2 only partially closes it**: W-S2 (`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:1353`,
  `PATCH_CLOSE_UNITS_PER_TURN = 256` now, confirmed live) fixed retirement pacing for exactly the
  world-3d surface patch ladder (`PatchTracker`/`PendingPatchAuthority`/the kernel's
  `UiTurnPatchRetireArena`). Its own §7 records two ladders it deliberately left untouched:
  `SurfaceReconcileTerminal::close_step` and `MountedTreeTerminal::close_step` still retire ONE owner
  per unit (now driven 64 units/turn instead of 8, not page-granted), and `close_ui_turn_patch_transport_one`
  / `close_table_rows_view_one` were re-paced but never measured. Any surface whose retirement rides one
  of those un-fixed ladders — which includes non-world-3d surfaces touched by the SAME interactionSelect
  turn (inspection panel, outliner) — can still pin `has_unpublished`/`has_work` true for many turns,
  and `settlePluginTurn` counts every one of those turns as a "continuation" toward the SAME 4 096 cap,
  regardless of which surface is stalling it.
- **User-visible symptom**: a thrown JS error surfaces as `"action failed interactionSelect …"` /
  `"interactionHover …"` in the browser console and the click/hover silently does nothing — on Nakagin
  this is reachable from ordinary selection, the single most common interaction in the editor.
- **Architecture fix (not a bigger constant)**: extend W-S2's "retirement priced by page, not by item
  per turn" pattern to the two ladders it left out, and give `close_ui_turn_patch_transport_one`/
  `close_table_rows_view_one` the same `…_with_grant(items, bytes)` shape the kernel and
  `PendingPatchAuthority` already have. Concretely: `SurfaceReconcileTerminal::close_step` and
  `MountedTreeTerminal::close_step` (`🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`) need a grant parameter
  threaded from the turn's own close ladder exactly as `PatchTracker::close_step`/`ReadySlot::close_step`
  got in W-S2; `close_ui_turn_patch_transport_one`/`close_table_rows_view_one` need the same in the
  kernel module. Separately (host side, independent function): `settlePluginTurn` should stop treating
  "a continuation happened" and "no progress was made" as the same signal — track whether the LATEST
  continuation actually retired/published anything (it already has `acknowledgements.length` and
  `hasWork()`), and fail fast with a diagnostic naming which surface is still pending after, say, 64
  continuations with zero acknowledgements, instead of silently spinning to 4 096. That turns an opaque
  timeout into an attributable fault and shortens the failure window ~64×, independent of the Rust fix.
- **Functions to change**: `SurfaceReconcileTerminal::close_step`, `MountedTreeTerminal::close_step`
  (`🩹️patches/🦀️.rs`); `close_ui_turn_patch_transport_one`, `close_table_rows_view_one` (kernel module,
  file not read this audit — locate via `grep -a "fn close_ui_turn_patch_transport_one\|fn
  close_table_rows_view_one" 🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`); `settlePluginTurn`
  (`🔌️PluginRuntime/🟦️.tsx:1379-1410`, add a zero-progress fast-fail).
- **Law to add**: a native law in `⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs` sibling to
  `a_nakagin_scale_world_publication_reconciles_and_retires_within_a_handful_of_reactor_turns`, but
  driving a mixed-surface scenario (world-3d + one retained-table surface, e.g. the outliner) through
  the SAME turn and asserting total turns stays single-digit — this is the scenario W-S2's own law does
  not cover (§7: "the other per-turn `…_one()` ladders were not measured"). TS: extend
  `🔌️PluginRuntime/🟦️.tsx`'s existing continuation-loop tests with a case where `hasWork()` stays true
  but zero acknowledgements happen for >64 continuations, asserting the new fast-fail fires with a
  named surface rather than running to 4 096.
- **Edit scope**: ~4 Rust files (~60-100 lines, mostly grant-threading identical in shape to W-S2's own
  diff), 1 TS file (~20 lines for the fast-fail), 2 new test files/regions.

### 2. Main-thread cost of a Nakagin document switch/refresh — ~5.4 s of long tasks per window, unattributed and unfixed

- **Where**: no single constant — this is the residual W-S2 §2.3/§7 explicitly measured and explicitly
  did NOT attack: `gap=24.3s longtaskCpu=5.4s tasks=21 workerPosts=8799` against the live release
  target, of which W-S2's retirement fix addresses the ROUND-TRIP share (≈78% → collapses with
  137→5 turns) but not the main-thread share. The candidates W-S2 names and does not instrument: the
  first-publication intake cost (`RETAINED_UI_INTAKE_STEPS_PER_NODE = 8_192`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts:138`,
  measured 105 416 real steps ≈ 0.36 s for the packed 26-node Nakagin surface, ×2 windows ≈ 0.7 s —
  confirmed live, unchanged since W-R/W-S2), `projectOwnedUiSurface`'s per-node `advanceUiMaintenance`
  drain, and the React commit of 180 instances into the DOM/WebGL scene.
- **Measured**: 5.4 s of 24.3 s total (22%) is long-task CPU; W-S2's own attribution table puts
  round-trip retirement at 45-60% (now fixed) and "host projection + React commit + everything else on
  the main thread" at ≈22% with no finer split — this is a genuine open unknown, not merely unoptimized.
- **User-visible symptom**: every example switch to Nakagin, and by the same code path every full
  refresh that touches the world-3d surface (a leftover-triggered "full refresh" is called out
  explicitly in the brief), pays several seconds of main-thread jank even after the round-trip fix
  lands and is confirmed in-browser.
- **Architecture fix**: instrument first (this residual has never had a timed probe inside
  `acceptUiPatches`/`projectOwnedUiSurface`/the React commit — W-S2 §7 says so explicitly), then apply
  whichever of two clean fixes the attribution points at: (a) if it is the intake, the delta-priced
  validation program W-R already built (`shapePreserving`, `retainedUiGraphTouchedValidation`) only
  fires on a RE-publish, not a first publication — a first publication cannot skip validating structure
  because there is no "before" to diff against, so the only clean lever left is emitting FEWER nodes for
  a first publication (page the instances lane's initial send, publishing e.g. 32 objects immediately
  and the rest across the following turns the guest is already spending on `fillBuildTick`-style ticks,
  rather than 180 at once) — this is genuinely incremental publication, not a bigger budget; (b) if it
  is the React commit, key the per-lane React components so an unchanged lane (meshes, kind catalogs)
  skips re-render on a pure instance-position update — `React.memo` keyed by the lane's own carrier hash,
  which the wire already carries per `🚚️surface-scene-lanes`.
- **Functions to change**: instrument `acceptUiPatches` (`🔌️PluginRuntime/🟦️.tsx`, the call already
  named in W-S2 §7) and `projectOwnedUiSurface` (`🗣️Interpreter`) with a temporary `[DEBUG]` timing
  split (removed once attributed, per this repo's own `[DEBUG] ` convention); then, depending on
  attribution, either page the instances lane's first-publication content in
  `world_instances_geometry_json` (`WINDOW:128-152` per the 09-08 audit's own numbering) or add
  `React.memo`/lane-hash gating to whatever component in `🌐️World3dHost/🟦️.tsx` renders the instances
  lane.
- **Law to add**: `🔍️ws2-scene-latency-probe.ts` (kept in the ticket folder) re-run against a live
  target after the fix, asserting `gap` materially under the 24.3 s baseline once the round-trip share
  is confirmed out (target: single-digit seconds); a vitest law for whichever component gets memoized,
  asserting a re-render count of 0 for an update that only moves object positions.
- **Edit scope**: 1 temporary instrumentation pass (no lasting diff), then depending on attribution
  either ~1 Rust file (paged first publication, ~40-60 lines) or ~1 TS/TSX file (memoization, ~20-30
  lines) plus one new law each.

### 3. Fill session's own owner pages are still over the new guest contiguous-allocation ceiling

- **Where**: `DOCUMENT_OBJECT_SLOTS = 2048` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/
  🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️.rs:34`) backs a `FixedOwnerVec<FixtureObject, 2048>`
  page of ≈432 KiB, and `DOCUMENT_OWNER_PAGE_BYTES = 64 × FIXED_OWNER_PAGE_BYTES` (same file, line 30)
  permits up to 1 MiB per page — both far over `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES = 65_536`
  (`🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs:47`, confirmed live), the budget W-F6 itself minted from
  the exact incident this owner page caused (the `unreachable` trap at `📐️geometry/🦀️.rs:263`, now
  fixed to refuse instead of trap).
- **Measured**: W-F6 §8 item 2, in its own words: *"They are now honest about refusal, but they are
  still the largest single blocks the guest asks for; if the trap recurs after a wasm rebuild it will be
  one of these being refused"* — i.e. this is a named, acknowledged, unfixed hazard, not a hypothetical
  one. A Nakagin fill session (180 objects, `FILL_COUNT_MAX = 1000`) is exactly the workload that
  allocates this page.
- **User-visible symptom**: on a fragmented/near-full guest (the same condition build #29 hit at
  ~184 ticks / ~44 s into a fill run), fill preparation now REFUSES cleanly
  (`CollisionMutationStep::Rejected(Capacity(id))`) instead of crashing the whole guest component — a
  real improvement — but a refusal still aborts the fill plan the user was 44 s into, with no partial
  result and no retry-at-smaller-scale path. The Fill tool remains the least memory-resilient tool on
  Nakagin precisely because it is the one that allocates document-scale pages at all.
- **Architecture fix**: page these owners below the ceiling instead of widening them once and hoping —
  split `FixedOwnerVec<T, DOCUMENT_OBJECT_SLOTS>`'s single 2048-wide backing array into multiple
  ≤65 536-byte sub-pages (e.g. `Box<[Option<T>; 256]>` chunks, 8 chunks for 2048 slots), allocated
  lazily as `len` grows past each chunk boundary. This changes ONE allocation of 432 KiB into up to
  eight allocations of ≤54 KiB each (`size_of::<FixtureObject>() ≈ 216 B × 256 ≈ 55 KiB`, under the
  64 KiB growth granularity W-F6 documented) — a fragmented guest that can no longer serve 432 KiB
  contiguous can very plausibly still serve 55 KiB, so the refusal rate drops without touching the
  document-scale capacity numbers W-F fixed a session earlier.
- **Functions to change**: `FixedOwnerVec::new`/`try_push`/`get`/`get_mut` and the analogous
  `FixedOwnerMap` methods (`⏳️precompute/📐️geometry/🦀️.rs`, the same region W-F6 already touched for the
  refusal fix) — change `page: Option<Box<[Option<T>; N]>>` to `pages: Vec<Option<Box<[Option<T>;
  CHUNK]>>>`, chunked index arithmetic in the three or four call sites that index `self.page[i]`
  directly. `FillBuilder`'s field declarations (`⏳️precompute/🪣️fill/🦀️.rs`) do not change — they
  already just name `N`, the chunking is internal to the container.
- **Law to add**: extend W-F6's own `an_owner_whose_page_was_refused_refuses_every_insert_instead_of_
  trapping` (`⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs`) with a sibling law that injects an
  allocator refusing any single request over 65 536 B and asserts a Nakagin-scale fill preparation still
  reaches `Admitting`/places at least one object (mirroring `nakagin_scale_fill_is_not_refused_and_
  places_at_least_one_object` from W-F, but under the tighter allocator) — this is the law that
  currently does NOT exist and would fail today.
- **Edit scope**: 1 Rust file (`📐️geometry/🦀️.rs`, the container implementation, ~80-120 lines to
  re-shape three container types' backing storage), 1 new/extended test region.

---

## Reverified as FIXED at current HEAD (confirmed by reading the live source, not by trusting a report)

| # | Ceiling | Where fixed | Confirmed live at |
|---|---|---|---|
| 1 | Fill planner hard-capped at 32 total objects (audit fix #0, "cannot be used on Nakagin at all") | `FIXED_OWNER_SLOTS` stays 32 for bookkeeping; new `DOCUMENT_OBJECT_SLOTS=2048`/`DOCUMENT_ATTRACTION_SLOTS=2048`/`DOCUMENT_VORTEX_SLOTS=4096`/`DOCUMENT_KIND_SLOTS=256`/`DOCUMENT_CANDIDATE_SLOTS=1024`/`DOCUMENT_CELL_SLOTS=8192` size every document-scale `FillBuilder` field | `⏳️precompute/📐️geometry/🦀️.rs:23,30,34,38,41,44,48,51,55` |
| 2 | Interactive brush-suggestion cache, O(N²×C) unindexed linear scan | `Puzzle3dCollision` gained its own `brush_index: CollisionSpatialIndex`, incremental `BrushIndexSync`, indexed `preview_collides_indexed`/`brush_broad_phase_page` | `⏳️precompute/🦀️.rs` (W-P §1.2) |
| 3 | Whole-app-per-call defeats `geometry_cache`/`document_tree_cache`/registered meshes | Process-global session registry, `PUZZLE3D_SESSION_SLOTS=64`, `PUZZLE3D_SESSION_PROCESS_BYTES=96 MiB` | `✏️editor/🦀️.rs:2770,2777` |
| 4 | `openVortexSuggestions`/`fillBuildTick` single-turn prologue cost (17-18 ms, over the 8 ms interactive ceiling) | `handle_action_impl` split into `Puzzle3dActionPrologue`'s `scene_step`/`sync_step`/`dispatch_step`, typed (not `Value`-shaped) construction | `✏️editor/🦀️.rs` region `🧾️ActionPrologue` (W-P3) — measured 18 turns / worst 1.07 ms after |
| 5 | Command-page authority reserved 262 272 B per command regardless of declared size | `CommandPageSet::try_new(declared)` | `🧰️framework/…/📡️spr/🧵️channel/🦀️.rs:165` |
| 6 | A refused fixed-owner page `expect`-panicked into a guest `unreachable` trap | `capacity()` answers 0 without a page; `try_push`/`try_insert` refuse and hand the value back | `⏳️precompute/📐️geometry/🦀️.rs` |
| 7 | Host-imposed `PLUGIN_JOB_STEP_LIMIT = 65 536` cancelled healthy fill plans mid-run | Deleted; `driveSpawnedJob` loops on the guest's own terminal / explicit cancel only | `🧰️framework/…/🧱️elements/🔌️PluginRuntime/🟦️.tsx` (only a stale comment remains, confirms removal) |
| 8 | `fillBuildTick` starved the Isolated job by holding the shared command-ingress lock every 120 ms | `worldFillBuildHostTickAllowed` skips the tick while a drive is active, unless a UI poll is due | `🌐️World3dHost/🟦️.tsx:1517,5061-5070` |
| 9 | Registered brush meshes not persisted across commands / `World3dHost` not wired to the mesh registry | `Puzzle3dMeshDecodeEngine` + process-global `brush_mesh_store()`; `puzzle3dBrushMeshRegistry` IS referenced from `World3dHost` (`.holds`/`.confirm`/`.forget`/`.observeResidency`) | `🌐️World3dHost/🟦️.tsx:4797-4855` — contradicts the 2026-09-10 checklist's "W-H: host not wired" line, which is stale |

## Reverified as NOT a live defect — the brief's explicit worldRelocate item

The brief asks to re-verify checklist §26 item 1 (`📓️2026-09-10-checklist-reverification.md`):
*"`worldRelocate` work-capacity fault on Nakagin — extent `objects + attractions` at `🦀️.rs:3289`
exceeds `PUZZLE_COMMAND_WORK_ITEMS` for ~180 objects."*

Read against current source: `Puzzle3dWorldRelocateWork::extent` (`✏️editor/🦀️.rs:4990-5000`) computes
`object_stage + existing_attraction_stage + candidate_dispatch_stage + candidate_scan_stage`, where
`candidate_scan_stage = object_vortices × 2 + objects.len()` and `object_vortices` sums each object's
REAL `vortices.len()` — not a flat per-object multiplier. For Nakagin (180 objects, ~358 vortices per
the 09-08 audit's own figure): `181 + 359 + 181 + (358×2+180) ≈ 1 617`, comfortably under
`PUZZLE_COMMAND_WORK_ITEMS = 4_096` (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:12`, unchanged).
Line 3289 of the current file is unrelated tool-switch dispatch logic, not this `extent` function — the
checklist's line reference has drifted from intervening same-day edits. **Verdict: this specific claim
is stale; `worldRelocate` does not currently fault on Nakagin.** (This matches the 09-08 audit's own
account: the flat-charge bug and its regression tests predate that audit, i.e. the real fix already
existed before the checklist reverification was written — the checklist likely read a transient
in-flight state from a peer edit that day.)

## Noted but explicitly out of this audit's ranked list (low near-term impact, real but not hit on Nakagin today)

- **`PUZZLE_COMMAND_WORK_ITEMS = 4_096` / `PUZZLE_COMMAND_RAW_BYTES = 8_192`** stay hard, all-or-nothing
  admission gates with no chunking (09-08 audit fix #5, never addressed): every one of the 16 typed
  `extent()` formulas fits Nakagin today, but the design has no graceful degradation if a future fixture
  is ~2.5× Nakagin or a regression reintroduces a flat per-object charge without a paired law (exactly
  what happened once already, per the 09-08 audit's account of the pre-fix `worldRelocate`/
  `createAttraction`/`acceptSuggestion`/`patchInspector` bugs). Worth a standing regression-guard note,
  not an implementation wave, since nothing on Nakagin reaches it.
- **`plugin_exchange`'s boxed future (76 848 B) and `wit_bindgen`'s `start_task` box (~189 328 B)** — both
  over `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`, both explicitly flagged unowned in W-F6 §8 item 1 (a
  peer wave was already on the `wit_bindgen` half). Same causal class as ceiling #3 above (guest linear
  memory only grows, large contiguous requests refuse first) but on the GENERIC command/turn path, not
  puzzle3d-specific — out of this audit's blast radius (framework, not puzzle3d), flagged for the owning
  wave.
- **`CommandEnvelopeSet::try_new()`** (`📡️spr/🧵️channel/🦀️.rs:411`) still reserves 64+64 slots
  unconditionally — but every caller is a HOST (`🏃️run`, `🌉️mcp`, the wgpu renderer) on the system
  allocator, so this is waste, not a hazard (W-F6 §8 item 3, confirmed unchanged).
- **`UI_TURN_PATCHES_MAXIMUM = 1`** (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1253`) — the wire contract
  carries exactly one UI patch per turn, so K dirty surfaces cost ≥K round trips even after W-S2's
  retirement fix. W-S2 §7 names this explicitly as "not attacked" and "a wire-contract change… worth
  doing only if the after-measurement shows the residual is still round-trip bound" — i.e. it is
  downstream of ceiling #2 above (the after-measurement was never taken), not independently ranked here.

---

## Recommended waves

Four independent waves, none touching the same function. Acceptance criteria are either a cargo law
(named) or a `🔍️browser-probe.ts`/`🔍️ws2-scene-latency-probe.ts` step (named).

### Wave 1 — Close the generic continuation ceiling (targets ranked ceiling #1)

- **Files/functions**: `SurfaceReconcileTerminal::close_step`, `MountedTreeTerminal::close_step`
  (`🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`); `close_ui_turn_patch_transport_one`,
  `close_table_rows_view_one` (`🎠️kernel/🦀️.rs`); `settlePluginTurn` zero-progress fast-fail
  (`🔌️PluginRuntime/🟦️.tsx`).
- **Acceptance**: new native law `a_mixed_surface_nakagin_publication_reconciles_within_a_handful_of_
  reactor_turns` (sibling to W-S2's own, `⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs`) asserting
  single-digit turns for a world-3d + retained-table mixed surface set; `🔍️browser-probe.ts --selection`
  on Nakagin (interactionSelect/interactionHover) completes with zero new faults matching
  `"4096 continuations"`.

### Wave 2 — Attribute and cut the main-thread refresh cost (targets ranked ceiling #2)

- **Files/functions**: temporary timing instrumentation in `acceptUiPatches` (`🔌️PluginRuntime/🟦️.tsx`)
  and `projectOwnedUiSurface` (`🗣️Interpreter`); then, per attribution, either paged first-publication
  content in `world_instances_geometry_json` (puzzle3d `WINDOW` module) or lane-hash `React.memo` in
  `🌐️World3dHost/🟦️.tsx`.
- **Acceptance**: `🔍️ws2-scene-latency-probe.ts` re-run against a live target reports `gap` materially
  under the 24.3 s pre-W-S2 baseline (target: single digits) with `workerPosts` already collapsed by
  W-S2; a new vitest law asserting the memoized component's render count stays 0 across a position-only
  update.

### Wave 3 — Page the fill session's document-scale owners below the guest contiguous ceiling (targets ranked ceiling #3)

- **Files/functions**: `FixedOwnerVec`/`FixedOwnerMap`'s backing-storage representation
  (`⏳️precompute/📐️geometry/🦀️.rs`) — chunked `Vec<Box<[Option<T>; CHUNK]>>` instead of one
  `Box<[Option<T>; N]>`; index arithmetic at the handful of direct `self.page[i]` call sites.
- **Acceptance**: new native law (sibling to `an_owner_whose_page_was_refused_refuses_every_insert_
  instead_of_trapping`, `⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs`) injecting an allocator that
  refuses any single request over `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` and asserting a Nakagin-scale
  `nakagin_scale_fill_is_not_refused_and_places_at_least_one_object`-style law still reaches `Admitting`
  and places at least one object.

### Wave 4 — Extend the "reserve only what's declared" pattern past `CommandPageSet` (noted, low priority — framework-owned, not puzzle3d)

- **Files/functions**: `plugin_exchange_boxed` (`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`);
  `CommandEnvelopeSet::try_new()` (`📡️spr/🧵️channel/🦀️.rs:411`).
- **Acceptance**: extend `📏️future-size`'s existing law to assert `plugin_exchange`'s boxed future stays
  ≤ `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`; `CommandEnvelopeSet` gains a `declared()`-style constructor
  mirroring `CommandPageSet::try_new(declared)` with a law analogous to
  `a_command_page_authority_reserves_only_the_pages_its_command_declares`.
- Only include this wave if Waves 1-3 land first and a coordinator confirms no peer wave already owns
  the `wit_bindgen` half (W-F6 §8 records one was in flight 2026-09-10).

Waves 1-3 are the three that move the needle on "every tool works on Nakagin" as observed in this
ticket's own evidence: #1 is the only ceiling with a reproduced hard failure this week, #2 is the
latency every user feels on every document switch, #3 is the only remaining path to a silent-crash-
class defect (now a graceful refusal, but still an availability gap unique to the Fill tool). Wave 4 is
real but framework-generic and already has a peer owner in motion.
