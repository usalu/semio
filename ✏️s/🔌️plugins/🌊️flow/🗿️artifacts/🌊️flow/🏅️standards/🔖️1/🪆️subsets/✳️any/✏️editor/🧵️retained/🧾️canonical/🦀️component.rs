//! 🧾️ Typed borrowed Flow JSON in exact serde declaration order, with retained native map iterators.

use super::{neural, FlowGui, FlowLayoutEntry, FlowMutation, FlowPreviewGui, NodeChrome, Widget};
use store::{ArtifactCanonicalJson, ArtifactCanonicalJsonArray as Array, ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonObject as Object, ArtifactCanonicalJsonValue as Value};

//#region 🔣️Scalars
fn text(value: &str) -> Value<'_> { Value::Scalar(Node::String(value)) }
fn number(value: f64) -> Value<'static> { Value::Scalar(Node::F64(value)) }
fn index(value: usize) -> Value<'static> { Value::Scalar(Node::U64(value as u64)) }
fn boolean(value: bool) -> Value<'static> { Value::Scalar(Node::Bool(value)) }
fn null() -> Value<'static> { Value::Scalar(Node::Null) }
fn object<'a, const N: usize>(fields: [(&'a str, Value<'a>); N]) -> Value<'a> { Value::Object(Object::new(fields.into_iter())) }
fn array<'a>(values: impl Iterator<Item = Value<'a>> + Send + 'a) -> Value<'a> { Value::Array(Array::new(values)) }
fn strings(values: &[String]) -> Value<'_> { array(values.iter().map(|value| text(value))) }
fn set(values: &flow::OrderedSet) -> Value<'_> { array(values.iter().map(|value| text(value))) }
fn layout(value: &flow::WidgetLayout) -> Value<'_> { object([("x", number(value.x)), ("y", number(value.y))]) }
fn optional_layout(value: &Option<flow::WidgetLayout>) -> Value<'_> { value.as_ref().map(layout).unwrap_or_else(null) }
//#endregion 🔣️Scalars

//#region 🧠️Neural
fn dictionary(value: &neural::Dictionary) -> Value<'_> { Value::Object(Object::new(value.iter().map(|(key, value)| (key.as_str(), neural_value(value))))) }

fn neural_value(value: &neural::Value) -> Value<'_> {
    match value {
        neural::Value::Dictionary(value) => dictionary(value),
        neural::Value::Atom(value) => match value {
            neural::Atom::Null => null(), neural::Atom::Boolean(value) => boolean(*value),
            neural::Atom::Integer(value) => Value::Scalar(Node::I64(*value)), neural::Atom::Decimal(value) => number(*value),
            neural::Atom::String(value) => text(value),
        },
    }
}

fn tree(value: &neural::Tree) -> Value<'_> {
    object([("neurons", array(value.neurons.iter().map(neuron))), ("synapses", array(value.synapses.iter().map(synapse)))])
}

fn neuron(value: &neural::Neuron) -> Value<'_> {
    Value::Object(Object::new([
        Some(("id", text(&value.id))), Some(("kind", text(&value.kind))), Some(("params", dictionary(&value.params))),
        value.tree.as_ref().map(|value| ("tree", tree(value))),
    ].into_iter().flatten()))
}

fn synapse(value: &neural::Synapse) -> Value<'_> {
    object([("id", text(&value.id)), ("from", text(&value.from)), ("to", text(&value.to)), ("fromPort", text(&value.from_port)), ("toPort", text(&value.to_port))])
}
//#endregion 🧠️Neural

//#region 🪟️Presentation
fn chrome(value: &NodeChrome) -> Value<'_> {
    match value {
        NodeChrome::Plain { preview } => object([("kind", text("plain")), ("preview", boolean(*preview))]),
        NodeChrome::Slider { label, min, max, step, value } => object([("kind", text("slider")), ("label", text(label)), ("min", number(*min)), ("max", number(*max)), ("step", number(*step)), ("value", number(*value))]),
        NodeChrome::Note { text: value } => object([("kind", text("note")), ("text", text(value))]),
        NodeChrome::Image { src } => object([("kind", text("image")), ("src", text(src))]),
        NodeChrome::Variable { name, schema } => object([("kind", text("variable")), ("name", text(name)), ("schema", text(schema))]),
    }
}

