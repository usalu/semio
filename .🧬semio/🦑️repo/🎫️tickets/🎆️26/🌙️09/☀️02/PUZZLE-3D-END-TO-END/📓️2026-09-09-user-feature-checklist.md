# Puzzle 3d — executable user-feature checklist (browser, React shell)

Read-only audit. Target: `http://localhost:6013/?plugin=puzzle3d`, Browser pane 1440×900. Every id/label
below is read directly from current source — `grep -a` needed throughout (emoji paths). Sources:
- Shell: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/{🏛️ShellHost,🛠️ShellHelpers,🌐️World3dHost}/🟦️.tsx`
- Plugin: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` (`🦀️.rs` root
  ~7593 lines; action registry/enum at `🦀️.rs:2050-2185`; `🎮️commands/*` one folder per verb;
  `🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/{🔄️transform,🖌️brush,🧊️volume-brush,🚚️world-relocate}`;
  `🎭️modes/✏️edit/🛠️tools/🪣️fill`; `📌️panels/{🔍️inspection,🗿️artifact,🛍️catalogue,⚙️settings}`).
- Known-broken cross-reference: `📓️2026-09-09-remaining-test-failures-audit.md` (§0-§8, "the audit"),
  `📓️2026-09-09-runtime-verification.md` ("runtime doc"), `📋️master-plan-2026-09-08.md` (latest state,
  16:00), `📓️2026-09-09-wave-D2-window-config-lane-and-scratch.md` ("W-D2"),
  `📓️2026-09-09-wave-S-selection-render-and-writes.md` ("W-S"),
  `📓️2026-09-09-wave-V-host-vortex-select.md` ("W-V").

## 0. Before driving this — build/compile caveat

As of the master-plan's last entry (16:00 09-09), the workspace has been **uncompilable since 15:04**
(a peer's `ArtifactCompositionFields`/`ChildMember` composition wave hitting
`✏️editor/🦀️.rs:6755`, `Puzzle3dPlaySnapshot: ArtifactCompositionFields` not satisfied). No component
rebuild has been confirmed against a tree with ALL of the fixes below landed together. Before trusting
any row marked "fixed", confirm the served component was rebuilt after 16:00 and after the workspace
compiles again. `nakagin_example_loads_via_operations` and the module's pass/fail count (96/38 best run,
load-dependent) are `cargo test` truth, not browser truth — **no full click-through in a real browser
has been recorded past wave V (~02:00)**; everything from W-M onward is source/unit-test verified only.

## 1. Windows — Top, Perspective

Edit mode opens exactly one window kind (`puzzle3d-main`) as two fixed instances — **no user-facing
split/new-window/close** exists for this app (by design, not a bug — `📓️2026-09-08-feature-inventory.md`
§1, master-plan "Interpretation of all windows"). Verify absence, don't hunt for controls that aren't there.

