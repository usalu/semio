//! 📜️ Procedural2d artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `FlowFixture`/`Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson` (from `flow`) and
//! `GenerationPlayState`/`FormGeneration`/`GenerationMutation` (from `playbook`) are all foreign to
//! this crate, so none can carry a `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires
//! the impl target type to live in the crate that also owns the trait or the type, and neither is
//! true here. The `*Dsl` types below are LOCAL structural twins the real types convert to/from right
//! at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary (same pattern as `fem_2d`'s `FemDof`
//! and `imperative_core`'s `ValueDsl`/`StepNodeDsl`/`PathDsl`) — `Procedural2dSnapshot`/
//! `Procedural2dMutation` themselves keep their ORIGINAL foreign field types unchanged.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::neural::{Atom, Dictionary, Value as NeuralValue};
use flow::playbook::{FormGeneration, GenerationPlayState};
use flow::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use std::collections::BTreeMap;

/// 📦️ The `procedural2d-play` "default" example, embedded at compile time as handcrafted `.procedural2d`
/// DSL text — shared by the manifest's `.example(...)` registration, the `default_snapshot` fallback,
/// and every test fixture.
pub const PROCEDURAL2D_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

//#region 🔖️DslMirror
/// 🔒️ `ValueDsl` mirrors `flow::neural::Value`/`Atom` field-for-field rather than routing through
/// the engine's dynamic `Shape::Value`/`DslValue` escape hatch, which merges `Atom::Integer`/
/// `Atom::Decimal` into one `Number(f64)` case — a real, observable loss of fidelity `ValueDsl`'s own
/// mutually-exclusive `Option` fields avoid entirely.
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
/// bare `Shape::Map` (`{ key=value }`): a `Shape::Map` key is a bare identifier, but real `Dictionary`
/// keys are arbitrary strings — notably `neural_engine::SCHEMA_KEY` (`"$schema"`).
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

pub fn dictionary_to_value_dsl_entries(dict: &Dictionary) -> Vec<DictEntryDsl> {
    dict.keys().map(|key| DictEntryDsl { key: key.clone(), value: value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")) }).collect()
}

pub fn value_dsl_entries_to_dictionary(entries: &[DictEntryDsl]) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, entry| dict.insert(entry.key.clone(), value_dsl_to_value(&entry.value)))
}

/// 🎥️ Local twin of `flow::CameraJson`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
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

/// 📍️ Local twin of `flow::WidgetLayout`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
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

/// 🔗️ Local twin of `flow::SynapseSpec` — a graph edge (`from@fromPort->to@toPort`) via the
/// engine's unified `dsl::Wire` shape.
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
            edge_label: dsl::WireEdgeLabel::default(),
            properties: dsl::DslValue::Object(Vec::new()),
        }),
    }
}

pub fn synapse_from_dsl(synapse: SynapseSpecDsl) -> SynapseSpec {
    let wire = synapse.wire.0;
    let to = wire.edge.map(|(_, to)| to).unwrap_or_default();
    SynapseSpec { id: synapse.id, from: wire.from.id, to: to.id, from_port: wire.from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() }
}

/// 🎛️ Local twin of `flow::Widget` — `Neuron`/`OutputPreview`'s `Dictionary` fields route
/// through `ValueDsl`; `Cluster`'s `tree`/`flow` are carried as an opaque `dsl::DslValue`.
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
        tree: dsl::DslValue,
        flow: dsl::DslValue,
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
        Widget::Cluster { id, name, tree, flow } => WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: dsl::to_dsl_value(tree).unwrap_or(dsl::DslValue::Null), flow: dsl::to_dsl_value(flow).unwrap_or(dsl::DslValue::Null) },
    }
}

pub fn widget_from_dsl(widget: WidgetDsl) -> Result<Widget, store::TextError> {
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
            tree: dsl::from_dsl_value(tree).map_err(|error| store::TextError::new(format!("invalid cluster tree: {error}"), store::TextSpan::at(1, 1)))?,
            flow: dsl::from_dsl_value(flow).map_err(|error| store::TextError::new(format!("invalid cluster flow: {error}"), store::TextSpan::at(1, 1)))?,
        },
    })
}

