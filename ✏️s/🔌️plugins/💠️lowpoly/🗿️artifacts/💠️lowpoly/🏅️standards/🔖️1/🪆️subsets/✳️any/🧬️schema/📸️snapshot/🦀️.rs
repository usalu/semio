//! 🧬️ Lowpoly snapshot schema — artifact-lane fields only.
//!
//! `ArtifactDsl` and `ArtifactPack` are the derived spec-driven text and pack of the one
//! `dsl::DslRecord` spec, which carries each object's composed `mesh` child handle.
use protocol::{Identified, Patchable};
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use managed_mesh::LowpolyMeshState;

use framework_schema::ArtifactSchema;




//#region 🔖️Snapshot
/// 📸️ Persisted lowpoly document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "lowpoly")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub objects: Vec<LowpolyObject>,
}

impl Default for LowpolySnapshot {
    fn default() -> Self {
        Self { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️DocumentHelpers
//#endregion 🔖️DocumentHelpers

//#region 🔖️Pixels
/// 🎨️ 256² RGBA = 256 KiB per layer. A paint stroke's transient carries the layer's base AND its
/// scratch, and a transform drag's transient carries a whole snapshot clone, so a larger texture
/// pushed both past the store's fixed 1 MiB one-item publication bound — the publication was then
/// refused with `transient mutation exceeds the one-item publication bound`
/// (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS, 2026-09-17).
pub const LOWPOLY_PAINT_TEXTURE_SIZE: usize = 256;
pub const LOWPOLY_DOCUMENT_SCHEMA: &str = "lowpoly.document";
/// 🎨️ An opaque-white RGBA buffer sized for one paint layer.
pub fn empty_paint_pixels() -> Vec<u8> {
    let mut pixels = vec![0u8; LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4];
    for chunk in pixels.chunks_mut(4) {
        chunk[0] = 255;
        chunk[1] = 255;
        chunk[2] = 255;
        chunk[3] = 255;
    }
    pixels
}
//#endregion 🔖️Pixels
//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct LowpolyTransform {
    #[dsl(coord)]
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}
impl Default for LowpolyTransform {
    fn default() -> Self {
        Self { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], scale: [1.0, 1.0, 1.0] }
    }
}
/// 🖌️ One paint layer of an object: compositing metadata plus its persisted RGBA pixel buffer.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct LowpolyPaintLayer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub blend_mode: String,
    #[value(default = "empty_paint_pixels", with = "semio_framework_value::bytes")]
    #[dsl(base64)]
    pub pixels: Vec<u8>,
}
impl LowpolyPaintLayer {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), pixels: Vec::new() }
    }

    /// 🎨️ The layer's pixels with the sparse empty form materialised as the opaque-white default —
    /// what every in-place brush/fill/diff path must start from (a stamp into an empty `Vec` panics).
    pub fn materialized_pixels(&self) -> Vec<u8> {
        if self.pixels.is_empty() { empty_paint_pixels() } else { self.pixels.clone() }
    }

    /// 🎨️ Drops a never-painted buffer back to the sparse empty form (ticket 26/08/29/LOWPOLY-END-TO-END,
    /// 2026-09-17): a full 1024² layer is 4 MiB, four times the store's one-item cap, so every
    /// `addPrimitive` was refused. An empty `pixels` IS the opaque-white default everywhere it is read.
    pub fn compacted(mut self) -> Self {
        if !self.pixels.is_empty() && self.pixels.iter().all(|byte| *byte == 255) {
            self.pixels = Vec::new();
        }
        self
    }
}
/// 🕸️ Deterministic content-addressed CHILD handle for an object's mesh — same `(child_id, target)`
/// for identical geometry, a different pair once the geometry actually changes. Content-addressing
/// (rather than a per-object-stable id) is what lets the parent's own diff/mutation machinery detect
/// "the mesh changed" from the HANDLE alone, without ever comparing embedded content — the handle
/// IS the change signal. Shared by `snapshot_from_mesh_json` and the app's kernel session
/// (`⚙️engine::LowpolyDocument::sync_meshes_to_snapshot`, `add_primitive`), which both need the
/// identical rule so the same geometry always resolves to the same handle.
pub fn mesh_child_handle(object_id: &str, mesh_json: &str) -> store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot> {
    let child_id = store::content_id("mesh", mesh_json.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("{object_id}-mesh"), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
/// 🕸️ Managed geometry identity hashes the complete typed word and occurrence owner.
pub fn managed_mesh_child_handle(object_id: &str, state: &LowpolyMeshState) -> store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot> {
    let value=managed_mesh::json::state_value(state);
    let bytes=store::pack_rt::encode_wire_value(&value);
    semio_framework_value::FromValue::retire_decoded(value);
    let child_id=store::content_id("mesh",&bytes);
    let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"mesh".into()};
    store::os_store::artifact_child::ArtifactChild::new(child_id,semio_framework_artifact_reference::ArtifactRef{artifact_id:format!("{object_id}-mesh"),dialect})
}
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct LowpolyObject {
    pub id: String,
    pub name: String,
    pub transform: LowpolyTransform,
    pub smooth_shading: bool,
    /// 🕸️ Independently persisted identity of the optional mesh child.
    pub mesh: Option<store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot>>,
    #[value(default)]
    pub paint_layers: Vec<LowpolyPaintLayer>,
    /// 🧱️ Persisted mesh source text, preserved independently of the child identity.
    #[value(default)]
    pub mesh_content: String,
    /// 🕸️ The complete managed kernel state is independent of literal source text.
    #[canonical_json(tree)]
    #[value(with="managed_mesh::json")]
    pub mesh_state: Option<LowpolyMeshState>,
}
impl Identified<String> for LowpolyObject {
    fn id(&self) -> &String {
        &self.id
    }
}
/// 🎯️ Ephemeral component selection — never part of the document, threaded into the compute
/// session so mesh operations know their target vertices/edges/faces.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolySelectionTargets {
    pub mesh: bool,
    pub vertex: bool,
    pub edge: bool,
    pub face: bool,
}
impl Default for LowpolySelectionTargets {
    fn default() -> Self {
        Self { mesh: true, vertex: false, edge: false, face: false }
    }
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolySelection {
    #[value(default)]
    pub targets: LowpolySelectionTargets,
    #[value(default)]
    pub keys: Vec<String>,
    pub mode: String,
    pub ids: Vec<u32>,
}
impl Default for LowpolySelection {
    fn default() -> Self {
        Self { targets: LowpolySelectionTargets::default(), keys: Vec::new(), mode: "mesh".into(), ids: Vec::new() }
    }
}
//#endregion 🔖️Snapshot
//#region 🔖️Patches
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LowpolyObjectPatch {
    pub name: Option<String>,
    pub smooth_shading: Option<bool>,
    pub position: Option<[f32; 3]>,
    pub rotation: Option<[f32; 3]>,
    pub scale: Option<[f32; 3]>,
    /// 🕸️ Double-`Option`: outer = "this patch touches the `mesh` slot", inner = the new handle,
    /// `None` to clear it — matches `✳️object`'s own `mesh: Option<Option<ArtifactChild<…>>>` diff
    /// convention (`w2c-object-kit-report.md`). Never carries content, only `child_id`/`target`.
    /// Replaces the old `mesh_json: Option<String>` scalar patch field; `dsl::DslRecord` dropped
    /// from this struct (unused in practice — nothing calls its derived DSL machinery, confirmed by
    /// grep — and `store::os_store::artifact_child::ArtifactChild<S>` has no `DslField` impl to derive against anyway).
    pub mesh: Option<Option<store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot>>>,
    /// 🕸️ New `LowpolyObject::mesh_content`, set together with `mesh`.
    #[value(default)]
    pub mesh_content: Option<String>,
    #[value(default,with="managed_mesh::json::patch")]
    pub mesh_state: Option<Option<LowpolyMeshState>>,
}
impl Patchable<LowpolyObjectPatch> for LowpolyObject {
    fn apply_patch(&mut self, patch: &LowpolyObjectPatch) {
        if let Some(value) = &patch.name {
            self.name = value.clone();
        }
        if let Some(value) = patch.smooth_shading {
            self.smooth_shading = value;
        }
        if let Some(value) = patch.position {
            self.transform.position = value;
        }
        if let Some(value) = patch.rotation {
            self.transform.rotation = value;
        }
        if let Some(value) = patch.scale {
            self.transform.scale = value;
        }
        if let Some(value) = &patch.mesh {
            self.mesh = value.clone();
        }
        if let Some(value) = &patch.mesh_content {
            self.mesh_content = value.clone();

        }
        if let Some(value)=&patch.mesh_state { self.mesh_state=value.clone(); }
    }
}
//#endregion 🔖️Patches
//#region 🔖️Dialect
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`/`👁️viewer`) under this artifact's `✳️any` subset shares — lives at the
/// ARTIFACT level (not under either surface) specifically so `👁️viewer/🦀️.rs` can read it
/// without ever importing through the sibling editor module.
pub const LOWPOLY_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.lowpoly.lowpoly", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
//#endregion 🔖️Dialect
//#region 🔖️ArtifactKind
/// 🧱️ The two artifact kinds this plugin contributes — lifted out of the old ui crate's manifest
/// builder chain so the app's `🔖️Manifest` region can stitch it in as a single passthrough.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.lowpoly".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D Lowpoly", "3D-Lowpoly"),
        source_format: "lowpoly.fixture".into(),
        component_kind: "lowpoly".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

/// 📸️ The snapshot facet's schema leaves (rust, typescript, graphql, json schema, proto).
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto"), };
