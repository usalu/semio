//! 📜️ Procedural3d artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! See `procedural2d`'s sibling `🗣️dsl/🦀️component.rs` docstring for why the `*Dsl` mirror types below
//! are LOCAL structural twins rather than derives on the foreign `flow`/`playbook` types directly.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::neural::{Atom, Dictionary, Value as NeuralValue};
use flow::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use flow::playbook::{FormGeneration, GenerationPlayState};
use std::collections::BTreeMap;

//#region 🔖️Examples
pub const PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT: &str = include_str!("../../../📚️examples/🎬️hexagonal-mushroom-column/🖼️assets/🗣️hexagonal-mushroom-column.dsl.semio");
pub const PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT: &str = include_str!("../../../📚️examples/🎬️rectangle-extrude-volume/🖼️assets/🗣️rectangle-extrude-volume.dsl.semio");
pub const PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT: &str = include_str!("../../../📚️examples/🎬️sphere-cut-with-torus/🖼️assets/🗣️sphere-cut-with-torus.dsl.semio");
pub const PROCEDURAL3D_EXAMPLE_BOX_FILLET_TEXT: &str = include_str!("../../../📚️examples/🎬️box-fillet-preview/🖼️assets/🗣️box-fillet-preview.dsl.semio");
pub const PROCEDURAL3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT: &str = include_str!("../../../📚️examples/🎬️sphere-box-fuse/🖼️assets/🗣️sphere-box-fuse.dsl.semio");
pub const PROCEDURAL3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT: &str = include_str!("../../../📚️examples/🎬️face-sweep-extrude/🖼️assets/🗣️face-sweep-extrude.dsl.semio");
pub const PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT: &str = include_str!("../../../📚️examples/🎬️rectangle-wire-preview/🖼️assets/🗣️rectangle-wire-preview.dsl.semio");
pub const PROCEDURAL3D_EXAMPLE_BOX_SHELL_TEXT: &str = include_str!("../../../📚️examples/🎬️box-shell-preview/🖼️assets/🗣️box-shell-preview.dsl.semio");
//#endregion 🔖️Examples

//#region 🔖️DslMirror
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct ValueDsl {
    null: Option<bool>,
    #[dsl(key = "bool")]
    boolean: Option<bool>,
    #[dsl(key = "int")]
    integer: Option<i64>,
    decimal: Option<f64>,
    text: Option<String>,
    #[dsl(key = "dict")]
    dictionary: Option<Vec<DictEntryDsl>>}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct DictEntryDsl {
    key: String,
    #[dsl(block)]
    value: ValueDsl}

async fn value_to_value_dsl(value: &NeuralValue) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        NeuralValue::Atom(Atom::Null) => dsl_value.null = Some(true),
        NeuralValue::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        NeuralValue::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        NeuralValue::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        NeuralValue::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        NeuralValue::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_entries(dict))}
    dsl_value
}

async fn value_dsl_to_value(dsl_value: &ValueDsl) -> NeuralValue {
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
        None => NeuralValue::Atom(Atom::Null)}
}

pub async fn dictionary_to_value_dsl_entries(dict: &Dictionary) -> Vec<DictEntryDsl> {
    dict.keys().map(|key| DictEntryDsl { key: key.clone(), value: value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")) }).collect()
}

pub async fn value_dsl_entries_to_dictionary(entries: &[DictEntryDsl]) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, entry| dict.insert(entry.key.clone(), value_dsl_to_value(&entry.value)))
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct CameraJsonDsl {
    x: f64,
    y: f64,
    zoom: f64}

pub async fn camera_to_dsl(camera: &CameraJson) -> CameraJsonDsl {
    CameraJsonDsl { x: camera.x, y: camera.y, zoom: camera.zoom }
}

pub async fn camera_from_dsl(camera: &CameraJsonDsl) -> CameraJson {
    CameraJson { x: camera.x, y: camera.y, zoom: camera.zoom }
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct WidgetLayoutDsl {
    x: f64,
    y: f64}

pub async fn layout_to_dsl(layout: &WidgetLayout) -> WidgetLayoutDsl {
    WidgetLayoutDsl { x: layout.x, y: layout.y }
}

pub async fn layout_from_dsl(layout: &WidgetLayoutDsl) -> WidgetLayout {
    WidgetLayout { x: layout.x, y: layout.y }
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct SynapseSpecDsl {
    id: String,
    wire: dsl::Wire}

pub async fn synapse_to_dsl(synapse: &SynapseSpec) -> SynapseSpecDsl {
    SynapseSpecDsl {
        id: synapse.id.clone(),
        wire: dsl::Wire(dsl::WireValue {
            from: dsl::WireNode { id: synapse.from.clone(), kind: None, port: (!synapse.from_port.is_empty()).then(|| synapse.from_port.clone()) },
            edge: Some((true, dsl::WireNode { id: synapse.to.clone(), kind: None, port: (!synapse.to_port.is_empty()).then(|| synapse.to_port.clone()) })),
            edge_label: dsl::WireEdgeLabel::default(),
            properties: dsl::DslValue::Object(Vec::new())})}
}

pub async fn synapse_from_dsl(synapse: SynapseSpecDsl) -> SynapseSpec {
    let wire = synapse.wire.0;
    let to = wire.edge.map(|(_, to)| to).unwrap_or_default();
    SynapseSpec { id: synapse.id, from: wire.from.id, to: to.id, from_port: wire.from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() }
}

#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum WidgetDsl {
    Neuron {
        id: String,
        neuron_kind: String,
        preview: bool,
        input_ports: Vec<String>,
        output_ports: Vec<String>,
        #[dsl(table)]
        params: Vec<DictEntryDsl>},
    InputSlider {
        id: String,
        value: f64,
        min: f64,
        max: f64,
        step: f64},
    InputNote {
        id: String,
        text: String},
    InputImage {
        id: String,
        src: String},
    Variable {
        id: String,
        name: String,
        schema: String},
    OutputPreview {
        id: String,
        #[dsl(table)]
        preview: Vec<DictEntryDsl>,
        expanded: Vec<String>},
    OutputAction {
        id: String,
        action: String},
    OutputExport {
        id: String,
        format: String},
    Cluster {
        id: String,
        name: String,
        tree: dsl::DslValue,
        flow: dsl::DslValue}}

pub async fn widget_to_dsl(widget: &Widget) -> WidgetDsl {
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
        Widget::Cluster { id, name, tree, flow } => WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: dsl::to_dsl_value(tree).unwrap_or(dsl::DslValue::Null), flow: dsl::to_dsl_value(flow).unwrap_or(dsl::DslValue::Null) }}
}

