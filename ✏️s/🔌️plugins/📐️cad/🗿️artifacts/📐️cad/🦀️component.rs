//! 📐️ CAD artifact — the `cad.scene` document schema: the `CadScene` projection, its object/
//! reference/geometry/camera records, and the pane vocabulary every other cad node addresses them by.
//! The declarative `spatial.interaction` spec types live beside this file in
//! `🎬️interaction-spec/🦀️component.rs`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Domain
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
    #[dsl(coord)]
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
    #[dsl(coord)]
    pub origin: [f64; 3],
    #[dsl(dir)]
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

/// @emoji 🧱️ Authored brep topology carried alongside spatial model objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadGeometry {
    #[serde(default)]
    pub anchors: Vec<dsl::DslValue>,
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
    #[dsl(coord)]
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
    /// 📐️ Uniform scale factor applied to the image plane (unlike `CadObject.scale`, references
    /// are flat and never scaled non-uniformly per axis — every call site only ever reads/writes
    /// a single number, see `apply_reference_patch`/`sample_reference` in `cad/op/rs`).
    #[serde(default)]
    pub scale: Option<f64>,
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

/// 📐️ Local twin of `semio_framework_plugin::WorldProjectionConfig`'s flat 15-field classical
/// taxonomy (Parallel: Orthographic/Axonometric/Oblique, Perspective: 1/2/3-Point/Curvilinear) —
/// mirrored here rather than imported because `cad/rs` has no dependency on the plugin layer;
/// `cad/engine/rs`'s `cad_camera_projection_config`/`cad_camera_set_projection_config` convert
/// field-for-field between this and the real `WorldProjectionConfig` around the shared projection
/// helpers. See https://en.wikipedia.org/wiki/Axonometric_projection and
/// https://en.wikipedia.org/wiki/Oblique_projection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct CadProjectionDsl {
    pub kind: String,
    pub orthographic_view: String,
    pub axonometric_variant: String,
    pub axonometric_angle_a: f64,
    pub axonometric_angle_b: f64,
    pub axonometric_quadrant: String,
    pub oblique_variant: String,
    pub oblique_angle: f64,
    pub oblique_depth: f64,
    pub one_point_axis: String,
    pub fov: f64,
    pub two_point_shift: f64,
    pub curvilinear_fov: f64,
    pub curvilinear_strength: f64,
    pub curvilinear_mapping: String,
}

