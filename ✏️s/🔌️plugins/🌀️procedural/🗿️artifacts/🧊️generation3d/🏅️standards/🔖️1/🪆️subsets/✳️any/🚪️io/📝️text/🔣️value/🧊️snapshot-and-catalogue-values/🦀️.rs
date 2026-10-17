//! 🧊️ Snapshot and catalogue values of generation3d: the artifact, preview camera, localized text, the widget catalogue records, the snapshot, the diff patches and the inference, topology and geometry records. See [`super`].

use super::*;

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

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dSnapshot {
    pub host_snapshot: FlowHostSnapshot,
    pub generation: GenerationPlayRoot,
}
}

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

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation3dInference {
    pub topology: Generation3dTopology,
    pub geometry: Generation3dGeometryRecord,
}
}

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

value_codec! {
#[derive(ToValue)]
#[value(rename_all = "camelCase")]
pub struct GeometryWire {
    pub from: String,
    pub from_port: String,
    pub to_port: String,
}
}

#[derive(semio_framework_value_derive::ToValue)]
struct GeometryScalarLiteral<T: semio_framework_value::ToValue> {
    pub(super) value: T,
}

impl semio_framework_value::ToValue for GeometryDependencyLiteral {
    fn to_value(&self) -> DslValue {
        use semio_framework_value::ToValue;
        match self {
            Self::Parameters(value) => value.to_value(),
            Self::Number { value } => GeometryScalarLiteral { value: *value }.to_value(),
            Self::Text { value } => GeometryScalarLiteral { value: value.as_str() }.to_value(),
            Self::Absent => DslValue::Null,
        }
    }

    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, semio_framework_value::ValueError> {
        use semio_framework_value::ToValue;
        match self {
            Self::Parameters(value) => value.to_value_controlled(control),
            Self::Number { value } => GeometryScalarLiteral { value: *value }.to_value_controlled(control),
            Self::Text { value } => GeometryScalarLiteral { value: value.as_str() }.to_value_controlled(control),
            Self::Absent => DslValue::Null.to_value_controlled(control),
        }
    }
}

value_codec! {
#[derive(ToValue)]
#[value(rename_all = "camelCase")]
pub struct GeometryDependency {
    pub variant: String,
    pub kind: String,
    pub kind_definition: Option<Kind>,
    pub literal: GeometryDependencyLiteral,
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
