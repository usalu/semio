//! 📜️ Generation3d artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! See `generation2d`'s sibling `🗣️dsl/🦀️.rs` docstring for why the `*Dsl` mirror types below
//! are LOCAL structural twins rather than derives on the foreign `flow`/`playbook` types directly.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
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
pub use semio_s_procedural_core_generation_sqlite_native::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl,
};

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(id = "procedural.generation3d", layout = "lines")]
pub struct Generation3dSnapshotDsl {
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
pub fn generation3d_document_spec_controlled(control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordSpec,ValueError>{Generation3dSnapshotDsl::__dsl_spec_controlled(control)}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl pack::snapshot::ArtifactDsl for Generation3dSnapshotDsl {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}


//#endregion 🔖️HandcraftedArtifactCodecs

pub fn generation3d_document_to_dsl(document: &Generation3dSnapshot) -> Generation3dSnapshotDsl {
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

pub fn generation3d_document_from_dsl(parsed: Generation3dSnapshotDsl) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(&entry))).collect();
    Ok(Generation3dSnapshot {
        host_snapshot: FlowHostSnapshot { schema: parsed.schema, camera: camera_from_dsl(&parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text }.into(),
    })
}

impl pack::snapshot::ArtifactDsl for Generation3dSnapshot {
    const EXTENSION: &'static str = "generation3d";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = <Generation3dSnapshotDsl as pack::snapshot::ArtifactDsl>::parse_dsl(text)?;
        generation3d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Generation3dSnapshotDsl as pack::snapshot::ArtifactDsl>::print_dsl(&generation3d_document_to_dsl(self))
    }
}


//#endregion 🔖️DslMirror

