//! ✅️ First-party answer validation for authoring previews and durable submissions.
use crate::{FormQuestion, FormStep};
use semio_framework_value::DslValue;
use crate::playbook::PlaybookValues;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct FormsAnswerError {
    pub question_id: String,
    pub code: String,
}

fn empty(value: &DslValue) -> bool {
    match value {
        semio_framework_value::DslValue::Null => true,
        semio_framework_value::DslValue::String(text) => text.trim().is_empty(),
        semio_framework_value::DslValue::Array(values) => values.is_empty(),
        semio_framework_value::DslValue::Object(values) => values.is_empty(),
        _ => false,
    }
}

fn valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' || bytes.iter().enumerate().any(|(index, byte)| index != 4 && index != 7 && !byte.is_ascii_digit()) { return false; }
    let year = value[..4].parse::<u32>().unwrap_or(0);
    let month = value[5..7].parse::<usize>().unwrap_or(0);
    let day = value[8..].parse::<u32>().unwrap_or(0);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    month > 0 && month <= 12 && day > 0 && day <= days[month - 1]
}

pub fn answer_error(question: &FormQuestion, value: &DslValue) -> Option<&'static str> {
    if matches!(question.kind.as_str(), "note" | "image") { return None; }
    if empty(value) { return question.required.unwrap_or(false).then_some("required"); }
    match question.kind.as_str() {
        "text" | "longText" | "file" => (!matches!(value, DslValue::String(_))).then_some("type"),
        "boolean" => (!matches!(value, DslValue::Bool(_))).then_some("type"),
        "number" | "slider" => {
            let Some(number) = value.as_f64().filter(|number| number.is_finite()) else { return Some("type"); };
            if question.min.is_some_and(|min| number < min) || question.max.is_some_and(|max| number > max) { return Some("range"); }
            if let Some(step) = question.step {
                let ticks = (number - question.min.unwrap_or(0.0)) / step;
                if !ticks.is_finite() || (ticks - ticks.round()).abs() > 1e-9 { return Some("step"); }
            }
            None
        }
        "single" => {
            let semio_framework_value::DslValue::String(text) = value else { return Some("type"); };
            (!question.options.iter().flatten().any(|option| &option.value == text)).then_some("option")
        }
        "multi" => {
            let semio_framework_value::DslValue::Array(items) = value else { return Some("type"); };
            let mut selected = std::collections::HashSet::new();
            items.iter().any(|item| match item {
                semio_framework_value::DslValue::String(text) => !selected.insert(text) || !question.options.iter().flatten().any(|option| &option.value == text),
                _ => true,
            }).then_some("option")
        }
        "date" => match value { semio_framework_value::DslValue::String(text) => (!valid_date(text)).then_some("date"), _ => Some("type") },
        "color" => match value {
            semio_framework_value::DslValue::String(text) => (!(text.len() == 7 && text.starts_with('#') && text.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit))).then_some("color"),
            _ => Some("type"),
        },
        "vector" => match value {
            semio_framework_value::DslValue::Array(items) => (items.len() != question.fields.as_ref().map_or(0, Vec::len) || items.iter().any(|item| item.as_f64().is_none_or(|number| !number.is_finite()))).then_some("vector"),
            _ => Some("vector"),
        },
        _ => (!matches!(value, DslValue::Object(_))).then_some("type"),
    }
}

pub fn step_errors(step: &FormStep, values: &PlaybookValues) -> Vec<FormsAnswerError> {
    crate::schema::visible_questions(step, values).into_iter().filter_map(|question| answer_error(question, values.get(&question.id).unwrap_or(&semio_framework_value::DslValue::Null)).map(|code| FormsAnswerError { question_id: question.id.clone(), code: code.into() })).collect()
}

pub fn can_advance(step: &FormStep, values: &PlaybookValues) -> bool {
    step_errors(step, values).is_empty()
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
