//! ⚙️ Lowpoly app — headless compute (constitutional: engine): the mutable mesh-editing compute session
//! (`LowpolyDocument`) wrapping `kernel_3d_mesh`, pixel/paint compute, the default example projection and
//! media export/import compute.

use kernel_3d_mesh::{EdgeId, FaceId, HalfedgeMesh, MeshKernelError, Vec3, VertexId};
use lowpoly::{empty_paint_pixels, LowpolyObject, LowpolyPaintLayer, LowpolyProjection, LowpolySelection, LOWPOLY_PAINT_TEXTURE_SIZE};
use semio_framework_plugin::MeshData;
use serde_json::Value;

//#region ⚠️ Errors
/// ⚠️ `LowpolyDocument` compute-session and mesh-operation failure.
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

//#region 🔖DefaultProjection
pub fn default_projection() -> LowpolyProjection {
    lowpoly_dsl::parse_dsl(lowpoly_dsl::LOWPOLY_EXAMPLE_TEXT).expect("default projection DSL parses")
}
//#endregion 🔖DefaultProjection

//#region 🔖ProjectionHelpers
/// 🔧 Shared by the compute session (`LowpolyDocument`) and `op`'s `apply_lowpoly_operation`/
/// `invert_lowpoly_operation` — a mutable lookup of an object by id within a projection.
pub fn object_mut<'a>(projection: &'a mut LowpolyProjection, object_id: &str) -> Option<&'a mut LowpolyObject> {
    projection.objects.iter_mut().find(|object| object.id == object_id)
}

/// 🔧 Shared by the compute session and `op`'s `invert_lowpoly_operation` (`PaintStroke` inverse reads
/// the currently-stored bytes at each run's offset).
pub fn layer_pixels_at<'a>(projection: &'a LowpolyProjection, object_id: &str, layer_index: usize) -> Option<&'a [u8]> {
    projection.objects.iter().find(|object| object.id == object_id).and_then(|object| object.paint_layers.get(layer_index)).map(|layer| layer.pixels.as_slice())
}
//#endregion 🔖ProjectionHelpers

//#region 🔖ComputeSession
/// @emoji 🛠️ Mutable compute session built from a projection clone plus ephemeral editing context
/// (active object + selection). The program runs a mesh/paint edit against it, then reads the mutated
/// `mesh_json`/pixels back out to construct the typed operation it emits. Never the source of truth.
pub struct LowpolyDocument {
    projection: LowpolyProjection,
    active_object_id: String,
    selection: LowpolySelection,
    meshes: Vec<HalfedgeMesh>,
    next_object_serial: u32,
}

fn prepare_paint_mesh(mesh: &mut HalfedgeMesh) {
    let _ = mesh.unwrap_uv();
}

impl LowpolyDocument {
    pub fn new(projection: LowpolyProjection) -> Result<Self, LowpolyCoreError> {
        let active_object_id = projection.objects.first().map(|object| object.id.clone()).unwrap_or_default();
        Self::with_context(projection, active_object_id, LowpolySelection::default())
    }

    pub fn with_context(projection: LowpolyProjection, active_object_id: String, selection: LowpolySelection) -> Result<Self, LowpolyCoreError> {
        let mut doc = Self { projection, active_object_id, selection, meshes: Vec::new(), next_object_serial: 100 };
        doc.reload_meshes()?;
        doc.ensure_all_paint_buffers();
        Ok(doc)
    }

    pub fn projection(&self) -> &LowpolyProjection {
        &self.projection
    }

    pub fn projection_mut(&mut self) -> &mut LowpolyProjection {
        &mut self.projection
    }

    pub fn active_object_id(&self) -> &str {
        &self.active_object_id
    }

    pub fn selection(&self) -> &LowpolySelection {
        &self.selection
    }

    pub fn ensure_all_paint_buffers(&mut self) {
        for object in &mut self.projection.objects {
            if object.paint_layers.is_empty() {
                object.paint_layers.push(LowpolyPaintLayer::new("Base"));
            }
            for layer in &mut object.paint_layers {
                if layer.pixels.len() != LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4 {
                    layer.pixels = empty_paint_pixels();
                }
            }
        }
    }

    pub fn layer_pixels(&self, object_id: &str, layer_index: usize) -> Result<&[u8], LowpolyCoreError> {
        layer_pixels_at(&self.projection, object_id, layer_index).ok_or(LowpolyCoreError::LayerIndexOutOfRange)
    }

