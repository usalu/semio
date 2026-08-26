//! ⚙️ Lowpoly play app engine — the mutable mesh-editing compute session (`LowpolyDocument`)
//! wrapping `kernel_3d_mesh`, plus the mesh media-export leaf that depends on it. A short-lived,
//! freshly-constructed compute session (never a persisted field on the app itself — see the
//! `TransformSession`/`mesh_edit` call sites in `🖌️session/🦀️component.rs`), so it lives here in the
//! app's own `⚙️engine` slot rather than the artifact (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): an artifact is schema + io, never an engine.
//! Relocated verbatim from the deleted `🗿️artifacts/💠️lowpoly/…/⚙️engine/🦀️component.rs`.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyPaintLayer, LowpolySelection, LowpolySnapshot, LOWPOLY_PAINT_TEXTURE_SIZE};
use semio_framework_3d::mesh::{EdgeId, FaceId, HalfedgeMesh, MeshKernelError, Vec3, VertexId};
use semio_framework_plugin::MeshData;
use serde_json::Value;
use std::collections::HashMap;

//#region ⚠️ Errors
/// ⚠️ `LowpolyDocument` compute-session and mesh-mutation / compute-session failure.
#[derive(Debug)]
pub enum LowpolyCoreError {
    Mesh(MeshKernelError),
    Json(serde_json::Error),
    UnknownPrimitive(String),
    LayerIndexOutOfRange,
    NoActiveObject,
    MeshMissing,
    ObjectNotFound,
    /// 🕸️ The session-local `mesh_workspace` cache (`🖌️session::LowpolyScratch`) has no entry for an
    /// object, or its cached JSON no longer matches the object's persisted `mesh` handle — e.g. after
    /// an undo/redo of a `create-mesh`/`delete-mesh` (store-level undo/redo bypass `ArtifactApp::handle`
    /// entirely, so this cache is never resynced by them). Fails closed rather than silently editing
    /// stale geometry and re-deriving a WRONG handle from it via `sync_meshes_to_snapshot`.
    StaleMeshWorkspace(String),
}

impl std::fmt::Display for LowpolyCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mesh(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
            Self::UnknownPrimitive(primitive) => write!(formatter, "unknown primitive: {primitive}"),
            Self::LayerIndexOutOfRange => formatter.write_str("layer index out of range"),
            Self::NoActiveObject => formatter.write_str("no active object"),
            Self::MeshMissing => formatter.write_str("mesh missing"),
            Self::ObjectNotFound => formatter.write_str("object not found"),
            Self::StaleMeshWorkspace(object) => write!(formatter, "mesh workspace missing or stale for object {object}"),
        }
    }
}

impl std::error::Error for LowpolyCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Mesh(error) => std::error::Error::source(error),
            Self::Json(error) => std::error::Error::source(error),
            _ => None,
        }
    }
}

impl From<MeshKernelError> for LowpolyCoreError {
    fn from(error: MeshKernelError) -> Self {
        Self::Mesh(error)
    }
}

impl From<serde_json::Error> for LowpolyCoreError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
//#endregion ⚠️ Errors

//#region 🔖️ComputeSession
/// @emoji 🛠️ Mutable compute session built from a projection clone plus ephemeral editing context
/// (active object + selection) PLUS the session-local `mesh_workspace` cache (round 2 of ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's round-trip law fix — `LowpolyObject` carries no live
/// mesh content field at all any more, only the `🖌️session::LowpolyScratch` cache does). The program
/// runs a mesh/paint edit against it, then reads the mutated `mesh_workspace`/pixels back out to
/// construct the typed operation it emits. Never the source of truth.
pub struct LowpolyDocument {
    snapshot: LowpolySnapshot,
    active_object_id: String,
    selection: LowpolySelection,
    meshes: Vec<HalfedgeMesh>,
    next_object_serial: u32,
    /// 🕸️ Live half-edge-mesh JSON per object id — seeded from the caller's session cache
    /// (`LowpolyScratch::mesh_workspace_map()`), updated in place by `sync_meshes_to_snapshot`/
    /// `add_primitive`, and read back out via `mesh_workspace()` so the caller can merge it back into
    /// its own session cache after a successful edit.
    mesh_workspace: HashMap<String, String>,
}

