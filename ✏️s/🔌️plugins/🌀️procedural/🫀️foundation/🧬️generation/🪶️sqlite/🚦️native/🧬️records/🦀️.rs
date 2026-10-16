//! 🧬️ The seven local DSL twins of the generation host and play-state records, shared by generation 2d and 3d, with the conversions to and from the foreign `flow` and `playbook` types.
//!
//! The foreign types cannot carry the derive themselves (orphan rule), so the twins live here and both artifacts re-export them.
use semio_framework_os_flow_contract::neural::{Atom, Dictionary, Value as NeuralValue};
use semio_framework_os_flow_contract::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use semio_framework_os_playbook_contract::FormGeneration;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

/// 🔒️ `ValueDsl` mirrors `semio_framework_artifact_flow_flow::neural::Value`/`Atom` field-for-field rather than routing through
/// the engine's dynamic `Shape::Value`/`DslValue` escape hatch, which merges `Atom::Integer`/
/// `Atom::Decimal` into one `Number(f64)` case — a real, observable loss of fidelity `ValueDsl`'s own
/// mutually-exclusive `Option` fields avoid entirely.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct ValueDsl {
    /// 🕳️ Presence-only flag (the payload is never inspected) — `Atom::Null`'s tag.
    pub(crate) null: Option<bool>,
    #[dsl(key = "bool")]
    pub(crate) boolean: Option<bool>,
    #[dsl(key = "int")]
    pub(crate) integer: Option<i64>,
    pub(crate) decimal: Option<f64>,
    pub(crate) text: Option<String>,
    #[dsl(key = "dict")]
    pub(crate) dictionary: Option<Vec<DictEntryDsl>>,
}

/// 🗝️ One `Dictionary`/`Value::Dictionary` entry — a `Vec` of `(key, value)` records rather than a
/// bare `Shape::Map` (`{ key=value }`): a `Shape::Map` key is a bare identifier, but real `Dictionary`
/// keys are arbitrary strings — notably `neural_engine::SCHEMA_KEY` (`"$schema"`).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct DictEntryDsl {
    pub(crate) key: String,
    #[dsl(block)]
    pub(crate) value: ValueDsl,
}

fn value_to_value_dsl(value: &NeuralValue) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        NeuralValue::Atom(Atom::Null) => dsl_value.null = Some(true),
        NeuralValue::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        NeuralValue::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        NeuralValue::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        NeuralValue::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        NeuralValue::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_entries(dict)),
    }
    dsl_value
}

fn value_dsl_to_value(dsl_value: &ValueDsl) -> NeuralValue {
    if dsl_value.null.is_some() {
        return NeuralValue::Atom(Atom::Null);
    }
    if let Some(b) = dsl_value.boolean {
        return NeuralValue::Atom(Atom::Boolean(b));
    }
    if let Some(i) = dsl_value.integer {
        return NeuralValue::Atom(Atom::Integer(i));
    }
    if let Some(d) = dsl_value.decimal {
        return NeuralValue::Atom(Atom::Decimal(d));
    }
    if let Some(s) = &dsl_value.text {
        return NeuralValue::Atom(Atom::String(s.clone()));
    }
    match &dsl_value.dictionary {
        Some(entries) => NeuralValue::Dictionary(value_dsl_entries_to_dictionary(entries)),
        None => NeuralValue::Atom(Atom::Null),
    }
}

pub fn dictionary_to_value_dsl_entries(dict: &Dictionary) -> Vec<DictEntryDsl> {
    dict.keys().map(|key| DictEntryDsl { key: key.clone(), value: value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")) }).collect()
}

pub fn value_dsl_entries_to_dictionary(entries: &[DictEntryDsl]) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, entry| dict.insert(entry.key.clone(), value_dsl_to_value(&entry.value)))
}

/// 🎥️ Local twin of `semio_framework_artifact_flow_flow::CameraJson`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct CameraJsonDsl {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) zoom: f64,
}

pub fn camera_to_dsl(camera: &CameraJson) -> CameraJsonDsl {
    CameraJsonDsl { x: camera.x, y: camera.y, zoom: camera.zoom }
}

pub fn camera_from_dsl(camera: &CameraJsonDsl) -> CameraJson {
    CameraJson { x: camera.x, y: camera.y, zoom: camera.zoom }
}

/// 📍️ Local twin of `semio_framework_artifact_flow_flow::WidgetLayout`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct WidgetLayoutDsl {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

pub fn layout_to_dsl(layout: &WidgetLayout) -> WidgetLayoutDsl {
    WidgetLayoutDsl { x: layout.x, y: layout.y }
}

pub fn layout_from_dsl(layout: &WidgetLayoutDsl) -> WidgetLayout {
    WidgetLayout { x: layout.x, y: layout.y }
}

/// 🔗️ Local twin of `semio_framework_artifact_flow_flow::SynapseSpec` — a graph edge (`from@fromPort->to@toPort`) via the
/// engine's unified `dsl::Wire` shape.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct SynapseSpecDsl {
    pub(crate) id: String,
    pub(crate) wire: semio_framework_dsl_record::Wire,
}

