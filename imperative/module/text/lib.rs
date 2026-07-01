//! 📝 Imperative text module: string action operators.

use neural_engine::{
    channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operation, OperatorImpl, OperatorInfo, Registry, Value,
};

// #region 🔖TextConcat
pub struct TextConcat;

impl Operation for TextConcat {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let left = read_string(input, "left")?;
        let right = read_string(input, "right")?;
        Ok(channel_output(
            "text",
            Dictionary::new().insert("value", Value::Atom(Atom::String(format!("{left}{right}")))),
        ))
    }
}
// #endregion 🔖TextConcat

// #region 🔖TextUppercase
pub struct TextUppercase;

impl Operation for TextUppercase {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let text = read_string(input, "text")?;
        Ok(channel_output(
            "text",
            Dictionary::new().insert("value", Value::Atom(Atom::String(text.to_uppercase()))),
        ))
    }
}
// #endregion 🔖TextUppercase

// #region 🔖TextLength
pub struct TextLength;

impl Operation for TextLength {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let text = read_string(input, "text")?;
        Ok(channel_output(
            "length",
            Dictionary::new().insert("value", Value::Atom(Atom::Decimal(text.chars().count() as f64))),
        ))
    }
}
// #endregion 🔖TextLength

// #region 🔖Helpers
fn read_string(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_str())
        .map(str::to_string)
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn string_channel(name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Str", name, name)
}

fn operator_info(id: &str, name: &str, abbreviation: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        module: "text".into(),
        name: name.into(),
        abbreviation: abbreviation.into(),
        icon: "emoji:📝".into(),
        summary: summary.into(),
        inputs,
        outputs,
        ..Default::default()
    }
}

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operation }], &[]);
}

pub fn register(registry: &mut Registry) {
    register_simple(
        registry,
        operator_info(
            "text.concat",
            "Text Concat",
            "Cat",
            "Concatenates two strings",
            vec![string_channel("left"), string_channel("right")],
            vec![ChannelSpec::named("T", "Txt", "text", "Text")],
        ),
        Box::new(TextConcat),
    );
    register_simple(
        registry,
        operator_info(
            "text.uppercase",
            "Text Uppercase",
            "Up",
            "Uppercases a string",
            vec![string_channel("text")],
            vec![ChannelSpec::named("T", "Txt", "text", "Text")],
        ),
        Box::new(TextUppercase),
    );
    register_simple(
        registry,
        operator_info(
            "text.length",
            "Text Length",
            "Len",
            "Returns the character length of a string",
            vec![string_channel("text")],
            vec![ChannelSpec::named("L", "Len", "length", "Length")],
        ),
        Box::new(TextLength),
    );
    registry.finalize();
}

pub fn catalogue_json(registry: &Registry) -> String {
    let items: Vec<serde_json::Value> = ["text.concat", "text.uppercase", "text.length"]
        .iter()
        .filter_map(|kind| registry.operator_info(kind))
        .map(|info| {
            serde_json::json!({
                "kind": info.id,
                "name": info.name,
                "abbreviation": info.abbreviation,
                "icon": info.icon,
                "summary": info.summary,
                "module": info.module,
                "inputs": info.inputs.iter().map(|channel| serde_json::json!({
                    "name": channel.name,
                    "code": channel.code,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    serde_json::to_string(&serde_json::json!({
        "schema": "imperative.catalogue/v1",
        "sections": [{
            "id": "text",
            "title": "Text",
            "items": items,
        }],
    }))
    .unwrap_or_else(|_| "{}".into())
}

pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖Helpers

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_concat_joins_strings() {
        let registry = module_registry();
        let input = Dictionary::new()
            .insert("left", Value::Atom(Atom::String("hello ".into())))
            .insert("right", Value::Atom(Atom::String("world".into())));
        let output = registry.dispatch("text.concat", &input).expect("dispatch");
        let value = output
            .get("text")
            .and_then(|v| v.as_dictionary())
            .and_then(|dict| dict.get("value"))
            .and_then(|v| v.as_atom())
            .and_then(|a| a.as_str());
        assert_eq!(value, Some("hello world"));
    }

    #[test]
    fn catalogue_json_lists_text_operators() {
        let registry = module_registry();
        let raw = catalogue_json(&registry);
        assert!(raw.contains("text.uppercase"));
        assert!(raw.contains("\"id\":\"text\""));
    }
}
