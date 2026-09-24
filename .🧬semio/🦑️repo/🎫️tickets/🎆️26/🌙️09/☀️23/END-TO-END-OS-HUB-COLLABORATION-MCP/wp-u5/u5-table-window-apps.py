#!/usr/bin/env python3
"""📊️ U5 6b — one-off: move every `TableWindowKit::render_rows` caller (Space Home editor + viewer, Space index editor +
viewer) and the SDK's own table laws onto the windowed table kit. Run AFTER `u5-table-window-sdk.py`, in the same
breath, so the tree compiles at every moment. Exact single-match edits only; nothing is written unless all match."""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SPACE = ROOT / "✏️s/🔌️plugins/🪐️space"
HOME_ANY = SPACE / "🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any"
INDEX_ANY = SPACE / "🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any"
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"

edits: list[tuple[pathlib.Path, str, str]] = []
blocks: list[tuple[pathlib.Path, str, str, str]] = []


def edit(path: pathlib.Path, old: str, new: str) -> None:
    edits.append((path, old, new))


def block(path: pathlib.Path, start: str, end: str, new: str) -> None:
    blocks.append((path, start, end, new))


core = SPACE / "🫀️core/🦀️.rs"
edit(core, '''        column_actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";''', '''        column_actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";
        table_name: native_en "Studios", native_de "Studios", reuse_en "Studios", reuse_de "Studios";''')

home_editor = HOME_ANY / "✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs"
edit(home_editor, "use semio_framework_plugin::app::{TableRow, TableRowAction, TableRowsView, TableWindowKit, WindowKit};", "use semio_framework_plugin::app::{table_row_action, table_window_row, TableWindowKit, TreeWindows, WindowKit};")
edit(home_editor, "use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};", "use semio_framework_ui_contract::{Buildable, HasBase, HasChildren, HasStackLayout};")
block(home_editor, "fn home_row_action(", "/// 🆕️ ticket §C0 lane 4-F", '''fn home_space_action(action_id: &str, space_id: &str) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    let mut args = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.table.action-args", "fixed table action argument admission failed"))?;
    args.push("spaceId".to_owned(), semio_framework_plugin::UiValue::Text(fixed_text(space_id, "ui.table.space-id")?))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.action-args.space-id", "fixed table action argument admission failed"))?;
    ActionFactory::new(S_HOME_CONTROLLER_ID).action(action_id, Some(semio_framework_plugin::UiValue::Map(args.finish())))
}

fn home_row_action(icon: IconName, label: semio_framework_plugin::LabelText, action_id: &str, space_id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::RowAction> {
    table_row_action(icon.as_str(), label.as_str(), home_space_action(action_id, space_id)?)
}

/// 🛂️ `openSpace` is offered to every row; the directory-owned lifecycle affordances
/// (rename/share/delete) and the administration pane (`manageSpace`) are offered ONLY when the
/// caller's own current membership role is `author`. Hub origin alone is not a capability: a
/// spectator reaching a control the server correctly rejects is exactly the role blindness this
/// replaces. The pane it opens still renders solely from the server's own capability flags.
fn row_actions(labels: &SHomeLabels, row: &crate::HomeSpaceRow) -> semio_framework_plugin::UiAssemblyResult<Vec<semio_framework_plugin::RowAction>> {
    let mut actions = vec![home_row_action(IconName::FolderOpen, labels.action_open, "openSpace", &row.id)?];
    if row.data_class == "ephemeralLocalOnly" {
        actions.push(home_row_action(IconName::Cloud, labels.action_promote, "promoteToHubSpace", &row.id)?);
        actions.push(home_row_action(IconName::Save, labels.action_persist, "persistLocally", &row.id)?);
        return Ok(actions);
    }
    if row.origin == "hub" && row.role == Some(crate::DirectorySpaceRole::Author) {
        actions.push(home_row_action(IconName::Pencil, labels.action_rename, "renameSpace", &row.id)?);
        actions.push(home_row_action(IconName::Link, labels.action_share, "shareSpace", &row.id)?);
        actions.push(home_row_action(IconName::Trash2, labels.action_delete, "deleteSpace", &row.id)?);
        actions.push(home_row_action(IconName::Users, labels.action_manage, "manageSpace", &row.id)?);
    }
    Ok(actions)
}

/// 🧪️ The pure per-row-list core, split out from `render` so the empty-state branch is unit-testable
/// in ISOLATION from `crate::list_all_space_catalog_entries()`'s process-global catalog singleton.
/// Every space is one `TableRow` record (cells and row actions are props) inside the windowed table
/// kit, so any number of spaces stays inside the window's node budget: the host streams the rows its
/// viewport shows (ticket 26/09/18 U5 §6b — 9 author rows used to fault the whole window at
/// `nodes 129 > 128`). A row's own activation (Enter on the focused row) opens the space.
fn render_rows(rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if rows.is_empty() {
        return semio_framework_ui_contract::text(fixed_label(table.empty_message, "ui.table.empty-label")?)
            .try_id(S_HOME_EMPTY)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.empty-id", "empty table id admission failed"))?
            .try_build()
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.empty", "empty table text admission failed"));
    }
    let columns = [table.column_name.as_str(), table.column_kind.as_str(), table.column_visibility.as_str(), table.column_members.as_str(), table.column_updated.as_str(), table.column_origin.as_str()];
    TableWindowKit::render_rows(windows, table.table_name.as_str(), &columns, Some(table.column_actions.as_str()), rows, |row| {
        let origin = if row.origin == "hub" { table.origin_hub.as_str() } else { table.origin_local.as_str() };
        let key = format!("space:{}", row.id);
        table_window_row(&key, &[row.name.as_str(), row.kind.as_str(), row.visibility.as_str(), row.members.as_str(), row.updated.as_str(), origin], row_actions(actions, row)?, Some(home_space_action("openSpace", &row.id)?))
    })
}

''')
edit(home_editor, '''fn render_rows_wrapped(rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let table_node = render_rows(rows, table, actions)?;''', '''fn render_rows_wrapped(rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let table_node = render_rows(rows, table, actions, windows)?;''')
edit(home_editor, '''    semio_framework_ui_contract::column()
        .try_children(children)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.build", "window admission failed"))
}

pub fn render(''', '''    semio_framework_ui_contract::column()
        .grow(true)
        .try_children(children)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.children", "fixed window child admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.window.build", "window admission failed"))
}

pub fn render(''')
edit(home_editor, '''    render_rows_wrapped(&rows, table, actions)
}''', '''    render_rows_wrapped(&rows, table, actions, &TreeWindows::for_body(view_state, S_HOME_BODY))
}''')

