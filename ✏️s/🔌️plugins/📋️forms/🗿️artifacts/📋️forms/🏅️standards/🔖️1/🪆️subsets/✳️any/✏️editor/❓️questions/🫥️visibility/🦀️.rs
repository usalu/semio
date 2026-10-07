//! 🫥️ Visual condition edits preserve the closed expression tree and sibling rules.
use crate::FormExpr;
use crate::standards::v1::subsets::any::io::text::snapshot::value_to_dsl;
use semio_framework_pack_json::Value;

fn create_condition(kind: &str) -> Result<FormExpr, String> {
    Ok(match kind {
        "const" => FormExpr::Const { value: semio_framework_value::DslValue::Bool(true) },
        "var" => FormExpr::Var { name: String::new() },
        "eq" => FormExpr::Eq { left: Box::new(create_condition("var")?), right: Box::new(FormExpr::Const { value: semio_framework_value::DslValue::String(String::new()) }) },
        "truthy" => FormExpr::Truthy { expr: Box::new(create_condition("var")?) },
        "and" => FormExpr::And { items: vec![create_condition("const")?] },
        "or" => FormExpr::Or { items: vec![create_condition("const")?] },
        _ => return Err("invalid-condition-edit".into()),
    })
}

fn edit(node: &mut FormExpr, path: &[usize], field: &str, value: &Value) -> Result<(), String> {
    if let Some((&index, rest)) = path.split_first() {
        match node {
            FormExpr::And { items } | FormExpr::Or { items } => {
                if index >= items.len() { return Err("invalid-path".into()); }
                if rest.is_empty() && field == "remove" { items.remove(index); }
                else { edit(&mut items[index], rest, field, value)?; }
            }
            FormExpr::Eq { left, right } if index < 2 => edit(if index == 0 { left } else { right }, rest, field, value)?,
            FormExpr::Truthy { expr } if index == 0 => edit(expr, rest, field, value)?,
            _ => return Err("invalid-path".into()),
        }
        return Ok(());
    }
    if field == "kind" {
        let kind = value.as_str().ok_or("invalid-condition-edit")?;
        let same = matches!((&*node, kind), (FormExpr::Const { .. }, "const") | (FormExpr::Var { .. }, "var") | (FormExpr::Eq { .. }, "eq") | (FormExpr::Truthy { .. }, "truthy") | (FormExpr::And { .. }, "and") | (FormExpr::Or { .. }, "or"));
        if !same { *node = create_condition(kind)?; }
        return Ok(());
    }
    if field == "remove" { *node = FormExpr::Const { value: semio_framework_value::DslValue::Bool(false) }; return Ok(()); }
    match (node, field) {
        (FormExpr::Var { name }, "name") => *name = value.as_str().ok_or("invalid-condition-edit")?.into(),
        (FormExpr::Const { value: slot }, "value") => *slot = value_to_dsl(value),
        (FormExpr::Const { value: slot }, "valueType") => *slot = match value.as_str() {
            Some("boolean") => semio_framework_value::DslValue::Bool(false), Some("number") => semio_framework_value::DslValue::int(0),
            Some("text") => semio_framework_value::DslValue::String(String::new()), Some("null") => semio_framework_value::DslValue::Null,
            _ => return Err("invalid-condition-edit".into()),
        },
        (FormExpr::And { items } | FormExpr::Or { items }, "add") => items.push(create_condition("const")?),
        _ => return Err("invalid-condition-edit".into()),
    }
    Ok(())
}

/// 🧭️ Applies one validated rule edit; an empty root condition means always visible.
pub fn patch_condition(condition: Option<&FormExpr>, path: &str, field: &str, value: &Value) -> Result<Option<FormExpr>, String> {
    let indices: Vec<usize> = if path.is_empty() { Vec::new() } else { path.split('/').map(|part| {
        if part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()) { return Err("invalid-path".to_string()); }
        part.parse().map_err(|_| "invalid-path".to_string())
    }).collect::<Result<_, _>>()? };
    if indices.len() > 32 { return Err("invalid-path".into()); }
    if indices.is_empty() && (field == "remove" || field == "kind" && value.as_str() == Some("none")) { return Ok(None); }
    let mut root = condition.cloned().unwrap_or(create_condition("const")?);
    edit(&mut root, &indices, field, value)?;
    Ok(Some(root))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️editing/🦀️.rs"]
mod tests;
