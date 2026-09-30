# History UI and UI stack map (read-only audit)

Ticket: `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Auditor: explore-history-ui. Date: 2026-09-30.
No source file was edited. Line numbers were read on 2026-09-30; `🔌️plugin/🦀️.rs` (45k lines) and the
other big files are edited concurrently and already shifted by 15 lines during this audit, so prefer the
`//#region` names and symbol names over the numbers.

Path aliases used below (expand literally):

- `FW` = `🧰️framework/🔨️modules`
- `UI` = `🧰️framework/🔨️modules/🖱️ui`
- `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`
- `PLUGIN` = `OS/🔌️plugin/🦀️.rs` (the plugin SDK, one 45k-line file)
- `RE` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements` (React host elements)
- `VCSAPP` = `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor` (the vcs demo editor)

## 0. Ten findings that drive the design

1. The history panel exists TWICE. Rust `ui_history_panel` (`PLUGIN` 12133-12231, region `🔖️HistoryPanel`) produces the
   `framework.body.history` body that the wgpu shell renders. React never renders that body: `RE/🏛️ShellHost/🟦️.tsx`
   11006-11110 (`frameworkUtilitiesHistoryTab`) hand-builds its own tree from the `HistoryPatch` projection. They already
   diverge (React has no filter, no create-alternative; different labels; different revert affordance).
2. Row identity is `seq` (a session-local, runtime-only command-log counter). The wire row `HistoryEntry`
   (`FW/🎠️kernel/🦀️.rs` 1795) carries NO `edit_id`, NO severity, NO messages. `CommandView` (`PLUGIN` 12006) has `edit_id`
   but the projection drops it. Editable rows need the persisted `edit_id` on the wire.
3. Per-row severity does not exist end to end. The 4-level `Severity` (`FW/⚠️diagnostic/🦀️.rs` 125), `MutationMessage`
   (`FW/📡️replication/🎮️mutation/🦀️.rs` 953), `DispatchReport` and `MergeReport.replayed: Vec<EditMessages>`
   (`FW/📡️replication/⚔️conflict/🦀️.rs` 275/322) exist, but `Edit` (mutation `🦀️.rs` 1488) persists no messages and the
   Rust `store::HistoryColumn` (`OS/🏪️store/🦀️.rs` 13656) has no `mutation_level` although React `HistoryTable` and the wgpu
   graph-timeline scene both consume `mutationLevel`. `MergeReport.replayed` is the ready-made payload for "downstream
   replay, per-row outcome".
4. The UI vocabulary is a closed, schema-first `Component` enum (`UI/🧬️contract/🧩️component/🦀️.rs` 838). Slider has
   `min/max/step/unit`, NumberStepper `min/max/step/uniform`, Input(number) `min/max/step`. There is NO snap-point field, NO
   vector/vec3 component, NO two-thumb range in the contract. `snapValues` exists only inside the React `Slider` element
   (`UI/🧱️elements/🎚️Slider/🟦️.tsx` 73, 218-254), fed by nothing, absent from the wgpu slider (`🎚️Slider/🎯️targets/🧊️wgpu/🦀️.rs`).
5. The only schema-to-control derivation is `ActionArgDef::control()` (`FW/🛂️manifest/🦀️.rs` 492): `ArgSchema` (stored truth) ->
   `ActionArgControl` {Text, Number, Slider, Toggle, Select, Vec3, IconSelect, ArtifactKind, SurfaceApp}. It is rendered only by
   React `renderStagedArgControl` (`RE/🛠️ShellHelpers/🟦️.tsx` 4402) inside staged forms (`UIDialog`, action panes, command
   palette). Panels built as `Component` trees author controls by hand per field (see raster `field_row`).
6. Modal: `DialogDefinition` (`FW/🛂️manifest/🦀️.rs` 3365; title, body, staged `args`, one submit, one cancel) opened by
   `Effect::OpenDialog`. React renders it accessibly (`UIDialog`); wgpu `ChromeDialogRequest` (`RE/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
   24016) supports only title/body/confirm/cancel, "staged-form args out of scope". No three-way choice exists anywhere.
7. Global indicators: only ONE transient notice (4 s, `role=status`) plus three persistent `role=status` bands hand-coded in
   ShellHost (route admission, plugin install with progress+cancel, command stall with cancel). An author-declared
   `ModeDefinition` is the wrong tool for "time travel" (switching a mode re-seeds the layout and clears the tool).
8. i18n has five parallel mechanisms and several English-default leaks (`is_de` booleans, `frozenLabelText`,
   `createAlternative` default name "Alternative", `Effect::Notify{message:String}`, raw `level` badge text). The good
   pattern to copy is exhaustive `match locale` / `LocalizedLabel::native(en, de)`.
9. Alternatives/versions UI today: the framework panel only has a "Create Alternative" button (Rust body only, name always
   "Alternative"); a swimlane `HistoryTable` exists but is mounted only in the vcs demo app (`GraphTimeline` window). There is
   no store command that rewrites or replaces history (`ArtifactCommand` `OS/🏪️store/🦀️.rs` 2906 has only Apply/Undo/Redo/
   Checkpoint/CreateAlternative/SwitchAlternative/CheckoutCheckpoint/...), so "overwrite existing artifact" has no UI or
   engine target yet.
10. Hard capacities constrain the panel: `UiText` 512 B, `UiFixedList` 32 items, 128 node records per surface (103 usable
    for windowed containers, 24 fixed headroom of which the history panel already spends 7), ONE process-wide `UiValue` arena
    page (~31 interactive rows, 5 collections each, max 2 row actions per row), and a 64-edit applied ledger per store with no
    compaction. Time-travel state MUST NOT be written to a history-ledgered store (config lane) or it kills the app in minutes.

## 1. The history panel

### 1.1 File map

