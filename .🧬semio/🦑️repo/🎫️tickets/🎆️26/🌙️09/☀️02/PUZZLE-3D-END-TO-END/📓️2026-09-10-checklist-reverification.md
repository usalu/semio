# Puzzle 3D user-feature checklist — reverification (2026-09-10, audit A1)

Read-only re-verification of `📓️2026-09-09-user-feature-checklist.md` (25 sections) against the **current** source tree, cross-referenced with wave reports D4, J, K, L, N, G, H (in progress), P/P2 (in progress), F (in progress), and `📓️2026-09-09-runtime-verification.md`. No builds, no browser runs, no git mutations.

**Method:** `rg -a` throughout (emoji paths; plain `grep` returns nothing on several TS files). Status meanings:

| Status | Meaning |
|---|---|
| **FIXED** | Claim holds in current source; cite `file:line`. May still need browser confirmation. |
| **OPEN-OWNED** | In-flight wave owns it (wave id in parentheses). |
| **OPEN-UNOWNED** | No active wave; ranked in §26. |
| **BROWSER-ONLY** | Source insufficient; exact probe listed. |
| **N/A** | Deliberate design, not a defect. |

**Build caveat (checklist §0):** Still applies. Last coordinator browser pass was rebuild #26 (2026-09-10 00:40); rebuild #27 queued after W-P/W-F/W-H. Native puzzle3d suite reported 613 passed / 9 failed (W-N handover).

---

## §0 Before driving — build/compile caveat

| Item | Status | Evidence / probe |
|---|---|---|
| Workspace must compile and wasm must be materialized before trusting browser rows | **BROWSER-ONLY** | Coordinator: stale wasm served until `materialize` fixed (`runtime-verification.md` §19:15–19:25). Probe: confirm served `core.wasm` mtime matches latest `component-release` artifact. |

---

## §1 Windows — Top, Perspective

| Feature | Status | Evidence |
|---|---|---|
| Top window renders (`puzzle3d-main-top`) | **FIXED** (source); **BROWSER-ONLY** (runtime) | Window instances `WINDOW_INSTANCE_TOP` / `PERSPECTIVE` at `main/🦀️.rs:31-32`; `render` builds `World3dScene` at `:513-534`. Runtime doc 07:33 confirmed both panes; not re-run on #26+. |
| Perspective window renders | same | same |
| Focus a window (shortcuts/utility-bar scope) | **FIXED** (framework) | Per-window `windowViewContext` in `ShellHost/🟦️.tsx:5407` (tool branch pattern). |
| Split / open / close window | **N/A** | Two fixed instances by design (`main/🦀️.rs:8-10`). |

---

## §2 Camera — orbit / pan / zoom

| Feature | Status | Evidence |
|---|---|---|
| Orbit / pan / zoom via `setCamera` | **FIXED** (W-D2) | `WindowConfig` publication no longer blocks forever: byte grant vs owner ceiling at `🎚️config/🦀️.rs:100-104` (`grant.maximum_bytes < self.retained_bytes` → `Err`, not infinite `Blocked`). Test `camera_actions_are_view_actions_that_emit_no_artifact_mutations` in unit suite. **BROWSER-ONLY:** drag orbit on perspective pane, confirm no 4096-continuation hang. |
| Camera per-window | **FIXED** (source) | `set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched` in `🧪️tests/🔬️unit/🦀️.rs`. |
| Malformed camera silent no-op | **FIXED** (unchanged) | `🎮️commands/📷️set-camera/🦀️.rs` — parse failure is no-op by design. |

---

## §3 Projection pane options

| Feature | Status | Evidence |
|---|---|---|
| Projection family / orientation measures → `setProjection` / `setProjectionParam` | **FIXED** (W-D2) | Same `WindowConfig` lane fix as §2. Measures from `options/🎥️projection/🦀️.rs:17`. **BROWSER-ONLY:** unfold utility rail → Projection, switch orthographic ↔ three-point. |

---

## §4 Window options — grid / LOD / vortex / sun / select