home_editor_tests = HOME_ANY / "✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs"
edit(home_editor_tests, '''fn buttons(node: &semio_framework_plugin::BuiltNode) -> Vec<&semio_framework_ui_contract::ActionBinding> {
    node.children.iter().filter(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))).map(|child| child.bindings.get(0).expect("Home button carries an action binding")).collect()
}''', '''fn buttons(node: &semio_framework_plugin::BuiltNode) -> Vec<&semio_framework_ui_contract::ActionBinding> {
    let semio_framework_ui_contract::Component::TableRow(props) = &node.component else { panic!("a Home row is one TableRow record") };
    props.row_actions.iter().map(|action| &action.action).collect()
}

fn rows(windows: &TreeWindows<'_>, rows: &[crate::HomeSpaceRow], table: &HomeTableLabels, actions: &SHomeLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    render_rows(rows, table, actions, windows)
}''')
for label in ("NATIVE_EN", "NATIVE_DE"):
    pass
home_test_text = home_editor_tests.read_text(encoding="utf-8")
home_test_count = home_test_text.count("render_rows(&")
if home_test_count == 0:
    sys.exit("refused: home editor tests have no render_rows calls")

home_viewer = HOME_ANY / "👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs"
edit(home_viewer, "use semio_framework_plugin::app::{TableRow, TableRowsView, TableWindowKit, WindowKit};", "use semio_framework_plugin::app::{table_window_row, TableWindowKit, TreeWindows, WindowKit};")
block(home_viewer, "fn render_rows(rows: &[crate::HomeSpaceRow], labels: &HomeTableLabels)", "/// 👁️ No `SHomeSnapshot` argument", '''fn render_rows(rows: &[crate::HomeSpaceRow], labels: &HomeTableLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if rows.is_empty() {
        return semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(labels.empty_message.as_str().to_string()))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.empty", "empty table text admission failed"));
    }
    let columns = [labels.column_name.as_str(), labels.column_kind.as_str(), labels.column_visibility.as_str(), labels.column_members.as_str(), labels.column_updated.as_str(), labels.column_origin.as_str()];
    TableWindowKit::render_rows(windows, labels.table_name.as_str(), &columns, None, rows, |row| {
        let origin = if row.origin == "hub" { labels.origin_hub.as_str() } else { labels.origin_local.as_str() };
        let key = format!("space:{}", row.id);
        table_window_row(&key, &[row.name.as_str(), row.kind.as_str(), row.visibility.as_str(), row.members.as_str(), row.updated.as_str(), origin], std::iter::empty(), None)
    })
}

''')
edit(home_viewer, "    render_rows(&rows, labels)\n}", "    render_rows(&rows, labels, &TreeWindows::for_body(view_state, S_HOME_VIEW_BODY))\n}")