/// 🧬️ Local twin of `flow::playbook::FormGeneration`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct FormGenerationDsl {
    id: String,
    name: String,
    values: BTreeMap<String, dsl::DslValue>,
}

pub fn form_generation_to_dsl(generation: &FormGeneration) -> FormGenerationDsl {
    FormGenerationDsl { id: generation.id.clone(), name: generation.name.clone(), values: generation.values.iter().map(|(key, value)| (key.clone(), dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null))).collect() }
}

pub fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {
    FormGeneration { id: generation.id, name: generation.name, values: generation.values.into_iter().filter_map(|(key, value)| dsl::from_dsl_value(value).ok().map(|json| (key, json))).collect() }
}

/// 🧾️ Local twin of `Procedural2dSnapshot`, flattening `FlowFixture`/`GenerationPlayState`'s fields
/// into one top-level `#[derive(dsl::DslRecord)]` grammar.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(id = "procedural.procedural2d", layout = "lines")]
struct Procedural2dSnapshotDsl {
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
impl store::ArtifactDsl for Procedural2dSnapshotDsl {
    const EXTENSION: &'static str = "procedural2d";
    fn envelope_id() -> &'static str {
        "procedural.procedural2d"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Procedural2dSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

fn procedural2d_document_to_dsl(document: &Procedural2dSnapshot) -> Procedural2dSnapshotDsl {
    let fixture = &document.fixture;
    let generation = &document.generation;
    Procedural2dSnapshotDsl {
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

fn procedural2d_document_from_dsl(parsed: Procedural2dSnapshotDsl) -> Result<Procedural2dSnapshot, store::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(&entry))).collect();
    Ok(Procedural2dSnapshot {
        fixture: FlowFixture { schema: parsed.schema, camera: camera_from_dsl(&parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text }.into(),
    })
}

/// 📜️ `.procedural2d` textual document — derive-engine grammar via `Procedural2dSnapshotDsl`.
impl store::ArtifactDsl for Procedural2dSnapshot {
    const EXTENSION: &'static str = "procedural2d";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural2dSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        procedural2d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Procedural2dSnapshotDsl as store::ArtifactDsl>::print_dsl(&procedural2d_document_to_dsl(self))
    }
}

/// 📦️ `.procedural2d` binary pack — same `Procedural2dSnapshotDsl` mirror as `ArtifactDsl` above;
/// `dsl::DslArtifact`'s derive already gives `Procedural2dSnapshotDsl` its own `ArtifactPack` impl.
impl store::ArtifactPack for Procedural2dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let document = procedural2d_document_to_dsl(self);
        let inner = store::pack_rt::encode_document(&Procedural2dSnapshotDsl::__dsl_spec(), &document.__dsl_to_record(), options)?;
        let mut bytes = Vec::with_capacity(4 + inner.len());
        bytes.extend_from_slice(b"P2D2");
        bytes.extend_from_slice(&inner);
        Ok(bytes)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if !bytes.starts_with(b"P2D2") {
            return Err(store::PackError::Schema("procedural2d pack discriminator mismatch".into()));
        }
        let (record, _report) = store::pack_rt::decode_document(&bytes[4..], &Procedural2dSnapshotDsl::__dsl_spec(), options)?;
        let parsed = Procedural2dSnapshotDsl::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;
        procedural2d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }
}
//#endregion 🔖️DslMirror