    pub fn layer_pixels_mut(&mut self, object_id: &str, layer_index: usize) -> Result<&mut Vec<u8>, LowpolyCoreError> {
        self.ensure_paint_layer(object_id, layer_index)?;
        object_mut(&mut self.projection, object_id).and_then(|object| object.paint_layers.get_mut(layer_index)).map(|layer| &mut layer.pixels).ok_or(LowpolyCoreError::LayerIndexOutOfRange)
    }

    pub fn reload_meshes(&mut self) -> Result<(), LowpolyCoreError> {
        self.meshes.clear();
        for object in &self.projection.objects {
            let mesh = HalfedgeMesh::from_json(&object.mesh_json)?;
            self.meshes.push(mesh);
        }
        Ok(())
    }

    pub fn sync_meshes_to_projection(&mut self) -> Result<(), LowpolyCoreError> {
        for (object, mesh) in self.projection.objects.iter_mut().zip(self.meshes.iter()) {
            object.mesh_json = mesh.to_json()?;
        }
        Ok(())
    }

    pub fn active_index(&self) -> Option<usize> {
        self.projection.objects.iter().position(|o| o.id == self.active_object_id)
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
        self.projection.objects.iter().position(|o| o.id == object_id).ok_or(LowpolyCoreError::ObjectNotFound)
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

    /// @emoji ➕ Appends a primitive object, making it active, and returns its new id.
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
        self.projection.objects.push(LowpolyObject { id: id.clone(), name: kind.into(), transform: Default::default(), smooth_shading: false, mesh_json, paint_layers: vec![LowpolyPaintLayer::new("Base")] });
        self.meshes.push(mesh);
        self.active_object_id = id.clone();
        Ok(id)
    }

    pub fn ensure_paint_layer(&mut self, object_id: &str, layer_index: usize) -> Result<(), LowpolyCoreError> {
        let idx = self.object_index(object_id)?;
        if self.projection.objects[idx].paint_layers.is_empty() {
            self.projection.objects[idx].paint_layers.push(LowpolyPaintLayer::new("Base"));
        }
        if layer_index >= self.projection.objects[idx].paint_layers.len() {
            return Err(LowpolyCoreError::LayerIndexOutOfRange);
        }
        Ok(())
    }

