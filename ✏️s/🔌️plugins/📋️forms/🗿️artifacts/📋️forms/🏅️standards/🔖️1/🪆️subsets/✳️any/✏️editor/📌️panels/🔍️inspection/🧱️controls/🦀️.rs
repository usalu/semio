//! 🎛️ Accessible semantic controls for editing Forms properties.
use crate::editor::forms::{forms_action, ui_admit, ui_label, ui_text_value, ui_value_list, ui_value_map, ui_value_text};
use crate::editor::forms::terminology::FormsLabels;
use crate::FormQuestion;
use semio_framework_plugin::{tree_item_with_action, BuiltNode, UiAssemblyResult, UiValue};
use semio_framework_ui_contract as ui;
use ui::{Buildable, HasBase, HasChildren};

pub fn arguments(mut entries: Vec<(&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    entries.sort_by_key(|(key, _)| *key);
    ui_value_map(entries)
}

pub fn question_ids(ids: &[String]) -> UiAssemblyResult<UiValue> {
    ui_value_list(ids.iter().map(|id| ui_value_text(id)).collect::<UiAssemblyResult<Vec<_>>>()?)
}

pub fn row(id: &str, label: &str, control: impl Into<BuiltNode>, action: &str, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    row_on(id, label, control, ui::Trigger::Change, action, args)
}

fn row_on(id: &str, label: &str, control: impl Into<BuiltNode>, trigger: ui::Trigger, action: &str, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    let mut control = control.into();
    control.key = ui_text_value(format!("{id}.control"))?;
    control.accessibility.label = Some(ui_label(label)?);
    let (action, args) = forms_action(action, Some(args))?;
    ui_admit(control.bindings.try_push(ui::ActionBinding { trigger, action, args, capability: None }))?;
    ui_admit(ui_admit(ui_admit(ui::tree_item(ui_label(label)?).try_id(id))?.try_child(control))?.try_build())
}

/// ✍️ A typed field is one edit per committed value: a text-like field commits on blur or Enter (`Trigger::Commit`), a
/// number field is a continuous control whose press — the framework scrub — commits ONE transaction on release.
pub fn input_row(id: &str, label: &str, kind: ui::InputKind, value: impl AsRef<str>, action: &str, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    let input = ui::input(kind).value(ui_text_value(value.as_ref())?);
    match kind {
        ui::InputKind::Number => row(id, label, input, action, args),
        _ => row_on(id, label, input.commit(ui_text_value("blur")?), ui::Trigger::Commit, action, args),
    }
}

pub fn text_row(id: &str, label: &str, value: &str, action: &str, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    input_row(id, label, ui::InputKind::Text, value, action, args)
}

pub fn button(id: &str, label: &str, action: &str, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    tree_item_with_action(id, ui_label(label)?, None, forms_action(action, Some(args))?)
}

pub fn group(id: &str, label: &str, children: Vec<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    ui_admit(ui_admit(ui_admit(ui::tree_item(ui_label(label)?).try_id(id))?.try_children(children))?.try_build())
}

fn property_label<'a>(field: &str, labels: &'a FormsLabels) -> &'a str {
    match field {
        "kind" => labels.kind.as_str(), "label" => labels.label.as_str(), "description" => labels.description.as_str(),
        "required" => labels.required.as_str(), "placeholder" => labels.placeholder.as_str(),
        "default" => labels.default.as_str(), "min" => labels.min.as_str(), "max" => labels.max.as_str(),
        "step" => labels.step_field.as_str(), "unit" => labels.unit.as_str(), "schema" => labels.schema.as_str(),
        "text" => labels.text.as_str(), "src" => labels.src.as_str(), "accept" => labels.accept.as_str(),
        "fixtureSlug" => labels.fixture_slug.as_str(), _ => labels.label.as_str(),
    }
}

fn property_value(question: &FormQuestion, field: &str) -> String {
    match field {
        "label" => question.label.clone(), "description" => question.description.clone().unwrap_or_default(),
        "placeholder" => question.placeholder.clone().unwrap_or_default(),
        "default" => question.default.as_ref().map(crate::schema::dsl_string_value).unwrap_or_default(),
        "min" => question.min.map(|v| v.to_string()).unwrap_or_default(),
        "max" => question.max.map(|v| v.to_string()).unwrap_or_default(),
        "step" => question.step.map(|v| v.to_string()).unwrap_or_default(),
        "unit" => question.unit.clone().unwrap_or_default(), "schema" => question.schema.clone().unwrap_or_default(),
        "text" => question.text.clone().unwrap_or_default(), "src" => question.src.clone().unwrap_or_default(),
        "accept" => question.accept.clone().unwrap_or_default(), "fixtureSlug" => question.fixture_slug.clone().unwrap_or_default(),
        _ => String::new(),
    }
}

