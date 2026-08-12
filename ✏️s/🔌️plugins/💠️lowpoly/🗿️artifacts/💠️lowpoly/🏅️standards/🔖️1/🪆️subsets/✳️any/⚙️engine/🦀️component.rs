//! ⚙️ Lowpoly artifact — headless compute (constitutional: engine): the mutable mesh-editing compute
//! session (`LowpolyDocument`) wrapping `kernel_3d_mesh`, the default example projection, the app's
//! typed media I/O surface and shared projection lookups. Pixel/paint compute lives in the sibling
//! `🎨️paint/🦀️component.rs`; media export/import compute lives in `🧵️media/🦀️component.rs`.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyPaintLayer, LowpolySnapshot, LowpolySelection, LOWPOLY_PAINT_TEXTURE_SIZE};
use semio_framework_3d::mesh::{EdgeId, FaceId, HalfedgeMesh, MeshKernelError, Vec3, VertexId};
use serde_json::Value;

//#region ⚠️ Errors
/// ⚠️ `LowpolyDocument` compute-session and mesh-mutation / compute-session failure.
#[derive(Debug, thiserror::Error)]
pub enum LowpolyCoreError {
    #[error(transparent)]
    Mesh(#[from] MeshKernelError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("unknown primitive: {0}")]
    UnknownPrimitive(String),
    #[error("layer index out of range")]
    LayerIndexOutOfRange,
    #[error("no active object")]
    NoActiveObject,
    #[error("mesh missing")]
    MeshMissing,
    #[error("object not found")]
    ObjectNotFound,
}
//#endregion ⚠️ Errors

pub use crate::artifacts::lowpoly::engine::paint::{composite_layer_pixels, flood_fill, pixel_runs_from_diff, sample_pixel_from, stamp_brush};
pub use crate::artifacts::lowpoly::engine::media::{
    lowpoly_document_from_mesh,
    lowpoly_mesh_from_document,
    mesh_data_from_transfer,
    mesh_document_from_mesh,
    mesh_from_mesh_document,
};

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifacts::lowpoly::artifact_kind()` declares for `"3d.lowpoly"`, plus the two workflow
/// ports: `mesh:in` (Many, unrequired — accepts upstream mesh producers, e.g. cad via a Brep→Mesh
/// conversion) and `mesh:out` (Many, unrequired, `kind_id: "3d.mesh"` — the shared interchange kind
/// `crate::artifacts::lowpoly::mesh_artifact_kind()` declares).
pub fn lowpoly_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "mesh:in".into(),
                label: "Mesh".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
                kind_id: None,
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "mesh:out".into(),
                label: "Mesh".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
                kind_id: Some("3d.mesh".into()),
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
        ],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.lowpoly".into(), name: "3D Lowpoly".into(), dimension: "3d".into(), component_kind: "lowpoly".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Register
/// 🔎 Returns whether `s.lowpoly.lowpoly` is present in the process-local schema registry.
pub fn artifact_schema_registered() -> bool {
    ::schema::artifact_schema_descriptor_registered("s.lowpoly.lowpoly")
}
//#endregion 🔖️Register

//#region 🔖️DefaultProjection
/// 🎞️ Default document projection used by tests and the play app. Built programmatically from a
/// unit box until the handcrafted mesh DSL codec replaces derive-based `parse_dsl` (the reuse
/// `.lowpoly` example is already structured half-edge text without `mesh-json`).
pub fn default_snapshot() -> LowpolySnapshot {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim");
    let _ = mesh.unwrap_uv();
    let mesh_json = mesh.to_json().expect("mesh json");
    crate::artifacts::lowpoly::snapshot_from_mesh_json(&mesh_json, "obj-1", "Unit Box")
}
//#endregion 🔖️DefaultProjection

//#region 🔖️ProjectionHelpers
/// 🔧️ Shared by the compute session and the `edit-paint-layer`/`insert-paint-layer` mutation leaves —
/// a mutable lookup of an object by id within a projection.
pub fn object_mut<'a>(projection: &'a mut LowpolySnapshot, object_id: &str) -> Option<&'a mut LowpolyObject> {
    projection.objects.iter_mut().find(|object| object.id == object_id)
}

/// 🔧️ Shared by the compute session and `edit-paint-layer`'s `↩️inverse` leaf (which reads the
/// currently-stored bytes at each run's offset to compute the undo runs).
pub fn layer_pixels_at<'a>(projection: &'a LowpolySnapshot, object_id: &str, layer_index: usize) -> Option<&'a [u8]> {
    projection.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(layer_index)).map(|layer| layer.pixels.as_slice())
}
//#endregion 🔖️ProjectionHelpers

//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent lowpoly artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct LowpolyEngine {
    artifact: crate::artifacts::lowpoly::schema::LowpolyArtifact,
    snapshot: LowpolySnapshot,
}

impl LowpolyEngine {
    pub fn new(snapshot: LowpolySnapshot) -> Self {
        let artifact = crate::artifacts::lowpoly::schema::LowpolyArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> LowpolySnapshot {
        self.snapshot
    }

    fn sync_snapshot_from_artifact(&mut self) {
        self.snapshot = self.artifact.to_snapshot();
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️ComputeSession
/// @emoji 🛠️ Mutable compute session built from a projection clone plus ephemeral editing context
/// (active object + selection). The program runs a mesh/paint edit against it, then reads the mutated
/// `mesh_json`/pixels back out to construct the typed operation it emits. Never the source of truth.
pub struct LowpolyDocument {
    snapshot: LowpolySnapshot,
    active_object_id: String,
    selection: LowpolySelection,
    meshes: Vec<HalfedgeMesh>,
    next_object_serial: u32,
}

fn prepare_paint_mesh(mesh: &mut HalfedgeMesh) {
    let _ = mesh.unwrap_uv();
}

impl LowpolyDocument {
    pub fn new(snapshot: LowpolySnapshot) -> Result<Self, LowpolyCoreError> {
        let active_object_id = snapshot.objects.first().map(|object| object.id.clone()).unwrap_or_default();
        Self::with_context(snapshot, active_object_id, LowpolySelection::default())
    }

    pub fn with_context(snapshot: LowpolySnapshot, active_object_id: String, selection: LowpolySelection) -> Result<Self, LowpolyCoreError> {
        let next_object_serial = snapshot
            .objects
            .iter()
            .filter_map(|object| object.id.strip_prefix("obj-")?.parse::<u32>().ok())
            .max()
            .unwrap_or(100);
        let mut doc = Self { snapshot, active_object_id, selection, meshes: Vec::new(), next_object_serial };
        doc.reload_meshes()?;
        doc.ensure_all_paint_buffers();
        Ok(doc)
    }

    pub fn snapshot(&self) -> &LowpolySnapshot {
        &self.snapshot
    }

    pub fn snapshot_mut(&mut self) -> &mut LowpolySnapshot {
        &mut self.snapshot
    }

    pub fn active_object_id(&self) -> &str {
        &self.active_object_id
    }

    pub fn selection(&self) -> &LowpolySelection {
        &self.selection
    }

    pub fn ensure_all_paint_buffers(&mut self) {
        for object in &mut self.snapshot.objects {
            if object.paint_layers.is_empty() {
                object.paint_layers.push(LowpolyPaintLayer::new("Base"));
            }
            for layer in &mut object.paint_layers {
                if layer.pixels.len() != LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4 {
                    layer.pixels = crate::artifacts::lowpoly::empty_paint_pixels();
                }
            }
        }
    }

    pub fn layer_pixels(&self, object_id: &str, layer_index: usize) -> Result<&[u8], LowpolyCoreError> {
        layer_pixels_at(&self.snapshot, object_id, layer_index).ok_or(LowpolyCoreError::LayerIndexOutOfRange)
    }

    pub fn layer_pixels_mut(&mut self, object_id: &str, layer_index: usize) -> Result<&mut Vec<u8>, LowpolyCoreError> {
        self.ensure_paint_layer(object_id, layer_index)?;
        object_mut(&mut self.snapshot, object_id).and_then(|object| object.paint_layers.get_mut(layer_index)).map(|layer| &mut layer.pixels).ok_or(LowpolyCoreError::LayerIndexOutOfRange)
    }

    pub fn reload_meshes(&mut self) -> Result<(), LowpolyCoreError> {
        self.meshes.clear();
        for object in &self.snapshot.objects {
            let mesh = HalfedgeMesh::from_json(&object.mesh_json)?;
            self.meshes.push(mesh);
        }
        Ok(())
    }

    pub fn sync_meshes_to_snapshot(&mut self) -> Result<(), LowpolyCoreError> {
        for (object, mesh) in self.snapshot.objects.iter_mut().zip(self.meshes.iter()) {
            object.mesh_json = mesh.to_json()?;
        }
        Ok(())
    }

    pub fn active_index(&self) -> Option<usize> {
        self.snapshot.objects.iter().position(|o| o.id == self.active_object_id)
    }

    pub fn active_mesh_mut(&mut self) -> Result<&mut HalfedgeMesh, LowpolyCoreError> {
        let idx = self.active_index().ok_or(LowpolyCoreError::NoActiveObject)?;
        self.meshes.get_mut(idx).ok_or(LowpolyCoreError::MeshMissing)
    }

    pub fn active_mesh(&self) -> Result<&HalfedgeMesh, LowpolyCoreError> {
        let idx = self.active_index().ok_or(LowpolyCoreError::NoActiveObject)?;
        self.meshes.get(idx).ok_or(LowpolyCoreError::MeshMissing)
    }

    pub fn mesh_at(&self, index: usize) -> Option<&HalfedgeMesh> {
        self.meshes.get(index)
    }

    pub fn object_index(&self, object_id: &str) -> Result<usize, LowpolyCoreError> {
        self.snapshot.objects.iter().position(|o| o.id == object_id).ok_or(LowpolyCoreError::ObjectNotFound)
    }

    pub fn selected_face_ids(&self) -> Vec<FaceId> {
        if self.selection.mode != "face" {
            return Vec::new();
        }
        self.selection.ids.iter().map(|&id| FaceId(id)).collect()
    }

    pub fn selected_vertex_ids(&self) -> Vec<VertexId> {
        if self.selection.mode != "vertex" {
            return Vec::new();
        }
        self.selection.ids.iter().map(|&id| VertexId(id)).collect()
    }

    pub fn selected_edge_ids(&self) -> Vec<EdgeId> {
        if self.selection.mode != "edge" {
            return Vec::new();
        }
        self.selection.ids.iter().map(|&id| EdgeId(id)).collect()
    }

    pub fn normalize_selection_mode(mode: &str) -> String {
        if mode == "object" {
            "mesh".into()
        } else {
            mode.into()
        }
    }

    pub fn apply_selection(&mut self, mode: &str, ids: Vec<u32>) {
        self.selection.mode = Self::normalize_selection_mode(mode);
        self.selection.ids = ids;
    }

    pub fn selection_vertex_ids(&self) -> Result<Vec<VertexId>, LowpolyCoreError> {
        let mesh = self.active_mesh()?;
        match self.selection.mode.as_str() {
            "vertex" => Ok(self.selected_vertex_ids()),
            "face" => {
                let mut verts = Vec::new();
                let mut seen = std::collections::HashSet::new();
                for fid in self.selected_face_ids() {
                    for vid in mesh.face_vertex_ids(fid)? {
                        if seen.insert(vid.0) {
                            verts.push(vid);
                        }
                    }
                }
                Ok(verts)
            }
            "edge" => {
                let mut verts = Vec::new();
                let mut seen = std::collections::HashSet::new();
                for eid in self.selected_edge_ids() {
                    let (v0, v1) = mesh.edge_endpoints(eid)?;
                    for vid in [v0, v1] {
                        if seen.insert(vid.0) {
                            verts.push(vid);
                        }
                    }
                }
                Ok(verts)
            }
            _ => Ok(Vec::new()),
        }
    }

    pub fn selection_transform_pivot(&self) -> Result<Vec3, LowpolyCoreError> {
        let mesh = self.active_mesh()?;
        if self.selection.mode == "mesh" {
            let count = mesh.vertex_count();
            if count == 0 {
                return Ok(Vec3::new(0.0, 0.0, 0.0));
            }
            let mut sum = Vec3::new(0.0, 0.0, 0.0);
            for index in 0..count {
                sum = sum.add(mesh.vertex_position(VertexId(index as u32))?);
            }
            return Ok(sum.scale(1.0 / count as f32));
        }
        let verts = self.selection_vertex_ids()?;
        if verts.is_empty() {
            return Ok(Vec3::new(0.0, 0.0, 0.0));
        }
        let mut sum = Vec3::new(0.0, 0.0, 0.0);
        for vid in &verts {
            sum = sum.add(mesh.vertex_position(*vid)?);
        }
        Ok(sum.scale(1.0 / verts.len() as f32))
    }

    /// @emoji ➕️ Appends a primitive object, making it active, and returns its new id.
    pub fn add_primitive(&mut self, kind: &str) -> Result<String, LowpolyCoreError> {
        let mut mesh = match kind {
            "box" => HalfedgeMesh::box_prim(1.0, 1.0, 1.0),
            "plane" => HalfedgeMesh::plane_prim(2.0, 2.0),
            "cylinder" => HalfedgeMesh::cylinder_prim(0.5, 1.0, 12),
            "cone" => HalfedgeMesh::cone_prim(0.5, 1.0, 12),
            "ico_sphere" => HalfedgeMesh::ico_sphere_prim(0.5, 1),
            _ => return Err(LowpolyCoreError::UnknownPrimitive(kind.to_string())),
        }?;
        prepare_paint_mesh(&mut mesh);
        self.next_object_serial += 1;
        let id = format!("obj-{}", self.next_object_serial);
        let mesh_json = mesh.to_json()?;
        self.snapshot.objects.push(LowpolyObject { id: id.clone(), name: kind.into(), transform: Default::default(), smooth_shading: false, mesh_json, paint_layers: vec![LowpolyPaintLayer::new("Base")] });
        self.meshes.push(mesh);
        self.active_object_id = id.clone();
        Ok(id)
    }

    pub fn ensure_paint_layer(&mut self, object_id: &str, layer_index: usize) -> Result<(), LowpolyCoreError> {
        let idx = self.object_index(object_id)?;
        if self.snapshot.objects[idx].paint_layers.is_empty() {
            self.snapshot.objects[idx].paint_layers.push(LowpolyPaintLayer::new("Base"));
        }
        if layer_index >= self.snapshot.objects[idx].paint_layers.len() {
            return Err(LowpolyCoreError::LayerIndexOutOfRange);
        }
        Ok(())
    }

    pub fn tessellate_transfer_json(mesh: &HalfedgeMesh) -> Result<Value, LowpolyCoreError> {
        let transfer = mesh.tessellate()?;
        Ok(serde_json::json!({
            "positions": transfer.positions,
            "normals": transfer.normals,
            "indices": transfer.indices,
            "edgePositions": transfer.edge_positions,
            "faceIds": transfer.face_ids,
            "vertexIds": transfer.vertex_ids,
            "edgeIds": transfer.edge_ids,
            "edgeUvs": transfer.edge_uvs,
            "edgeIsSeam": transfer.edge_is_seam,
            "uvs": transfer.uvs,
        }))
    }

    pub fn tessellate_all_json(&self) -> Result<String, LowpolyCoreError> {
        let active = self.active_object_id.clone();
        let mut items = Vec::new();
        for (idx, object) in self.snapshot.objects.iter().enumerate() {
            let mesh = self.meshes.get(idx).ok_or(LowpolyCoreError::MeshMissing)?;
            items.push(serde_json::json!({
                "id": object.id,
                "index": idx,
                "name": object.name,
                "transform": object.transform,
                "smoothShading": object.smooth_shading,
                "active": object.id == active,
                "tessellation": Self::tessellate_transfer_json(mesh)?,
            }));
        }
        Ok(serde_json::to_string(&items)?)
    }

    pub fn composite_layers(&self, object_id: &str) -> Result<Vec<u8>, LowpolyCoreError> {
        let idx = self.object_index(object_id)?;
        Ok(super::paint::composite_layer_pixels(&self.snapshot.objects[idx].paint_layers))
    }

    /// @emoji 🖌️ Stamps a soft brush (or eraser) into a layer's pixel buffer in place.
    #[allow(clippy::too_many_arguments, reason = "1:1 forwarder for stamp_brush's own justified 8 args plus object_id/layer_index; a params struct would only move the same fields around for this single call site")]
    pub fn paint_stroke(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, radius: f32, color: [u8; 4], hardness: f32, opacity: f32, eraser: bool) -> Result<(), LowpolyCoreError> {
        let layer_pixels = self.layer_pixels_mut(object_id, layer_index)?;
        super::paint::stamp_brush(layer_pixels, u, v, radius, color, hardness, opacity, eraser);
        Ok(())
    }

    pub fn fill_bucket(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, color: [u8; 4]) -> Result<(), LowpolyCoreError> {
        let layer_pixels = self.layer_pixels_mut(object_id, layer_index)?;
        super::paint::flood_fill(layer_pixels, u, v, color);
        Ok(())
    }

    pub fn sample_pixel(&self, object_id: &str, u: f32, v: f32) -> Result<[u8; 4], LowpolyCoreError> {
        let composite = self.composite_layers(object_id)?;
        Ok(super::paint::sample_pixel_from(&composite, u, v))
    }
}
//#endregion 🔖️ComputeSession

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_snapshot_has_unit_box_object() {
        let projection = default_snapshot();
        assert_eq!(projection.schema, crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
        assert_eq!(projection.objects.len(), 1);
        assert_eq!(projection.objects[0].id, "obj-1");
        assert_eq!(projection.objects[0].name, "Unit Box");
        assert_eq!(projection.objects[0].paint_layers.len(), 1);
    }

    #[test]
    fn default_unit_box_mesh_parses_and_has_faces() {
        let projection = default_snapshot();
        let mesh = HalfedgeMesh::from_json(&projection.objects[0].mesh_json).expect("default mesh");
        assert!(mesh.face_count() >= 6, "unit box should expose six faces");
        assert!(mesh.vertex_count() >= 8, "unit box should expose eight vertices");
    }

    #[test]
    fn document_loads_meshes() {
        let doc = LowpolyDocument::new(default_snapshot()).unwrap();
        assert_eq!(doc.meshes.len(), 1);
        assert!(doc.meshes[0].face_count() > 0);
    }

    #[test]
    fn active_mesh_tessellates() {
        let doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
        assert!(!transfer.positions.is_empty());
        assert!(!transfer.indices.is_empty());
    }

    #[test]
    fn add_primitive_box() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let id = doc.add_primitive("box").unwrap();
        assert!(doc.snapshot.objects.iter().any(|o| o.id == id));
        assert_eq!(doc.meshes.len(), 2);
    }

    #[test]
    fn tessellate_all_returns_every_object() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let _ = doc.add_primitive("box").unwrap();
        let json = doc.tessellate_all_json().unwrap();
        let items: Vec<Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn projection_json_embeds_paint_pixels_as_base64() {
        let doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let json = serde_json::to_string(&doc.snapshot).unwrap();
        assert!(json.contains("\"pixels\""));
        // base64 white, never a raw integer array.
        assert!(!json.contains("255,255,255"));
    }

    #[test]
    fn default_snapshot_mesh_has_unwrapped_uvs() {
        let doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
        assert!(transfer.uvs.iter().any(|uv| *uv > 0.0));
    }

    #[test]
    fn paint_stroke_writes_pixels() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let object_id = doc.active_object_id.clone();
        doc.paint_stroke(&object_id, 0, 0.5, 0.5, 4.0, [255, 0, 0, 255], 0.5, 1.0, false).unwrap();
        let composite = doc.composite_layers(&object_id).unwrap();
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        let center = (size / 2 * size + size / 2) * 4;
        assert!(composite[center] > 200);
    }

    #[test]
    fn projection_round_trips_paint_pixels_through_base64_json() {
        let mut projection = default_snapshot();
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        let json = serde_json::to_string(&projection).unwrap();
        let restored: LowpolySnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, projection);
    }

    //#region 🔖️ComputeSessionCoverage
    #[test]
    fn add_primitive_supports_every_known_kind() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        for kind in ["plane", "cylinder", "cone", "ico_sphere"] {
            let id = doc.add_primitive(kind).unwrap();
            assert_eq!(doc.active_object_id(), id);
            assert!(doc.snapshot().objects.iter().any(|o| o.id == id));
        }
        assert_eq!(doc.snapshot().objects.len(), 5);
    }

    #[test]
    fn add_primitive_unknown_kind_errors() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let result = doc.add_primitive("teapot");
        assert!(matches!(result, Err(LowpolyCoreError::UnknownPrimitive(kind)) if kind == "teapot"));
    }

    #[test]
    fn object_index_errors_for_unknown_id() {
        let doc = LowpolyDocument::new(default_snapshot()).unwrap();
        assert!(matches!(doc.object_index("missing"), Err(LowpolyCoreError::ObjectNotFound)));
    }

    #[test]
    fn ensure_paint_layer_errors_for_unknown_object_and_out_of_range_index() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let object_id = doc.active_object_id().to_string();
        assert!(matches!(doc.ensure_paint_layer("missing", 0), Err(LowpolyCoreError::ObjectNotFound)));
        assert!(matches!(doc.ensure_paint_layer(&object_id, 99), Err(LowpolyCoreError::LayerIndexOutOfRange)));
    }

    #[test]
    fn layer_pixels_errors_for_out_of_range_index() {
        let doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let object_id = doc.active_object_id().to_string();
        assert!(matches!(doc.layer_pixels(&object_id, 5), Err(LowpolyCoreError::LayerIndexOutOfRange)));
    }

    #[test]
    fn active_mesh_errors_when_active_object_id_is_unknown() {
        let doc = LowpolyDocument::with_context(default_snapshot(), "does-not-exist".into(), LowpolySelection::default()).unwrap();
        assert!(matches!(doc.active_mesh(), Err(LowpolyCoreError::NoActiveObject)));
    }

    #[test]
    fn mesh_at_returns_none_past_object_count() {
        let doc = LowpolyDocument::new(default_snapshot()).unwrap();
        assert!(doc.mesh_at(0).is_some());
        assert!(doc.mesh_at(99).is_none());
    }

    #[test]
    fn sync_meshes_to_snapshot_writes_back_mesh_json() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.active_mesh_mut().unwrap().translate(Vec3::new(1.0, 0.0, 0.0)).unwrap();
        let idx = doc.active_index().unwrap();
        let before = doc.snapshot().objects[idx].mesh_json.clone();
        doc.sync_meshes_to_snapshot().unwrap();
        assert_ne!(doc.snapshot().objects[idx].mesh_json, before);
    }

    #[test]
    fn normalize_selection_mode_maps_object_to_mesh_and_passes_through_others() {
        assert_eq!(LowpolyDocument::normalize_selection_mode("object"), "mesh");
        assert_eq!(LowpolyDocument::normalize_selection_mode("face"), "face");
        assert_eq!(LowpolyDocument::normalize_selection_mode("vertex"), "vertex");
    }

    #[test]
    fn apply_selection_normalizes_mode_and_stores_ids() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        doc.apply_selection("object", vec![3, 4]);
        assert_eq!(doc.selection().mode, "mesh");
        assert_eq!(doc.selection().ids, vec![3, 4]);
    }

    #[test]
    fn selected_ids_are_empty_when_selection_mode_mismatches() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        doc.apply_selection("face", vec![1, 2]);
        assert!(doc.selected_vertex_ids().is_empty());
        assert!(doc.selected_edge_ids().is_empty());
        assert_eq!(doc.selected_face_ids().len(), 2);
    }

    #[test]
    fn selection_vertex_ids_face_mode_dedupes_shared_vertices() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.apply_selection("face", vec![0, 1]);
        let verts = doc.selection_vertex_ids().unwrap();
        let mesh = doc.active_mesh().unwrap();
        let mut expected: Vec<u32> = Vec::new();
        for face in [FaceId(0), FaceId(1)] {
            for vid in mesh.face_vertex_ids(face).unwrap() {
                if !expected.contains(&vid.0) {
                    expected.push(vid.0);
                }
            }
        }
        assert_eq!(verts.into_iter().map(|v| v.0).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn selection_vertex_ids_edge_mode_returns_endpoints() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.apply_selection("edge", vec![0]);
        let verts = doc.selection_vertex_ids().unwrap();
        let mesh = doc.active_mesh().unwrap();
        let (v0, v1) = mesh.edge_endpoints(EdgeId(0)).unwrap();
        assert_eq!(verts, vec![v0, v1]);
    }

    #[test]
    fn selection_vertex_ids_mesh_mode_is_empty() {
        let doc = LowpolyDocument::new(default_snapshot()).unwrap();
        assert!(doc.selection_vertex_ids().unwrap().is_empty());
    }

    #[test]
    fn selection_transform_pivot_mesh_mode_averages_all_vertices() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        doc.add_primitive("box").unwrap();
        let mesh = doc.active_mesh().unwrap();
        let count = mesh.vertex_count();
        let mut sum = Vec3::new(0.0, 0.0, 0.0);
        for index in 0..count {
            sum = sum.add(mesh.vertex_position(VertexId(index as u32)).unwrap());
        }
        let expected = sum.scale(1.0 / count as f32);
        let pivot = doc.selection_transform_pivot().unwrap();
        assert!((pivot.x() - expected.x()).abs() < 1e-5);
        assert!((pivot.y() - expected.y()).abs() < 1e-5);
        assert!((pivot.z() - expected.z()).abs() < 1e-5);
    }

    #[test]
    fn selection_transform_pivot_vertex_mode_averages_selected_vertices() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.apply_selection("vertex", vec![0, 1]);
        let mesh = doc.active_mesh().unwrap();
        let p0 = mesh.vertex_position(VertexId(0)).unwrap();
        let p1 = mesh.vertex_position(VertexId(1)).unwrap();
        let expected = p0.add(p1).scale(0.5);
        let pivot = doc.selection_transform_pivot().unwrap();
        assert!((pivot.x() - expected.x()).abs() < 1e-5);
    }

    #[test]
    fn selection_transform_pivot_empty_vertex_selection_is_origin() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        doc.apply_selection("vertex", vec![]);
        let pivot = doc.selection_transform_pivot().unwrap();
        assert_eq!((pivot.x(), pivot.y(), pivot.z()), (0.0, 0.0, 0.0));
    }

    #[test]
    fn ensure_all_paint_buffers_adds_missing_layer_and_fixes_wrong_size() {
        let mut projection = default_snapshot();
        projection.objects[0].paint_layers.clear();
        let mut doc = LowpolyDocument::new(projection).unwrap();
        assert_eq!(doc.snapshot().objects[0].paint_layers.len(), 1);
        assert_eq!(doc.snapshot().objects[0].paint_layers[0].name, "Base");
        doc.snapshot_mut().objects[0].paint_layers[0].pixels = vec![1, 2, 3];
        doc.ensure_all_paint_buffers();
        assert_eq!(doc.snapshot().objects[0].paint_layers[0].pixels.len(), LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4);
    }

    #[test]
    fn fill_bucket_and_sample_pixel_reflect_new_color() {
        let mut doc = LowpolyDocument::new(default_snapshot()).unwrap();
        let object_id = doc.active_object_id().to_string();
        doc.fill_bucket(&object_id, 0, 0.5, 0.5, [10, 20, 30, 255]).unwrap();
        assert_eq!(doc.sample_pixel(&object_id, 0.5, 0.5).unwrap(), [10, 20, 30, 255]);
    }
    
    #[test]
    fn artifact_engine_apply_and_inverse_round_trip() {
        // 🩹 Was `protocol::ArtifactEngine`-based (that trait doesn't exist anywhere in the
        // codebase — a stale reference predating 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL — and
        // `LowpolyEngine` never exposed a `snapshot()`/apply-mutation API either) and constructed
        // the since-removed `LowpolyMutation::ObjectsPatch` bag variant. Rewritten against the real
        // `protocol::Mutation` diff/apply/inverse contract and the new `rename-object` mutation.
        use crate::artifacts::lowpoly::mutations::rename_object;
        use crate::artifacts::lowpoly::LowpolyMutation;
        use protocol::{Mutation, MutationDiff};
        let base = default_snapshot();
        let object_id = base.objects[0].id.clone();
        let mutation = LowpolyMutation::RenameObject(rename_object::mutation::RenameObject { id: object_id, new_name: "Renamed".into() });
        let after = mutation.diff(&base).apply(&base);
        assert_eq!(after.objects[0].name, "Renamed");
        let inverse = mutation.inverse(&base);
        let mut state = after;
        for step in &inverse {
            state = step.diff(&base).apply(&state);
        }
        assert_eq!(state.objects[0].name, "Unit Box");
    }

    //#endregion 🔖️ComputeSessionCoverage
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::LowpolyComposer as LowpolyAnyComposer;
    use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::LowpolyBuilder as LowpolyAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const LOWPOLY_DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };
    const LOWPOLY_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::lowpoly::LowpolySnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == LOWPOLY_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => LowpolyAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => LowpolyAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "LowpolyComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == LOWPOLY_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::lowpoly::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "LowpolyComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_las(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_LAS_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_ply(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PLY_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::lowpoly::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<LowpolyAnyComposer>(),
            ComposerEntry { writes: EXPORT_LAS_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_las },
            ComposerEntry { writes: EXPORT_PLY_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_ply },
            ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_png },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_dwg },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_gltf },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