fn prepare_paint_mesh(mesh: &mut HalfedgeMesh) {
    let _ = mesh.unwrap_uv();
}

impl LowpolyDocument {
    pub fn new(snapshot: LowpolySnapshot, mesh_workspace: HashMap<String, String>) -> Result<Self, LowpolyCoreError> {
        let active_object_id = snapshot.objects.first().map(|object| object.id.clone()).unwrap_or_default();
        Self::with_context(snapshot, active_object_id, LowpolySelection::default(), mesh_workspace)
    }

    pub fn with_context(snapshot: LowpolySnapshot, active_object_id: String, selection: LowpolySelection, mesh_workspace: HashMap<String, String>) -> Result<Self, LowpolyCoreError> {
        let next_object_serial = snapshot.objects.iter().filter_map(|object| object.id.strip_prefix("obj-")?.parse::<u32>().ok()).max().unwrap_or(100);
        let mut doc = Self { snapshot, active_object_id, selection, meshes: Vec::new(), next_object_serial, mesh_workspace };
        doc.reload_meshes()?;
        doc.ensure_all_paint_buffers();
        Ok(doc)
    }

    /// 🕸️ The live half-edge-mesh JSON per object id, current as of the last `reload_meshes`/
    /// `sync_meshes_to_snapshot`/`add_primitive` — callers merge this back into their own session
    /// cache (`LowpolyScratch::set_mesh_workspace_map`) after a successful edit.
    pub fn mesh_workspace(&self) -> &HashMap<String, String> {
        &self.mesh_workspace
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
        crate::artifacts::lowpoly::schema::layer_pixels_at(&self.snapshot, object_id, layer_index).ok_or(LowpolyCoreError::LayerIndexOutOfRange)
    }

    pub fn layer_pixels_mut(&mut self, object_id: &str, layer_index: usize) -> Result<&mut Vec<u8>, LowpolyCoreError> {
        self.ensure_paint_layer(object_id, layer_index)?;
        crate::artifacts::lowpoly::schema::object_mut(&mut self.snapshot, object_id).and_then(|object| object.paint_layers.get_mut(layer_index)).map(|layer| &mut layer.pixels).ok_or(LowpolyCoreError::LayerIndexOutOfRange)
    }

    /// 🕸️ Reloads every object's `HalfedgeMesh` from the session-local `mesh_workspace` cache. An
    /// object with a `mesh` handle whose cached JSON is missing, or hashes to a DIFFERENT handle
    /// (`mesh_child_handle`, "the handle IS the change signal" — see that fn's own doc comment) is a
    /// stale cache (e.g. an undo/redo the session never observed, see `LowpolyCoreError::StaleMeshWorkspace`'s
    /// own doc comment) and fails closed rather than silently loading the wrong geometry.
    pub fn reload_meshes(&mut self) -> Result<(), LowpolyCoreError> {
        self.meshes.clear();
        for object in &self.snapshot.objects {
            let Some(json) = self.mesh_workspace.get(&object.id) else {
                return Err(LowpolyCoreError::StaleMeshWorkspace(object.id.clone()));
            };
            if let Some(handle) = &object.mesh {
                if crate::artifacts::lowpoly::mesh_child_handle(&object.id, json) != *handle {
                    return Err(LowpolyCoreError::StaleMeshWorkspace(object.id.clone()));
                }
            }
            let mesh = HalfedgeMesh::from_json(json)?;
            self.meshes.push(mesh);
        }
        Ok(())
    }