pub fn scalar_row(question: &FormQuestion, ids: &[String], field: &str, view: &semio_framework_plugin::ViewModel, config: &crate::editor::forms::config::FormsConfig) -> UiAssemblyResult<BuiltNode> {
    let labels = crate::editor::forms::terminology::forms_play_labels(view);
    let id = format!("forms-play-inspector.question.{field}");
    let label = property_label(field, labels);
    let args = arguments(vec![("field", ui_value_text(field)?), ("questionIds", question_ids(ids)?)])?;
    if field == "kind" {
        let contributions = crate::editor::forms::parse_contributions(config);
        let kinds = crate::editor::forms::catalogue_kinds(&contributions, view);
        let mut select = ui::select(ui_text_value(&question.kind)?);
        if !kinds.iter().any(|(kind, _, _)| kind == &question.kind) {
            select = ui_admit(select.try_item(ui_text_value(&question.kind)?, ui_label(&question.kind)?))?;
        }
        for (kind, label, _) in kinds { select = ui_admit(select.try_item(ui_text_value(kind)?, ui_label(label)?))?; }
        return row(&id, label, select, "patchQuestions", args);
    }
    if field == "required" || field == "default" && question.kind == "boolean" {
        let on = if field == "required" { question.required.unwrap_or(false) } else { question.default.as_ref().is_some_and(|value| *value == dsl::DslValue::Bool(true)) };
        return row(&id, label, ui::toggle(on).text(ui_label(label)?), "patchQuestions", args);
    }
    if field == "default" && question.kind == "multi" {
        let mut rows = Vec::new();
        for option in question.options.iter().flatten() {
            let selected = question.default.as_ref().is_some_and(|value| matches!(value, dsl::DslValue::Array(items) if items.contains(&dsl::DslValue::String(option.value.clone()))));
            rows.push(row(&format!("{id}.{}", option.value), &option.label, ui::toggle(selected).text(ui_label(&option.label)?), "patchQuestionOptions", arguments(vec![("field", ui_value_text("default")?), ("optionValue", ui_value_text(&option.value)?), ("questionIds", question_ids(ids)?)])?)?);
        }
        return group(&id, label, rows);
    }
    if field == "default" && question.kind == "single" {
        let mut select = ui_admit(ui::select(ui_text_value(property_value(question, field))?).try_item(ui_text_value("")?, ui_label(labels.unset.as_str())?))?;
        for option in question.options.iter().flatten() {
            select = ui_admit(select.try_item(ui_text_value(&option.value)?, ui_label(&option.label)?))?;
        }
        return row(&id, label, select, "patchQuestions", args);
    }
    let kind = match field {
        "min" | "max" | "step" => ui::InputKind::Number,
        "description" | "text" => ui::InputKind::LongText,
        "default" => match question.kind.as_str() {
            "number" | "slider" => ui::InputKind::Number, "date" => ui::InputKind::Date,
            "color" => ui::InputKind::Color, "longText" => ui::InputKind::LongText, _ => ui::InputKind::Text,
        },
        _ => ui::InputKind::Text,
    };
    input_row(&id, label, kind, property_value(question, field), "patchQuestions", args)
}

pub fn option_row(question: &FormQuestion, option: &crate::FormQuestionOption, labels: &FormsLabels) -> UiAssemblyResult<BuiltNode> {
    let id = format!("forms-play-inspector.option.{}", option.value);
    let ids = || question_ids(&[question.id.clone()]);
    group(&id, &option.value, vec![
        text_row(&format!("{id}.value"), labels.value.as_str(), &option.value, "patchQuestionOptions", arguments(vec![("field", ui_value_text("value")?), ("optionValue", ui_value_text(&option.value)?), ("questionIds", ids()?)])?)?,
        text_row(&format!("{id}.label"), labels.label.as_str(), &option.label, "patchQuestionOptions", arguments(vec![("field", ui_value_text("label")?), ("optionValue", ui_value_text(&option.value)?), ("questionIds", ids()?)])?)?,
        button(&format!("{id}.remove"), labels.remove_option.as_str(), "removeQuestionOption", arguments(vec![("optionValue", ui_value_text(&option.value)?), ("questionId", ui_value_text(&question.id)?)])?)?,
    ])
}

pub fn vector_row(question: &FormQuestion, field: &crate::FormVectorField, labels: &FormsLabels) -> UiAssemblyResult<BuiltNode> {
    let id = format!("forms-play-inspector.vector.{}", field.key);
    let args = |property: &str| arguments(vec![("field", ui_value_text(property)?), ("fieldKey", ui_value_text(&field.key)?), ("questionId", ui_value_text(&question.id)?)]);
    group(&id, &field.key, vec![
        text_row(&format!("{id}.label"), labels.label.as_str(), field.label.as_deref().unwrap_or(&field.key), "patchVectorField", args("label")?)?,
        input_row(&format!("{id}.value"), labels.default.as_str(), ui::InputKind::Number, field.value.map(|value| value.to_string()).unwrap_or_default(), "patchVectorField", args("value")?)?,
        button(&format!("{id}.remove"), labels.remove.as_str(), "removeVectorField", arguments(vec![("fieldKey", ui_value_text(&field.key)?), ("questionId", ui_value_text(&question.id)?)])?)?,
    ])
}
