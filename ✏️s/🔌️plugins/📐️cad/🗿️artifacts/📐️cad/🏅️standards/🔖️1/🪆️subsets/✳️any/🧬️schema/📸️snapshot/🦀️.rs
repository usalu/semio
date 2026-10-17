//! 🧬️ Cad snapshot schema — artifact-lane fields only.
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;

use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::CadReferenceIndex;




//#region 🔖️Snapshot
/// 📸️ Persisted cad document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the four per-pane object/geometry field
/// pairs that used to duplicate `SemioBrepSnapshot`'s topology inline (`CadObject`/`CadGeometry` at
/// `crate::🦀️.rs`) are replaced by four fixed composed
/// `s.stdio.semio.model` CHILD slots — one per `CadPaneId` — plus a forward `drawings` composition
/// slot per the design map's `cad | engineering assembly | model, drawing` row. `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
///
/// 🛡️ `deny_unknown_fields` closes that replacement: a snapshot still carrying the retired inline
/// `objects`/`shapeGeometry`/`activeModelDefinitionId` keys must FAIL to decode, never decode with
/// them silently dropped (`🧫️fixtures/🪪️document`'s `invalidDocuments`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "cad")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub building_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub energy_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub structure_classic_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub drawings: Vec<CadDrawingChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub breps: Vec<CadBrepChild>,
    #[value(default)]
    #[state(artifact)]
    pub references_by_model_definition_id: CadReferenceIndex,
    #[value(default)]
    #[state(artifact)]
    pub nodes: Vec<CadNode>,
}

//#region 🔖️ExactChildren



