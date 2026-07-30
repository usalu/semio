//! 📐 CAD scene document + typed VCS on `vcs`.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

//#region 🔖Domain
pub const CAD_DOCUMENT_SCHEMA: &str = "cad.scene";

pub const CAD_PLAY_DOCUMENT_SCHEMA: &str = "cad.document";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum CadPaneId {
    Shape,
    Building,
    Energy,
    StructureClassic,
}

impl CadPaneId {
    pub fn model_definition_id(self) -> &'static str {
        match self {
            Self::Shape => "spatial.shape",
            Self::Building => "aec.building",
            Self::Energy => "aec.building.energy",
            Self::StructureClassic => "aec.building.structure.classic",
        }
    }

    pub fn all() -> [Self; 4] {
        [Self::Shape, Self::Building, Self::Energy, Self::StructureClassic]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadPrimitiveSlot {
    pub slot: String,
    pub primitive_id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadVertex {
    pub id: String,
    pub position: [f64; 3],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadEdgeCurve {
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadEdge {
    pub id: String,
    pub vertex_ids: Vec<String>,
    #[dsl(block)]
    pub curve: CadEdgeCurve,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadWire {
    pub id: String,
    pub edge_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadPlaneSurface {
    pub kind: String,
    pub origin: [f64; 3],
    pub normal: [f64; 3],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadFace {
    pub id: String,
    pub wire_ids: Vec<String>,
    #[dsl(block)]
    pub surface: CadPlaneSurface,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadShell {
    pub id: String,
    pub face_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadSolid {
    pub id: String,
    pub shell_ids: Vec<String>,
}

/// @emoji 🧱 Authored brep topology carried alongside spatial model objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadGeometry {
    #[serde(default)]
    pub anchors: Vec<Value>,
    #[serde(default)]
    pub vertices: Vec<CadVertex>,
    #[serde(default)]
    pub edges: Vec<CadEdge>,
    #[serde(default)]
    pub wires: Vec<CadWire>,
    #[serde(default)]
    pub faces: Vec<CadFace>,
    #[serde(default)]
    pub shells: Vec<CadShell>,
    #[serde(default)]
    pub solids: Vec<CadSolid>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadObject {
    pub id: String,
    pub label: String,
    pub typology: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    #[serde(default)]
    pub scale: Option<[f64; 3]>,
    #[serde(default, rename = "meshUrl")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub extent: Option<[f64; 3]>,
    #[serde(default, rename = "solidHandle")]
    pub solid_handle: Option<String>,
    #[serde(default)]
    pub primitives: Vec<CadPrimitiveSlot>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadReference {
    pub id: String,
    pub source_url: String,
    #[serde(default = "default_image_media_kind")]
    pub media_kind: String,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    #[serde(default)]
    pub scale: Option<Value>,
    #[serde(default = "default_width_world")]
    pub width_world: f64,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub opacity: Option<f64>,
}

fn default_image_media_kind() -> String {
    "image".into()
}

fn default_width_world() -> f64 {
    10.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadCamera {
    #[serde(default = "default_camera_position")]
    pub position: [f64; 3],
    #[serde(default = "default_camera_target")]
    pub target: [f64; 3],
    #[serde(default = "one_f64")]
    pub zoom: f64,
    #[serde(default = "default_fov")]
    pub fov: f64,
    /// 📐 Serialized `semio_framework_plugin::WorldProjectionConfig` — kept as raw json here (cad/rs has no
    /// dependency on the plugin layer); `cad/plugin/rs` parses/writes it around the shared projection helpers.
    #[serde(default)]
    pub projection: Value,
}

impl Default for CadCamera {
    fn default() -> Self {
        Self { position: default_camera_position(), target: default_camera_target(), zoom: one_f64(), fov: default_fov(), projection: Value::Null }
    }
}

fn default_camera_position() -> [f64; 3] {
    [12.0, -12.0, 8.0]
}

fn default_camera_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

fn default_fov() -> f64 {
    50.0
}

fn one_f64() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadNode {
    pub id: String,
    pub label: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "cad", layout = "lines")]
pub struct CadScene {
    pub schema: String,
    pub id: String,
    #[serde(default)]
    #[dsl(block)]
    pub camera: CadCamera,
    #[serde(default)]
    #[dsl(block)]
    pub camera_building: CadCamera,
    #[serde(default)]
    #[dsl(block)]
    pub camera_energy: CadCamera,
    #[serde(default)]
    #[dsl(block)]
    pub camera_structure_classic: CadCamera,
    #[serde(default)]
    #[dsl(table)]
    pub objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    pub building_objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    pub energy_objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    pub structure_classic_objects: Vec<CadObject>,
    #[serde(default)]
    pub references_by_model_definition_id: BTreeMap<String, Vec<CadReference>>,
    #[serde(default)]
    #[dsl(table)]
    pub nodes: Vec<CadNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub shape_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub building_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub energy_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub structure_classic_geometry: Option<CadGeometry>,
    #[serde(default = "default_model_definition_id")]
    pub active_model_definition_id: String,
}

pub fn cad_pane_geometry(scene: &CadScene, pane: CadPaneId) -> Option<&CadGeometry> {
    match pane {
        CadPaneId::Shape => scene.shape_geometry.as_ref(),
        CadPaneId::Building => scene.building_geometry.as_ref(),
        CadPaneId::Energy => scene.energy_geometry.as_ref(),
        CadPaneId::StructureClassic => scene.structure_classic_geometry.as_ref(),
    }
}

pub fn cad_pane_geometry_mut(scene: &mut CadScene, pane: CadPaneId) -> &mut Option<CadGeometry> {
    match pane {
        CadPaneId::Shape => &mut scene.shape_geometry,
        CadPaneId::Building => &mut scene.building_geometry,
        CadPaneId::Energy => &mut scene.energy_geometry,
        CadPaneId::StructureClassic => &mut scene.structure_classic_geometry,
    }
}

pub fn cad_pane_camera(scene: &CadScene, pane: CadPaneId) -> &CadCamera {
    match pane {
        CadPaneId::Shape => &scene.camera,
        CadPaneId::Building => &scene.camera_building,
        CadPaneId::Energy => &scene.camera_energy,
        CadPaneId::StructureClassic => &scene.camera_structure_classic,
    }
}

pub fn cad_pane_camera_mut(scene: &mut CadScene, pane: CadPaneId) -> &mut CadCamera {
    match pane {
        CadPaneId::Shape => &mut scene.camera,
        CadPaneId::Building => &mut scene.camera_building,
        CadPaneId::Energy => &mut scene.camera_energy,
        CadPaneId::StructureClassic => &mut scene.camera_structure_classic,
    }
}

fn default_model_definition_id() -> String {
    "spatial.shape".into()
}

pub fn empty_cad_projection() -> CadScene {
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        camera: CadCamera::default(),
        camera_building: CadCamera::default(),
        camera_energy: CadCamera::default(),
        camera_structure_classic: CadCamera::default(),
        objects: Vec::new(),
        building_objects: Vec::new(),
        energy_objects: Vec::new(),
        structure_classic_objects: Vec::new(),
        references_by_model_definition_id: BTreeMap::new(),
        nodes: Vec::new(),
        shape_geometry: None,
        building_geometry: None,
        energy_geometry: None,
        structure_classic_geometry: None,
        active_model_definition_id: default_model_definition_id(),
    }
}

pub fn cad_pane_objects(scene: &CadScene, pane: CadPaneId) -> &[CadObject] {
    match pane {
        CadPaneId::Shape => &scene.objects,
        CadPaneId::Building => &scene.building_objects,
        CadPaneId::Energy => &scene.energy_objects,
        CadPaneId::StructureClassic => &scene.structure_classic_objects,
    }
}

pub fn cad_pane_objects_mut(scene: &mut CadScene, pane: CadPaneId) -> &mut Vec<CadObject> {
    match pane {
        CadPaneId::Shape => &mut scene.objects,
        CadPaneId::Building => &mut scene.building_objects,
        CadPaneId::Energy => &mut scene.energy_objects,
        CadPaneId::StructureClassic => &mut scene.structure_classic_objects,
    }
}

pub fn cad_find_object_pane(scene: &CadScene, object_id: &str) -> Option<CadPaneId> {
    CadPaneId::all().into_iter().find(|&pane| cad_pane_objects(scene, pane).iter().any(|object| object.id == object_id))
}

pub fn cad_all_objects(scene: &CadScene) -> impl Iterator<Item = (&CadObject, CadPaneId)> {
    CadPaneId::all().into_iter().flat_map(|pane| cad_pane_objects(scene, pane).iter().map(move |object| (object, pane)))
}

pub fn cad_pane_from_model_definition_id(model_definition_id: &str) -> Option<CadPaneId> {
    CadPaneId::all().into_iter().find(|pane| pane.model_definition_id() == model_definition_id)
}

/// 🪆 `Box<CadScene>` needs its own `dsl::DslField` binding for `CadOperation::SetScene` — `Box` is
/// `#[fundamental]` in `std`, so implementing a foreign trait (`dsl::DslField`) for `Box<CadScene>`
/// (a local type inside the foreign, fundamental `Box` wrapper) is permitted by the orphan rules;
/// this delegates entirely to `CadScene`'s own derive-generated `DslField` impl (from `DslDocument`).
impl dsl::DslField for Box<CadScene> {
    fn shape() -> dsl::Shape {
        <CadScene as dsl::DslField>::shape()
    }
    fn to_value(&self) -> dsl::FieldValue {
        <CadScene as dsl::DslField>::to_value(self)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        <CadScene as dsl::DslField>::from_value(value).map(Box::new)
    }
}
//#endregion 🔖Domain

//#region 🔖InteractionSpec
/// A path root within an expression/effect target — `context` (session context), `event` (the
/// event payload being handled), or `params` (an enclosing action's parameters; unused by the
/// interaction machine interpreter itself, only by `spatial.action` step specs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExprPathRoot {
    Context,
    Event,
    Params,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ExprPathSegment {
    Field { name: String },
    Index { index: usize },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExprPathTarget {
    pub root: ExprPathRoot,
    #[serde(default)]
    pub segments: Vec<ExprPathSegment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExprBinding {
    pub name: String,
    pub value: Box<Expr>,
}

/// `spatial://schema/json/expression` — a small declarative expression AST. Only the kinds
/// actually used by the interaction machine specs' guards/effects/display are interpreted here
/// (`kernel.call`/`distance`/`fold` appear only in `spatial.action` step specs, which are not
/// executed generically — see the commit-action runner in `cad/plugin/rs/interaction.rs`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Expr {
    Path {
        root: ExprPathRoot,
        #[serde(default)]
        segments: Vec<ExprPathSegment>,
    },
    Const {
        value: Value,
    },
    Var {
        name: String,
    },
    Let {
        bindings: Vec<ExprBinding>,
        #[serde(rename = "in")]
        body: Box<Expr>,
    },
    Exists {
        target: ExprPathTarget,
    },
    NotEmpty {
        target: ExprPathTarget,
    },
    All {
        args: Vec<Expr>,
    },
    Any {
        args: Vec<Expr>,
    },
    Not {
        arg: Box<Expr>,
    },
    Abs {
        arg: Box<Expr>,
    },
    Distance {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    #[serde(rename = "kernel.call")]
    KernelCall {
        function: String,
        #[serde(default)]
        args: std::collections::HashMap<String, Expr>,
    },
    Binop {
        operation: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Fold {
        operation: String,
        args: Vec<Expr>,
    },
}

/// Evaluation environment for {@link Expr}: `context` is the engagement session's persistent
/// state, `event` is the payload of the event currently being handled (if any).
pub struct ExprEnv<'a> {
    pub context: &'a std::collections::HashMap<String, Value>,
    pub event: Option<&'a Value>,
}

fn expr_path_get(root_value: Option<&Value>, segments: &[ExprPathSegment]) -> Option<Value> {
    let mut current = root_value?.clone();
    for segment in segments {
        current = match segment {
            ExprPathSegment::Field { name } => current.get(name)?.clone(),
            ExprPathSegment::Index { index } => current.get(index)?.clone(),
        };
    }
    Some(current)
}

fn expr_value_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map(|v| v != 0.0).unwrap_or(false),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

fn expr_value_not_empty(value: Option<&Value>) -> bool {
    match value {
        None => false,
        Some(Value::Null) => false,
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(o)) => !o.is_empty(),
        Some(Value::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

fn expr_as_f64(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}

/// Evaluates an {@link Expr} against `env` and an outer `let`-binding scope (`vars`).
pub fn evaluate_expr(expr: &Expr, env: &ExprEnv, vars: &std::collections::HashMap<String, Value>) -> Value {
    match expr {
        Expr::Path { root, segments } => {
            let root_value = match root {
                ExprPathRoot::Context => Some(serde_json::to_value(env.context).unwrap_or(Value::Null)),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            expr_path_get(root_value.as_ref(), segments).unwrap_or(Value::Null)
        }
        Expr::Const { value } => value.clone(),
        Expr::Var { name } => vars.get(name).cloned().unwrap_or(Value::Null),
        Expr::Let { bindings, body } => {
            let mut scope = vars.clone();
            for binding in bindings {
                let value = evaluate_expr(&binding.value, env, &scope);
                scope.insert(binding.name.clone(), value);
            }
            evaluate_expr(body, env, &scope)
        }
        Expr::Exists { target } => {
            let root_value = match target.root {
                ExprPathRoot::Context => Some(serde_json::to_value(env.context).unwrap_or(Value::Null)),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            Value::Bool(expr_path_get(root_value.as_ref(), &target.segments).is_some())
        }
        Expr::NotEmpty { target } => {
            let root_value = match target.root {
                ExprPathRoot::Context => Some(serde_json::to_value(env.context).unwrap_or(Value::Null)),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            Value::Bool(expr_value_not_empty(expr_path_get(root_value.as_ref(), &target.segments).as_ref()))
        }
        Expr::All { args } => Value::Bool(args.iter().all(|arg| expr_value_truthy(&evaluate_expr(arg, env, vars)))),
        Expr::Any { args } => Value::Bool(args.iter().any(|arg| expr_value_truthy(&evaluate_expr(arg, env, vars)))),
        Expr::Not { arg } => Value::Bool(!expr_value_truthy(&evaluate_expr(arg, env, vars))),
        Expr::Abs { arg } => json!(expr_as_f64(&evaluate_expr(arg, env, vars)).abs()),
        Expr::Distance { a, b } => {
            let av = evaluate_expr(a, env, vars);
            let bv = evaluate_expr(b, env, vars);
            let da: Option<[f64; 3]> = serde_json::from_value(av).ok();
            let db: Option<[f64; 3]> = serde_json::from_value(bv).ok();
            match (da, db) {
                (Some(a), Some(b)) => {
                    json!(((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt())
                }
                _ => Value::Null,
            }
        }
        // `kernel.call` expressions are only used inside `spatial.action` step specs (not executed
        // generically by this interpreter); evaluating one directly yields null.
        Expr::KernelCall { .. } => Value::Null,
        Expr::Binop { operation, left, right } => {
            let lv = evaluate_expr(left, env, vars);
            let rv = evaluate_expr(right, env, vars);
            match operation.as_str() {
                "==" => Value::Bool(lv == rv),
                "!=" => Value::Bool(lv != rv),
                ">" => Value::Bool(expr_as_f64(&lv) > expr_as_f64(&rv)),
                "<" => Value::Bool(expr_as_f64(&lv) < expr_as_f64(&rv)),
                ">=" => Value::Bool(expr_as_f64(&lv) >= expr_as_f64(&rv)),
                "<=" => Value::Bool(expr_as_f64(&lv) <= expr_as_f64(&rv)),
                "+" => json!(expr_as_f64(&lv) + expr_as_f64(&rv)),
                "-" => json!(expr_as_f64(&lv) - expr_as_f64(&rv)),
                "*" => json!(expr_as_f64(&lv) * expr_as_f64(&rv)),
                "/" => json!(expr_as_f64(&lv) / expr_as_f64(&rv)),
                _ => Value::Null,
            }
        }
        Expr::Fold { operation, args } => {
            let values: Vec<f64> = args.iter().map(|arg| expr_as_f64(&evaluate_expr(arg, env, vars))).collect();
            match operation.as_str() {
                "min" => values.into_iter().fold(f64::INFINITY, f64::min).into(),
                "max" => values.into_iter().fold(f64::NEG_INFINITY, f64::max).into(),
                _ => Value::Null,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Effect {
    Assign {
        target: ExprPathTarget,
        value: Expr,
    },
    Clear {
        target: ExprPathTarget,
    },
    Append {
        target: ExprPathTarget,
        value: Expr,
    },
    Emit {
        event: Value,
    },
    Raise {
        event: String,
    },
    OpenTransaction,
    CommitTransaction,
    RollbackTransaction,
    RequestPreview,
    #[serde(rename = "kernel.query")]
    KernelQuery {
        #[serde(default)]
        query: Option<String>,
        #[serde(default, rename = "assignTo")]
        assign_to: Option<ExprPathTarget>,
    },
    ResolveEditable,
    SetDiagnostic {
        severity: String,
        code: String,
        message: String,
    },
    ClearDiagnostic {
        code: String,
    },
    Action {
        action: String,
        #[serde(default)]
        params: std::collections::HashMap<String, Expr>,
        #[serde(default, rename = "assignTo")]
        assign_to: Option<ExprPathTarget>,
    },
    /// Asset-only extension (not in the formal schema): delegates to a nested sub-interaction
    /// (`interaction`), then maps each of its `outputs[].value` expressions (evaluated against
    /// the sub-interaction's context) onto `outputs[].target` in the parent context. Used only by
    /// the curve-drawing sub-flow (`mode.curve` in the wall/slab/column specs) — not yet
    /// interpreted (sub-interaction composition is a follow-up; the primary `mode.2points` flow
    /// does not depend on it).
    #[serde(rename = "interaction.call")]
    InteractionCall {
        interaction: String,
        #[serde(default)]
        outputs: Vec<InteractionCallOutput>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionCallOutput {
    pub target: ExprPathTarget,
    pub value: Expr,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionSpec {
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub guard: Option<String>,
    #[serde(default)]
    pub transient: bool,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventHandlerSpec {
    pub event: String,
    #[serde(default)]
    pub transitions: Vec<TransitionSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionSpec {
    #[serde(default)]
    pub accept: Vec<String>,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub prompt: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateDefSpec {
    pub name: String,
    #[serde(default)]
    pub r#final: bool,
    #[serde(default)]
    pub selection: Option<SelectionSpec>,
    #[serde(default)]
    pub on: Vec<EventHandlerSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineSpec {
    pub initial: String,
    pub states: Vec<StateDefSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuardSpec {
    pub name: String,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LengthEntrySpec {
    pub state: String,
    pub anchor: String,
    pub field: String,
    #[serde(default)]
    pub commit: Option<String>,
    #[serde(default)]
    pub control: Option<String>,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub step: Option<f64>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub default: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarEntrySpec {
    pub state: String,
    pub event: String,
    pub field: String,
    #[serde(default)]
    pub commit: Option<String>,
    #[serde(default)]
    pub axis_anchor: Option<String>,
    #[serde(default)]
    pub axis_floor: Option<String>,
    #[serde(default)]
    pub axis: Option<[f64; 3]>,
    #[serde(default)]
    pub control: Option<String>,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub step: Option<f64>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub default: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialInteractionConfig {
    #[serde(default)]
    pub spatial_ground_pick: bool,
    #[serde(default)]
    pub pick_disabled_states: Vec<String>,
    #[serde(default)]
    pub ground_pointer_move_states: Vec<String>,
    #[serde(default)]
    pub height_drag_states: Vec<String>,
    #[serde(default)]
    pub vertical_rod_states: Vec<String>,
    #[serde(default)]
    pub height_confirm_state: Option<String>,
    #[serde(default)]
    pub length_entry: Vec<LengthEntrySpec>,
    #[serde(default)]
    pub scalar_entry: Vec<ScalarEntrySpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DisplayItemSpec {
    Point {
        id: String,
        #[serde(default)]
        role: Option<String>,
        position: Expr,
    },
    Label {
        id: String,
        #[serde(default)]
        role: Option<String>,
        text: String,
        position: Expr,
    },
    Segment {
        id: String,
        #[serde(default)]
        role: Option<String>,
        from: Expr,
        to: Expr,
    },
    #[serde(rename = "linear-handle")]
    LinearHandle {
        id: String,
        #[serde(default)]
        role: Option<String>,
        axis: [f64; 3],
        origin: Expr,
    },
    #[serde(rename = "box-preview")]
    BoxPreview {
        id: String,
        #[serde(default)]
        role: Option<String>,
        #[serde(rename = "cornerA")]
        corner_a: Expr,
        #[serde(rename = "cornerB")]
        corner_b: Expr,
        height: Expr,
    },
    #[serde(rename = "entity-highlight")]
    EntityHighlight {
        id: String,
        #[serde(default)]
        role: Option<String>,
        #[serde(rename = "geometryEntityKind")]
        geometry_entity_kind: String,
        #[serde(rename = "entityId")]
        entity_id: Expr,
    },
    Curve {
        id: String,
        #[serde(default)]
        role: Option<String>,
    },
    Mesh {
        id: String,
        #[serde(default)]
        role: Option<String>,
    },
    /// Asset-only extension kind (`"preview"`) not in the formal schema: a generic wireframe
    /// preview keyed by `previewKind`, evaluated params passed through verbatim to the renderer.
    Preview {
        id: String,
        #[serde(default)]
        role: Option<String>,
        #[serde(default, rename = "previewKind")]
        preview_kind: Option<String>,
        #[serde(default)]
        params: std::collections::HashMap<String, Expr>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayStateSpec {
    pub state: String,
    pub items: Vec<DisplayItemSpec>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplaySpec {
    #[serde(default)]
    pub states: Vec<DisplayStateSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitSpec {
    #[serde(default)]
    pub when: Option<String>,
    #[serde(default)]
    pub from_states: Vec<String>,
    pub operation: CommitOperationSpec,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitOperationSpec {
    pub action: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, Expr>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionProducesSpec {
    #[serde(default)]
    pub typology: Option<String>,
}

/// `spatial://schema/json/interaction` — the full declarative construction-interaction spec, as
/// authored in `cad/asset/modelDefinition/*/interaction/*.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionSpec {
    pub id: String,
    pub version: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub produces: InteractionProducesSpec,
    #[serde(default)]
    pub guards: Vec<GuardSpec>,
    pub machine: MachineSpec,
    #[serde(default)]
    pub display: DisplaySpec,
    #[serde(default)]
    pub interaction: SpatialInteractionConfig,
    pub commit: CommitSpec,
}

impl InteractionSpec {
    pub fn state<'a>(&'a self, name: &str) -> Option<&'a StateDefSpec> {
        self.machine.states.iter().find(|state| state.name == name)
    }

    pub fn guard(&self, name: &str, env: &ExprEnv) -> bool {
        self.guards.iter().find(|guard| guard.name == name).map(|guard| expr_value_truthy(&evaluate_expr(&guard.expr, env, &std::collections::HashMap::new()))).unwrap_or(false)
    }
}
//#endregion 🔖InteractionSpec

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_cameras_isolate_states() {
        let mut scene = empty_cad_projection();

        // Assert initial defaults
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Shape).fov, 50.0);
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Building).fov, 50.0);

        // Update Shape camera
        cad_pane_camera_mut(&mut scene, CadPaneId::Shape).fov = 40.0;

        // Verify isolation
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Shape).fov, 40.0);
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Building).fov, 50.0);

        // Update Building camera
        cad_pane_camera_mut(&mut scene, CadPaneId::Building).fov = 60.0;

        // Verify isolation
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Shape).fov, 40.0);
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Building).fov, 60.0);
    }

    #[test]
    fn interaction_spec_parses_box_asset() {
        let raw = include_str!("../../../asset/modelDefinition/spatial.shape/interaction/box.json");
        let spec: InteractionSpec = serde_json::from_str(raw).expect("box.json parses as InteractionSpec");
        assert_eq!(spec.id, "primitive.box");
        assert_eq!(spec.machine.initial, "idle");
        assert!(spec.state("first_corner").is_some());
        assert!(spec.state("ready").is_some());
        assert_eq!(spec.commit.operation.action, "primitive.createBoxFromCorners");
        assert!(spec.commit.operation.params.contains_key("cornerA"));
        assert!(spec.commit.operation.params.contains_key("cornerB"));
        assert!(spec.commit.operation.params.contains_key("height"));
        assert_eq!(spec.commit.from_states, vec!["ready".to_string()]);
    }

    #[test]
    fn interaction_spec_parses_sphere_asset_with_command_finish() {
        let raw = include_str!("../../../asset/modelDefinition/spatial.shape/interaction/sphere.json");
        let spec: InteractionSpec = serde_json::from_str(raw).expect("sphere.json parses as InteractionSpec");
        assert_eq!(spec.id, "solid.sphere");
        assert_eq!(spec.commit.operation.action, "command.finish");
        assert!(spec.display.states.iter().any(|s| s.state == "radius"));
    }

    #[test]
    fn interaction_spec_parses_all_energy_and_structure_classic_assets() {
        let sources = [
            include_str!("../../../asset/modelDefinition/aec.building.energy/interaction/constructBasePlate.json"),
            include_str!("../../../asset/modelDefinition/aec.building.energy/interaction/constructExternalWall.json"),
            include_str!("../../../asset/modelDefinition/aec.building.energy/interaction/constructHull.json"),
            include_str!("../../../asset/modelDefinition/aec.building.energy/interaction/constructRoof.json"),
            include_str!("../../../asset/modelDefinition/aec.building.energy/interaction/constructWindows.json"),
            include_str!("../../../asset/modelDefinition/aec.building.structure.classic/interaction/constructOneWayReinforcedConcreteSlab.json"),
            include_str!("../../../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteColumn.json"),
            include_str!("../../../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteExternalWall.json"),
            include_str!("../../../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteInternalWall.json"),
        ];
        for raw in sources {
            let spec: InteractionSpec = serde_json::from_str(raw).expect("asset parses as InteractionSpec");
            assert!(spec.commit.operation.action.ends_with("From2PointsAndHeight") || spec.commit.operation.action.ends_with("FromSurface"));
            assert!(spec.commit.operation.params.contains_key("pointA"));
            assert!(spec.commit.operation.params.contains_key("pointB"));
            assert!(spec.commit.operation.params.contains_key("height"));
            assert!(spec.commit.operation.params.contains_key("typology"));
        }
    }

    /// Regression guard: every `interaction/*.json` asset in the tree must parse as
    /// `InteractionSpec` — catches schema drift between the JSON assets and these Rust types.
    #[test]
    fn every_interaction_asset_on_disk_parses_as_interaction_spec() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../asset/modelDefinition");
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else { return };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.file_name().and_then(|n| n.to_str()).map(|n| n.ends_with(".json")).unwrap_or(false) && path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()) == Some("interaction") {
                    out.push(path);
                }
            }
        }
        let mut files = Vec::new();
        walk(&root, &mut files);
        assert!(files.len() >= 40, "expected at least 40 interaction assets, found {}", files.len());
        let mut failures = Vec::new();
        for file in &files {
            let raw = std::fs::read_to_string(file).expect("read asset");
            if let Err(err) = serde_json::from_str::<InteractionSpec>(&raw) {
                failures.push(format!("{}: {}", file.display(), err));
            }
        }
        assert!(failures.is_empty(), "{} interaction assets failed to parse:\n{}", failures.len(), failures.join("\n"));
    }

    #[test]
    fn evaluate_expr_supports_path_const_var_and_boolean_combinators() {
        let mut context = std::collections::HashMap::new();
        context.insert("height".to_string(), json!(2.5));
        context.insert("origin".to_string(), json!([0.0, 0.0, 0.0]));
        let env = ExprEnv { context: &context, event: None };
        let vars = std::collections::HashMap::new();

        let path_expr = Expr::Path { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "height".into() }] };
        assert_eq!(evaluate_expr(&path_expr, &env, &vars), json!(2.5));

        let exists_expr = Expr::Exists { target: ExprPathTarget { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "origin".into() }] } };
        assert_eq!(evaluate_expr(&exists_expr, &env, &vars), json!(true));

        let missing_exists_expr = Expr::Exists { target: ExprPathTarget { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "missing".into() }] } };
        assert_eq!(evaluate_expr(&missing_exists_expr, &env, &vars), json!(false));

        let binop_expr = Expr::Binop { operation: ">".into(), left: Box::new(path_expr.clone()), right: Box::new(Expr::Const { value: json!(1.0) }) };
        assert_eq!(evaluate_expr(&binop_expr, &env, &vars), json!(true));

        let all_expr = Expr::All { args: vec![exists_expr, binop_expr] };
        assert_eq!(evaluate_expr(&all_expr, &env, &vars), json!(true));

        let let_expr = Expr::Let {
            bindings: vec![ExprBinding { name: "h".into(), value: Box::new(path_expr) }],
            body: Box::new(Expr::Binop { operation: "*".into(), left: Box::new(Expr::Var { name: "h".into() }), right: Box::new(Expr::Const { value: json!(2.0) }) }),
        };
        assert_eq!(evaluate_expr(&let_expr, &env, &vars), json!(5.0));
    }

    #[test]
    fn interaction_spec_guard_evaluates_against_context() {
        let raw = include_str!("../../../asset/modelDefinition/aec.building.energy/interaction/constructExternalWall.json");
        let spec: InteractionSpec = serde_json::from_str(raw).expect("parses");
        let mut context = std::collections::HashMap::new();
        let env_without = ExprEnv { context: &context, event: None };
        assert!(!spec.guard("hasConstructMode", &env_without));
        context.insert("constructMode".to_string(), json!("2PointsAndHeight"));
        let env_with = ExprEnv { context: &context, event: None };
        assert!(spec.guard("hasConstructMode", &env_with));
    }
}
//#endregion 🧪Tests