| Concern | Path | Lines / symbol |
|---|---|---|
| Rust panel builder | `PLUGIN` | `ui_history_panel` 12133-12231; region `🔖️HistoryPanel` 12066-12232 |
| Rust panel served | `PLUGIN` | `VcsArtifactApp::render` 34126 (`if body_key == FRAMEWORK_HISTORY_BODY_KEY`, served BEFORE any app body key) |
| Row source data | `PLUGIN` | `CommandLogEntry` 11964, `CommandView` 12006, `HistoryView` 12046, `HISTORY_ROW_OPERATION_PREVIEW`=8 (12042) |
| Row builder | `PLUGIN` | `build_history_view` 26827, `backfill_command_log` 26683, `record_command` 26651, `refresh_cache` (below `history_patch` 26893) |
| Wire projection (Rust) | `FW/🎠️kernel/🦀️.rs` | `HistoryEntry` 1795, `HistoryPatch` 1833, `InvocationResult.history_patch` 1874 |
| Wire projection (TS) | `FW/🎠️kernel/🟦️.ts` | `HistoryEntry` 1936, `historyEntryLabelText` 1973, `HistoryPatch` 1980 |
| Projection producer | `PLUGIN` | `history_patch(snapshot)` 26893 (upserts only dirty seqs unless snapshot) |
| React panel | `RE/🏛️ShellHost/🟦️.tsx` | `frameworkUtilitiesHistoryTab` 11006-11110; store fold `applyHistoryPatch` 2311, `refreshHistorySnapshot` 2406, `shellUncommittedEditCountV1` 1008 |
| React helpers | `RE/🛠️ShellHelpers/🟦️.tsx` | `HISTORY_PANEL_LABELS` 3170, `historyPanelText` 3178, `hostEffectRefreshScopeV1`/history scope 5616 |
| Live tree source | `UI/🧱️elements/🖼️Panel/🟦️.tsx` | `liveTreePanelDefinition` 62, `TreePanelConfig` 34; `UI/🧱️elements/🌳️Tree/🟦️.tsx` `TreeDataItem` 1094 |
| Reserved actions (schema) | `FW/🛂️manifest/🦀️.rs` | `history_action_definitions` 1205, `REVERT_TO_COMMAND_ACTION_ID` 1202, `set_history_command_filter_action_definition` 1240 |
| Reserved actions (runtime) | `PLUGIN` | `framework_reserved_job!` ordinals 0-19 at 17871-17894 (next free = 20), `framework_reserved_route_job` 17917, `HISTORY_ACTION_IDS` 23619, `is_framework_reserved_action_id` 23627, `VIEWER_REJECTED_ACTION_IDS` 23671, `command_from_action` (createAlternative/switchAlternative/checkoutCheckpoint) ~27013, `DOCUMENT_COMMAND_ACTION_IDS` 41492 |
| Revert handling | `PLUGIN` | `begin_framework_revert_route` 29235, `framework_revert_unit` 29260, filter handling `commit_framework_shared_host_route` 29345 |
| Tests | `OS/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | `ui_history_panel_*` laws 5896-6140 |
| Checkpoint graph rows | `OS/🏪️store/🦀️.rs` | `HistoryColumn` 13656, `build_history_columns` 13728, `assign_history_checkpoint_lanes` 13695 |
| Checkpoint graph UI | `UI/🧱️elements/🕰️HistoryTable/🟦️.tsx` | `HistoryColumn` 29, `HistoryTable` 177-251 |
| Graph host (React) | `RE/🌳️GraphTimelineHost/🟦️.tsx` | 19-94 (dispatches `checkoutCheckpoint`) |
| Graph host (wgpu) | `RE/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | `column.mutation_level` badge 6361, `checkoutCheckpoint` hit 6373 |
| Graph scene | `UI/🎬️scene/🎬️scenes/🦀️.rs` | `GraphTimelineScene { columns_json: String }` 2787-2806 (opaque JSON string) |
| Only mount of the graph | `VCSAPP/🎭️modes/✏️edit/🪟️windows/📜️history/🦀️.rs` | `SurfaceKind::GraphTimeline` window `vcs-history` |

### 1.2 Row data, layer by layer

| Field | `CommandLogEntry` (runtime only) | `CommandView` | `HistoryEntry` (wire to hosts) | Rust row uses | React row uses |
|---|---|---|---|---|---|
| `seq` (session-local id) | yes | yes | yes | tree item id `framework.history.entry.{seq}`, revert arg `entrySeq` | same id, revert arg |
| `action_id` | yes | yes | yes | no | `navbarExampleIdFromHistoryUpserts` only |
| `label: LocalizedLabel` (native/reuse x en/de) | yes | yes | yes | `resolve(Native, locale)`, clipped to 512 B, `x{count}` suffix | `historyRowLabelText` (never English fallback; defensive vs missing axis) |
| `kind` (mutation/view/history/clipboard/shell/interaction) | yes | yes | yes (string) | icon (`history_panel_icon_id` 12095) | check-in count only |
| `timestamp` | yes | yes | yes | not shown | not shown |
| `edit_id` (persisted VCS edit) | yes | yes | NO | used by revert + filter | NO |
| `config_edit_ids` / `config_edit_id` | yes | latest only | NO | revert lane | NO |
| `child_edit_ids` | yes | yes | NO | group revert | NO |
| `op_lines` (<=8 printed ops), `op_count` | no | yes | yes | tree-item description (joined " . ") | description with leading "..." when truncated |
| `applied` | no (derived) | yes | yes | `dimmed(true)` if edit-linked and not applied | `dimmed` |
| `revertible` | no (derived) | yes | yes | gates the revert row action | gates the revert button |
| `count` (folded repeats) | yes | yes | yes | label suffix | label suffix |
| `inverse` (memory-only replay) | yes | yes | NO | revert of shell/view rows | NO |
| severity / messages | NO | NO | NO | NO | NO |

