
/// 🗝️ The named arguments an action's JSON input carries, as the framework's own values: the members of an object, none for a null input, a named refusal for anything else. The one place a channel implementation meets the JSON type of [`AppCommand::PureCommand`].
pub fn action_input_arguments(input: &serde_json::Value) -> Result<std::collections::BTreeMap<String, DslValue>, String> {
    match input {
        serde_json::Value::Object(map) => Ok(map.iter().map(|(key, value)| (key.clone(), json_value_to_dsl(value))).collect()),
        serde_json::Value::Null => Ok(std::collections::BTreeMap::new()),
        other => Err(format!("an action's input is a named argument object; this call carried {other}")),
    }
}
