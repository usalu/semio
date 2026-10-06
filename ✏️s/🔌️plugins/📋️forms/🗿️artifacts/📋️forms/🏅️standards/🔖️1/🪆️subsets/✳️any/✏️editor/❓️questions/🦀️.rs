//! ❓️ Shared question construction for form authoring commands.

#[path = "🫥️visibility/🦀️.rs"]
pub mod visibility;
#[path = "📍️placement/🦀️.rs"]
pub mod placement;
#[path = "🧩️extensions/🦀️.rs"]
pub mod extensions;

use crate::{FormQuestion, FormVectorField};
use crate::schema::value_to_dsl;
use semio_framework_pack_json::Value;

/// ✏️ Validates one field edit before the command emits a document event.
pub fn patch_question(question: &FormQuestion, field: &str, value: &Value) -> Result<FormQuestion, String> {
    let mut next = question.clone();
    let text = || value.as_str().map(str::to_owned).ok_or_else(|| "invalid-value".to_string());
    let optional_text = || if value.is_null() { Ok(None) } else { text().map(Some) };
    let number = || if value.is_null() || value.as_str() == Some("") { Ok(None) } else { value.as_f64().filter(|number| number.is_finite()).map(Some).ok_or_else(|| "invalid-value".to_string()) };
    match field {
        "label" => next.label = text()?,
        "kind" => {
            let kind = text()?;
            if kind.trim().is_empty() { return Err("invalid-value".into()); }
            if kind != question.kind {
                next = default_question_for_kind(&kind, question.id.clone());
                next.label = question.label.clone();
                next.description = question.description.clone();
                next.required = question.required;
                next.condition = question.condition.clone();
            }
        }
        "description" => next.description = optional_text()?,
        "placeholder" => next.placeholder = optional_text()?,
        "text" => next.text = optional_text()?,
        "unit" => next.unit = optional_text()?,
        "schema" => next.schema = optional_text()?,
        "src" => next.src = optional_text()?,
        "accept" => next.accept = optional_text()?,
        "exampleId" => next.example_id = optional_text()?,
        "required" => next.required = Some(value.as_bool().ok_or("invalid-value")?),
        "min" => next.min = number()?,
        "max" => next.max = number()?,
        "step" => next.step = number()?,
        "default" => {
            if !value.is_null() {
                let valid = match next.kind.as_str() {
                    "number" | "slider" => value.as_f64().is_some_and(f64::is_finite),
                    "boolean" => value.as_bool().is_some(),
                    "text" | "longText" | "date" | "color" | "single" => value.as_str().is_some(),
                    "multi" => value.as_array().is_some_and(|items| items.iter().all(|item| item.as_str().is_some())),
                    _ => true,
                };
                if !valid { return Err("invalid-value".into()); }
            }
            next.default = (!value.is_null()).then(|| value_to_dsl(value));
        }
        "condition" => next.condition = if value.is_null() { None } else { Some(semio_framework_value::FromValue::from_value(value_to_dsl(value)).map_err(|_| "invalid-condition")?) },
        "params" => {
            if !value.is_null() && value.as_object().is_none() { return Err("invalid-value".into()); }
            next.params = (!value.is_null()).then(|| value_to_dsl(value));
        }
        _ => return Err("unknown-field".into()),
    }
    if next.min.zip(next.max).is_some_and(|(min, max)| min > max) || next.step.is_some_and(|step| step <= 0.0) { return Err("invalid-range".into()); }
    Ok(next)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️patches/🦀️.rs"]
mod patch_tests;

/// 🌱️ A blank question of the given `kind`/`id` — every field defaulted to `None`.
pub fn question_shell(id: String, label: String, kind: String) -> FormQuestion {
    FormQuestion {
        id,
        label,
        kind,
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        example_id: None,
        params: None,
        condition: None,
    }
}

/// 🌱️ A freshly created question, seeded with sensible per-kind defaults — shared by `addQuestion` and
/// `dropQuestionKind`.
pub fn default_question_for_kind(kind: &str, id: String) -> FormQuestion {
    match kind {
        "text" => {
            let mut question = question_shell(id, "Text".into(), "text".into());
            question.placeholder = Some("Enter text".into());
            question
        }
        "longText" => {
            let mut question = question_shell(id, "Long Text".into(), "longText".into());
            question.placeholder = Some("Enter long text".into());
            question
        }
        "number" => {
            let mut question = question_shell(id, "Number".into(), "number".into());
            question.default = Some(value_to_dsl(&Value::from(0)));
            question.min = Some(0.0);
            question.max = Some(100.0);
            question.step = Some(1.0);
            question
        }
        "slider" => {
            let mut question = question_shell(id, "Slider".into(), "slider".into());
            question.default = Some(value_to_dsl(&Value::from(50)));
            question.min = Some(0.0);
            question.max = Some(100.0);
            question.step = Some(1.0);
            question
        }
        "boolean" => {
            let mut question = question_shell(id, "Boolean".into(), "boolean".into());
            question.default = Some(value_to_dsl(&Value::from(false)));
            question
        }
        "single" | "multi" => {
            let mut question = question_shell(id, if kind == "single" { "Single Select" } else { "Multi Select" }.into(), kind.into());
            question.default = if kind == "multi" { Some(value_to_dsl(&Value::Array(vec![]))) } else { None };
            question.options = Some(vec![crate::FormQuestionOption { value: "a".into(), label: "Option A".into() }, crate::FormQuestionOption { value: "b".into(), label: "Option B".into() }]);
            question
        }
        "note" => {
            let mut question = question_shell(id, "Note".into(), "note".into());
            question.text = Some("Informational note".into());
            question
        }
        "date" => question_shell(id, "Date".into(), "date".into()),
        "color" => {
            let mut question = question_shell(id, "Color".into(), "color".into());
            question.default = Some(value_to_dsl(&Value::from("#336699")));
            question
        }
        "image" => question_shell(id, "Image".into(), "image".into()),
        "file" => question_shell(id, "File".into(), "file".into()),
        "vector" => {
            let mut question = question_shell(id, "Vector".into(), "vector".into());
            question.schema = Some("vec3".into());
            question.step = Some(0.1);
            question.fields = Some(vec![
                FormVectorField { key: "x".into(), label: Some("X".into()), value: Some(0.0) },
                FormVectorField { key: "y".into(), label: Some("Y".into()), value: Some(0.0) },
                FormVectorField { key: "z".into(), label: Some("Z".into()), value: Some(0.0) },
            ]);
            question
        }
        _ => question_shell(id, kind.into(), kind.into()),
    }
}

/// 🔘️ Edits a choice and its selected defaults as one atomic document change.
pub fn patch_choice(question: &FormQuestion, option_value: &str, field: &str, value: &Value) -> Result<FormQuestion, String> {
    let mut next = question.clone();
    let Some(options) = next.options.as_mut() else { return Ok(next); };
    let Some(index) = options.iter().position(|option| option.value == option_value) else { return Ok(next); };
    let replacement = match field {
        "label" => { options[index].label = value.as_str().ok_or("invalid-value")?.to_owned(); return Ok(next); }
        "value" => {
            let name = value.as_str().filter(|name| !name.trim().is_empty()).ok_or("invalid-value")?;
            if options.iter().enumerate().any(|(other, option)| other != index && option.value == name) { return Err("duplicate-option".into()); }
            options[index].value = name.to_owned();
            Some(name)
        }
        "remove" => { options.remove(index); None }
        "default" => {
            if next.kind != "multi" { return Err("invalid-value".into()); }
            let enabled = value.as_bool().ok_or("invalid-value")?;
            let mut selected: std::collections::HashSet<String> = match &next.default {
                Some(semio_framework_value::DslValue::Array(items)) => items.iter().filter_map(|item| match item { semio_framework_value::DslValue::String(value) => Some(value.clone()), _ => None }).collect(),
                _ => Default::default(),
            };
            if enabled { selected.insert(option_value.to_owned()); } else { selected.remove(option_value); }
            next.default = Some(semio_framework_value::DslValue::Array(options.iter().filter(|option| selected.contains(&option.value)).map(|option| semio_framework_value::DslValue::String(option.value.clone())).collect()));
            return Ok(next);
        }
        _ => return Err("unknown-field".into()),
    };
    match (&next.kind[..], &mut next.default) {
        ("single", Some(semio_framework_value::DslValue::String(selected))) if selected == option_value => next.default = replacement.map(|value| semio_framework_value::DslValue::String(value.into())),
        ("multi", Some(semio_framework_value::DslValue::Array(selected))) => {
            *selected = selected.iter().filter_map(|item| match item {
                semio_framework_value::DslValue::String(value) if value == option_value => replacement.map(|value| semio_framework_value::DslValue::String(value.into())),
                _ => Some(item.clone()),
            }).collect();
        }
        _ => {}
    }
    Ok(next)
}
