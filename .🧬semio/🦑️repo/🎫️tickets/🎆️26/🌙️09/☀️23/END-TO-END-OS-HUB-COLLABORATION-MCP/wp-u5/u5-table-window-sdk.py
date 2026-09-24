#!/usr/bin/env python3
"""📊️ U5 6b — one-off: the plugin SDK's windowed table kit replaces the fixed 32-row `TableRowsView`.

`TableWindowKit::render_rows` builds a `Component::Table` whose rows are `Component::TableRow` records (cells and row
actions as props, one node each) and only the host's slice of them, charged to the SAME `TreeWindows` ledger the tree
panels spend. The retained `TableRowsView` carrier, its retire arena and the reactor's arena ladder go away: rows are
ordinary built nodes now, retired by the built-node ladder every other node already uses."""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
SDK = PLUGIN / "🦀️.rs"

NEW_KIT = '''    /// 🎬️ One row action of a windowed table row — an `Activate` binding with its icon and accessible label,
    /// painted in the table's trailing actions column.
    pub fn table_row_action(icon: &str, label: &str, action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<RowAction> {
        Ok(RowAction {
            icon: UiText::try_from_str(icon).ok_or_else(|| ui_assembly_error("table-window.action-icon"))?,
            label: Some(Label::try_from(label).map_err(|_| ui_assembly_error("table-window.action-label"))?),
            action: ActionBinding { trigger: Trigger::Activate, action: action.0, args: action.1, capability: None },
            placement: RowActionPlacement::Row,
        })
    }

    /// 📊️ One windowed table row: record key `key` (the row's stable identity, e.g. `"space:<id>"`),
    /// `cells` positional to the table's columns, `actions` in its actions column and `activate` its primary
    /// activation (Enter on the focused row). One node record however many cells and actions it carries.
    pub fn table_window_row(key: &str, cells: &[&str], actions: impl IntoIterator<Item = RowAction>, activate: Option<(ActionId, Option<UiValue>)>) -> UiAssemblyResult<BuiltNode> {
        let mut row_cells = UiFixedList::default();
        for cell in cells {
            row_cells.try_push(UiText::try_from_str(cell).ok_or_else(|| ui_assembly_error("table-window.cell"))?).map_err(|_| ui_assembly_error("table-window.cells"))?;
        }
        let mut builder = table_row(row_cells).try_id(key).map_err(|_| ui_assembly_error("table-window.row-id"))?;
        for action in actions {
            builder = builder.try_row_action(action).map_err(|_| ui_assembly_error("table-window.row-actions"))?;
        }
        let builder = match activate {
            Some((action, Some(args))) => builder.try_on_with(Trigger::Activate, action, args).map_err(|_| ui_assembly_error("table-window.row-activate"))?,
            Some((action, None)) => builder.try_on(Trigger::Activate, action).map_err(|_| ui_assembly_error("table-window.row-activate"))?,
            None => builder,
        };
        builder.try_build().map_err(|_| ui_assembly_error("table-window.row-build"))
    }

    impl TableWindowKit {
        /// 📊️ One windowed table as the window body's growing child: the header (`columns`) and accessible
        /// name (`label`) live on the `Table` node, and only the host's slice of `entries` is built — each a
        /// single [`table_window_row`] record — so the table costs `1 + materialised rows` on the SAME
        /// body-wide [`TreeWindows`] ledger the tree panels spend, and its `window` stamp is the contract the
        /// host virtualises trees with. Any row count stays inside `UI_DOCUMENT_NODES`: the host streams the
        /// rows its viewport shows, the first paint serves what the unreserved ledger and the viewport budget
        /// allow.
        pub fn render_rows<T>(windows: &TreeWindows<'_>, label: &str, columns: &[&str], actions_label: Option<&str>, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
            TreeWindows::admit_key(Self::KIND_ID)?;
            let path = windows.path_of(Self::KIND_ID);
            windows.claim_window(&path, entries.len())?;
            windows.debit_container();
            let slice = windows.sliced(&path, true, entries.len());
            let mut headers = UiFixedList::default();
            for column in columns {
                headers.try_push(Label::try_from(*column).map_err(|_| ui_assembly_error("table-window.column"))?).map_err(|_| ui_assembly_error("table-window.columns"))?;
            }
            let builder = table(Label::try_from(label).map_err(|_| ui_assembly_error("table-window.label"))?, headers).grow(true).try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("table-window.id"))?;
            let builder = match actions_label {
                Some(actions_label) => builder.actions_label(Label::try_from(actions_label).map_err(|_| ui_assembly_error("table-window.actions-label"))?),
                None => builder,
            };
            let builder = tree_window_rows(builder, windows, Self::KIND_ID, entries, &slice, row)?;
            let builder = match windows.stamp(&path, &slice) {
                Some(window) => builder.window(window),
                None => builder,
            };
            builder.try_build().map_err(|_| ui_assembly_error("table-window.build"))
        }
    }
'''

text = SDK.read_text(encoding="utf-8")
start = text.index("    pub const TABLE_WINDOW_COLUMNS: usize = 32;\n")
end = text.index("    //#endregion 🔖️TableWindowKit\n")
if text.count("    pub const TABLE_WINDOW_COLUMNS: usize = 32;\n") != 1 or text.count("    //#endregion 🔖️TableWindowKit\n") != 1:
    sys.exit("refused: table kit anchors")
text = text[:start] + NEW_KIT + text[end:]

reactor = PLUGIN / "⚛️reactor/🔄️turn/🦀️.rs"
reactor_text = reactor.read_text(encoding="utf-8")
old = "    retire_while_progress(retirement_deadline, || crate::app::close_table_rows_view_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT));\n"
if reactor_text.count(old) != 1:
    sys.exit("refused: reactor table ladder")
reactor_text = reactor_text.replace(old, "")

SDK.write_text(text, encoding="utf-8")
reactor.write_text(reactor_text, encoding="utf-8")
print("edited", SDK.relative_to(ROOT))
print("edited", reactor.relative_to(ROOT))