pub fn synapse_to_dsl(synapse: &SynapseSpec) -> SynapseSpecDsl {
    SynapseSpecDsl {
        id: synapse.id.clone(),
        wire: semio_framework_dsl_record::Wire(semio_framework_dsl_record::WireValue {
            from: semio_framework_dsl_record::WireNode { id: synapse.from.clone(), kind: None, port: (!synapse.from_port.is_empty()).then(|| synapse.from_port.clone()) },
            edge: Some((true, semio_framework_dsl_record::WireNode { id: synapse.to.clone(), kind: None, port: (!synapse.to_port.is_empty()).then(|| synapse.to_port.clone()) })),
            edge_label: semio_framework_dsl_record::WireEdgeLabel::default(),
            properties: DslValue::Object(Vec::new()),
        }),
    }
}

pub fn synapse_from_dsl(synapse: SynapseSpecDsl) -> SynapseSpec {
    let wire = synapse.wire.0;
    let to = wire.edge.map(|(_, to)| to).unwrap_or_default();
    SynapseSpec { id: synapse.id, from: wire.from.id, to: to.id, from_port: wire.from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() }
}

/// 🎛️ Local twin of `semio_framework_artifact_flow_flow::Widget` — `Neuron`/`OutputPreview`'s `Dictionary` fields route
/// through `ValueDsl`; `Cluster`'s `tree`/`flow` are carried as an opaque `DslValue`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum WidgetDsl {
    Neuron {
        id: String,
        neuron_kind: String,
        preview: bool,
        input_ports: Vec<String>,
        output_ports: Vec<String>,
        #[dsl(table)]
        params: Vec<DictEntryDsl>,
    },
    InputSlider {
        id: String,
        label: String,
        value: f64,
        min: f64,
        max: f64,
        step: f64,
    },
    InputNote {
        id: String,
        text: String,
    },
    InputImage {
        id: String,
        src: String,
    },
    Variable {
        id: String,
        name: String,
        schema: String,
    },
    OutputPreview {
        id: String,
        #[dsl(table)]
        preview: Vec<DictEntryDsl>,
        expanded: Vec<String>,
    },
    OutputAction {
        id: String,
        action: String,
    },
    OutputExport {
        id: String,
        format: String,
    },
    Cluster {
        id: String,
        name: String,
        tree: DslValue,
        flow: DslValue,
    },
}

pub fn widget_to_dsl(widget: &Widget) -> WidgetDsl {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            WidgetDsl::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), preview: *preview, input_ports: input_ports.clone(), output_ports: output_ports.clone(), params: dictionary_to_value_dsl_entries(params) }
        }
        Widget::InputSlider { id, label, value, min, max, step } => WidgetDsl::InputSlider { id: id.clone(), label: label.clone(), value: *value, min: *min, max: *max, step: *step },
        Widget::InputNote { id, text } => WidgetDsl::InputNote { id: id.clone(), text: text.clone() },
        Widget::InputImage { id, src } => WidgetDsl::InputImage { id: id.clone(), src: src.clone() },
        Widget::Variable { id, name, schema } => WidgetDsl::Variable { id: id.clone(), name: name.clone(), schema: schema.clone() },
        Widget::OutputPreview { id, preview, expanded } => WidgetDsl::OutputPreview { id: id.clone(), preview: dictionary_to_value_dsl_entries(preview), expanded: expanded.iter().cloned().collect() },
        Widget::OutputAction { id, action } => WidgetDsl::OutputAction { id: id.clone(), action: action.clone() },
        Widget::OutputExport { id, format } => WidgetDsl::OutputExport { id: id.clone(), format: format.clone() },
        Widget::Cluster { id, name, tree, flow } => WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: semio_framework_value::ToValue::to_value(tree), flow: semio_framework_value::ToValue::to_value(flow) },
    }
}

pub fn widget_from_dsl(widget: WidgetDsl) -> Result<Widget, semio_framework_diagnostic::TextError> {
    Ok(match widget {
        WidgetDsl::Neuron { id, neuron_kind, preview, input_ports, output_ports, params } => Widget::Neuron { id, neuron_kind, params: value_dsl_entries_to_dictionary(&params), input_ports, output_ports, preview },
        WidgetDsl::InputSlider { id, label, value, min, max, step } => Widget::InputSlider { id, label, value, min, max, step },
        WidgetDsl::InputNote { id, text } => Widget::InputNote { id, text },
        WidgetDsl::InputImage { id, src } => Widget::InputImage { id, src },
        WidgetDsl::Variable { id, name, schema } => Widget::Variable { id, name, schema },
        WidgetDsl::OutputPreview { id, preview, expanded } => Widget::OutputPreview { id, preview: value_dsl_entries_to_dictionary(&preview), expanded: expanded.into_iter().collect() },
        WidgetDsl::OutputAction { id, action } => Widget::OutputAction { id, action },
        WidgetDsl::OutputExport { id, format } => Widget::OutputExport { id, format },
        WidgetDsl::Cluster { id, name, tree, flow } => Widget::Cluster {
            id,
            name,
            tree: semio_framework_value::FromValue::from_value(tree).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("invalid cluster tree: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?,
            flow: semio_framework_value::FromValue::from_value(flow).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("invalid cluster flow: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?,
        },
    })
}

/// 🧬️ Local twin of `semio_framework_artifact_playbook_playbook::FormGeneration`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct FormGenerationDsl {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) values: BTreeMap<String, DslValue>,
}

pub fn form_generation_to_dsl(generation: &FormGeneration) -> FormGenerationDsl {
    FormGenerationDsl { id: generation.id.clone(), name: generation.name.clone(), values: generation.values.iter().map(|(key, value)| (key.clone(), value.clone())).collect() }
}

pub fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {
    FormGeneration { id: generation.id, name: generation.name, values: generation.values.into_iter().collect() }
}
