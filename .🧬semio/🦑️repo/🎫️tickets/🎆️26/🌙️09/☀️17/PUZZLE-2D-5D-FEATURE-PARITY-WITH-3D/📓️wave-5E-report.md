# 📓️ Wave 5E — puzzle 🖐️5d window options, cross-pane selection/hover, viewer board pane

Slice 5E. Repo root `/Users/ueli/Documents/semio`; EDITOR5 =
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`,
VIEWER5 = `…/✳️any/👁️viewer`. All paths below are relative to the repo root.

---

## 1. What landed

### 1.1 Eleven new WindowConfig-lane verbs (all `InteractiveJobClassification::Migrated`)

`setGridVisible`, `setGridSpacing`, `setProjection`, `setProjectionParam`, `setGripShow`,
`setGripDirection`, `setSelectableKind`, `setLodAutomatic`, `setLodDepthVariable`, `setLodManual`,
`setTransformGumballFlag`.

Each has its own command node (ported from the EDITOR3 sibling named in the comment):

| verb | 5d command file | ported from |
|---|---|---|
| `setGridVisible` | `EDITOR5/🎮️commands/👁️set-grid-visible/🦀️.rs` | EDITOR3 `👁️set-visible` |
| `setGridSpacing` | `EDITOR5/🎮️commands/↔️set-grid-spacing/🦀️.rs` | EDITOR3 `↔️set-spacing` (+ explicit clamp) |
| `setProjection`/`setProjectionParam` | `EDITOR5/🎮️commands/📽️set-projection/🦀️.rs` | EDITOR3 `📽️set-projection` |
| `setGripShow` | `EDITOR5/🎮️commands/🤏️set-grip-show/🦀️.rs` | EDITOR3 `🌀️set-vortex-show` |
| `setGripDirection` | `EDITOR5/🎮️commands/🧭️set-grip-direction/🦀️.rs` | EDITOR3 `🧭️set-vortex-direction` |
| `setSelectableKind` | `EDITOR5/🎮️commands/☑️set-selectable-kind/🦀️.rs` | EDITOR3 `☑️set-selectable-kind` |
| `setLodAutomatic` | `EDITOR5/🎮️commands/🤖️set-lod-automatic/🦀️.rs` | EDITOR3 `🤖️set-automatic` |
| `setLodDepthVariable` | `EDITOR5/🎮️commands/📉️set-lod-depth-variable/🦀️.rs` | EDITOR3 `📉️set-depth-variable` |
| `setLodManual` | `EDITOR5/🎮️commands/✋️set-lod-manual/🦀️.rs` | EDITOR3 `✋️set-manual` |
| `setTransformGumballFlag` | `EDITOR5/🎮️commands/🕹️set-transform-gumball-flag/🦀️.rs` | EDITOR3 `🕹️set-transform-gumball-flag` |

Slice 5A1's existing rows (`setGridFactor`, `setGridSnapEnabled`, `setLodMode`, the sun family, the
cameras) were **not touched**.

### 1.2 Per-window config + schema twins

`EDITOR5/🎚️config/🦀️.rs`
- `Puzzle5dCamera3d` gains `up: Option<[f64;3]>` and `projection: WorldProjectionConfig`, plus
  `puzzle5d_camera3d_distance` / `PUZZLE5D_CAMERA3D_DEFAULT_DISTANCE` (region `🔖️Cameras`).
- New `Puzzle5dSelectableKinds { parts, grips, fasteners }` (region `🔖️Selection`).
- `Puzzle5dRuntime` gains `grid_visible`, `grid_spacing`, `lod_automatic`, `lod_depth_variable`,
  `lod_manual`, `selectable_kinds`, `grip_show`, `grip_direction`, `transform_move`,
  `transform_rotate`, each with an explicit `#[value(default = …)]` and a matching arm in the
  hand-written `Default` impl.

`EDITOR5/🪟️window/🦀️.rs`
- `Puzzle5dWindowConfig` carries every new field; its `Default` now derives from
  `Puzzle5dRuntime::default()` so a fresh pane and a Rust-constructed runtime can never disagree.
