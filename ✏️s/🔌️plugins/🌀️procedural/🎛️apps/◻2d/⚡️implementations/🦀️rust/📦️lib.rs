//! 📏️ Procedural 2D app — document entities (constitutional: general).

use flow_core::neural::{Atom, Dictionary, Value as NeuralValue};
use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use playbook::{FormGeneration, GenerationPlayState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d";

//#region 🔖️Document
/// 🧾️ Persistent procedural-2d document — the flow fixture plus the generation vocabulary state.
/// Ephemeral view state (selection, show mode, preview evaluations) lives in the plugin app struct.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDocument {
    pub fixture: FlowFixture,
    #[serde(default)]
    pub generation: GenerationPlayState,
}

/// 🪪️ A flow widget's stable id, across every widget variant (mirrors flow_core's private accessor).
pub fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}
//#endregion 🔖️Document

//#region 🔖️Dsl
//#region 🔖️DslMirror
/// 🔒️ `FlowFixture`/`Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson` (from `flow_core`) and
/// `GenerationPlayState`/`FormGeneration`/`GenerationOperation` (from `playbook`) are all foreign to
/// this crate, so none can carry a `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires
/// the impl target type to live in the crate that also owns the trait or the type, and neither is
/// true here. The `*Dsl` types below are LOCAL structural twins the real types convert to/from right
/// at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary (same pattern as `fem_2d`'s `FemDof`
/// and `imperative_core`'s `ValueDsl`/`StepNodeDsl`/`PathDsl`) — `Procedural2dDocument`/
/// `Procedural2dOperation` themselves keep their ORIGINAL foreign field types unchanged, so
/// `procedural-plugin` (which constructs/matches on them directly) keeps compiling unmodified.
///
/// `ValueDsl` mirrors `flow_core::neural::Value`/`Atom` field-for-field (duplicating
/// `imperative_core::ValueDsl`'s approach for the identical `neural_engine` types, since crates can't
/// share a private type across the orphan boundary either) rather than routing through the engine's
/// dynamic `Shape::Value`/`DslValue` escape hatch, which merges `Atom::Integer`/`Atom::Decimal` into
/// one `Number(f64)` case — a real, observable loss of fidelity `ValueDsl`'s own mutually-exclusive
/// `Option` fields avoid entirely.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct ValueDsl {
    /// 🕳️ Presence-only flag (the payload is never inspected) — `Atom::Null`'s tag.
    null: Option<bool>,
    #[dsl(key = "bool")]
    boolean: Option<bool>,
    #[dsl(key = "int")]
    integer: Option<i64>,
    decimal: Option<f64>,
    text: Option<String>,
    #[dsl(key = "dict")]
    dictionary: Option<Vec<DictEntryDsl>>,
}

/// 🗝️ One `Dictionary`/`Value::Dictionary` entry — a `Vec` of `(key, value)` records rather than a
/// bare `Shape::Map` (`{ key=value }`): a `Shape::Map` key is a bare identifier (the engine lexer's
/// `is_ident_start` only accepts alphabetic/`_`), but real `Dictionary` keys are arbitrary strings —
/// notably `neural_engine::SCHEMA_KEY` (`"$schema"`), which every schema-tagged value in this
/// codebase carries and which starts with `$`, a character no bare identifier can start with.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct DictEntryDsl {
    key: String,
    #[dsl(block)]
    value: ValueDsl,
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

fn dictionary_to_value_dsl_entries(dict: &Dictionary) -> Vec<DictEntryDsl> {
    dict.keys().map(|key| DictEntryDsl { key: key.clone(), value: value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")) }).collect()
}

fn value_dsl_entries_to_dictionary(entries: &[DictEntryDsl]) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, entry| dict.insert(entry.key.clone(), value_dsl_to_value(&entry.value)))
}

/// 🎥️ Local twin of `flow_core::CameraJson`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct CameraJsonDsl {
    x: f64,
    y: f64,
    zoom: f64,
}

pub fn camera_to_dsl(camera: &CameraJson) -> CameraJsonDsl {
    CameraJsonDsl { x: camera.x, y: camera.y, zoom: camera.zoom }
}

pub fn camera_from_dsl(camera: CameraJsonDsl) -> CameraJson {
    CameraJson { x: camera.x, y: camera.y, zoom: camera.zoom }
}

/// 📍️ Local twin of `flow_core::WidgetLayout`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct WidgetLayoutDsl {
    x: f64,
    y: f64,
}

pub fn layout_to_dsl(layout: &WidgetLayout) -> WidgetLayoutDsl {
    WidgetLayoutDsl { x: layout.x, y: layout.y }
}

