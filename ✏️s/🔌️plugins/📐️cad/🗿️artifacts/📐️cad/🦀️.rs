//! 📐️ CAD artifact — the `cad.scene` document schema: the `CadSnapshot` projection, its object/
//! reference/geometry/camera records, and the pane vocabulary every other cad node addresses them by.
//! The declarative `spatial.interaction` spec types live beside this file in
//! `🎬️interaction-spec/🦀️.rs`.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_cad_demo_tests;
extern crate semio_framework_schema as framework_schema;

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
pub use crate::schema::CadReferenceIndex;

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
pub type CadModelChild = store::ArtifactChild<SemioModelSnapshot>;

/// 📐️ Composed `s.stdio.semio.drawing` child — cad's forward-declared "engineering assembly"
/// composition slot (design-full-plan.md §4: `cad | engineering assembly | model, drawing`). Empty
/// today (cad carries no persisted 2D drawing content yet); real cardinality grows via
/// `create-drawing`/`delete-drawing` once a caller actually attaches one.
pub type CadDrawingChild = store::ArtifactChild<SemioDrawingSnapshot>;

/// 🪪️ Validates one owned CAD child reference before it enters a parent diff.
pub fn cad_child<S>(child_id: &str, target: &semio_framework_artifact_reference::ArtifactRef, expected_subset: &str) -> Result<store::ArtifactChild<S>, String> {
    if child_id.is_empty() || target.artifact_id.is_empty() || target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.standard != "v1" || target.dialect.subset != expected_subset {
        return Err(format!("CAD child target must own a nonempty identity and s.stdio.semio v1 {expected_subset} dialect"));
    }
    Ok(store::ArtifactChild::new(child_id.to_string(), target.clone()))
}

/// 🧩️ Validates a composed model child handle.
pub fn cad_model_child(child_id: &str, target: &semio_framework_artifact_reference::ArtifactRef) -> Result<CadModelChild, String> { cad_child(child_id, target, "model") }

/// 📐️ Validates a composed drawing child handle.
pub fn cad_drawing_child(child_id: &str, target: &semio_framework_artifact_reference::ArtifactRef) -> Result<CadDrawingChild, String> { cad_child(child_id, target, "drawing") }

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
    pub(crate) objects: Vec<standards::v1::subsets::any::schema::geometry::CadObject>,
    #[value(default)]
    pub(crate) building_objects: Vec<standards::v1::subsets::any::schema::geometry::CadObject>,
    #[value(default)]
    pub(crate) energy_objects: Vec<standards::v1::subsets::any::schema::geometry::CadObject>,
    #[value(default)]
    pub(crate) structure_classic_objects: Vec<standards::v1::subsets::any::schema::geometry::CadObject>,
    #[value(default)]
    pub(crate) geometry: Option<standards::v1::subsets::any::schema::geometry::CadGeometry>,
    #[value(default)]
    pub(crate) building_geometry: Option<standards::v1::subsets::any::schema::geometry::CadGeometry>,
    #[value(default)]
    pub(crate) energy_geometry: Option<standards::v1::subsets::any::schema::geometry::CadGeometry>,
    #[value(default)]
    pub(crate) structure_classic_geometry: Option<standards::v1::subsets::any::schema::geometry::CadGeometry>,
}

