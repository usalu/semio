//! 🔷️ Lowpoly artifact — the mesh + paint document projection, its patch types, and the ephemeral
//! selection value threaded into the compute session (never part of the persisted document).

use protocol::{Identified, Patchable};
use serde::{Deserialize, Serialize};

//#region 🔖️Pixels
pub const LOWPOLY_PAINT_TEXTURE_SIZE: usize = 1024;
pub const LOWPOLY_DOCUMENT_SCHEMA: &str = "lowpoly.document";

/// @emoji 🎨️ An opaque-white RGBA buffer sized for one paint layer.
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

/// @emoji 🧬️ Base64 (de)serialization for a raw RGBA layer buffer so persisted documents stay ~1.4 MB
/// per layer instead of a multi-megabyte JSON integer array. Empty/missing decodes to opaque white.
mod pixels_base64 {
    use base64::Engine;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(pixels: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(pixels))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        if encoded.is_empty() {
            return Ok(super::empty_paint_pixels());
        }
        base64::engine::general_purpose::STANDARD.decode(encoded.as_bytes()).map_err(serde::de::Error::custom)
    }
}
//#endregion 🔖️Pixels

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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

/// @emoji 🖌️ One paint layer of an object: compositing metadata plus its persisted RGBA pixel buffer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyPaintLayer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub blend_mode: String,
    #[serde(with = "pixels_base64", default = "empty_paint_pixels")]
    #[dsl(base64)]
    pub pixels: Vec<u8>,
}

impl LowpolyPaintLayer {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), pixels: empty_paint_pixels() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyObject {
    pub id: String,
    pub name: String,
    #[dsl(block)]
    pub transform: LowpolyTransform,
    pub smooth_shading: bool,
    // ⚠️ Deliberately plain `Shape::Text` (bare `String`, no `#[dsl(lang = "json")]`), NOT an
    // oversight: `LowpolyObject` is the element type of `LowpolySnapshot.objects: Vec<LowpolyObject>`,
    // a `Shape::List(Shape::Record(..))` field printed in `JoinMode::Document`. The engine's `Writer`
    // only forces a line break after a `Shape::Embed` field's closing fence when the NEXT chunk is
    // pushed via `new_record()` (as `Shape::Block`/`Shape::Table` fields do) — plain list-item
    // iteration and the list's own closing `]` atom do not, so annotating this field glues the fence's
    // closing "```" to the following `]` on the same text line and breaks the fence lexer's "closing
    // ``` must be alone on its line" rule. Confirmed empirically (fails even with a single object):
    // reparsing the printed output errors with "unterminated fenced block". This is a genuine ENGINE
    // GAP distinct from the already-documented multi-`Shape::Embed`-field Lines-layout bug (see
    // `trinity::rewrite::RewriteRuleModel`) — out of scope here, do not annotate until the engine's
    // `Writer` forces a boundary after every `Shape::Embed` chunk regardless of what follows it.
    pub mesh_json: String,
    #[serde(default)]
    pub paint_layers: Vec<LowpolyPaintLayer>,
}

impl Identified<String> for LowpolyObject {
    fn id(&self) -> &String {
        &self.id
    }
}

/// @emoji 📸️ Persisted lowpoly snapshot: schema plus mesh objects (geometry, transform, shading,
/// paint layers). Non-persistent fields live on [`schema::LowpolyArtifact`](crate::artifacts::lowpoly::schema::LowpolyArtifact).
pub use crate::artifacts::lowpoly::snapshot::schema::LowpolySnapshot;





pub use crate::artifacts::lowpoly::snapshot::schema::snapshot_from_mesh_json;

