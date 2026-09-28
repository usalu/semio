#!/usr/bin/env python3
"""🏠️ LB2 prepared patch p3 — ONE row-action representation for windowed table rows (SDK half; WG11 owns the wgpu TableRow
painter, both land together in window 3).

Measured (S18, session 14b): Home rows carried each row action twice — the `RowAction` prop AND a `row-action-<i>` child
button (`table_row_action_buttons`, added so the wgpu target, which never painted `TableRowProps.row_actions`, had something
to paint) — so the body ITEM budget ended Home's window after 27 rows < a 32-row viewport and visible rows stayed blank.
The contract already says row actions are props "rendered in the table's trailing actions column"; the React Interpreter
paints them from `rowActions` and filtered the duplicate children out by key prefix.

Hunks: SDK drops `table_row_action_buttons` and both call sites (`table_window_row`, `editable_table_window_row_at`: an
editable row's children are exactly its cells again); React Interpreter's `row-action-` key filter goes (a row's children
are its cell nodes); law `home_shaped_rows_carry_actions_as_props_and_fill_the_default_window` (SDK app-window-kits tests)
+ fixture/schema `🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity`.

Usage: lb2-p3-row-actions.py [--dry-run | --write] [--root <repo-or-overlay root>]  (default --dry-run on the live tree)"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
PAYLOAD = Path(__file__).resolve().parent / "payload" / "p3"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
TESTS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs"
INTERPRETER = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx"
FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity/🔣️.json"
SCHEMA = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity/🧬️schema/🔣️.json"

BUTTONS_FN = """    fn table_row_action_buttons(actions: &[RowAction]) -> UiAssemblyResult<Vec<BuiltNode>> {
        let mut buttons = Vec::with_capacity(actions.len());
        for (index, action) in actions.iter().enumerate() {
            let duplicate = action.credited_clone().ok_or_else(|| ui_assembly_error("table-window.row-action-clone"))?;
            let label = duplicate.label.ok_or_else(|| ui_assembly_error("table-window.row-action-label"))?;
            let binding = duplicate.action;
            if binding.capability.is_some() {
                return Err(ui_assembly_error("table-window.row-action-capability"));
            }
            let builder = ui::button(label).icon(duplicate.icon).try_id(&format!("row-action-{index}")).map_err(|_| ui_assembly_error("table-window.row-action-id"))?;
            let builder = match binding.args {
                Some(arguments) => builder.try_on_with(binding.trigger, binding.action, arguments).map_err(|_| ui_assembly_error("table-window.row-action-binding"))?,
                None => builder.try_on(binding.trigger, binding.action).map_err(|_| ui_assembly_error("table-window.row-action-binding"))?,
            };
            buttons.push(builder.try_build().map_err(|_| ui_assembly_error("table-window.row-action-button"))?);
        }
        Ok(buttons)
    }

"""
ROW_DOC_OLD = """    /// activation (Enter on the focused row). One node record however many cells and actions it carries.
"""
ROW_DOC_NEW = """    /// activation (Enter on the focused row). One node record however many cells and actions it carries: row actions
    /// travel only as [`RowAction`] props, the one representation every renderer paints in the trailing actions column.
"""
ROW_OLD = """        let actions = actions.into_iter().collect::<Vec<_>>();
        let action_buttons = table_row_action_buttons(&actions)?;
        let mut builder = table_row(row_cells).try_id(key).map_err(|_| ui_assembly_error("table-window.row-id"))?;
        if !action_buttons.is_empty() {
            builder = builder.try_children(action_buttons).map_err(|_| ui_assembly_error("table-window.row-action-children"))?;
        }
        for action in actions {
"""
ROW_NEW = """        let mut builder = table_row(row_cells).try_id(key).map_err(|_| ui_assembly_error("table-window.row-id"))?;
        for action in actions {
"""
EDITABLE_OLD = """        let actions = actions.into_iter().collect::<Vec<_>>();
        children.extend(table_row_action_buttons(&actions)?);
        let mut builder = table_row(row_cells).try_id(key).map_err(|_| ui_assembly_error("table-window.editable-row-key"))?;
"""
EDITABLE_NEW = """        let mut builder = table_row(row_cells).try_id(key).map_err(|_| ui_assembly_error("table-window.editable-row-key"))?;
"""
FILTER_OLD = """              const cellNodes = (row.children ?? []).filter((id) => !store.getState().nodes.get(id)?.key.startsWith("row-action-"));
"""
FILTER_NEW = """              const cellNodes = row.children ?? [];
"""
LAW_ANCHOR = """        assert_eq!(node.children.len(), 60, "rows well inside the item budget are all materialised");
        assert!(windows.items_remaining() > 0);
    }
"""

problems, changed = [], {}


def text(rel):
    return changed[rel] if rel in changed else (ROOT / rel).read_text(encoding="utf-8")


def replace_once(rel, old, new, done_marker):
    current = text(rel)
    if done_marker(current):
        print(f"already applied: {rel} ({old.strip().splitlines()[0][:60]!r})")
        return
    count = current.count(old)
    if count != 1:
        problems.append(f"{rel}: expected 1× {old.strip().splitlines()[0][:70]!r}, found {count}")
        return
    changed[rel] = current.replace(old, new, 1)


replace_once(SDK, BUTTONS_FN, "", lambda current: "fn table_row_action_buttons" not in current)
replace_once(SDK, ROW_DOC_OLD, ROW_DOC_NEW, lambda current: "row actions\n    /// travel only as [`RowAction`] props" in current)
replace_once(SDK, ROW_OLD, ROW_NEW, lambda current: "let action_buttons = table_row_action_buttons" not in current)
replace_once(SDK, EDITABLE_OLD, EDITABLE_NEW, lambda current: "children.extend(table_row_action_buttons" not in current)
replace_once(INTERPRETER, FILTER_OLD, FILTER_NEW, lambda current: FILTER_NEW in current)
law = (PAYLOAD / "law.rs").read_text(encoding="utf-8")
replace_once(TESTS, LAW_ANCHOR, LAW_ANCHOR + law, lambda current: "fn home_shaped_rows_carry_actions_as_props_and_fill_the_default_window" in current)
for rel, source in [(FIXTURE, "fixture.json"), (SCHEMA, "schema.json")]:
    payload = (PAYLOAD / source).read_text(encoding="utf-8")
    target = ROOT / rel
    if target.exists():
        if target.read_text(encoding="utf-8") == payload:
            print(f"already applied: {rel}")
        else:
            problems.append(f"{rel} exists with different content")
        continue
    changed[rel] = payload
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={len(changed)} problems={len(problems)} write={WRITE}")
for rel in changed:
    print("  ", rel)
if WRITE and not problems:
    for rel, content in changed.items():
        target = ROOT / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content, encoding="utf-8")
    print("written")
sys.exit(1 if problems else 0)
