//! 📝️ Direct `change-schema` text payload codec and aggregate wire bridge.

use super::super::PlaygroundMutation;
use super::ChangeSchema;

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

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
        let arguments = parse_arguments(rest).map_err(|error| store::TextError::new(error, store::TextSpan::at(1, 1)))?;
        let argument = |key: &str| arguments.get(key).cloned().ok_or_else(|| store::TextError::new(format!("playground mutation: missing arg '{key}' for '{keyword}'"), store::TextSpan::at(1, 1)));
        match keyword {
            TEXT_OPCODE => Ok(PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: decode_string(&argument("new-schema")?).map_err(|error| store::TextError::new(error, store::TextSpan::at(1, 1)))? })),
            other => Err(store::TextError::new(format!("playground mutation: unknown keyword {other:?}"), store::TextSpan::at(1, 1))),
        }
    }
}
//#endregion 🔖️OpText

//#region 🧪️RoundTrip
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_wire_form_round_trips() {
        let operation = PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: "playground custom".into() });
        store::os_store::test_support::assert_op_line_round_trip(&operation);
    }
}
//#endregion 🧪️RoundTrip
