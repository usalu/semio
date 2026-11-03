import sys
sys.path.insert(0, ".")
from spec import *
from rows import *
p = MUT + "/🧭️edit-rules/🦀️.rs"
s = open(p, encoding="utf8").read()
if "fn update(" not in s:
    ins = "".join(f'        InsertItemRule {{ path: "{r.camel_coll}", kind: "insert-{r.slug}", selectors: &[], index: "index", item: "{camel(r.item)}" }},\n' for r in ROWS if "insert" not in r.existing)
    rem = "".join(f'        RemoveItemRule {{ path: "{r.camel_coll}", kind: "remove-{r.slug}", selectors: &[], index: "index" }},\n' for r in ROWS if "remove" not in r.existing)
    s = s.replace('        InsertItemRule { path: "piles", kind: "insert-pile", selectors: &[], index: "index", item: "pile" },\n', '        InsertItemRule { path: "piles", kind: "insert-pile", selectors: &[], index: "index", item: "pile" },\n' + ins, 1)
    s = s.replace('        RemoveItemRule { path: "piles", kind: "remove-pile", selectors: &[], index: "index" },\n', '        RemoveItemRule { path: "piles", kind: "remove-pile", selectors: &[], index: "index" },\n' + rem, 1)
    arms = "".join(f'        "{r.camel_coll}" => En1997Mutation::{struct(r, "update")}({mod(r, "update")}::{struct(r, "update")} {{ {r.item}: decode(entity)? }}),\n' for r in ROWS)
    uses = ", ".join(mod(r, "update") for r in ROWS)
    s = s.replace("use semio_s_plugin_norm_app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};", "use semio_framework_value::{DslValue, FromValue, ToValue};\nuse semio_s_plugin_norm_app_surface::{get_value_at_path, parse_path, set_value_at_path, InsertItemRule, NormEdit, NormEditRules, PathSegment, RemoveItemRule, SelectorKey, SetFieldRule};")
    s = s.replace("use crate::En1997Mutation;", f"use crate::{{{uses}, En1997Mutation}};")
    i = s.index("/// 🎯️ Resolves an editor edit")
    new = '''fn decode<T: FromValue>(entity: &DslValue) -> Result<T, String> {
    T::from_value(entity.clone()).map_err(|error| format!("{error:?}"))
}

fn update(collection: &str, entity: &DslValue) -> Result<En1997Mutation, String> {
    Ok(match collection {
''' + arms + '''        other => return Err(format!("no update kind for collection '{other}'")),
    })
}

/// 🧬️ A leaf below `collection[id=…]` (also below nested `loadCases`, `testProfiles`, `tests`, `points` rows) raises the whole-row `update-<row>` kind of its top-level row; the row id itself is the key and stays immutable.
fn resolve_row_leaf(document: &En1997Snapshot, path: &str, value: &DslValue) -> Result<Vec<En1997Mutation>, String> {
    let segments = parse_path(path)?;
    let (Some(PathSegment::Field(collection)), Some(PathSegment::Id(id))) = (segments.first(), segments.get(1)) else { return Err(format!("path '{path}' does not address a row leaf")) };
    if segments.len() == 3 && matches!(segments.get(2), Some(PathSegment::Field(field)) if field == "id") {
        return Err(format!("the id of '{collection}[id={id}]' is its key and cannot be edited"));
    }
    let mut tree = document.to_value();
    set_value_at_path(&mut tree, path, value.clone())?;
    let entity = get_value_at_path(&tree, &format!("{collection}[id={id}]"))?;
    Ok(vec![update(collection, entity)?])
}

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first, then the scalar and row-list rules of [`EDIT_RULES`], and every other row leaf through its typed whole-row `update-<row>` kind.
pub fn resolve_edit(document: &En1997Snapshot, edit: &NormEdit) -> Result<Vec<En1997Mutation>, String> {
    if let NormEdit::RemoveItem { path, index } = edit {
        if let Some(rule) = ID_REMOVALS.iter().find(|rule| rule.path == path) {
            let tree = semio_framework_value::ToValue::to_value(document);
            let mut cursor = &tree;
            for segment in path.split('.') {
                let semio_framework_value::DslValue::Object(entries) = cursor else { return Err(format!("expected object before '{segment}' in path '{path}'")) };
                cursor = &entries.iter().find(|(key, _)| key == segment).ok_or_else(|| format!("missing field '{segment}' in path '{path}'"))?.1;
            }
            let semio_framework_value::DslValue::Array(items) = cursor else { return Err(format!("'{path}' is not a list")) };
            let semio_framework_value::DslValue::Object(row) = items.get(*index).ok_or_else(|| format!("'{path}' has no row {index}"))? else { return Err(format!("row {index} of '{path}' is not a record")) };
            let id = row.iter().find(|(key, _)| key == "id").map(|(_, value)| value.clone()).ok_or_else(|| format!("row {index} of '{path}' has no id"))?;
            return <En1997Mutation as protocol::Mutation<En1997Snapshot>>::from_payload_value(rule.kind, semio_framework_value::DslValue::object([(rule.id.to_string(), id)])).map(|mutation| vec![mutation]).map_err(|error| error.to_string());
        }
    }
    match edit {
        NormEdit::SetField { path, value } => EDIT_RULES.resolve(document, edit).or_else(|first| resolve_row_leaf(document, path, value).map_err(|second| format!("{first}; {second}"))),
        _ => EDIT_RULES.resolve(document, edit),
    }
}
'''
    s = s[:i] + new
    open(p, "w", encoding="utf8").write(s)
print("ok")
