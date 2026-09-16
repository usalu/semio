# 🖱️ Framework UI primitives, outliner/inspector/animation precedents (2026-09-16)

Read-only exploration (Sonnet). Neither fem2d nor fem3d has a `📌️panels` directory; conventions come from sibling plugins.

## 1. UI vocabulary (Rust builder API)

Builder module `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` (re-exported as `ui::*`; most plugins import through `semio_framework_plugin::{…}`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`). Every builder chains and ends in `.try_build() -> Result<BuiltNode, BuiltNode>`; `HasBase` gives id/disabled/style/tone/`try_on`/`try_on_with`; containers add `HasChildren`.

- `stack(axis)`, `column()`, `row()` — Container Plain (861-892)
- `section(label)`, `field(label)` — Container Section|Field; `.description()`, `.required()`, `.error()`, `.default_open()` (950-988)
- `text(Label)` `.emphasize()` (1000-1026); `button(Label)` `.icon(UiText)` (1039-1067)
- `input(InputKind::{Text,LongText,Number,Date,Color,File})` `.value()`, `.placeholder()`, `.commit("blur")`, `.min()/.max()/.step()` (1085-1156)
- `toggle(bool)` `.icon()`, `.text()` (1169-1204); `select(UiText)` `.try_item(value, label)`, `.placeholder()` (1217-1254)
- `slider(f64)` defaults min 0 max 1 step 0.1; `.min()/.max()/.step()/.unit()` (1270-1317)
- `progress(completed, Label)` `.total()` (1331-1357)
- `tree()`, `tree_section(label)`, `tree_item(label)` `.description()`, `.icon()`, `.default_open()`, `.draggable()`, `.dimmed()`, `.try_row_action(RowAction)` (1369-1541)
- `image(src)` needs `.alt()`/`.decorative()`; `surface(SurfaceProps)`; `extension(name)`

Binding: `try_on(trigger, action_id)` / `try_on_with(trigger, action_id, UiValue)` push `ActionBinding{trigger, action, args, capability}`. Host: `emitIntent` → `UiIntent{trigger, action, args, input}` → `uiIntentToActionDescriptor` (`🛠️ShellHelpers/🟦️.tsx:2046`) merges the trigger's scalar `input` into `args` under `"value"` (`"delta"` for `Trigger::Delta`) → plugin `handle`.

Constraints (`🧬️contract/🎬️action/🦀️.rs`): `UI_TEXT_MAX_BYTES = 512`; `UI_FIXED_LIST_ITEMS = 32`; `UI_BUILT_CHILDREN_MAX = 32` (`🏗️builder/🦀️.rs:51`), `UI_VALUE_PAGE_ROWS = 31`; `UI_VALUE_ROW_ACTIONS = 2`; `UI_DOCUMENT_NODES = 128` (`📃️document/🦀️.rs:95`) per rendered panel; `UI_FIXED_BYTES = 32 KiB`; one live `UiValue` page per process, four resident surfaces; `UiMapBuilder::push` keys strictly ascending.

Panel kit (`🔌️plugin/🦀️.rs` region `🔖️PanelKit`, ~5688-5980): `tree_item`, `tree_item_desc`, `tree_group`, `tree_item_with_action`, `tree_item_with_action_draggable`; `PanelTreeBuilder::new(ns)` `.section(id,label,open,items)` `.section_or_placeholder(...)` `.selected(ids)`/`.highlighted(ids)` (recorded, not rendered — framework gap) `.interaction_domain(id)` `.drop_action(id)` `.build()`; `ActionFactory::new(controller).action(name, args)`; paging `panel_page_rows()`, `PanelRowBudget::new(rows)`/`.spend()`/`.nested(reserved, |b| …)`, `paged_panel_section(section_id, entries, rows, &mut budget, |entry, b| …)`, `panel_continuation_row`. Also `FormPanelBuilder` (`🔖️FormKit`).

`ActionArgDef` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:312-390`): `.text/.number/.slider(id,label,min,max)/.toggle/.select/.vec3`; host renders them in a modal action dialog (`renderUiControl`, `🗣️Interpreter/🟦️.tsx:1030-1068`) — not a persistent inspector.