    pub fn tessellate_transfer_json(mesh: &HalfedgeMesh) -> Result<serde_json::Value, LowpolyCoreError> {
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
        for (idx, object) in self.projection.objects.iter().enumerate() {
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
        Ok(composite_layer_pixels(&self.projection.objects[idx].paint_layers))
    }

    /// @emoji 🖌️ Stamps a soft brush (or eraser) into a layer's pixel buffer in place.
    #[allow(clippy::too_many_arguments, reason = "1:1 forwarder for stamp_brush's own justified 8 args plus object_id/layer_index; a params struct would only move the same fields around for this single call site")]
    pub fn paint_stroke(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, radius: f32, color: [u8; 4], hardness: f32, opacity: f32, eraser: bool) -> Result<(), LowpolyCoreError> {
        let layer_pixels = self.layer_pixels_mut(object_id, layer_index)?;
        stamp_brush(layer_pixels, u, v, radius, color, hardness, opacity, eraser);
        Ok(())
    }

    pub fn fill_bucket(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, color: [u8; 4]) -> Result<(), LowpolyCoreError> {
        let layer_pixels = self.layer_pixels_mut(object_id, layer_index)?;
        flood_fill(layer_pixels, u, v, color);
        Ok(())
    }

    pub fn sample_pixel(&self, object_id: &str, u: f32, v: f32) -> Result<[u8; 4], LowpolyCoreError> {
        let composite = self.composite_layers(object_id)?;
        Ok(sample_pixel_from(&composite, u, v))
    }
}

/// @emoji 🎨 Alpha-composites an object's paint layers into one RGBA buffer (bottom to top).
pub fn composite_layer_pixels(layers: &[LowpolyPaintLayer]) -> Vec<u8> {
    let mut out = vec![0u8; LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4];
    for layer in layers.iter() {
        if !layer.visible {
            continue;
        }
        let pixels = layer.pixels.as_slice();
        let opacity = layer.opacity.clamp(0.0, 1.0);
        for (dst, src) in out.chunks_mut(4).zip(pixels.chunks(4)) {
            let sa = (src.get(3).copied().unwrap_or(255) as f32 / 255.0) * opacity;
            let da = dst[3] as f32 / 255.0;
            let out_a = sa + da * (1.0 - sa);
            if out_a < 1e-6 {
                continue;
            }
            for (c, dst_c) in dst.iter_mut().enumerate().take(3) {
                let sc = src.get(c).copied().unwrap_or(0) as f32 / 255.0;
                let dc = *dst_c as f32 / 255.0;
                *dst_c = ((sc * sa + dc * da * (1.0 - sa)) / out_a * 255.0).round().clamp(0.0, 255.0) as u8;
            }
            dst[3] = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
        }
    }
    out
}

/// @emoji 🖌️ Stamps a soft round brush (or eraser) into a raw RGBA buffer in place. Shared by the
/// compute session and the plugin's mid-drag scratch buffer.
#[allow(clippy::too_many_arguments, reason = "one brush stamp per call site; a params struct would only move the same 8 fields around for this single leaf fn")]
pub fn stamp_brush(pixels: &mut [u8], u: f32, v: f32, radius: f32, color: [u8; 4], hardness: f32, opacity: f32, eraser: bool) {
    let size = LOWPOLY_PAINT_TEXTURE_SIZE as f32;
    let cx = (u.clamp(0.0, 1.0) * (size - 1.0)).round() as i32;
    let cy = ((1.0 - v.clamp(0.0, 1.0)) * (size - 1.0)).round() as i32;
    let r = radius.max(0.5);
    let r_i = r.ceil() as i32;
    let hard = hardness.clamp(0.0, 1.0);
    let alpha_scale = opacity.clamp(0.0, 1.0);
    for y in (cy - r_i)..=(cy + r_i) {
        for x in (cx - r_i)..=(cx + r_i) {
            if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 {
                continue;
            }
            let dx = x as f32 - cx as f32;
            let dy = y as f32 - cy as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > r {
                continue;
            }
            let t = 1.0 - dist / r;
            let falloff = hard + (1.0 - hard) * t;
            let stamp = (falloff * alpha_scale * 255.0).round().clamp(0.0, 255.0) as u8;
            let offset = (y as usize * LOWPOLY_PAINT_TEXTURE_SIZE + x as usize) * 4;
            if eraser {
                let current = pixels[offset + 3];
                pixels[offset + 3] = current.saturating_sub(stamp);
            } else {
                pixels[offset..(3 + offset)].copy_from_slice(&color[..3]);
                let current = pixels[offset + 3];
                pixels[offset + 3] = current.saturating_add(stamp);
            }
        }
    }
}

/// @emoji 🪣 Flood-fills a contiguous same-color region of a raw RGBA buffer in place.
pub fn flood_fill(pixels: &mut [u8], u: f32, v: f32, color: [u8; 4]) {
    let size = LOWPOLY_PAINT_TEXTURE_SIZE;
    let sx = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let sy = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let start = (sy * size + sx) * 4;
    let target = [pixels[start], pixels[start + 1], pixels[start + 2], pixels[start + 3]];
    let mut stack = vec![(sx, sy)];
    let mut visited = vec![false; size * size];
    while let Some((x, y)) = stack.pop() {
        let pi = y * size + x;
        if visited[pi] {
            continue;
        }
        visited[pi] = true;
        let offset = pi * 4;
        let pixel = [pixels[offset], pixels[offset + 1], pixels[offset + 2], pixels[offset + 3]];
        if pixel != target {
            continue;
        }
        pixels[offset..(4 + offset)].copy_from_slice(&color);
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < size {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < size {
            stack.push((x, y + 1));
        }
    }
}

/// @emoji 💧 Reads one RGBA sample from a composited buffer at UV.
pub fn sample_pixel_from(composite: &[u8], u: f32, v: f32) -> [u8; 4] {
    let size = LOWPOLY_PAINT_TEXTURE_SIZE;
    let x = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let y = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let offset = (y * size + x) * 4;
    [composite[offset], composite[offset + 1], composite[offset + 2], composite[offset + 3]]
}

/// @emoji 🧮 Coalesces a `before`/`after` layer-buffer pair into the minimal contiguous pixel runs
/// (`(offset, bytes)`) that turn `before` into `after`; the seam where a mutated scratch buffer becomes a
/// `PaintStroke` operation. Returns raw `(offset, bytes)` tuples — `op` wraps each into its own `PixelRun`.
pub fn pixel_runs_from_diff(before: &[u8], after: &[u8]) -> Vec<(u32, Vec<u8>)> {
    let mut runs = Vec::new();
    let len = before.len().min(after.len());
    let mut index = 0;
    while index < len {
        if before[index] == after[index] {
            index += 1;
            continue;
        }
        let start = index;
        while index < len && before[index] != after[index] {
            index += 1;
        }
        runs.push((start as u32, after[start..index].to_vec()));
    }
    runs
}
//#endregion 🔖ComputeSession

//#region 🔖MeshTransfer
/// @emoji 🧵 Builds a `MeshData` transfer payload from a raw tessellation-transfer JSON value (as
/// produced by `LowpolyDocument::tessellate_transfer_json`), attaching a composited paint texture when
/// one is supplied. Shared by the app's live 3D scene builder and media export.
pub fn mesh_data_from_transfer(transfer: &Value, paint_texture: Option<String>) -> MeshData {
    let read_f32 = |key: &str| -> Vec<f32> {
        transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
    };
    let read_u32 = |key: &str| -> Vec<u32> {
        transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
    };
    let read_u8 = |key: &str| -> Vec<u8> {
        transfer.get(key).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
    };
    MeshData {
        positions: read_f32("positions"),
        normals: read_f32("normals"),
        indices: read_u32("indices"),
        uvs: read_f32("uvs"),
        face_ids: read_u32("faceIds"),
        vertex_ids: read_u32("vertexIds"),
        edge_positions: read_f32("edgePositions"),
        edge_ids: read_u32("edgeIds"),
        edge_uvs: read_f32("edgeUvs"),
        edge_is_seam: read_u8("edgeIsSeam"),
        paint_texture_base64: paint_texture,
        ..MeshData::default()
    }
}
//#endregion 🔖MeshTransfer

//#region 🔖MediaExportImport
/// 🔺 Tessellates a lowpoly document's active object into a `MeshData` for media export.
pub fn lowpoly_mesh_from_document(doc: &serde_json::Value) -> Result<MeshData, String> {
    let projection: LowpolyProjection = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    let loaded = LowpolyDocument::new(projection).map_err(|e| e.to_string())?;
    Ok(loaded
        .active_mesh()
        .ok()
        .and_then(|mesh| LowpolyDocument::tessellate_transfer_json(mesh).ok())
        .map(|transfer| mesh_data_from_transfer(&transfer, None))
        .unwrap_or_default())
}

/// 🔺 Rebuilds a fresh single-object lowpoly projection from a DWG-imported mesh.
pub fn lowpoly_document_from_mesh(mesh: &MeshData) -> Result<serde_json::Value, String> {
    let halfedge = HalfedgeMesh::from_indexed_triangles(&mesh.positions, &mesh.indices).map_err(|err| format!("{err:?}"))?;
    let mesh_json = halfedge.to_json().map_err(|err| format!("{err:?}"))?;
    let projection = lowpoly::projection_from_mesh_json(&mesh_json, "obj-1", "Imported Mesh");
    serde_json::to_value(projection).map_err(|err| err.to_string())
}

/// 🧊 Minimal document wrapper for `3d.mesh` resources — no dedicated schema exists yet.
pub fn mesh_document_from_mesh(mesh: &MeshData) -> Result<serde_json::Value, String> {
    let mesh_value = serde_json::to_value(mesh).map_err(|err| err.to_string())?;
    Ok(serde_json::json!({ "schema": "mesh.document", "mesh": mesh_value }))
}

pub fn mesh_from_mesh_document(doc: &serde_json::Value) -> Result<MeshData, String> {
    doc.get("mesh")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .filter(|mesh: &MeshData| !mesh.positions.is_empty() && !mesh.indices.is_empty())
        .map(Ok)
        .unwrap_or_else(|| Ok(semio_framework_plugin::mesh_from_kind("box")))
}
//#endregion 🔖MediaExportImport

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_projection_has_concrete_forest_left_object() {
        let projection = default_projection();
        assert_eq!(projection.schema, lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
        assert_eq!(projection.objects.len(), 1);
        assert_eq!(projection.objects[0].name, "Concrete Forest Left");
    }

    #[test]
    fn default_concrete_forest_mesh_has_no_spanning_support_gap_faces() {
        let projection = default_projection();
        let mesh = HalfedgeMesh::from_json(&projection.objects[0].mesh_json).expect("default mesh");
        assert!(
            (0..mesh.face_count()).any(|fi| mesh.face_vertex_ids(FaceId(fi as u32)).map(|v| v.len()).unwrap_or(0) >= 8),
            "expected coplanar-merged plate-side n-gon with >= 8 corners"
        );
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            let mut min_z = f32::MAX;
            let mut max_z = f32::MIN;
            for vid in verts {
                let p = mesh.vertex_position(vid).expect("vertex");
                min_x = min_x.min(p.x());
                max_x = max_x.max(p.x());
                min_z = min_z.min(p.z());
                max_z = max_z.max(p.z());
            }
            assert!(
                !((max_x - min_x) > 4.0 && (max_z - min_z) > 1.0),
                "default mesh face {fi} spans the support gap — CAD wire rebuild regressed to fill_holes caps"
            );
        }
    }

    #[test]
    fn document_loads_meshes() {
        let doc = LowpolyDocument::new(default_projection()).unwrap();
        assert_eq!(doc.meshes.len(), 1);
        assert!(doc.meshes[0].face_count() > 0);
    }

    #[test]
    fn active_mesh_tessellates() {
        let doc = LowpolyDocument::new(default_projection()).unwrap();
        let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
        assert!(!transfer.positions.is_empty());
        assert!(!transfer.indices.is_empty());
    }

    #[test]
    fn add_primitive_box() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        let id = doc.add_primitive("box").unwrap();
        assert!(doc.projection.objects.iter().any(|o| o.id == id));
        assert_eq!(doc.meshes.len(), 2);
    }