/// 🌉 WRITE direction: a deterministic, content-addressed `s.stdio.semio.model` CHILD HANDLE for
/// `pane`, minted from `content_json` (mirrors `💠️lowpoly`'s `mesh_child_handle` — same pattern,
/// same shared `store::ArtifactChild::new`/`ArtifactDialect` shape). Two callers with byte-identical
/// content mint the same handle. This is a HANDLE only — the target artifact id is a forward
/// reference; materializing it as a real, persisted, resolvable document is the two-step
/// host-level gesture (mint the child, dispatch `create-<pane>-model` against the result)
/// `🚪️io/🦀️.rs`'s own comments already document, since minting the actual store entry
/// needs `ChildStoreFactory`/`CompositionCoordinator`, out of a pure function's reach.
pub fn cad_model_child_handle(pane: CadPaneId, content_json: &str) -> CadModelChild {
    let child_id = store::content_id(&format!("{}-model", cad_model_child_pane_slug(pane)), content_json.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
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
    store::ArtifactChild::new(child_id, target)
}

/// 🈳️ The composed model child an EMPTY pane of a new document is born with, so the document composes every pane from
/// its first render and an object gesture always has a child lane to land on (design §20.15). Its genesis is the empty
/// scene of the catalogue ([`cad_bundled_pane_scene`]).
pub fn cad_empty_pane_child(pane: CadPaneId) -> CadModelChild {
    cad_named_pane_child("cad", pane)
}

/// 📚️ The immutable catalogue of every pane genesis scene the plugin ships, keyed by the pane child's stable `child_id`
/// ([`cad_named_pane_child`]): the bundled examples' panes and the four empty panes ([`cad_empty_pane_child`]). A hit is
/// the scene the document was born with: its objects are the child's genesis content and its raw geometry is what
/// `SemioModelSnapshot` has no field for. This resolves shipped assets; it never shares one live document's state with
/// another.
pub(crate) fn cad_bundled_pane_scene(child_id: &str) -> Option<std::sync::Arc<CadWorkingScene>> {
    static CATALOGUE: std::sync::OnceLock<Vec<(String, std::sync::Arc<CadWorkingScene>)>> = std::sync::OnceLock::new();
    CATALOGUE
        .get_or_init(|| {
            let forest = standards::v1::subsets::any::schema::inferences::forest_play_scene();
            let empty = std::sync::Arc::new(CadWorkingScene::default());
            let bundled = CadPaneId::all().into_iter().filter_map(|pane| Some((cad_pane_model(&forest, pane)?.child_id.clone(), standards::v1::subsets::any::io::geometry_import::forest_pane_scene(pane))));
            bundled.chain(CadPaneId::all().into_iter().map(|pane| (cad_empty_pane_child(pane).child_id, empty.clone()))).collect()
        })
        .iter()
        .find(|(id, _)| id == child_id)
        .map(|(_, scene)| scene.clone())
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

/// 🌱️ `ArtifactEditor`/`ArtifactViewer::genesis_child_pack`: the pack an owned child the snapshot declares is born with
/// when no store carries it (design §20.15, §22.36). A pane model child whose stable id names a catalogue scene
/// ([`cad_bundled_pane_scene`]) is born with that scene's objects; every other declared child — the one a
/// `create-*-model` or `create-drawing` leaf hands out — is born EMPTY, because a handle is two strings: content reaches
/// a child only as an archive member or as child-lane leaves, never through its name. So a parent decoded from pack or
/// text with no archive members (the React shell's `loadDocumentPair` sends `members: []`) composes every declared
/// child, and a fresh instance plus a creation leaf is a loadable document. A child the snapshot does not declare has
/// no genesis.
pub fn cad_genesis_child_pack(snapshot: &CadSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use store::ArtifactPack;
    if slot == CAD_DRAWINGS_SLOT {
        return snapshot.drawings.iter().any(|child| child.child_id == child_id).then(|| <SemioDrawingSnapshot as ArtifactPack>::encode_pack(&SemioDrawingSnapshot::default()));
    }
    let pane = CadPaneId::all().into_iter().find(|pane| cad_pane_model_slot(*pane) == slot)?;
    cad_pane_model(snapshot, pane).filter(|child| child.child_id == child_id)?;
    let genesis = cad_bundled_pane_scene(child_id);
    let objects = genesis.as_deref().map_or(&[][..], |scene| cad_scene_pane_objects(scene, pane));
    Some(<SemioModelSnapshot as ArtifactPack>::encode_pack(&standards::v1::subsets::any::schema::geometry::semio_model_snapshot_from_objects(objects)))
}

/// 🧬️ The bounded projection of this snapshot's composed child handles, as both surfaces hand it to
/// the framework (`ArtifactEditor::child_restore_projection` / `ArtifactViewer::child_restore_projection`).
pub fn cad_child_restore_projection(snapshot: &CadSnapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::from(format!("cad child projection failed: {error}")))
}

/// 🧱️ One pane's objects in a working scene.
pub(crate) fn cad_scene_pane_objects(scene: &CadWorkingScene, pane: CadPaneId) -> &[standards::v1::subsets::any::schema::geometry::CadObject] {
    match pane {
        CadPaneId::Shape => &scene.objects,
        CadPaneId::Building => &scene.building_objects,
        CadPaneId::Energy => &scene.energy_objects,
        CadPaneId::StructureClassic => &scene.structure_classic_objects,
    }
}

/// 📐️ One pane's raw geometry in a working scene.
pub(crate) fn cad_scene_pane_geometry(scene: &CadWorkingScene, pane: CadPaneId) -> Option<&standards::v1::subsets::any::schema::geometry::CadGeometry> {
    match pane {
        CadPaneId::Shape => scene.geometry.as_ref(),
        CadPaneId::Building => scene.building_geometry.as_ref(),
        CadPaneId::Energy => scene.energy_geometry.as_ref(),
        CadPaneId::StructureClassic => scene.structure_classic_geometry.as_ref(),
    }
}

/// 🪆️ One pane as every reader sees it: its objects read out of the pane's composed `s.stdio.semio@v1/model` child, over
/// the immutable genesis scene its stable child id names ([`cad_bundled_pane_scene`]) — the raw geometry the model has no
/// field for, and the `Arc` whose identity keys the pane's tessellation cache.
#[derive(Clone, Debug, Default)]
pub(crate) struct CadComposedPane {
    pub(crate) objects: Vec<standards::v1::subsets::any::schema::geometry::CadObject>,
    pub(crate) genesis: Option<std::sync::Arc<CadWorkingScene>>,
}

impl CadComposedPane {
    /// 📐️ The pane's raw genesis geometry, when its child id names a bundled scene.
    pub(crate) fn geometry(&self, pane: CadPaneId) -> Option<&standards::v1::subsets::any::schema::geometry::CadGeometry> {
        self.genesis.as_deref().and_then(|scene| cad_scene_pane_geometry(scene, pane))
    }
}

/// 🪆️ The four panes every reader works on (design §20.15): each pane's composed child read through the child view only.
/// The child is the pane's one source of truth, so an edit, an undo, a remote change or a reload is seen the moment its
/// child store has it; a pane whose child is not composed yet reads the genesis content its stable id names, and a pane
/// without a handle has no objects.
#[derive(Clone, Debug, Default)]
pub(crate) struct CadComposedPanes([CadComposedPane; 4]);

impl CadComposedPanes {
    /// 🪆️ Composes `snapshot`'s pane handles with their live child content.
    pub(crate) fn compose(snapshot: &CadSnapshot, children: &semio_framework_plugin::ChildContentView) -> Self {
        use standards::v1::subsets::any::schema::geometry::objects_from_model_snapshot;
        Self(CadPaneId::all().map(|pane| {
            let Some(handle) = cad_pane_model(snapshot, pane) else { return CadComposedPane::default() };
            let slot = cad_pane_model_slot(pane);
            let genesis = cad_bundled_pane_scene(&handle.child_id);
            let modelled = children.dialect(slot, &handle.child_id).is_some_and(|dialect| dialect.artifact_kind == "s.stdio.semio" && dialect.standard == "v1" && dialect.subset == "model");
            let objects = match modelled.then(|| children.typed_read::<SemioModelSnapshot>(slot, &handle.child_id).ok()).flatten() {
                Some(content) => objects_from_model_snapshot(&content),
                None => genesis.as_deref().map(|scene| cad_scene_pane_objects(scene, pane).to_vec()).unwrap_or_default(),
            };
            CadComposedPane { objects, genesis }
        }))
    }

    /// 🔎️ `pane` as composed.
    pub(crate) fn pane(&self, pane: CadPaneId) -> &CadComposedPane {
        &self.0[pane.index()]
    }
}
//#endregion 🔖️WorkingScene

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
        references_by_model_definition_id: CadReferenceIndex::new(),
        nodes: Vec::new(),
    }
}