    /// 🕸️ Writes the live kernel geometry back into the session-local `mesh_workspace` cache, then
    /// (re-)derives the persisted `mesh` CHILD handle from that content via `mesh_child_handle` —
    /// identical geometry always resolves to the identical handle, so an unchanged mesh produces no
    /// spurious diff on the handle even though this runs on every sync.
    pub fn sync_meshes_to_snapshot(&mut self) -> Result<(), LowpolyCoreError> {
        for (object, mesh) in self.snapshot.objects.iter_mut().zip(self.meshes.iter()) {
            let json = mesh.to_json()?;
            object.mesh = Some(crate::artifacts::lowpoly::mesh_child_handle(&object.id, &json));
            self.mesh_workspace.insert(object.id.clone(), json);
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
        let mesh_workspace = mesh.to_json()?;
        let mesh_handle = crate::artifacts::lowpoly::mesh_child_handle(&id, &mesh_workspace);
        self.snapshot.objects.push(LowpolyObject { id: id.clone(), name: kind.into(), transform: Default::default(), smooth_shading: false, mesh: Some(mesh_handle), paint_layers: vec![LowpolyPaintLayer::new("Base")] });
        self.mesh_workspace.insert(id.clone(), mesh_workspace);
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
        Ok(crate::artifacts::lowpoly::schema::composite_layer_pixels(&self.snapshot.objects[idx].paint_layers))
    }

    /// @emoji 🖌️ Stamps a soft brush (or eraser) into a layer's pixel buffer in place.
    #[allow(clippy::too_many_arguments, reason = "1:1 forwarder for stamp_brush's own justified 8 args plus object_id/layer_index; a params struct would only move the same fields around for this single call site")]
    pub fn paint_stroke(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, radius: f32, color: [u8; 4], hardness: f32, opacity: f32, eraser: bool) -> Result<(), LowpolyCoreError> {
        let layer_pixels = self.layer_pixels_mut(object_id, layer_index)?;
        crate::artifacts::lowpoly::schema::stamp_brush(layer_pixels, u, v, radius, color, hardness, opacity, eraser);
        Ok(())
    }

    pub fn fill_bucket(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, color: [u8; 4]) -> Result<(), LowpolyCoreError> {
        let layer_pixels = self.layer_pixels_mut(object_id, layer_index)?;
        crate::artifacts::lowpoly::schema::flood_fill(layer_pixels, u, v, color);
        Ok(())
    }

    pub fn sample_pixel(&self, object_id: &str, u: f32, v: f32) -> Result<[u8; 4], LowpolyCoreError> {
        let composite = self.composite_layers(object_id)?;
        Ok(crate::artifacts::lowpoly::schema::sample_pixel_from(&composite, u, v))
    }
}
//#endregion 🔖️ComputeSession

//#region 🔖️MediaExport
/// 🔺️ Tessellates a lowpoly document's active object into a `MeshData` for media export. Depends on
/// `LowpolyDocument`, so it lives beside it here rather than the pure conversions in the artifact's own
/// schema (`crate::artifacts::lowpoly::schema::mesh_data_from_transfer` et al). Takes the caller's
/// session-local `mesh_workspace` cache explicitly (round 2 of this ticket's round-trip law fix) —
/// `doc: &Value` alone no longer carries live mesh content, only the persisted `mesh` handle.
pub fn lowpoly_mesh_from_document(doc: &Value, mesh_workspace: &HashMap<String, String>) -> Result<MeshData, String> {
    let snapshot: LowpolySnapshot = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    let loaded = LowpolyDocument::new(snapshot, mesh_workspace.clone()).map_err(|e| e.to_string())?;
    Ok(loaded.active_mesh().ok().and_then(|mesh| LowpolyDocument::tessellate_transfer_json(mesh).ok()).map(|transfer| crate::artifacts::lowpoly::schema::mesh_data_from_transfer(&transfer, None)).unwrap_or_default())
}
//#endregion 🔖️MediaExport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::schema::{default_mesh_workspace, default_snapshot};