    #[test]
    fn tessellate_all_returns_every_object() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        let _ = doc.add_primitive("box").unwrap();
        let json = doc.tessellate_all_json().unwrap();
        let items: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn projection_json_embeds_paint_pixels_as_base64() {
        let doc = LowpolyDocument::new(default_projection()).unwrap();
        let json = serde_json::to_string(&doc.projection).unwrap();
        assert!(json.contains("\"pixels\""));
        // base64 white, never a raw integer array.
        assert!(!json.contains("255,255,255"));
    }

    #[test]
    fn default_projection_mesh_has_unwrapped_uvs() {
        let doc = LowpolyDocument::new(default_projection()).unwrap();
        let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
        assert!(transfer.uvs.iter().any(|uv| *uv > 0.0));
    }

    #[test]
    fn paint_stroke_writes_pixels() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        let object_id = doc.active_object_id.clone();
        doc.paint_stroke(&object_id, 0, 0.5, 0.5, 4.0, [255, 0, 0, 255], 0.5, 1.0, false).unwrap();
        let composite = doc.composite_layers(&object_id).unwrap();
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        let center = (size / 2 * size + size / 2) * 4;
        assert!(composite[center] > 200);
    }

    #[test]
    fn projection_round_trips_paint_pixels_through_base64_json() {
        let mut projection = default_projection();
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        let json = serde_json::to_string(&projection).unwrap();
        let restored: LowpolyProjection = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, projection);
    }

    #[test]
    fn pixel_runs_from_diff_captures_only_changed_bytes() {
        let mut before = vec![0u8; 16];
        let mut after = before.clone();
        after[4] = 9;
        after[5] = 9;
        after[10] = 3;
        let runs = pixel_runs_from_diff(&before, &after);
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].0, 4);
        assert_eq!(runs[0].1, vec![9, 9]);
        assert_eq!(runs[1].0, 10);
        assert_eq!(runs[1].1, vec![3]);
        before[4] = 9;
        before[5] = 9;
        before[10] = 3;
        assert!(pixel_runs_from_diff(&before, &after).is_empty());
    }

    //#region 🔖ComputeSessionCoverage
    #[test]
    fn add_primitive_supports_every_known_kind() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        for kind in ["plane", "cylinder", "cone", "ico_sphere"] {
            let id = doc.add_primitive(kind).unwrap();
            assert_eq!(doc.active_object_id(), id);
            assert!(doc.projection().objects.iter().any(|o| o.id == id));
        }
        assert_eq!(doc.projection().objects.len(), 5);
    }

    #[test]
    fn add_primitive_unknown_kind_errors() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        let result = doc.add_primitive("teapot");
        assert!(matches!(result, Err(LowpolyCoreError::UnknownPrimitive(kind)) if kind == "teapot"));
    }

    #[test]
    fn object_index_errors_for_unknown_id() {
        let doc = LowpolyDocument::new(default_projection()).unwrap();
        assert!(matches!(doc.object_index("missing"), Err(LowpolyCoreError::ObjectNotFound)));
    }

    #[test]
    fn ensure_paint_layer_errors_for_unknown_object_and_out_of_range_index() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        let object_id = doc.active_object_id().to_string();
        assert!(matches!(doc.ensure_paint_layer("missing", 0), Err(LowpolyCoreError::ObjectNotFound)));
        assert!(matches!(doc.ensure_paint_layer(&object_id, 99), Err(LowpolyCoreError::LayerIndexOutOfRange)));
    }

    #[test]
    fn layer_pixels_errors_for_out_of_range_index() {
        let doc = LowpolyDocument::new(default_projection()).unwrap();
        let object_id = doc.active_object_id().to_string();
        assert!(matches!(doc.layer_pixels(&object_id, 5), Err(LowpolyCoreError::LayerIndexOutOfRange)));
    }

    #[test]
    fn active_mesh_errors_when_active_object_id_is_unknown() {
        let doc = LowpolyDocument::with_context(default_projection(), "does-not-exist".into(), LowpolySelection::default()).unwrap();
        assert!(matches!(doc.active_mesh(), Err(LowpolyCoreError::NoActiveObject)));
    }

    #[test]
    fn mesh_at_returns_none_past_object_count() {
        let doc = LowpolyDocument::new(default_projection()).unwrap();
        assert!(doc.mesh_at(0).is_some());
        assert!(doc.mesh_at(99).is_none());
    }

    #[test]
    fn sync_meshes_to_projection_writes_back_mesh_json() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.active_mesh_mut().unwrap().translate(Vec3::new(1.0, 0.0, 0.0)).unwrap();
        let idx = doc.active_index().unwrap();
        let before = doc.projection().objects[idx].mesh_json.clone();
        doc.sync_meshes_to_projection().unwrap();
        assert_ne!(doc.projection().objects[idx].mesh_json, before);
    }

    #[test]
    fn normalize_selection_mode_maps_object_to_mesh_and_passes_through_others() {
        assert_eq!(LowpolyDocument::normalize_selection_mode("object"), "mesh");
        assert_eq!(LowpolyDocument::normalize_selection_mode("face"), "face");
        assert_eq!(LowpolyDocument::normalize_selection_mode("vertex"), "vertex");
    }

    #[test]
    fn apply_selection_normalizes_mode_and_stores_ids() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        doc.apply_selection("object", vec![3, 4]);
        assert_eq!(doc.selection().mode, "mesh");
        assert_eq!(doc.selection().ids, vec![3, 4]);
    }

    #[test]
    fn selected_ids_are_empty_when_selection_mode_mismatches() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        doc.apply_selection("face", vec![1, 2]);
        assert!(doc.selected_vertex_ids().is_empty());
        assert!(doc.selected_edge_ids().is_empty());
        assert_eq!(doc.selected_face_ids().len(), 2);
    }

    #[test]
    fn selection_vertex_ids_face_mode_dedupes_shared_vertices() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
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
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        doc.add_primitive("box").unwrap();
        doc.apply_selection("edge", vec![0]);
        let verts = doc.selection_vertex_ids().unwrap();
        let mesh = doc.active_mesh().unwrap();
        let (v0, v1) = mesh.edge_endpoints(EdgeId(0)).unwrap();
        assert_eq!(verts, vec![v0, v1]);
    }

    #[test]
    fn selection_vertex_ids_mesh_mode_is_empty() {
        let doc = LowpolyDocument::new(default_projection()).unwrap();
        assert!(doc.selection_vertex_ids().unwrap().is_empty());
    }

    #[test]
    fn selection_transform_pivot_mesh_mode_averages_all_vertices() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
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
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
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
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        doc.apply_selection("vertex", vec![]);
        let pivot = doc.selection_transform_pivot().unwrap();
        assert_eq!((pivot.x(), pivot.y(), pivot.z()), (0.0, 0.0, 0.0));
    }

    #[test]
    fn ensure_all_paint_buffers_adds_missing_layer_and_fixes_wrong_size() {
        let mut projection = default_projection();
        projection.objects[0].paint_layers.clear();
        let mut doc = LowpolyDocument::new(projection).unwrap();
        assert_eq!(doc.projection().objects[0].paint_layers.len(), 1);
        assert_eq!(doc.projection().objects[0].paint_layers[0].name, "Base");
        doc.projection_mut().objects[0].paint_layers[0].pixels = vec![1, 2, 3];
        doc.ensure_all_paint_buffers();
        assert_eq!(doc.projection().objects[0].paint_layers[0].pixels.len(), LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4);
    }

    #[test]
    fn fill_bucket_and_sample_pixel_reflect_new_color() {
        let mut doc = LowpolyDocument::new(default_projection()).unwrap();
        let object_id = doc.active_object_id().to_string();
        doc.fill_bucket(&object_id, 0, 0.5, 0.5, [10, 20, 30, 255]).unwrap();
        assert_eq!(doc.sample_pixel(&object_id, 0.5, 0.5).unwrap(), [10, 20, 30, 255]);
    }

    #[test]
    fn composite_layer_pixels_skips_invisible_layers() {
        let mut layer = LowpolyPaintLayer::new("Hidden");
        layer.visible = false;
        layer.pixels = vec![255, 0, 0, 255];
        let out = composite_layer_pixels(&[layer]);
        assert_eq!(&out[0..4], &[0, 0, 0, 0]);
    }

    #[test]
    fn composite_layer_pixels_blends_partial_opacity_over_transparent_base() {
        let mut layer = LowpolyPaintLayer::new("Half");
        layer.opacity = 0.5;
        layer.pixels = vec![200, 100, 50, 255];
        let out = composite_layer_pixels(&[layer]);
        assert_eq!(&out[0..4], &[200, 100, 50, 128]);
    }

    #[test]
    fn composite_layer_pixels_blends_stacked_opaque_and_translucent_layers() {
        let base = LowpolyPaintLayer { name: "Base".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), pixels: vec![255, 0, 0, 255] };
        let top = LowpolyPaintLayer { name: "Top".into(), visible: true, opacity: 0.5, blend_mode: "normal".into(), pixels: vec![0, 0, 255, 255] };
        let out = composite_layer_pixels(&[base, top]);
        assert_eq!(&out[0..4], &[128, 0, 128, 255]);
    }

    #[test]
    fn stamp_brush_eraser_reduces_alpha_at_center() {
        let mut pixels = empty_paint_pixels();
        stamp_brush(&mut pixels, 0.5, 0.5, 4.0, [0, 0, 0, 0], 1.0, 1.0, true);
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        let center = (size / 2 * size + size / 2) * 4;
        assert!(pixels[center + 3] < 255);
    }

    #[test]
    fn flood_fill_only_affects_contiguous_matching_region() {
        let mut pixels = empty_paint_pixels();
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        for y in 0..10 {
            for x in 0..10 {
                let offset = (y * size + x) * 4;
                pixels[offset..offset + 4].copy_from_slice(&[0, 255, 0, 255]);
            }
        }
        flood_fill(&mut pixels, 0.99, 0.01, [255, 0, 0, 255]);
        assert_eq!(&pixels[0..4], &[0, 255, 0, 255]);
        let far_offset = (500 * size + 500) * 4;
        assert_eq!(&pixels[far_offset..far_offset + 4], &[255, 0, 0, 255]);
    }
    //#endregion 🔖ComputeSessionCoverage
}
//#endregion 🧪Tests