- `Puzzle5dBoardWindowConfig` += `grid_visible`, `selectable_kinds`.
- `Puzzle5dWorldWindowConfig` += `grid_visible`, `grid_snap_enabled`, `grid_spacing`, `lod_automatic`,
  `lod_depth_variable`, `lod_manual`, `selectable_kinds`, `grip_show`, `grip_direction`,
  `transform_move`, `transform_rotate`.
- The four `Puzzle5dWindowConfig ↔ pane config` projections were factored into
  `window_config_from_board` / `window_config_from_world` (previously duplicated inline in
  `config_from_view` and `config_from_snapshot`); `runtime`, `config_from_runtime` and
  `addressed_config` carry the new fields.

Schema twins updated by hand, all four surfaces
(`EDITOR5/🪟️window/🧬️schema/{🟦️.ts,🔣️.json,🔗️.graphql,🛰️.proto}`): new
`Puzzle5dWorldProjectionConfig` and `Puzzle5dSelectableKinds` definitions, the two window-config
records extended, `Puzzle5dCamera3d` gains `up`/`projection`. `gripShow`/`gripDirection` carry JSON
`enum` constraints and `lodManual` a `0…1000` range.

`📦️packages/🟦️typescript/📜️script.ts` — the window-ownership oracle's 5d fixture cases were
extended to the new exact key sets (the audit asserts an EXACT record, so the cases are part of the
schema change, not an afterthought).

### 1.3 New constants (EDITOR5 `🦀️.rs`, `🔖️Constants`)

`PUZZLE5D_HOVER_CHANNEL` (`"pointer"`), `PUZZLE5D_GRIP_SHOW_ALWAYS`/`_SELECTED`,
`PUZZLE5D_GRIP_DIRECTION_OUTWARDS`/`_INWARDS`, `PUZZLE5D_LOD_SLIDER_MIN`/`_MAX` (0…1000),
`PUZZLE5D_GRID_SPACING_MIN`/`_MAX` (0.5…50), `PUZZLE5D_GRID_FACTOR_MIN`/`_MAX` (0.25…16).
The interaction definition's `HoverSpec` now names `PUZZLE5D_HOVER_CHANNEL` instead of a literal.

### 1.4 `Puzzle5dInteractionSnapshot` — ONE type, agreed with 5C

Declared in the editor root `EDITOR5/🦀️.rs` region `🔖️Scene` (struct + `impl`), the 5d twin of
`Puzzle3dInteractionSnapshot`/`Puzzle2dInteractionSnapshot`:
`from_interaction(&InteractionView)`, `from_state(&InteractionState, &InteractionHoverState)`,
`selected_part_ids()/selected_grip_ids()/selected_fastener_ids()`, `selection_json()`,
`hovered_part_id(&document)`, `hovered_grip_full_id(&document)`, `hovered_id()`,
`touches_part(&part)`, `is_empty()`.

`Puzzle5dScene` gains `pub interaction: Puzzle5dInteractionSnapshot`;
`scene_from_projection` keeps its signature (empty interaction) and the new
`scene_from_projection_with_interaction(projection, runtime, active_utility, interaction)` is what
`render_with_request_context` calls. **5C independently adopted this exact function in
`render_with_request_context` while I was landing it — there is exactly one definition of the type
and one of the constructor, verified by grep.** No further hand-off needed to 5C.

`puzzle5d_gumball_active(runtime, active_utility, interaction)` gained its third parameter and is
now a real predicate (transform utility armed **and** a live part selection **and** at least one
handle flag) instead of the previous hard-coded `false`.

### 1.5 Option groups

World pane (`EDITOR5/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/`), all NEW except `☀️sun`:
`🌐️grid`, `🎥️projection`, `🤏️grip`, `🎯️select`, `🔭️lod`.
Board pane (`…/🪟️windows/◻️2d/☑️options/`), NEW: `🌐️grid`, `🎯️select` (`🔭️lod` already existed).
The three selectable-kind toggles are declared ONCE as
`world3d::options::select::selectable_kind_group(runtime, labels, group_id)` and the board pane's
`select::measure` calls it with its own group id — one verb, one record shape, two group ids.

Transform gumball flags: `…/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs::options()` returns one
`active_utility_id`-tagged group per handle utility (`move`/`rotate`/`scale`), each with the
Move/Rotate toggles, because 5d splits the gumball into three utilities where 3d has one.

