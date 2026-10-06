//! 📜️ Generation2d artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `FlowHostSnapshot`/`Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson` (from `flow`) and
//! `GenerationPlayState`/`FormGeneration`/`GenerationMutation` (from `playbook`) are all foreign to
//! this crate, so none can carry a `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires
//! the impl target type to live in the crate that also owns the trait or the type, and neither is
//! true here. The `*Dsl` types below are LOCAL structural twins the real types convert to/from right
//! at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary (same pattern as `fem_2d`'s `FemDof`
//! and `imperative_core`'s `ValueDsl`/`StepNodeDsl`/`PathDsl`) — `Generation2dSnapshot`/
//! `Generation2dMutation` themselves keep their ORIGINAL foreign field types unchanged.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Generation2dSnapshot;
use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};
use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};
use semio_framework_artifact_playbook_playbook::{FormGeneration, GenerationPlayState};
use std::collections::BTreeMap;

/// 📦️ The `procedural2d-play` "default" example, embedded at compile time as handcrafted `.generation2d`
/// DSL text — shared by the manifest's `.example(...)` registration, the `default_snapshot` fallback,
/// and every test host_snapshot.
pub const GENERATION2D_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

//#region 🔖️DslMirror
/// 🔒️ `ValueDsl` mirrors `semio_framework_artifact_flow_flow::neural::Value`/`Atom` field-for-field rather than routing through
/// the engine's dynamic `Shape::Value`/`DslValue` escape hatch, which merges `Atom::Integer`/
/// `Atom::Decimal` into one `Number(f64)` case — a real, observable loss of fidelity `ValueDsl`'s own
/// mutually-exclusive `Option` fields avoid entirely.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
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
/// bare `Shape::Map` (`{ key=value }`): a `Shape::Map` key is a bare identifier, but real `Dictionary`
/// keys are arbitrary strings — notably `neural_engine::SCHEMA_KEY` (`"$schema"`).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
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

pub fn dictionary_to_value_dsl_entries(dict: &Dictionary) -> Vec<DictEntryDsl> {
    dict.keys().map(|key| DictEntryDsl { key: key.clone(), value: value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")) }).collect()
}

pub fn value_dsl_entries_to_dictionary(entries: &[DictEntryDsl]) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, entry| dict.insert(entry.key.clone(), value_dsl_to_value(&entry.value)))
}

/// 🎥️ Local twin of `semio_framework_artifact_flow_flow::CameraJson`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct CameraJsonDsl {
    x: f64,
    y: f64,
    zoom: f64,
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
    x: f64,
    y: f64,
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
    id: String,
    wire: semio_framework_dsl_record::Wire,
}

pub fn synapse_to_dsl(synapse: &SynapseSpec) -> SynapseSpecDsl {
    SynapseSpecDsl {
        id: synapse.id.clone(),
        wire: semio_framework_dsl_record::Wire(semio_framework_dsl_record::WireValue {
            from: semio_framework_dsl_record::WireNode { id: synapse.from.clone(), kind: None, port: (!synapse.from_port.is_empty()).then(|| synapse.from_port.clone()) },
            edge: Some((true, semio_framework_dsl_record::WireNode { id: synapse.to.clone(), kind: None, port: (!synapse.to_port.is_empty()).then(|| synapse.to_port.clone()) })),
            edge_label: semio_framework_dsl_record::WireEdgeLabel::default(),
            properties: semio_framework_value::DslValue::Object(Vec::new()),
        }),
    }
}

pub fn synapse_from_dsl(synapse: SynapseSpecDsl) -> SynapseSpec {
    let wire = synapse.wire.0;
    let to = wire.edge.map(|(_, to)| to).unwrap_or_default();
    SynapseSpec { id: synapse.id, from: wire.from.id, to: to.id, from_port: wire.from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() }
}

/// 🎛️ Local twin of `semio_framework_artifact_flow_flow::Widget` — `Neuron`/`OutputPreview`'s `Dictionary` fields route
/// through `ValueDsl`; `Cluster`'s `tree`/`flow` are carried as an opaque `semio_framework_value::DslValue`.
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
        tree: semio_framework_value::DslValue,
        flow: semio_framework_value::DslValue,
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
    id: String,
    name: String,
    values: BTreeMap<String, semio_framework_value::DslValue>,
}

