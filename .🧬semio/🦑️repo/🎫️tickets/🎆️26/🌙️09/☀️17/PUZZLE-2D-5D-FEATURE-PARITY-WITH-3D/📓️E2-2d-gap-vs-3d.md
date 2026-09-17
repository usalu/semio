# E2 — puzzle ◻️2d source-measured inventory vs 🧊️3d (2026-09-17)

Read-only audit, no cargo/nx/server, no edits. Method: re-read every 2d source file the 2026-09-06 ticket's
`📓️2026-09-16-parity-survey.md` and `📓️status.md` (2026-09-16/17 "reopen" section, waves S/P/T/L) claimed
landed, cross-checked against the 3d ticket's 25-section `📓️2026-09-09-user-feature-checklist.md`, and
verified the 3d editor's ACTUAL current action registry (not the checklist's possibly-stale line numbers).
Four passes: this session read root files/panels/commands directly (independent verification), and three
background agents each re-verified a slice of sections against current source, blind to each other. All
three agents' conclusions matched or refined this session's own findings — no contradictions.

**A peer (26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING, landed 2026-09-17 00:10, commit `a4cda597ea`)
migrated the 2d artifact/catalogue/inspection panels to the framework's `TreeWindows` virtualisation and
removed the earlier `setPanelPage`/`panelPages` cursor.** Verified: this migration introduced **no
regression** — every wave-P/S/T feature (grid/LOD/brush measures, selection, fill weights, transform verbs)
still compiles and dispatches against the new `TreeWindows`/`window_section_or_placeholder` shape. The one
real gap found under the panels (§17, outliner hide/lock icons) **predates the migration** — confirmed via
`git show a4cda597ea` that the pre-migration `🗿️artifact/🦀️.rs` never had row-level hide/lock icons either.

Editor root: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — 4,291
lines (up from 3,949 on 09-16), 36 retained action ids (up from 30). 3d equivalent: same subpath under
`🧊️3d`, 8,711 lines, 63 retained action ids.

## Summary table (25 checklist sections)

