# 🧩️ Puzzle 3d — user-perspective feature inventory (2026-09-08)

Read-only audit of `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` against
current source (no build run — machine load). Every claim below is grounded in an actual read of the
reducer arm / command-file body / struct field, not in a name or a doc comment alone; doc comments that
turned out to be stale are called out explicitly.

## TL;DR — user-visible features that are currently dead or degraded, ranked

1. **Nothing the user selects or hovers is visible to the app at render time — this is the single
   biggest defect and it cascades into six other symptoms below.** `ArtifactApp::render` (and
   `context_menu`) never gained an `InteractionView` parameter (framework gap, ticket
   `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`), so every render-time function that used to read
   live selection/hover is now hardcoded: `gumball_active()` → `false` always
   (`✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:114-116`), `world_selection_json()` emits
   permanently-empty `targetVolumeIds`/`vortexIds` and no hover paint (`🦀️.rs:425-459`), and
   `puzzle3d_brush_target_vortex()` is a one-line `None` regardless of its argument (`editor/🦀️.rs:674-676`).
   **Fix direction**: thread `InteractionView` through `ArtifactApp::render`/`context_menu`
   framework-wide (out of this crate — flagged to the coordinator in the code itself); until then none of
   items 2-6 can be fixed locally.
2. **The transform gumball never renders and dragging it does nothing.** Two independent breaks stack:
   (a) `gumball_active()` always `false` (item 1), so the viewport never draws the handle even though the
   Move/Rotate toggle options (`setTransformGumballFlag`) work and persist; (b) `transformBegin`/
   `transformEnd` are `BatchOnlyPendingRewrite` — blocked at the UI dispatch gate before they ever run, and
   even invoked directly they only call `self.begin_transform_session(...)`/`commit_transform(...)` on
   ephemeral `RefCell` state with no `Puzzle3dConfig` field to persist it (`editor/🦀️.rs:2348-2361`, confirmed
   current). **Fix direction**: (1) is the framework blocker; separately, add a `transform_session` field to
   `Puzzle3dConfig` so a drag survives a tick, then migrate the two actions.
3. **"Select Same Kind" does nothing.** `select_same_kind()` computes the clicked object's kind, then
   discards it (`let _ = kind;`) and unconditionally sets `ctx.abort = true`
   (`🎮️commands/🧬️select-same-kind/🦀️.rs:949-960`) — the action is `Migrated`, wired, and reachable, but
   its own doc comment confirms it can no longer write a selection back (item 1) and the command was
   reduced to a validation-only stub. **Fix direction**: needs a selection-write channel from the app layer
   (same framework gap as item 1) before this can select anything again.
4. **The Inspection panel never shows what's selected.** It always renders the static document summary
   (schema/domain/object count) — the per-selected-entity field editor it used to have was removed because
   `render` has no live selection (item 1); `patch_inspector`'s own doc comment now says "every real caller
   today passes `ids` explicitly" because the panel that would supply them from a click no longer exists
   (`📌️panels/🔍️inspection/🦀️.rs:28-34`, `🎮️commands/🩹️patch-inspector/🦀️.rs:1099-1102`). The backend
   action (`patchInspector`) is real; there is currently no UI path that drives it for a specific selection.
   **Fix direction**: same framework gap as item 1, then restore per-entity field groups.
5. **Marquee rectangle/lasso method and merge-mode are not renderable window options anymore** — moved to
   the framework `vortex` interaction domain's own args, with "no longer renderable here" stated directly
   in the `select` option's doc comment (`🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs:3-7`). The 5 cursor plans
   targeting marquee/selection UX (`fix_puzzle3d_marquee_selection`, `puzzle_3d_marquee_selection`,
   `puzzle_3d_selection_overhaul`, `_lag`, `_perf`, `_freeze`/`_commit_freeze`) all target a since-replaced
   TypeScript host (see §6) — none of their fixes exist in current source, and the underlying symptom
   (marquee UX) has a different concrete cause today. **Fix direction**: same framework gap as item 1 for
   method/merge; independently verify marquee still exists at all in the current generic World3d host.
6. **`setFixtureJson` cannot load any real fixture from the UI.** Blocked twice: it is
   `BatchOnlyPendingRewrite` (dead at the UI dispatch gate), and even if migrated, Nakagin's DSL is 128,755
   bytes against the shared `PUZZLE_COMMAND_RAW_BYTES = 8,192` cap
   (`🎮️commands/🧵️retained/🦀️.rs:10`) — a guaranteed `RawWireLimit` rejection. The command body itself is
   real (`🎮️commands/🧪️set-fixture-json/🦀️.rs`). **Fix direction**: needs a chunked/resumable wire path, not
   a classification flip; `setActiveExample` (which resolves fixtures by id server-side) is the working
   substitute today.