impl Default for CadProjectionDsl {
    fn default() -> Self {
        Self {
            kind: "threePoint".into(),
            orthographic_view: "top".into(),
            axonometric_variant: "isometric".into(),
            axonometric_angle_a: 15.0,
            axonometric_angle_b: 12.0,
            axonometric_quadrant: "ne".into(),
            oblique_variant: "cavalier".into(),
            oblique_angle: 45.0,
            oblique_depth: 1.0,
            one_point_axis: "y".into(),
            fov: 50.0,
            two_point_shift: 0.0,
            curvilinear_fov: 120.0,
            curvilinear_strength: 1.0,
            curvilinear_mapping: "fisheye".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadCamera {
    #[serde(default = "default_camera_position")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default = "default_camera_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[serde(default = "one_f64")]
    pub zoom: f64,
    #[serde(default = "default_fov")]
    pub fov: f64,
    #[serde(default)]
    #[dsl(block)]
    pub projection: CadProjectionDsl,
}

impl Default for CadCamera {
    fn default() -> Self {
        Self { position: default_camera_position(), target: default_camera_target(), zoom: one_f64(), fov: default_fov(), projection: CadProjectionDsl::default() }
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
#[dsl(id = "cad.cad", layout = "lines")]
pub struct CadScene {
    pub schema: String,
    pub id: String,
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

fn default_model_definition_id() -> String {
    "spatial.shape".into()
}

pub fn empty_cad_projection() -> CadScene {
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
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

/// 🪆️ `Box<CadScene>` needs its own `dsl::DslField` binding for `CadOperation::SetScene` — `Box` is
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
//#endregion 🔖️Domain

//#region 🔖️ArtifactKind
/// 🗿️ The `3d.cad` artifact kind this plugin contributes — lifted out of the app manifest builder's
/// `.artifact_kind(…)` so the artifact node owns its own identity (schema, media capability, and the
/// import/export format set the kernel exposes for it).
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.cad".into(),
        name: "3D CAD".into(),
        source_format: "cad.scene".into(),
        component_kind: "cad".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::Brep,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Brep },
        schema: "cad.scene".into(),
        export_formats: vec![semio_framework_plugin::OsMediaFormat::Step, semio_framework_plugin::OsMediaFormat::Obj, semio_framework_plugin::OsMediaFormat::Stl, semio_framework_plugin::OsMediaFormat::Glb],
        import_formats: vec![semio_framework_plugin::OsMediaFormat::Step, semio_framework_plugin::OsMediaFormat::Obj, semio_framework_plugin::OsMediaFormat::Stl],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Testkit
/// 🧪️ Shared sample records for every cad artifact node's tests (diff/op/dsl/pack/spr) — one
/// definition instead of the four byte-identical copies the old per-module crates each carried.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;

    pub fn sample_object(id: &str) -> CadObject {
        CadObject {
            id: id.into(),
            label: "Box".into(),
            typology: "spatial.shape.box".into(),
            visible: true,
            locked: false,
            origin: [1.0, 2.0, 3.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: Some([1.0, 1.0, 1.0]),
            mesh_url: Some("https://example.test/mesh.glb".into()),
            extent: Some([2.0, 2.0, 2.0]),
            solid_handle: Some("solid-1".into()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }],
        }
    }

    pub fn sample_geometry() -> CadGeometry {
        CadGeometry {
            anchors: vec![serde_json::from_value(serde_json::json!({ "id": "anchor-1", "position": [0.0, 0.0, 0.0] })).expect("dsl value")],
            vertices: vec![CadVertex { id: "v1".into(), position: [0.0, 0.0, 0.0] }, CadVertex { id: "v2".into(), position: [1.0, 0.0, 0.0] }],
            edges: vec![CadEdge { id: "e1".into(), vertex_ids: vec!["v1".into(), "v2".into()], curve: CadEdgeCurve { kind: "line".into() } }],
            wires: vec![CadWire { id: "w1".into(), edge_ids: vec!["e1".into()] }],
            faces: vec![CadFace { id: "f1".into(), wire_ids: vec!["w1".into()], surface: CadPlaneSurface { kind: "plane".into(), origin: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] } }],
            shells: vec![CadShell { id: "s1".into(), face_ids: vec!["f1".into()] }],
            solids: vec![CadSolid { id: "sol1".into(), shell_ids: vec!["s1".into()] }],
        }
    }

    /// 🧊️ `sample_geometry` without the `anchors` row — the shape the old `🎒️pack` module's copy used.
    pub fn sample_geometry_without_anchors() -> CadGeometry {
        CadGeometry { anchors: Vec::new(), ..sample_geometry() }
    }

    pub fn sample_reference() -> CadReference {
        CadReference {
            id: "ref-1".into(),
            source_url: "https://example.test/plan.png".into(),
            media_kind: "image".into(),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: Some(1.5),
            width_world: 8.0,
            hidden: false,
            locked: true,
            opacity: Some(0.8),
        }
    }

    pub fn sample_scene() -> CadScene {
        sample_scene_with(sample_geometry())
    }

    pub fn sample_scene_with(geometry: CadGeometry) -> CadScene {
        let mut scene = empty_cad_projection();
        scene.objects.push(sample_object("object-1"));
        scene.building_objects.push(sample_object("object-2"));
        scene.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        scene.shape_geometry = Some(geometry);
        scene.references_by_model_definition_id.insert(CadPaneId::Shape.model_definition_id().to_string(), vec![sample_reference()]);
        scene.active_model_definition_id = CadPaneId::Shape.model_definition_id().to_string();
        scene
    }
}
//#endregion 🧪️Testkit
