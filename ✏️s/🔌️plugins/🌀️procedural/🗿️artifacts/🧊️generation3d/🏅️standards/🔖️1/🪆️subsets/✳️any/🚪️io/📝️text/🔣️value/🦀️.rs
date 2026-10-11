//! 🚪️ IO-owned value representations of generation3d native semantic records.

use semio_framework_value_derive::value_codec;
use semio_framework_value::DslValue;
use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};
use semio_framework_artifact_playbook_playbook::{FormGeneration, GenerationPlayRoot};
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::schema::mutations::{create_widget, update_widget, delete_widget, connect_synapse, update_synapse, disconnect_synapse, move_widget, delete_widget_position, update_camera, change_schema, create_generation, delete_generation, rename_generation, change_generation_value, change_slider_value, drag_transforms, rotate_transforms, scale_transforms, move_nodes, change_widget_input, select_generation, change_generation_preview};

fn is_false(value: &bool) -> bool { !value }

use crate::standards::v1::subsets::any::schema::{Generation3dArtifact, Generation3dPreviewCamera};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dArtifact {
    pub host_snapshot: FlowHostSnapshot,
    pub generation: GenerationPlayRoot,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dPreviewCamera {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub fov: f64,
}
}

use crate::standards::v1::subsets::any::schema::catalogue::{Localized, Quality, PortType, ShapeKind, SelectionComponent, Selection, EnumOption, Port, PickComponent, Pick, GumballMotion, Along, Gumball, Interaction, Kind, Category, CategoryFile};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct Localized {
    pub en: String,
    pub de: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum Quality {
    ExactAnalytic,
    ExactNumerical,
    Approximate,
    MeshDerivedBrep,
    PolygonMesh,
    TessellatedMesh,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum PortType {
    Number,
    Integer,
    Angle,
    Length,
    Boolean,
    Text,
    Enum,
    Vector,
    Point,
    Plane,
    Shape,
    Shapes,
    Mesh,
    Selection,
    Any,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum ShapeKind {
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Curve,
    Surface,
    Vertex,
    Compound,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum SelectionComponent {
    Face,
    Edge,
    Vertex,
    Mode,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Selection {
    pub component: SelectionComponent,
    pub source: String,
    pub multiple: bool,
    pub mode_from: Option<String>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct EnumOption {
    pub value: String,
    pub label: Localized,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Port {
    pub name: String,
    pub label: Localized,
    pub description: Localized,
    #[value(rename = "type")]
    pub port_type: PortType,
    pub shape_kinds: Option<Vec<ShapeKind>>,
    pub selection: Option<Selection>,
    #[value(default, skip_serializing_if = "is_false")]
    pub list: bool,
    pub min_items: Option<u32>,
    pub max_items: Option<u32>,
    pub default: Option<DslValue>,
    pub min: Option<f64>,
    #[value(default, skip_serializing_if = "is_false")]
    pub exclusive_min: bool,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub unit: Option<String>,
    pub options: Option<Vec<EnumOption>>,
    #[value(default, skip_serializing_if = "is_false")]
    pub optional: bool,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum PickComponent {
    Shape,
    Mesh,
    Face,
    Edge,
    Vertex,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct Pick {
    pub component: PickComponent,
    pub port: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GumballMotion {
    Translate,
    Rotate,
    Scale,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum Along {
    Normal,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Gumball {
    pub motion: GumballMotion,
    pub port: String,
    pub axis_port: Option<String>,
    pub origin_port: Option<String>,
    pub along: Option<Along>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct Interaction {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub pick: Vec<Pick>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub gumball: Vec<Gumball>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Kind {
    pub id: String,
    pub category: String,
    pub emoji: String,
    pub label: Localized,
    pub description: Localized,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
    pub quality: Quality,
    pub interaction: Option<Interaction>,
    pub preview: bool,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct Category {
    pub id: String,
    pub emoji: String,
    pub order: u32,
    pub label: Localized,
    pub description: Localized,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct CategoryFile {
    pub category: Category,
    pub kinds: Vec<Kind>,
}
}

use crate::standards::v1::subsets::any::schema::snapshot::{Generation3dSnapshot};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dSnapshot {
    pub host_snapshot: FlowHostSnapshot,
    pub generation: GenerationPlayRoot,
}
}

use crate::standards::v1::subsets::any::schema::diff::{Generation3dWidgetsDelta, Generation3dSynapsesDelta, Generation3dGenerationsDelta, Generation3dDiff, Generation3dWidgetPatch, Generation3dLayoutRow, Generation3dLayoutDelta, Generation3dGenerationPatch, Generation3dValueRow, Generation3dValuesDelta, Generation3dSelectionChange, Generation3dPreviewChange, Generation3dStringList, Generation3dSynapsePatch};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dDiff {
    pub schema: Option<String>,
    pub camera: Option<CameraJson>,
    pub widgets: Option<Generation3dWidgetsDelta>,
    pub synapses: Option<Generation3dSynapsesDelta>,
    pub layout: Option<Generation3dLayoutDelta>,
    pub generations: Option<Generation3dGenerationsDelta>,
    pub selected_generation: Option<Generation3dSelectionChange>,
    pub preview_text: Option<Generation3dPreviewChange>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Generation3dWidgetPatch {
    Replace { widget: Widget },
    Slider { value: f64, min: f64, max: f64, step: f64 },
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dLayoutRow {
    pub id: String,
    pub layout: WidgetLayout,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dLayoutDelta {
    pub added: Vec<Generation3dLayoutRow>,
    pub removed: Vec<String>,
    pub patched: Vec<Generation3dLayoutRow>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dGenerationPatch {
    pub name: Option<String>,
    pub values: Option<Generation3dValuesDelta>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dValueRow {
    pub question_id: String,
    pub value: DslValue,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dValuesDelta {
    pub added: Vec<Generation3dValueRow>,
    pub removed: Vec<String>,
    pub patched: Vec<Generation3dValueRow>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dSelectionChange {
    pub id: Option<String>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dPreviewChange {
    pub text: Option<String>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dStringList {
    pub values: Vec<String>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(transparent)]
pub struct Generation3dSynapsePatch(pub SynapseSpec);
}

use crate::standards::v1::subsets::any::schema::inferences::{Generation3dInference};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dInference {
    pub topology: Generation3dTopology,
    pub geometry: Generation3dGeometryRecord,
}
}

use crate::standards::v1::subsets::any::schema::inferences::topology::{Generation3dTopology};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dTopology {
    pub node_count: u32,
    pub edge_count: u32,
    pub topo_order: Vec<String>,
    pub depth: u32,
    pub cycle_free: bool,
}
}

use crate::standards::v1::subsets::any::schema::inferences::geometry::{GeometryWire, GeometryDependency, Generation3dFaultRecord, Generation3dOutputRecord, Generation3dWidgetRecord, Generation3dGeometryRecord};

value_codec! {
#[derive(ToValue)]
#[value(rename_all = "camelCase")]
pub struct GeometryWire {
    pub from: String,
    pub from_port: String,
    pub to_port: String,
}
}

value_codec! {
#[derive(ToValue)]
#[value(rename_all = "camelCase")]
pub struct GeometryDependency {
    pub variant: String,
    pub kind: String,
    pub kind_definition: Option<Kind>,
    pub literal: DslValue,
    pub wiring: Vec<GeometryWire>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dFaultRecord {
    pub code: String,
    pub en: String,
    pub de: String,
    pub port: Option<String>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dOutputRecord {
    pub port: String,
    pub kind: String,
    pub detail: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dWidgetRecord {
    pub quality: Quality,
    pub fault: Option<Generation3dFaultRecord>,
    pub outputs: Vec<Generation3dOutputRecord>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dGeometryRecord {
    pub widgets: BTreeMap<String, Generation3dWidgetRecord>,
    pub faulted: u32,
}
}

use crate::standards::v1::subsets::any::schema::inferences::geometry::service::{GeometryCursor, GeometryProgress, GeometryWidgetResult, GeometryResult};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GeometryCursor {
    pub digest: String,
    pub next: u32,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GeometryProgress {
    pub completed: u32,
    pub total: u32,
    pub fraction: f64,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GeometryWidgetResult {
    pub id: String,
    pub dep: String,
    pub quality: Quality,
    pub fault: Option<Generation3dFaultRecord>,
    pub outputs: Vec<Generation3dOutputRecord>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GeometryResult {
    pub complete: bool,
    pub progress: GeometryProgress,
    pub computed: u32,
    pub cache_hits: u32,
    pub fuel_used: u32,
    pub faulted: u32,
    pub cursor: Option<GeometryCursor>,
    pub widgets: Vec<GeometryWidgetResult>,
}
}

use crate::standards::v1::subsets::any::schema::mutations::{Generation3dMutation};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum Generation3dMutation {
    CreateWidget(create_widget::CreateWidget),
    UpdateWidget(update_widget::UpdateWidget),
    DeleteWidget(delete_widget::DeleteWidget),
    ConnectSynapse(connect_synapse::ConnectSynapse),
    UpdateSynapse(update_synapse::UpdateSynapse),
    DisconnectSynapse(disconnect_synapse::DisconnectSynapse),
    MoveWidget(move_widget::MoveWidget),
    DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition),
    UpdateCamera(update_camera::UpdateCamera),
    ChangeSchema(change_schema::ChangeSchema),
    CreateGeneration(create_generation::CreateGeneration),
    DeleteGeneration(delete_generation::DeleteGeneration),
    RenameGeneration(rename_generation::RenameGeneration),
    ChangeGenerationValue(change_generation_value::ChangeGenerationValue),
    ChangeSliderValue(change_slider_value::ChangeSliderValue),
    DragTransforms(drag_transforms::DragTransforms),
    RotateTransforms(rotate_transforms::RotateTransforms),
    ScaleTransforms(scale_transforms::ScaleTransforms),
    MoveNodes(move_nodes::MoveNodes),
    ChangeWidgetInput(change_widget_input::ChangeWidgetInput),
    SelectGeneration(select_generation::SelectGeneration),
    ChangeGenerationPreview(change_generation_preview::ChangeGenerationPreview),
}
}

use crate::standards::v1::subsets::any::schema::mutations::create_widget::{CreateWidget};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}
}

use crate::standards::v1::subsets::any::schema::mutations::update_widget::{UpdateWidget};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UpdateWidget {
    pub widget: Widget,
}
}

use crate::standards::v1::subsets::any::schema::mutations::delete_widget::{DeleteWidget};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DeleteWidget {
    pub id: String,
}
}

use crate::standards::v1::subsets::any::schema::mutations::connect_synapse::{ConnectSynapse};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}
}

use crate::standards::v1::subsets::any::schema::mutations::update_synapse::{UpdateSynapse};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UpdateSynapse {
    pub synapse: SynapseSpec,
}
}

use crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse::{DisconnectSynapse};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DisconnectSynapse {
    pub id: String,
}
}

use crate::standards::v1::subsets::any::schema::mutations::move_widget::{MoveWidget};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}
}

use crate::standards::v1::subsets::any::schema::mutations::delete_widget_position::{DeleteWidgetPosition};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DeleteWidgetPosition {
    pub id: String,
}
}

use crate::standards::v1::subsets::any::schema::mutations::update_camera::{UpdateCamera};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UpdateCamera {
    pub camera: CameraJson,
}
}

use crate::standards::v1::subsets::any::schema::mutations::change_schema::{ChangeSchema};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeSchema {
    pub new_schema: String,
}
}

use crate::standards::v1::subsets::any::schema::mutations::create_generation::{CreateGeneration};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CreateGeneration {
    pub generation: FormGeneration,
    pub index: Option<usize>,
}
}

use crate::standards::v1::subsets::any::schema::mutations::delete_generation::{DeleteGeneration};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DeleteGeneration {
    pub id: String,
}
}

use crate::standards::v1::subsets::any::schema::mutations::rename_generation::{RenameGeneration};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RenameGeneration {
    pub id: String,
    pub new_name: String,
}
}

use crate::standards::v1::subsets::any::schema::mutations::change_generation_value::{ChangeGenerationValue};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub new_value: semio_framework_value::DslValue,
}
}

use crate::standards::v1::subsets::any::schema::mutations::change_slider_value::{ChangeSliderValue};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeSliderValue {
    pub id: String,
    pub value: f64,
}
}

use crate::standards::v1::subsets::any::schema::mutations::drag_transforms::{DragTransforms};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DragTransforms {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}
}

use crate::standards::v1::subsets::any::schema::mutations::rotate_transforms::{RotateTransforms};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RotateTransforms {
    pub targets: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
}
}

use crate::standards::v1::subsets::any::schema::mutations::scale_transforms::{ScaleTransforms};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ScaleTransforms {
    pub targets: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}
}

use crate::standards::v1::subsets::any::schema::mutations::move_nodes::{MoveNodes};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct MoveNodes {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}
}

use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{WidgetInputPlane, WidgetInputValue, ChangeWidgetInput};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(tag = "type", content = "value", rename_all = "camelCase")]
pub enum WidgetInputValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Point([f64; 3]),
    Vector([f64; 3]),
    Plane(WidgetInputPlane),
    NumberList(Vec<f64>),
    TextList(Vec<String>),
    BooleanList(Vec<bool>),
    PointList(Vec<[f64; 3]>),
    VectorList(Vec<[f64; 3]>),
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeWidgetInput {
    pub id: String,
    pub channel: String,
    #[value(flatten)]
    pub input: WidgetInputValue,
}
}

