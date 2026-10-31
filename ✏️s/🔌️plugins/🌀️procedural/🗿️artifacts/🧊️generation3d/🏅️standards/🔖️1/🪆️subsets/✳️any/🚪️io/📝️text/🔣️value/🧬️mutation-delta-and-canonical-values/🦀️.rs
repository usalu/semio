//! 🧬️ Mutation, delta and canonical values of generation3d: the mutation enum with its payload records, the widget, synapse and generation deltas, the typed widget input literals and the canonical JSON trees. See [`super`].

use semio_framework_value_derive::{FromValue, ToValue};
use super::*;


























/// 🧩️ The typed literal an operator param holds: `{"$schema", value}`, or `{"$schema", x, y, z}` for a point or a
/// vector.
pub fn widget_input_literal(input: &WidgetInputValue) -> semio_framework_value::DslValue {
    if let Some(items) = input.items() {
        return semio_framework_value::DslValue::Object(std::iter::once(("$schema".into(), semio_framework_value::DslValue::String("list".into()))).chain(items.iter().enumerate().map(|(index, item)| (index.to_string(), widget_input_literal(item)))).collect());
    }
    let schema = ("$schema".to_string(), semio_framework_value::DslValue::String(input.schema().into()));
    let scalar = |value| semio_framework_value::DslValue::Object(vec![schema.clone(), ("value".to_string(), value)]);
    match input {
        WidgetInputValue::Number(value) => scalar(semio_framework_value::DslValue::float(*value)),
        WidgetInputValue::Text(value) => scalar(semio_framework_value::DslValue::String(value.clone())),
        WidgetInputValue::Boolean(value) => scalar(semio_framework_value::DslValue::Bool(*value)),
        WidgetInputValue::Point(axes) | WidgetInputValue::Vector(axes) => semio_framework_value::DslValue::Object(std::iter::once(schema.clone()).chain(["x", "y", "z"].iter().zip(axes).map(|(axis, value)| (axis.to_string(), semio_framework_value::DslValue::float(*value)))).collect()),
        WidgetInputValue::Plane(WidgetInputPlane { origin, normal }) => semio_framework_value::DslValue::Object(vec![schema.clone(), ("origin".to_string(), widget_input_literal(&WidgetInputValue::Point(*origin))), ("normal".to_string(), widget_input_literal(&WidgetInputValue::Vector(*normal)))]),
        _ => unreachable!(),
    }
}

/// 🔎️ The typed input a param literal states, or `None` for a literal of another shape.
pub fn widget_input_of_literal(literal: &semio_framework_value::DslValue) -> Option<WidgetInputValue> {
    if literal.get("$schema").and_then(semio_framework_value::DslValue::as_str) == Some("list") {
        return widget_input_of_literal_as(literal, literal.get("0")?.get("$schema")?.as_str()?);
    }
    let number = |key: &str| literal.get(key).and_then(semio_framework_value::DslValue::as_f64);
    let axes = || Some([number("x")?, number("y")?, number("z")?]);
    Some(match literal.get("$schema").and_then(semio_framework_value::DslValue::as_str)? {
        "number" => WidgetInputValue::Number(number("value")?),
        "text" => WidgetInputValue::Text(literal.get("value")?.as_str()?.to_string()),
        "boolean" => WidgetInputValue::Boolean(literal.get("value")?.as_bool()?),
        "point" => WidgetInputValue::Point(axes()?),
        "vector" => WidgetInputValue::Vector(axes()?),
        "plane" => match (widget_input_of_literal(literal.get("origin")?)?, widget_input_of_literal(literal.get("normal")?)?) {
            (WidgetInputValue::Point(origin), WidgetInputValue::Vector(normal)) => WidgetInputValue::Plane(WidgetInputPlane { origin, normal }),
            _ => return None,
        },
        _ => return None,
    })
}

/// 🏷️ Reads an existing neural list dictionary using its declared element type, including an empty list.
pub fn widget_input_of_literal_as(literal: &semio_framework_value::DslValue, schema: &str) -> Option<WidgetInputValue> {
    if literal.get("$schema").and_then(semio_framework_value::DslValue::as_str) != Some("list") { return widget_input_of_literal(literal); }
    let fields = literal.as_object()?;
    if fields.iter().any(|(key, _)| key != "$schema" && key.parse::<usize>().ok().is_none_or(|index| index.to_string() != *key)) { return None; }
    let count = fields.len().checked_sub(1)?;
    let items = (0..count).map(|index| widget_input_of_literal(literal.get(&index.to_string())?)).collect::<Option<Vec<_>>>()?;
    WidgetInputValue::collection(schema, items)
}