/// 📖️ Parses `.procedural2d` DSL text into a `Procedural2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Procedural2dSnapshot, store::TextError> {
    <Procedural2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Procedural2dSnapshot` back to `.procedural2d` DSL text.
pub fn print_dsl(document: &Procedural2dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural2d::PROCEDURAL_2D_SCHEMA;
    use semio_framework_os_kernel::os_store::test_support;
    use store::ArtifactDsl;

    //#region 🔖️DslTests
    #[test]
    fn dsl_round_trip_empty_projection() {
        test_support::assert_dsl_round_trip(&Procedural2dSnapshot::default());
        test_support::assert_dsl_pack_equivalence(&Procedural2dSnapshot::default());
    }

    #[test]
    fn dsl_round_trip_example_fixture() {
        let projection = Procedural2dSnapshot::parse_dsl(PROCEDURAL2D_EXAMPLE_TEXT).expect("parse 🌀️default.procedural2d fixture");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_with_generation_state() {
        let mut projection = Procedural2dSnapshot::default();
        let mut values = serde_json::Map::new();
        // 🌱️ A fractional literal, not a whole number: a whole-number float still normalizes to an
        // integer-backed `serde_json::Number` somewhere on this round trip — a real, engine-owned
        // behavior, not a bug in this crate's mirror/conversion code — so a whole-number input like
        // `3.0` would legitimately compare unequal to its round-tripped `3` here. `3.5` has no such
        // ambiguity.
        values.insert("count".into(), serde_json::json!(3.5));
        projection.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("generation-1".into());
        projection.generation.cold_builder_mut().expect("unique cold generation owner").preview_text = Some("42".into());
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_covers_every_widget_kind() {
        let mut projection = Procedural2dSnapshot::default();
        projection.fixture.widgets = vec![
            Widget::InputSlider { id: "slider".into(), label: "Number".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
            Widget::InputImage { id: "image".into(), src: "data:image/png;base64,abc".into() },
            Widget::Variable { id: "variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputAction { id: "action".into(), action: "export".into() },
            Widget::OutputExport { id: "export".into(), format: "svg".into() },
            Widget::Cluster { id: "cluster".into(), name: "Group".into(), tree: Default::default(), flow: Default::default() },
        ];
        projection.fixture.synapses = vec![];
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }
    //#endregion 🔖️DslTests

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law: proves `Procedural2dMutation`'s `Edit` round-trips through
    /// `protocol::MutationEnvelope`s beside this file's existing dsl/pack round-trip laws.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::procedural2d::op::Procedural2dMutation;
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<Procedural2dSnapshot, Procedural2dMutation> = ArtifactStore::new(create_document_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", Procedural2dSnapshot::default(), None)).expect("valid artifact store fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::procedural2d::op::replace_widget(Widget::InputNote { id: "note-9".into(), text: String::new() })], description: None }).expect("apply");
        let edit: &Edit<Procedural2dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        test_support::assert_command_envelope_round_trip::<Procedural2dSnapshot, Procedural2dMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests

    //#region 🔖️DslErrorTests
    #[test]
    fn dsl_parse_rejects_malformed_text() {
        let error = Procedural2dSnapshot::parse_dsl("schema=\"flow.fixture").unwrap_err();
        assert!(error.message.contains("unterminated string literal"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_missing_required_field() {
        let text = "camera { x=0 y=0 zoom=1 }\nwidgets { }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dSnapshot::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("found Absent"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_missing_camera_block() {
        let error = Procedural2dSnapshot::parse_dsl("schema=\"flow.fixture\"\n").unwrap_err();
        assert!(error.message.contains("expected Record, found Absent"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_unquoted_value_for_string_field() {
        let text = "schema=123\ncamera { x=0 y=0 zoom=1 }\nwidgets { }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dSnapshot::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected Text"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_non_numeric_value_for_number_field() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { input-slider id=\"s\" value=abc min=0 max=1 step=1 }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dSnapshot::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected a float"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_invalid_bool_value() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { neuron id=\"n\" neuron-kind=math.add preview=maybe input-ports= [ ] output-ports= [ ] params= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dSnapshot::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected 'true' or 'false'"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_malformed_value_literal() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { cluster id=\"n\" name=\"n\" tree=bogusvalue flow= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dSnapshot::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected a value literal"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_unknown_widget_kind() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { bogus id=\"n\" }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dSnapshot::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected RBrace"), "unexpected error: {}", error.message);
    }
    //#endregion 🔖️DslErrorTests
}
//#endregion 🧪️Tests