pub fn layout_from_dsl(layout: WidgetLayoutDsl) -> WidgetLayout {
    WidgetLayout { x: layout.x, y: layout.y }
}

/// 🔗️ Local twin of `flow_core::SynapseSpec` — a graph edge (`from@fromPort->to@toPort`) via the
/// engine's unified `dsl::Wire` shape; an empty `from_port`/`to_port` (the "no explicit port" sentinel
/// the real `SynapseSpec` uses) round-trips through an absent `WireNode::port`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct SynapseSpecDsl {
    id: String,
    wire: dsl::Wire,
}

pub fn synapse_to_dsl(synapse: &SynapseSpec) -> SynapseSpecDsl {
    SynapseSpecDsl {
        id: synapse.id.clone(),
        wire: dsl::Wire(dsl::WireValue {
            from: dsl::WireNode { id: synapse.from.clone(), kind: None, port: (!synapse.from_port.is_empty()).then(|| synapse.from_port.clone()) },
            edge: Some((true, dsl::WireNode { id: synapse.to.clone(), kind: None, port: (!synapse.to_port.is_empty()).then(|| synapse.to_port.clone()) })),
            properties: dsl::DslValue::Object(Vec::new()),
        }),
    }
}

pub fn synapse_from_dsl(synapse: SynapseSpecDsl) -> SynapseSpec {
    let wire = synapse.wire.0;
    let to = wire.edge.map(|(_, to)| to).unwrap_or_default();
    SynapseSpec { id: synapse.id, from: wire.from.id, to: to.id, from_port: wire.from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() }
}

/// 🎛️ Local twin of `flow_core::Widget` — `Neuron`/`OutputPreview`'s `Dictionary` fields route
/// through `ValueDsl`; `Cluster`'s `tree`/`flow` (a `neural_engine::Tree`/`flow_core::FlowGui` pair,
/// each themselves nested foreign aggregates several layers deep) are carried as an opaque
/// `serde_json::Value` blob — exactly the same "already-JSON-shaped" treatment the OLD hand-rolled
/// `procedural2d_text::kv_json` gave them, just bound through the engine's own `Shape::Value` bridge
/// instead of a hand-rolled quoted-string re-parse.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
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
        tree: dsl::DslValue,
        flow: dsl::DslValue,
    },
}

pub fn widget_to_dsl(widget: &Widget) -> WidgetDsl {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            WidgetDsl::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), preview: *preview, input_ports: input_ports.clone(), output_ports: output_ports.clone(), params: dictionary_to_value_dsl_entries(params) }
        }
        Widget::InputSlider { id, value, min, max, step } => WidgetDsl::InputSlider { id: id.clone(), value: *value, min: *min, max: *max, step: *step },
        Widget::InputNote { id, text } => WidgetDsl::InputNote { id: id.clone(), text: text.clone() },
        Widget::InputImage { id, src } => WidgetDsl::InputImage { id: id.clone(), src: src.clone() },
        Widget::Variable { id, name, schema } => WidgetDsl::Variable { id: id.clone(), name: name.clone(), schema: schema.clone() },
        Widget::OutputPreview { id, preview, expanded } => WidgetDsl::OutputPreview { id: id.clone(), preview: dictionary_to_value_dsl_entries(preview), expanded: expanded.iter().cloned().collect() },
        Widget::OutputAction { id, action } => WidgetDsl::OutputAction { id: id.clone(), action: action.clone() },
        Widget::OutputExport { id, format } => WidgetDsl::OutputExport { id: id.clone(), format: format.clone() },
        Widget::Cluster { id, name, tree, flow } => WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: dsl::to_dsl_value(tree).unwrap_or(dsl::DslValue::Null), flow: dsl::to_dsl_value(flow).unwrap_or(dsl::DslValue::Null) },
    }
}

pub fn widget_from_dsl(widget: WidgetDsl) -> Result<Widget, store::TextError> {
    Ok(match widget {
        WidgetDsl::Neuron { id, neuron_kind, preview, input_ports, output_ports, params } => Widget::Neuron { id, neuron_kind, params: value_dsl_entries_to_dictionary(&params), input_ports, output_ports, preview },
        WidgetDsl::InputSlider { id, value, min, max, step } => Widget::InputSlider { id, value, min, max, step },
        WidgetDsl::InputNote { id, text } => Widget::InputNote { id, text },
        WidgetDsl::InputImage { id, src } => Widget::InputImage { id, src },
        WidgetDsl::Variable { id, name, schema } => Widget::Variable { id, name, schema },
        WidgetDsl::OutputPreview { id, preview, expanded } => Widget::OutputPreview { id, preview: value_dsl_entries_to_dictionary(&preview), expanded: expanded.into_iter().collect() },
        WidgetDsl::OutputAction { id, action } => Widget::OutputAction { id, action },
        WidgetDsl::OutputExport { id, format } => Widget::OutputExport { id, format },
        WidgetDsl::Cluster { id, name, tree, flow } => Widget::Cluster {
            id,
            name,
            tree: dsl::from_dsl_value(tree).map_err(|error| store::TextError::new(format!("invalid cluster tree: {error}"), store::TextSpan::at(1, 1)))?,
            flow: dsl::from_dsl_value(flow).map_err(|error| store::TextError::new(format!("invalid cluster flow: {error}"), store::TextSpan::at(1, 1)))?,
        },
    })
}