index_editor = INDEX_ANY / "✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/🦀️.rs"
edit(index_editor, "use semio_framework_plugin::app::{TableRow, TableRowAction, TableRowsView, TableWindowKit, WindowKit};", "use semio_framework_plugin::app::{table_row_action, table_window_row, TableWindowKit, TreeWindows, WindowKit};")
edit(index_editor, "use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};", "use semio_framework_ui_contract::{Buildable, HasBase, HasChildren, HasStackLayout};")
block(index_editor, "fn artifact_row_action(", "/// 🩹️ **Known framework gap, worked around here**", '''fn open_artifact_action(row: &SpaceArtifactRow) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    let args = crate::editor::space_index::ui_value_map([("id", crate::editor::space_index::ui_value_text(&row.id)?)])?;
    space_index_action("openArtifact", Some(args))
}

/// 📊️ `config` supplies the live presence fold (`presence-heartbeat`/`fold-directory-events`); the ID
/// column's own cell still carries the raw artifact id, while the row's OWN identity carries the
/// `artifact:<id>` grammar contract §C0 needs. One `TableRow` record per artifact inside the windowed
/// table kit, so a space of any size stays inside the window's node budget. Split out from `render`
/// (lane 4-F) so the pure table structure stays unit-testable in isolation.
fn render_table(config: &SpaceIndexConfig, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    TableWindowKit::render_rows(windows, "Artifacts", &SPACE_INDEX_TABLE_COLUMNS, Some("Actions"), &config.indexed_artifacts, |row| {
        let cells = space_index_table_row(row, &config.presence_for(&row.id).join(", "));
        let cells: Vec<&str> = cells.iter().map(String::as_str).collect();
        let key = format!("artifact:{}", row.id);
        table_window_row(&key, &cells, [table_row_action(IconName::FolderOpen.as_str(), "Open", open_artifact_action(row)?)?], Some(open_artifact_action(row)?))
    })
}

''')
edit(index_editor, "pub fn render(_document: &SSpaceSnapshot, config: &SpaceIndexConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {", "pub fn render(_document: &SSpaceSnapshot, config: &SpaceIndexConfig, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {")
edit(index_editor, "    for child in [window_content_dead_line_spacer(), window_content_dead_line_spacer(), create_artifact_button()?, render_table(config)?] {", "    for child in [window_content_dead_line_spacer(), window_content_dead_line_spacer(), create_artifact_button()?, render_table(config, &TreeWindows::for_body(view_state, BODY_KEY))?] {")
edit(index_editor, '''    semio_framework_ui_contract::column()
        .try_children(children)''', '''    semio_framework_ui_contract::column()
        .grow(true)
        .try_children(children)''')
index_root = INDEX_ANY / "✏️editor/🦀️.rs"
edit(index_root, "            main::BODY_KEY => Ok(built_to_component_tree(main::render(doc.snapshot, cfg.snapshot)?)),", "            main::BODY_KEY => Ok(built_to_component_tree(main::render(doc.snapshot, cfg.snapshot, view_state)?)),")

index_editor_tests = INDEX_ANY / "✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs"
edit(index_editor_tests, '''fn buttons(node: &semio_framework_plugin::BuiltNode) -> Vec<&semio_framework_ui_contract::ActionBinding> {
    node.children.iter().filter(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))).map(|child| child.bindings.get(0).expect("Space row button carries an action binding")).collect()
}''', '''fn buttons(node: &semio_framework_plugin::BuiltNode) -> Vec<&semio_framework_ui_contract::ActionBinding> {
    let semio_framework_ui_contract::Component::TableRow(props) = &node.component else { panic!("a Space row is one TableRow record") };
    props.row_actions.iter().map(|action| &action.action).collect()
}''')
edit(index_editor_tests, '''    let _ = project(render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default()).expect("default Space rows"));''', '''    let _ = project(render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default(), &semio_framework_plugin::ViewModel::default()).expect("default Space rows"));''')
edit(index_editor_tests, '''    let json = project(render(&document, &config).expect("Space rows with presence"));''', '''    let json = project(render(&document, &config, &semio_framework_plugin::ViewModel::default()).expect("Space rows with presence"));''')
edit(index_editor_tests, '''    observe(render_table(&config).expect("Space artifact rows"), |root| {''', '''    observe(render_table(&config, &TreeWindows::unhosted()).expect("Space artifact rows"), |root| {''')
edit(index_editor_tests, '''    observe(render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default()).expect("Space rows with create action"), |root| {''', '''    observe(render(&SSpaceSnapshot::default(), &SpaceIndexConfig::default(), &semio_framework_plugin::ViewModel::default()).expect("Space rows with create action"), |root| {''')