Notes: `seq` is never persisted; `backfill_command_log` (26683) re-creates one row per `envelope.vcs.edits` entry after
load (`action_id: "apply"`, label from `edit.description` or first op's `print_op`). `MutationMeta.label` and
`semantic_kind` (`FW/📡️replication/🎮️mutation/🦀️.rs` 1362-1399) already carry a human label and the `"<schema>#<kind>"` id of
the authoring mutation, which is the hook from a row to its mutation schema.

### 1.3 Rust body versus React body (they must converge)

| Aspect | Rust `ui_history_panel` | React `frameworkUtilitiesHistoryTab` |
|---|---|---|
| Sections | Actions (undo, redo, commitCheckpoint, Check In, createAlternative, filter select) + windowed Commands | Actions (undo, redo, checkpoint, check-in inline form, status) + all Commands (no window) |
| Filter (`setHistoryCommandFilter`) | yes (`select` + `Trigger::Change`) | none |
| Create alternative | yes (button, no name arg) | none |
| Revert affordance | menu row action "Backwards"/"Zurueck bis hier", icon RotateCcw, `RowActionPlacement::Menu` | inline `↶` button, title "Revert to Command" |
| Row target/args | `row_target(controller_id, {entrySeq}, None)` | `onAction({controllerId, action:"revertToCommand", args:{entrySeq}})` |
| Read-only (viewer) | all mutating items disabled, revert omitted | undo/redo disabled, checkpoint/check-in/revert omitted |
| i18n | `is_de: bool` plus hard-coded pairs (English default) | `FrozenLabel {en, de}` + `frozenLabelText` (English for any non-"de") |
| Check-in | button dispatches `framework.checkin` `submit` (no message, shell falls back to "check-in") | inline input + Commit/Cancel using shell state `checkinDialog` |
| Windowing | `tree_window_section` key `framework.history.commands` under body key `framework.body.history` | none (React sorts all entries) |
| Accessibility | tree rows (label + description), `disabled` states | plain buttons, `aria-label` on revert only |

### 1.4 How a row dispatches

1. Rust body: tree item carries `RowTarget { scope: controller_id, version: 1, args: {entrySeq}, activation: None }` and a
   `RowAction { icon, label, verb: "revertToCommand", placement: Menu }` (`FW`-contract `UI/🧬️contract/🧩️component/🦀️.rs` 152-231).
   `RowTarget::action_binding` builds the `ActionBinding { trigger: Activate, action: ActionId(scope, verb, version), args }`.
2. Host `onAction({controllerId, action, args})` (React funnel in ShellHost; wgpu `dispatch_action`).
3. Plugin `is_framework_reserved_action_id` (23627) -> `framework_reserved_route_job` (17917) -> `FrameworkRevertToCommandJob`
   (ordinal 6, 17877) -> `begin_framework_revert_route` (29235: resolves `entrySeq` -> `(lane, edit_id)` through `self.cache`, only
   if `entry.revertible`) -> repeated `framework_revert_unit` (29260) = one `ArtifactCommand::Undo` per unit until the target is
   the tail. This is "revert to just after entry N", i.e. destructive-looking walk-back, not an edit.
4. Result carries `history_patch` (kernel 1874) and `UiDirtyScope::Partial { panel_bodies:[FRAMEWORK_HISTORY_BODY_KEY] }`
   where needed; the React host folds it in `applyHistoryPatch` (ShellHost 2311, stale-cursor guard `historyPatchShouldApplyV1`).
5. Viewer gate: `VIEWER_REJECTED_ACTION_IDS` (23671) in the plugin plus `read_only` in the builder.

### 1.5 Drifts and defects noticed on the way (not fixed, listed for the coordinator)

- Filter option values: manifest declares `withoutOperations`/`onlyOperations` (`FW/🛂️manifest/🦀️.rs` 1243-1244); panel and
  handler use `withoutMutations`/`onlyMutations` (`PLUGIN` 12150, 29353) and `HistoryPatch.commandFilter` too. The handler
  silently maps unknown values to `All`.
- `createAlternative` name defaults to the literal `"Alternative"` (`PLUGIN` ~27013) and the panel button sends no name.
- `HistoryTable` shows raw English `level` text in the badge and the literal `checkpoint` placeholder (`🕰️HistoryTable/🟦️.tsx`
  147, 163); `Effect::Notify` carries a plain `String` (`FW/🎠️kernel/🦀️.rs` 368).
- `mutationLevel` has consumers (React `HistoryColumn` 40, wgpu 6361, schema `RE/🌳️GraphTimelineHost/🧬️schema/🎨️layout/🔣️.json`)
  but no Rust producer (`store::HistoryColumn` 13656 lacks the field).
- The vcs demo document panel labels alternatives by id because `HistoryColumn` carries no alternative name
  (`VCSAPP/📌️panels/🗿️artifact/🦀️.rs` doc comment) and `Alternative { id, name, checkpoint_ids }` (`OS/🌿️vcs/🦀️.rs` 190) is not
  in `HistoryView`.

## 2. The UI description layer

### 2.1 Pipeline (schema first)

```
Rust authoring (builders)          UI/🧬️contract/🏗️builder/🦀️.rs   column() row() section() field() text() button() input(kind)
   |                                                              select() toggle() slider() progress() tree() tree_section()
   v                                                              tree_item() table() table_row() image() surface() extension()
BuiltNode -> ComponentTree (built_to_component_tree, PLUGIN)
   v
Flat id-keyed snapshot/patch      UI/🧬️contract/📃️document/🦀️.rs  UiNodeRecord { id, key, component, layout, style, activity,
   (UiSnapshot / UiPatch)                                          disabled, transition, accessibility, bindings, menu, children }
   v
Renderers: React DOM              RE/🗣️Interpreter/🟦️.tsx  (interpretUiNode 2799, renderComponent 2740)
           wgpu (GPU)             UI/🎯️targets/🧊️wgpu/{🌳️tree,🔀️reconcile,🧩️component,🪀️widgets,♿️accessibility}
           terminal               UI/🎯️targets/⌨️tui
```

- Source of record: Rust `Component` enum and prop structs, `UI/🧬️contract/🧩️component/🦀️.rs`. TS is generated
  (`@generated by bun nx run @semio-tech/ui-contract-rs:generate`) into `FW/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`
  (`Component` 100, `SliderProps` 414, `NumberStepperProps` 262, `Trigger` 648, `UiNodeRecord` 747, `UiValue` 798, `Tone` 556,
  `Activity` 43). Schema exports registered in `UI/🧬️contract/🧬️schema/🦀️.rs`; JSON schema `🧬️schema/🔣️.json`.
- Language-neutral conformance corpus: `UI/🧬️contract/🧫️fixtures/🧪️conformance/` (`📇️catalog.json` groups: accessibility, layout,
  composite {dialog, form-with-validation, surface-embedded, toolbar, tree-nested-sections}, rejection, component, patch;
  roles snapshot/expect/patch). Every renderer runs the same cases. New composites should be added here.
- Wire value types: `UiValue` = Null | Bool | Number(f64) | Text(UiText) | List | Map (`UI/🧬️contract/🎬️action/🦀️.rs` 1815).
  `UiMapBuilder` requires STRICTLY ASCENDING keys (memory: silent refusal otherwise).
- `Trigger` = activate, change, commit, delta, drop, submit, abort, repeatLast, hoverPreview. `Liveness` = off/polite/assertive
  (`🧬️contract/♿️accessibility/🦀️.rs`). `Tone` = neutral, primary, secondary, tertiary, info, success, warning, danger.
  `Activity` = waiting, loading, idle, finished.

### 2.2 Input widgets that exist today (all three catalogs)

Catalog A, semantic `Component` (what a panel body can contain):

| Component | Input props | Notes |
|---|---|---|
| `Slider` | `value, min, max, step, unit?` | no snaps, no ready extent (the React element and the window-measure catalog have `ready`) |
| `NumberStepper` | `value, step, uniform, min?, max?` | `uniform=false` = mixed value; no builder exists in `🏗️builder` (only the enum + both renderers) |
| `Input` | `kind` Text/LongText/Number/Date/Color/File, `value, placeholder?, commit?("blur"), min?, max?, step?, accept?` | number input with no `commit` is a continuous control (coalescing lane in React `InputView` 1307) |
| `Toggle` | `on, icon, text?, appearance` Button/Checkbox | |
| `Select` | `value, items: UiFixedList<SelectItem>(<=32), placeholder?` | dispatches picked value under key `value` |
| `IconSelect`, `Ring` | classifier / orb `t` | special |
| `Progress` | `completed, total?, value_text` | read-only; determinate iff `total` present |
| `Container` role Plain/Section/Group/Field/Form/Toolbar | `label?, description?, required?, error?, default_open?, drop_overlay?` | `error` text = validation slot |
| `Tree`, `TreeSection`, `TreeItem` | item: `label, description?, icon?, dimmed?, window?, row_actions (UiFixedList, arena-priced for 2), target, inline_toolbar, detail(Surface), draggable` | rows may contain control children (raster `field_row`) |
| `Table`, `TableRow` | columns, cells, row actions, windows | editable cells via `editable_table_window_row` |
| `KeyValueList`, `Text`, `Button`, `Separator`, `Image`, `Surface`, `Extension` | | |
| NOT present | vector/vec3 field, range (two thumbs), snap points, banner/alert, dialog as Component | |

Catalog B, `ActionArgControl` (staged forms; derived from `ArgSchema`, `FW/🛂️manifest/🦀️.rs`): `Text{placeholder}`,
`Number{min,max,step}`, `Slider{min,max,step,unit}`, `Toggle`, `Select{options}`, `Vec3`, `IconSelect{classifier_kind}`,
`ArtifactKind{roles}`, `SurfaceApp{roles,dialect_arg}`. `ArgSchema` (158): `String{options,min_len,max_len,pattern,format}`,
`Number{min,max,step,integer,unit}`, `Boolean`, `Vec3{unit}`, `Array{items,min_items,max_items}`, `Object{fields}`, `Any`;
`ArgPresentation` = Slider, IconSelect, Multiline, Hidden. `control()` rule (492-516): options => Select; Slider presentation
or fully bounded Number => Slider; else Number stepper/field. Also `json_schema()` (521) for the MCP/LLM lane. React rendering:
`renderStagedArgControl` (`RE/🛠️ShellHelpers/🟦️.tsx` 4402; Vec3 = three number inputs). wgpu has no staged-form renderer.

Catalog C, window chrome (`UI/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` 1085-1415): `WindowMeasure` {Slider(min,max,step,ready,loading,waiting,
disabled), Number(unbounded), Toggle, Group}, `WindowEngagementControl` {Slider, Stepper, Ring, ToggleGroup, Select},
`WindowEngagement.status: Vec<{id,text}>` (per-window status chips), `session_active`, `possible_engagements`.

React elements behind them (`UI/🧱️elements/`): `🎚️Slider` (props include `snapValues`, `minStepsBetweenThumbs`, `ready`,
`clampToReady`, `loading`, `waiting`, `orientation`, `dir`, `formatDisplayValue`), `🪜️Stepper` (`onDelta` relative path),
`✏️Input`, `☑️Checkbox`, `🔽️Select`, `🌈️Surface`, `🕰️HistoryTable`, `📨️UIDialog`, `💬️Dialog`, `🗂️WindowChrome`, `📑️Tabs`.
wgpu counterparts: `🎚️Slider/🎯️targets/🧊️wgpu/🦀️.rs` (65 lines, `quantize_step` only), `🪜️Stepper/🎯️targets/🧊️wgpu/🦀️.rs`,
plus `UI/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`.

### 2.3 Authoring modifiers useful for history rows (all on `HasBase`, `🏗️builder/🦀️.rs` ~634-760)

`try_id`, `disabled`, `activity(Activity)`, `tone(Tone)`, `variant`, `size`, `emphasis`, `density`, `try_label` (accessible name),
`try_describe`, `live(Liveness)` (only way a background change is announced), `try_shortcut` (`aria-keyshortcuts`),
`try_on(Trigger, ActionId)` / `try_on_with(..., UiValue)`, `menu(MenuRef)`. Tree item: `.description`, `.dimmed`, `.row_action`,
`.target`, `.detail`, `.inline_toolbar` (buttons only, <=4, `TREE_INLINE_TOOLBAR_BUTTONS`).

### 2.4 Precedents for "row with an input control"

- Raster inspector `field_row`: `ui::tree_item(label).try_child(control)` where control is `toggle`/`select`/`input(kind).min().max().step()`
  bound with `try_on_with(Trigger::Change|Commit, action, args_map)`; bounds are hard-coded per field with `match` (not schema-driven).
  `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs` 64-99.
- `editable_table_window_row` (`PLUGIN` 35186) and `TextWindowKit::render_draft`: commit-on-blur inputs, plus a draft surface with
  explicit Apply/Discard/conflict/applying/cancel/failed states, its strings chosen by an exhaustive `match locale { De, En }`.
- `operation_progress_controls` (`OS/🔌️plugin/⏳️operation-progress/🦀️.rs` 32-47): per-operation column with `live(Polite)` text,
  `ui::progress`, Cancel button dispatching `cancelTypedOperation {operationId, generation}` (hex text args), exhaustive locale match.
- Conflicts panel (`RE/📌️ChromePanels/🟦️.tsx` 1076-1131): row per conflict with Accept/Discard buttons and a `DiffViewHost` preview,
  labels `ui.conflict.accept|discard`; the closest existing "accept or discard a proposed change" UI.

### 2.5 Hard limits the new panel must respect

| Limit | Value | Where |
|---|---|---|
| `UiText` | 512 bytes; use `UiText::clipped` for any user/op data | `UI/🧬️contract/🎬️action/🦀️.rs` 18 |
| `UiFixedList` | 32 items (select items, table columns, row actions, labels) | same, 21 |
| Node records per surface | 128 (`UI_DOCUMENT_NODES`); bindings per node 32 | `📃️document/🦀️.rs` 95-96 |
| Windowed body budget | 103 records (`TREE_WINDOW_BODY_NODE_BUDGET`); fixed rows headroom 24 (history panel uses 7; a control row costs 2) | `🧩️component/🦀️.rs` 592-605 |
| `UiValue` arena | ONE live page, process-wide; ~31 interactive rows x 5 collections; row action args <=4 entries; <=2 row actions per row | `🎬️action/🦀️.rs` 23-68 |
| Applied-edit ledger | 64 per store, no compaction; dead app when exceeded | `OS/🌿️vcs/🦀️.rs` 196 (memory note) |
| View context | identifiers <=256 chars, printable, `panelJson` <=65 536 chars | `FW/🛂️manifest/🦀️.rs` 5134-5200 |

One failing admission (for example an unclipped label) rejects the whole `refreshUi` and freezes every later refresh
(memory: one-ui-admission-fault-kills-every-later-refresh).

## 3. Modals, prompts, notices, banners, modes

| Mechanism | Definition | React | wgpu | Notes |
|---|---|---|---|---|
| Declared modal `DialogDefinition {id,title,body,args,submit_action,submit_label,cancel_action,cancel_label}` | `FW/🛂️manifest/🦀️.rs` 3365-3432; app registers via `AppBuilder::dialog` (`PLUGIN` 5155, validated 5979) | `OwnedShellDialog` (`RE/🏛️ShellHost/🗨️dialog-origin/🌐️browser/🟦️.tsx`) -> `UIDialog` (`UI/🧱️elements/📨️UIDialog/🟦️.tsx` 40-120): `role=dialog`, modal isolation, focus return, Escape cancel, configurable chords `ui.dialog.submit|cancel` (`SHELL_KEYBINDINGS` 163-190), a11y fixture `🧫️fixtures/♿️modal/🔣️.json` en+de | `ChromeDialogRequest` title/body/confirm/cancel only (24016), `chrome_dialog_request` 6804, `Effect::OpenDialog` 8844 | opened by `Effect::OpenDialog{dialog_id,args}` (kernel 504); the framework does NOT inject any dialog (it injects History actions and the History panel tab at `PLUGIN` 5622-5630) |
| Shell-owned prompt (check-in) | reserved controller `framework.checkin` (`PLUGIN` 12071), message is shell state | inline input in the Actions section (ShellHost 11044-11066) | `checkin_dialog_draft` (wgpu shell 4207) | precedent for "answer is shell state, intercepted before any app sees it" |
| Transient notice | `ShellState.overlays.transientNotice {id,message,kind:Severity,code?}` (`RE/🐚️Shell/🟦️.tsx` 581-614, reducer 1049) | `showTransientNotice` (ShellHost 9727), 4 s (`SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS` 935), render 13249-13262, tone map `TRANSIENT_NOTICE_TONE_CLASS` 927 | `ShellTransientNotice` (wgpu shell 23669), `transient_notice_tone` 23685 | single slot, replaces itself; `role=status`; mutation-rejected toast uses `ui.mutation.*` keys (ShellHost 9750) |
| Persistent status bands | none in the contract | route-admission (13237), plugin-install with `<progress>` + real cancel (13266-13288), command-stall list with per-item cancel (13289-13305) | partial | hand-coded, English text via helper functions with `uiLocale`, `role=status aria-live=polite` |
| Effect notify | `Effect::Notify { message: String }` | banner state | | plain string, not `LocalizedLabel` |
| Per-window status chip | `WindowEngagement.status[{id,text}]` | | | plain strings |
| Mode | `ModeDefinition {id,label,icon_id,tools,layout_id,commands}` (`FW/🛂️manifest/🦀️.rs` 3439); host-owned `ViewModel.active_mode_id` (4944); taxonomy path `<artifact>/✏️editor/🎭️modes/<mode>/{🪟️windows,🎮️commands,🎚️config,🫧️transient}` (example `VCSAPP/🎭️modes/✏️edit/🦀️.rs`) | navbar `ButtonGroup` (ShellHost `modeSwitcherElement` 11222-11244), `applyModeChange` 9162 (clears tool + re-seeds layout), keys `ui.shell.mode.next|previous` | `session.view_state.active_mode_id` (wgpu shell 7545, 10111) | app-authored, per app; not framework-owned |
| Severity vocabulary | `Severity` Info<Warning<Error<Fatal | `HistoryTable` badge classes (`🕰️HistoryTable/🟦️.tsx` 137-142) | `graph_timeline_mutation_tone` | i18n keys `ui.mutation.level.*`, `ui.mutation.code.*` (frozen 7 codes), `ui.conflict.*` |

## 4. i18n

Mechanisms in play (one per layer; all must be honoured or replaced by one):

1. Axes, single source: `UI/🎚️axes/🔣️.json` (locales en, de; terminologies native, reuse) generates Rust `Locale`/`Terminology`
   (`OS/🌐️locale/🤖️generated/🦀️.rs`, `Locale::ALL`, exhaustive matches break on a new locale) and TS types.
   Generated `Locale` still has `#[default] En`.
2. Manifest labels: `LocalizedLabel` (`OS/🌐️locale/🏷️label/🦀️.rs` 94-134; wire schema `🌐️locale/🧬️schema/🔣️.json` requires every
   terminology x locale cell). `LocalizedLabel::native(en, de)` uses an exhaustive `match locale`; `LocalizedLabel::data(x)` for
   locale-invariant data. Used by actions, panel tabs, dialogs, modes, window kinds, and history rows. Host resolves via
   `resolveManifestLabel` (`RE/🛠️ShellHelpers/🟦️.tsx` 3269) / `historyEntryLabelText` (kernel TS 1973, never English fallback).
3. Plugin body strings: `app_labels!` (`PLUGIN` 7056, compile-time checked `LabelText`, `.fill(&[(name,value)])`), or ad hoc
   `match locale { Locale::En => .., Locale::De => .. }` (best pattern, `⏳️operation-progress/🦀️.rs` 18-21), or the bad
   `is_de: bool` (`ui_history_panel`).
4. React chrome: i18next tree typed by `UiTranslationSchema` (`UI/🧱️elements/📚️I18n/🟦️.tsx` 108+; `ui.mutation` 642, `ui.conflict` 675)
   with leaves `{ label: { normal, beginner }, manual?, tutorial? }`, bundles `uiChromeTranslationBundles` in
   `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts` (104; de block first, mutation at 859 (de) and 1785 (en)); products add keys with
   `registerUiTranslationBundles` (1985). Access: `shellLabel(key)` (`RE/🛠️ShellHelpers/🟦️.tsx` 2806), `useLabel(key)`
   (`UI/🧱️elements/🏷️Label/🟦️.tsx` 36). Tier chosen by the active `UiDriver.labelTier`.
5. Frozen pairs in ShellHelpers: `FrozenLabel {en,de}` + `frozenLabelText` (2873-2877, `locale === "de" ? de : en`; English is the
   fallback for every other locale), used by `HISTORY_PANEL_LABELS` (3170) and all check-in text.

To add en/de strings: (a) manifest-level or plugin-body text: `LocalizedLabel::native(en, de)` or an exhaustive locale match;
(b) React shell chrome: add the key to the `UiTranslationSchema` type AND both language blocks in `uiChromeTranslationBundles`
(the type makes a missing cell a compile error); (c) do not add new `FrozenLabel`s or `is_de` flags. Interpolation:
`.fill` (Rust, named placeholders) or `{{count}}` (i18next, e.g. `ui.presence.overflow`).

## 5. Alternatives, branches, version history in the shell today

- Store commands: `ArtifactCommand::{CommitCheckpoint{message,authors}, CreateAlternative{name}, SwitchAlternative{alternative_id},
  CheckoutCheckpoint{checkpoint_id}, Undo, Redo, AmendLast, UndoInLane, ...}` (`OS/🏪️store/🦀️.rs` 2906-2993, `HistoryLane` 2640).
  Types `Checkpoint {id, change_ids, parent_id, authors, message, timestamp, composition_pins}` and `Alternative {id, name,
  checkpoint_ids}` in `OS/🌿️vcs/🦀️.rs` 170-194. History view exposes `HistoryView.columns`, `active_alternative_id`,
  `current_checkpoint_id` (`PLUGIN` 12046).
- Framework History panel: one "Create Alternative" button (Rust body only). No alternative list, no switch UI, no naming.
- Swimlane graph: `HistoryTable` (lanes, elbow connectors, alternative-name chips, author avatars, `mutationLevel` badge, row
  = `role=button` with Enter/Space and `aria-label`) via `GraphTimelineScene` -> React `GraphTimelineHost` / wgpu `Scenes`. Only the
  vcs demo mounts it (`vcs-history` window, `SurfaceKind::GraphTimeline`); its document panel lists checkpoints and alternatives with
  `checkoutCheckpoint`/`switchAlternative` clicks (`VCSAPP/📌️panels/🗿️artifact/🦀️.rs`).
- Sync/hub: footer sync pill `computeSyncPillState` (`RE/🛠️ShellHelpers/🟦️.tsx` 3198), check-in status band, hub check-in
  (`documentCheckInUi`); unrelated to branches.
- Name collision to avoid: React `TreeDataItem.alternatives?: TreeDataItem[][]` (`UI/🧱️elements/🌳️Tree/🟦️.tsx` 1103) is a UI tree-branch
  concept, not VCS alternatives.
- Missing: any store operation that replaces or rewrites an edit or builds a new alternative from a re-run mutation suffix. The nearest
  primitives: `IngestRemote` with `MergeReport { accepted, insertion_index, replayed: Vec<EditMessages>, worst, conflict }` (suffix replay with
  per-edit messages), `Conflict` kinds Quarantined/Degraded with Accept/Discard (`ResolveConflict`), `MergePolicy` LaissezFaire/Normal/Vigilant.

## 6. How a panel reads state; per-window transient

- App panel bodies: `ArtifactApp::render_with_request_context(owner, body_key, doc: &ArtifactView, cfg: &ConfigView, view_state: &ViewModel,
  transient: &TransientView, interaction: &InteractionView)` (`PLUGIN` 13625). `ArtifactView { snapshot, history: &HistoryView, children, .. }`
  (9553); `ConfigView { snapshot, window }` (10763); `TransientView { snapshot, window }` (10810). `window::<O>()` reads a typed per-window slice.
- Framework-owned bodies (history, tool-run) are served first in `VcsArtifactApp::render` (34126) from `HistoryView` (`refresh_cache`
  keyed by store generation, config generation, `log_generation`, filter), independent of the app snapshot, so the app cannot customise them.
- View context: `ViewModel` (`FW/🛂️manifest/🦀️.rs` 4941): host-owned `active_mode_id`, `active_window_kind_id`, `active_utility_id`,
  `active_utility_by_window_id`, `active_tool_id`, `panel_json` (opaque host panel state), `session_identity`, `locale`, `terminology`,
  `window_id`, `focused_window_id`, `window_instances`, `tree_windows` (open/offset/rows per container), `tree_viewport_rows`.
  `ViewModel::for_panel()` (5099) drops `window_id`/kind/utility but keeps `focused_window_id` if it is a live window; `for_window_instance`
  (5105) binds a call to one window. Every host dispatch must carry `windowInstances` (memory: action-view-state-needs-window-instances).
- Window config (persisted local-only, history-ledgered): `WindowConfigOwner` (`OS/🔌️plugin/🪟️window/🎚️config/🦀️.rs` 17). A panel read
  binds to `view.window_id`, or for a panel to `focused_window_id`, so it sees ONLY the focused pane's config (memory:
  panel-projection-captures-focused-window-config). Writes must carry `windowId`. Every amend costs ledger capacity (64) unless coalesced.