| feature | how reached | DOM id / label | dispatch | success criterion | status |
|---|---|---|---|---|---|
| Top window renders | boot | `#framework.window.puzzle3d-main-top` (`ShellHost/🟦️.tsx:8566`, `childElementId("framework.window", instance.id)`) | n/a (mount) | orthographic grid + example footprint painted (WebGL canvas inside the element) | fixed — rendered 07:33 runtime doc; not reconfirmed after 12:15+ waves |
| Perspective window renders | boot | `#framework.window.puzzle3d-main-perspective` | n/a | 3-point/50° FOV view, gizmo, example object | same as above |
| Focus a window (which pane's shortcuts/utility-bar apply) | click into the pane | the pane's own `framework.window.<segment>` container | host-only (no plugin action) | `ViewModel.window_id` for subsequent dispatches matches the clicked pane | not separately audited; standard shell chrome |
| Split / open new window / close window | — | not offered | — | — | **not applicable** — fixed 2-instance layout is a deliberate design choice, flagged to the dev 2026-09-05 unanswered (`📓️findings-2026-09-05.md` §4) |

## 2. Camera — orbit / pan / zoom

Pure canvas pointer/wheel gestures in `World3dHost/🟦️.tsx` (OrbitControls-family; no static DOM id —
verify via resulting `cameraJson`, not by clicking an id). Dispatches `setCamera` on gesture release,
merged into the per-window camera (`ShellHelpers` `windowMeasuresChrome`/World3dHost camera-merge at
`:749-763`).

| feature | how reached | dispatch | success criterion | status |
|---|---|---|---|---|
| Orbit | left-drag on empty canvas (perspective window) | `setCamera` | `envelope.runtime.camera` rotation changes; view re-renders | **was hung** — WindowConfig lane (§3.1 of the audit) never quiesced on ANY `setCamera` call, 100% repeatable, `puzzle3d app never quiesced` after 1,048,576-turn runaway. **Fixed in W-D2** (`🪟️window/🎚️config/🦀️.rs:82`, byte-grant-vs-owner-ceiling bug) — 6 tests including `camera_actions_are_view_actions_that_emit_no_artifact_mutations` now settle. Not yet runtime-reconfirmed (§0). |
| Pan | right/middle-drag | `setCamera` | camera target translates | same status as orbit |
| Zoom | mouse wheel | `setCamera` | camera distance/FOV changes | same status as orbit |
| Camera is per-window | orbit one pane | `setCamera` (window-scoped) | sibling window's camera untouched (`set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched`) | this exact test is in bucket (c) — hits the load-dependent `job-session.terminal-fault` flake, not a logic bug (audit §5) |

Handler: `🎮️commands/📷️set-camera/🦀️.rs:6-12` — parses `args.camera`; **a malformed/unparseable camera
payload is a complete silent no-op**, no fault surfaced (not user-reachable via normal orbit/pan/zoom,
only relevant if the engagement bar or an external caller sends a hand-built payload).

## 3. Projection pane options

Framework-generic `world3d_projection_measures("puzzle3d", &runtime.camera.projection, …)`
(`🎭️modes/✏️edit/☑️options/🎥️projection/🦀️.rs:17`; builder in
`🧰️framework/…/🔌️plugin/🦀️.rs:32233`). Measure ids are prefix `puzzle3d-measure-projection-*`
(orthographic/1·2·3-point family select + cardinal/free orientation select). Reached via the window's
utility-options rail (unfold the utility bar, "Projection" group). Dispatches `setProjection` /
`setProjectionParam`.

| success criterion | status |
|---|---|
| `envelope.runtime.camera.projection` changes; window re-renders in the new projection family | same WindowConfig-lane hang as §2 — **fixed in W-D2**, not runtime-reconfirmed |

## 4. Window options — grid / LOD / vortex show+direction / sun / select

All per-window, materialized via `load_window`/`save_window` (`🎚️config/🦀️.rs`), reached by unfolding
the window's utility-options rail (same rail as §3). Exact ids, prefix `PUZZLE3D_PLAY_CONTROLLER_ID`
= `"puzzle3d-play"` unless noted:

| group | measure id | action | notes |
|---|---|---|---|
| Grid | `puzzle3d-play-grid` (group), `-grid-visible` (toggle→`setGridVisible`), `-grid-snap` (toggle→`setGridSnapEnabled`), `-grid-spacing` (slider→`setGridSpacing`) | | |
| LOD | `puzzle3d-play-lod` (group), `-lod-auto` (toggle→`setLodAutomatic`), `-lod-depth-variable` (toggle→`setLodDepthVariable`), `-lod-value` (slider 0-1000→`setLodManual`) | | |
| Select (selectable kinds) | `puzzle3d-play-select` (group), `-select-objects`/`-select-vortices`/`-select-attractions` (toggles→`setSelectableKind`) | | marquee-method/merge-mode controls used to live here — moved to the framework `vortex` interaction domain's own args, "no longer renderable here" per the option's own doc comment (feature-inventory §1 TL;DR #5) |
| Vortex | `puzzle3d-play-vortex-show` (select Always/Selected→`setVortexShow`), `puzzle3d-play-vortex-direction` (select Outwards/Inwards→`setVortexDirection`) | | |
| Sun | `puzzle3d-measure-sun` (group), `-measure-sun-enabled` (toggle→`toggleSun`), `-measure-sun-azimuth` (0-360 slider→`setSunAzimuth`), `-measure-sun-elevation` (0-90→`setSunElevation`), `-measure-sun-intensity` (0-4→`setSunIntensity`) | | framework-shared `world3d_sun_measures`, id_prefix `"puzzle3d"` not `"puzzle3d-play"` |

Several of these commands silently ignore unrecognized values rather than erroring — not bugs so much
as good negative-path QA candidates: `setVortexShow`/`setVortexDirection` (unrecognized mode string,
`🎮️commands/🌀️set-vortex-show/🦀️.rs:6-11`), `setVoxelDims`/`setSelectableKind`/`setTransformGumballFlag`/
`setTargetVolumeFlag` (unrecognized axis/kind/flag).

**Status — ALL of the above:** every one publishes through the `WindowConfig` lane, which **never
quiesced** (the audit's §3.1, the single largest reason a session reads as "frozen": touch ANY of
these and the app stops responding). Root cause and fix landed in **W-D2**
(`🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🦀️.rs:82` compared the 4,096-byte per-turn grant against
the owner's 65,536-byte schema ceiling → `Blocked` forever for any owner above 4 KiB). 6 targeted tests
now pass in isolation; **not yet confirmed in a live browser** (§0) — this is priority #1 to click
through by hand.

## 5. Example switcher (Concrete Forest ↔ Nakagin)

| feature | DOM id | dispatch | success criterion | status |
|---|---|---|---|---|
| Example select | `#playground.navbar.fixture` (`ShellHost/🟦️.tsx:7614`, `NavbarExampleSelect`; desktop navbar center cluster, also mirrored into the mobile "App" tab) | `setActiveExample {exampleId}` (`""` = empty, `"concrete-forest"`, `"nakagin-capsule-tower"`) | catalogue tree, scene meshes, `envelope.fixture` swap to the new example; boot order fixed to Concrete Forest first (07:15 runtime doc) | Concrete Forest: confirmed rendering 07:33 (runtime doc). Nakagin: was hard-blocked by the **applied-edit ledger ceiling** (`ARTIFACT_HISTORY_LEDGER_CAPACITY=64`, Nakagin needs ~182 edits) — **fixed by wave W-B** (batched one-`Edit`-per-`Emit`; `nakagin_example_loads_via_operations` passes, 200-mutation test proves >64 lands in one ledger slot). The pure step/extent unit test `set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin` still fails but is a framework/test-shape issue unrelated to the store (bucket (c)). Not runtime-reconfirmed. |
| Empty example → `addObjectKind` | switch to `""`, then Catalogue → add | `setActiveExample {exampleId:""}` then `addObjectKind` | one object appears, referencing a materialized default kind row in `meta.kindCatalogs` | **was a silent no-op** (audit §3.3: cleared/no-catalog document → zero mutations, nothing added, no error). **Fixed in W-D2** (`Puzzle3dAddObjectKindWork` new `Catalog` stage, `🦀️.rs:3506-3708`) — materializes the declared default kind + object in one batched edit. Not runtime-reconfirmed. |
| Unrecognized `exampleId` | dispatch a bogus id | `setActiveExample {exampleId:"bogus"}` | — | `next = None` → **silent no-op** on the document, but attractions are still re-resolved regardless (`resolve_puzzle3d_attractions` always runs) — not user-reachable via the picker (its options are enumerated from the manifest), only relevant to a scripted/engagement-bar dispatch |

**New nuance (this pass):** `setActiveExample` also resets `ctx.scene.runtime = Puzzle3dRuntime::default()`
(camera/grid/sun/LOD/vortex-show all reset) locally in the handler, but its publication contract is
**`Artifact` lane only** (`🦀️.rs:6283`), NOT `WindowConfig` — the same per-window store §2-§4 publish
through. Whether that local runtime reset actually reaches the live per-window `Puzzle3dWindowConfig`
record (so camera/grid really do visibly reset on example switch) or is silently dropped because it
never round-trips through the WindowConfig lane is **unverified — check camera/grid state specifically
before vs. after an example switch**, don't assume it resets just because the handler assigns a default.

## 6. Selection

| mode | how | mechanism | success criterion | status |
|---|---|---|---|---|
| Click select (instance) | click an object in the viewport | `handleInstancePointerDown` → `interactionSelect` via `world3dSelectionActionArgs` (gated on `scene.domainId`) | `selectionJson.ids` / `interaction_state()` carries the id | fixed by **W-S** (render-time `InteractionView` threaded through; click-to-select bound with `domain_id="vortex"`+`domain_granularity_id="object"`) — was previously hardcoded empty (feature-inventory TL;DR #1). Not runtime-confirmed (W-S §9). |
| Click select (vortex marker) | click a vortex marker in the viewport | `handleVortexSelect`/`dispatchVortexHover` — was unconditional `worldVortexSelect`/`worldVortexHover` dispatch to **nothing** (dead verb) | selected vortex id shows in inspection/brush picker | **fixed in W-V** (routes through the same generic `interactionSelect`/`interactionHover` path once the scene is domain-bound); attractions remain **not pickable** by the host (`WorldAttractionLines` has no pointer callbacks — declared limitation, not a bug) |
| Tree/catalogue select | click a row in the Artifact/document panel or Catalogue panel | `interactionSelect` via `puzzle3d_interaction_select` | same as above | confirmed working path per W-S §6 (this was never broken) |
| Marquee / rectangle select | drag on empty canvas | `SelectionMarquee` component, `marqueeModeFromModifiers`/`marqueeCoverageFromGesture` (World3dHost) — modifier-key-gated inclusive/exclusive mode, framework-owned `vortex` domain args now, not a puzzle3d window option | selection set replaces/extends per marquee coverage | method/merge-mode UI moved out of puzzle3d's own window options (§4); marquee itself is host/framework chrome — verify it still exists by dragging, not by an id lookup |
| Select same kind | context menu "Select Same Kind" row (`id="select-same-kind"`) or `selectSameKindSelection` action | `selectSameKindSelection` | selection widens to every object of the clicked object's kind | **was a no-op by construction** (`let _ = kind; ctx.abort = true` — feature-inventory TL;DR #3). **Fixed in W-S** — real replace-select now emitted; test `select_same_kind_widens_the_selection_to_every_object_of_that_kind` added. Not runtime-confirmed. |
| Select all | framework-owned generic action, not puzzle3d-specific (`SELECT_ALL_ACTION_ID`, imported from `semio_framework_plugin` at `🦀️.rs:43`) | `selectAll` (framework verb, exact host-side id/keybinding not located in this pass — likely a shell menu item or Ctrl/Cmd+A) | selection widens to every selectable entity per the window's `setSelectableKind` filters | exists at the framework level (unlike this checklist's earlier guess of "not implemented") — puzzle3d has no override, so verify it behaves sanely against puzzle3d's five entity kinds |
| Clear selection | click empty background | framework `CLEAR_SELECTION_ACTION_ID` (background-clear path, World3dHost `:~4906`) | `selectionJson.ids` empties; `world_pick_locked_object_clears_like_background`/`world_pick_null_clears_without_reselecting_first_object` cover this | one of the two Nakagin/Concrete-Forest flaky tests in bucket (d); functionally real, occasionally hits the `job-session.terminal-fault` load flake |

Confirmed (this pass): select/marquee/select-all/clear-selection/set-selection-mode are **entirely
framework-owned** verbs — `dispatch_puzzle3d_action`'s match never mentions them; puzzle3d only supplies
the `vortex` interaction-domain declaration (granularities, hover channel) they operate against. Don't
look for a puzzle3d command file for any of these; audit them as generic shell/framework behavior.

## 7. Hover

| feature | mechanism | success criterion | status |
|---|---|---|---|
| Hover an object/vortex | `dispatchInstanceHover`/`dispatchVortexHover` → `interactionHover`, `"pointer"` hover channel (`PUZZLE3D_HOVER_CHANNEL`) | `world_vortices_json`/`world_selection_json` carry a `hovered`/`hoveredVortexFullId` flag; visual hover highlight | **was hardcoded absent** (feature-inventory TL;DR #1). **Fixed in W-S** (`world_vortices_carry_their_own_selected_and_hovered_flags` test added) + **W-V** for the vortex-marker path specifically. Not runtime-confirmed. |

## 8. Gumball / transform

| feature | how | dispatch | success criterion | status |
|---|---|---|---|---|
| Move/Rotate flag toggles | utility bar → Transform → options | `setTransformGumballFlag` | flag persists per-window (`transform_utility_options_expose_move_and_rotate_flags`) | flag persistence always real; **gumball never rendered** because `gumball_active()` was hardcoded `false` (TL;DR #2). **Fixed in W-S** — `gumball_active_only_for_transform_utilities_with_object_selection` now asserts `true` when an object is selected under the Transform utility |
| Gumball drag → translate/rotate/scale | drag a gumball handle in the viewport (`UnifiedGumball`/`SceneGumball`, canvas-rendered, no static DOM id) | `translateSelection`/`rotateSelection`/`scaleSelection` — **one absolute delta dispatched on release**, not incremental per-frame | mesh pose updates once, on gesture release; `mesh_selection_ids` (objects+target volumes) move, attractions re-derived | **corrected in this pass** (supersedes the 2026-09-08 feature-inventory's "dead" verdict, which is stale): `transformBegin`/`transformEnd` are current-source `HostOnly`-lane brackets that **deliberately complete empty** (`puzzle3d_shell_only_emit`, `🦀️.rs:2953-2958`) — the drag session itself is host/canvas-local (`gumballTransformDeltaBetweenPoses` computes start→end client-side with zero WASM round-trips mid-drag), and the ONE real dispatch is the commit on release. This is real, working-as-designed architecture, not a stub — the earlier "BatchOnlyPendingRewrite, dead at the dispatch gate" claim describes a since-superseded classification. **Still verify the live drag visual (mid-drag preview, `WorldGumballLivePreviewDelta`) and the commit**, but do not expect `transformBegin`/`transformEnd` themselves to do anything — they are correctly no-ops. |
| Programmatic translate/rotate/scale (no drag) | engagement command line or any direct caller | `translateSelection`/`rotateSelection`/`scaleSelection` | selection pose updates, attractions re-derived | real per feature-inventory §3 (`Puzzle3dScaleWork`), independent of the drag-session gap above |
| Selection required | select nothing, open Transform | `gumball_active_only_for_transform_utilities_with_object_selection` also tests `gumball_inactive_when_every_handle_flag_is_off` | gumball absent with no selection or all flags off | fixed alongside gumball_active (W-S) |

## 9. Brush utility

Utility bar → **Brush** (unfold `#framework.window.puzzle3dMainPerspective.utilityBar.unfold`
per runtime doc → bar shows Transform, Brush, Volume Brush, Relocate).

| feature | dispatch | success criterion | status |
|---|---|---|---|
| Placement candidate picker | render-time only, `🪛️utilities/🖌️brush/🦀️.rs:39-41`, gated on `puzzle3d_brush_target_vortex` | Utility Options shows a picker widget when hovering/selecting a live brush target vortex (`puzzle3d-brush-placement` measure id) | **was hardcoded `None`/dead** (feature-inventory TL;DR #9). **Fixed in W-S** (`puzzle3d_brush_target_vortex` implemented for real; `options(..)` takes the interaction snapshot; test `brush_placement_picker_appears_only_for_a_live_brush_target` added — this test ALSO needed the window-less-`ViewModel` fix in W-D2 to actually pass, both now applied). Not runtime-confirmed. |
| Cycle candidates | brush picker forward/back control | `cycleBrushCandidate` / `cycleBrushCandidateBack` | brush candidate index advances/retreats | real when driven explicitly (feature-inventory §3); depends on the picker widget above actually rendering |
| Add (dock) via explicit click/context menu | click/right-click a vortex, "Suggest Objects" or direct add | `addBrushObject` | object placed at the vortex, collision-checked; W-S adds re-select of the placed object | real; W-S added the re-select-on-place fix. **New (this pass):** `🎮️commands/🖌️add-brush-object/🦀️.rs:22` only handles `Ok(Puzzle3dEngineOutcome::Fixture(_))` from the placement engine — any other outcome, including a real error (e.g. collision), is a **silent no-op with no fault/toast surfaced**. QA: try to force a placement failure (overlap the budget) and confirm nothing visibly happens instead of an error message. |
| Register brush mesh (GLB collision geometry) | host-side asset load, not user-clickable | `registerBrushMesh` | collision geometry available for `addBrushObject`/candidate cycling | real, 21-line command, no chunking (session-index-mesh-cancel-design doc: real GLBs ~127-129 KB exceed the 8,192-byte `PUZZLE_COMMAND_RAW_BYTES` wire cap — verify large meshes actually load, not just small fixture ones); also silently rejects oversized payloads (`MAX_LEAF_BYTES`/`MAX_POSITIONS`/`MAX_INDICES`) with no error |

## 10. Volume Brush utility

| feature | how | dispatch | success criterion | status |
|---|---|---|---|---|
| Paint a target volume | Alt+click in the viewport, grid-snapped | `addTargetVolume` | new target volume in the document, snapped to window grid spacing; constrains Fill | real (feature-inventory §3) |
| Voxel dims (w/d/h) | Utility Options → Volume Brush (`puzzle3d-play-utility-options-volume-brush` group) | `setVoxelDims` | volume brush paint size changes | real |

## 11. Relocate utility

| feature | how | dispatch | success criterion | status |
|---|---|---|---|---|
| Drag-drop relocate | drag an object to a new absolute position | `worldRelocate` | object moves; nearby compatible vortices auto-attract within `proximity_radius` | real logic, but **faults on Nakagin** (180 objects) — `Work::extent()` bound `objects×66 + attractions` vs `PUZZLE_COMMAND_WORK_ITEMS=4,096` cap dies above 62 objects (feature-inventory TL;DR #7, `📓️extent-tightening-report.md` scoped a fix: count actual vortices per object instead of assuming 64 — not confirmed landed). **Verify specifically on the Nakagin example**; works fine on Concrete Forest (1 object) |

## 12. Fill tool

Tool tab: `#framework.category.tool` → `#tool.fill` (activate toggle `tool.fill.activate.toggle`,
options container `tool.fill.options`; `ShellHelpers/🟦️.tsx:3645-3697`).

| feature | DOM id | dispatch | success criterion | status |
|---|---|---|---|---|
| Activate | inner `<button id="tool.fill">` | `setActiveTool {toolId:"fill"}` | tool becomes active; measures panel populates | **was hard-dead** — `host_configuration_mutation` always returned `None`, `setActiveTool`/`setActiveUtility` faulted `interactive-job.missing-owned-reducer` (audit §3.2, 17 tests — every fill/utility test). **Fixed by a coordinator patch to `dispatch_action`** (host-owned verbs with no app mutation dispatch an empty `Emit`) — `interactive-job.missing-owned-reducer` is gone repo-wide as of the W-D2 baseline. **Mechanism confirmed (this pass):** `host_configuration_mutation` returning `None` for `setActiveTool`/`setActiveUtility` is now understood to be **deliberate**, not a bug (`🦀️.rs:6836-6838` doc comment) — puzzle3d's OWN handler (`🎮️commands/🧰️set-active/🦀️.rs:7-17`) also sets `runtime.active_tool_id` and clears `suggestion_menu`/`engagement_input`/`brush_candidate_index` as a real side effect, forwarded to the host via `Effect::SetActiveTool`/`Effect::SetActiveUtility` (`🦀️.rs:2906-2910`) rather than a Config mutation — i.e. there IS a real effect on click, just not the Config-mutation shape the audit's missing-owned-reducer fault was gating. **Residual gap**: the host `ViewModel.active_tool_id`/`active_utility_by_window_id` must persist this across renders for the tab to visually stay "on" — **verify the fill tab visually stays "on" after clicking it**, this is the single most-recently-changed piece of the whole crate |
| Count slider | `puzzle3d-fill-count` | `setFillCount` (parsed by `set_fill_count::parse_count`, clamped to `PUZZLE3D_FILL_COUNT_MAX=1000`) | slider `ready` extent tracks background planning; drag reveals/hides already-planned pieces client-side (reveal key `puzzle3d-fill`) with zero round-trips | real, chunked/resumable; W-P2 fixed the fill planner never running (worker-pool pump starvation) — precompute 104/28 → 132/0. **Nuance (this pass):** `setFillCount` is NOT in `dispatch_puzzle3d_action`'s normal match — it's routed through the special `Puzzle3dPrecomputeCommandWork` state machine (`🦀️.rs:6069,6097-6165`), whose `Publish` stage emits `Puzzle3dConfigMutation::SetFillCount{count}` on the shared `Config` (not per-window/`WindowConfig`) plus `artifact_mutations` for newly-placed objects — a different lane than every other fill-adjacent measure, worth isolating if fill behaves differently from the rest of the tool. |
| Cancel | `puzzle3d-play-fill-cancel` (toggle, icon `circle-stop`) | `cancelFillBuild {job, operation, generation}` | row disappears only while a background plan is in flight (`cancel_measure` returns `None` once `progress.done`); stale cancels for a superseded run are no-ops by carried identity | **was fully unreachable as of the 2026-09-08 design doc** (`cancel_fill_job` had zero callers, no UI affordance at all) — **current source has a real, wired cancel measure** (`🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:53-69`), so this was implemented sometime between 09-08 and now; not mentioned explicitly fixed in any 09-09 wave report read here — **verify it actually cancels a running plan in the browser**, since no test/runtime evidence for it was found in this audit pass |
| Distribution weights | group `puzzle3d-play-distribution`; children `puzzle3d-play-distribution-object-<objectKindId>` (object weight) and `puzzle3d-play-joint-vortex-<objectKindId>-<vortexKindId>` (per-joint weight) | `setObjectKindWeight` / `setVortexKindWeight` | weight sliders change fill distribution; `set_object_kind_weight_declares_fill_options_ui_scope` passes | real; this specific test passes per W-D2 §6 ("host-owned verbs" table) |
| `fillBuildTick` (background driver) | automatic, 120 ms host tick while fill tool active | `fillBuildTick` | background precompute job advances | real; **performance**, not correctness: was 17.6 ms/turn (over the 8 ms law) before W-P3 (→1.03 ms); W-R2 later found it "still breaches 25.6 ms on turn 763" under load — watch for viewport stutter while filling, not a hard failure |

## 13. Vortex suggestions

Right-click / "Suggest Objects" on a single selected vortex (context menu row `id="suggest"`, only
shown when exactly one vortex is selected).

| feature | dispatch | success criterion | status |
|---|---|---|---|
| Open | `openVortexSuggestions {fullId}` | popup opens: `interactionJson.suggestionMenu.{open,candidates,windowId}` all populate | **was dropped before render** — command wrote `runtime.suggestion_menu` but render read a different (always-`None`) partition because the test/render call never named a window (audit §3.4). **Fixed in W-D2** — `render_body` now derives the window instance from the body key; `open_vortex_suggestions_opens_the_suggestion_popup`/`open_vortex_suggestions_records_explicit_window_id` pass |
| Hover-to-preview | hover a candidate | `hoverSuggestion` → `brushCandidateIndex` updates, live brush preview mesh moves | **still broken, but for a NEW reason found by W-D2, not the popup**: `hover_suggestion_updates_the_brush_candidate_index_and_live_preview` now reaches the popup but fails on `candidates` being empty — the render path's precompute session never builds brush candidates (a precompute/render-warm-up gap, `🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:322-345`), not the transient/window-addressing bug. **Flagged to the precompute owner, unresolved** |
| Accept | click a candidate / `acceptSuggestion` | object placed at the accepted candidate; popup dismisses; selection moves to the new object | **popup dismiss was missing entirely** — `Puzzle3dAcceptSuggestionWork` never retired the popup on any terminal path. **Fixed in W-D2** (`dismissal()`/`take_ephemeral` added, `🦀️.rs:5624-5693`) — `accept_suggestion_closes_menu_even_when_placement_fails` passes. `open_and_accept_vortex_suggestions_preserve_active_utility` partially passes; its remaining failure is the same empty-`candidates` precompute gap as hover, above. **Same silent-failure pattern as `addBrushObject` (§9)**: `🎮️commands/✅️accept-suggestion/🦀️.rs:38` only handles `Ok(Fixture(_))`; a real placement error produces no visible feedback even though the menu correctly closes ("closes even when placement fails" is literally the test name — the failure itself is just invisible). |
| Close | click away / Escape / `closeVortexSuggestions` | popup clears | `close_vortex_suggestions_clears_the_menu`/`close_vortex_suggestions_clears_sticky_hover` pass (the latter flips flaky between runs, bucket (d), not a real regression) |
| Suggestions tick | automatic | `suggestionsTick` | popup candidate list refreshes while open (collision re-check) | real, `puzzle3d_suggestions_tick_scope` |

## 14. Engagement bar (command-line HUD)

Framework-generic chrome (`ShellHost/🟦️.tsx` engagement rendering; not separately re-audited in this
pass — locate by the "Actions" section under the active window's status line, `windowEngagementsByWindowId`,
carrying `WindowEngagementStatus{id:"puzzle3d-world-status", text:"<n> objects · <n> attractions"}`
as the status readout). Sub-verbs `"clear"`/`"rectangle"`/`"lasso"` were deliberately removed
(marquee/selection-method ownership moved to the framework `vortex` domain); `"fill <n>"`, `"brush"`,
`"zoom"` still parse.

| feature | dispatch | success criterion |
|---|---|---|
| Input | `engagementInput` | staged text buffer updates |
| Submit | `engagementSubmit` | parsed sub-verb executes (`fill <n>`, `brush`, `zoom`, etc.) |
| Repeat last | `engagementRepeatLast` | last submitted command re-runs |
| Abort | `engagementAbort` | in-flight engagement cancels |
| Control-select | `engagementControlSelect` | selection modified via typed control token |

All five listed real (feature-inventory §3); not re-verified against the WindowConfig/window-less-
`ViewModel` fixes in this pass — verify live.

**Confirmed dead sub-verbs (this pass):** `engagement_submit`'s parser (`🎮️commands/📨️engagement-submit/🦀️.rs:9-32`)
only recognizes `"fill <n>"` (activates fill + dispatches `setFillCount`), `"brush"` (activates the brush
utility), and `"zoom"` (focuses the current selection) — anything else is a silent no-op. But the
placeholder text actually shown in the input (`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:537`) reads
`"brush, fill <n>, zoom, clear, rectangle, lasso"` — **`clear`/`rectangle`/`lasso` are advertised but
were deliberately dropped** (selection-method ownership moved to the framework `vortex` domain per the
command's own doc comment). Typing any of those three and pressing Enter does nothing. `engagementAbort`
(Escape) does work — it clears input/candidate-index and resets the active utility, correctly forwarded
to the host via `Effect::SetActiveUtility` (`🦀️.rs:2896-2911`). `engagementRepeatLast` only does
anything when the fill utility is active (increments fill count by 1); it's a no-op otherwise, not a
generic "repeat whatever I last submitted."

## 15. Context menu (right-click)

Per selection kind, built by `puzzle3d_context_menu_items` (`🦀️.rs:2347-2419`). **Rows bypass the
registry-validated `Menu::action`/`Menu::action_args` builders** (`puzzle3d_context_menu_row` constructs
`ContextMenuItemSpec` directly, `registry` param unused for validation) — so a typo'd action id is
never caught at build time, only at dispatch.

| selection | rows (id → action) |
|---|---|
| Object(s) | `duplicate`→`duplicateSelection`, `select-same-kind`→`selectSameKindSelection`, `zoom`→**`zoomToSelection`**, group `hand`: `hide-show`→`setSelectionFlag{flag:"hidden"}`, `lock-unlock`→`setSelectionFlag{flag:"locked"}`; `delete`→`deleteSelection` |
| Vortex (single) | `suggest`→`openVortexSuggestions{fullId}`, `zoom`→**`zoomToSelection`**, `delete`→`deleteSelection` |
| Attraction | `delete`→`deleteAttraction{id}` |
| Target volume | group `targets`: `hide-show`/`lock-unlock`→`setTargetVolumeFlag`; `delete`→`deleteTargetVolume{id}` |
| Reference | `zoom`→**`zoomToSelection`**, `delete`→`deleteSelection` |

**New defect found in this audit pass (not in either named audit report):** `"zoomToSelection"` (used
for the object/vortex/reference "Zoom to Selection" row, `🦀️.rs:2358,2379,2414`) **is not a registered
action** — the `Puzzle3dCommand` enum / `TOOL_JOB_IDS` list (`🦀️.rs:2050-2185`) only declares
`"focusSelection"` (`Puzzle3dFocusSelectionWork`). Clicking "Zoom to Selection" from the context menu on
an object, vortex, or reference will dispatch an unknown action id and should fault at `dispatch_action`
(no handler matches `"zoomToSelection"` anywhere in the crate — confirmed by grep). **Verify this row
faults when clicked**; fix is either renaming the row's action string to `"focusSelection"` or adding a
`"zoomToSelection"` alias.

## 16. Inspection panel (`patchInspector`)

Body key `puzzle.3d.play.inspector`. Rewritten in **W-S**: per-granularity field groups replace the old
static schema/domain/count-only summary.

| state | ids | dispatch |
|---|---|---|
| No selection | `puzzle3d-play-inspector.empty` | — |
| Object selected | `puzzle3d-play-inspector.object.id`, `.object.origin`, `.object.hidden`, `.object.locked` | `patchInspector` |
| Vortex selected | `puzzle3d-play-inspector.vortex.full-id`, `.vortex.radius` | `patchInspector` |

**Status:** was permanently dead as a per-entity inspector (always the static summary — feature-inventory
TL;DR #4). **Fixed in W-S** — `selected_object_inspector_renders_that_object_field_group` /
`selected_vortex_inspector_renders_the_vortex_field_group` pass. Not runtime-confirmed (W-S §9 explicitly
flags "the inspection panel switching" as unobserved, derived from source only).

## 17. Artifact / outliner panel

Body key `puzzle.3d.play.document`, root `puzzle3d-play-document`. Four sections: objects (with nested
vortices), references, target volumes, attractions. Each row dispatches `interactionSelect` on click;
inline hide/lock icons dispatch `setSelectionFlag{entity, ids}` directly on the row without selecting
first. Memoized against the fixture's geometry fingerprint. Confirmed real and complete (feature-inventory
§2, "row-action wiring reads correctly" as of 09-07). Click-to-select through this panel was never part
of the render/hover gap (TL;DR #1) — it always worked via `interactionSelect`, unlike viewport clicks.

**High-confidence new defect found in this pass:** `📌️panels/🗿️artifact/🦀️.rs`'s `flag_args` helper
(lines 122-124), used by `object_row`/`reference_row`/`target_volume_row` (lines 149/166/174), **hardcodes
`("value", ui_value_bool(true))` regardless of current state** — the icon/label correctly alternate
("Hide"/"Show", "Lock"/"Unlock") based on `hidden`/`locked`, but every click **always sends
`setSelectionFlag{value:true}`**. Concretely: hiding a visible object via the outliner row works, but
clicking "Show" on an already-hidden row **re-sends `hidden:true` and the object stays hidden** — same
for Lock/Unlock. The identical action via the context menu (§15, `!all_hidden`/`!all_locked` negation,
`🦀️.rs:2360-2368`) and the inspection panel's flag row (§16, `!pressed`, `📌️panels/🔍️inspection/🦀️.rs:80`)
are both correct — only the outliner's own inline toggle is broken. **QA: hide an object via the
outliner row, then try to un-hide it via the SAME row action (expect it to silently fail); confirm the
context menu or inspection panel can undo it.**

## 18. Catalogue panel

Body key `puzzle.3d.play.kinds`, root `puzzle3d-play-kinds` (section keys e.g.
`puzzle3d-play-kinds.objects`). Object-kind rows are draggable (`PUZZLE3D_CATALOGUE_DRAG_MIME`) and
dispatch `addObjectKind` on click (`ActionFactory::new("puzzle3d-play").action("addObjectKind", …)`);
nested rim-vortex templates; read-only vortex/cable/attraction kind rows sourced from
`meta.kindCatalogs`. Paged/virtualized since W-O (`+N` continuation rows, fail-soft on wide catalogs).

| feature | success criterion | status |
|---|---|---|
| Add object of kind | click a kind row | new object appears in the document/outliner, selected? (duplicate/accept-suggestion gained re-select in W-S; addObjectKind's own re-select was not explicitly mentioned — verify) | real when `meta.kindCatalogs` exists; the empty-document no-op case is covered in §5 (fixed in W-D2) |
| Drag a kind onto the viewport | drag-drop | object placed at drop location | not separately audited here — verify |

## 19. Settings panel

Body key `puzzle.3d.play.settings`, root `puzzle3d-play-settings`. Four steppers, whole-session (not
per-window):

| id | label | dispatch |
|---|---|---|
| `puzzle3d-play-settings.overlap-budget` | brush overlap budget | `setBrushPlacementOverlapBudget` |
| `puzzle3d-play-settings.proximity-radius` | relocate proximity radius | `setProximityRadius` |
| `puzzle3d-play-settings.chunk-size` | viewport chunk size | `setChunkSize` |
| `puzzle3d-play-settings.grid-spacing` | grid spacing | `setGridSpacing` |

All real (feature-inventory §2). **Confirmed in this pass:** all four steppers publish through the
**`WindowConfig`** lane (`🦀️.rs:6317-6329`) — i.e. despite the panel being titled "Settings" with no
obvious per-window framing, `overlap-budget`/`proximity-radius`/`chunk-size`/`grid-spacing` are actually
**per-window state**, the same lane §2-§4 depend on (so also subject to the same hang bug, fixed in
W-D2). `setGridSpacing` here and the per-window grid option in §4 dispatch the identical action id from
two different UI locations onto what is architecturally the same per-window field — **verify in a
split-window test that changing it from Settings only affects the currently-focused window**, not both
panes at once (the panel not visually scoping to one window is a plausible UX confusion even once the
underlying lane works).

## 20. History panel (framework-generic — every app gets one)

Panel tab `framework.panel.history`. Section `framework.history.actions`:

| id | control | dispatch |
|---|---|---|
| `framework.history.undo` | button "Undo" | `undo` |
| `framework.history.redo` | button "Redo" | `redo` |
| `framework.history.checkpoint` | button "Checkpoint" (hidden for a viewer role) | `commitCheckpoint` |
| `framework.history.checkin` / `#s-checkin` | opens an inline message dialog (`#s-checkin-message` input, submit/cancel buttons) | `commitCheckpoint`-equivalent with a message (check-in) |

Section `framework.history.commands`: one row per applied edit, id `framework.history.entry.${seq}`,
revert control (↶) dispatches `revertToCommand {entrySeq}` when `entry.revertible`.

**Status — likely still broken on the React/wasm target as of the latest runtime doc entry (14:25):**
`undo`/`redo`/`commitCheckpoint`/`revertToCommand` are **framework-reserved routes**
(`run_framework_reserved_job`), not puzzle3d actions. The runtime doc's last entry found that EVERY
framework-reserved route hangs the actor on wasm — `resolve_ready`'s poll loop never pumps the
cooperative process pool, so a mounted worker step submitted by `pump_one` can never execute
(`noteShellCommand` polled `Submitted` 4096+ times inside one turn with no progress). A fix was written
(`run_framework_reserved_job` pumps the pool itself on wasm after each non-terminal poll) and a rebuild
(**#11**) was queued behind other work at 14:25 — **no later runtime-verification entry confirms this
rebuild ever landed or was tested**. **Treat Undo/Redo/Checkpoint/Check-in/Revert as the single most
likely feature to still hang the whole app** until proven otherwise; this is the top thing to click
first when driving the browser.

## 21. Copy / cut / paste

**Confirmed (this pass): no app-specific handler exists.** No `clipboard`/`copy`/`cut`/`paste` identifier
appears anywhere in `✏️editor/🦀️.rs` — only "Duplicate" (`duplicateSelection`, an offset-clone, not a
clipboard op — see §22). The `shell.clipboard` capability the plugin declares, if the shell exposes menu
items for it, is entirely framework-driven (`clipboard_action_definitions` lives only in the compiled
`semio_framework_plugin` rmeta, not reimplemented here) — puzzle3d does not customize or opt into
anything clipboard-specific. **QA: verify whether Copy/Cut/Paste even appear as options for this editor
at all**; if they do, they're the same framework-reserved-route class as history/undo (§20), so subject
to the same hang risk if routed through `run_framework_reserved_job`.

## 22. Delete / duplicate / focus selection

| feature | keybinding | dispatch | success criterion | status |
|---|---|---|---|---|
| Delete | Del / Backspace | `deleteSelection` (objects/vortices/references), `deleteAttraction{id}`, `deleteTargetVolume{id}` | selected entities removed (`🎮️commands/🗑️delete-selection/🦀️.rs:6-21` — one pass, incl. attractions touching a deleted object) | real (feature-inventory §3) |
| Duplicate | Ctrl/Cmd+D | `duplicateSelection` | clones created with new ids, offset `+0.5` x/y (`🎮️commands/👯️duplicate-selection/🦀️.rs:13-33`) | real; **re-select of the clones was missing** (cosmetic gap, same root as TL;DR #1) — **fixed in W-S** (`duplicate_selection_reselects_the_created_clones` test added) |
| Focus selection (camera zoom-to-selection) | F | `focusSelection` (the REAL action — see §15's `zoomToSelection` defect) | camera frames the selection, `ui_scope` forced to viewport-only (`🎮️commands/🎯️focus-selection/🦀️.rs:5-8`) | real (`Puzzle3dFocusSelectionWork`) — but only reachable today via a menu/hotkey that dispatches the CORRECT id; the context-menu "Zoom to Selection" row does not (§15) |

## 23. Add Object dialog

`openAddObjectDialog` → `Effect::OpenDialog{dialog_id:"addObject"}` (`🦀️.rs:2955,3138`), a
`HostOnly`-lane action, real (opens the shell's generic dialog chrome via `session.app.dialogs` /
`resolveDialogDefinition` in `ShellHost/🟦️.tsx`). The dialog itself drives the existing `addObjectKind`
action once submitted (`🦀️.rs:7496` comment). Same empty-document no-op history as §5/§18 — now fixed.

**High-confidence new defect found in this pass:** the dialog's own arg schema AND the standalone
`addObjectKind` action-arg schema (`🦀️.rs:7509-7511` and `7416-7418`) both hardcode a **single static
select option, `ActionArgOption::new("Object", …)`** — unlike the Catalogue panel (§18), which reads
`fixture.meta.kind_catalogs` dynamically, this dialog does **not** enumerate the live catalog. On any
document whose catalog has kinds beyond the literal id `"Object"` (both Concrete Forest and Nakagin
likely do), the dialog's dropdown will only ever offer "Object". **QA: open Add Object on a real
example and check whether the kind select shows more than one option** — if not, this dialog cannot add
most of the document's actual object kinds and the Catalogue panel is the only working "add" path.

## 24. Import / export

No import/export action id or command file found anywhere in the editor crate. `setFixtureJson` is the
closest thing to an "import" (load an arbitrary fixture JSON) but is **`BatchOnlyPendingRewrite`** —
dead at the UI dispatch gate — and even if migrated, Nakagin's DSL (128,755 bytes) exceeds the shared
8,192-byte `PUZZLE_COMMAND_RAW_BYTES` wire cap, guaranteeing a `RawWireLimit` rejection
(feature-inventory TL;DR #6). `setActiveExample` (server-resolved by id) is the only working substitute.
**Conclusion: import/export does not exist as a usable end-user feature.** Do not expect an
Import/Export menu item to do anything.

## 25. Locale / terminology (English / German, no default)

Full EN+DE label set, compile-checked (`app_labels!`). `document_and_kinds_trees_use_german_reuse_section_labels`
fails (`assert!(document_json.contains("Baukomponenten"))`) — root cause not isolated in the audit;
could be a genuine locale-resolution gap in this specific render path, or the same bare-`ViewModel`
test-omission class as §4's stale tests. **Verify by actually switching the shell to German and reading
the document tree section labels**, since source-level evidence is ambiguous.

---

## Summary — priority order for a first browser pass

1. **History panel (undo/redo/checkpoint/revert)** — likely still hangs the actor on wasm; rebuild #11's
   fix was never runtime-confirmed (§20).
2. **Any window option (camera/projection/grid/LOD/vortex/sun)** — was a 100%-repeatable infinite hang;
   fixed in source (W-D2) but never runtime-confirmed (§2, §3, §4).
3. **Fill tool activation** (`tool.fill`) and **any utility** (Transform/Brush/Volume Brush/Relocate)
   activation — was fully dead (`interactive-job.missing-owned-reducer`); fixed by a coordinator patch,
   never runtime-confirmed (§12).
4. **Nakagin example load** — was hard-blocked by the ledger ceiling; fixed by W-B batching, never
   runtime-confirmed end-to-end in a browser (§5).
5. **`addObjectKind` on an empty/cleared document** — was a silent no-op; fixed in W-D2 (§5, §18).
6. **Vortex suggestion popup** open/close/accept — fixed in W-D2; **hover-to-preview candidates** still
   genuinely broken (empty candidates, precompute gap) (§13).
7. **Context menu "Zoom to Selection"** — dispatches an unregistered action id (`zoomToSelection` vs the
   real `focusSelection`); found independently by both this pass's direct reading AND the dedicated
   Rust-registry research pass (cross-confirmed, high confidence). Expect a visible no-op/fault on click
   for object, vortex, and reference selections (§15, §22).
8. **Outliner panel hide/lock buttons never toggle back off** — `📌️panels/🗿️artifact/🦀️.rs:122-124`
   hardcodes `value:true` on every click regardless of current state; "Show"/"Unlock" on an
   already-hidden/locked row silently re-applies `true` and nothing changes. Works correctly via the
   context menu and inspection panel. High-confidence, newly found in this pass (§17).
9. **Add Object dialog's kind selector is hardcoded to one static option** (`"Object"`), not populated
   from the live catalog the way the Catalogue panel is — likely cannot add most real object kinds
   through this dialog on either example. High-confidence, newly found in this pass (§23).
10. **Engagement bar advertises `clear`/`rectangle`/`lasso` in its placeholder text but silently drops
    them** — only `fill <n>`/`brush`/`zoom` actually parse (§14).
11. **Gumball drag** — CORRECTED in this pass: `transformBegin`/`transformEnd` are deliberately empty
    host-only brackets by design (one absolute delta commits on release via
    `translateSelection`/`rotateSelection`/`scaleSelection`), not a dead/blocked feature as the stale
    2026-09-08 doc implied. Verify the live mid-drag preview and the commit, but don't expect the
    begin/end brackets themselves to do anything (§8).
12. **Fill cancel button** — exists in current source (unlike the 09-08 design doc's "no affordance at
    all" claim) but has no test or runtime evidence it actually cancels a job (§12).
13. **Silent engine-failure swallowing** in `addBrushObject` and `acceptSuggestion` — a real placement
    failure (e.g. collision/overlap-budget) produces no visible error, just nothing happening (§9, §13).
14. **Copy/cut/paste** — no puzzle3d-specific implementation located; likely generic-shell or
    nonexistent. **Select All** DOES exist as a framework-generic action (`SELECT_ALL_ACTION_ID`) —
    correcting this checklist's earlier draft guess that it might not exist (§6, §21).

No claim in this checklist above the word "confirmed"/"observed" is a runtime observation from THIS
audit — everything is `cargo check`/`cargo test`/direct source reading, per the ticket's own read-only,
no-cargo, no-server constraint. The coordinator driving the browser is the first party to actually see
any of this render.