pub fn form_generation_to_dsl(generation: &FormGeneration) -> FormGenerationDsl {
    FormGenerationDsl { id: generation.id.clone(), name: generation.name.clone(), values: generation.values.iter().map(|(key, value)| (key.clone(), value.clone())).collect() }
}

pub fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {
    FormGeneration { id: generation.id, name: generation.name, values: generation.values.into_iter().collect() }
}

/// 🧾️ Local twin of `Generation2dSnapshot`, flattening `FlowHostSnapshot`/`GenerationPlayState`'s fields
/// into one top-level `#[derive(dsl::DslRecord)]` grammar.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(id = "procedural.generation2d", layout = "lines")]
struct Generation2dSnapshotDsl {
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
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Generation2dSnapshotDsl {
    const EXTENSION: &'static str = "generation2d";
    fn envelope_id() -> &'static str {
        "procedural.generation2d"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Generation2dSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

fn generation2d_document_to_dsl(document: &Generation2dSnapshot) -> Generation2dSnapshotDsl {
    let host_snapshot = &document.host_snapshot;
    let generation = &document.generation;
    Generation2dSnapshotDsl {
        schema: host_snapshot.schema.clone(),
        camera: camera_to_dsl(&host_snapshot.camera),
        widgets: host_snapshot.widgets.iter().map(widget_to_dsl).collect(),
        synapses: host_snapshot.synapses.iter().map(synapse_to_dsl).collect(),
        layout: host_snapshot.layout.iter().map(|(id, entry)| (id.clone(), layout_to_dsl(entry))).collect(),
        selected_generation_id: generation.selected_generation_id.clone(),
        preview_text: generation.preview_text.clone(),
        generations: generation.generations.iter().map(form_generation_to_dsl).collect(),
    }
}

fn generation2d_document_from_dsl(parsed: Generation2dSnapshotDsl) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(&entry))).collect();
    Ok(Generation2dSnapshot {
        host_snapshot: FlowHostSnapshot { schema: parsed.schema, camera: camera_from_dsl(&parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text }.into(),
    })
}

/// 📜️ `.generation2d` textual document — derive-engine grammar via `Generation2dSnapshotDsl`.
impl store::ArtifactDsl for Generation2dSnapshot {
    const EXTENSION: &'static str = "generation2d";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = <Generation2dSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        generation2d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Generation2dSnapshotDsl as store::ArtifactDsl>::print_dsl(&generation2d_document_to_dsl(self))
    }
}

/// 📦️ `.generation2d` binary pack — same `Generation2dSnapshotDsl` mirror as `ArtifactDsl` above;
/// `dsl::DslArtifact`'s derive already gives `Generation2dSnapshotDsl` its own `ArtifactPack` impl.
impl store::ArtifactPack for Generation2dSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let document = generation2d_document_to_dsl(self);
        let inner = store::pack_rt::encode_document(&Generation2dSnapshotDsl::__dsl_spec(), &document.__dsl_to_record(), options)?;
        let mut bytes = Vec::with_capacity(4 + inner.len());
        bytes.extend_from_slice(b"P2D2");
        bytes.extend_from_slice(&inner);
        Ok(bytes)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if !bytes.starts_with(b"P2D2") {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "generation2d pack discriminator mismatch")));
        }
        let (record, _report) = store::pack_rt::decode_document(&bytes[4..], &Generation2dSnapshotDsl::__dsl_spec(), options)?;
        let parsed = Generation2dSnapshotDsl::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;
        generation2d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Generation2dSnapshotDsl as store::ArtifactPack>::record_spec()
    }
}
//#endregion 🔖️DslMirror

