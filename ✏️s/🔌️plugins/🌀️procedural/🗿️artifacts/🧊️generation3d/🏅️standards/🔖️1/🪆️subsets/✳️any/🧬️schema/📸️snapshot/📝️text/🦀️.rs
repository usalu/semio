//! 📜️ Generation3d artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! See `generation2d`'s sibling `🗣️dsl/🦀️.rs` docstring for why the `*Dsl` mirror types below
//! are LOCAL structural twins rather than derives on the foreign `flow`/`playbook` types directly.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};
use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};
use semio_framework_artifact_playbook_playbook::{FormGeneration, GenerationPlayState};
use std::collections::BTreeMap;

//#region 🔖️Examples
pub const GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT: &str = include_str!("../../../📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT: &str = include_str!("../../../📚️examples/📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT: &str = include_str!("../../../📚️examples/🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_BOX_FILLET_TEXT: &str = include_str!("../../../📚️examples/📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT: &str = include_str!("../../../📚️examples/🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT: &str = include_str!("../../../📚️examples/🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT: &str = include_str!("../../../📚️examples/🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_BOX_SHELL_TEXT: &str = include_str!("../../../📚️examples/🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_MESH_WORKBENCH_TEXT: &str = include_str!("../../../📚️examples/🥽️mesh-workbench/🖼️assets/🥽️mesh-workbench/🗣️.dsl.semio");
//#endregion 🔖️Examples

//#region 🔖️DslMirror
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct ValueDsl {
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

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct FormGenerationDsl {
    id: String,
    name: String,
    values: BTreeMap<String, semio_framework_value::DslValue>,
}

pub fn form_generation_to_dsl(generation: &FormGeneration) -> FormGenerationDsl {
    FormGenerationDsl { id: generation.id.clone(), name: generation.name.clone(), values: generation.values.iter().map(|(key, value)| (key.clone(), semio_framework_value::ToValue::to_value(value))).collect() }
}

pub fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {
    FormGeneration { id: generation.id, name: generation.name, values: generation.values.into_iter().filter_map(|(key, value)| semio_framework_value::FromValue::from_value(value).ok().map(|json| (key, json))).collect() }
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(id = "procedural.generation3d", layout = "lines")]
struct Generation3dSnapshotDsl {
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
/// 📐️ Projects the original document text declaration under the retained operation's admission.
pub(crate) fn generation3d_document_spec_controlled(control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{Generation3dSnapshotDsl::__dsl_spec_controlled(control)}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Generation3dSnapshotDsl {
    const EXTENSION: &'static str = "generation3d";
    fn envelope_id() -> &'static str {
        "procedural.generation3d"
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

impl store::ArtifactPack for Generation3dSnapshotDsl {
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

fn generation3d_document_to_dsl(document: &Generation3dSnapshot) -> Generation3dSnapshotDsl {
    let host_snapshot = &document.host_snapshot;
    let generation = &document.generation;
    Generation3dSnapshotDsl {
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

//#region 🔎️OriginalTextSource
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource, FieldProjectionView as ProjectionView, projection_path_error};
use semio_framework_artifact_flow_flow::{FlowUi, FlowNodeGui, NodeChrome, FlowPreviewGui, FlowChannelRef};
use semio_framework_artifact_flow_flow::neural::{Tree, Neuron, Synapse};
use semio_framework_value::ordered::{OrderedMap, OrderedSet};
use semio_framework_value::{DslValue, Number, ValueError};

enum SnapshotTextBlock<'a> { Camera(&'a CameraJson), Widgets(&'a [Widget]), Value(&'a NeuralValue) }
enum SnapshotTextNode<'a> {
    Root(&'a Generation3dSnapshot), Leaf(ProjectionView<'a>), Block(SnapshotTextBlock<'a>),
    Camera(&'a CameraJson), Widgets(&'a [Widget]), Widget(&'a Widget), Strings(&'a [String]), Set(&'a OrderedSet),
    Entries(&'a Dictionary), Entry(&'a str, &'a NeuralValue), Value(&'a NeuralValue),
    Synapses(&'a [SynapseSpec]), Synapse(&'a SynapseSpec), Wire(&'a SynapseSpec),
    Layouts(&'a OrderedMap<WidgetLayout>), Layout(&'a WidgetLayout),
    Generations(&'a [FormGeneration]), Generation(&'a FormGeneration), Values(&'a OrderedMap<DslValue>),
    Intrinsic(&'a DslValue), IntrinsicValue(&'a NeuralValue), IntrinsicDictionary(&'a Dictionary),
    Tree(&'a Tree), Neurons(&'a [Neuron]), Neuron(&'a Neuron), Edges(&'a [Synapse]), Edge(&'a Synapse),
    Flow(&'a FlowUi), Nodes(&'a OrderedMap<FlowNodeGui>), Node(&'a FlowNodeGui), Chrome(&'a NodeChrome),
    Previews(&'a [FlowPreviewGui]), Preview(&'a FlowPreviewGui), Channel(&'a FlowChannelRef),
    IntrinsicCamera(&'a CameraJson), IntrinsicLayout(&'a WidgetLayout), IntrinsicSet(&'a OrderedSet), EmptyObject,
}

fn snapshot_widget_keyword(widget: &Widget) -> &'static str {
    match widget { Widget::Neuron { .. } => "neuron", Widget::InputSlider { .. } => "input-slider", Widget::InputNote { .. } => "input-note", Widget::InputImage { .. } => "input-image", Widget::Variable { .. } => "variable", Widget::OutputPreview { .. } => "output-preview", Widget::OutputAction { .. } => "output-action", Widget::OutputExport { .. } => "output-export", Widget::Cluster { .. } => "cluster" }
}

impl<'a> SnapshotTextNode<'a> {
    fn text(value: &'a str) -> Self { Self::Leaf(ProjectionView::Text(value)) }
    fn intrinsic_text(value: &'a str) -> Self { Self::Leaf(ProjectionView::IntrinsicText(value)) }
    fn float(value: f64) -> Self { Self::Leaf(ProjectionView::Float(value)) }
    fn intrinsic_float(value: f64) -> Self { Self::Leaf(ProjectionView::IntrinsicNumber(Number::Float(value))) }
    fn optional_text(value: Option<&'a str>) -> Self { Self::Leaf(value.map(ProjectionView::Text).unwrap_or(ProjectionView::Absent)) }
    fn view(self) -> Result<ProjectionView<'a>, ValueError> {
        use ProjectionView as V;
        Ok(match self {
            Self::Root(_) => V::Record(&[0,1,2,3,4,5,6,7]), Self::Leaf(value) => value, Self::Block(_) => V::Block,
            Self::Camera(_) => V::Record(&[0,1,2]), Self::Widgets(values) => V::Statements(values.len()),
            Self::Widget(value) => V::Record(match value { Widget::Neuron { .. } | Widget::InputSlider { .. } => &[0,1,2,3,4,5], Widget::InputNote { .. } | Widget::InputImage { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => &[0,1], Widget::Variable { .. } | Widget::OutputPreview { .. } => &[0,1,2], Widget::Cluster { .. } => &[0,1,2,3] }),
            Self::Strings(values) => V::List(values.len()), Self::Set(values) => V::List(values.len()), Self::Entries(values) => V::List(values.len()), Self::Entry(..) => V::Record(&[0,1]), Self::Value(_) => V::Record(&[0,1,2,3,4,5]),
            Self::Synapses(values) => V::List(values.len()), Self::Synapse(_) => V::Record(&[0,1]), Self::Wire(_) => V::Wire(Some(true)), Self::Layouts(values) => V::Map(values.len()), Self::Layout(_) => V::Record(&[0,1]),
            Self::Generations(values) => V::List(values.len()), Self::Generation(_) => V::Record(&[0,1,2]), Self::Values(values) => V::Map(values.len()),
            Self::Intrinsic(value) => match value { DslValue::Null => V::IntrinsicNull, DslValue::Bool(value) => V::IntrinsicBool(*value), DslValue::Number(value) => V::IntrinsicNumber(*value), DslValue::String(value) => V::IntrinsicText(value), DslValue::Bytes(value) => V::IntrinsicBytes(value), DslValue::Array(values) => V::IntrinsicArray(values.len()), DslValue::Object(values) => V::IntrinsicObject(values.len()) },
            Self::IntrinsicValue(value) => match value { NeuralValue::Atom(Atom::Null) => V::IntrinsicNull, NeuralValue::Atom(Atom::Boolean(value)) => V::IntrinsicBool(*value), NeuralValue::Atom(Atom::Integer(value)) => V::IntrinsicNumber(Number::Int(*value)), NeuralValue::Atom(Atom::Decimal(value)) => V::IntrinsicNumber(Number::Float(*value)), NeuralValue::Atom(Atom::String(value)) => V::IntrinsicText(value), NeuralValue::Dictionary(values) => V::IntrinsicObject(values.len()) },
            Self::IntrinsicDictionary(values) => V::IntrinsicObject(values.len()), Self::Tree(_) => V::IntrinsicObject(2), Self::Neurons(values) => V::IntrinsicArray(values.len()), Self::Neuron(_) => V::IntrinsicObject(4), Self::Edges(values) => V::IntrinsicArray(values.len()), Self::Edge(_) => V::IntrinsicObject(5),
            Self::Flow(_) => V::IntrinsicObject(3), Self::Nodes(values) => V::IntrinsicObject(values.len()), Self::Node(_) => V::IntrinsicObject(2), Self::Chrome(value) => V::IntrinsicObject(match value { NodeChrome::Plain { .. } | NodeChrome::Note { .. } | NodeChrome::Image { .. } => 2, NodeChrome::Slider { .. } => 6, NodeChrome::Variable { .. } => 3 }),
            Self::Previews(values) => V::IntrinsicArray(values.len()), Self::Preview(_) => V::IntrinsicObject(6), Self::Channel(_) => V::IntrinsicObject(2), Self::IntrinsicCamera(_) => V::IntrinsicObject(3), Self::IntrinsicLayout(_) => V::IntrinsicObject(2), Self::IntrinsicSet(values) => V::IntrinsicArray(values.len()), Self::EmptyObject => V::IntrinsicObject(0),
        })
    }
    fn child(self, index: usize) -> Result<Self, ValueError> {
        use ProjectionView as V;
        Ok(match self {
            Self::Root(source) => match index { 0 => Self::text(&source.host_snapshot.schema), 1 => Self::Block(SnapshotTextBlock::Camera(&source.host_snapshot.camera)), 2 => Self::Block(SnapshotTextBlock::Widgets(&source.host_snapshot.widgets)), 3 => Self::Synapses(&source.host_snapshot.synapses), 4 => Self::Layouts(&source.host_snapshot.layout), 5 => Self::optional_text(source.generation.selected_generation_id.as_deref()), 6 => Self::optional_text(source.generation.preview_text.as_deref()), 7 => Self::Generations(&source.generation.generations), _ => return Err(projection_path_error()) },
            Self::Block(value) if index == 0 => match value { SnapshotTextBlock::Camera(value) => Self::Camera(value), SnapshotTextBlock::Widgets(value) => Self::Widgets(value), SnapshotTextBlock::Value(value) => Self::Value(value) },
            Self::Camera(value) => Self::float(match index { 0 => value.x, 1 => value.y, 2 => value.zoom, _ => return Err(projection_path_error()) }),
            Self::Widgets(values) => Self::Widget(values.get(index).ok_or_else(projection_path_error)?),
            Self::Widget(value) => match (value,index) {
                (Widget::Neuron { id, .. },0) | (Widget::InputSlider { id, .. },0) | (Widget::InputNote { id, .. },0) | (Widget::InputImage { id, .. },0) | (Widget::Variable { id, .. },0) | (Widget::OutputPreview { id, .. },0) | (Widget::OutputAction { id, .. },0) | (Widget::OutputExport { id, .. },0) | (Widget::Cluster { id, .. },0) => Self::text(id),
                (Widget::Neuron { neuron_kind, .. },1) => Self::text(neuron_kind), (Widget::Neuron { preview, .. },2) => Self::Leaf(V::Bool(*preview)), (Widget::Neuron { input_ports, .. },3) => Self::Strings(input_ports), (Widget::Neuron { output_ports, .. },4) => Self::Strings(output_ports), (Widget::Neuron { params, .. },5) => Self::Entries(params),
                (Widget::InputSlider { label, .. },1) => Self::text(label), (Widget::InputSlider { value, .. },2) => Self::float(*value), (Widget::InputSlider { min, .. },3) => Self::float(*min), (Widget::InputSlider { max, .. },4) => Self::float(*max), (Widget::InputSlider { step, .. },5) => Self::float(*step),
                (Widget::InputNote { text, .. },1) => Self::text(text), (Widget::InputImage { src, .. },1) => Self::text(src), (Widget::Variable { name, .. },1) => Self::text(name), (Widget::Variable { schema, .. },2) => Self::text(schema),
                (Widget::OutputPreview { preview, .. },1) => Self::Entries(preview), (Widget::OutputPreview { expanded, .. },2) => Self::Set(expanded), (Widget::OutputAction { action, .. },1) => Self::text(action), (Widget::OutputExport { format, .. },1) => Self::text(format),
                (Widget::Cluster { name, .. },1) => Self::text(name), (Widget::Cluster { tree, .. },2) => Self::Tree(tree), (Widget::Cluster { flow, .. },3) => Self::Flow(flow), _ => return Err(projection_path_error()),
            },
            Self::Strings(values) => Self::text(values.get(index).ok_or_else(projection_path_error)?), Self::Set(values) => Self::text(values.key_at_rank(index).ok_or_else(projection_path_error)?),
            Self::Entries(values) => { let (key,value)=values.entry_at_rank(index).ok_or_else(projection_path_error)?; Self::Entry(key,value) }, Self::Entry(key,value) => match index { 0 => Self::text(key), 1 => Self::Block(SnapshotTextBlock::Value(value)), _ => return Err(projection_path_error()) },
            Self::Value(value) => match (value,index) { (NeuralValue::Atom(Atom::Null),0) => Self::Leaf(V::Bool(true)), (NeuralValue::Atom(Atom::Boolean(value)),1) => Self::Leaf(V::Bool(*value)), (NeuralValue::Atom(Atom::Integer(value)),2) => Self::Leaf(V::Int(*value)), (NeuralValue::Atom(Atom::Decimal(value)),3) => Self::float(*value), (NeuralValue::Atom(Atom::String(value)),4) => Self::text(value), (NeuralValue::Dictionary(values),5) => Self::Entries(values), (_,0..=5) => Self::Leaf(V::Absent), _ => return Err(projection_path_error()) },
            Self::Synapses(values) => Self::Synapse(values.get(index).ok_or_else(projection_path_error)?), Self::Synapse(value) => match index { 0 => Self::text(&value.id), 1 => Self::Wire(value), _ => return Err(projection_path_error()) },
            Self::Wire(value) => match index { 0 => Self::text(&value.from), 2 => Self::optional_text((!value.from_port.is_empty()).then_some(value.from_port.as_str())), 3 => Self::text(&value.to), 5 => Self::optional_text((!value.to_port.is_empty()).then_some(value.to_port.as_str())), 1|4|6|7 => Self::Leaf(V::Absent), 8 => Self::EmptyObject, _ => return Err(projection_path_error()) },
            Self::Layouts(values) => Self::Layout(values.entry_at_rank(index).ok_or_else(projection_path_error)?.1), Self::Layout(value) => Self::float(match index { 0 => value.x, 1 => value.y, _ => return Err(projection_path_error()) }),
            Self::Generations(values) => Self::Generation(values.get(index).ok_or_else(projection_path_error)?), Self::Generation(value) => match index { 0 => Self::text(&value.id), 1 => Self::text(&value.name), 2 => Self::Values(&value.values), _ => return Err(projection_path_error()) }, Self::Values(values) => Self::Intrinsic(values.entry_at_rank(index).ok_or_else(projection_path_error)?.1),
            Self::Intrinsic(value) => Self::Intrinsic(match value { DslValue::Array(values) => values.get(index).ok_or_else(projection_path_error)?, DslValue::Object(values) => &values.get(index).ok_or_else(projection_path_error)?.1, _ => return Err(projection_path_error()) }),
            Self::IntrinsicValue(NeuralValue::Dictionary(values)) | Self::IntrinsicDictionary(values) => Self::IntrinsicValue(values.entry_at_rank(index).ok_or_else(projection_path_error)?.1),
            Self::Tree(value) => match index { 0 => Self::Neurons(&value.neurons), 1 => Self::Edges(&value.synapses), _ => return Err(projection_path_error()) }, Self::Neurons(values) => Self::Neuron(values.get(index).ok_or_else(projection_path_error)?), Self::Neuron(value) => match index { 0 => Self::intrinsic_text(&value.id), 1 => Self::intrinsic_text(&value.kind), 2 => Self::IntrinsicDictionary(&value.params), 3 => value.tree.as_deref().map(Self::Tree).unwrap_or(Self::Leaf(V::IntrinsicNull)), _ => return Err(projection_path_error()) }, Self::Edges(values) => Self::Edge(values.get(index).ok_or_else(projection_path_error)?), Self::Edge(value) => Self::intrinsic_text(match index { 0 => &value.id, 1 => &value.from, 2 => &value.to, 3 => &value.from_port, 4 => &value.to_port, _ => return Err(projection_path_error()) }),
            Self::Flow(value) => match index { 0 => Self::IntrinsicCamera(&value.camera), 1 => Self::Nodes(&value.nodes), 2 => Self::Previews(&value.previews), _ => return Err(projection_path_error()) }, Self::Nodes(values) => Self::Node(values.entry_at_rank(index).ok_or_else(projection_path_error)?.1), Self::Node(value) => match index { 0 => Self::IntrinsicLayout(&value.layout), 1 => Self::Chrome(&value.chrome), _ => return Err(projection_path_error()) },
            Self::Chrome(value) => match (value,index) { (NodeChrome::Plain { .. },0) => Self::intrinsic_text("plain"), (NodeChrome::Slider { .. },0) => Self::intrinsic_text("slider"), (NodeChrome::Note { .. },0) => Self::intrinsic_text("note"), (NodeChrome::Image { .. },0) => Self::intrinsic_text("image"), (NodeChrome::Variable { .. },0) => Self::intrinsic_text("variable"), (NodeChrome::Plain { preview },1) => Self::Leaf(V::IntrinsicBool(*preview)), (NodeChrome::Slider { label, .. },1) => Self::intrinsic_text(label), (NodeChrome::Slider { min, .. },2) => Self::intrinsic_float(*min), (NodeChrome::Slider { max, .. },3) => Self::intrinsic_float(*max), (NodeChrome::Slider { step, .. },4) => Self::intrinsic_float(*step), (NodeChrome::Slider { value, .. },5) => Self::intrinsic_float(*value), (NodeChrome::Note { text },1) => Self::intrinsic_text(text), (NodeChrome::Image { src },1) => Self::intrinsic_text(src), (NodeChrome::Variable { name, .. },1) => Self::intrinsic_text(name), (NodeChrome::Variable { schema, .. },2) => Self::intrinsic_text(schema), _ => return Err(projection_path_error()) },
            Self::Previews(values) => Self::Preview(values.get(index).ok_or_else(projection_path_error)?), Self::Preview(value) => match index { 0 => Self::intrinsic_text(&value.id), 1 => value.source.as_ref().map(Self::Channel).unwrap_or(Self::Leaf(V::IntrinsicNull)), 2 => Self::intrinsic_text(&value.mode), 3 => Self::IntrinsicDictionary(&value.preview), 4 => Self::IntrinsicSet(&value.expanded), 5 => value.layout.as_ref().map(Self::IntrinsicLayout).unwrap_or(Self::Leaf(V::IntrinsicNull)), _ => return Err(projection_path_error()) }, Self::Channel(value) => Self::intrinsic_text(match index { 0 => &value.neuron, 1 => &value.channel, _ => return Err(projection_path_error()) }),
            Self::IntrinsicCamera(value) => Self::intrinsic_float(match index { 0 => value.x, 1 => value.y, 2 => value.zoom, _ => return Err(projection_path_error()) }), Self::IntrinsicLayout(value) => Self::intrinsic_float(match index { 0 => value.x, 1 => value.y, _ => return Err(projection_path_error()) }), Self::IntrinsicSet(values) => Self::intrinsic_text(values.key_at_rank(index).ok_or_else(projection_path_error)?),
            _ => return Err(projection_path_error()),
        })
    }
    fn key(self, index: usize) -> Result<&'a str, ValueError> {
        let keys: &[&str] = match self {
            Self::Widgets(values) => return values.get(index).map(snapshot_widget_keyword).ok_or_else(projection_path_error), Self::Layouts(values) => return values.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(projection_path_error), Self::Values(values) => return values.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(projection_path_error),
            Self::Intrinsic(DslValue::Object(values)) => return values.get(index).map(|(key,_)|key.as_str()).ok_or_else(projection_path_error), Self::IntrinsicValue(NeuralValue::Dictionary(values)) | Self::IntrinsicDictionary(values) => return values.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(projection_path_error),
            Self::Tree(_) => &["neurons","synapses"], Self::Neuron(_) => &["id","kind","params","tree"], Self::Edge(_) => &["id","from","to","fromPort","toPort"], Self::Flow(_) => &["camera","nodes","previews"], Self::Nodes(values) => return values.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(projection_path_error), Self::Node(_) => &["layout","chrome"],
            Self::Chrome(value) => match value { NodeChrome::Plain { .. } => &["kind","preview"], NodeChrome::Slider { .. } => &["kind","label","min","max","step","value"], NodeChrome::Note { .. } => &["kind","text"], NodeChrome::Image { .. } => &["kind","src"], NodeChrome::Variable { .. } => &["kind","name","schema"] },
            Self::Preview(_) => &["id","source","mode","preview","expanded","layout"], Self::Channel(_) => &["neuron","channel"], Self::IntrinsicCamera(_) => &["x","y","zoom"], Self::IntrinsicLayout(_) => &["x","y"], _ => return Err(projection_path_error()),
        };
        keys.get(index).copied().ok_or_else(projection_path_error)
    }
}

impl FieldProjectionSource for Generation3dSnapshot {
    fn projection_view(&self, path: &[usize]) -> Result<ProjectionView<'_>, ValueError> { let mut node=SnapshotTextNode::Root(self); for &index in path { node=node.child(index)?; } node.view() }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, ValueError> { let mut node=SnapshotTextNode::Root(self); for &index in path { node=node.child(index)?; } node.key(index) }
}
//#endregion 🔎️OriginalTextSource

fn generation3d_document_from_dsl(parsed: Generation3dSnapshotDsl) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(&entry))).collect();
    Ok(Generation3dSnapshot {
        host_snapshot: FlowHostSnapshot { schema: parsed.schema, camera: camera_from_dsl(&parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text }.into(),
    })
}

impl store::ArtifactDsl for Generation3dSnapshot {
    const EXTENSION: &'static str = "generation3d";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = <Generation3dSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        generation3d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Generation3dSnapshotDsl as store::ArtifactDsl>::print_dsl(&generation3d_document_to_dsl(self))
    }
}

impl store::ArtifactPack for Generation3dSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Generation3dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&generation3d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Generation3dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        generation3d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Generation3dSnapshotDsl as store::ArtifactPack>::record_spec()
    }
}
//#endregion 🔖️DslMirror

/// 📖️ Parses `.generation3d` DSL text into a `Generation3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    <Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Generation3dSnapshot` back to `.generation3d` DSL text.
pub fn print_dsl(document: &Generation3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Generation3dSnapshotText = String;

type ControlledSnapshotDsl = Generation3dSnapshotDsl;
type ControlledSnapshot = Generation3dSnapshot;
const CONTROLLED_ENVELOPE_ID: &str = "procedural.generation3d";
const CONTROLLED_PACK_PREFIX: Option<&[u8]> = None;
#[path="../../../../../../../../../🫀️core/🧬️generation/🪶️sqlite/🚦️native/🦀️.rs"]mod controlled;
pub(crate) use controlled::{decode as decode_sqlite_native,encode as encode_sqlite_native};
//#endregion 🚚️Carrier