/// 🧬️ Local twin of `playbook::FormGeneration` — `values` is already a `serde_json::Map`/`Value` pair
/// in the real type, so it binds directly through the engine's `Shape::Value` bridge with no
/// intermediate conversion.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct FormGenerationDsl {
    id: String,
    name: String,
    values: std::collections::BTreeMap<String, dsl::DslValue>,
}

pub fn form_generation_to_dsl(generation: &FormGeneration) -> FormGenerationDsl {
    FormGenerationDsl { id: generation.id.clone(), name: generation.name.clone(), values: generation.values.iter().map(|(key, value)| (key.clone(), dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null))).collect() }
}

pub fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {
    FormGeneration { id: generation.id, name: generation.name, values: generation.values.into_iter().filter_map(|(key, value)| dsl::from_dsl_value(value).ok().map(|json| (key, json))).collect() }
}

/// 🧾️ Local twin of `Procedural2dDocument`, flattening `FlowFixture`/`GenerationPlayState`'s fields
/// into one top-level `#[derive(dsl::DslDocument)]` grammar.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "procedural2d", layout = "lines")]
struct Procedural2dDocumentDsl {
    schema: String,
    #[dsl(block)]
    camera: CameraJsonDsl,
    #[dsl(statements, block)]
    widgets: Vec<WidgetDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseSpecDsl>,
    layout: BTreeMap<String, WidgetLayoutDsl>,
    #[dsl(key = "selected-generation")]
    selected_generation_id: Option<String>,
    preview_text: Option<String>,
    #[dsl(table)]
    generations: Vec<FormGenerationDsl>,
}

fn procedural2d_document_to_dsl(document: &Procedural2dDocument) -> Procedural2dDocumentDsl {
    let fixture = &document.fixture;
    let generation = &document.generation;
    Procedural2dDocumentDsl {
        schema: fixture.schema.clone(),
        camera: camera_to_dsl(&fixture.camera),
        widgets: fixture.widgets.iter().map(widget_to_dsl).collect(),
        synapses: fixture.synapses.iter().map(synapse_to_dsl).collect(),
        layout: fixture.layout.iter().map(|(id, entry)| (id.clone(), layout_to_dsl(entry))).collect(),
        selected_generation_id: generation.selected_generation_id.clone(),
        preview_text: generation.preview_text.clone(),
        generations: generation.generations.iter().map(form_generation_to_dsl).collect(),
    }
}

fn procedural2d_document_from_dsl(parsed: Procedural2dDocumentDsl) -> Result<Procedural2dDocument, store::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(entry))).collect();
    Ok(Procedural2dDocument {
        fixture: FlowFixture { schema: parsed.schema, camera: camera_from_dsl(parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text },
    })
}
//#endregion 🔖️DslMirror

/// 📜️ `.procedural2d` textual document — derive-engine grammar via `Procedural2dDocumentDsl`
/// (see `🔖️DslMirror`); `parse_dsl`/`print_dsl` convert at the boundary.
impl store::DocumentDsl for Procedural2dDocument {
    const EXTENSION: &'static str = "procedural2d";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural2dDocumentDsl as store::DocumentDsl>::parse_dsl(text)?;
        procedural2d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Procedural2dDocumentDsl as store::DocumentDsl>::print_dsl(&procedural2d_document_to_dsl(self))
    }
}

/// 📦️ `.procedural2d` binary pack — same `Procedural2dDocumentDsl` mirror as `DocumentDsl` above (see
/// `🔖️DslMirror`); `dsl::DslDocument`'s derive already gives `Procedural2dDocumentDsl` its own
/// `DocumentPack` impl, so this just routes through the same to/from-dsl boundary functions.
impl store::DocumentPack for Procedural2dDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Procedural2dDocumentDsl as store::DocumentPack>::encode_pack_with(&procedural2d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Procedural2dDocumentDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        procedural2d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }
}
//#endregion 🔖️Dsl
