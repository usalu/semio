# Wave 2, item 4: retained UiNode::Table (opus55) — NOTHING COMPILED OR RUN (build freeze)

## Done
- Declaration (`📜️declaration/🧩️ui-graph/📊️table-nodes`): `UiTableNode {id?, label, columns, actions_label?, rows, presence, menu}` and `UiTableRowNode {id, cells, action?, actions, presence}` (serde + ToValue/FromValue), `UiNode::Table`, exported; presence/presence_mut/menu/menu_mut wired.
- Panel projection (shell panel-records): `UiNode::Table` -> `Component::Table(TableProps)` + one `Component::TableRow` child per row (cells positional, row actions on ONE target). `tree_item_row_target` became the shared `row_target(row_id, activation, actions)` used by tree items and table rows. Tag arm `table`. Imports added in the shell wgpu root.
- Exhaustive matches updated by hand: retained paint step (presence step), skeleton kind (Rows), reference painter (no own paint), reconcile testkit (kind 23, explicit id), interpreter introspection (tag/declared id/visual fields), `RetainedNodeFocusKind::Table`. Layout/events/accessibility have wildcards (Leaf / non-interactive). reconcile never produces `UiNode::Table` (it already maps `Component::Table` to a Tree with the `document_table` binding), so the record path is the only live one.
- Task Manager (`doc-open/🧵️task-manager-runtime`): `task_manager_table_node` replaces the TableScene surface: 11 text columns, `actions_label` = `taskManager.columns.actions` (en+de), row id = actor id, Suspend/Resume/Cancel as row actions on `os.task-manager` with `{actorId}`. `build_task_manager_table_scene` stays (React-parity fixture helper, still tested).
- Tests: docs-3 `task_manager_leaf_lists_the_runtime_actors_with_all_three_row_actions` now asserts the retained table; new `task_manager_actor_table_projects_to_retained_table_records_with_one_row_target`; wire-format golden `ui_table_node_wire_format_round_trips` plus Table in the every-variant presence/menu tests. Fixture reused: `🧵️TaskManager/🧪️tests/🧫️fixtures/🧵️actor-table.json`.

## Gaps
- Sorting/`sortable` flags of the columns are not carried (contract TableProps columns are labels only).
- Cells are text only (React Table's number/buttons cells map to text + row actions).
- No TS twin for UiNode::Table (TS side already has Component Table).

## Run list (not run)
`repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p <pkg>` for: semio-ui-declaration (wire-format tests), semio-ui-retained-tree (reconcile testkit), semio-ui-wgpu-paint(+paint-laws), semio-ui-wgpu-mounted-layout, semio-renderer-wgpu-shell-doc-open, semio-renderer-wgpu-shell-windows, the Interpreter introspection crate, and the shell tests crate holding `🔄️detached-sync-task-manager` (docs-3). Compile risks: `kind` Copy in `*kind`, `shell_chrome_string` return type into `Label::data`, `ui_contract` alias in the tests crate.