/// 📖️ Parses `.generation2d` DSL text into a `Generation2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    <Generation2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Generation2dSnapshot` back to `.generation2d` DSL text.
pub fn print_dsl(document: &Generation2dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Generation2dSnapshotText = String;

type ControlledSnapshotDsl = Generation2dSnapshotDsl;
type ControlledSnapshot = Generation2dSnapshot;
const CONTROLLED_ENVELOPE_ID: &str = "procedural.generation2d";
const CONTROLLED_PACK_PREFIX: Option<&[u8]> = Some(b"P2D2");
#[path="../../../../../../../../../🫀️core/🧬️generation/🪶️sqlite/🚦️native/🦀️.rs"]mod controlled;
pub(crate) use controlled::{decode as decode_sqlite_native,encode as encode_sqlite_native};
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshot;
use semio_framework_artifact_infinite_dag::DagHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_host_snapshot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::render_scene_json;
use ::semio_framework_schema::ArtifactSchema;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::{flow_host_with_session, flow_neuron_kind_info_map, FlowEvalSession, FlowHost};
#[cfg(feature = "component-app-assembly")]
use semio_framework_ui::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactDsl;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;

#[cfg(feature = "component-app-assembly")]
pub fn collect_drawing_handles_from_eval(value: &semio_framework_pack_json::Value, handles: &mut Vec<String>) {
    match value {
        semio_framework_pack_json::Value::Object(map) => {
            if map.get("$schema").and_then(|entry| entry.as_str()) == Some("draw.drawing") {
                if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                    handles.push(handle.into());
                }
            }
            for (_, entry) in map.iter() {
                collect_drawing_handles_from_eval(entry, handles);
            }
        }
        semio_framework_pack_json::Value::Array(items) => {
            for item in items {
                collect_drawing_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

#[cfg(feature = "component-app-assembly")]
pub fn affine_transform_array(value: &semio_framework_pack_json::Value) -> [f64; 6] {
    if let Some(matrix) = value.as_array() {
        let mut out = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (index, entry) in matrix.iter().take(6).enumerate() {
            out[index] = entry.as_f64().unwrap_or(if index == 0 || index == 3 { 1.0 } else { 0.0 });
        }
        return out;
    }
    if let Some(matrix) = value.get("0").and_then(|entry| entry.as_array()) {
        let wrapped = semio_framework_pack_json::Value::Array(matrix.clone());
        return affine_transform_array(&wrapped);
    }
    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

#[cfg(feature = "component-app-assembly")]
pub fn path_segments_from_node(node: &semio_framework_pack_json::Value) -> Vec<semio_framework_pack_json::Value> {
    if let Some(segments) = node.get("segments").and_then(|entry| entry.as_array()) {
        return segments.clone();
    }
    for key in ["path", "shape", "line", "polyline", "rect", "ellipse", "circle", "polygon"] {
        if let Some(inner) = node.get(key) {
            if let Some(segments) = inner.get("segments").and_then(|entry| entry.as_array()) {
                return segments.clone();
            }
        }
    }
    Vec::new()
}

#[cfg(feature = "component-app-assembly")]
pub fn scene_layers_from_drawing_handle(handle: &str, prefix: &str) -> Vec<semio_framework_pack_json::Value> {
    let scene_json = render_scene_json(handle);
    let Ok(scene) = semio_framework_pack_json::parse(&scene_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
        return Vec::new();
    };
    if scene.get("error").is_some() {
        return Vec::new();
    }
    let Some(nodes) = scene.get("nodes").and_then(|entry| entry.as_array()) else {
        return Vec::new();
    };
    nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let node_body = node.get("node").unwrap_or(node);
            let transform: Vec<semio_framework_pack_json::Value> = affine_transform_array(node.get("transform").unwrap_or(&semio_framework_pack_json::Value::Null)).into_iter().map(semio_framework_pack_json::Value::from).collect();
            let mut object = semio_framework_pack_json::Object::new();
            object.insert("id", semio_framework_pack_json::Value::from(format!("{prefix}-{handle}-{index}")));
            object.insert("transform", semio_framework_pack_json::Value::from(transform));
            object.insert("segments", semio_framework_pack_json::Value::from(path_segments_from_node(node_body)));
            object.insert("fill", node.get("fill").cloned().unwrap_or(semio_framework_pack_json::Value::Null));
            object.insert("stroke", node.get("stroke").cloned().unwrap_or(semio_framework_pack_json::Value::Null));
            object.insert("opacity", semio_framework_pack_json::Value::from(node.get("opacity").and_then(|entry| entry.as_f64()).unwrap_or(1.0)));
            object.insert("blendMode", semio_framework_pack_json::Value::from("normal"));
            object.insert("visible", semio_framework_pack_json::Value::from(true));
            object.insert("needsKernel", semio_framework_pack_json::Value::from(false));
            semio_framework_pack_json::Value::Object(object)
        })
        .collect()
}

#[cfg(feature = "component-app-assembly")]
pub fn generation_preview_host(host_snapshot: &FlowHostSnapshot, values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> FlowHost {
    let snapshot_json = semio_framework_pack_json::to_json_string(host_snapshot);
    let object: semio_framework_pack_json::Object = values.iter().map(|(key, value)| (key.clone(), semio_framework_pack_json::from_dsl_value(value))).collect();
    let patched = apply_generation_values_to_host_snapshot(&snapshot_json, &object);
    let patched_fixture = FlowHost::parse_host_snapshot_json(&patched).unwrap_or_else(|_| host_snapshot.clone());
    FlowHost::from_host_snapshot(patched_fixture)
}

/// 📤️ The drawings a program OUTPUTS: every drawing a synapse delivers into an output widget
/// (`output-preview`, `output-export`), read off the evaluation at the synapse's source port, in
/// synapse order without repeats. Intermediate drawings (a shape before its style) are not outputs.
#[cfg(feature = "component-app-assembly")]
pub fn output_drawing_handles(host_snapshot: &FlowHostSnapshot, outputs: &semio_framework_pack_json::Value) -> Vec<String> {
    use semio_framework_artifact_flow_flow::Widget;
    let is_output = |id: &str| host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::OutputPreview { id: output, .. } | Widget::OutputExport { id: output, .. } if output == id));
    let mut handles = Vec::new();
    for synapse in host_snapshot.synapses.iter().filter(|synapse| is_output(&synapse.to)) {
        let Some(out) = outputs.get(&synapse.from).and_then(|node| node.get("out")) else { continue };
        let delivered = if synapse.from_port.is_empty() { Some(out) } else { out.get(&synapse.from_port) };
        let mut found = Vec::new();
        if let Some(value) = delivered {
            collect_drawing_handles_from_eval(value, &mut found);
        }
        for handle in found {
            if !handles.contains(&handle) {
                handles.push(handle);
            }
        }
    }
    handles
}

/// 🖼️ [`output_drawing_handles`] as the scene layers the `drawing:out` port publishes.
#[cfg(feature = "component-app-assembly")]
pub fn generation_output_layers(host_snapshot: &FlowHostSnapshot, eval_json: &str) -> String {
    let layers: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|outputs| output_drawing_handles(host_snapshot, &outputs).iter().flat_map(|handle| scene_layers_from_drawing_handle(handle, "generation2d-drawing-out")).collect()).unwrap_or_default();
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::from(layers))
}