7. **`worldRelocate`, `createAttraction`, `acceptSuggestion`, `patchInspector` fault on the Nakagin example
   (180 objects) but work fine on the default Concrete-Forest example (1 object).** All four hit
   `Work::extent()` bounds against `PUZZLE_COMMAND_WORK_ITEMS = 4,096`; `worldRelocate`'s formula
   (`objects × 66 + attractions`, `editor/🦀️.rs:4202-4214`) dies above 62 objects — 2.9× over budget on
   Nakagin. Pre-dates this migration and is not a regression from it, but it also isn't delivered.
   **Fix direction**: tighten the bound (count actual vortices per object instead of assuming 64), not raise
   the cap — `📓️extent-tightening-report.md` in this ticket already scoped this.
8. **Multiplayer presence advertises selection/hover it doesn't carry.** `Puzzle3dPresence`'s own doc
   comment says "shareable live subset of puzzle view state (selection, hover, camera, active utility)"
   but the struct has only `camera_position`/`camera_target`/`camera_zoom`/`active_utility_id`/
   `active_tool_id` — no selection or hover field at all (`👥️presence/🦀️.rs:7-18`). A collaborator's cursor
   position and pick never reach you. **Fix direction**: add the fields once item 1's channel exists to
   source them from.
9. **The Brush utility's "Placement" candidate picker never appears in Utility Options** — gated on
   `puzzle3d_brush_target_vortex(envelope)` at render time, which is the same hardcoded `None` as item 1
   (`🪛️utilities/🖌️brush/🦀️.rs:39-41`). Brush placement via an explicit click/right-click path (through
   `Puzzle3dActionCtx::selected_vortex_ids()`, a different, working channel) still functions; only the
   passive hover-driven picker widget is dead.

Everything else declared `Migrated` (57 of 60) has a real, non-stub implementation reachable from the UI —
see §4 for the full table.

---

## 1. Windows

Edit mode (`🎭️modes/✏️edit/🦀️.rs`) opens exactly **one window kind**, `puzzle3d-main`
(`🪟️windows/🧊️main/🦀️.rs:30`), as **two instances** in a fixed row layout: `puzzle3d-main-top` (left ⅓,
orthographic, `TEMPLATE_TOP`) and `puzzle3d-main-perspective` (right ⅔, three-point/50° FOV,
`TEMPLATE_PERSPECTIVE`). Both instances view the **same document** — `Puzzle3dScene.fixture` is global,
not per-window — so there is no "two different examples side by side" mode; a scope note in this ticket
(`📓️findings-2026-09-05.md` §4) already flagged this to the dev as a design question, not a bug.