| Feature | Status | Evidence |
|---|---|---|
| Grid, LOD, vortex show/direction, sun, selectable-kind toggles | **FIXED** (W-D2) | All publish via `WindowConfig` (`load_window`/`save_window`); lane fix at `🎚️config/🦀️.rs:100-104`. Measures wired in `main/🦀️.rs:68-79`. **BROWSER-ONLY:** toggle grid visible, LOD slider, vortex Always/Selected. |

---

## §5 Example switcher

| Feature | Status | Evidence |
|---|---|---|
| Example select (Concrete Forest / Nakagin) | **FIXED** (W-B ledger + W-K mesh refs + W-P lanes) | `nakagin_app()` + law `the_nakagin_world_scene_publishes_every_lane_under_the_page_cap_with_the_popup_open` at `🧪️tests/🔬️unit/🦀️.rs:1576-1618` (spine ≤ 32 KiB, lanes paged). Mesh refs: `world3d_mesh_kind_entry` emits `{id,kind}` (`plugin/🦀️.rs:33538`). **BROWSER-ONLY:** switch picker to Nakagin, confirm scene + outliner update (blocked on #26 intake stall — see §26). |
| Empty example → `addObjectKind` | **FIXED** (W-D2) | `Puzzle3dAddObjectKindWork` `Catalog` stage materializes default kind: `🦀️.rs:3864-3866`, `declared_default_kind` at `:3808`. |
| Unrecognized `exampleId` silent no-op | **FIXED** (unchanged) | By design. |
| Runtime reset on example switch (camera/grid/sun) | **OPEN-UNOWNED** | `set_active_example` assigns `ctx.scene.runtime = Puzzle3dRuntime::default()` (`🎮️commands/🛍️set-active-example/🦀️.rs:20`) but publication is `Artifact` lane only — whether per-window `Puzzle3dWindowConfig` visibly resets is **unverified**. **BROWSER-ONLY:** switch example, compare grid spacing / camera in both panes before vs after. |

---

## §6 Selection

| Feature | Status | Evidence |
|---|---|---|
| Click select (instance) | **FIXED** (W-S) | `domain_id` + `interactionSelect` path; tests `world_pick_*`, `interactionSelect` via testkit. **BROWSER-ONLY** on #26+ (intake stall blocked picks — `runtime-verification.md` §00:40). |
| Click select (vortex marker) | **FIXED** (W-V) | `world_vortices_carry_their_own_selected_and_hovered_flags` at `🧪️tests/🔬️unit/🦀️.rs:2973`. |
| Tree/catalogue select | **FIXED** | `interactionSelect` via `puzzle3d_interaction_select` (testkit `:486`). |
| Marquee / rectangle select | **FIXED** (framework) | `SelectionMarquee` in `World3dHost`; `selection_method` on window config (W-D4 engagement verbs). **BROWSER-ONLY:** drag empty canvas with/without Shift. |
| Select same kind | **FIXED** (W-S) | `select_same_kind_widens_the_selection_to_every_object_of_that_kind`. |
| Select all | **FIXED** (framework) | `SELECT_ALL_ACTION_ID` imported `🦀️.rs:43`. |
| Clear selection | **FIXED** (framework) | Background-clear in `World3dHost`. |

---

## §7 Hover

| Feature | Status | Evidence |
|---|---|---|
| Hover object/vortex flags in scene JSON | **FIXED** (W-S + W-V) | `world_vortices_carry_their_own_selected_and_hovered_flags` (`🧪️:2973`); `world_vortices_json` at `main/🦀️.rs:237`. **BROWSER-ONLY:** hover slab, confirm highlight in both panes. |

---

## §8 Gumball / transform

| Feature | Status | Evidence |
|---|---|---|
| Move/rotate flag toggles persist | **FIXED** | `transform_utility_options_expose_move_and_rotate_flags` (unit suite). |
| `gumball_active` when transform + selection | **FIXED** (W-S) | `gumball_active` at `main/🦀️.rs:114-115`; test `gumball_active_only_for_transform_utilities_with_object_selection` (`🧪️:3167`). |
| Gumball drag → one commit on release | **FIXED** (architecture) | `transformBegin`/`transformEnd` host-only brackets; delta via `translateSelection`/`rotateSelection`/`scaleSelection`. **BROWSER-ONLY:** mid-drag preview + release commit. |
| Programmatic translate/rotate/scale | **FIXED** | Command reducers + `Puzzle3dScaleWork` with refusal at `🦀️.rs:4134` (W-N). |
| Gumball undo coalesce | **OPEN-OWNED** (W-U) | `gumball_translate_drag_coalesces_into_one_edit` still fails UNDO (`🧪️:3254`, W-N §4.3). W-U investigating (`📓️2026-09-10-wave-U-editor-correctness.md`). |

---

## §9 Brush utility

| Feature | Status | Evidence |
|---|---|---|
| Placement candidate picker | **FIXED** (W-S) | `puzzle3d_brush_target_vortex` at `🦀️.rs:759`; `brush/🦀️.rs:40`; test `brush_placement_picker_appears_only_for_a_live_brush_target`. |
| Cycle candidates | **FIXED** | `cycleBrushCandidate` commands wired. |
| `addBrushObject` + collision notice | **FIXED** (W-D4) | `🎮️commands/🖌️add-brush-object/🦀️.rs:37` → `ctx.notice`; law `a_refused_placement_surfaces_exactly_one_notice`. |
| `registerBrushMesh` paging | **FIXED** (W-M2) | Paged upload in `World3dHost/🟦️.tsx:4355-4377`; `stage_brush_mesh_page` laws in `⏳️precompute/🧪️`. |
| Brush mesh re-announce after actor restore | **OPEN-OWNED** (W-H) | Guest publishes `meshResidency` / `meshReuploadUrls` (`main/🦀️.rs:389-390`, `precompute/🦀️.rs:1663-1677`). Host `Puzzle3dBrushMeshRegistry` exists (`ShellHelpers/🟦️.tsx:2585-2644`) but **`World3dHost` still uses old `registeredBrushMeshesRef.set` at enqueue** (`World3dHost/🟦️.tsx:4367`) — W-H host wiring incomplete per `📓️2026-09-10-wave-H-brush-mesh-reannounce.md`. |

---

## §10 Volume Brush utility

| Feature | Status | Evidence |
|---|---|---|
| `addTargetVolume` (Alt+click) | **FIXED** | Command + `addTargetVolume` work at `🦀️.rs:3318`. |
| `setVoxelDims` utility options | **FIXED** | `volume_brush/🦀️.rs` options group. |

---

## §11 Relocate utility

| Feature | Status | Evidence |
|---|---|---|
| `worldRelocate` on Concrete Forest | **FIXED** (source) | `world_relocate` command at `🦀️.rs:3156`. |
| `worldRelocate` on Nakagin (180 objects) | **OPEN-UNOWNED** | Extent still `objects.len() + attractions.len()` at `🦀️.rs:3289` vs `PUZZLE_COMMAND_WORK_ITEMS=4096` — fails above ~62 objects with 64 vortices assumed. No per-object vortex count fix found. **BROWSER-ONLY:** relocate one object on Nakagin, watch for work-capacity fault. |
| `relocateTargetVolume` ActionKind honesty | **OPEN-OWNED** (W-U) | Declared `ActionKind::View` while mutating (`wave-N` §5.4). |

---

## §12 Fill tool

| Feature | Status | Evidence |
|---|---|---|
| Activate fill (`setActiveTool`) | **FIXED** (W-D2 coordinator + W-G tab) | `ShellHost/🟦️.tsx:5386-5417` builds full view state for tool dispatch; `reconcileToolTabSelection` at `:8402` (`ShellHelpers/🟦️.tsx:3915`). W-G browser-verified on HMR (`wave-G-shell-tool-tab.md` §5.1). |
| Count slider `puzzle3d-fill-count` / `setFillCount` | **FIXED** (mechanism); **OPEN-OWNED** (ready=0) | Precompute route `Puzzle3dPrecomputeCommandWork` (`🦀️.rs:7168`). Slider `clampToReady` design unchanged. **`ready` stays 0** until fill planner runs — guest **OOM** after ~180 ticks (`runtime-verification.md` §23:20). **OPEN-OWNED (W-F).** |
| Cancel `puzzle3d-play-fill-cancel` | **FIXED** (source) | `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:53-69`. **BROWSER-ONLY:** start fill plan, press cancel. |
| Distribution weight sliders | **FIXED** | `set_object_kind_weight_declares_fill_options_ui_scope`. |
| `fillBuildTick` background driver | **FIXED** (W-J + W-L) | Bounded job: `register_bounded_job_kind(FILL_JOB_KIND)` (`precompute/🦀️.rs:2945`); `fill_faulted` latch (`:2529`). Host: `driveSpawnedJob` (`PluginRuntime/🟦️.tsx:1731`); tick returns dispatch promise (`World3dHost/🟦️.tsx:4532-4534`). Per-tick **memory growth → OOM**: **OPEN-OWNED (W-F).** |
| `fill_and_brush_params` 2 MiB stack overflow | **OPEN-OWNED** (W-F) | Test at `🧪️:2525`; needs `RUST_MIN_STACK=4194304` (`runtime-verification.md` §23:20). |

---

## §13 Vortex suggestions

| Feature | Status | Evidence |
|---|---|---|
| Open popup | **FIXED** (W-D2) | `open_vortex_suggestions_opens_the_suggestion_popup`; window id in `openVortexSuggestions` args. |
| Hover-to-preview candidates | **FIXED** (peer precompute) | Was empty candidates; test `hover_suggestion_updates_the_brush_candidate_index_and_live_preview` (`🧪️:1453`) now requires non-empty `candidates` and brush preview. `open_vortex_suggestions` calls `refresh_brush_candidates` (`🔓️open-vortex-suggestions/🦀️.rs:27`). **BROWSER-ONLY:** open suggestions, hover row, confirm 3D preview moves. |
| Accept / dismiss | **FIXED** (W-D4) | `accept_suggestion_closes_menu_even_when_placement_fails`; notices on rejection. |
| Close | **FIXED** | `close_vortex_suggestions_clears_the_menu`. |
| `suggestionsTick` | **FIXED** | Tick loop gated `World3dHost/🟦️.tsx:4521-4526`. |

---

## §14 Engagement bar

| Feature | Status | Evidence |
|---|---|---|
| `engagementInput` / `Submit` / `Abort` / `RepeatLast` / `ControlSelect` | **FIXED** (source) | Commands exist; submit parser at `📨️engagement-submit/🦀️.rs:12-38`. |
| Advertised verbs `clear`/`pick`/`rectangle`/`lasso` | **FIXED** (W-D4) | `PUZZLE3D_ENGAGEMENT_VERBS` constant; arms at `:33-38`; law `every_advertised_engagement_verb_is_implemented`. |
| `rectangle` vs `pick` visually identical | **OPEN-UNOWNED** | W-D4 §5.3: host marquee only branches on `"lasso"` (`World3dHost/🟦️.tsx`); value changes, draw does not. |
| Full engagement bar live | **BROWSER-ONLY** | Type `fill 5`, `brush`, `zoom`, `clear` in engagement input after boot. |

---

## §15 Context menu

| Feature | Status | Evidence |
|---|---|---|
| Zoom to Selection → `focusSelection` | **FIXED** (W-D4) | Rows at `🦀️.rs:2512,2533,2568` dispatch `"focusSelection"`; law `every_context_menu_row_dispatches_a_declared_action`. |
| Group labels EN/DE | **FIXED** (W-L + W-N) | `mapContextMenuSpecs` → `contextMenuGroupLabel` (`World3dHost/🟦️.tsx:1205`); wgpu twin W-N. |
| Wrong rows (Delete Attraction on object) | **FIXED** (W-L) | `deleteAttraction`/`deleteTargetVolume`/`setTargetVolumeFlag` `.in_palette(false)` (`🦀️.rs:7721-7724`). |
| Plugin menu vs shell fallback when guest returns `[]` | **OPEN-UNOWNED** | W-L §5.1: `requestContextMenu` still conflates failure with empty; shell fallback may still appear if context menu build fails. **BROWSER-ONLY:** right-click selected object — expect plugin vocabulary ("Duplicate", "Select all of same kind"), not shell window-action labels. |
| Empty-selection duplicate no-op | **FIXED** (W-L) | `refuse_without_selection` (`🦀️.rs:2397`); law `selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice`. |

---

## §16 Inspection panel

| Feature | Status | Evidence |
|---|---|---|
| Per-entity field groups | **FIXED** (W-S) | `inspection::render` at `🦀️.rs:7530`; tests `selected_object_inspector_renders_that_object_field_group` (`🧪️:1089`), vortex at `:1111`. |
| UI scope refreshes inspector on selection | **FIXED** (W-N) | `puzzle3d_scope` table `Selection` class names inspector body (`🦀️.rs:1978-2063`). |
| Empty tree blank rectangle | **FIXED** (W-N) | `TreeView` empty state (`Interpreter/🟦️.tsx:1245-1250`). |
| Live panel after viewport pick | **BROWSER-ONLY** | Pick object → Inspection tab shows `puzzle3d-play-inspector.object.*` rows (not empty summary). On #26, blocked until intake stall cleared. |

---

## §17 Artifact / outliner panel

| Feature | Status | Evidence |
|---|---|---|
| Document tree structure + `interactionSelect` | **FIXED** | `document/🦀️.rs`; panel tests in `📌️panels/🗿️artifact/🧪️`. |
| Hide/lock inline toggle inverse | **FIXED** (W-D4) | `flag_args(entity, id, flag, value)` at `artifact/🦀️.rs:127`; `hide_lock_actions` passes `!hidden`/`!locked` at `:136,142`. Laws `outliner_hide_and_lock_rows_dispatch_the_inverse_of_the_current_flag`, `an_outliner_flag_row_undoes_itself_on_the_second_click`. |
| Continuation `+N` row inert | **OPEN-OWNED** (W-U) | W-U item #3 investigating. |

---

## §18 Catalogue panel

| Feature | Status | Evidence |
|---|---|---|
| Add object of kind (click) | **FIXED** | `addObjectKind` from catalogue; empty-doc path §5. |
| Drag kind onto viewport | **BROWSER-ONLY** | Drag `PUZZLE3D_CATALOGUE_DRAG_MIME` row onto perspective canvas; confirm object at drop. |
| Paged catalogue | **FIXED** (W-O) | Continuation rows + page guard in `catalogue/🦀️.rs` tests. |

---

## §19 Settings panel

| Feature | Status | Evidence |
|---|---|---|
| Four steppers (overlap, proximity, chunk, grid) | **FIXED** | `settings/🦀️.rs`; publish `WindowConfig` lane (`🦀️.rs:6317-6329` area). |
| Per-window vs global UX | **BROWSER-ONLY** | Split focus: change grid spacing from Settings in top pane only; confirm perspective pane unchanged. |

---

## §20 History panel

| Feature | Status | Evidence |
|---|---|---|
| Undo / redo / checkpoint / revert | **FIXED** (W-D2 + wasm pump) | `run_framework_reserved_job` pumps pool on wasm (`plugin/🦀️.rs:18614,18629`). Rebuild #11: `noteShellCommand` OK 1.6s (`runtime-verification.md` §17:10). |
| History description overflow | **FIXED** | `UiText::clipped` (`plugin/🦀️.rs:9837`); contract test `ui_text_clipped_keeps_short_values…`. |
| History panel live undo | **BROWSER-ONLY** | Edit → Undo; confirm no hang and document reverts. |

---

## §21 Copy / cut / paste

| Feature | Status | Evidence |
|---|---|---|
| Puzzle3d-specific clipboard | **OPEN-UNOWNED** | No `clipboard`/`copy`/`cut`/`paste` in puzzle3d editor (`rg` over `✏️editor/` — zero handler matches). Framework may expose menu items; if so, same reserved-route class as §20. **BROWSER-ONLY:** check Edit menu for Copy/Paste on selection. |

---

## §22 Delete / duplicate / focus

| Feature | Status | Evidence |
|---|---|---|
| Delete | **FIXED** | `delete_selection` command. |
| Duplicate (Ctrl/Cmd+D) | **FIXED** (W-S) | `duplicate_selection_reselects_the_created_clones`. Empty selection refuses with notice (W-L). |
| Focus selection (F) | **FIXED** | `focusSelection` work; keybinding `🦀️.rs:7700`. |
| Context menu Zoom | **FIXED** | §15. |

---

## §23 Add Object dialog

| Feature | Status | Evidence |
|---|---|---|
| Dialog opens | **FIXED** | `openAddObjectDialog` → `Effect::OpenDialog` (`🦀️.rs:2955` area). |
| Kind select from live catalog | **FIXED** (W-D4) | `puzzle3d_object_kind_options()` at `🦀️.rs:7634`; law `the_add_object_dialog_offers_every_object_kind_of_both_examples`. |

---

## §24 Import / export

| Feature | Status | Evidence |
|---|---|---|
| End-user import/export | **OPEN-UNOWNED** | No import/export command files in editor; `setFixtureJson` not exposed as UI. `setActiveExample` is the working substitute. |

---

## §25 Locale / terminology

| Feature | Status | Evidence |
|---|---|---|
| EN+DE labels compile | **FIXED** | `app_labels!`; German test sets axes explicitly (`🧪️:1296-1314` `set_label_axes(De, Reuse)`). |
| German document tree in live shell | **BROWSER-ONLY** | Switch shell locale to DE; confirm outliner sections show e.g. `Baukomponenten` not `Objects`. |

---

## §26 Ranked OPEN-UNOWNED defects (user impact)

1. **`worldRelocate` work-capacity fault on Nakagin** — extent `objects + attractions` at `🦀️.rs:3289` exceeds `PUZZLE_COMMAND_WORK_ITEMS` for ~180 objects; relocate unusable on the large example. *Probe:* Nakagin → Relocate utility → drag object.

2. **Copy / cut / paste absent** — no puzzle3d clipboard implementation; duplicate is the only clone path. *Probe:* select object → Edit menu / Cmd+C.

3. **Import / export not available** — no UI path to load arbitrary fixture JSON. *Probe:* search menus for Import/Export.

4. **`setActiveExample` per-window runtime reset unverified** — local `Puzzle3dRuntime::default()` at `set-active-example/🦀️.rs:20` may not round-trip through `WindowConfig`; camera/grid may not visibly reset on example switch. *Probe:* note camera pose → switch example → check if camera/grid reset.

5. **Context menu shell fallback when plugin returns `[]`** — W-L fixed wrong palette rows but not empty-vs-error conflation; user may still see shell window-action menu instead of plugin menu. *Probe:* right-click selection; compare row labels to checklist §15 vocabulary.

6. **Engagement `rectangle` vs `pick` indistinguishable in viewport** — method stored but marquee draw unchanged (W-D4 flagged). *Probe:* type `rectangle` then `pick`, drag marquee — same shape?

7. **Locked-volume gumball empty completion** — W-N §5.6: selection of locked volumes passes `Puzzle3dScaleWork` guard, completes with empty mutation. *Probe:* lock object → Transform → drag gumball.

8. **`relocateTargetVolume` / `worldRelocate` declared `View` while mutating** — undo/history semantics wrong (W-N §5.4); W-U may fix. *Probe:* relocate → History panel entry kind.

---

## Wave-claim summary (OPEN-OWNED only)

| Wave | Checklist areas still owned |
|---|---|
| **W-F** | Fill guest OOM (~2.8 MB/tick), `ready` stays 0, `fill_and_brush_params` 2 MiB stack |
| **W-H** | Brush mesh re-announce: guest `meshResidency`/`meshReuploadUrls` landed; **host `World3dHost` not wired to `puzzle3dBrushMeshRegistry`** |
| **W-P / W-P2** | Nakagin scene via paged lanes (**Rust+TS laws green**); **browser intake stall** `plugin-ui.intake-budget-exhausted` on #26 blocks refresh/pick |
| **W-U** | Gumball undo coalesce, relocate ActionKind, outliner continuation row, inspection ids bound |

---

## Commands run (audit A1)

```
rg -a (multiple) over ✏️editor/, 🧰️framework/…/ShellHost, World3dHost, ShellHelpers, Interpreter, PluginRuntime, plugin/🦀️.rs
Read: checklist, wave reports D4/J/K/L/N, runtime-verification, wave-G/H/U/coordination docs
No cargo, no vitest, no browser
```