fn preview(value: &FlowPreviewGui) -> Value<'_> {
    object([
        ("id", text(&value.id)),
        ("source", value.source.as_ref().map(|value| object([("neuron", text(&value.neuron)), ("channel", text(&value.channel))])).unwrap_or_else(null)),
        ("mode", text(&value.mode)), ("preview", dictionary(&value.preview)), ("expanded", set(&value.expanded)), ("layout", optional_layout(&value.layout)),
    ])
}

fn gui(value: &FlowGui) -> Value<'_> {
    object([
        ("camera", object([("x", number(value.camera.x)), ("y", number(value.camera.y)), ("zoom", number(value.camera.zoom))])),
        ("nodes", Value::Object(Object::new(value.nodes.iter().map(|(key, value)| (key.as_str(), object([("layout", layout(&value.layout)), ("chrome", chrome(&value.chrome))])))))),
        ("previews", array(value.previews.iter().map(preview))),
    ])
}

fn widget(value: &Widget) -> Value<'_> {
    match value {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => object([
            ("kind", text("neuron")), ("id", text(id)), ("neuronKind", text(neuron_kind)), ("params", dictionary(params)),
            ("input_ports", strings(input_ports)), ("output_ports", strings(output_ports)), ("preview", boolean(*preview)),
        ]),
        Widget::InputSlider { id, label, value, min, max, step } => object([
            ("kind", text("inputSlider")), ("id", text(id)), ("label", text(label)), ("value", number(*value)), ("min", number(*min)), ("max", number(*max)), ("step", number(*step)),
        ]),
        Widget::InputNote { id, text: value } => object([("kind", text("inputNote")), ("id", text(id)), ("text", text(value))]),
        Widget::InputImage { id, src } => object([("kind", text("inputImage")), ("id", text(id)), ("src", text(src))]),
        Widget::Variable { id, name, schema } => object([("kind", text("variable")), ("id", text(id)), ("name", text(name)), ("schema", text(schema))]),
        Widget::OutputPreview { id, preview, expanded } => object([("kind", text("outputPreview")), ("id", text(id)), ("preview", dictionary(preview)), ("expanded", set(expanded))]),
        Widget::OutputAction { id, action } => object([("kind", text("outputAction")), ("id", text(id)), ("action", text(action))]),
        Widget::OutputExport { id, format } => object([("kind", text("outputExport")), ("id", text(id)), ("format", text(format))]),
        Widget::Cluster { id, name, tree: value, flow } => object([("kind", text("cluster")), ("id", text(id)), ("name", text(name)), ("tree", tree(value)), ("flow", gui(flow))]),
    }
}

fn layout_entry(value: &FlowLayoutEntry) -> Value<'_> { object([("id", text(&value.id)), ("layout", optional_layout(&value.layout))]) }
//#endregion 🪟️Presentation

//#region 🧬️Mutations
impl ArtifactCanonicalJson for FlowMutation {
    fn canonical_json_borrowed_root(&self) -> Result<Option<Value<'_>>, String> {
        Ok(Some(match self {
            Self::CreateWidget(value) => object([("mutation", text("createWidget")), ("index", index(value.index)), ("widget", widget(&value.widget))]),
            Self::DeleteWidget(value) => object([("mutation", text("deleteWidget")), ("id", text(&value.id))]),
            Self::ReorderWidgets(value) => object([("mutation", text("reorderWidgets")), ("id", text(&value.id)), ("toIndex", index(value.to_index))]),
            Self::ReplaceWidget(value) => object([("mutation", text("replaceWidget")), ("id", text(&value.id)), ("widget", widget(&value.widget))]),
            Self::ConnectWidgets(value) => object([
                ("mutation", text("connectWidgets")), ("index", index(value.index)), ("id", text(&value.id)), ("from", text(&value.from)), ("fromPort", text(&value.from_port)), ("to", text(&value.to)), ("toPort", text(&value.to_port)),
            ]),
            Self::DisconnectWidgets(value) => object([("mutation", text("disconnectWidgets")), ("id", text(&value.id))]),
            Self::ReorderSynapses(value) => object([("mutation", text("reorderSynapses")), ("id", text(&value.id)), ("toIndex", index(value.to_index))]),
            Self::UpdateSynapseEndpoints(value) => object([
                ("mutation", text("updateSynapseEndpoints")), ("id", text(&value.id)), ("from", text(&value.from)), ("fromPort", text(&value.from_port)), ("to", text(&value.to)), ("toPort", text(&value.to_port)),
            ]),
            Self::MoveWidgets(value) => object([("mutation", text("moveWidgets")), ("entries", array(value.entries.iter().map(layout_entry)))]),
            Self::DuplicateWidget(value) => object([
                ("mutation", text("duplicateWidget")), ("sourceId", text(&value.source_id)), ("newId", text(&value.new_id)), ("synapseId", text(&value.synapse_id)), ("fromPort", text(&value.from_port)), ("toPort", text(&value.to_port)),
            ]),
        }))
    }
}
//#endregion 🧬️Mutations