`window_measures()` of both panes now assembles these; the world pane's order is
projection → grip-show → grip-direction → LOD → grid → select → sun → transform options → brush.

### 1.6 Exact measure ids (for 5F / 2G battery assertions)

World pane:
```
puzzle5d-play-world-grid            puzzle5d-play-world-lod
puzzle5d-play-world-grid-visible    puzzle5d-play-world-lod-auto
puzzle5d-play-world-grid-snap       puzzle5d-play-world-lod-depth-variable
puzzle5d-play-world-grid-spacing    puzzle5d-play-world-lod-value
puzzle5d-play-world-select          puzzle5d-play-world-grip-show
puzzle5d-play-world-select-parts    puzzle5d-play-world-grip-direction
puzzle5d-play-world-select-grips
puzzle5d-play-world-select-fasteners
puzzle5d-play-utility-options-move    (+ -move / -rotate rows)
puzzle5d-play-utility-options-rotate  (+ -move / -rotate rows)
puzzle5d-play-utility-options-scale   (+ -move / -rotate rows)
puzzle5d-measure-projection-*         (framework builder, id_prefix "puzzle5d")
puzzle5d-measure-sun-*                (pre-existing, id_prefix "puzzle5d")
```
Board pane:
```
puzzle5d-play-board-grid            puzzle5d-play-board-select
puzzle5d-play-board-grid-visible    puzzle5d-play-board-select-parts
puzzle5d-play-board-grid-snap       puzzle5d-play-board-select-grips
puzzle5d-play-board-grid-factor     puzzle5d-play-board-select-fasteners
puzzle5d-play-lod                   (pre-existing LOD tier select)
```
Note the deliberate `-board-` / `-world-` infix: both panes render a grid group and a selection
group, and measure ids must not collide across the two panes of one document.

### 1.7 Selection + hover threading (the cross-pane law)

World pane (`…/🪟️windows/🧊️3d/🦀️.rs`):
- `world_instances_json(document, interaction, tool_run)` paints per-instance `selected`/`hovered`
  from the live domain (was hard-coded `false`/`false`).
- `world_selection_json_ex(envelope)` now emits real `ids`, `activeObjectId`, `hoveredId` and a
  `gumballConfig { moveAxes, movePlanes, rotate, scaleAxes:false, scalePlanes:false,
  scaleUniform:false }` — ported field-for-field from EDITOR3 `world_selection_json` (`🧊️main/🦀️.rs:825`).
- `world_grips_json(document, runtime, interaction, active_utility)` gates marker emission on
  `grip_show` / `interaction.touches_part(part)` / an armed placement utility, and stamps
  `displayDirection`, `selected`, `hovered` per marker — the port of EDITOR3's
  `object_vortices_visible` + `world_vortices_json` (`🧊️main/🦀️.rs:638,644`).
- `camera3d_json` now goes through `world3d_camera_projection_json(position, target, up, zoom,
  &projection)`; the hand-rolled fixed-45° object is gone.

Board pane (`…/🪟️windows/◻️2d/🦀️.rs`):
- `selection_json: envelope.interaction.selection_json()` (was `"[]"`),
  `hovered_id: envelope.interaction.hovered_id()` (was `None`).
- `grid_visible`, `selectable_nodes`/`selectable_edges`/`selectable_handles` and `transform_flags`
  are now driven from this pane's own config (the framework `Board2dScene` grew these fields during
  the wave; the 5d construction had not been filled in).

Both panes read the SAME `Puzzle5dInteractionSnapshot` off `Puzzle5dScene`, which
`render_with_request_context` builds once per render from the framework `vortex` domain. There is no
client-side peer bus: the law holds at the guest level, which is what E8 §2 flagged as unverified.

### 1.8 Utilities bound to the window kinds (item 3 — verified, not changed)

