//! 🧬️ Mutation, delta and canonical values of generation3d: the mutation enum with its payload records, the widget, synapse and generation deltas, the typed widget input literals and the canonical JSON trees. See [`super`].

use super::*;

value_codec! {
#[derive(ToValue, FromValue)]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum Generation3dMutation {
    CreateWidget(CreateWidget),
    UpdateWidget(UpdateWidget),
    DeleteWidget(DeleteWidget),
    ConnectSynapse(ConnectSynapse),
    UpdateSynapse(UpdateSynapse),
    DisconnectSynapse(DisconnectSynapse),
    MoveWidget(MoveWidget),
    DeleteWidgetPosition(DeleteWidgetPosition),
    UpdateCamera(UpdateCamera),
    ChangeSchema(ChangeSchema),
    CreateGeneration(CreateGeneration),
    DeleteGeneration(DeleteGeneration),
    RenameGeneration(RenameGeneration),
    ChangeGenerationValue(ChangeGenerationValue),
    ChangeSliderValue(ChangeSliderValue),
    DragTransforms(DragTransforms),
    RotateTransforms(RotateTransforms),
    ScaleTransforms(ScaleTransforms),
    MoveNodes(MoveNodes),
    ChangeWidgetInput(ChangeWidgetInput),
    SelectGeneration(SelectGeneration),
    ChangeGenerationPreview(ChangeGenerationPreview),
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UpdateWidget {
    pub widget: Widget,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DeleteWidget {
    pub id: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UpdateSynapse {
    pub synapse: SynapseSpec,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DisconnectSynapse {
    pub id: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DeleteWidgetPosition {
    pub id: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UpdateCamera {
    pub camera: CameraJson,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeSchema {
    pub new_schema: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CreateGeneration {
    pub generation: FormGeneration,
    pub index: Option<usize>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DeleteGeneration {
    pub id: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RenameGeneration {
    pub id: String,
    pub new_name: String,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub new_value: DslValue,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeSliderValue {
    pub id: String,
    pub value: f64,
}
}

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

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct MoveNodes {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WidgetInputPlane {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
}
}

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

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SelectGeneration {
    pub generation_id: Option<String>,
}
}

value_codec! {
#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ChangeGenerationPreview {
    pub text: Option<String>,
}
}

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

/// 🧩️ The typed literal an operator param holds: `{"$schema", value}`, or `{"$schema", x, y, z}` for a point or a
/// vector.
pub fn widget_input_literal(input: &WidgetInputValue) -> semio_framework_value::DslValue {
    if let Some(items) = input.items() {
        return semio_framework_value::DslValue::Object(std::iter::once(("$schema".into(), semio_framework_value::DslValue::String("list".into()))).chain(items.iter().enumerate().map(|(index, item)| (index.to_string(), widget_input_literal(item)))).collect());
    }
    let schema = ("$schema".to_string(), semio_framework_value::DslValue::String(input.schema().into()));
    let scalar = |value| semio_framework_value::DslValue::Object(vec![schema.clone(), ("value".to_string(), value)]);
    match input {
        WidgetInputValue::Number(value) => scalar(semio_framework_value::DslValue::float(*value)),
        WidgetInputValue::Text(value) => scalar(semio_framework_value::DslValue::String(value.clone())),
        WidgetInputValue::Boolean(value) => scalar(semio_framework_value::DslValue::Bool(*value)),
        WidgetInputValue::Point(axes) | WidgetInputValue::Vector(axes) => semio_framework_value::DslValue::Object(std::iter::once(schema.clone()).chain(["x", "y", "z"].iter().zip(axes).map(|(axis, value)| (axis.to_string(), semio_framework_value::DslValue::float(*value)))).collect()),
        WidgetInputValue::Plane(WidgetInputPlane { origin, normal }) => semio_framework_value::DslValue::Object(vec![schema.clone(), ("origin".to_string(), widget_input_literal(&WidgetInputValue::Point(*origin))), ("normal".to_string(), widget_input_literal(&WidgetInputValue::Vector(*normal)))]),
        _ => unreachable!(),
    }
}

/// 🔎️ The typed input a param literal states, or `None` for a literal of another shape.
pub fn widget_input_of_literal(literal: &semio_framework_value::DslValue) -> Option<WidgetInputValue> {
    if literal.get("$schema").and_then(semio_framework_value::DslValue::as_str) == Some("list") {
        return widget_input_of_literal_as(literal, literal.get("0")?.get("$schema")?.as_str()?);
    }
    let number = |key: &str| literal.get(key).and_then(semio_framework_value::DslValue::as_f64);
    let axes = || Some([number("x")?, number("y")?, number("z")?]);
    Some(match literal.get("$schema").and_then(semio_framework_value::DslValue::as_str)? {
        "number" => WidgetInputValue::Number(number("value")?),
        "text" => WidgetInputValue::Text(literal.get("value")?.as_str()?.to_string()),
        "boolean" => WidgetInputValue::Boolean(literal.get("value")?.as_bool()?),
        "point" => WidgetInputValue::Point(axes()?),
        "vector" => WidgetInputValue::Vector(axes()?),
        "plane" => match (widget_input_of_literal(literal.get("origin")?)?, widget_input_of_literal(literal.get("normal")?)?) {
            (WidgetInputValue::Point(origin), WidgetInputValue::Vector(normal)) => WidgetInputValue::Plane(WidgetInputPlane { origin, normal }),
            _ => return None,
        },
        _ => return None,
    })
}

/// 🏷️ Reads an existing neural list dictionary using its declared element type, including an empty list.
pub fn widget_input_of_literal_as(literal: &semio_framework_value::DslValue, schema: &str) -> Option<WidgetInputValue> {
    if literal.get("$schema").and_then(semio_framework_value::DslValue::as_str) != Some("list") { return widget_input_of_literal(literal); }
    let fields = literal.as_object()?;
    if fields.iter().any(|(key, _)| key != "$schema" && key.parse::<usize>().ok().is_none_or(|index| index.to_string() != *key)) { return None; }
    let count = fields.len().checked_sub(1)?;
    let items = (0..count).map(|index| widget_input_of_literal(literal.get(&index.to_string())?)).collect::<Option<Vec<_>>>()?;
    WidgetInputValue::collection(schema, items)
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ChangeSchema {
    pub new_schema: String,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct DisconnectSynapse {
    pub id: String,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct DragTransforms {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct UpdateWidget {
    pub widget: Widget,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct WidgetInputPlane {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ConnectSynapse {
    pub index: usize,
    pub synapse: SynapseSpec,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SelectGeneration {
    pub generation_id: Option<String>,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct RenameGeneration {
    pub id: String,
    pub new_name: String,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ChangeSliderValue {
    pub id: String,
    pub value: f64,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct MoveNodes {
    pub ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct DeleteGeneration {
    pub id: String,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct DeleteWidget {
    pub id: String,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct UpdateSynapse {
    pub synapse: SynapseSpec,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct UpdateCamera {
    pub camera: CameraJson,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct CreateGeneration {
    pub generation: FormGeneration,
    /// 📍 Zero-based insertion position among the generations; `None` or past the end appends.
    pub index: Option<usize>,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ChangeGenerationPreview {
    pub text: Option<String>,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct RotateTransforms {
    pub targets: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub new_value: semio_framework_value::DslValue,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ScaleTransforms {
    pub targets: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}
}

value_codec! {
#[derive(CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct DeleteWidgetPosition {
    pub id: String,
}
}

/// 🧵️ Canonical tree of the flattened wire record: `id` and `channel` followed by the input's own `type` and `value` fields.
impl ArtifactCanonicalJsonTree for ChangeWidgetInput {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        match self.input.canonical_tree_node()? {
            semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(length) => Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(length + 2)),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "change-widget-input flattened input is not an object")),
        }
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, semio_framework_value::ValueError> {
        match ordinal {
            0 => Ok(&self.id),
            1 => Ok(&self.channel),
            _ => self.input.canonical_tree_child(ordinal - 2),
        }
    }

    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        match ordinal {
            0 => Ok("id".into()),
            1 => Ok("channel".into()),
            _ => self.input.canonical_tree_key(ordinal - 2),
        }
    }
}
