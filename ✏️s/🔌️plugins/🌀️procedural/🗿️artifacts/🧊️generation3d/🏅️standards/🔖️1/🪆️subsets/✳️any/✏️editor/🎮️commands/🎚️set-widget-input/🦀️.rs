//! 🎚️ Typed operator inputs and source text are editable through the document's event history: a committed field is ONE
//! absolute `change-widget-input` leaf (design §19), so a history edit changes exactly that typed value.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{change_widget_input, WidgetInputValue};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use dsl::ToValue as _;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-widget-input")]
pub struct SetWidgetInput {
    pub widget_id: String,
    pub channel: String,
    pub value: String,
    pub component: Option<String>,
}

/// 🧬️ Preserves the declared schema while changing one literal or coordinate.
pub(crate) fn edit_input_value(types: &[String], current: Option<&dsl::DslValue>, text: &str, component: Option<&str>) -> Result<dsl::DslValue, String> {
    let schema = current.and_then(|value| value.get("$schema")).and_then(dsl::DslValue::as_str).or_else(|| types.first().map(String::as_str)).ok_or("Connect a compatible output to this input")?;
    if !types.iter().any(|kind| kind == schema) || !matches!(schema, "number" | "text" | "boolean" | "point" | "vector") { return Err("Connect a compatible output to this input".into()); }
    let mut fields = match current { Some(dsl::DslValue::Object(fields)) => fields.clone(), _ => vec![("$schema".into(), dsl::DslValue::String(schema.into()))] };
    let number = || text.trim().parse::<f64>().ok().filter(|number| number.is_finite()).map(dsl::DslValue::float).ok_or_else(|| "Input must be a finite number".to_string());
    let (field, value) = match schema {
        "point" | "vector" => {
            let axis = component.filter(|axis| matches!(*axis, "x" | "y" | "z")).ok_or("Choose a coordinate to edit")?;
            for coordinate in ["x", "y", "z"] { if !fields.iter().any(|(name, _)| name == coordinate) { fields.push((coordinate.into(), dsl::DslValue::float(0.0))); } }
            (axis, number()?)
        }
        _ if component.is_some() => return Err("Scalar input has no coordinates".into()),
        "number" => ("value", number()?),
        "text" => ("value", dsl::DslValue::String(text.into())),
        "boolean" => ("value", dsl::DslValue::Bool(match text { "true" => true, "false" => false, _ => return Err("Boolean input must be true or false".into()) })),
        _ => return Err("Unsupported input schema".into()),
    };
    if let Some((_, current)) = fields.iter_mut().find(|(name, _)| name == field) { *current = value; } else { fields.push((field.into(), value)); }
    Ok(dsl::DslValue::Object(fields))
}

/// ✍️ The ONE `change-widget-input` leaf a committed field edit is, validated against the operator's declared port before
/// any document mutation (an unconnected scalar input of a declared literal type); a text source's `text` is a text
/// input. `None` when the input already holds the edited value — an unchanged field is no edit.
pub(crate) fn input_leaf(host_snapshot: &FlowHostSnapshot, payload: &SetWidgetInput) -> Result<Option<Generation3dMutation>, String> {
    if payload.value.len() > 1_048_576 { return Err("Input text exceeds 1 MiB".into()); }
    let widget = host_snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == payload.widget_id).ok_or("The selected widget no longer exists")?;
    if let Widget::InputNote { text, .. } = widget {
        if payload.channel != "text" || payload.component.is_some() { return Err("Select an operator input or a text source".into()); }
        return Ok((*text != payload.value).then(|| change_widget_input(&payload.widget_id, "text", WidgetInputValue::Text(payload.value.clone()))));
    }
    let Widget::Neuron { neuron_kind, params, .. } = widget else { return Err("Select an operator input or a text source".into()) };
    if host_snapshot.synapses.iter().any(|synapse| synapse.to == payload.widget_id && synapse.to_port == payload.channel) { return Err("This input is connected; edit its source or disconnect it first".into()); }
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let port = infos.get(neuron_kind).and_then(|info| info.inputs.iter().find(|input| input.name == payload.channel)).ok_or("The selected operator input is unavailable")?;
    if port.cardinality.is_collection() { return Err("Connect a collection output to this input".into()); }
    let stored = params.get(&payload.channel).map(|value| value.to_value());
    let current = stored.clone().or_else(|| port.default.as_ref().map(|value| value.to_value()));
    let edited = edit_input_value(&port.value_types, current.as_ref(), &payload.value, payload.component.as_deref())?;
    let input = WidgetInputValue::of_literal(&edited).ok_or("Unsupported input schema")?;
    Ok((stored.as_ref().and_then(WidgetInputValue::of_literal).as_ref() != Some(&input)).then(|| change_widget_input(&payload.widget_id, &payload.channel, input)))
}

pub fn handle(payload: &SetWidgetInput, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::mutations(input_leaf(&doc.snapshot.host_snapshot, payload).map_err(Fault::from)?.into_iter().collect()))
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
