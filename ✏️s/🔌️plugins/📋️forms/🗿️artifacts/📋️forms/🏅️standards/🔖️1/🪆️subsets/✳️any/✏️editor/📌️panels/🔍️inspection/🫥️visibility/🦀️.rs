//! 🫥️ Visual, localized conditional visibility authoring without executable expressions.
use super::controls::*;
use crate::editor::forms::{ui_admit, ui_label, ui_text_value, ui_value_text};
use crate::editor::forms::terminology::FormsLabels;
use crate::{FormExpr, FormQuestion, FormStep};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult, UiValue};
use semio_framework_ui_contract as ui;

fn args(question: &FormQuestion, path: &str, field: &str, button: bool) -> UiAssemblyResult<UiValue> {
    let mut entries = vec![("field", ui_value_text(format!("condition.{field}"))?), ("paramKey", ui_value_text(path)?), ("questionIds", question_ids(&[question.id.clone()])?)];
    if button { entries.push(("valueJson", ui_value_text("null")?)); }
    arguments(entries)
}

pub fn render(question: &FormQuestion, steps: &[FormStep], labels: &FormsLabels) -> UiAssemblyResult<BuiltNode> {
    render_node(question, question.condition.as_ref(), "", steps, labels)
}

fn render_node(question: &FormQuestion, expression: Option<&FormExpr>, path: &str, steps: &[FormStep], labels: &FormsLabels) -> UiAssemblyResult<BuiltNode> {
    let id = format!("forms-play-inspector.visibility.{}", if path.is_empty() { "root".into() } else { path.replace('/', ".") });
    let kind = match expression {
        None => "none", Some(FormExpr::Const { .. }) => "const", Some(FormExpr::Var { .. }) => "var",
        Some(FormExpr::Eq { .. }) => "eq", Some(FormExpr::Truthy { .. }) => "truthy",
        Some(FormExpr::And { .. }) => "and", Some(FormExpr::Or { .. }) => "or",
    };
    let mut select = ui::select(ui_text_value(kind)?);
    let mut kinds = vec![("const", labels.rule_constant.as_str()), ("var", labels.rule_answer.as_str()), ("eq", labels.rule_equals.as_str()), ("truthy", labels.rule_answered.as_str()), ("and", labels.rule_all.as_str()), ("or", labels.rule_any.as_str())];
    if path.is_empty() { kinds.insert(0, ("none", labels.rule_always.as_str())); }
    for (kind, label) in kinds { select = ui_admit(select.try_item(ui_text_value(kind)?, ui_label(label)?))?; }
    let mut rows = vec![row(&format!("{id}.kind"), labels.rule.as_str(), select, "patchQuestions", args(question, path, "kind", false)?)?];
    let child_path = |index| if path.is_empty() { format!("{index}") } else { format!("{path}/{index}") };
    match expression {
        Some(FormExpr::Var { name }) => {
            let mut select = ui_admit(ui::select(ui_text_value(name)?).try_item(ui_text_value("")?, ui_label(labels.choose_question.as_str())?))?;
            for source in steps.iter().flat_map(|step| &step.blocks).filter(|source| source.id != question.id) {
                select = ui_admit(select.try_item(ui_text_value(&source.id)?, ui_label(&source.label)?))?;
            }
            rows.push(row(&format!("{id}.source"), labels.question.as_str(), select, "patchQuestions", args(question, path, "name", false)?)?);
        }
        Some(FormExpr::Const { value }) => {
            let kind = match value { dsl::DslValue::Bool(_) => "boolean", dsl::DslValue::Number(_) => "number", dsl::DslValue::String(_) => "text", dsl::DslValue::Null => "null", _ => "structured" };
            let mut select = ui::select(ui_text_value(kind)?);
            for (value, label) in [("text", labels.kind_text.as_str()), ("number", labels.kind_number.as_str()), ("boolean", labels.kind_boolean.as_str()), ("null", labels.rule_empty.as_str())] {
                select = ui_admit(select.try_item(ui_text_value(value)?, ui_label(label)?))?;
            }
            rows.push(row(&format!("{id}.value-type"), labels.kind.as_str(), select, "patchQuestions", args(question, path, "valueType", false)?)?);
            if let dsl::DslValue::Bool(value) = value {
                rows.push(row(&format!("{id}.value"), labels.value.as_str(), ui::toggle(*value).text(ui_label(labels.value.as_str())?), "patchQuestions", args(question, path, "value", false)?)?);
            } else if kind == "text" || kind == "number" {
                rows.push(row(&format!("{id}.value"), labels.value.as_str(), ui::input(if kind == "number" { ui::InputKind::Number } else { ui::InputKind::Text }).value(ui_text_value(crate::schema::dsl_string_value(value))?), "patchQuestions", args(question, path, "value", false)?)?);
            }
        }
        Some(FormExpr::Eq { left, right }) => {
            rows.push(render_node(question, Some(left), &child_path(0), steps, labels)?);
            rows.push(render_node(question, Some(right), &child_path(1), steps, labels)?);
        }
        Some(FormExpr::Truthy { expr }) => rows.push(render_node(question, Some(expr), &child_path(0), steps, labels)?),
        Some(FormExpr::And { items } | FormExpr::Or { items }) => {
            for (index, item) in items.iter().enumerate() { rows.push(render_node(question, Some(item), &child_path(index), steps, labels)?); }
            rows.push(button(&format!("{id}.add"), labels.add_rule.as_str(), "patchQuestions", args(question, path, "add", true)?)?);
        }
        None => {}
    }
    if expression.is_some() { rows.push(button(&format!("{id}.remove"), labels.remove.as_str(), "patchQuestions", args(question, path, "remove", true)?)?); }
    group(&id, labels.rule.as_str(), rows)
}