/// @emoji 🎯️ Ephemeral component selection — never part of the document, threaded into the compute
/// session so mesh operations know their target vertices/edges/faces.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolySelection {
    #[serde(default)]
    pub targets: LowpolySelectionTargets,
    #[serde(default)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyObjectPatch {
    pub name: Option<String>,
    pub smooth_shading: Option<bool>,
    pub transform: Option<LowpolyTransform>,
    // 🧬️ Unlike `LowpolyObject::mesh_json` (see its doc comment for the confirmed engine-gap reason
    // it stays plain), `#[dsl(lang = "json")]` IS safe here: `LowpolyObjectPatch` only derives
    // `dsl::DslRecord` (never `DslDocument`) and is only ever printed through `DslOps::print_op`,
    // which always uses `JoinMode::Inline` — `Shape::Embed` renders as an ordinary escaped quoted
    // string in Inline mode (never a fence), so the Document-mode list-nesting fence-glue bug this
    // record never reaches simply doesn't apply. Confirmed via the existing `op_text_round_trip_*`
    // tests in the `op` component.
    #[dsl(lang = "json")]
    pub mesh_json: Option<String>,
}

impl Patchable<LowpolyObjectPatch> for LowpolyObject {
    fn apply_patch(&mut self, patch: &LowpolyObjectPatch) {
        if let Some(value) = &patch.name {
            self.name = value.clone();
        }
        if let Some(value) = patch.smooth_shading {
            self.smooth_shading = value;
        }
        if let Some(value) = &patch.transform {
            self.transform = value.clone();
        }
        if let Some(value) = &patch.mesh_json {
            self.mesh_json = value.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<LowpolyObjectPatch> {
        let patch = LowpolyObjectPatch {
            name: (self.name != other.name).then(|| other.name.clone()),
            smooth_shading: (self.smooth_shading != other.smooth_shading).then_some(other.smooth_shading),
            transform: (self.transform != other.transform).then(|| other.transform.clone()),
            mesh_json: (self.mesh_json != other.mesh_json).then(|| other.mesh_json.clone()),
        };
        (patch != LowpolyObjectPatch::default()).then_some(patch)
    }
}

/// 🖌️ Applies a paint-layers sub-delta onto one object.
pub fn apply_paint_layers_delta(object: &mut LowpolyObject, delta: &crate::artifacts::lowpoly::diff::schema::LowpolyPaintLayersDelta) {
    for index in delta.removed.iter().copied().rev() {
        let i = index as usize;
        if i < object.paint_layers.len() {
            object.paint_layers.remove(i);
        }
    }
    for entry in &delta.added {
        let i = (entry.index as usize).min(object.paint_layers.len());
        object.paint_layers.insert(i, entry.layer.clone());
    }
    for entry in &delta.patched {
        let i = entry.index as usize;
        if let Some(layer) = object.paint_layers.get_mut(i) {
            let p = &entry.patch;
            if let Some(value) = &p.name { layer.name = value.clone(); }
            if let Some(value) = p.visible { layer.visible = value; }
            if let Some(value) = p.opacity { layer.opacity = value; }
            if let Some(value) = &p.blend_mode { layer.blend_mode = value.clone(); }
        }
    }
    for stroke in &delta.strokes {
        let i = stroke.layer_index as usize;
        if let Some(layer) = object.paint_layers.get_mut(i) {
            for run in &stroke.runs {
                let start = run.offset as usize;
                let end = (start + run.bytes.len()).min(layer.pixels.len());
                if start < layer.pixels.len() {
                    layer.pixels[start..end].copy_from_slice(&run.bytes[..end - start]);
                }
            }
        }
    }
}
//#endregion 🔖️Patches

//#region 🔖️ArtifactKind
/// 🧱️ The two artifact kinds this plugin contributes — lifted out of the old ui crate's manifest
/// builder chain so the app's `🔖️Manifest` region can stitch it in as a single passthrough.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.lowpoly".into(),
        name: "3D Lowpoly".into(),
        source_format: "lowpoly.fixture".into(),
        component_kind: "lowpoly".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        schema: "lowpoly.fixture".into(),
        export_formats: vec![semio_framework_plugin::MediaFormat::Glb, semio_framework_plugin::MediaFormat::Obj, semio_framework_plugin::MediaFormat::Stl],
        import_formats: vec![semio_framework_plugin::MediaFormat::Glb, semio_framework_plugin::MediaFormat::Obj],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}

/// 🧱️ The shared `3d.mesh` interchange kind — declared alongside `artifact_kind()` because several
/// sibling plugins declare the identical shape privately; kept here so lowpoly's manifest stitch stays
/// a single passthrough per kind.
pub fn mesh_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.mesh".into(),
        name: "3D Mesh".into(),
        source_format: "mesh.reference".into(),
        component_kind: "mesh".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        schema: "mesh.reference".into(),
        export_formats: vec![semio_framework_plugin::MediaFormat::Glb, semio_framework_plugin::MediaFormat::Obj, semio_framework_plugin::MediaFormat::Stl],
        import_formats: vec![semio_framework_plugin::MediaFormat::Glb, semio_framework_plugin::MediaFormat::Obj],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_patch_apply_mutates_and_inverse_restores_all_fields() {
        let mesh_json = "{}".to_string();
        let mut object = LowpolyObject { id: "obj-1".into(), name: "Original".into(), transform: LowpolyTransform::default(), smooth_shading: false, mesh_json, paint_layers: vec![LowpolyPaintLayer::new("Base")] };
        let original = object.clone();
        let new_mesh = "{\"changed\":true}".to_string();
        let patch = LowpolyObjectPatch { name: Some("Renamed".into()), smooth_shading: Some(true), transform: Some(LowpolyTransform { position: [1.0, 2.0, 3.0], ..LowpolyTransform::default() }), mesh_json: Some(new_mesh.clone()) };
        object.apply_patch(&patch);
        assert_eq!(object.name, "Renamed");
        assert!(object.smooth_shading);
        assert_eq!(object.transform.position, [1.0, 2.0, 3.0]);
        assert_eq!(object.mesh_json, new_mesh);
        let inverse = object.diff_patch(&original).expect("patch changed state");
        object.apply_patch(&inverse);
        assert_eq!(object, original);
    }