/// 📖️ Parses `.generation3d` DSL text into a `Generation3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    <Generation3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Generation3dSnapshot` back to `.generation3d` DSL text.
pub fn print_dsl(document: &Generation3dSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Generation3dSnapshotText = String;

struct Generation3dNative;

impl semio_s_procedural_core_generation_sqlite_native::GenerationNative for Generation3dNative {
    type Snapshot = Generation3dSnapshot;
    const ENVELOPE_ID: &'static str = "procedural.generation3d";
    const PACK_PREFIX: Option<&'static [u8]> = None;
    fn spec_producer() -> semio_framework_dsl_record::RecordSpecProducer {
        Generation3dSnapshotDsl::__dsl_spec_producer()
    }
    fn fields(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<semio_s_procedural_core_generation_sqlite_native::GenerationFields, semio_framework_value::ValueError> {
        let parsed = Generation3dSnapshotDsl::__dsl_from_record_controlled(record, control)?;
        Ok(semio_s_procedural_core_generation_sqlite_native::GenerationFields {
            schema: parsed.schema,
            camera: parsed.camera,
            widgets: parsed.widgets,
            synapses: parsed.synapses,
            layout: parsed.layout,
            selected_generation_id: parsed.selected_generation_id,
            preview_text: parsed.preview_text,
            generations: parsed.generations,
        })
    }
    fn host(snapshot: &Generation3dSnapshot) -> &FlowHostSnapshot {
        &snapshot.host_snapshot
    }
    fn state(snapshot: &Generation3dSnapshot) -> &GenerationPlayState {
        snapshot.generation.as_state()
    }
    fn assemble(host_snapshot: FlowHostSnapshot, generation: semio_framework_artifact_playbook_playbook::GenerationPlayRoot) -> Generation3dSnapshot {
        Generation3dSnapshot { host_snapshot, generation }
    }
}

pub fn encode_sqlite_native(document: &Generation3dSnapshot, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, semio_framework_value::ValueError> {
    semio_s_procedural_core_generation_sqlite_native::encode::<Generation3dNative>(document, encoding, control, native_owner)
}

pub fn decode_sqlite_native(payload: &store::io_schema::IoPayload, control: &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Generation3dSnapshot, semio_framework_value::ValueError> {
    semio_s_procedural_core_generation_sqlite_native::decode::<Generation3dNative>(payload, control, native_owner)
}
//#endregion 🚚️Carrier

mod source_examples {
use super::*;
use semio_s_artifact_procedural_3d_model::*;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use pack::snapshot::ArtifactDsl;
/// 📄️ The `procedural3d-play` "default" document — parsed from the bundled "hexagonal mushroom
/// column" example host_snapshot.
pub fn default_snapshot() -> Generation3dSnapshot {
    Generation3dSnapshot::parse_dsl(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT).unwrap_or_default()
}
/// 🧾️ Builds the projection for a named bundled example; unknown ids return `None`.
pub fn example_snapshot(example_id: &str) -> Option<Generation3dSnapshot> {
    let dsl = match example_id {
        PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo" => Some(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT),
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE => Some(GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_TORUS => Some(GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT),
        PROCEDURAL_EXAMPLE_BOX_FILLET => Some(GENERATION3D_EXAMPLE_BOX_FILLET_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE => Some(GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT),
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE => Some(GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE => Some(GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT),
        PROCEDURAL_EXAMPLE_MESH_WORKBENCH => Some(GENERATION3D_EXAMPLE_MESH_WORKBENCH_TEXT),
        PROCEDURAL_EXAMPLE_BOX_SHELL => Some(GENERATION3D_EXAMPLE_BOX_SHELL_TEXT),
        _ => None,
    };
    dsl.and_then(|text| Generation3dSnapshot::parse_dsl(text).ok())
}
/// 🧾️ Serializes an example's bare projection for registration via `App::example`.
pub fn example_document_json(example_id: &str) -> String {
    let snapshot = example_snapshot(example_id).unwrap_or_default();
    let json = semio_framework_pack_json::to_json_string(&snapshot);
    snapshot.retire_cold();
    json
}
}
pub use source_examples::*;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use semio_s_artifact_procedural_3d_model::*;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::widget_id;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_host_snapshot as apply_generation_values_to_host_snapshot_json;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_os_flow::{flow_host_with_session, FlowEvalSession, FlowHost};
use semio_framework_value_derive::{FromValue, ToValue};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;

/// 🌉️ Bridges a `FormGeneration.values` map (`semio_framework_artifact_playbook_playbook::PlaybookValues`, see `FormGeneration`
/// in `📖️playbook/🦀️.rs`) into the `pack::json::Object` that `forms_bridge::apply_generation_values_to_host_snapshot`
/// actually takes.
pub(crate) fn generation_values_to_pack_object(values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> semio_framework_pack_json::Object {
    values.iter().map(|(key, value)| (key.clone(), semio_framework_pack_json::from_dsl_value(value))).collect()
}

/// 🔌️ The widget id behind a preview instance id. Instance ids are channel-qualified
/// (`{widgetId}@{channel}#{index}`) so both suffixes have to come off; a bare widget id, a port id
/// and an instance id therefore all resolve to the same widget.
pub fn widget_id_from_instance_id(instance_id: &str) -> &str {
    let base = instance_id.split('#').next().unwrap_or(instance_id);
    base.split('@').next().unwrap_or(base)
}

pub fn evaluate_generation_preview(host_snapshot: &FlowHostSnapshot, values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> String {
    let snapshot_json = semio_framework_pack_json::to_json_string(host_snapshot);
    let patched = apply_generation_values_to_host_snapshot_json(&snapshot_json, &generation_values_to_pack_object(values));
    let patched_fixture = FlowHost::parse_host_snapshot_json(&patched).unwrap_or_else(|_| host_snapshot.clone());
    let mut host = FlowHost::from_host_snapshot(patched_fixture).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY);
    host.set_neuron_kind_info_map(semio_framework_os_flow::flow_neuron_kind_info_map());
    let evaluated = host.evaluate().unwrap_or_default();
    host.retire_cold();
    evaluated
}

}
pub use snapshot_codec::*;

#[path = "🗂️catalogue/🦀️.rs"]
pub mod catalogue;


