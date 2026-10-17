//! 🌉️ Forms action bridge: folds the shell `{action, args}` vocabulary (camelCase keys, block-list verbs, merged control values) into the typed `FormsCommand`.


//#region 🔖️ActionBridge
use super::*;
use semio_framework_plugin::{FaultCode, FaultOrigin};

fn snake(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 4);
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            out.push('_');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

pub(super) fn camel(key: &str) -> String {
    let mut out = String::with_capacity(key.len());
    let mut upper = false;
    for ch in key.chars() {
        if ch == '_' {
            upper = true;
        } else if upper {
            out.push(ch.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

fn put(entries: &mut Vec<(String, semio_framework_value::DslValue)>, key: &str, value: semio_framework_value::DslValue) {
    entries.retain(|(existing, _)| existing != key);
    entries.push((key.to_string(), value));
}

/// 🔢️ The host's JSON round trip turns every integer into `Number::Float` (`generation: 1` arrives
/// as `Float(1.0)`), and the `u64`/`i64` codecs decode EXACT integers only — restore the integer
/// variant for whole, finite floats so the payloads' counters (`generation`, `index`, `input_index`,
/// `vector_index`, …) decode; `f64` fields accept any `Number` variant, so nothing else changes.
fn integral(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    match value {
        semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
            if float >= 0.0 { semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
        }
        semio_framework_value::DslValue::Array(items) => semio_framework_value::DslValue::Array(items.into_iter().map(integral).collect()),
        semio_framework_value::DslValue::Object(entries) => semio_framework_value::DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
        other => other,
    }
}

/// 🔁️ Emits every key of `args` under BOTH spellings (the Try-window payloads decode camelCase
/// wire names, the document payloads snake_case; `FromValue` ignores keys it does not know),
/// applies `aliases` (snake_case source → destination), and prints `json` sources (a snake_case
/// key holding any JSON value) into their string destination when that field is absent.
pub(super) fn fold(args: Option<&semio_framework_value::DslValue>, aliases: &[(&str, &str)], json: &[(&str, &str)]) -> semio_framework_value::DslValue {
    let mut entries: Vec<(String, semio_framework_value::DslValue)> = Vec::new();
    if let Some(semio_framework_value::DslValue::Object(object)) = args {
        for (key, value) in object {
            let mut key = snake(key);
            if let Some((_, to)) = aliases.iter().find(|(from, _)| *from == key) {
                key = (*to).to_string();
            }
            let value = integral(value.clone());
            put(&mut entries, &camel(&key), value.clone());
            put(&mut entries, &key, value);
        }
    }
    for (from, into) in json {
        if entries.iter().any(|(key, _)| key == into) {
            continue;
        }
        if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
            let text = semio_framework_value::DslValue::String(semio_framework_pack_json::to_json_string(&value));
            put(&mut entries, &camel(into), text.clone());
            put(&mut entries, into, text);
        }
    }
    semio_framework_value::DslValue::Object(entries)
}

pub(super) fn decode<T: semio_framework_value::FromValue>(action: &str, value: semio_framework_value::DslValue) -> Result<T, Fault> {
    T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("forms action '{action}' arguments do not decode: {error}")))
}

pub fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<FormsCommand, Fault> {
    const VALUE_JSON: &[(&str, &str)] = &[("value", "value_json")];
    let plain = || fold(args, &[], &[]);
    Ok(match action {
        "setTryValue" => FormsCommand::SetTryValue(decode(action, fold(args, &[], VALUE_JSON))?),
        set_try_value::SET_TRY_VALUE_STEP_ACTION_ID => FormsCommand::SetTryValueStep(decode(action, fold(args, &[], VALUE_JSON))?),
        "setTryValues" => FormsCommand::SetTryValues(decode(action, fold(args, &[], &[("values", "values_json"), ("value", "values_json")]))?),
        "resetTry" => FormsCommand::ResetTry(decode(action, plain())?),
        "previousStep" => FormsCommand::PreviousStep(decode(action, plain())?),
        "nextStep" => FormsCommand::NextStep(decode(action, plain())?),
        "submit" => FormsCommand::Submit(decode(action, plain())?),
        "setContributions" => FormsCommand::SetContributions(decode(action, plain())?),
        "addStep" => FormsCommand::AddStep(decode(action, plain())?),
        "patchStep" => FormsCommand::PatchStep(decode(action, plain())?),
        "removeStep" => FormsCommand::RemoveStep(decode(action, plain())?),
        "moveStep" => FormsCommand::MoveStep(decode(action, plain())?),
        "updateForm" => FormsCommand::UpdateForm(decode(action, fold(args, &[("value", "title")], &[]))?),
        "addBlock" => FormsCommand::AddQuestion(decode(action, plain())?),
        "removeBlock" => FormsCommand::RemoveQuestion(decode(action, fold(args, &[("block_id", "question_id")], &[]))?),
        "patchQuestions" => FormsCommand::PatchQuestions(decode(action, fold(args, &[], VALUE_JSON))?),
        "patchQuestionOptions" => FormsCommand::PatchQuestionOptions(decode(action, fold(args, &[], VALUE_JSON))?),
        "addQuestionOption" => FormsCommand::AddQuestionOption(decode(action, plain())?),
        "removeQuestionOption" => FormsCommand::RemoveQuestionOption(decode(action, plain())?),
        "patchVectorField" => FormsCommand::PatchVectorField(decode(action, fold(args, &[], VALUE_JSON))?),
        "addVectorField" => FormsCommand::AddVectorField(decode(action, plain())?),
        "removeVectorField" => FormsCommand::RemoveVectorField(decode(action, plain())?),
        "moveBlock" => {
            // 🧱️ The block-list host sends `{blockId, fromStepId, toStepId, index}`; `position` is
            // only consulted when `index` is absent, so it defaults to "after".
            let mut folded = fold(args, &[("block_id", "question_id")], &[]);
            if let semio_framework_value::DslValue::Object(entries) = &mut folded {
                if !entries.iter().any(|(key, _)| key == "position") {
                    entries.push(("position".into(), semio_framework_value::DslValue::String("after".into())));
                }
            }
            FormsCommand::MoveQuestion(decode(action, folded)?)
        }
        "dropQuestionKind" => FormsCommand::DropQuestionKind(decode(action, fold(args, &[("block_id", "target_id"), ("position", "drop_position")], &[]))?),
        "setSpecJson" => FormsCommand::SetSpecJson(decode(action, fold(args, &[], &[("document", "json"), ("value", "json")]))?),
        "setActiveExample" => FormsCommand::SetActiveExample(decode(action, fold(args, &[("id", "example_id"), ("value", "example_id")], &[]))?),
        "exportSnapshot" => FormsCommand::ExportSnapshot(decode(action, plain())?),
        "exportResponses" => FormsCommand::ExportResponses(decode(action, plain())?),
        "discardResponse" => FormsCommand::DiscardResponse(decode(action, plain())?),
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the forms editor has no command for action '{action}'"))),
    })
}
//#endregion 🔖️ActionBridge