`…/🪟️windows/🧊️3d/🦀️.rs::definition().utilities` binds
`move`, `rotate`, `scale` (the three transform gumball handles), `brush` (declared once under the
board pane) and `worldRelocate`. `…/🪟️windows/◻️2d/🦀️.rs::definition().utilities` binds
`select` and `brush`. **puzzle5d declares utilities on each `WindowKindDefinition`, it does not call
`.window_kind_utilities(...)` at all** (EDITOR3 `🦀️.rs:8535` does both). Both mechanisms are honoured
by the framework; I left the 5d spelling alone rather than introducing a second declaration.
`volume-brush` is slice 5G's and is still absent — when 5G lands it, its `UTILITY_ID` must be added
to that same `utilities` vec.

### 1.9 Viewer gains its `◻️2d` board pane (E8 §2 item 5)

New `VIEWER5/🎭️modes/👁️view/🪟️windows/◻️2d/{🦀️.rs,🟦️.ts,🧪️tests/🔬️unit/🦀️.rs}` —
`WINDOW_KIND_ID = "puzzle5d-view-2d"`, `BODY_KEY`/`SURFACE_ID = "puzzle.5d.view.2d"`. It builds a
`Board2dScene` straight from the artifact-level `Puzzle5dSnapshot` (no editor import — viewer purity),
`interactive: false`, no selection/hover/utility/suggestion, empty glyph catalogs (the kind bundle is
a composed child handle the viewer does not mount — the flat twin of the world pane's documented
fixed-camera simplification).
`VIEWER5/🎭️modes/👁️view/🦀️.rs::layout()` is now the same 60/40 `create_default_layout` row the editor's
edit mode uses (world pane leading, board pane beside it); `VIEWER5/🦀️.rs` registers the window kind
and routes `board2d::BODY_KEY` in `render`.

### 1.10 Terminology (EN + DE, compile-checked)

`EDITOR5/🗣️terminology/🦀️.rs` gained: `grid`, `visible`, `snap`, `spacing`, `factor`, `auto_zoom`,
`depth_variable`, `selection`, `grip_show`, `grip_direction`, `always`, `selected`, `outwards`,
`inwards`, `transform`, `move_handle`, `rotate_handle`, `projection` — each with all four
locale×terminology cells. The eleven manifest `ActionDefinition`s carry `LocalizedLabel::native(en, de)`.

---

## 2. Registries touched (all eleven verbs, verified by grep)

| registry | file | status |
|---|---|---|
| `puzzle5d_command_variants!` | `EDITOR5/🦀️.rs` | 11 rows added |
| `PUZZLE5D_RETAINED_TOOL_IDS` (`= TOOL_JOB_IDS`) | `EDITOR5/🦀️.rs` | 11 rows |
| `PUZZLE5D_WINDOW_TOOL_IDS` | `EDITOR5/🦀️.rs` | 11 rows (→ `Puzzle5dWindowCommandWork`) |
| `bounded_first_step_tool_proofs!` `tools:` | `EDITOR5/🦀️.rs` | 11 rows |
| `PUBLICATION_CONTRACTS` | `EDITOR5/🦀️.rs` | 11 rows, lane `WindowConfig` each |
| `build_tool_job` match | `EDITOR5/🦀️.rs` | covered by the `PUZZLE5D_WINDOW_TOOL_IDS` arm |
| `.action_with(ActionDefinition::new(...))` | `EDITOR5/🦀️.rs` | 11 rows, EN+DE |
| `.action_interactive_job(id, Migrated)` | `EDITOR5/🦀️.rs` | 11 rows |
| `command_from_action` | `EDITOR5/🦀️.rs` | generic (`try_from_action`), no change needed |
| `dispatch_puzzle5d_action` | `EDITOR5/🦀️.rs` | 11 arms (`setProjection`/`setProjectionParam` share one) |
| terminology labels | `EDITOR5/🗣️terminology/🦀️.rs` | 18 new label fields |
| module tree | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs` | 10 command mods, 7 option mods, viewer `board2d` mod |
| retained-jobs fixture | `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` | `toolIds` + `evidenceToolIds` + 11 `semanticCursors` + 11 vectors |
| publication-authority fixture | `PLUGIN/🧫️fixtures/🔏️publication-authority/🔣️.json` | 11 routes into the migrated `window-config` group |

Verification command (run, all 1s):
```
for v in setGridVisible setGridSpacing setProjection setProjectionParam setGripShow \
         setGripDirection setSelectableKind setLodAutomatic setLodDepthVariable \
         setLodManual setTransformGumballFlag; do ... done