//#region 🗿️Scene
impl ArtifactCanonicalJson for crate::artifacts::flow::FlowWorkingScene {
    fn canonical_json_borrowed_root(&self) -> Result<Option<Value<'_>>, String> {
        Ok(Some(object([
            ("widgets", array(self.widgets.iter().map(widget))),
            ("synapses", array(self.synapses.iter().map(|value| object([
                ("id", text(&value.id)), ("from", text(&value.from)), ("to", text(&value.to)), ("fromPort", text(&value.from_port)), ("toPort", text(&value.to_port)),
            ])))),
            ("layout", Value::Object(Object::new(self.layout.iter().map(|(key, value)| (key.as_str(), layout(value)))))),
        ])))
    }
}
//#endregion 🗿️Scene

//#region 🧪️SerdeOracle
#[cfg(test)]
mod tests {
    use super::*;

    fn encode(value: Value<'_>, bytes: &mut Vec<u8>) {
        match value {
            Value::Scalar(value) => match value {
                Node::Null => serde_json::to_writer(bytes, &()).unwrap(), Node::Bool(value) => serde_json::to_writer(bytes, &value).unwrap(),
                Node::I64(value) => serde_json::to_writer(bytes, &value).unwrap(), Node::U64(value) => serde_json::to_writer(bytes, &value).unwrap(),
                Node::I128(value) => serde_json::to_writer(bytes, &value).unwrap(), Node::U128(value) => serde_json::to_writer(bytes, &value).unwrap(),
                Node::F32(value) => serde_json::to_writer(bytes, &value).unwrap(), Node::F64(value) => serde_json::to_writer(bytes, &value).unwrap(),
                Node::String(value) => serde_json::to_writer(bytes, value).unwrap(), _ => panic!("borrowed scalar cannot be an indexed container"),
            },
            Value::Source(value) => encode(value.canonical_json_borrowed_root().unwrap().unwrap(), bytes),
            Value::Array(values) => {
                bytes.push(b'[');
                for (index, value) in values.enumerate() { if index != 0 { bytes.push(b','); } encode(value, bytes); }
                bytes.push(b']');
            }
            Value::Object(values) => {
                bytes.push(b'{');
                for (index, (key, value)) in values.enumerate() {
                    if index != 0 { bytes.push(b','); }
                    serde_json::to_writer(&mut *bytes, key).unwrap(); bytes.push(b':'); encode(value, bytes);
                }
                bytes.push(b'}');
            }
        }
    }

    #[test]
    fn every_artifact_variant_matches_serde_bytes_including_nested_chrome() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🧾️artifact-canonical.json")).unwrap();
        for row in fixture["widgets"].as_array().unwrap() {
            let value: Widget = serde_json::from_value(row.clone()).unwrap();
            let mut bytes = Vec::new(); encode(widget(&value), &mut bytes);
            assert_eq!(bytes, serde_json::to_vec(&value).unwrap(), "widget {:?}", row["kind"]);
        }
        for row in fixture["mutations"].as_array().unwrap() {
            let value: FlowMutation = serde_json::from_value(row.clone()).unwrap();
            let mut bytes = Vec::new(); encode(value.canonical_json_borrowed_root().unwrap().unwrap(), &mut bytes);
            assert_eq!(bytes, serde_json::to_vec(&value).unwrap(), "mutation {:?}", row["mutation"]);
        }
    }

    #[test]
    fn large_unicode_key_and_label_scene_matches_serde_without_an_ordinal_map_scan() {
        let scene = crate::artifacts::flow::FlowWorkingScene {
            widgets: vec![Widget::InputSlider { id: "height".into(), label: "🌊".repeat(2048), value: 6.0, min: 0.0, max: 10.0, step: 0.5 }],
            layout: flow::OrderedMap::from([("🌊".repeat(1025), flow::WidgetLayout { x: 1.0, y: 2.0 })]),
            synapses: Vec::new(),
        };
        let mut bytes = Vec::new(); encode(scene.canonical_json_borrowed_root().unwrap().unwrap(), &mut bytes);
        assert_eq!(bytes, serde_json::to_vec(&scene).unwrap());
        assert!(bytes.len() > 4096);
    }
}
//#endregion 🧪️SerdeOracle