use crate::standards::v1::subsets::any::schema::mutations::select_generation::{SelectGeneration};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SelectGeneration {
    pub generation_id: Option<String>,
}
}

use crate::standards::v1::subsets::any::schema::mutations::change_generation_preview::{ChangeGenerationPreview};

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeGenerationPreview {
    pub text: Option<String>,
}
}

use crate::standards::v1::subsets::any::schema::diff::{Generation3dWidgetRemoval, Generation3dWidgetInsertion, Generation3dWidgetRelocation, Generation3dWidgetModification};
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dWidgetRemoval { pub id: String, pub index: usize, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dWidgetInsertion { pub index: usize, pub row: Widget, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dWidgetRelocation { pub id: String, pub from: usize, pub to: usize, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dWidgetModification { pub id: String, pub patch: Generation3dWidgetPatch, }
}
value_codec! {
#[value(rename_all = "camelCase", default)]
pub struct Generation3dWidgetsDelta { pub removed: Vec<Generation3dWidgetRemoval>, pub inserted: Vec<Generation3dWidgetInsertion>, pub moved: Vec<Generation3dWidgetRelocation>, pub modified: Vec<Generation3dWidgetModification>, }
}
use crate::standards::v1::subsets::any::schema::diff::{Generation3dSynapseRemoval, Generation3dSynapseInsertion, Generation3dSynapseRelocation, Generation3dSynapseModification};
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dSynapseRemoval { pub id: String, pub index: usize, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dSynapseInsertion { pub index: usize, pub row: SynapseSpec, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dSynapseRelocation { pub id: String, pub from: usize, pub to: usize, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dSynapseModification { pub id: String, pub patch: Generation3dSynapsePatch, }
}
value_codec! {
#[value(rename_all = "camelCase", default)]
pub struct Generation3dSynapsesDelta { pub removed: Vec<Generation3dSynapseRemoval>, pub inserted: Vec<Generation3dSynapseInsertion>, pub moved: Vec<Generation3dSynapseRelocation>, pub modified: Vec<Generation3dSynapseModification>, }
}
use crate::standards::v1::subsets::any::schema::diff::{Generation3dGenerationRemoval, Generation3dGenerationInsertion, Generation3dGenerationRelocation, Generation3dGenerationModification};
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dGenerationRemoval { pub id: String, pub index: usize, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dGenerationInsertion { pub index: usize, pub row: FormGeneration, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dGenerationRelocation { pub id: String, pub from: usize, pub to: usize, }
}
value_codec! {
#[value(rename_all = "camelCase")]
pub struct Generation3dGenerationModification { pub id: String, pub patch: Generation3dGenerationPatch, }
}
value_codec! {
#[value(rename_all = "camelCase", default)]
pub struct Generation3dGenerationsDelta { pub removed: Vec<Generation3dGenerationRemoval>, pub inserted: Vec<Generation3dGenerationInsertion>, pub moved: Vec<Generation3dGenerationRelocation>, pub modified: Vec<Generation3dGenerationModification>, }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