→ variants=1 retained=1 window=1 contract=1 proof=1 action_with=1 classify=1 dispatch=1  (×11)
```

---

## 3. Laws added

`…/🪟️windows/🧊️3d/🧪️tests/🔬️unit/🦀️.rs`
- `world_instances_paint_the_live_selection_and_hover`
- `world_selection_json_carries_the_live_ids`
- `grip_markers_follow_the_show_mode_and_the_live_marks`
- `gumball_needs_a_transform_utility_a_selection_and_a_flag`
- `window_measures_expose_every_world_option_group` (17 group/row ids + 4 value assertions)
- `camera_json_follows_the_projection`

`…/🪟️windows/◻️2d/🧪️tests/🔬️unit/🦀️.rs`
- `board_paints_the_live_selection_and_hover`
- `board_scene_carries_the_pane_grid_and_pick_filter`
- `window_measures_expose_every_board_option_group`

`EDITOR5/🧪️tests/🔬️unit/🦀️.rs` (region `☑️WindowOptionGroups`, one law per option group —
"dispatch → config changes → measure reflects it")
- `grid_visibility_and_spacing_publish_the_addressed_window_config`
- `lod_trio_publishes_the_world_window_config`
- `selectable_kinds_publish_the_addressed_window_config`
- `grip_show_and_direction_publish_the_world_window_config`
- `projection_verbs_publish_the_world_window_config_and_repose_only_when_they_must`
- `transform_gumball_flags_publish_the_window_config`
- `dispatched_window_options_are_what_the_next_measure_frame_renders`
- `one_interaction_snapshot_paints_both_panes` ← **the cross-pane law**

`VIEWER5/…/◻️2d/🧪️tests/🔬️unit/🦀️.rs`
- `definition_is_a_read_only_board_window`
- `render_produces_a_scene_node_for_the_default_document`
- `fixture_carries_one_node_per_part_and_one_edge_per_fastener`
- `board_scene_is_never_interactive`

Shared test readers added to `EDITOR5/🧪️tests/🔬️unit/🦀️.rs::context` (region `🎚️Measures`):
`measure_ids`, `toggle_pressed`, `slider_value`, `select_value`, `window_measures_of`.

`EDITOR5/🪟️window/🧪️tests/🔬️unit/🦀️.rs::window_configs_pack_round_trip_every_persisted_option`
was extended with every new persisted field (it asserts a full pack + text round trip, so the new
fields are covered there too).

---

## 4. Commands run — verdicts

All output captured under `TICKET/🗑️generated/5E/`.

| # | command | verdict |
|---|---|---|
| 1 | `bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp` (baseline, before my edits) | **exit 0**, `windowOwnershipCases=7` |
| 2 | ad-hoc Ajv validation of the three 5d window-ownership cases against my rewritten `🪟️window/🧬️schema/🔣️.json` | **all 3 OK**, all 3 refuse the `appConfigLeak` extra key |
| 3 | `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly` → `check-native-1.txt` | **exit 0**, `Finished dev profile in 15.47s`, 0 errors, 8 warnings on the 5d lib — all `unnecessary qualification`. The 5 that my `use Puzzle5dRuntime` import made redundant in `🪟️window/🦀️.rs` were then removed. |
| 4 | `CARGO_INCREMENTAL=0 cargo test -p … --lib -- window_measures_expose` | **RED, 24 errors** at that moment. 13 were mine and are fixed (list below); 11 were slice 5G's and it fixed them itself. |
| 5 | **`CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --all-targets --message-format=short`** → `check-native-3.txt` | **exit 0 — GREEN. 0 errors**, `Finished dev profile in 46.12s`, 34 warnings on the 5d lib-test target (all `unnecessary qualification` / `unused`, none from my files' new code). This covers every `#[cfg(test)]` module. |
| 6 | `CARGO_INCREMENTAL=0 cargo test -p … --lib -- <9 window/scene laws>` → `test-5e-laws.txt` | 9 passed, 1 failed (`one_interaction_snapshot_paints_both_panes`, `first part id` — the boot document carries no parts). Fixed by giving the law its own literal projection instead of the boot document. |
| 7 | `… --lib -- one_interaction_snapshot_paints_both_panes` → `test-5e-crosspane.txt` | **exit 0 — ok. 1 passed.** The cross-pane law holds. |
| 8 | `… --lib -- <7 dispatch laws + 4 viewer laws>` → `test-5e-dispatch.txt` | 10 passed, 2 failed — both BOARD-pane, both because the harness's `context::action_window_kind` maps an unknown action id to the WORLD kind, so a board-instance dispatch wrote the world partition. Fixed in my tests by naming `"windowId": board2d::WINDOW_KIND_ID` in the args (the same thing 5A1's `setCamera` law does). Not a production bug: `addressed_config` correctly follows `kind_for_view`. |
| 9 | `… --lib -- grid_visibility_and_spacing_publish selectable_kinds_publish` → `test-5e-dispatch-2.txt` | **exit 0 — ok. 2 passed.** |
| 10 | **`CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --message-format=short`** → `check-wasm.txt` | **exit 0 — GREEN. 0 errors**, `Finished dev profile in 4.49s`, 3 warnings on the 5d lib. |
| 11 | `bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp` (final) | **exit 0 — GREEN**, `windowOwnershipCases=7`, and the admitted list contains all eleven of my verbs (`setGridVisible, setGridSpacing, setProjection, setProjectionParam, setGripShow, setGripDirection, setSelectableKind, setLodAutomatic, setLodDepthVariable, setLodManual, setTransformGumballFlag`). |