index_viewer = INDEX_ANY / "👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs"
edit(index_viewer, "use semio_framework_plugin::app::{TableRow, TableRowsView, TableWindowKit, WindowKit};", "use semio_framework_plugin::app::{table_window_row, TableWindowKit, TreeWindows, WindowKit};")
block(index_viewer, "pub fn render(document: &SSpaceSnapshot)", "//#endregion 🔖️Render", '''/// 👁️ The viewer folds no `fold-directory-events`/`presence-heartbeat` commands of its own (no `Config`
/// state to fold into — `NoConfig`), so its presence cell is always empty; the editor's window is the one
/// live presence source. One `TableRow` record per artifact, no row actions: the viewer has no mutating
/// affordance.
pub fn render(document: &SSpaceSnapshot, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    TableWindowKit::render_rows(&TreeWindows::for_body(view_state, BODY_KEY), "Artifacts", &SPACE_INDEX_TABLE_COLUMNS, None, &document.artifacts, |row| {
        let cells = space_index_table_row(row, "");
        let cells: Vec<&str> = cells.iter().map(String::as_str).collect();
        table_window_row(&format!("artifact:{}", row.id), &cells, std::iter::empty(), None)
    })
}
''')
index_viewer_root = INDEX_ANY / "👁️viewer/🦀️.rs"
edit(index_viewer_root, '''    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => Ok(built_to_component_tree(main::render(doc.snapshot)?)),''', '''    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => Ok(built_to_component_tree(main::render(doc.snapshot, view_state)?)),''')
index_viewer_tests = INDEX_ANY / "👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs"
edit(index_viewer_tests, '''    let _ = project(render(&SSpaceSnapshot::default()).expect("default Space viewer rows"));''', '''    let _ = project(render(&SSpaceSnapshot::default(), &semio_framework_plugin::ViewModel::default()).expect("default Space viewer rows"));''')
edit(index_viewer_tests, '''    observe(render(&document).expect("Space viewer rows"), |root| {
        let row = root.children.iter().find(|node| node.key.as_str() == "artifact:artifact-1").expect("Space viewer row id");
        assert!(!row.children.iter().any(|child| matches!(&child.component, semio_framework_ui_contract::Component::Button(_))), "the viewer never carries a row action button");''', '''    observe(render(&document, &semio_framework_plugin::ViewModel::default()).expect("Space viewer rows"), |root| {
        let row = root.children.iter().find(|node| node.key.as_str() == "artifact:artifact-1").expect("Space viewer row id");
        let semio_framework_ui_contract::Component::TableRow(props) = &row.component else { panic!("a viewer row is one TableRow record") };
        assert!(props.row_actions.is_empty() && row.bindings.is_empty(), "the viewer never carries a row action");''')