- Window transient (ephemeral local-only, no history): `WindowTransientOwner` (`OS/🔌️plugin/🪟️window/🫧️transient/🦀️.rs` 12), typed state +
  mutation, addressed `WindowTransientMutation::of::<O>(window_id, m)`. This is where per-frame or session-scratch state belongs.
- Lane taxonomy (AGENTS): persisted shared = document store; persisted local = config/window config; ephemeral local = transient / window
  transient / plain `VcsArtifactApp` fields (`history_filter`, `shell_redo`, `command_log`, `log_generation`, `history_dirty_sequences`);
  ephemeral shared = presence store.
- Refresh scope: results declare `UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels }`
  (`FW/🎠️kernel/🦀️.rs` 1650, `UiDirtyScope`); `HistoryPatch` rides `InvocationResult.history_patch` independent of it.
- Rendering a body against a substituted document already exists: `render(body_key, snapshot_override_json, view_state)`
  (`PLUGIN` 34124-34160, `plugin_render_with_document` ~41214, `documentJson` in `FW/🛂️manifest/🟦️.ts` 1210) builds `ArtifactView` from the
  supplied snapshot. That is the natural seam for "show the document as of the edited mutation".

## 7. Recommendations

### 7.1 One history body, produced in Rust, rendered everywhere