**Per-window options** (`☑️options/*`, materialized per instance via `load_window`/`save_window` in
`🎚️config/🦀️.rs`):
- `🌐️grid` — visible toggle, snap toggle, spacing slider (`setGridVisible`/`setGridSnapEnabled`/`setGridSpacing`, all real).
- `🔭️lod` — automatic-zoom toggle, depth-variable toggle, manual slider 0-1000 (`setLodAutomatic`/`setLodDepthVariable`/`setLodManual`, all real).
- `🎥️projection` — delegated wholesale to the framework's `world3d_projection_measures` (orthographic/1-2-3-point + cardinal/free orientation), real.
- `🎯️select` — selectable-kind toggles (objects/vortices/attractions, real, `setSelectableKind`); the marquee method and default-merge-mode controls that used to live here are gone (TL;DR #5).
- `☀️sun` — delegated to `world3d_sun_measures` (enabled/azimuth/elevation/intensity), real.
- `🌀️vortex` — show mode (Always / Selected) and direction (Outwards / Inwards) selects. **"Selected" mode is unreachable**: `object_vortices_visible()` only checks `runtime.vortex_show == PUZZLE3D_VORTEX_SHOW_ALWAYS` (`main/🦀️.rs:232-234`), so choosing "Selected" silently shows nothing extra (no crash, just a dead option value — same root cause as TL;DR #1).

**Scene rendering** (`main::render`, `main/🦀️.rs:466-486`) — everything below is populated and wired, confirmed by reading `render()`: objects/meshes (`world_instances_geometry_json`/`world_meshes_json`), vortices (`world_vortices_json`, gated by the show-mode above), attractions (`world_attractions_json`), target volumes (`world_target_volumes_json`), reference image planes (`world_references_json`), fill preview (`world_fill_preview_json`, real, ties to the fill precompute session), brush preview (`world_brush_preview_json`, only populates when a `suggestion_menu` is open — the passive-hover ghost is dead per TL;DR #9), LOD/chunking/environment JSON. **Not populated**: the gumball (`gumballActive` hardcoded `false`) and any live selection/hover overlay (`targetVolumeIds`/`vortexIds` hardcoded empty) — TL;DR #1/#2. A marquee rectangle overlay is not emitted by this app at all (it would be host/framework chrome, out of scope for this file).

## 2. Panels

- **⚙️Settings** (`📌️panels/⚙️settings/🦀️.rs`) — four number steppers: brush overlap budget, relocate
  proximity radius, viewport chunk size, grid spacing. All dispatch real scalar-config actions
  (`setBrushPlacementOverlapBudget`/`setProximityRadius`/`setChunkSize`/`setGridSpacing`). Whole-session
  settings, not per-window.
- **🔍️Inspection** (`📌️panels/🔍️inspection/🦀️.rs`) — dead as a per-entity inspector (TL;DR #4); always
  renders schema/domain/object-count only.
- **🗿️Artifact / document tree** (`📌️panels/🗿️artifact/🦀️.rs`) — real and complete: four sections
  (objects with nested vortices, references, target volumes, attractions), each row dispatches
  `interactionSelect` on activate and carries inline hide/lock row actions
  (`setSelectionFlag` with explicit `{entity, ids}`). Memoized against the fixture's geometry fingerprint.
  (Findings §16 records a prior `rowActions` panel crash here — confirmed FIXED as of the 09-07 session;
  current source's row-action wiring reads correctly.)
- **🛍️Catalogue** (`📌️panels/🛍️catalogue/🦀️.rs`) — real: object-kind rows (draggable via
  `PUZZLE3D_CATALOGUE_DRAG_MIME`, nested rim-vortex templates, dispatches `addObjectKind` on click) plus
  read-only vortex/cable/attraction kind rows from `meta.kindCatalogs`.

## 3. Tools & utilities

- **🪣️Fill** (mode-level tool, `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`) — count slider (`setFillCount`,
  bounded/resumable via `set_fill_count::begin`/`step`, real chunked placement with a reveal-cutoff so the
  viewport shows/hides already-planned pieces client-side) plus a distribution-weight tree
  (`setObjectKindWeight`/`setVortexKindWeight`). `fillBuildTick` (120ms host tick) drives the background
  precompute job. All real.
- **🖌️Brush** utility — hover a vortex, cycle collision-free candidates, click to dock
  (`addBrushObject`/`cycleBrushCandidate(Back)`/`registerBrushMesh` for GLB collision geometry). Real when
  driven by an explicit click/context-menu vortex id; the passive hover-driven "Placement" picker widget in
  Utility Options is dead (TL;DR #9).
- **🧊️Volume Brush** utility — Alt+click paints grid-snapped target volumes (`addTargetVolume`, real,
  snaps to the window's grid spacing) that constrain Fill; its only options are w/d/h voxel-dimension
  sliders (`setVoxelDims`, real).
- **🔄️Transform gumball** — Move/Rotate flag toggles are real and persist
  (`setTransformGumballFlag`); the gumball itself never renders and drag begin/end are no-ops (TL;DR #2).
  Programmatic translate/rotate/scale (e.g. via the engagement command line or a future non-drag caller)
  still works: `translateSelection`/`rotateSelection`/`scaleSelection` are real, backed by
  `Puzzle3dScaleWork` with proper mesh-selection-id resolution and attraction re-derivation.
- **🚚️World Relocate** utility — drag-drop an object to an absolute position and auto-attract nearby
  compatible vortices within `proximity_radius` (`worldRelocate`, real business logic — see TL;DR #7 for
  its extent-bound ceiling on large fixtures).
- **Marquee / select-same-kind / focus** — marquee method rendering is gone (TL;DR #5);
  `selectSameKindSelection` is wired but behaviorally a no-op (TL;DR #3); `focusSelection` (camera
  zoom-to-selection) is real (`Puzzle3dFocusSelectionWork`).
- **Duplicate / delete** — both real (`duplicateSelection`, `deleteSelection`,
  `deleteAttraction`/`deleteTargetVolume`); duplicate no longer re-selects its new copies (selection-write
  gap, same root cause as TL;DR #1, cosmetic only — the document-side duplicate itself is correct).
- **Suggestions** (vortex context menu) — open/close/hover/cycle/accept all real
  (`openVortexSuggestions`/`closeVortexSuggestions`/`hoverSuggestion`/`cycleBrushCandidate(Back)`/
  `acceptSuggestion`); accept and open no longer auto-select the result (same cosmetic selection gap).
- **Engagement** (command-line HUD: input/submit/repeat-last/abort/control-select) — all real
  (`engagementInput`/`engagementSubmit`/`engagementRepeatLast`/`engagementAbort`/
  `engagementControlSelect`). `engagementSubmit`'s `"clear"`/`"rectangle"`/`"lasso"` text sub-verbs were
  explicitly dropped (dead code path removed, not silently broken) because their target (selection method)
  is framework-owned now; `"fill <n>"`, `"brush"`, `"zoom"` still work.
- **Snapping** — grid snap toggle real (`setGridSnapEnabled`), and both `addTargetVolume` and
  `worldRelocate`'s auto-attract honor grid spacing / proximity radius.
- **Target volumes** — add (real, grid-snapped), delete, relocate (gumball-driven absolute pose push,
  real), flag (hidden/locked) — all real.
- **Examples** — `setActiveExample` real (concrete-forest / nakagin / empty, resolved server-side by id);
  `setFixtureJson` dead at the UI (TL;DR #6).
- **Camera/projection** — unified through the shared `world3d_projection_measures`/`setCamera`/
  `setProjection`/`setProjectionParam`, real.
- **Locale/terminology** (`🗣️terminology/🦀️.rs`) — full English + German label set, compile-checked via
  `semio_framework_plugin::app_labels!` per ticket `26/08/03/COMPILE-TIME-CHECKED-UI-LABELS…`. No default
  language, both first-class, per CLAUDE.md.
- **Inspector patch** — real mutation logic (`patchInspector`/`apply_puzzle3d_inspector_patch`); no UI
  panel currently drives it for a live selection (TL;DR #4).
- **Kind weights / selectable kinds / depth variable / proximity radius / chunk size / voxel dims /
  spacing / manual / automatic / active / visible** — all real scalar-config actions, verified by reading
  every one of their command-file bodies (§4).

## 4. Every declared action — classification, lanes, Work type, real vs stub

Extracted programmatically from `editor/🦀️.rs`'s `.action_interactive_job(...)` declarations (line
7322-7384), `PUZZLE3D_RETAINED_TOOL_IDS` (2526-2531), `PUBLICATION_CONTRACTS` (6213-6274), and the
`build_tool_job` `match tool_id` (6733-6785), cross-checked against every command-file body under
`🎮️commands/*` and the two callers of `dispatch_puzzle3d_action` (`handle_action_impl` at 2342, itself
called from `puzzle3d_retained_reduce` at 2549 and the legacy `ArtifactEditor::handle` at 6836).

**Totals**: 63 declared actions = **60 `Migrated`** + **3 `BatchOnlyPendingRewrite`**. The three sets
(`Migrated` declarations, `PUZZLE3D_RETAINED_TOOL_IDS`, `PUBLICATION_CONTRACTS` tool ids) are set-equal —
**fully reconciled, no drift**, an improvement over the ticket's last recorded snapshot
(`📓️status.md` "Final migration state": 64 Migrated / 3 BatchOnlyPendingRewrite = 67 total, "62 of 65
admitted") — one prior discrepancy has since been cleaned up. Of the 60 Migrated, **59 have a real,
document/config-mutating implementation**; **1 (`selectSameKindSelection`) is wired and reachable but its
command body is a no-op by construction**. All 3 `BatchOnlyPendingRewrite` are blocked at the UI dispatch
gate (`validate_ui_dispatch_classification` admits only `Migrated`) regardless of what their code does in
isolation.

| Action id | Classification | Lanes | Work type | Real / stub | Feature |
|---|---|---|---|---|---|
| `openAddObjectDialog` | Migrated | HostOnly | `NoopPuzzleCommandWork`* | Real (shell effect) | Opens the "Add Object" dialog (`Effect::OpenDialog`) |
| `worldPointerDown` | Migrated | HostOnly | `NoopPuzzleCommandWork` | No-op by design | Host-side pick passthrough, not app state |
| `setActiveExample` | Migrated | Artifact, Config | `Puzzle3dSetActiveExampleWork` | Real | Switch example (concrete-forest/nakagin/empty) |
| `setFillCount` | Migrated | Config | `Puzzle3dPrecomputeCommandWork` | Real | Fill tool count slider (begin) |
| `setFillCountStep` | Migrated | Artifact, Config | `Puzzle3dPrecomputeCommandWork` | Real | Fill tool chunked build step (was a documented no-op risk 09-05, now fixed — see §5) |
| `addTargetVolume` | Migrated | Artifact | `BoundedFirstStepCommandWork`→special-cased in `puzzle3d_retained_reduce` | Real | Volume Brush: paint a target volume |
| `acceptSuggestion` | Migrated | Artifact, Config | `Puzzle3dAcceptSuggestionWork` | Real | Accept hovered/indexed brush suggestion |
| `addBrushObject` | Migrated | Artifact, Config | `Puzzle3dAddBrushObjectWork` | Real | Brush: place explicit payload |
| `addObjectKind` | Migrated | Artifact, Config | `Puzzle3dAddObjectKindWork` | Real | Catalogue: add object of kind |
| `createAttraction` | Migrated | Artifact, Config | `Puzzle3dCreateAttractionWork` | Real (extent-bound on Nakagin, TL;DR #7) | Connect two vortices |
| `deleteAttraction` | Migrated | Artifact, Config | `BoundedFirstStepCommandWork`→`dispatch_puzzle3d_action` | Real | Delete one attraction |
| `deleteSelection` | Migrated | Artifact, Config | `BoundedFirstStepCommandWork` | Real | Delete selected objects/vortices/attractions/volumes/refs |
| `deleteTargetVolume` | Migrated | Artifact, Config | `BoundedFirstStepCommandWork` | Real | Delete a target volume |
| `duplicateSelection` | Migrated | Artifact, Config | `BoundedFirstStepCommandWork` | Real (no reselect, cosmetic) | Duplicate selected objects |
| `patchInspector` | Migrated | Artifact, Config | `Puzzle3dPatchInspectorWork` | Real (extent-bound on Nakagin, TL;DR #7) | Edit an entity field (no live UI caller, TL;DR #4) |
| `rotateSelection` | Migrated | Artifact, Config | `Puzzle3dScaleWork` | Real | Gumball rotate |
| `scaleSelection` | Migrated | Artifact, Config | `Puzzle3dScaleWork` | Real | Gumball scale |
| `setSelectionFlag` | Migrated | Artifact, Config | `BoundedFirstStepCommandWork` | Real | Hide/lock toggle (row actions + context menu) |
| `setTargetVolumeFlag` | Migrated | Artifact, Config | `BoundedFirstStepCommandWork` | Real | Hide/lock a target volume |
| `translateSelection` | Migrated | Artifact, Config | `Puzzle3dScaleWork` | Real | Gumball translate |
| `worldRelocate` | Migrated | Artifact, Config | `Puzzle3dWorldRelocateWork` | Real (extent-bound on Nakagin, TL;DR #7) | World Relocate utility |
| `closeVortexSuggestions` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Dismiss suggestion popup |
| `cycleBrushCandidate` | Migrated | Config | `Puzzle3dPrecomputeCommandWork` | Real | Next brush candidate |
| `cycleBrushCandidateBack` | Migrated | Config | `Puzzle3dPrecomputeCommandWork` | Real | Previous brush candidate |
| `engagementAbort` | Migrated | Config | `Puzzle3dEngagementAbortWork` | Real | Abort engagement HUD input |
| `engagementControlSelect` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Brush placement picker select |
| `engagementInput` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Engagement text input change |
| `engagementRepeatLast` | Migrated | Config | `Puzzle3dEngagementRepeatWork` | Real | Repeat last fill request |
| `engagementSubmit` | Migrated | Config | `Puzzle3dEngagementSubmitWork` | Real (`clear`/`rectangle`/`lasso` sub-verbs intentionally dropped) | Engagement text command submit |
| `fillBuildTick` | Migrated | Config | `Puzzle3dPrecomputeCommandWork` | Real | 120ms fill background-job driver |
| `focusSelection` | Migrated | Config | `Puzzle3dFocusSelectionWork` | Real | Zoom camera to selection |
| `hoverSuggestion` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Hover a suggestion candidate |
| `openVortexSuggestions` | Migrated | Config | `BoundedFirstStepCommandWork` | Real (no auto-select of target, cosmetic) | Open suggestion popup |
| `registerBrushMesh` | Migrated | Config | `Puzzle3dPrecomputeCommandWork` | Real | Register GLB collision mesh |
| `relocateTargetVolume` | Migrated | Artifact | `Puzzle3dRelocateVolumeWork` | Real | Gumball-drag a target volume |
| `selectSameKindSelection` | Migrated | Config | `BoundedFirstStepCommandWork` | **Stub by construction** (TL;DR #3) | "Select Same Kind" |
| `setBrushPlacementOverlapBudget` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Brush/fill collision budget |
| `setCamera` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Set camera pose |
| `setChunkSize` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Viewport chunk size |
| `setGridSnapEnabled` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Grid snap toggle |
| `setGridSpacing` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Grid spacing |
| `setGridVisible` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Grid visibility |
| `setLodAutomatic` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | LOD auto-zoom toggle |
| `setLodDepthVariable` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | LOD depth-variable toggle |
| `setLodManual` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | LOD manual slider |
| `setObjectKindWeight` | Migrated | Config | `Puzzle3dKindWeightWork` | Real | Fill distribution: object weight |
| `setProjection` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Camera projection type |
| `setProjectionParam` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Camera projection parameter |
| `setProximityRadius` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | World Relocate proximity radius |
| `setSelectableKind` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Which kinds a pick may reach |
| `setSunAzimuth` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Sun azimuth |
| `setSunElevation` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Sun elevation |
| `setSunIntensity` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Sun intensity |
| `setTransformGumballFlag` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real (config only — gumball itself dead, TL;DR #2) | Gumball Move/Rotate flags |
| `setVortexDirection` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Vortex arrow direction display |
| `setVortexKindWeight` | Migrated | Config | `Puzzle3dKindWeightWork` | Real | Fill distribution: vortex weight |
| `setVortexShow` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real (config only — "Selected" mode unreachable, §1) | Vortex marker show mode |
| `setVoxelDims` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Volume Brush voxel dimensions |
| `suggestionsTick` | Migrated | Config | `Puzzle3dPrecomputeCommandWork` | Real | 120ms suggestion-lane driver |
| `toggleSun` | Migrated | Config | `Puzzle3dScalarConfigWork` | Real | Sun enabled toggle |
| `setFixtureJson` | **BatchOnlyPendingRewrite** | — (not admitted) | n/a (not in `build_tool_job`) | Real body, **dead at UI + would exceed wire cap** (TL;DR #6) | Load a fixture from raw JSON |
| `transformBegin` | **BatchOnlyPendingRewrite** | — (not admitted) | `NoopPuzzleCommandWork` (dead arm — never reached, id not in retained list) | **Dead at UI**; ephemeral-only when called directly (TL;DR #2) | Gumball drag start |
| `transformEnd` | **BatchOnlyPendingRewrite** | — (not admitted) | `NoopPuzzleCommandWork` (dead arm) | **Dead at UI**; ephemeral-only when called directly (TL;DR #2) | Gumball drag commit |

\* `openAddObjectDialog`/`worldPointerDown`/`addTargetVolume` are special-cased at the top of
`puzzle3d_retained_reduce` (2556-2572) before the generic dispatch even runs; their `build_tool_job` Work
type entry is present but effectively bypassed for these three ids specifically.

## 5. Structural holdbacks — re-verified against current source (2026-09-08)

All four items from `📓️status.md` "Final migration state" and `📓️findings-2026-09-05.md` §3/§5/§7 were
re-checked directly against today's `editor/🦀️.rs`, not against the ticket notes:

- **`transformBegin`/`transformEnd`** — still exactly as described: `EditorApp<Puzzle3dPlayApp>` has no
  `Puzzle3dConfig` field bridging the gumball drag session (only an in-process `RefCell` on the app, which
  a fresh WASM/job invocation would not share), so `handle_action_impl`'s arms for them
  (`editor/🦀️.rs:2348-2361`) begin/commit an ephemeral session and return `Emit::default()` — no persisted
  mutation either way. **Still genuinely un-migratable without a config-level session field.** Confirmed
  current, unchanged from 09-05. **User-visible break**: TL;DR #2. **Clean fix**: add
  `transform_session: Option<TransformSession>` (or similar) to `Puzzle3dConfig`, serialize drag-start
  pose/ids into it on `transformBegin`, read it back on `transformEnd`.
- **`setFixtureJson`** — still blocked by the shared 8,192-byte `PUZZLE_COMMAND_RAW_BYTES` cap
  (`🎮️commands/🧵️retained/🦀️.rs:10`) against a 128,755-byte Nakagin DSL payload. Confirmed current — the
  command body (`🎮️commands/🧪️set-fixture-json/🦀️.rs`) is unchanged and still real; only the transport is
  the blocker. **User-visible break**: TL;DR #6. **Clean fix**: a chunked/resumable wire path (the
  `ToolExecutionContract::resumable(...)` already used for retained commands supports multi-step payloads
  in principle; `setFixtureJson` needs its own multi-chunk assembly rather than one raw blob).
- **`worldRelocate`/`createAttraction`/`acceptSuggestion`/`patchInspector`** — all four still self-reject
  above `PUZZLE_COMMAND_WORK_ITEMS = 4,096` via their `extent()` bodies. Re-measured `worldRelocate`'s
  formula directly from current source (`editor/🦀️.rs:4202-4214`):
  `object_stage + existing_attraction_stage + candidate_dispatch_stage + candidate_scan_stage`, which
  reduces to roughly `objects × (2 + 2×avg_vortices_per_object) + attractions` — dies above ~62 objects on
  a typical few-vortex-per-object fixture. Nakagin has 180 object instances (recounted directly from the
  DSL source), ~2.9× over budget. **Not a regression** — these bodies predate the classification migration
  and were simply unreachable before it (dead behind `interactive-job.not-ui-safe`); the migration makes
  them reachable-but-still-bounded, which is progress, not new breakage. **Default example is unaffected**
  (`default_fixture()` returns the 1-object Concrete-Forest fixture) — the fault only appears after
  switching to Nakagin. **User-visible break**: TL;DR #7. **Clean fix**: tighten each `extent()` to count
  actual per-object vortices instead of the flat `× 64`/`× 66` assumption real objects never reach — the
  ticket's own `📓️extent-tightening-report.md` already scoped this per-action.
- **`setFillCountStep`** (findings §3's documented risk) — **this one has changed since 09-05, for the
  better.** The 09-05 note predicted a silent no-op because it had no explicit `build_tool_job` arm and
  would fall to `dispatch_puzzle3d_action`'s `_ => {}` default. Current source has it explicitly mapped to
  `Puzzle3dPrecomputeCommandWork::new(tool_id)` (`editor/🦀️.rs:6764`, matched together with `fillBuildTick`/
  `registerBrushMesh`/`setFillCount`/`suggestionsTick`/`cycleBrushCandidate(Back)`), and the legacy
  `ArtifactEditor::handle` path (6836-6864) special-cases it explicitly, calling the real
  `set_fill_count::step(...)` (`🎮️commands/🧮️set-fill-count/🦀️.rs:1022-1050`, a genuine chunked
  placement stepper with generation-checked continuation). **Confirmed real and working, not a stub** —
  the semantic wave mentioned in the findings closed this gap.

## 6. Comparison with puzzle 5d, `💠️lowpoly`, and the `.cursor/plans` history

**Migration maturity, measured the same way for all three** (declared `action_interactive_job`
classifications, current source):

| App | Migrated | Other | % Migrated |
|---|---|---|---|
| `💠️lowpoly` (reference implementation) | 46 | 0 | **100%** |
| `🧊️puzzle3d` | 60 | 3 | **95%** |
| `🖐️puzzle5d` | 9 | 41 | **18%** |

Puzzle3d is close behind the reference implementation and dramatically ahead of its sibling 5d editor —
5d has 41 of 50 actions still `BatchOnlyPendingRewrite`, including `deleteAttraction`, `translateSelection`,
`rotateSelection`, `patchPart`, `patchFastener`, `createFastener`, `worldRelocate`, and every camera/LOD/grid
scalar-config action. 5d's `setFixtureJson` IS migrated (unlike 3d's) — worth flagging as a **latent bug**,
not a feature lead: 5d fixtures may simply be small enough today that nobody has hit the same
`PUZZLE_COMMAND_RAW_BYTES` wall yet, not evidence 5d solved the wire-size problem.

Camera commands: 5d has three separate action ids (`🎥️set-camera`, `🖼️set-camera-2d`, `🎦️set-camera-3d`)
plus a dedicated `🔄️rotate-selection` (not `rotate-part3d` specifically — no exact match for that id in
current 5d source). Puzzle3d instead unifies camera/projection into one `setCamera`/`setProjection`/
`setProjectionParam` triple backed by the shared `world3d_projection_measures`/`world3d_sun_measures`
framework helpers. This reads as 3d having made a **cleaner, more consolidated** design choice rather than
a capability gap — 3d does not need per-projection-type action ids because the shared helper already
handles every projection mode through one surface.

**The `render`-time selection/hover gap (TL;DR #1) is a framework-wide limitation, not puzzle3d-specific**:
lowpoly's own render-adjacent signature at `editor/🦀️.rs:1763` also takes an unused `_interaction:
&InteractionView<'_>` — confirmed present as a parameter (unlike puzzle3d's `render`, which has no such
parameter at all) but unused at that call site too. So even the 100%-migrated reference implementation has
not fully exploited live interaction state in render; puzzle3d is blocked on the same framework plumbing
gap documented in ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`, not on anything unique to
this crate.

**`.cursor/plans` — all 16 puzzle3d/puzzle_3d plans target a superseded architecture.** Every referenced
file path (`puzzle/3d/react/index.tsx`, `puzzle/3d/play/index.ts`, `puzzle/3d/rs/lib.rs`,
`framework/product/playground/renderer/react/index.tsx`) was checked against the current tree and **does
not exist** — confirmed via direct `find`. These plans predate the `26/08/12/ARTIFACTS-ONLY-PLUGIN-
ARCHITECTURE` and `26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` rewrites (both cited in doc
comments across the current `editor/`/`⏳️precompute/` tree as the reason logic was "rehomed" from a former
`⚙️engine/` module into the app layer). The bespoke TypeScript PlayCanvas host, its dedicated `parry3d`
WASM crate, and the per-plugin React renderer these plans edit are all gone; today puzzle3d emits scene
JSON (§1) to a generic framework World3d host. **Verdict for all 16: superseded — not verifiable as
done/not-done against current source in the literal sense, because their target files no longer exist.**
Whatever they achieved lives only in git history predating the rewrite. Two exceptions worth naming
specifically:

- **`puzzle3d_wgpu_feature_parity`** (goal: rebuild vortices/attractions/target-volumes/reference-planes/
  brush-preview/relocate visualization in the new renderer after the PlayCanvas→WGPU-host cutover) — **its
  core visualization goals are DONE in current source**: `main::render()` populates `vortices_json`,
  `attractions_json`, `target_volumes_json`, `references_json`, and `brush_preview_json`, all confirmed
  present and real (§1). The one goal not obviously done: a dedicated relocate/proximity-connect preview
  effect — no JSON channel for it was found in `render()`; `worldRelocate` mutates the document directly
  with no drag-preview ghost.
- **Marquee/selection plans** (`fix_puzzle3d_marquee_selection`, `puzzle_3d_marquee_selection`,
  `_selection_overhaul`, `_selection_lag`, `_selection_perf`, `_select_freeze`, `_select_commit_freeze`,
  `_selection_context_menu`) — their target symptom (marquee correctness, selection latency, a right-click
  context menu) may or may not still exist under the current generic World3d host; what current source
  *does* independently confirm is that the option-level marquee method/merge controls this app used to
  expose are gone (TL;DR #5) for an unrelated, current reason (framework interaction-domain ownership).
  Re-diagnosing marquee/selection latency against the current host would need a fresh runtime session, not
  a source read.
- **`puzzle_3d_brush_tool`, `_brush_fill_diagnosis`, `_brush_fill_context_menu_fix`, `_catalogue_drop`,
  `_precompute_worker`, `_fill_preview_perf`, `_fill_reroll`** — all target the removed TS/PlayCanvas
  engine and JS-side fill/brush session code (`puzzle/3d/play/index.ts`'s `preparePuzzle3dFillSession`,
  `puzzle/3d/react/index.tsx`'s `BrushSession`); the current Rust `⏳️precompute/🪣️fill`,
  `⏳️precompute/🖌️brush` modules are a from-scratch reimplementation in a different language and layer.
  None of these plans' specific line-numbered changes apply to current source.

## Files read

Primary: `editor/🦀️.rs` (7,402 lines, full read of the action-dispatch/retained-command/Work regions);
every file under `🎮️commands/*/🦀️.rs` (46 command modules, full read); `🎭️modes/✏️edit/🦀️.rs`,
`🎭️modes/✏️edit/☑️options/*/🦀️.rs` (6 files), `🪟️windows/🧊️main/🦀️.rs`,
`🪟️windows/🧊️main/🪛️utilities/*/🦀️.rs` (4 files), `📌️panels/*/🦀️.rs` (4 files), `👥️presence/🦀️.rs`,
headers of `⏳️precompute/{🦀️.rs,🪣️fill/🦀️.rs,🖌️brush/🦀️.rs,📐️geometry/🦀️.rs}`, `🗣️terminology/🦀️.rs`,
`🎚️config/🦀️.rs`. Ticket notes: `📓️status.md`, `📓️findings-2026-09-05.md`. Comparison:
`🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs` (9,099 lines, classification-only), `🗿️artifacts/💠️lowpoly/…/✏️editor/🦀️.rs`
(2,043 lines, classification + selection-plumbing check). All 16 `.cursor/plans/puzzle3d_*` and
`puzzle_3d_*.plan.md` files, full read.