//#region 🔖ExportConcreteForestMeshTests
#[cfg(test)]
mod export_concrete_forest_mesh_tests {
    use cad_document_engine::geometry_import::{objects_from_fixture_model, parse_geometry};
    use kernel_3d_brepkit::BrepkitKernel;
    use kernel_3d_engine::GeometryHandle;
    use kernel_3d_mesh::{FaceId, HalfedgeMesh, Vec3 as MeshVec3, VertexId};
    use serde_json::Value;
    use std::collections::HashMap;

    /// Asserts every directed edge (by vertex id, after welding) has an opposite-winding counterpart, i.e. the
    /// mesh has no open boundary loops.
    fn assert_watertight(mesh: &HalfedgeMesh) {
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
            let n = verts.len();
            for i in 0..n {
                *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
            }
        }
        let open: Vec<(u32, u32)> = directed.keys().copied().filter(|&(a, b)| !directed.contains_key(&(b, a))).collect();
        assert!(open.is_empty(), "mesh is not watertight: {} open boundary edges, e.g. {:?}", open.len(), &open[..open.len().min(5)]);
    }

    fn open_boundary_count(mesh: &HalfedgeMesh) -> usize {
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
            let n = verts.len();
            for i in 0..n {
                *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
            }
        }
        directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
    }

    /// Spurious `fill_holes` caps on this solid spanned the open gap between vertical supports: large X
    /// extent *and* large Z extent on one face. Real CAD faces are either horizontal slabs (small Δz) or
    /// vertical support sides (small Δx).
    fn assert_no_spanning_face_across_support_gap(mesh: &HalfedgeMesh) {
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).expect("face verts");
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            let mut min_z = f32::MAX;
            let mut max_z = f32::MIN;
            for vid in verts {
                let p = mesh.vertex_position(vid).expect("vertex");
                min_x = min_x.min(p.x());
                max_x = max_x.max(p.x());
                min_z = min_z.min(p.z());
                max_z = max_z.max(p.z());
            }
            let dx = max_x - min_x;
            let dz = max_z - min_z;
            assert!(
                !(dx > 4.0 && dz > 1.0),
                "face {fi} spans the support gap (dx={dx:.3}, dz={dz:.3}) — likely a filled hole, not a CAD face"
            );
        }
    }

    #[test]
    fn export_concrete_forest_left_lowpoly_mesh_json() {
        if std::env::var("EXPORT_LOWPOLY_FOREST_MESH").ok().as_deref() != Some("1") {
            return;
        }
        let source = include_str!("../../../../../../../../../✏️s/🔌plugin/📐cad/🖼️asset/🎮play/🔣hexagonal-cut-concrete-forest-left.🔣model.json");
        let root: Value = serde_json::from_str(source).expect("fixture");
        let geometry = parse_geometry(root.pointer("/models/0/model/geometry"));
        let objects = root.pointer("/models/0/model/objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
        let mut kernel = BrepkitKernel::new();
        let imported = objects_from_fixture_model(&mut kernel, &objects, &geometry);
        let handle = GeometryHandle(imported[0].solid_handle.clone().expect("handle"));
        let (positions, face_loops) = kernel.solid_face_loops_sync(&handle).expect("CAD face loops");
        let holed = face_loops.iter().filter(|(_, holes)| !holes.is_empty()).count();
        eprintln!(
            "[DEBUG] CAD face loops: verts={} faces={} holed={}",
            positions.len(),
            face_loops.len(),
            holed
        );
        let mut mesh = HalfedgeMesh::from_face_loops(&positions, &face_loops).expect("halfedge from CAD wires");
        let flips = mesh.orient_faces_consistently().expect("orient faces");
        eprintln!(
            "[DEBUG] after wire build+orient: verts={} faces={} flips={} open={}",
            mesh.vertex_count(),
            mesh.face_count(),
            flips,
            open_boundary_count(&mesh)
        );
        let before_merge = mesh.face_count();
        let merges = mesh.merge_coplanar_faces().expect("merge coplanar faces");
        eprintln!(
            "[DEBUG] after coplanar merge: verts={} faces={} merges={} (was {}) open={}",
            mesh.vertex_count(),
            mesh.face_count(),
            merges,
            before_merge,
            open_boundary_count(&mesh)
        );
        assert!(
            mesh.face_count() <= before_merge,
            "coplanar merge must not increase face count"
        );
        assert!(
            merges > 0 || before_merge == mesh.face_count(),
            "expected coplanar merge to join adjacent CAD faces on the plate/supports"
        );
        assert!(
            (0..mesh.face_count()).any(|fi| mesh.face_vertex_ids(FaceId(fi as u32)).map(|v| v.len()).unwrap_or(0) > 3),
            "expected at least one non-triangle CAD face"
        );
        assert_watertight(&mesh);
        assert_no_spanning_face_across_support_gap(&mesh);
        let mut min = MeshVec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = MeshVec3::new(f32::MIN, f32::MIN, f32::MIN);
        for index in 0..mesh.vertex_count() {
            let position = mesh.vertex_position(VertexId(index as u32)).expect("vertex");
            min = MeshVec3([min.x().min(position.x()), min.y().min(position.y()), min.z().min(position.z())]);
            max = MeshVec3([max.x().max(position.x()), max.y().max(position.y()), max.z().max(position.z())]);
        }
        let center = min.add(max).scale(0.5);
        mesh.translate(center.scale(-1.0)).expect("center mesh");
        let _ = mesh.unwrap_uv();
        let json = mesh.to_json().expect("mesh json");
        eprintln!("LOWPOLY_FOREST_MESH_JSON_START");
        eprintln!("{json}");
        eprintln!("LOWPOLY_FOREST_MESH_JSON_END");
    }
}
//#endregion 🔖ExportConcreteForestMeshTests