pub async fn widget_from_dsl(widget: WidgetDsl) -> Result<Widget, store::TextError> {
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
            flow: dsl::from_dsl_value(flow).map_err(|error| store::TextError::new(format!("invalid cluster flow: {error}"), store::TextSpan::at(1, 1)))?}})
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct FormGenerationDsl {
    id: String,
    name: String,
    values: BTreeMap<String, dsl::DslValue>}

pub async fn form_generation_to_dsl(generation: &FormGeneration) -> FormGenerationDsl {
    FormGenerationDsl { id: generation.id.clone(), name: generation.name.clone(), values: generation.values.iter().map(|(key, value)| (key.clone(), dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null))).collect() }
}

pub async fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {
    FormGeneration { id: generation.id, name: generation.name, values: generation.values.into_iter().filter_map(|(key, value)| dsl::from_dsl_value(value).ok().map(|json| (key, json))).collect() }
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(id = "procedural.procedural3d", layout = "lines")]
struct Procedural3dSnapshotDsl {
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
    generations: Vec<FormGenerationDsl>}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Procedural3dSnapshotDsl {
    const EXTENSION: &'static str = "procedural3d";
    async fn envelope_id() -> &'static str { "procedural.procedural3d" }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text};
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Procedural3dSnapshotDsl {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedArtifactCodecs




async fn procedural3d_document_to_dsl(document: &Procedural3dSnapshot) -> Procedural3dSnapshotDsl {
    let fixture = &document.fixture;
    let generation = &document.generation;
    Procedural3dSnapshotDsl {
        schema: fixture.schema.clone(),
        camera: camera_to_dsl(&fixture.camera),
        widgets: fixture.widgets.iter().map(widget_to_dsl).collect(),
        synapses: fixture.synapses.iter().map(synapse_to_dsl).collect(),
        layout: fixture.layout.iter().map(|(id, entry)| (id.clone(), layout_to_dsl(entry))).collect(),
        selected_generation_id: generation.selected_generation_id.clone(),
        preview_text: generation.preview_text.clone(),
        generations: generation.generations.iter().map(form_generation_to_dsl).collect()}
}

async fn procedural3d_document_from_dsl(parsed: Procedural3dSnapshotDsl) -> Result<Procedural3dSnapshot, store::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(&entry))).collect();
    Ok(Procedural3dSnapshot {
        fixture: FlowFixture { schema: parsed.schema, camera: camera_from_dsl(&parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text }})
}

impl store::ArtifactDsl for Procedural3dSnapshot {
    const EXTENSION: &'static str = "procedural3d";

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural3dSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        procedural3d_document_from_dsl(parsed)
    }

    async fn print_dsl(&self) -> String {
        <Procedural3dSnapshotDsl as store::ArtifactDsl>::print_dsl(&procedural3d_document_to_dsl(self))
    }
}

impl store::ArtifactPack for Procedural3dSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Procedural3dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&procedural3d_document_to_dsl(self), options)
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Procedural3dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        procedural3d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }
}
//#endregion 🔖️DslMirror

/// 📖️ Parses `.procedural3d` DSL text into a `Procedural3dSnapshot`.
pub async fn parse_dsl(text: &str) -> Result<Procedural3dSnapshot, store::TextError> {
    <Procedural3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Procedural3dSnapshot` back to `.procedural3d` DSL text.
pub async fn print_dsl(document: &Procedural3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA;
    use semio_framework_os_kernel::os_store::test_support;
    use store::{ArtifactDsl};

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trip_empty_projection() {
        test_support::assert_dsl_round_trip(&Procedural3dSnapshot::default());
        test_support::assert_dsl_pack_equivalence(&Procedural3dSnapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_round_trip_every_bundled_example() {
        for text in [
            PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT,
            PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT,
            PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT,
            PROCEDURAL3D_EXAMPLE_BOX_FILLET_TEXT,
            PROCEDURAL3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT,
            PROCEDURAL3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT,
            PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT,
            PROCEDURAL3D_EXAMPLE_BOX_SHELL_TEXT,
        ] {
            let projection = Procedural3dSnapshot::parse_dsl(text).expect("parse bundled example");
            test_support::assert_dsl_round_trip(&projection);
            test_support::assert_dsl_pack_equivalence(&projection);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::procedural3d::op::Procedural3dMutation;
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<Procedural3dSnapshot, Procedural3dMutation> = ArtifactStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", Procedural3dSnapshot::default(), None)).expect("valid artifact store fixture");
        use crate::artifacts::procedural3d::mutations::create_widget::mutation::CreateWidget;
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Procedural3dMutation::CreateWidget(CreateWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } })], description: None }).expect("apply");
        let edit: &Edit<Procedural3dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        test_support::assert_command_envelope_round_trip::<Procedural3dSnapshot, Procedural3dMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
}
//#endregion 🧪️Tests