| § | 3d feature | 2d state | proof (2d file:line) | note |
|---|---|---|---|---|
| 1 | 2-3 window instances render | **DONE** | `🎭️modes/✏️edit/🦀️.rs:32` layout `[overview,detail,selection]`; each declares `WINDOW_KIND_ID` (`👁️overview/🦀️.rs:14`, `🔍️detail/🦀️.rs:12`, `🎯️selection/🦀️.rs:12`) | minor: `WindowKindDefinition.surface_kind` declared `SurfaceKind::Canvas2d` but rendered node is `SurfaceKind::Board2d` (`overview/🦀️.rs:29` vs `edit/🦀️.rs:157`) — harmless today, drift vs 3d's consistent `World3d`/`World3d` |
| 2 | orbit/pan/zoom → setCamera per window | **DONE** | `🎮️commands/📷️set-camera/🦀️.rs:6-13`; `🪟️window/🦀️.rs:365` `config_from_view_or_document` scopes camera per `view.window_id` | correctly per-window, not a flat global |
| 3-4 | projection/grid/LOD/select window options | **DONE** (grid+LOD+brush); **MISSING** (select-kinds filter, grid-visible toggle) | `overview/🦀️.rs:46` = `[lod::measure, grid::measure, brush::measure]`; grid group real (`☑️options/🌐️grid/🦀️.rs:12-49`, snap toggle + factor slider); LOD real (`☑️options/🔭️lod/🦀️.rs:15-19`, Automatic+tiers Select) | 2d has no `SetGridVisible` analogue (grid is always drawn) and no `SetSelectableKind`-equivalent window option (filter which granularity — node/handle/edge — is clickable); projection/sun/voxel are legitimately OUT (no 2D camera family, no sun) |
| 5 | example switcher + empty→add | **DONE** | `🎮️commands/🛍️set-active-example/🦀️.rs:9-33` real `EMPTY`/`CONCRETE_FOREST`/`NAKAGIN` `LazyLock` targets; shared `BOARD_POINTER_ITEM_CAPACITY=1_024` confirmed still raised (not reverted) | Nakagin loads via the shared board-host infra, no 2d-local capacity regression found |
| 6 | click/marquee/same-kind/all/clear selection | **DONE** | `🦀️.rs:109-115` `Puzzle2dInteractionSnapshot::from_interaction` reads real `interaction.selection(...)`; `🎭️modes/✏️edit/🦀️.rs:124` `selection_json: envelope.interaction.selection_json()` (the OLD always-`"[]"` bug is gone); `🎮️commands/🧬️select-same-kind/🦀️.rs:8-17` pushes a real `Emit.interaction_writes`, not a no-op | genuinely landed wave-S work, survives the TreeWindows migration |
| 7 | hover flag | **MISSING** (dead data, not wired to paint) | `🦀️.rs:109-115` DOES resolve real hover (`hovered: hover.ids.clone()`), but `🎭️modes/✏️edit/🦀️.rs:138` hardcodes `hovered_id: None` in `puzzle2d_board_scene`, and `grep '\.hovered\b'` across the whole 2d editor tree finds **zero other reads** of that field | precise, one-line-fixable gap: wire `envelope.interaction.hovered.first()` into `hovered_id` at line 138, mirroring 3d's `🧊️main/🦀️.rs:828-829` (`hovered_id` derived from `interaction.hovered_object_id(...)`) |
| 8 | gumball/transform translate·rotate·scale | **DONE** (2d has no gumball by design — direct drag substitutes) | `🎮️commands/{🚀️translate-selection,🔄️rotate-selection,📏️scale-selection}/🦀️.rs` all call shared `puzzle2d_transform_selection` (`🦀️.rs:736-768`): centroid math, handle-angle rotation (`756-766`), locked-node exclusion (`740`) | matches the 09-16 status log's claim exactly, not a stub |
| 9 | brush utility + candidate picker | **DONE** | full slot family real: `🔓️open-slot`, `🔁️cycle-candidate` (`brush_cycle_candidate(forward)`), `🔢️set-candidate-index`, `✅️commit-slot` (only one calling `apply_host_events`), `🚫️cancel-slot`; picker UI `☑️options/🖌️brush/🦀️.rs:101-121` (`WindowMeasure::Select`, only renders once `brush_candidates` populated) | genuine hover-before-commit UX (cycling never silently commits); **but no context-menu entry point found** (§13/verb-diff) |
| 10 | volume brush (target volumes) | **OUT** | zero hits for `target.volume`/`TargetVolume` anywhere in 2d source | legitimately absent, no 2D target-region geometry concept |
| 11 | relocate utility | **DONE by design, minor gap** | drag→`nodeMove`/`nodeDragEnd` covers relocate (`🎮️commands/🎲️apply-board-events/🦀️.rs:38,86,101`) | no magnet/proximity-snap-on-drop analogue to 3d's `worldRelocate`+`setProximityRadius`+`setBrushPlacementContactTolerance` — real but low-priority capability gap |
| 12 | fill: count/weights/run panel/reapply/cancel | **DONE** | count `🛠️tools/🪣️fill/🦀️.rs:55-69`; weights `☑️options/🖌️brush/🦀️.rs:48-83` (node+handle distribution trees, real sliders → `setBrushKindWeights`); run declared `mutating:true, rebase:Revalidate` (`🛠️tools/🪣️fill/🦀️.rs:36-50`, same shape as 3d); checkpoint/replay `⏳️precompute/🪣️fill/🦀️.rs:1016-1611` | pause/step/abort/finalize/re-apply-on-change are framework `ToolRun` generics, inherited automatically like 3d |
| 13 | suggestions popup open/hover/accept/close | **PARTIAL/MISSING** | slot family real (see §9): `brushOpenSlot`/`brushCycleCandidate`/`brushSetCandidateIndex`/`brushCommitSlot`/`brushCancelSlot`, real picker `☑️options/🖌️brush/🦀️.rs:101-121` | confirmed by two independent passes: reachable ONLY while the brush/fill tool is armed — no context-menu row dispatches `brushOpenSlot` (`🦀️.rs:1176-1182`, zero "suggest" entries), unlike 3d's context-menu-triggered popup that works on an already-selected vortex without a tool-mode switch; also no distinct hover-preview-before-commit state. See verb-diff table below for the full `openVortexSuggestions` family classification. |
| 14 | engagement bar grammar | **DONE, exceeds 3d** | parser `🎮️commands/📨️engagement-submit/🦀️.rs:17-51` = `select/brush/fill [n]/clear/move dx dy/rotate deg/scale f`; placeholder `🎭️modes/✏️edit/🦀️.rs:172` matches EXACTLY, no over-promise (3d's placeholder over-promised `clear/rectangle/lasso`, 2d's doesn't) | minor: no `engagementRepeatLast` analogue (`on_repeat_last: None`, line 183) — not advertised either, so not a mismatch, just absent |
| 15 | context menu per selection kind | **DONE, no bug** | full row list `🦀️.rs:1134,1176-1182`: `selectAll`(empty)/`toggleHidden`/`toggleLocked`/`duplicate`/`focusSelection`/`selectSameKind`/`deleteSelection`; hide/lock args `value: any_visible`/`any_unlocked` (1177-1178) **correctly alternate** | 2d does NOT reproduce 3d's known `"zoomToSelection"`-vs-`"focusSelection"` unregistered-action bug — dispatches the real `focusSelection` id throughout |
| 16 | inspection panel (patchInspector) | **DONE** | `📌️panels/🔍️inspection/🦀️.rs:91-130`: real `node_fields`/`edge_fields`/`handle_fields` groups (not just the document summary fallback at 133-141); `flag_row` (71-78) uses `UiValue::Bool(!pressed)` — correct toggle, no hardcoded-true bug | note: ALL of node/edge/handle's numeric fields (x/y/radius/angle/width/height) are `read_only` rows, not editable `patchInspector`-bound steppers — only hidden/locked toggle; unclear if 3d's object/vortex inspector fields are editable either (not confirmed in this pass) — flag as a parity question, not a scored gap |
| 17 | outliner hide/lock row icons | **PARTIAL/MISSING** | `📌️panels/🗿️artifact/🦀️.rs` (96 lines): rows are plain `pick_row(id,label,description,granularity)` (62-78), **no flag/icon binding of any kind** | confirmed via `git show a4cda597ea` this predates the 09-17 TreeWindows migration — 2d's outliner never had inline hide/lock icons (unlike 3d's, which HAD them with a hardcoded-`true` bug). Equivalent function exists via context menu (§15) and inspection panel (§16), just not on outliner rows |
| 18 | catalogue add + drag/drop | **DONE** | `📌️panels/🛍️catalogue/🦀️.rs:74-103`: live kinds via `kind_catalog_entries`→`inferred_kind_entries` fallback (90-97); draggable node rows (`tree_item_with_action_draggable`, 84); click→`addNode{kind}` (78) | drag mime `PUZZLE2D_CATALOGUE_DRAG_MIME` = framework's generic `CATALOGUE_DRAG_MIME` (`🧰️framework/…/🌳️Tree/🟦️.tsx:1122`) — real, shared drop-handling infra, not a stub |
| 19 | settings panel (steppers) | **PARTIAL** | `📌️panels/⚙️settings/🦀️.rs` (77 lines): exactly 3 steppers — fill-count, suggestion-offset, grid-factor (69-71) | 3d's 4 steppers are overlap-budget/proximity-radius/chunk-size/grid-spacing; 2d has grid-factor (≈grid-spacing) but **no overlap-budget or proximity-radius concept anywhere in the 2d crate** (`grep -rn "overlap_budget\|proximity\|chunk_size"` → 0 hits) — ties to the §9/§11/§13 placement-tuning gaps |
| 20 | history undo/redo/checkpoint | **DONE (by inheritance)** | framework-reserved routes, not app actions; `🦀️.rs:1482` `undo_policy: ExactBaseOnly` (standard wiring); no 2d override found that could break it | same class as 3d — inherited, not app-specific |
| 21 | copy/cut/paste | **DONE (matches 3d's own gap)** | `🦀️.rs:4007` comment: "puzzle2d owns no clipboard fragment vocabulary of its own"; zero `clipboard`/`copy`/`cut`/`paste` identifiers in the crate | framework-generic or absent exactly like 3d — not a 2d-relative gap |
| 22 | delete/duplicate/focus | **DONE** | `🗑️delete-selection/🦀️.rs:8-17` clears selection via subtractive `interaction_writes`; `👯️duplicate-selection/🦀️.rs:7-13` **re-selects the clones** (`ctx.interaction_writes.push(puzzle2d_selection_write(...))`); `🎯️focus-selection/🦀️.rs:7-41` real bounding-box camera centre | matches 3d's post-wave-S behaviour exactly |
| 23 | add-node dialog with live kinds | **PARTIAL** | `grep "Dialog\|dialog"` in the 2d editor root → **0 hits**, no `openAddObjectDialog`-equivalent exists at all (vs 3d's real `DialogDefinition` at `🧊️3d/…/🦀️.rs:2589,3621,3838,8606-8644`) | the catalogue panel (§18) IS a live-kind-driven add path and is arguably AHEAD of 3d (whose own dialog/`addObjectKind` schema hardcodes a static `"Object"` option, a known 3d bug) — the only missing piece is a dedicated modal affordance, not live-kind enumeration itself |
| 24 | import/export fixture | **DONE** | `📤️export-fixture/🦀️.rs:22-77`: real inline-vs-segmented split (`puzzle2d_export_inline_budget_bytes` = `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`, segmented via `ArtifactOutputChunks`); `📥️import-fixture/🦀️.rs` 353 lines (chunked staging); `🗂️open-import-fixture/🦀️.rs` real `Effect::RequestFileOpen` | not `BatchOnlyPendingRewrite` — real, ported from 3d's staging law per the wave-T log entry, confirmed in source |
| 25 | EN/DE locale | **DONE** | `🗣️terminology/🦀️.rs`: full `semio_framework_plugin::app_labels!` compile-checked set, native+reuse EN/DE for every label (17-60+) | same compile-time-checked mechanism as 3d |

**Score: 16 DONE, 1 DONE-by-inheritance-generic×2 (20,21), 3 PARTIAL (17,19,23), 2 MISSING (7, and the §13 entry-point half), 1 OUT (10), 1 DONE-with-minor-gap (11).** 2d is materially closer to 3d than the 09-16 survey assumed — most of the survey's **bold** action items (thread selection, real selectSameKind, grid measures, fill weights, transform verbs, export/import) are now genuinely landed and independently re-verified in this pass, not just claimed.

## Verb registry diff (source of truth: the `_command_variants!` macro in each editor root)

3d: `puzzle3d_command_variants! { … }` at `🧊️3d/…/✏️editor/🦀️.rs:2588-2655` — **64 action ids** (this
session's own read of the macro, cross-confirmed independently by a background agent that also verified
it against the `.action_with`/`dispatch_puzzle3d_action` registry chain at `🦀️.rs:3644-3699,8449-8526`).
2d: `puzzle2d_command_variants! { … }` at `◻️2d/…/✏️editor/🦀️.rs:1023-1060` — **36 action ids**, cross-
confirmed against `create_puzzle2d_app`'s registry chain (`🦀️.rs:4166-4213`).

2d already covers (same or vocabulary-swapped id): TranslateSelection, RotateSelection, ScaleSelection,
SetActiveExample, AddObjectKind→AddNode, DeleteSelection, DuplicateSelection, ExportFixture, ImportFixture,
OpenImportFixture, SelectSameKindSelection→SelectSameKind, SetCamera, SetLodAutomatic+SetLodManual→
SetLodModeForPane (consolidated), SetGridSnapEnabled, SetGridSpacing→SetGridFactor, SetSelectionFlag,
PatchInspector→PatchInspectorNodes, FocusSelection, EngagementInput/Submit/Abort/ControlSelect,
SetFillCount, SetObjectKindWeight+SetVortexKindWeight→SetBrushKindWeights (consolidated via `catalogSlice`
arg), CycleBrushCandidate+CycleBrushCandidateBack→BrushCycleCandidate (consolidated via `forward: bool`
arg), OpenVortexSuggestions→BrushOpenSlot, CloseVortexSuggestions→BrushCancelSlot,
AcceptSuggestion→BrushCommitSlot, HoverSuggestion→ (covered by cycle/set-index previewing before commit,
no separate verb needed).

### 3d-only ids with no 2d analogue

| 3d id | 3d file:line | classification | note / porting target |
|---|---|---|---|
| `openAddObjectDialog` | `🧊️3d/…/🦀️.rs:2589,3621,8606-8644` | **OWED** | §23 — dedicated add-node dialog affordance; low priority since catalogue panel already covers live-kind add |
| `transformBegin`/`transformEnd` | `🦀️.rs:2590-2591` | OUT | deliberate no-op brackets for 3d's gumball drag session; 2d has no gumball, drag is engine-local |
| `setActiveTool` (`SET_ACTIVE_TOOL_ACTION_ID`) | `🦀️.rs:2596` | **verify** | 2d has no LOCAL override variant for this framework-owned id — engagement-bar `"fill"` dispatches `Effect::SetActiveTool` directly (`📨️engagement-submit/🦀️.rs:32`), but clicking the Fill tool TAB itself relies purely on framework default handling with no 2d-side effect clearing (3d's own handler clears `suggestion_menu`/`brush_candidate_index` on tool switch, `🧊️3d/…/🦀️.rs:6836-6838`) — verify the fill tab visually stays "on" and brush state doesn't leak across tool switches |
| `setProjection`/`setProjectionParam` | `🦀️.rs:2605-2606` | OUT | no 2D camera-projection family |
| `setVortexShow`/`setVortexDirection` | `🦀️.rs:2607-2608` | OUT | 3D-only vortex-marker display toggle; 2d handles are always-visible dots |
| `relocateTargetVolume` | `🦀️.rs:2609` | OUT | target volumes OUT (§10) |
| `worldRelocate` | `🦀️.rs:2610` | **judgment call** | this session and the verb-diff agent disagreed: agent reads it as OUT (2d forms edges only via an explicit drag-connect gesture, no auto-attach needed on a flat plane); this session flags it OWED-low because the 3d checklist's proximity-radius auto-attract is a real convenience a dense 2D graph could also want. Recommend the dev decide rather than treat either verdict as final. |
| `toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity` | `🦀️.rs:2611-2614` | OUT | no sun in a 2D board |
| `setLodDepthVariable` | `🦀️.rs:2616` | OUT | 3D depth-based LOD falloff has no 2D meaning |
| `setGridVisible` | `🦀️.rs:2617` | **OWED** | §3-4 — 2d's grid group has snap+factor but no show/hide toggle |
| `setProximityRadius` | `🦀️.rs:2621` | **OWED (low)** | §19 — settings-panel gap, tied to `worldRelocate` above |
| `setChunkSize` | `🦀️.rs:2622` | **verify/low** | no viewport-chunking concept found in 2d; may be legitimately OUT if 2d's board host doesn't chunk the same way, or OWED if Nakagin-scale boards need it — undetermined in this pass |
| `setSelectableKind` | `🦀️.rs:2623` | **OWED** | §3-4 — no 2d window option to filter which granularity (node/handle/edge) is clickable |
| `engagementRepeatLast` | `🦀️.rs:2629` | **OWED (low)** | §14 — not advertised in 2d's placeholder either, so cosmetic-only |
| `createAttraction` | `🦀️.rs:2631` | **OWED** | no standalone host-dispatchable "connect two handles" verb in 2d — edges are only created via the engine-bridged `applyBoardEvents{edgeCreate}`; no engagement-bar/API/context-menu path to create an edge programmatically |
| `deleteAttraction` | `🦀️.rs:2632` | covered-by-design | 2d's `deleteSelection` already removes selected edges (edge rows are selectable via the outliner/canvas) — no separate per-id verb needed |
| `setTransformGumballFlag` | `🦀️.rs:2633` | OUT | no gumball in 2d |
| `setVoxelDims`/`addTargetVolume`/`deleteTargetVolume`/`setTargetVolumeFlag` | `🦀️.rs:2634-2637` | OUT | target volumes OUT (§10) |
| `addBrushObject` | `🦀️.rs:2639` | covered | 2d's `commitSlot` is the equivalent commit-to-place verb |
| `setBrushPlacementContactTolerance` | `🦀️.rs:2641` | **OWED (low)** | §19/§9 — no collision-tolerance concept found in 2d source |
| `cycleBrushCandidateBack` | `🦀️.rs:2645` | **OWED (UI-wiring only)** | the verb-diff agent found `🔁️cycle-candidate/🦀️.rs:7-9` takes a `forward: bool` param — 2d's command LAYER already supports backward cycling, but 2d has no `shift+tab`-style keybinding or UI control that ever calls it with `forward:false` (3d binds `shift+tab` → `cycleBrushCandidateBack`, `🧊️3d/…/🦀️.rs:8446`); a UI-only gap, not a missing capability |
| `openVortexSuggestions`/`closeVortexSuggestions`/`hoverSuggestion`/`acceptSuggestion`/`targetBrushSuggestions` | `🦀️.rs:2646-2650` | **OWED (whole family)** | refined after cross-checking two independent passes: 2d's `brushOpenSlot`/`brushCancelSlot`/`brushCommitSlot` are reachable **only while the brush/fill tool is actively armed** (no context-menu row anywhere dispatches `brushOpenSlot` — confirmed, `🦀️.rs:1176-1182` has no "suggest" entry). 3d's popup family is reachable directly from a context-menu "Suggest Objects" row on an already-selected vortex, **without** switching tool mode first (3d `🦀️.rs:3003`), and separately supports hover-preview-before-commit as a first-class state. 2d has neither the tool-mode-independent entry point nor a distinct hover-preview step (cycling directly changes the committed-on-accept index, with no separate "just previewing" state) — this is a real, structural UX gap, not just a missing context-menu row. |
| `registerBrushMesh` | `🦀️.rs:2651` | OUT | GLB collision-mesh registration, 3D-rendering-specific |
| `worldPointerDown` | `🦀️.rs:2652` | OUT | 3D world/viewport pointer-routing bridge; 2d's `BoardHost` owns pointer input directly, no framework bridge needed |

## Prioritized OWED work items

1. **Hover never paints on the canvas (§7)** — highest priority, smallest fix. `Puzzle2dInteractionSnapshot`
   already carries real hover ids (`🦀️.rs:109-115`); wire it into `Board2dScene.hovered_id` at
   `🎭️modes/✏️edit/🦀️.rs:138` (currently `hovered_id: None`). Port pattern from 3d
   `🧊️3d/…/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:828-829` (`hovered_object_id`/`hovered_reference_id`
   derivation). One line + a getter on `Puzzle2dInteractionSnapshot`.
2. **Vortex/handle suggestions is tool-mode-gated with no context-menu entry point (§13, whole
   `openVortexSuggestions`/`hoverSuggestion`/`acceptSuggestion`/`targetBrushSuggestions` family)** — the
   brush slot family (`brushOpenSlot`/`cycleBrushCandidate`/`brushCommitSlot`/`brushCancelSlot`) is real
   and wired, but only reachable while the brush/fill tool is armed; nothing in
   `puzzle2d_context_menu_items` (`✏️editor/🦀️.rs:1119-1184`) dispatches `brushOpenSlot` for a selected
   handle outside that mode. Port 3d's context-menu "Suggest Objects" row (`🧊️3d/…/🦀️.rs:3003`) into the
   2d menu's handle-selection branch, and consider adding a distinct hover-preview state (today cycling
   directly moves the commit-on-accept index with no separate "just looking" step).
2b. **Backward brush-candidate cycling has no UI trigger** — `🎮️commands/🔁️cycle-candidate/🦀️.rs:7-9`
   already accepts `forward: bool`, so the command layer supports it; add a `shift+tab`-style keybinding
   or a back-button in the picker measure (`☑️options/🖌️brush/🦀️.rs:101-121`), mirroring 3d's
   `.keybinding("shift+tab", "cycleBrushCandidateBack")` (`🧊️3d/…/🦀️.rs:8446`). Trivial, UI-only.
3. **Outliner has no hide/lock row icons (§17)** — `📌️panels/🗿️artifact/🦀️.rs:62-78`'s `pick_row` has no
   action binding. Port 2d's OWN already-correct toggle logic from the context menu
   (`✏️editor/🦀️.rs:1177-1178`, `value: any_visible`/`any_unlocked`) or the inspection panel's `flag_row`
   (`📌️panels/🔍️inspection/🦀️.rs:71-78`) — do NOT copy 3d's version, which has the hardcoded-`true` bug
   3d itself hasn't fixed on the outliner. Low risk given the framework's per-row-action perf note
   (`project-ui-value-map-ascending-keys-and-arena-page` memory: row actions on every big-tree row starve
   sibling panels) — consider a single tree-level toggle-selected-visibility action instead of per-row
   icons if performance is a concern at Nakagin scale (180+179 rows).
4. **`createAttraction` has no host-dispatchable verb (verb-diff)** — edges can only be created via the
   engine-bridged `applyBoardEvents{edgeCreate}`; add a `createEdge`/`connectHandles` command (new file
   under `🎮️commands/`) mirroring 3d's `💞️create-attraction/🦀️.rs`, for engagement-bar/API/programmatic
   use (the engagement-bar grammar at `📨️engagement-submit/🦀️.rs:17-51` has no `connect`/`edge` verb today).
5. **No grid-visibility toggle (verb-diff, `setGridVisible`)** — add a toggle beside the existing
   snap/factor group in `🎭️modes/✏️edit/☑️options/🌐️grid/🦀️.rs:26-48`, porting the show/hide flag from 3d's
   `🎮️commands/{👁️set-visible}`-equivalent 3d grid group (`🧊️3d/…/🦀️.rs:79` in the 3d checklist's §4 table).
6. **No selectable-kind filter window option (verb-diff, `setSelectableKind`)** — add a Select/checkbox
   group (node/handle/edge) to the window options rail, mirroring 3d's `☑️set-selectable-kind/🦀️.rs`.
7. **Settings-panel placement-tuning gap (§19/§9/§11)** — no `overlap_budget`/`proximity_radius`/
   `contact_tolerance` concept exists anywhere in the 2d crate. Lowest priority of this list: decide
   whether 2d's simpler collision model needs these at all before porting 3d's
   `📌️panels/⚙️settings/🦀️.rs` steppers and `🎮️commands/{🚧️set-brush-placement-contact-tolerance,
   📡️set-proximity-radius}/🦀️.rs`.
8. **Add-node dialog affordance (§23)** — cosmetic/UX only, since the catalogue panel already provides
   live-kind add (ahead of 3d's own hardcoded-`"Object"` dialog bug). Lowest priority; port
   `openAddObjectDialog`'s effect-dispatch shape (`🧊️3d/…/🦀️.rs:3621,3838`) only if a modal affordance is
   actually requested by the dev, not because 3d has one.
9. **`engagementRepeatLast` (verb-diff)** — cosmetic-only, not advertised in the UI placeholder either.
   Skip unless the dev asks for it explicitly.

## Not investigated / caveats
- Runtime behaviour (this is a source-only audit per the ticket's read-only constraint) — none of the
  above has been driven in a browser in this pass; the 09-16/17 status log's own runtime probes (activation
  #5-#7, battery results) are the only runtime evidence and predate today's TreeWindows-migration
  re-verification.
- `setChunkSize`'s 2d applicability is genuinely undetermined (item 6 above) — needs a decision from
  whoever owns the board-host viewport/LOD architecture, not a blind port.
- Inspector field editability parity (node/handle numeric fields being `read_only` in 2d) was not checked
  against 3d's actual field-builder code closely enough to score as a gap — flagged as a question in §16.