Delete `frameworkUtilitiesHistoryTab` (ShellHost 11006-11110) and let React render `framework.body.history` through the interpreter like every
other panel body; keep `HistoryPatch` for the host-side undo/redo/check-in state only. Reason: AGENTS demands schema-first, multi-implementation;
today's two bespoke builders guarantee drift, and every new feature below (controls, severity, banner) would have to be written twice.
If the coordinator refuses that scope, the minimum is a shared JSON schema plus fixture for one `HistoryRow` view and a conformance test
run by both (pattern: `🧬️schema/🔣️.json` + `🧫️fixtures` + Rust test + TS test, as `GraphTimelineHost/🧬️schema/🎨️layout`).

### 7.2 Schema additions (schema first, then regenerate TS)

- `HistoryEntry` (kernel 1795 / TS 1936): add `editId?: string` (persisted id, the new row identity), `configEditId?`, `outcome?: { level: Severity,
  messages: [{ level, code, message, target[], opIndex? }] }`, `editState: idle | editing | pending-replay | replaying | replayed | fatal`, and
  `editable: bool`. Keep `seq` only as sort/display order. Row action args then become `{editId}` (ascending key order, one entry).
- New framework-reserved actions in `history_action_definitions` (`FW/🛂️manifest/🦀️.rs` 1205), each with EN+DE `describe`/`use_when`, `ApprovalMode`
  via `.destructive()` for overwrite, plus route ordinals 20+ (`framework_reserved_job!` 17871-17894), entries in `HISTORY_ACTION_IDS` (23619),
  `VIEWER_REJECTED_ACTION_IDS` (23671), the `skip` list in `assert_declared_actions_bridge_to_commands` (8051), `DOCUMENT_COMMAND_ACTION_IDS` (41492),
  and the plugin bridge law: `beginMutationEdit{editId}`, `setMutationInput{editId, path, value}` (or one map arg), `acceptMutationEdit`,
  `discardMutationEdit`, `editReplayedMutation{editId}` (fatal rows), `finalizeHistoryEdit{choice: newAlternative|overwrite, name?}`,
  `cancelHistoryReplay` (or reuse `cancelTypedOperation`).