    #[test]
    fn snapshot_from_mesh_json_builds_single_object_with_base_layer() {
        let mesh_json = "{}".to_string();
        let snapshot = snapshot_from_mesh_json(&mesh_json, "obj-42", "Widget");
        assert_eq!(snapshot.schema, LOWPOLY_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.objects.len(), 1);
        assert_eq!(snapshot.objects[0].id, "obj-42");
        assert_eq!(snapshot.objects[0].name, "Widget");
        assert_eq!(snapshot.objects[0].mesh_json, mesh_json);
        assert_eq!(snapshot.objects[0].paint_layers.len(), 1);
        assert_eq!(snapshot.objects[0].paint_layers[0].name, "Base");
    }

    #[test]
    fn lowpoly_selection_defaults_target_whole_mesh() {
        let targets = LowpolySelectionTargets::default();
        assert!(targets.mesh);
        assert!(!targets.vertex && !targets.edge && !targets.face);
        let selection = LowpolySelection::default();
        assert_eq!(selection.mode, "mesh");
        assert!(selection.ids.is_empty());
    }
}

    #[test]
    fn artifact_schema_descriptor_leaves_parse_and_field_states_match_snapshot_json() {
        use schema::{parse_state_class_kebab, ArtifactSchemaFields};
        let descriptor = crate::artifacts::lowpoly::schema::lowpoly_artifact_schema_descriptor();
        assert_eq!(descriptor.id, "s.lowpoly.lowpoly");
        let schema: serde_json::Value = serde_json::from_str(descriptor.snapshot.json_schema).expect("snapshot json");
        assert_eq!(schema["title"], "LowpolySnapshot");
        let properties = schema["properties"].as_object().expect("properties");
        let mut json_states: Vec<(String, _)> = properties.iter().map(|(name, prop)| {
            let raw = prop["x-semio-state"].as_str().expect("state");
            (name.clone(), parse_state_class_kebab(raw).expect("parse"))
        }).collect();
        json_states.sort_by(|a, b| a.0.cmp(&b.0));
        let mut derived: Vec<(String, _)> = crate::artifacts::lowpoly::snapshot::schema::LowpolySnapshot::field_states()
            .iter().map(|(n, c)| ((*n).to_string(), *c)).collect();
        derived.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(derived, json_states);
        assert_eq!(crate::artifacts::lowpoly::schema::LowpolyArtifact::artifact_schema_id(), "s.lowpoly.lowpoly");
    }
//#endregion 🧪️Tests