    #[semio_framework_async_macros::async_test]
    async fn document_loads_meshes() {
        let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        assert_eq!(doc.meshes.len(), 1);
        assert!(doc.meshes[0].face_count() > 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn active_mesh_tessellates() {
        let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
        assert!(!transfer.positions.is_empty());
        assert!(!transfer.indices.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_primitive_box() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let id = doc.add_primitive("box").unwrap();
        assert!(doc.snapshot.objects.iter().any(|o| o.id == id));
        assert_eq!(doc.meshes.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn tessellate_all_returns_every_object() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let _ = doc.add_primitive("box").unwrap();
        let json = doc.tessellate_all_json().unwrap();
        let items: Vec<Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(items.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_json_embeds_paint_pixels_as_base64() {
        let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let json = serde_json::to_string(&doc.snapshot).unwrap();
        assert!(json.contains("\"pixels\""));
        // base64 white, never a raw integer array.
        assert!(!json.contains("255,255,255"));
    }

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_mesh_has_unwrapped_uvs() {
        let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
        assert!(transfer.uvs.iter().any(|uv| *uv > 0.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn paint_stroke_writes_pixels() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let object_id = doc.active_object_id.clone();
        doc.paint_stroke(&object_id, 0, 0.5, 0.5, 4.0, [255, 0, 0, 255], 0.5, 1.0, false).unwrap();
        let composite = doc.composite_layers(&object_id).unwrap();
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        let center = (size / 2 * size + size / 2) * 4;
        assert!(composite[center] > 200);
    }

    //#region 🔖️ComputeSessionCoverage
    #[semio_framework_async_macros::async_test]
    async fn add_primitive_supports_every_known_kind() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        for kind in ["plane", "cylinder", "cone", "ico_sphere"] {
            let id = doc.add_primitive(kind).unwrap();
            assert_eq!(doc.active_object_id(), id);
            assert!(doc.snapshot().objects.iter().any(|o| o.id == id));
        }
        assert_eq!(doc.snapshot().objects.len(), 5);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_primitive_unknown_kind_errors() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let result = doc.add_primitive("teapot");
        assert!(matches!(result, Err(LowpolyCoreError::UnknownPrimitive(kind)) if kind == "teapot"));
    }

    #[semio_framework_async_macros::async_test]
    async fn object_index_errors_for_unknown_id() {
        let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        assert!(matches!(doc.object_index("missing"), Err(LowpolyCoreError::ObjectNotFound)));
    }

    #[semio_framework_async_macros::async_test]
    async fn ensure_paint_layer_errors_for_unknown_object_and_out_of_range_index() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let object_id = doc.active_object_id().to_string();
        assert!(matches!(doc.ensure_paint_layer("missing", 0), Err(LowpolyCoreError::ObjectNotFound)));
        assert!(matches!(doc.ensure_paint_layer(&object_id, 99), Err(LowpolyCoreError::LayerIndexOutOfRange)));
    }

    #[semio_framework_async_macros::async_test]
    async fn layer_pixels_errors_for_out_of_range_index() {
        let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let object_id = doc.active_object_id().to_string();
        assert!(matches!(doc.layer_pixels(&object_id, 5), Err(LowpolyCoreError::LayerIndexOutOfRange)));
    }

    #[semio_framework_async_macros::async_test]
    async fn active_mesh_errors_when_active_object_id_is_unknown() {
        let doc = LowpolyDocument::with_context(default_snapshot(), "does-not-exist".into(), LowpolySelection::default(), default_mesh_workspace()).unwrap();
        assert!(matches!(doc.active_mesh(), Err(LowpolyCoreError::NoActiveObject)));
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_at_returns_none_past_object_count() {
        let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        assert!(doc.mesh_at(0).is_some());
        assert!(doc.mesh_at(99).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn sync_meshes_to_snapshot_writes_back_mesh_json() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.active_mesh_mut().unwrap().translate(Vec3::new(1.0, 0.0, 0.0)).unwrap();
        let idx = doc.active_index().unwrap();
        let object_id = doc.snapshot().objects[idx].id.clone();
        let before = doc.mesh_workspace().get(&object_id).cloned();
        doc.sync_meshes_to_snapshot().unwrap();
        assert_ne!(doc.mesh_workspace().get(&object_id).cloned(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn normalize_selection_mode_maps_object_to_mesh_and_passes_through_others() {
        assert_eq!(LowpolyDocument::normalize_selection_mode("object"), "mesh");
        assert_eq!(LowpolyDocument::normalize_selection_mode("face"), "face");
        assert_eq!(LowpolyDocument::normalize_selection_mode("vertex"), "vertex");
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_selection_normalizes_mode_and_stores_ids() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        doc.apply_selection("object", vec![3, 4]);
        assert_eq!(doc.selection().mode, "mesh");
        assert_eq!(doc.selection().ids, vec![3, 4]);
    }

    #[semio_framework_async_macros::async_test]
    async fn selected_ids_are_empty_when_selection_mode_mismatches() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        doc.apply_selection("face", vec![1, 2]);
        assert!(doc.selected_vertex_ids().is_empty());
        assert!(doc.selected_edge_ids().is_empty());
        assert_eq!(doc.selected_face_ids().len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn selection_vertex_ids_face_mode_dedupes_shared_vertices() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
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

    #[semio_framework_async_macros::async_test]
    async fn selection_vertex_ids_edge_mode_returns_endpoints() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.apply_selection("edge", vec![0]);
        let verts = doc.selection_vertex_ids().unwrap();
        let mesh = doc.active_mesh().unwrap();
        let (v0, v1) = mesh.edge_endpoints(EdgeId(0)).unwrap();
        assert_eq!(verts, vec![v0, v1]);
    }

    #[semio_framework_async_macros::async_test]
    async fn selection_vertex_ids_mesh_mode_is_empty() {
        let doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        assert!(doc.selection_vertex_ids().unwrap().is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn selection_transform_pivot_mesh_mode_averages_all_vertices() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
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

    #[semio_framework_async_macros::async_test]
    async fn selection_transform_pivot_vertex_mode_averages_selected_vertices() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.apply_selection("vertex", vec![0, 1]);
        let mesh = doc.active_mesh().unwrap();
        let p0 = mesh.vertex_position(VertexId(0)).unwrap();
        let p1 = mesh.vertex_position(VertexId(1)).unwrap();
        let expected = p0.add(p1).scale(0.5);
        let pivot = doc.selection_transform_pivot().unwrap();
        assert!((pivot.x() - expected.x()).abs() < 1e-5);
    }

    #[semio_framework_async_macros::async_test]
    async fn selection_transform_pivot_empty_vertex_selection_is_origin() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        doc.apply_selection("vertex", vec![]);
        let pivot = doc.selection_transform_pivot().unwrap();
        assert_eq!((pivot.x(), pivot.y(), pivot.z()), (0.0, 0.0, 0.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn ensure_all_paint_buffers_adds_missing_layer_and_fixes_wrong_size() {
        let mut projection = default_snapshot();
        projection.objects[0].paint_layers.clear();
        let mut doc = LowpolyDocument::new(projection, default_mesh_workspace()).unwrap();
        assert_eq!(doc.snapshot().objects[0].paint_layers.len(), 1);
        assert_eq!(doc.snapshot().objects[0].paint_layers[0].name, "Base");
        doc.snapshot_mut().objects[0].paint_layers[0].pixels = vec![1, 2, 3];
        doc.ensure_all_paint_buffers();
        assert_eq!(doc.snapshot().objects[0].paint_layers[0].pixels.len(), LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn fill_bucket_and_sample_pixel_reflect_new_color() {
        let mut doc = LowpolyDocument::new(default_snapshot(), default_mesh_workspace()).unwrap();
        let object_id = doc.active_object_id().to_string();
        doc.fill_bucket(&object_id, 0, 0.5, 0.5, [10, 20, 30, 255]).unwrap();
        assert_eq!(doc.sample_pixel(&object_id, 0.5, 0.5).unwrap(), [10, 20, 30, 255]);
    }
    //#endregion 🔖️ComputeSessionCoverage
}
//#endregion 🧪️Tests