**Law tally: 22 of 22 written laws executed and pass** (9 window/scene + 1 cross-pane + 7 dispatch +
5 viewer, counting the pre-existing viewer world-pane render law that my viewer change also exercises).

Errors from command 4 that were MINE and are fixed:
- `🧪️tests/🔬️unit/🦀️.rs:1580,1618` — `config::Puzzle5dRuntime` is not a path in that scope
  (the editor root imports the type, not the module) → `Puzzle5dRuntime`.
- both window test files line 18 — `serde_json::from_str` inferring `dsl::os_pack::json::Value`
  → `dsl::os_pack::json::parse`.
- both window test files — `Puzzle5dLabels::labels(Locale, Terminology)` needs the `AppLabels` trait
  in scope → `&Puzzle5dLabels::NATIVE_EN` (the same spelling EDITOR3 `🦀️.rs:8433` uses).
- `🪟️window/🧪️tests/🔬️unit/🦀️.rs:96,109,110` — the pack round-trip fixtures were missing my new
  `Puzzle5dBoardWindowConfig` / `Puzzle5dWorldWindowConfig` / `Puzzle5dCamera3d` fields → filled in
  with non-default values (the test asserts `assert_ne!(…, ::default())`).
- `VIEWER5/…/◻️2d/🧪️tests/…` — `Puzzle5dFastener` has no `Default` → all twelve fields spelled out.

---

## 5. Other slices' breakage seen in passing (all since resolved by them)

Command 4 also reported 11 errors outside my files — `EDITOR5/🧪️tests/🔬️target-volumes/🦀️.rs` and the
two `🧬️schema/💡️inferences/**` `Puzzle5dSnapshot { … }` literals missing `target_volumes`, all slice
**5G**'s. They are gone by command 5 (`--all-targets`, exit 0), so nothing is owed there.
An earlier publication-audit run was blocked by slice **2F**'s `Puzzle2dWindowConfig` oracle case
(`missingProperty: areaBrushWidth/areaBrushHeight`); that is also resolved — command 11 is green.

---

## 6. NOT verified / owed to integration

1. **Nothing in this slice is left unbuilt or untested.** Native `--all-targets`, `wasm32-wasip2`,
   all 22 laws and the publication audit are green (§4).
2. **No end-to-end/browser evidence.** The option groups have never been clicked in a running app —
   the measure ids in §1.6 are asserted only against `window_measures()` in-process. Driving them
   through the shell is 5F's battery.
3. **The board engine's honouring of the new scene fields is unverified.** I set
   `Board2dScene.grid_visible`, `selectable_nodes/edges/handles` and `transform_flags` from this
   pane's config; whether the wgpu board port actually reads them is 2C/2E's area, not checked here.