//#endregion 🔖️ExactChildren

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🔖️Domain
pub const CAD_DOCUMENT_SCHEMA: &str = "cad.scene";
pub const CAD_PLAY_DOCUMENT_SCHEMA: &str = "cad.document";
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`, `👁️viewer`) of the `✳️any` subset binds `ArtifactEditor::DIALECT`/
/// `ArtifactViewer::DIALECT` to — `"s.cad.cad"` matches the artifact-kind id this subset's own
/// mutation/inference descriptors already key off (see `definition()`'s `"s.cad.schema.artifact"`
/// row), standard `"1"` and subset `"*"` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any`
/// location. Lives at the artifact level (not under `editor`/`viewer`) so `policyViewerPurityBreaches`
/// never sees a viewer file importing through an `::editor::` path just to read this constant.
pub const CAD_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.cad.cad", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "kebab-case")]
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

    /// 🔢️ The pane's position in [`Self::all`].
    pub fn index(self) -> usize {
        self as usize
    }
}
/// 🧩️ Fixed per-pane composed `s.stdio.semio.model` child slot — one of the four fields the
/// derived `ArtifactSchema` on `CadSnapshot` classifies as a `#[child(...)]` slot. Kept as a plain
/// helper alias so accessor functions below read uniformly across all four panes.
pub type CadModelChild = store::os_store::artifact_child::ArtifactChild<SemioModelSnapshot>;
/// 📐️ Composed `s.stdio.semio.drawing` child — cad's forward-declared "engineering assembly"
/// composition slot (design-full-plan.md §4: `cad | engineering assembly | model, drawing`). Empty
/// today (cad carries no persisted 2D drawing content yet); real cardinality grows via
/// `create-drawing`/`delete-drawing` once a caller actually attaches one.
pub type CadDrawingChild = store::os_store::artifact_child::ArtifactChild<SemioDrawingSnapshot>;
/// 🧊️ Exact owned topology sibling of a pane model's geometry reference.
pub type CadBrepChild = store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot>;
/// 🪪️ Validates one owned CAD child reference before it enters a parent diff.
pub fn cad_child<S>(child_id: &str, target: &semio_framework_artifact_reference::ArtifactRef, expected_subset: &str) -> Result<store::os_store::artifact_child::ArtifactChild<S>, String> {
    if child_id.is_empty() || target.artifact_id.is_empty() || target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.standard != "v1" || target.dialect.subset != expected_subset {
        return Err(format!("CAD child target must own a nonempty identity and s.stdio.semio v1 {expected_subset} dialect"));
    }
    Ok(store::os_store::artifact_child::ArtifactChild::new(child_id.to_string(), target.clone()))
}
/// 🧩️ Validates a composed model child handle.
pub fn cad_model_child(child_id: &str, target: &semio_framework_artifact_reference::ArtifactRef) -> Result<CadModelChild, String> { cad_child(child_id, target, "model") }
/// 📐️ Validates a composed drawing child handle.
pub fn cad_drawing_child(child_id: &str, target: &semio_framework_artifact_reference::ArtifactRef) -> Result<CadDrawingChild, String> { cad_child(child_id, target, "drawing") }
/// 🧊️ Validates the topology child's exact dialect and local identity.
pub fn cad_brep_child(child_id: &str, target: &semio_framework_artifact_reference::ArtifactRef) -> Result<CadBrepChild, String> { cad_child(child_id, target, "brep") }
//#endregion 🔖️Domain
//#region 🔖️Domain
//#region 🔖️WorkingScene
/// 🧱️ EPHEMERAL working representation of per-pane object content and raw geometry — never persisted, never a
/// `CadSnapshot` field and never attached to a child handle: `CadSnapshot` composes only child HANDLES (`CadModelChild`,
/// two strings). It is what an importer derives from literal input before any child exists (the WRITE direction —
/// `🚪️io/🦀️.rs`'s `cad_document_from_dwg`/`scene_from_spatial_payload`) and what the bundled genesis catalogue holds per
/// pane ([`cad_bundled_pane_scene`]); readers see a pane through [`CadComposedPanes`].
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadWorkingScene {
    #[value(default)]
    pub objects: Vec<crate::geometry::CadObject>,
    #[value(default)]
    pub building_objects: Vec<crate::geometry::CadObject>,
    #[value(default)]
    pub energy_objects: Vec<crate::geometry::CadObject>,
    #[value(default)]
    pub structure_classic_objects: Vec<crate::geometry::CadObject>,
    #[value(default)]
    pub geometry: Option<crate::geometry::CadGeometry>,
    #[value(default)]
    pub building_geometry: Option<crate::geometry::CadGeometry>,
    #[value(default)]
    pub energy_geometry: Option<crate::geometry::CadGeometry>,
    #[value(default)]
    pub structure_classic_geometry: Option<crate::geometry::CadGeometry>,
}
/// 🌉 WRITE direction: a deterministic, content-addressed `s.stdio.semio.model` CHILD HANDLE for
/// `pane`, minted from `content_json` (mirrors `💠️lowpoly`'s `mesh_child_handle` — same pattern,
/// same shared `store::os_store::artifact_child::ArtifactChild::new`/`ArtifactDialect` shape). Two callers with byte-identical
/// content mint the same handle. This is a HANDLE only — the target artifact id is a forward
/// reference; materializing it as a real, persisted, resolvable document is the two-step
/// host-level gesture (mint the child, dispatch `create-<pane>-model` against the result)
/// `🚪️io/🦀️.rs`'s own comments already document, since minting the actual store entry
/// needs `ChildStoreFactory`/`CompositionCoordinator`, out of a pure function's reach.
pub fn cad_model_child_handle(pane: CadPaneId, content_json: &str) -> CadModelChild {
    let child_id = store::content_id(&format!("{}-model", cad_model_child_pane_slug(pane)), content_json.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
fn cad_model_child_pane_slug(pane: CadPaneId) -> &'static str {
    match pane {
        CadPaneId::Shape => "shape",
        CadPaneId::Building => "building",
        CadPaneId::Energy => "energy",
        CadPaneId::StructureClassic => "structure-classic",
    }
}
/// 🪪️ The STABLE composed model child of `pane` in the bundled document `document_id`: a NAME (`<document>-<pane>-model`),
/// never a digest of content — a child's content changes with every child-lane edit while its id stays (design §20.15),
/// so the genesis catalogue, the shipped assets and every reload agree on it and nothing is re-sealed when the model
/// bridge changes.
pub fn cad_named_pane_child(document_id: &str, pane: CadPaneId) -> CadModelChild {
    let child_id = format!("{document_id}-{}-model", cad_model_child_pane_slug(pane));
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
/// 🈳️ The composed model child an EMPTY pane of a new document is born with, so the document composes every pane from
/// its first render and an object gesture always has a child lane to land on (design §20.15). Its genesis is the empty
/// scene of the catalogue ([`cad_bundled_pane_scene`]).
pub fn cad_empty_pane_child(pane: CadPaneId) -> CadModelChild {
    cad_named_pane_child("cad", pane)
}
/// 🏷️ The composed `#[child]` slot name (camelCase field name) each pane's model child lives under.
pub fn cad_pane_model_slot(pane: CadPaneId) -> &'static str {
    match pane {
        CadPaneId::Shape => "shapeModel",
        CadPaneId::Building => "buildingModel",
        CadPaneId::Energy => "energyModel",
        CadPaneId::StructureClassic => "structureClassicModel",
    }
}
/// 🗂️ The composed `#[child]` slot name the document's drawing children live under.
pub const CAD_DRAWINGS_SLOT: &str = "drawings";
/// 🧬️ The bounded projection of this snapshot's composed child handles, as both surfaces hand it to
/// the framework (`ArtifactEditor::child_restore_projection` / `ArtifactViewer::child_restore_projection`).
pub fn cad_child_restore_projection(snapshot: &CadSnapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::from(format!("cad child projection failed: {error}")))
}
/// 🧱️ One pane's objects in a working scene.
pub fn cad_scene_pane_objects(scene: &CadWorkingScene, pane: CadPaneId) -> &[crate::geometry::CadObject] {
    match pane {
        CadPaneId::Shape => &scene.objects,
        CadPaneId::Building => &scene.building_objects,
        CadPaneId::Energy => &scene.energy_objects,
        CadPaneId::StructureClassic => &scene.structure_classic_objects,
    }
}
/// 📐️ One pane's raw geometry in a working scene.
pub fn cad_scene_pane_geometry(scene: &CadWorkingScene, pane: CadPaneId) -> Option<&crate::geometry::CadGeometry> {
    match pane {
        CadPaneId::Shape => scene.geometry.as_ref(),
        CadPaneId::Building => scene.building_geometry.as_ref(),
        CadPaneId::Energy => scene.energy_geometry.as_ref(),
        CadPaneId::StructureClassic => scene.structure_classic_geometry.as_ref(),
    }
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Domain
//#region 🔖️Domain
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct CadReference {
    pub id: String,
    pub source_url: String,
    #[value(default = "default_image_media_kind")]
    pub media_kind: String,
    #[value(default)]
    pub origin: [f64; 3],
    #[value(default)]
    pub orientation: Option<[f64; 4]>,
    /// 📐️ Uniform scale factor applied to the image plane (unlike `CadObject.scale`, references
    /// are flat and never scaled non-uniformly per axis — every call site only ever reads/writes
    /// a single number, see `apply_reference_patch`/`sample_reference` in `cad/op/rs`).
    #[value(default)]
    pub scale: Option<f64>,
    #[value(default = "default_width_world")]
    pub width_world: f64,
    #[value(default)]
    pub hidden: bool,
    #[value(default)]
    pub locked: bool,
    #[value(default)]
    pub opacity: Option<f64>,
}
fn default_image_media_kind() -> String {
    "image".into()
}
fn default_width_world() -> f64 {
    10.0
}
/// 📎 Map-value scalar for `referencesByModelDefinitionId` (schema parity).
pub type CadReferenceList = Vec<CadReference>;
/// 📐️ Local twin of `semio_framework_plugin::WorldProjectionConfig`'s flat 15-field classical
/// taxonomy (Parallel: Orthographic/Axonometric/Oblique, Perspective: 1/2/3-Point/Curvilinear) —
/// mirrored here rather than imported because `cad/rs` has no dependency on the plugin layer;
/// `cad/engine/rs`'s `cad_camera_projection_config`/`cad_camera_set_projection_config` convert
/// field-for-field between this and the real `WorldProjectionConfig` around the shared projection
/// helpers. See https://en.wikipedia.org/wiki/Axonometric_projection and
/// https://en.wikipedia.org/wiki/Oblique_projection.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct CadCamera {
    #[value(default = "default_camera_position")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default = "default_camera_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[value(default = "one_f64")]
    pub zoom: f64,
    #[value(default = "default_fov")]
    pub fov: f64,
    #[value(default)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct CadNode {
    pub id: String,
    pub label: String,
    pub kind: String,
}
/// 🧩️ Reads the fixed per-pane `s.stdio.semio.model` CHILD HANDLE (`child_id`/`target` only —
/// never the resolved content; a child is its own document, resolving it is a host/composition
/// concern, never something a pure `CadSnapshot` accessor can do — see `🔖️Composition` in
/// `🏪️store/🦀️.rs`).
pub fn cad_pane_model(scene: &CadSnapshot, pane: CadPaneId) -> Option<&CadModelChild> {
    match pane {
        CadPaneId::Shape => scene.shape_model.as_ref(),
        CadPaneId::Building => scene.building_model.as_ref(),
        CadPaneId::Energy => scene.energy_model.as_ref(),
        CadPaneId::StructureClassic => scene.structure_classic_model.as_ref(),
    }
}
pub fn cad_pane_model_mut(scene: &mut CadSnapshot, pane: CadPaneId) -> &mut Option<CadModelChild> {
    match pane {
        CadPaneId::Shape => &mut scene.shape_model,
        CadPaneId::Building => &mut scene.building_model,
        CadPaneId::Energy => &mut scene.energy_model,
        CadPaneId::StructureClassic => &mut scene.structure_classic_model,
    }
}
pub fn empty_cad_snapshot() -> CadSnapshot {
    CadSnapshot {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        shape_model: None,
        building_model: None,
        energy_model: None,
        structure_classic_model: None,
        drawings: Vec::new(),
        breps: Vec::new(),
        references_by_model_definition_id: CadReferenceIndex::new(),
        nodes: Vec::new(),
    }
}
pub fn cad_pane_from_model_definition_id(model_definition_id: &str) -> Option<CadPaneId> {
    CadPaneId::all().into_iter().find(|pane| pane.model_definition_id() == model_definition_id)
}
//#endregion 🔖️Domain

//#region 🔖️ArtifactKind
/// 🗿️ The `3d.cad` artifact kind this plugin contributes — lifted out of the app manifest builder's
/// `.artifact_kind(…)` so the artifact node owns its own identity (schema, media capability, and the
/// import/export format set the kernel exposes for it).
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.cad".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D CAD", "3D-CAD"),
        source_format: "cad.scene".into(),
        component_kind: "cad".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::Brep,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Brep },
        schema: "cad.scene".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["dwg".into(), "glb".into(), "gltf".into(), "ifc".into(), "json".into(), "obj".into(), "png".into(), "step".into(), "stl".into()],
        import_stdio_kinds: vec!["dwg".into(), "glb".into(), "gltf".into(), "ifc".into(), "json".into(), "obj".into(), "png".into(), "step".into(), "stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

/// 📸️ The snapshot facet's schema leaves (rust, typescript, graphql, json schema, proto).
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto"), };