## 2. Outliner precedent

Puzzle 2D `📌️panels/🗿️artifact/🦀️.rs` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs`): one `tree_item_with_action` per node/edge; action `INTERACTION_SELECT_ACTION_ID` with `{domainId, merge:"replace", method:"pick", targets:[{granularity,id}]}` via `ActionFactory`; `PanelTreeBuilder` `.section_or_placeholder(...)` `.interaction_domain(DOMAIN)`. Selection is framework-owned (`InteractionDefinition` + presence painting). Same triple `📌️panels/{🛍️catalogue,🔍️inspection,🗿️artifact}` in cad, gis, writer, procedural, sequence, norm, raster, forms, flow, architect, trinity, block, layout, remodel.

## 3. Inspector precedent

CAD `📌️panels/🔍️inspection/🦀️.rs`: `build_properties_panel` tries selected object → reference → node → summary. Read-only rows `tree_item_desc`; editable rows `tree_item_with_action` bound to `patchCadPlayReference{field, value?, delta?, …ids}`; command `✏️editor/🎮️commands/🖼️reference/🦀️.rs:31-60` matches `field`, `resolve_number_edit(current, value, delta)`, emits the typed mutation via `Emit::mutations`.

Continuous fields: procedural `✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs:196-275` `generation_form` — `field(label).child(control)` with real `slider`/`input(Number)`/`toggle`/`select`, `generation_control_action` = `builder.try_on_with(Trigger::Change, action, args)`.

fem2d already has `replace-element/material/section/support/region` and `update-analysis-settings`; no command dispatches them from a UI row.

## 4. Animation precedent

No `Component::Timeline`. (a) fem3d results: `DisplayMode::Modal(i)/Buckling(i)` recomputed per render; `SetResultDisplay` writes `Fem3dResultsWindowConfig.result_mode_index` through `Emit{ window_config_mutations }` — the right tier for "current frame / playing". (b) Host-side rAF: `🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx` `useFrame` — no plugin round trip; plus `useContinuousTriggerLane` (`🗣️Interpreter/🟦️.tsx:962-985`) coalesces slider drags (`{value, gesture, commit:false}` then `commit:true`) — Slider and continuous number Input ride it. (c) `⏯️tool-run` module + ToolRun panel for job progress (not playback).

Coordinator addition: `Effect::DispatchAction{ req, action, args, delay_ms }` (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:503-515`) is honoured by the React ShellHost (`🏛️ShellHost/🟦️.tsx:5555-5578` → `scheduleDispatchAction`, `🛠️ShellHelpers/🟦️.tsx:840`) with the resolved target view state, and can chain — a plugin-driven playback clock. `delayMs: 0` is an unthrottled macrotask; hidden renderers clamp to ~1 tick/s.

## 5. React host pipeline

`🗣️Interpreter/🟦️.tsx`: `UiDocumentStore` (`📃️UiDocumentStore/🟦️.tsx`) reconciles `BuiltNode` snapshots; views `InputView` (1181, number without `commit` is continuous), `SelectView` (1240), `ToggleView` (1258), `SliderView` (1277), `NumberStepperView`, `TreeView` (1454), `SurfaceView`; `dispatchTrigger` (945); `resolveComponentSceneHost` (344) maps `canvas2d` → `📐️Canvas2dHost`; `uiIntentPayload` (2036) merges input into args; `uiNodeToTreePanelConfig` (2056) hosts plugin panels in shell chrome. Canvas2dHost tracks no layer picking; it emits `canvasPointerDown/Move/Up`, `canvasDoubleClick` with `{x, y, button, shift, ctrl, meta, alt, width, height}` (`📐️Canvas2dHost/🟦️.tsx:550-611`).