- `ArgSchema::Number` / `SliderProps` / `NumberStepperProps` / `InputProps`: add `snaps: Vec<f64>` (ascending, <=32) and keep `min/max/step`;
  add a `Vec3` component or a `Container(Group)` recipe of three number inputs (both renderers can already do the group). Update the React
  slider to take snaps from the record (it already supports `snapValues`), implement snapping + tick marks in
  `🎚️Slider/🎯️targets/🧊️wgpu`, extend `ActionArgControl::Slider/Number`, `ActionArgDef::json_schema`, and fixtures in
  `UI/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component`.
- Clamp semantics: `mutation.clamped` (`ui.mutation.code.clamped`) already exists as the frozen code for "value was clamped to its valid range";
  show min/max as control bounds AND surface a `clamped` message when the replayed value is coerced.
- Time-travel status carrier: add `timeTravel?: { editId, phase, replayDone, replayTotal, worst? }` to `HistoryPatch` (and `HistoryView`), so both
  hosts and MCP get it event-driven from one place.

### 7.3 Row model and states

Map state to existing tokens, not new visuals: pending = `Activity::Waiting`, replaying = `Activity::Loading`, ok = `Tone::Success`,
warning = `Tone::Warning`, error = `Tone::Danger`, fatal = `Tone::Danger` + `Emphasis::Strong` + a text state (never colour alone), edited
= `Tone::Primary`. Put severity text into the row label/description (`ui.mutation.level.*` + first message's `ui.mutation.code.*`) and set
`live(Polite)` only on the summary line, not on every row. Rows keep `TreeItem` with: activation = open editor (or focus), row action 1 = revert
(menu), row action 2 = "edit" (max 2 row actions per row, 4-key args). Fatal rows: `TreeItemProps.error`-style description plus the edit action
enabled, accept/finalize disabled while any row is fatal (`disabled` + `try_describe` reason).

### 7.4 Time-travel banner

Do not model it as an app `ModeDefinition` (author-declared, resets tool/layout, `applyModeChange` 9162). Keep the authority in the plugin as an
ephemeral local-only field (like `history_filter`), publish through `HistoryPatch.timeTravel`, and render (a) a persistent host band styled after
the plugin-install band (ShellHost 13266-13288: `role=status aria-live=polite`, text, `<progress>`, real Cancel) and its wgpu twin next to
`ShellTransientNotice`, and (b) per-window `WindowEngagement.status` chips so every pane says "Time travel: state before <row label>". Text must come
from `LocalizedLabel::native` (plugin) or new `ui.timeTravel.*` keys (React chrome). Add chords to `SHELL_KEYBINDINGS`
(`UI/🔨️modules/🕹️control-keybinding-context/🟦️.tsx` 163): `ui.timeTravel.accept`, `ui.timeTravel.discard` so they are user-remappable; expose
`aria-keyshortcuts` via `try_shortcut`. Escape must not silently discard drafts (confirm or keep the draft, see `TextWindowKit::render_draft` states).
The view of the document as of the edited mutation is `render(..., snapshot_override_json, ...)` (34124-34160) with the time-travel snapshot; refresh
all window bodies with `UiDirtyScope::Partial { window_bodies: all, panel_bodies:[FRAMEWORK_HISTORY_BODY_KEY], engagements: true }`.
Time-travel state MUST NOT go through the config or window-config lane (64-edit ledger); use plain fields or the transient lane.

### 7.5 Input controls from schema metadata

Add one renderer-neutral Rust function `arg_control_node(&ActionArgDef, current: &DslValue, action: ActionId, args) -> BuiltNode` next to
`operation_progress_controls`/`editable_table_window_row`, following `ActionArgDef::control()` (492) so semantics stay single-sourced:
Number bounded => `slider` (+`unit`, +`snaps`) inside a row with a paired `input(Number)` for exact entry (React `SliderView` already shows the
value beside the slider only when `unit` is set, so always give `unit` or add the paired input for a11y); unbounded Number => `input(Number)`
or `NumberStepper` (add the missing builder); Boolean => `toggle` (Checkbox appearance for forms); Select => `select` (<=32 items, else a paged
list); Vec3 => `ContainerRole::Group` of three labelled number inputs; String => `input(Text|LongText)`. Bind `Trigger::Change` for continuous
controls only inside the editor (coalescing lane exists in React `useContinuousTriggerLane`), never into the document; the time-travel draft
buffer holds the value until `acceptMutationEdit`. Cap fixed rows: N inputs cost 2N+1 records, so window them
(`tree_window_indexed_section`, `PLUGIN` 6773) beyond ~8 inputs or the 24-record headroom breaks
(`tree_window_headroom_covers_the_fattest_shipped_panel` in `🔬️app-panel-kit`). The action-arg shapes of a mutation come from its `ActionDefinition.args`
(`ArgSchema`), located from the row via `CommandLogEntry.action_id` / `MutationMeta.semantic_kind`.

### 7.6 Accept, discard, replay progress, cancel

Reuse in this order: `operation_progress_controls` (progress + polite status + Cancel bound to `cancelTypedOperation`, `⏳️operation-progress/🦀️.rs`),
the Conflicts-panel row shape for per-row Accept/Discard (`RE/📌️ChromePanels/🟦️.tsx` 1094-1130), and the `TextWindowKit::render_draft`
state names (Apply, Discard, Applying..., Cancel, conflict, failed-keep-draft) for button labels and states. Replay outcome per row = the
`MergeReport.replayed[EditMessages]` shape already produced by suffix replay; keep it in the ephemeral session, not on `Edit`. Decide whether the
`⏯️tool-run` lifecycle (`FW/⏯️tool-run/🦀️.rs` 105: Starting, Running, Paused, Complete, Finalizing, Finalized, Aborting, Aborted, Faulted, with
provisional ops finalized as ONE edit, `TOOL_RUN_GROUP_ID_PREFIX`) is the state machine for the whole session: its vocabulary (finalize vs abort,
progress counters, step ring, EN/DE labels, `framework.panel.tool-run`) maps one to one and avoids inventing a tenth state enum.

### 7.7 Finalize prompt

Preferred: a framework-injected `DialogDefinition` (add next to the History tab injection, `PLUGIN` 5622-5630) opened with `Effect::OpenDialog`,
containing a `Select` arg `choice` {newAlternative, overwrite} and a `Text` arg `name` (required when newAlternative, default from a
`LocalizedLabel`, never the literal "Alternative"), submit = `finalizeHistoryEdit`, cancel = back to time travel (not discard). React works today
(`UIDialog`, fixtures `♿️modal`). wgpu needs `ChromeDialogRequest` extended from title/body/confirm/cancel to render args (its own comment marks
this out of scope): that is a required parity task, otherwise wgpu users cannot finalize. Prefer adding a `choices` list (each choice = label +
action) to `DialogDefinition` so a real two-button decision is possible without a Select; render as buttons in `UIDialog` (focus order,
`aria-describedby` description of the destructive consequence). "Overwrite" must be marked `.destructive()` (`ApprovalMode::WhenDestructive`) so the
agent lane also asks. Fallback with no modal: a Rust-built `Container(Form)` in the banner panel with the same choice/name controls.

### 7.8 Accessibility, i18n, customization checklist

- Accessible names via `try_label`/`try_describe`; live region only on the summary (`live(Polite)`); focus moves to the first editable control on
  entering time travel and returns to the row on accept/discard; row list keyboard-operable (tree roving focus already tested in `♿️tree-disclosure`);
  colour never the sole carrier (severity text + icon); reduced-motion respected by `Progress` (React `ProgressView` already honours it).
- i18n: every string through `LocalizedLabel::native(en, de)` or exhaustive locale matches (add fixtures for both, like `♿️modal`); no `is_de`,
  no `FrozenLabel`; add `ui.timeTravel.*`/`ui.history.*` to `UiTranslationSchema` and both bundles if React chrome needs them; localise level and code text
  through the existing `ui.mutation.level.*`/`ui.mutation.code.*` keys instead of printing raw `level`.
- Customization: respect `UiDriver` (`labels` full/icons, `labelTier`, `tooltips`, `hotkeys`, `drag`; `UI/🧱️elements/🚗️UiDriver/🟦️.tsx` 15-48), themes,
  remappable keybindings above, density via `StyleSpec.density`; icon-only drivers need `try_label` because the caption disappears.

### 7.9 Tests (language-agnostic first, third-party oracle second)

- Add composite conformance cases (`history-editable-row`, `time-travel-banner`, `finalize-choice`) to `UI/🧬️contract/🧫️fixtures/🧪️conformance/`
  (`📸️snapshot.json`, `🎯️expect.json`) and run them in Rust (`🧪️tests/🔬️conformance-unit`), React (`RE/🗣️Interpreter/🧪️tests`), wgpu.
- JSON schema for the new `HistoryEntry` fields validated by an external validator (Ajv, as the existing schema tests do) with the same fixtures
  the Rust decoder reads; a11y case validated in the browser against `axe-core` (third-party) plus the existing dialog fixture pattern.
- Rust laws next to `ui_history_panel_*` (`🔬️plugin-runtime-plugin-builder-contract` 5896-6140): row states, viewer gating, 512 B clipping, window budget.

### 7.10 Open decisions for the coordinator

1. Accept the single-producer change (7.1) and retire the React hand-built tab, or keep both and add the shared schema plus conformance test.
2. Row identity moves from `seq` to `edit_id` on the wire (breaking, allowed: greenfield).
3. Where snap points live in the schema (`ArgSchema::Number.snaps` proposed) and who owns the per-mutation `ArgSchema` (technology `ActionDefinition.args`).
4. Time-travel state machine: new enum vs reuse `ToolRunState`.
5. wgpu dialog args/choices parity as a blocking prerequisite for the finalize prompt.
6. Severity vocabulary: requirement says success/warning/error; the frozen contract has info/warning/error/fatal. Proposal: success = no message (or info), and
   keep fatal because it already encodes "must be edited" under `MergePolicy` Normal.
7. Fix the drifts in 1.5 as part of this ticket or split them out.

## Appendix: quick reference of reused mechanisms

`HistoryView`/`CommandView` (PLUGIN 12006-12046) - `HistoryEntry`/`HistoryPatch` (kernel 1795-1874) - `Severity`/`MutationMessage`/`MergeReport`
- `ui.mutation.*` i18n keys - `Component`+builders+conformance corpus - `ActionArgDef::control` - `operation_progress_controls` - Conflicts panel
- `DialogDefinition`/`UIDialog`/`OwnedShellDialog` - transient notice + `role=status` bands - `ViewModel` host-owned fields - `WindowTransientOwner`
- `snapshot_override_json` render seam - `⏯️tool-run` lifecycle - `SHELL_KEYBINDINGS` - `UiDriver` - `LocalizedLabel::native`.