#[cfg(feature = "component-app-assembly")]
pub fn generation_preview_layers(eval_json: &str) -> String {
    let prefix = "generation2d-generate-preview";
    let mut layers = Vec::new();
    if let Ok(outputs) = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        let mut handles = Vec::new();
        collect_drawing_handles_from_eval(&outputs, &mut handles);
        handles.sort();
        handles.dedup();
        for handle in handles {
            layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
        }
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::from(layers))
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::standards::v1::subsets::any::schema::*;
use crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshot;
use semio_framework_artifact_infinite_dag::DagHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_host_snapshot;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::render_scene_json;
use ::semio_framework_schema::ArtifactSchema;
#[cfg(feature = "component-app-assembly")]
use semio_framework_os_flow::{flow_host_with_session, flow_neuron_kind_info_map, FlowEvalSession, FlowHost};
#[cfg(feature = "component-app-assembly")]
use semio_framework_ui::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactDsl;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;

/// 📄️ The `procedural2d-play` "default" document — parsed from the bundled `.generation2d` example
/// host_snapshot, falling back to the empty document if the fixture ever fails to parse.
pub fn default_snapshot() -> Generation2dSnapshot {
    Generation2dSnapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::GENERATION2D_EXAMPLE_TEXT).unwrap_or_default()
}
}
pub use snapshot_wire_codec::*;
