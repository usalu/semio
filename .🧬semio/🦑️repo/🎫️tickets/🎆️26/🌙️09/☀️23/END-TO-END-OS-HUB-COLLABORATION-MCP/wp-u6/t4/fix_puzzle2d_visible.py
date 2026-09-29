"""🙈️ puzzle2d: a node/handle/edge is hidden through its document field `visible` — every hide write/read goes through it."""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"

def edit(rel, pairs):
    p = ROOT / rel
    s = p.read_text()
    for old, new in pairs:
        assert s.count(old) == 1, (rel, old[:90], s.count(old))
        s = s.replace(old, new)
    p.write_text(s)

edit("🦀️.rs", [
    ('''pub fn apply_selection_flag(fixture: &mut Value, selected: &[String], flag: &str, value: bool) {
    if selected.is_empty() {
        return;
    }
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let key = if flag == "locked" { "locked" } else { "hidden" };''', '''/// 🙈️ Whether a node, handle or edge is hidden. The document stores the inverse, `visible` (absent = shown): a `hidden`
/// key written onto one of them never reaches the typed model, so it would neither persist nor reach a peer.
pub fn puzzle2d_entity_hidden(entity: &Value) -> bool {
    entity.get("visible").and_then(Value::as_bool) == Some(false)
}

/// 🙈️ The document field and value one flag write stores on a node, handle or edge — `hidden` is written as its inverse,
/// `visible`, the field the typed model carries; `locked` is stored as itself.
fn puzzle2d_entity_flag_entry(flag: &str, value: bool) -> (&'static str, bool) {
    if flag == "locked" {
        ("locked", value)
    } else {
        ("visible", !value)
    }
}

pub fn apply_selection_flag(fixture: &mut Value, selected: &[String], flag: &str, value: bool) {
    if selected.is_empty() {
        return;
    }
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let (key, value) = puzzle2d_entity_flag_entry(flag, value);'''),
    ('''pub fn patch_inspector_nodes(fixture: &mut Value, ids: &[String], field: &str, value: Option<&Value>, delta: Option<&Value>) {
    let Some(nodes)''', '''pub fn patch_inspector_nodes(fixture: &mut Value, ids: &[String], field: &str, value: Option<&Value>, delta: Option<&Value>) {
    let visible = (field == "hidden").then(|| value.and_then(Value::as_bool).map(|hidden| Value::Bool(!hidden))).flatten();
    let (field, value) = if field == "hidden" { ("visible", visible.as_ref()) } else { (field, value) };
    let Some(nodes)'''),
    ('''    let any_visible = entities.iter().any(|entity| entity.get("hidden").and_then(|v| v.as_bool()) != Some(true));''',
     '''    let any_visible = entities.iter().any(|entity| !puzzle2d_entity_hidden(entity));'''),
])
edit("📌️panels/🗿️artifact/🦀️.rs", [
    ('''    let (hidden, locked) = (flag(node, "hidden"), flag(node, "locked"));''', '''    let (hidden, locked) = (crate::editor::puzzle2d::puzzle2d_entity_hidden(node), flag(node, "locked"));'''),
])
edit("📌️panels/🔍️inspection/🦀️.rs", [
    ('''    flag_row(&mut fields, "node.hidden", labels.hidden.as_str(), "hidden", flag(node, "hidden"))?;''', '''    flag_row(&mut fields, "node.hidden", labels.hidden.as_str(), "hidden", crate::editor::puzzle2d::puzzle2d_entity_hidden(node))?;'''),
    ('''    flag_row(&mut fields, "edge.hidden", labels.hidden.as_str(), "hidden", flag(edge, "hidden"))?;''', '''    flag_row(&mut fields, "edge.hidden", labels.hidden.as_str(), "hidden", crate::editor::puzzle2d::puzzle2d_entity_hidden(edge))?;'''),
])
edit("🎮️commands/🔗️proximity-connect/🦀️.rs", [
    ('''        .filter(|handle| handle.get("hidden").and_then(Value::as_bool) != Some(true) && handle.get("locked").''', '''        .filter(|handle| !puzzle2d_entity_hidden(handle) && handle.get("locked").'''),
    ('''    if moved.get("hidden").and_then(Value::as_bool) == Some(true) || moved.get("locked")''', '''    if puzzle2d_entity_hidden(moved) || moved.get("locked")'''),
    ('''        if node.get("id").and_then(Value::as_str) == Some(node_id) || node.get("hidden").and_then(Value::as_bool) == Some(true) {''', '''        if node.get("id").and_then(Value::as_str) == Some(node_id) || puzzle2d_entity_hidden(node) {'''),
])
edit("📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs", [
    ('''    scene.fixture["nodes"][0]["hidden"] = serde_json::json!(hidden);''', '''    scene.fixture["nodes"][0]["visible"] = serde_json::json!(!hidden);'''),
])
print("ok")