pub fn cad_pane_from_model_definition_id(model_definition_id: &str) -> Option<CadPaneId> {
    CadPaneId::all().into_iter().find(|pane| pane.model_definition_id() == model_definition_id)
}

//#region 🔖️Snapshot
pub use crate::schema::diff::CadDiff;
pub use crate::schema::mutations::CadMutation;
/// 📸️ Re-export persisted snapshot type (defined in snapshot schema facet).
pub use crate::schema::snapshot::CadSnapshot;
//#endregion 🔖️Snapshot

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

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `🗒️note`'s own `pilot_languages()` convention (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// M1/W1b). Relocated from `⚙️engine/🦀️.rs` alongside `declaration()` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — `declaration()`'s only caller, kept private.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "cad.document",
                    extension: Some("cad"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("cad.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "cad.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("cad.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "cad.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("cad.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "cad.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("cad.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "cad.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("cad.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration freezes its schema, inference, codec, composer, and language
/// contributions before plugin assembly.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.cad.cad.standard.v1", "standard", "1", &[], None),
        ("s.cad.cad.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.cad.cad.schema.artifact", "schema", "s.cad.cad", &[("schema", "s.cad.cad")], None),
        ("s.cad.cad.inference.artifact", "inference", "s.cad.cad.inference", &[("schema", "s.cad.cad.inference")], None),
        ("s.cad.cad.composer.native", "composer", "s.cad.cad@1/*", &[("dialect", "s.cad.cad@1/*")], None),
        ("s.cad.cad.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.cad.cad.grammar.document", "grammar", "cad.document", &[("grammar", "cad.document")], None),
        ("s.cad.cad.grammar.op", "grammar", "cad.op", &[("grammar", "cad.op")], None),
        ("s.cad.cad.grammar.diff", "grammar", "cad.diff", &[("grammar", "cad.diff")], None),
        ("s.cad.cad.grammar.pack", "grammar", "cad.pack", &[("grammar", "cad.pack")], None),
        ("s.cad.cad.grammar.spr", "grammar", "cad.spr", &[("grammar", "cad.spr")], None),
        ("s.cad.cad.codec.document.v1", "codec", "cad.scene:cad", &[("codec", "cad.scene"), ("codec-extension", "9:cad.scene:cad")], None),
        ("s.cad.cad.localization.en", "localization", "CAD", &[], Some(("en", "CAD"))),
        ("s.cad.cad.localization.de", "localization", "CAD-Modellierung", &[], Some(("de", "CAD-Modellierung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.cad.cad")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::cad_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::cad_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::app::EditorApp<editor::cad::CadPlayApp>>()
        .try_build()
}
//#endregion 🔖️ArtifactKind

//#region 🧫️Fixtures
/// 🧪️ Shared sample records for every cad artifact node's tests (diff/op/dsl/pack/spr) — one
/// definition instead of the four byte-identical copies the old per-module crates each carried.
#[cfg(test)]
#[path = "🧪️tests/🧩️sample-scene/🦀️.rs"]
pub(crate) mod sample_scene_fixture;
//#endregion 🧫️Fixtures

#[path = "🎬️interaction-spec/🦀️.rs"]
mod interaction_spec;
pub use interaction_spec::*;

// 🐛️ op/dsl/spr used to be their own #[path] mounts of the SAME files the schema tree
// below also mounts (mutations/text, snapshot/text, mutations/binary respectively) --
// mounting the same source twice creates two non-unified module instances with
// conflicting trait impls and mismatched types of the "same" struct. Re-export the
// ALREADY-mounted schema-tree module instead of re-compiling the file a second time.
pub use standards::v1::subsets::any::io::binary::mutations as spr;
pub use standards::v1::subsets::any::io::text::mutations as op;
pub use standards::v1::subsets::any::io::text::snapshot as document_dsl;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod demo {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📐️geometry/🦀️.rs"]
                    pub mod geometry;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod create_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-node/🧪️tests/🌱️appends-node-3/🦀️.rs"]
                            mod tests_appends_node_3;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/🚫️removes-node-2/🦀️.rs"]
                            mod tests_removes_node_2;
                        }
                        #[path = "."]
                        pub mod rename_node {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-node/🧪️tests/🔤️relabels-the-root-node/🦀️.rs"]
                            mod tests_relabels_the_root_node;
                        }
                        #[path = "."]
                        pub mod change_reference_hidden {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-reference-hidden/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-reference-hidden/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-reference-hidden/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-reference-hidden/🧪️tests/🙈️hides/🦀️.rs"]
                            mod tests_hides_the_shape_reference;
                        }
                        #[path = "."]
                        pub mod change_reference_locked {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-reference-locked/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-reference-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-reference-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-reference-locked/🧪️tests/🔓️unlocks/🦀️.rs"]
                            mod tests_unlocks_the_shape_reference;
                        }
                        #[path = "."]
                        pub mod change_reference_width {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-reference-width/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-reference-width/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-reference-width/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-reference-width/🧪️tests/↔️widens/🦀️.rs"]
                            mod tests_widens_the_shape_reference_plane;
                        }
                        #[path = "."]
                        pub mod move_reference {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-reference/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-reference/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-reference/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-reference/🧪️tests/📍️moves/🦀️.rs"]
                            mod tests_moves_the_shape_reference_off_origin;
                        }
                        #[path = "."]
                        pub mod replace_reference_media {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️replace-reference-media/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️replace-reference-media/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️replace-reference-media/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️replace-reference-media/🧪️tests/🖼️reattaches/🦀️.rs"]
                            mod tests_reattaches_the_shape_reference_to_a_new_plan;
                        }
                        #[path = "."]
                        pub mod replace_references {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎replace-references/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎replace-references/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎replace-references/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎replace-references/🧪️tests/🔄️swaps/🦀️.rs"]
                            mod tests_swaps_the_shape_reference_list;
                        }
                        #[path = "."]
                        pub mod create_shape_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-shape-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-shape-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-shape-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-shape-model/🧪️tests/🧱️rehandles/🦀️.rs"]
                            mod tests_rehandles_the_occupied_shape_slot;
                        }
                        #[path = "."]
                        pub mod delete_shape_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-shape-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-shape-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-shape-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-shape-model/🧪️tests/🕳️vacates-the-shape-slot/🦀️.rs"]
                            mod tests_vacates_the_shape_slot;
                        }
                        #[path = "."]
                        pub mod create_building_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢create-building-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢create-building-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢create-building-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢create-building-model/🧪️tests/🏢️rehandles/🦀️.rs"]
                            mod tests_rehandles_the_occupied_building_slot;
                        }
                        #[path = "."]
                        pub mod delete_building_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥delete-building-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥delete-building-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥delete-building-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥delete-building-model/🧪️tests/🏚️vacates/🦀️.rs"]
                            mod tests_vacates_the_building_slot;
                        }
                        #[path = "."]
                        pub mod create_energy_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡create-energy-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡create-energy-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡create-energy-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡create-energy-model/🧪️tests/⚡️rehandles/🦀️.rs"]
                            mod tests_rehandles_the_occupied_energy_slot;
                        }
                        #[path = "."]
                        pub mod delete_energy_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌delete-energy-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌delete-energy-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌delete-energy-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌delete-energy-model/🧪️tests/🔌️vacates/🦀️.rs"]
                            mod tests_vacates_the_energy_slot;
                        }
                        #[path = "."]
                        pub mod create_structure_classic_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-structure-classic/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-structure-classic/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-structure-classic/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-structure-classic/🧪️tests/🏛️rehandles/🦀️.rs"]
                            mod tests_rehandles_the_occupied_structure_classic_slot;
                        }
                        #[path = "."]
                        pub mod delete_structure_classic_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣delete-structure-classic-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣delete-structure-classic-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣delete-structure-classic-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣delete-structure-classic-model/🧪️tests/🏚️vacates/🦀️.rs"]
                            mod tests_vacates_the_structure_classic_slot;
                        }
                        #[path = "."]
                        pub mod create_drawing {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️create-drawing/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️create-drawing/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️create-drawing/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️create-drawing/🧪️tests/📐️appends-drawing-2/🦀️.rs"]
                            mod tests_appends_drawing_2;
                        }
                        #[path = "."]
                        pub mod delete_drawing {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-drawing/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-drawing/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-drawing/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-drawing/🧪️tests/🚫️removes-drawing-1/🦀️.rs"]
                            mod tests_removes_drawing_1;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🗺️geometry-import/🦀️.rs"]
                    pub mod geometry_import;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }

}

/// ✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the mutation-capable surface, migrated
/// wholesale from the retired `🎛️apps/📐️cad/` app tree into the owned subset's `✏️editor/` facet.
/// `⚙️engine` is not `surfaceChildDirs` vocabulary (app-only, pre-migration) — kept physically nested
/// under `✏️editor/` since every one of its callers (mode/window render, engagement commands) is
/// editor-only; see the packet's migration report for the full rationale.
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod cad {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod engine {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🕹️interaction/🦀️.rs"]
            pub mod interaction;
            /// 🧲️ Crate-local: the pick engine reads the EPHEMERAL `CadGeometry`/`CadObject` import
            /// types, which are `pub(crate)` by design (`🚪️io/🗺️geometry-import/🦀️.rs`'s module doc),
            /// so its own surface must not exceed theirs.
            ///
            /// 🧯️ `dead_code` — this module is a COMPLETE, test-pinned port of the React renderer's
            /// `🧲️GeometryTargets`/`🧲️GeometryInteraction` regions (ticket
            /// `26/09/17/WGPU-RENDERER-REACT-PARITY` packet W2f), so every React export has a twin
            /// here whether or not the play app calls it yet. Today `modes::edit` consumes the
            /// pick-target/visibility/entity-flag half through the `engagementPreview` overlay and
            /// `panels::inspection` the census half; the selection-merge, hover-alias, reveal and
            /// target-style half has no production caller until `World3dScene` grows a pick-target
            /// lane that both hosts read (W2f report §5, hand-off 1). Splitting the port to match
            /// today's call sites would leave the two implementations diverging silently, which is
            /// exactly what the parity laws in `🧲️picking/🧪️tests/🔬️unit/🦀️.rs` exist to prevent.
            #[allow(dead_code)]
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧲️picking/🦀️.rs"]
            pub(crate) mod picking;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧬️typology/🦀️.rs"]
            pub mod typology;
        }

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️camera/🦀️.rs"]
            pub mod camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️contribution/🦀️.rs"]
            pub mod contribution;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement/🦀️.rs"]
            pub mod engagement;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️io/🦀️.rs"]
            pub mod io;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️model-definition/🦀️.rs"]
            pub mod model_definition;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node/🦀️.rs"]
            pub mod node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️object/🦀️.rs"]
            pub mod object;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️reference/🦀️.rs"]
            pub mod reference;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️sun/🦀️.rs"]
            pub mod sun;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️transform/🦀️.rs"]
            pub mod transform;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️utility/🦀️.rs"]
            pub mod utility;
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🕹️dislocate/🦀️.rs"]
                    pub mod dislocate;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🎥️projection/🦀️.rs"]
                    pub mod projection;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🌞️sun/🦀️.rs"]
                    pub mod sun;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🧭️transform/🦀️.rs"]
                    pub mod transform;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🦀️.rs"]
                    pub mod config;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🫧️transient/🦀️.rs"]
                    pub mod transient;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏢️building/🦀️.rs"]
                    pub mod building;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔥️energy/🦀️.rs"]
                    pub mod energy;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️shape/🦀️.rs"]
                    pub mod shape;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏛️structure-classic/🦀️.rs"]
                    pub mod structure_classic;
                }
            }
        }
    }
}

/// 👁️ The read-only surface (contract §2.2/§2.6) — a genuinely independent module tree from
/// `editor` above, never `#[path]`-mounting anything under `✏️editor/`: that would let
/// `policyViewerPurityBreaches`' `::editor::` substring check catch a real dependency, but the
/// deeper reason is architectural — `CadViewer` must stay constructible without ever touching
/// `CadPlayApp`'s mutation-capable types.
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod cad {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📐️shape/🦀️.rs"]
                    pub mod shape;
                }
            }
        }
    }
}

//#region 📚️Examples
pub use standards::v1::subsets::any::examples;
//#endregion 📚️Examples

pub use crate::standards::v1::subsets::any::io::{CadBuilderConstruction, CadParts, CadAnalyzerAnalysis, CadBuilderFacets, CadBuilder, CadAnalyzer, CadComposer};

#[path = "."]
pub mod host {
#[path = "🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
