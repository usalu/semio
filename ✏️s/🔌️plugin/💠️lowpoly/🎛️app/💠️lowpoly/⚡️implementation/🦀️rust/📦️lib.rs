//! 🔷️ Lowpoly app — document entities (constitutional: general): the mesh + paint document projection
//! and its patch types.

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

//#region 🔖️Projection
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
    // oversight: `LowpolyObject` is the element type of `LowpolyProjection.objects: Vec<LowpolyObject>`,
    // a `Shape::List(Shape::Record(..))` field printed in `JoinMode::Document`. The engine's `Writer`
    // only forces a line break after a `Shape::Embed` field's closing fence when the NEXT chunk is
    // pushed via `new_record()` (as `Shape::Block`/`Shape::Table` fields do) — plain list-item
    // iteration and the list's own closing `]` atom do not, so annotating this field glues the fence's
    // closing "```" to the following `]` on the same text line and breaks the fence lexer's "closing
    // ``` must be alone on its line" rule. Confirmed empirically (fails even with a single object):
    // reparsing the printed output errors with "unterminated fenced block". This is a genuine ENGINE
    // GAP distinct from the already-documented multi-`Shape::Embed`-field Lines-layout bug (see
    // `trinity::rewrite::RewriteRuleState`) — out of scope here, do not annotate until the engine's
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

/// @emoji 🎞️ Persisted lowpoly document: a list of mesh objects each carrying geometry (`mesh_json`,
/// deliberately plain `String` — see the field's own doc comment for the confirmed engine-gap reason
/// it cannot be `#[dsl(lang = "json")]`), transform, shading and paint layers. Ephemeral editing
/// context (active object, selection, utilities, camera, brush) lives in the plugin's app struct,
/// never here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "lowpoly", layout = "lines")]
pub struct LowpolyProjection {
    pub schema: String,
    pub objects: Vec<LowpolyObject>,
}

pub fn projection_from_mesh_json(mesh_json: &str, object_id: &str, object_name: &str) -> LowpolyProjection {
    LowpolyProjection {
        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        objects: vec![LowpolyObject { id: object_id.into(), name: object_name.into(), transform: LowpolyTransform::default(), smooth_shading: false, mesh_json: mesh_json.into(), paint_layers: vec![LowpolyPaintLayer::new("Base")] }],
    }
}

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
//#endregion 🔖️Projection

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
    // tests in the `op` crate.
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

//#endregion 🔖️Patches

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_patch_apply_mutates_and_inverse_restores_all_fields() {
        let mesh_json = "{}".to_string();
        let mut object = LowpolyObject { id: "obj-1".into(), name: "Original".into(), transform: LowpolyTransform::default(), smooth_shading: false, mesh_json: mesh_json.clone(), paint_layers: vec![LowpolyPaintLayer::new("Base")] };
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
    fn projection_from_mesh_json_builds_single_object_with_base_layer() {
        let mesh_json = "{}".to_string();
        let projection = projection_from_mesh_json(&mesh_json, "obj-42", "Widget");
        assert_eq!(projection.schema, LOWPOLY_DOCUMENT_SCHEMA);
        assert_eq!(projection.objects.len(), 1);
        assert_eq!(projection.objects[0].id, "obj-42");
        assert_eq!(projection.objects[0].name, "Widget");
        assert_eq!(projection.objects[0].mesh_json, mesh_json);
        assert_eq!(projection.objects[0].paint_layers.len(), 1);
        assert_eq!(projection.objects[0].paint_layers[0].name, "Base");
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
//#endregion 🧪️Tests
