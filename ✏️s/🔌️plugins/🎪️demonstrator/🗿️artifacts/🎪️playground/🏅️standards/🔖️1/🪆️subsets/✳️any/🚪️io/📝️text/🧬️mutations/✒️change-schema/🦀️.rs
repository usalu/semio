//! 📝️ Direct `change-schema` text payload codec and aggregate wire bridge.

use crate::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;
use crate::standards::v1::subsets::any::schema::mutations::ChangeSchema;

/// 🏷️ Stable text opcode for `ChangeSchema`.
pub const TEXT_OPCODE: &str = "change-schema";

//#region 🔖️ScalarCodec
fn encode_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn decode_string(value: &str) -> Result<String, String> {
    let inner = value.strip_prefix('"').and_then(|value| value.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {value:?}"))?;
    let mut output = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        match chars.next() {
            Some('\\') => output.push('\\'),
            Some('"') => output.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(output)
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
fn tokenize_arguments(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(character) = chars.next() {
        match character {
            '"' => {
                current.push(character);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(character);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(character),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn parse_arguments(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_arguments(rest).into_iter().map(|token| token.split_once('=').map(|(key, value)| (key.to_string(), value.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️OpText
impl protocol::OpText for PlaygroundMutation {
    fn print_op(&self) -> String {
        match self {
            PlaygroundMutation::ChangeSchema(payload) => format!("{TEXT_OPCODE} new-schema={}", encode_string(&payload.new_schema)),
        }
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
        let arguments = parse_arguments(rest).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let argument = |key: &str| arguments.get(key).cloned().ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("playground mutation: missing arg '{key}' for '{keyword}'"), semio_framework_diagnostic::TextSpan::at(1, 1)));
        match keyword {
            TEXT_OPCODE => Ok(PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: decode_string(&argument("new-schema")?).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))? })),
            other => Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("playground mutation: unknown keyword {other:?}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
        }
    }
}
//#endregion 🔖️OpText

//#region 🧪️RoundTrip
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️RoundTrip

mod json_orchestration {
use crate::standards::v1::subsets::any::schema::{diff::PlaygroundDiff, mutations::PlaygroundMutation, snapshot::PlaygroundSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::change_schema::bridge_step;
use semio_framework_pack_json::{array, from_dsl_value, from_json_str, object, to_string, Value};
fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(PlaygroundSnapshot, PlaygroundMutation), String> {
    let snapshot = from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed playground snapshot JSON does not decode: {error}"))?;
    let mutation = from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed playground mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}
fn bridge_render(snapshot: &PlaygroundSnapshot, messages: Vec<String>) -> String {
    let value = object([("snapshot".to_string(), from_dsl_value(&semio_framework_value::ToValue::to_value(snapshot))), ("messages".to_string(), array(messages.into_iter().map(Value::String)))]);
    to_string(&value)
}
/// 🌉️ Applies one committed language-neutral mutation payload to a playground snapshot.
pub fn apply_playground_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (applied, messages) = bridge_step(&snapshot, &mutation)?;
    Ok(bridge_render(&applied, messages))
}
/// ↩️ Applies one mutation and every step of its inverse plan.
pub fn undo_playground_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    use protocol::Mutation;
    let (base, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (mut current, mut messages) = bridge_step(&base, &mutation)?;
    for undo in <PlaygroundMutation as Mutation<PlaygroundSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)? {
        let (next, raised) = bridge_step(&current, &undo)?;
        current = next;
        messages.extend(raised);
    }
    Ok(bridge_render(&current, messages))
}
}
pub use json_orchestration::{apply_playground_mutation_json,undo_playground_mutation_json};