kits = PLUGIN / "🧪️tests/🔬️app-window-kits/🦀️.rs"
block(kits, "    #[semio_framework_async_macros::async_test]\n    async fn table_kit_render_rows_stamps_a_stable_row_id", "    #[semio_framework_async_macros::async_test]\n    async fn tree_kit_renders_nested_items()", '''    fn table_fixture_row(index: &usize) -> UiAssemblyResult<BuiltNode> {
        let open = || ActionId::try_v1("s.space.home", "openSpace").expect("bounded action");
        let key = format!("space:{index}");
        let name = format!("Studio {index}");
        table_window_row(&key, &[name.as_str(), "atelier"], [table_row_action(IconName::FolderOpen.as_str(), "Open", (open(), None))?], Some((open(), None)))
    }

    fn table_fixture(windows: &TreeWindows<'_>, total: usize) -> BuiltNode {
        let entries: Vec<usize> = (0..total).collect();
        TableWindowKit::render_rows(windows, "Studios", &["Name", "Kind"], Some("Actions"), &entries, table_fixture_row).expect("windowed table")
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_render_rows_builds_one_table_node_with_one_record_per_row() {
        let node = table_fixture(&TreeWindows::unhosted(), 3);
        let Component::Table(props) = &node.component else { panic!("expected Table") };
        assert_eq!(node.key.as_str(), TableWindowKit::KIND_ID);
        assert_eq!(props.label.0.as_str(), "Studios");
        assert_eq!(props.columns.iter().map(|column| column.0.as_str()).collect::<Vec<_>>(), ["Name", "Kind"]);
        assert_eq!(props.actions_label.as_ref().map(|label| label.0.as_str()), Some("Actions"));
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((3, 0)));
        assert_eq!(node.children.len(), 3);
        let row = node.children.get(1).expect("second row");
        assert_eq!(row.key.as_str(), "space:1");
        let Component::TableRow(row_props) = &row.component else { panic!("expected TableRow") };
        assert_eq!(row_props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>(), ["Studio 1", "atelier"]);
        assert!(row.children.is_empty(), "cells and row actions are props, never child records");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_render_rows_carries_row_actions_and_the_row_activation_as_props() {
        let node = table_fixture(&TreeWindows::unhosted(), 1);
        let row = node.children.get(0).expect("row");
        let Component::TableRow(props) = &row.component else { panic!("expected TableRow") };
        let action = props.row_actions.get(0).expect("row action");
        assert_eq!(action.action.action.name.as_str(), "openSpace");
        assert_eq!(action.label.as_ref().map(|label| label.0.as_str()), Some("Open"));
        let activate = row.bindings.iter().find(|binding| binding.trigger == Trigger::Activate).expect("row activation");
        assert_eq!(activate.action.name.as_str(), "openSpace");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_render_rows_serves_exactly_the_hosts_window_on_the_shared_ledger() {
        let view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: TableWindowKit::KIND_ID.to_string(), open: Some(true), offset: 200, rows: 20 }], ..Default::default() };
        let windows = TreeWindows::for_body(&view, "body");
        let node = table_fixture(&windows, 500);
        let Component::Table(props) = &node.component else { panic!("expected Table") };
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((500, 200)));
        assert_eq!(node.children.len(), 20);
        assert_eq!(node.children.get(0).expect("first served row").key.as_str(), "space:200");
        assert_eq!(windows.nodes_remaining(), TREE_WINDOW_BODY_NODE_BUDGET - 21, "the table and its rows are charged to the body ledger the tree panels spend");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_first_paint_stays_inside_the_body_node_budget_at_any_row_count() {
        let windows = TreeWindows::unhosted();
        let node = table_fixture(&windows, 10_000);
        let Component::Table(props) = &node.component else { panic!("expected Table") };
        assert_eq!(props.window.map(|window| window.total), Some(10_000));
        assert!(node.children.len() + 1 <= TREE_WINDOW_BODY_NODE_BUDGET, "a first paint of 10 000 rows builds {} records", node.children.len() + 1);
        assert_eq!(node.children.len(), TREE_WINDOW_DEFAULT_ROWS as usize, "an unhosted first paint serves one default viewport of rows");
    }

''')

by_path: dict[pathlib.Path, str] = {}
for path, start, end, new in blocks:
    text = by_path.get(path) or path.read_text(encoding="utf-8")
    if text.count(start) != 1 or text.count(end) != 1:
        sys.exit(f"refused block: {path.relative_to(ROOT)} start={text.count(start)} end={text.count(end)} {start[:60]!r}")
    first = text.index(start)
    last = text.index(end)
    if last <= first:
        sys.exit(f"refused block order: {path.relative_to(ROOT)}")
    by_path[path] = text[:first] + new + text[last:]
for path, old, new in edits:
    text = by_path.get(path) or path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        sys.exit(f"refused: {path.relative_to(ROOT)} expects one match, found {count}: {old[:100]!r}")
    by_path[path] = text.replace(old, new)
home_tests = by_path[home_editor_tests]
home_tests = home_tests.replace("render_rows(&[], &HomeTableLabels::", "rows(&TreeWindows::unhosted(), &[], &HomeTableLabels::")
home_tests = home_tests.replace("render_rows(&[", "rows(&TreeWindows::unhosted(), &[")
home_tests = home_tests.replace("    render_rows_wrapped(rows, &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN)\n", "    render_rows_wrapped(rows, &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN, &TreeWindows::unhosted())\n")
if "render_rows(&" in home_tests:
    sys.exit("refused: home tests still call render_rows directly")
by_path[home_editor_tests] = home_tests
for path, text in by_path.items():
    path.write_text(text, encoding="utf-8")
    print(f"edited {path.relative_to(ROOT)}")