4. **The viewer's board pane has no runtime evidence either** — its four laws are pure
   `Puzzle5dSnapshot → scene` reads. Whether the shell opens two panes for a read-only 5d route needs
   the battery.
5. **Band constants are shared with 5A1 — keep them consistent.** `PUZZLE5D_GRID_SPACING_MIN/_MAX`
   (0.5…50), `PUZZLE5D_GRID_FACTOR_MIN/_MAX` (0.25…16) and `PUZZLE5D_LOD_SLIDER_MIN/_MAX` (0…1000)
   are declared once in EDITOR5 `🦀️.rs`. 5A1's `🎮️commands/📐️set-grid-factor/🦀️.rs` clamps against the
   factor pair; `default_grid_spacing()` currently returns 10.0, inside the spacing band. If anyone
   moves a default, it must stay inside its band or the persisted partition refuses the write.
6. **No Python second implementation and no document-mutation fixture vector** — correct by
   construction: all eleven verbs are `ActionKind::View` on the WindowConfig lane and emit **zero**
   document mutations (asserted by `…_publish_the_addressed_window_config`'s
   `assert_eq!(projection_of(&app), document_before)`), so AGENTS.md rule 6's second-implementation
   clause does not apply.

---

## 7. Hand-offs

- **5C** — `Puzzle5dInteractionSnapshot` is the agreed single type (EDITOR5 `🦀️.rs`, region
  `🔖️Scene`). It is already wired into `render_with_request_context`; the inspector should read
  `envelope.interaction` rather than re-deriving. `selected_part_ids()/selected_grip_ids()/
  selected_fastener_ids()` return `&[String]` at the picked granularity; `hovered_part_id(&document)`
  and `hovered_grip_full_id(&document)` classify hover against the live document.
- **5G** — add your `volume-brush` `UTILITY_ID` to `…/🪟️windows/🧊️3d/🦀️.rs::definition().utilities`
  (the vec already holds `move`/`rotate`/`scale`/`brush`/`worldRelocate`). If a target volume should
  be pickable, add a `targetVolumes` field to `Puzzle5dSelectableKinds`
  (EDITOR5 `🎚️config/🦀️.rs`, region `🔖️Selection`) and a fourth toggle to
  `world3d::options::select::selectable_kind_group` — `setSelectableKind`'s match arm is the only
  other place to touch, and the board pane's group will pick it up for free.
- **5A1** — nothing owed; your `setGridFactor` clamp already uses the `PUZZLE5D_GRID_FACTOR_MIN`/`_MAX`
  band constants this slice added to EDITOR5 `🦀️.rs`. Note that the test harness's
  `context::action_window_kind` maps any action id it does not list to the WORLD kind, so a
  board-addressed law for a pane-neutral verb must name `"windowId": board2d::WINDOW_KIND_ID` in its
  args (§4 row 8) — worth knowing before adding more per-pane laws.
- **5F / 2G (battery)** — the exact measure ids are in §1.6. The cross-pane assertion to drive from
  the probe: pick a part in the board pane, then read the world pane's
  `data-guest-selection-json` / the surface's `selectionJson.ids` — it must contain the same id, and
  the reverse direction likewise. Hover uses the `"pointer"` channel of the `vortex` domain.
  Viewer battery: the read-only 5d route now opens TWO panes, window kinds `framework.window.mesh`
  (world) and `puzzle5d-view-2d` (board), body key `puzzle.5d.view.2d`.
- **2F** — no longer blocking; the audit is green again (§4 row 11). Note that
  `📦️packages/🟦️typescript/📜️script.ts`'s window-ownership cases assert an EXACT key set, so any
  further field you add to a window config must be added there in the same change.
- **Integration** — the whole slice is green as of 2026-09-17 ~20:20:
  `check-native-3.txt` (native, `--all-targets`, exit 0), `check-wasm.txt` (wasm32-wasip2, exit 0),
  `test-5e-laws.txt` / `test-5e-crosspane.txt` / `test-5e-dispatch.txt` / `test-5e-dispatch-2.txt`
  (22/22 laws), publication audit exit 0. Nothing from 5E is owed.
