//! 🔷 Lowpoly core: mesh editing session wrapping kernel_3d_mesh.

use std::collections::HashMap;

use kernel_3d_mesh::{
    EdgeId, FaceId, HalfedgeMesh, MeshKernelError, MirrorAxis, Vec3, VertexId, WeldMode,
};
use serde::{Deserialize, Serialize};

//#region Fixture

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyTransform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Default for LowpolyTransform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

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
        Self {
            mesh: true,
            vertex: false,
            edge: false,
            face: false,
        }
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
        Self {
            targets: LowpolySelectionTargets::default(),
            keys: Vec::new(),
            mode: "mesh".into(),
            ids: Vec::new(),
        }
    }
}

pub const LOWPOLY_PAINT_TEXTURE_SIZE: usize = 1024;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyPaintLayer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub blend_mode: String,
}

impl LowpolyPaintLayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            visible: true,
            opacity: 1.0,
            blend_mode: "normal".into(),
        }
    }
}

fn empty_paint_pixels() -> Vec<u8> {
    let mut pixels = vec![0u8; LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4];
    for chunk in pixels.chunks_mut(4) {
        chunk[0] = 255;
        chunk[1] = 255;
        chunk[2] = 255;
        chunk[3] = 255;
    }
    pixels
}

fn prepare_paint_mesh(mesh: &mut HalfedgeMesh) {
    let _ = mesh.unwrap_uv();
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyObject {
    pub id: String,
    pub name: String,
    pub transform: LowpolyTransform,
    pub smooth_shading: bool,
    pub mesh_json: String,
    #[serde(default)]
    pub paint_layers: Vec<LowpolyPaintLayer>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyFixture {
    pub schema: String,
    pub objects: Vec<LowpolyObject>,
    pub active_object_id: String,
    pub selection: LowpolySelection,
}

impl Default for LowpolyFixture {
    fn default() -> Self {
        default_fixture()
    }
}

pub fn default_fixture() -> LowpolyFixture {
    let mut mesh = HalfedgeMesh::ico_sphere_prim(1.0, 1).unwrap_or_else(|_| HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap());
    let _ = mesh.extrude_faces(&[FaceId(0)], 0.3);
    prepare_paint_mesh(&mut mesh);
    let mesh_json = mesh.to_json().unwrap_or_else(|_| "{}".into());
    LowpolyFixture {
        schema: "lowpoly.fixture".into(),
        objects: vec![LowpolyObject {
            id: "obj-1".into(),
            name: "Rock".into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh_json,
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
        }],
        active_object_id: "obj-1".into(),
        selection: LowpolySelection {
            targets: LowpolySelectionTargets::default(),
            keys: vec!["lowpoly:obj-1:0:mesh:0".into()],
            mode: "mesh".into(),
            ids: vec![0],
        },
    }
}

//#endregion Fixture

//#region Document

struct LowpolyDocument {
    fixture: LowpolyFixture,
    meshes: Vec<HalfedgeMesh>,
    next_object_serial: u32,
    paint_pixels: HashMap<String, Vec<Vec<u8>>>,
}

impl LowpolyDocument {
    fn new(fixture: LowpolyFixture) -> Result<Self, String> {
        let mut doc = Self {
            fixture,
            meshes: Vec::new(),
            next_object_serial: 100,
            paint_pixels: HashMap::new(),
        };
        doc.reload_meshes()?;
        doc.ensure_all_paint_buffers();
        Ok(doc)
    }

    fn replace_fixture(&mut self, fixture: LowpolyFixture, preserved_pixels: HashMap<String, Vec<Vec<u8>>>) -> Result<(), String> {
        self.fixture = fixture;
        self.reload_meshes()?;
        self.paint_pixels = preserved_pixels;
        self.ensure_all_paint_buffers();
        Ok(())
    }

    fn ensure_all_paint_buffers(&mut self) {
        let specs: Vec<(String, usize)> = self
            .fixture
            .objects
            .iter()
            .map(|obj| (obj.id.clone(), obj.paint_layers.len()))
            .collect();
        for (object_id, layer_count) in specs {
            self.ensure_object_paint_buffers(&object_id, layer_count);
        }
    }

    fn ensure_object_paint_buffers(&mut self, object_id: &str, layer_count: usize) {
        let layers = self.paint_pixels.entry(object_id.to_string()).or_default();
        while layers.len() < layer_count {
            layers.push(empty_paint_pixels());
        }
        layers.truncate(layer_count.max(1));
        if layers.is_empty() {
            layers.push(empty_paint_pixels());
        }
    }

    fn layer_pixels(&self, object_id: &str, layer_index: usize) -> Result<&[u8], String> {
        self.paint_pixels
            .get(object_id)
            .and_then(|layers| layers.get(layer_index))
            .map(|pixels| pixels.as_slice())
            .ok_or_else(|| "layer index out of range".into())
    }

    fn layer_pixels_mut(&mut self, object_id: &str, layer_index: usize) -> Result<&mut Vec<u8>, String> {
        let layer_count = self
            .object_index(object_id)
            .ok()
            .and_then(|idx| self.fixture.objects.get(idx))
            .map(|obj| obj.paint_layers.len().max(1))
            .unwrap_or(1);
        self.ensure_object_paint_buffers(object_id, layer_count);
        self.paint_pixels
            .get_mut(object_id)
            .and_then(|layers| layers.get_mut(layer_index))
            .ok_or_else(|| "layer index out of range".into())
    }

    fn reload_meshes(&mut self) -> Result<(), String> {
        self.meshes.clear();
        for obj in &self.fixture.objects {
            let mesh = HalfedgeMesh::from_json(&obj.mesh_json).map_err(|e| format!("{e:?}"))?;
            self.meshes.push(mesh);
        }
        Ok(())
    }

    fn sync_meshes_to_fixture(&mut self) -> Result<(), String> {
        for (obj, mesh) in self.fixture.objects.iter_mut().zip(self.meshes.iter()) {
            obj.mesh_json = mesh.to_json().map_err(|e| format!("{e:?}"))?;
        }
        Ok(())
    }

    fn active_index(&self) -> Option<usize> {
        self.fixture
            .objects
            .iter()
            .position(|o| o.id == self.fixture.active_object_id)
    }

    fn active_mesh_mut(&mut self) -> Result<&mut HalfedgeMesh, String> {
        let idx = self.active_index().ok_or_else(|| "no active object".to_string())?;
        self.meshes.get_mut(idx).ok_or_else(|| "mesh missing".to_string())
    }

    fn active_mesh(&self) -> Result<&HalfedgeMesh, String> {
        let idx = self.active_index().ok_or_else(|| "no active object".to_string())?;
        self.meshes.get(idx).ok_or_else(|| "mesh missing".to_string())
    }

    fn selected_face_ids(&self) -> Vec<FaceId> {
        if self.fixture.selection.mode != "face" {
            return Vec::new();
        }
        self.fixture.selection.ids.iter().map(|&id| FaceId(id)).collect()
    }

    fn selected_vertex_ids(&self) -> Vec<VertexId> {
        if self.fixture.selection.mode != "vertex" {
            return Vec::new();
        }
        self.fixture.selection.ids.iter().map(|&id| VertexId(id)).collect()
    }

    fn selected_edge_ids(&self) -> Vec<kernel_3d_mesh::EdgeId> {
        if self.fixture.selection.mode != "edge" {
            return Vec::new();
        }
        self.fixture.selection.ids.iter().map(|&id| kernel_3d_mesh::EdgeId(id)).collect()
    }

    fn normalize_selection_mode(mode: &str) -> String {
        if mode == "object" { "mesh".into() } else { mode.into() }
    }

    fn apply_selection(&mut self, mode: &str, ids: Vec<u32>) {
        self.fixture.selection.mode = Self::normalize_selection_mode(mode);
        self.fixture.selection.ids = ids;
    }

    fn selection_vertex_ids(&self) -> Result<Vec<VertexId>, String> {
        let mesh = self.active_mesh()?;
        match self.fixture.selection.mode.as_str() {
            "vertex" => Ok(self.selected_vertex_ids()),
            "face" => {
                let mut verts = Vec::new();
                let mut seen = std::collections::HashSet::new();
                for fid in self.selected_face_ids() {
                    for vid in mesh.face_vertex_ids(fid).map_err(|e| format!("{e:?}"))? {
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
                    let (v0, v1) = mesh.edge_endpoints(eid).map_err(|e| format!("{e:?}"))?;
                    for vid in [v0, v1] {
                        if seen.insert(vid.0) {
                            verts.push(vid);
                        }
                    }
                }
                Ok(verts)
            }
            "mesh" => Ok(Vec::new()),
            _ => Ok(Vec::new()),
        }
    }

    fn selection_transform_pivot(&self) -> Result<Vec3, String> {
        let mesh = self.active_mesh()?;
        if self.fixture.selection.mode == "mesh" {
            let count = mesh.vertex_count();
            if count == 0 {
                return Ok(Vec3::new(0.0, 0.0, 0.0));
            }
            let mut sum = Vec3::new(0.0, 0.0, 0.0);
            for index in 0..count {
                sum = sum.add(mesh.vertex_position(VertexId(index as u32)).map_err(|e| format!("{e:?}"))?);
            }
            return Ok(sum.scale(1.0 / count as f32));
        }
        let verts = self.selection_vertex_ids()?;
        if verts.is_empty() {
            return Ok(Vec3::new(0.0, 0.0, 0.0));
        }
        let mut sum = Vec3::new(0.0, 0.0, 0.0);
        for vid in &verts {
            sum = sum.add(mesh.vertex_position(*vid).map_err(|e| format!("{e:?}"))?);
        }
        Ok(sum.scale(1.0 / verts.len() as f32))
    }

    fn add_primitive(&mut self, kind: &str) -> Result<String, String> {
        let mut mesh = match kind {
            "box" => HalfedgeMesh::box_prim(1.0, 1.0, 1.0),
            "plane" => HalfedgeMesh::plane_prim(2.0, 2.0),
            "cylinder" => HalfedgeMesh::cylinder_prim(0.5, 1.0, 12),
            "cone" => HalfedgeMesh::cone_prim(0.5, 1.0, 12),
            "ico_sphere" => HalfedgeMesh::ico_sphere_prim(0.5, 1),
            _ => return Err(format!("unknown primitive: {kind}")),
        }
        .map_err(|e| format!("{e:?}"))?;
        prepare_paint_mesh(&mut mesh);
        self.next_object_serial += 1;
        let id = format!("obj-{}", self.next_object_serial);
        let mesh_json = mesh.to_json().map_err(|e| format!("{e:?}"))?;
        self.fixture.objects.push(LowpolyObject {
            id: id.clone(),
            name: kind.into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh_json,
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
        });
        self.meshes.push(mesh);
        self.ensure_object_paint_buffers(&id, 1);
        self.fixture.active_object_id = id.clone();
        let object_index = (self.fixture.objects.len() - 1) as u32;
        self.fixture.selection = LowpolySelection {
            targets: LowpolySelectionTargets::default(),
            keys: vec![format!("lowpoly:{id}:{object_index}:mesh:{object_index}")],
            mode: "mesh".into(),
            ids: vec![object_index],
        };
        Ok(id)
    }

    fn object_index(&self, object_id: &str) -> Result<usize, String> {
        self.fixture
            .objects
            .iter()
            .position(|o| o.id == object_id)
            .ok_or_else(|| "object not found".to_string())
    }

    fn tessellate_transfer_json(mesh: &HalfedgeMesh) -> Result<serde_json::Value, String> {
        let transfer = mesh.tessellate().map_err(|e| format!("{e:?}"))?;
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

    fn tessellate_all_json(&self) -> Result<String, String> {
        let active = self.fixture.active_object_id.clone();
        let mut items = Vec::new();
        for (idx, obj) in self.fixture.objects.iter().enumerate() {
            let mesh = self.meshes.get(idx).ok_or_else(|| "mesh missing".to_string())?;
            items.push(serde_json::json!({
                "id": obj.id,
                "index": idx,
                "name": obj.name,
                "transform": obj.transform,
                "smoothShading": obj.smooth_shading,
                "active": obj.id == active,
                "tessellation": Self::tessellate_transfer_json(mesh)?,
            }));
        }
        serde_json::to_string(&items).map_err(|e| e.to_string())
    }

    fn ensure_paint_layer(&mut self, object_id: &str, layer_index: usize) -> Result<(), String> {
        let idx = self.object_index(object_id)?;
        if self.fixture.objects[idx].paint_layers.is_empty() {
            self.fixture.objects[idx].paint_layers.push(LowpolyPaintLayer::new("Base"));
        }
        if layer_index >= self.fixture.objects[idx].paint_layers.len() {
            return Err("layer index out of range".into());
        }
        Ok(())
    }

    fn composite_layers(&self, object_id: &str) -> Result<Vec<u8>, String> {
        let idx = self.object_index(object_id)?;
        let layers = &self.fixture.objects[idx].paint_layers;
        let mut out = vec![0u8; LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4];
        for (layer_index, layer) in layers.iter().enumerate() {
            if !layer.visible {
                continue;
            }
            let pixels = self.layer_pixels(object_id, layer_index)?;
            let opacity = layer.opacity.clamp(0.0, 1.0);
            for (dst, src) in out.chunks_mut(4).zip(pixels.chunks(4)) {
                let sa = (src.get(3).copied().unwrap_or(255) as f32 / 255.0) * opacity;
                let da = dst[3] as f32 / 255.0;
                let out_a = sa + da * (1.0 - sa);
                if out_a < 1e-6 {
                    continue;
                }
                for c in 0..3 {
                    let sc = src.get(c).copied().unwrap_or(0) as f32 / 255.0;
                    let dc = dst[c] as f32 / 255.0;
                    dst[c] = ((sc * sa + dc * da * (1.0 - sa)) / out_a * 255.0).round().clamp(0.0, 255.0) as u8;
                }
                dst[3] = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
            }
        }
        Ok(out)
    }

    fn paint_stroke(
        &mut self,
        object_id: &str,
        layer_index: usize,
        u: f32,
        v: f32,
        radius: f32,
        color: [u8; 4],
        hardness: f32,
        opacity: f32,
        eraser: bool,
    ) -> Result<(), String> {
        self.ensure_paint_layer(object_id, layer_index)?;
        let layer_pixels = self.layer_pixels_mut(object_id, layer_index)?;
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
                    let current = layer_pixels[offset + 3];
                    layer_pixels[offset + 3] = current.saturating_sub(stamp);
                } else {
                    for c in 0..3 {
                        layer_pixels[offset + c] = color[c];
                    }
                    let current = layer_pixels[offset + 3];
                    layer_pixels[offset + 3] = current.saturating_add(stamp).min(255);
                }
            }
        }
        Ok(())
    }

    fn fill_bucket(&mut self, object_id: &str, layer_index: usize, u: f32, v: f32, color: [u8; 4]) -> Result<(), String> {
        self.ensure_paint_layer(object_id, layer_index)?;
        let layer_pixels = self.layer_pixels_mut(object_id, layer_index)?;
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        let sx = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
        let sy = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
        let start = (sy * size + sx) * 4;
        let target = [
            layer_pixels[start],
            layer_pixels[start + 1],
            layer_pixels[start + 2],
            layer_pixels[start + 3],
        ];
        let mut stack = vec![(sx, sy)];
        let mut visited = vec![false; size * size];
        while let Some((x, y)) = stack.pop() {
            let pi = y * size + x;
            if visited[pi] {
                continue;
            }
            visited[pi] = true;
            let offset = pi * 4;
            let pixel = [
                layer_pixels[offset],
                layer_pixels[offset + 1],
                layer_pixels[offset + 2],
                layer_pixels[offset + 3],
            ];
            if pixel != target {
                continue;
            }
            for c in 0..4 {
                layer_pixels[offset + c] = color[c];
            }
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
        Ok(())
    }

    fn sample_pixel(&self, object_id: &str, u: f32, v: f32) -> Result<[u8; 4], String> {
        let composite = self.composite_layers(object_id)?;
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        let x = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
        let y = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
        let offset = (y * size + x) * 4;
        Ok([
            composite[offset],
            composite[offset + 1],
            composite[offset + 2],
            composite[offset + 3],
        ])
    }
}

fn map_err(e: MeshKernelError) -> String {
    format!("{e:?}")
}

//#endregion Document

//#region WasmSession

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = defaultFixtureJson)]
pub fn default_fixture_json() -> Result<String, JsValue> {
    serde_json::to_string(&default_fixture()).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct LowpolySession {
    doc: LowpolyDocument,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl LowpolySession {
    #[wasm_bindgen(constructor)]
    pub fn new(fixture_json: Option<String>) -> Result<LowpolySession, JsValue> {
        let fixture = match fixture_json {
            Some(json) => serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?,
            None => default_fixture(),
        };
        let doc = LowpolyDocument::new(fixture).map_err(|e| JsValue::from_str(&e))?;
        Ok(Self { doc })
    }

    #[wasm_bindgen(js_name = fixtureJson)]
    pub fn fixture_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.doc.fixture).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = loadFixtureJson)]
    pub fn load_fixture_json(&mut self, json: &str) -> Result<(), JsValue> {
        let fixture: LowpolyFixture = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let preserved_pixels = std::mem::take(&mut self.doc.paint_pixels);
        self.doc.replace_fixture(fixture, preserved_pixels).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = addPrimitive)]
    pub fn add_primitive(&mut self, kind: &str) -> Result<String, JsValue> {
        self.doc.add_primitive(kind).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setActiveObject)]
    pub fn set_active_object(&mut self, id: &str) -> Result<(), JsValue> {
        if !self.doc.fixture.objects.iter().any(|o| o.id == id) {
            return Err(JsValue::from_str("object not found"));
        }
        self.doc.fixture.active_object_id = id.into();
        Ok(())
    }

    #[wasm_bindgen(js_name = setSelection)]
    pub fn set_selection(&mut self, mode: &str, ids: Vec<u32>) -> Result<(), JsValue> {
        self.doc.apply_selection(mode, ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = tessellateActive)]
    pub fn tessellate_active(&self) -> Result<String, JsValue> {
        let mesh = self.doc.active_mesh().map_err(|e| JsValue::from_str(&e))?;
        LowpolyDocument::tessellate_transfer_json(mesh).map_err(|e| JsValue::from_str(&e)).and_then(|v| {
            serde_json::to_string(&v).map_err(|e| JsValue::from_str(&e.to_string()))
        })
    }

    #[wasm_bindgen(js_name = tessellateAll)]
    pub fn tessellate_all(&self) -> Result<String, JsValue> {
        self.doc.tessellate_all_json().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = markUvSeam)]
    pub fn mark_uv_seam(&mut self, seam: bool, edge_ids: Vec<u32>) -> Result<(), JsValue> {
        let edges: Vec<EdgeId> = edge_ids.into_iter().map(EdgeId).collect();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.mark_uv_seam(&edges, seam);
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = unwrapActive)]
    pub fn unwrap_active(&mut self) -> Result<(), JsValue> {
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.unwrap_uv().map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = compositePaintTexture)]
    pub fn composite_paint_texture(&self, object_id: &str) -> Result<Vec<u8>, JsValue> {
        self.doc.composite_layers(object_id).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = paintLayerPixels)]
    pub fn paint_layer_pixels(&self, object_id: &str, layer_index: u32) -> Result<Vec<u8>, JsValue> {
        self.doc
            .layer_pixels(object_id, layer_index as usize)
            .map(|pixels| pixels.to_vec())
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setPaintLayerPixels)]
    pub fn set_paint_layer_pixels(&mut self, object_id: &str, layer_index: u32, pixels: Vec<u8>) -> Result<(), JsValue> {
        let buffer = self
            .doc
            .layer_pixels_mut(object_id, layer_index as usize)
            .map_err(|e| JsValue::from_str(&e))?;
        if pixels.len() != buffer.len() {
            return Err(JsValue::from_str("pixel buffer length mismatch"));
        }
        buffer.copy_from_slice(&pixels);
        Ok(())
    }

    #[wasm_bindgen(js_name = paintStroke)]
    pub fn paint_stroke(
        &mut self,
        object_id: &str,
        layer_index: u32,
        u: f32,
        v: f32,
        radius: f32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        hardness: f32,
        opacity: f32,
        eraser: bool,
    ) -> Result<(), JsValue> {
        self.doc
            .paint_stroke(object_id, layer_index as usize, u, v, radius, [r, g, b, a], hardness, opacity, eraser)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = fillBucket)]
    pub fn fill_bucket(&mut self, object_id: &str, layer_index: u32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8) -> Result<(), JsValue> {
        self.doc
            .fill_bucket(object_id, layer_index as usize, u, v, [r, g, b, a])
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = samplePixel)]
    pub fn sample_pixel(&self, object_id: &str, u: f32, v: f32) -> Result<Vec<u8>, JsValue> {
        self.doc
            .sample_pixel(object_id, u, v)
            .map(|pixel| pixel.to_vec())
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = addPaintLayer)]
    pub fn add_paint_layer(&mut self, object_id: &str, name: &str) -> Result<u32, JsValue> {
        let idx = self.doc.object_index(object_id).map_err(|e| JsValue::from_str(&e))?;
        self.doc.fixture.objects[idx].paint_layers.push(LowpolyPaintLayer::new(name));
        let layer_index = self.doc.fixture.objects[idx].paint_layers.len() - 1;
        self.doc.ensure_object_paint_buffers(object_id, layer_index + 1);
        Ok(layer_index as u32)
    }

    #[wasm_bindgen(js_name = removePaintLayer)]
    pub fn remove_paint_layer(&mut self, object_id: &str, layer_index: u32) -> Result<(), JsValue> {
        let idx = self.doc.object_index(object_id).map_err(|e| JsValue::from_str(&e))?;
        let li = layer_index as usize;
        if li >= self.doc.fixture.objects[idx].paint_layers.len() {
            return Err(JsValue::from_str("layer index out of range"));
        }
        self.doc.fixture.objects[idx].paint_layers.remove(li);
        if let Some(layers) = self.doc.paint_pixels.get_mut(object_id) {
            if li < layers.len() {
                layers.remove(li);
            }
        }
        self.doc.ensure_object_paint_buffers(object_id, self.doc.fixture.objects[idx].paint_layers.len().max(1));
        Ok(())
    }

    #[wasm_bindgen(js_name = setLayerVisible)]
    pub fn set_layer_visible(&mut self, object_id: &str, layer_index: u32, visible: bool) -> Result<(), JsValue> {
        let idx = self.doc.object_index(object_id).map_err(|e| JsValue::from_str(&e))?;
        let layer = self
            .doc
            .fixture
            .objects
            .get_mut(idx)
            .and_then(|o| o.paint_layers.get_mut(layer_index as usize))
            .ok_or_else(|| JsValue::from_str("layer index out of range"))?;
        layer.visible = visible;
        Ok(())
    }

    #[wasm_bindgen(js_name = setLayerOpacity)]
    pub fn set_layer_opacity(&mut self, object_id: &str, layer_index: u32, opacity: f32) -> Result<(), JsValue> {
        let idx = self.doc.object_index(object_id).map_err(|e| JsValue::from_str(&e))?;
        let layer = self
            .doc
            .fixture
            .objects
            .get_mut(idx)
            .and_then(|o| o.paint_layers.get_mut(layer_index as usize))
            .ok_or_else(|| JsValue::from_str("layer index out of range"))?;
        layer.opacity = opacity;
        Ok(())
    }

    #[wasm_bindgen(js_name = exportObjActive)]
    pub fn export_obj_active(&self) -> Result<String, JsValue> {
        let mesh = self.doc.active_mesh().map_err(|e| JsValue::from_str(&e))?;
        mesh.to_obj().map_err(|e| JsValue::from_str(&map_err(e)))
    }

    #[wasm_bindgen(js_name = extrudeFaces)]
    pub fn extrude_faces(&mut self, distance: f32) -> Result<(), JsValue> {
        let faces = self.doc.selected_face_ids();
        if faces.is_empty() {
            return Err(JsValue::from_str("no faces selected"));
        }
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.extrude_faces(&faces, distance).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = insetFaces)]
    pub fn inset_faces(&mut self, amount: f32) -> Result<(), JsValue> {
        let faces = self.doc.selected_face_ids();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.inset_faces(&faces, amount).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = flipFaces)]
    pub fn flip_faces(&mut self, face_ids: Vec<u32>) -> Result<(), JsValue> {
        let faces: Vec<FaceId> = face_ids.into_iter().map(FaceId).collect();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.flip_faces(&faces).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = bevelEdges)]
    pub fn bevel_edges(&mut self, amount: f32, segments: u32) -> Result<(), JsValue> {
        let edges = self.doc.selected_edge_ids();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.bevel_edges(&edges, amount, segments).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = loopCut)]
    pub fn loop_cut(&mut self, cuts: u32) -> Result<(), JsValue> {
        let edges = self.doc.selected_edge_ids();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.loop_cut(&edges, cuts).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = mergeVertices)]
    pub fn merge_vertices(&mut self) -> Result<(), JsValue> {
        let verts = self.doc.selected_vertex_ids();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.merge_vertices(&verts, WeldMode::Center, 0.001).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = dissolveEdges)]
    pub fn dissolve_edges(&mut self) -> Result<(), JsValue> {
        let edges = self.doc.selected_edge_ids();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.dissolve_edges(&edges).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = subdivideFaces)]
    pub fn subdivide_faces(&mut self) -> Result<(), JsValue> {
        let faces = self.doc.selected_face_ids();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.subdivide_faces(&faces).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = triangulate)]
    pub fn triangulate(&mut self) -> Result<(), JsValue> {
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.triangulate().map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = mirror)]
    pub fn mirror(&mut self, axis: &str, weld_threshold: f32) -> Result<(), JsValue> {
        let mirror_axis = match axis {
            "x" => MirrorAxis::X,
            "y" => MirrorAxis::Y,
            "z" => MirrorAxis::Z,
            _ => return Err(JsValue::from_str("axis must be x, y, or z")),
        };
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.mirror(mirror_axis, weld_threshold).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = decimate)]
    pub fn decimate(&mut self, target_ratio: f32) -> Result<(), JsValue> {
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.decimate(target_ratio).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = translateSelection)]
    pub fn translate_selection(&mut self, mode: &str, ids: Vec<u32>, dx: f32, dy: f32, dz: f32) -> Result<(), JsValue> {
        self.doc.apply_selection(mode, ids);
        let delta = Vec3::new(dx, dy, dz);
        let selection_mode = self.doc.fixture.selection.mode.clone();
        let component_verts = match selection_mode.as_str() {
            "vertex" | "face" | "edge" => {
                let verts = self.doc.selection_vertex_ids().map_err(|e| JsValue::from_str(&e))?;
                if verts.is_empty() {
                    return Err(JsValue::from_str("no component vertices in selection"));
                }
                Some(verts)
            }
            _ => None,
        };
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        match selection_mode.as_str() {
            "vertex" | "face" | "edge" => {
                mesh.move_vertices(component_verts.as_ref().unwrap(), delta).map_err(|e| JsValue::from_str(&map_err(e)))?;
            }
            "mesh" | "object" => {
                mesh.translate(delta).map_err(|e| JsValue::from_str(&map_err(e)))?;
            }
            _ => {
                return Err(JsValue::from_str("unsupported selection mode for translate"));
            }
        }
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = rotateSelection)]
    pub fn rotate_selection(&mut self, mode: &str, ids: Vec<u32>, ax: f32, ay: f32, az: f32, angle: f32) -> Result<(), JsValue> {
        self.doc.apply_selection(mode, ids);
        let selection_mode = self.doc.fixture.selection.mode.clone();
        let pivot = self.doc.selection_transform_pivot().map_err(|e| JsValue::from_str(&e))?;
        let component_verts = match selection_mode.as_str() {
            "vertex" | "face" | "edge" => {
                let verts = self.doc.selection_vertex_ids().map_err(|e| JsValue::from_str(&e))?;
                if verts.is_empty() {
                    return Err(JsValue::from_str("no component vertices in selection"));
                }
                Some(verts)
            }
            _ => None,
        };
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        match selection_mode.as_str() {
            "vertex" | "face" | "edge" => {
                mesh.rotate_vertices(component_verts.as_ref().unwrap(), Vec3::new(ax, ay, az), angle, pivot)
                    .map_err(|e| JsValue::from_str(&map_err(e)))?;
            }
            "mesh" | "object" => {
                mesh.rotate(Vec3::new(ax, ay, az), angle).map_err(|e| JsValue::from_str(&map_err(e)))?;
            }
            _ => {
                return Err(JsValue::from_str("unsupported selection mode for rotate"));
            }
        }
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = scaleSelection)]
    pub fn scale_selection(&mut self, mode: &str, ids: Vec<u32>, sx: f32, sy: f32, sz: f32) -> Result<(), JsValue> {
        self.doc.apply_selection(mode, ids);
        let selection_mode = self.doc.fixture.selection.mode.clone();
        let pivot = self.doc.selection_transform_pivot().map_err(|e| JsValue::from_str(&e))?;
        let component_verts = match selection_mode.as_str() {
            "vertex" | "face" | "edge" => {
                let verts = self.doc.selection_vertex_ids().map_err(|e| JsValue::from_str(&e))?;
                if verts.is_empty() {
                    return Err(JsValue::from_str("no component vertices in selection"));
                }
                Some(verts)
            }
            _ => None,
        };
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        match selection_mode.as_str() {
            "vertex" | "face" | "edge" => {
                mesh.scale_vertices(component_verts.as_ref().unwrap(), Vec3::new(sx, sy, sz), pivot)
                    .map_err(|e| JsValue::from_str(&map_err(e)))?;
            }
            "mesh" | "object" => {
                mesh.scale(Vec3::new(sx, sy, sz)).map_err(|e| JsValue::from_str(&map_err(e)))?;
            }
            _ => {
                return Err(JsValue::from_str("unsupported selection mode for scale"));
            }
        }
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = snapToGrid)]
    pub fn snap_to_grid(&mut self, grid: f32) -> Result<(), JsValue> {
        let verts = self.doc.selected_vertex_ids();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.snap_vertices_to_grid(&verts, grid).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setSmoothShading)]
    pub fn set_smooth_shading(&mut self, smooth: bool) -> Result<(), JsValue> {
        if let Some(idx) = self.doc.active_index() {
            self.doc.fixture.objects[idx].smooth_shading = smooth;
        }
        let faces: Vec<FaceId> = (0..self.doc.active_mesh().map_err(|e| JsValue::from_str(&e))?.face_count())
            .map(|i| FaceId(i as u32))
            .collect();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.set_shading(&faces, smooth).map_err(|e| JsValue::from_str(&map_err(e)))?;
        mesh.recompute_normals().map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }
}

//#endregion WasmSession

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_fixture_has_rock_object() {
        let fixture = default_fixture();
        assert_eq!(fixture.schema, "lowpoly.fixture");
        assert_eq!(fixture.objects.len(), 1);
        assert_eq!(fixture.objects[0].name, "Rock");
    }

    #[test]
    fn document_loads_meshes() {
        let doc = LowpolyDocument::new(default_fixture()).unwrap();
        assert_eq!(doc.meshes.len(), 1);
        assert!(doc.meshes[0].face_count() > 0);
    }

    #[test]
    fn active_mesh_tessellates() {
        let doc = LowpolyDocument::new(default_fixture()).unwrap();
        let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
        assert!(!transfer.positions.is_empty());
        assert!(!transfer.indices.is_empty());
    }

    #[test]
    fn add_primitive_box() {
        let mut doc = LowpolyDocument::new(default_fixture()).unwrap();
        let id = doc.add_primitive("box").unwrap();
        assert!(doc.fixture.objects.iter().any(|o| o.id == id));
        assert_eq!(doc.meshes.len(), 2);
    }

    #[test]
    fn tessellate_all_returns_every_object() {
        let mut doc = LowpolyDocument::new(default_fixture()).unwrap();
        let _ = doc.add_primitive("box").unwrap();
        let json = doc.tessellate_all_json().unwrap();
        let items: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn fixture_json_excludes_paint_pixels() {
        let doc = LowpolyDocument::new(default_fixture()).unwrap();
        let json = serde_json::to_string(&doc.fixture).unwrap();
        assert!(json.len() < 100_000);
        assert!(!json.contains("\"pixels\""));
    }

    #[test]
    fn painted_pixels_survive_fixture_reload() {
        let mut doc = LowpolyDocument::new(default_fixture()).unwrap();
        let object_id = doc.fixture.active_object_id.clone();
        doc.paint_stroke(&object_id, 0, 0.5, 0.5, 4.0, [255, 0, 0, 255], 0.5, 1.0, false).unwrap();
        let before = doc.layer_pixels(&object_id, 0).unwrap().to_vec();
        let json = serde_json::to_string(&doc.fixture).unwrap();
        let fixture: LowpolyFixture = serde_json::from_str(&json).unwrap();
        let preserved = std::mem::take(&mut doc.paint_pixels);
        doc.replace_fixture(fixture, preserved).unwrap();
        let after = doc.layer_pixels(&object_id, 0).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn default_fixture_mesh_has_unwrapped_uvs() {
        let doc = LowpolyDocument::new(default_fixture()).unwrap();
        let transfer = doc.active_mesh().unwrap().tessellate().unwrap();
        assert!(transfer.uvs.iter().any(|uv| *uv > 0.0));
    }

    #[test]
    fn empty_paint_pixels_are_opaque_white() {
        let pixels = empty_paint_pixels();
        assert_eq!(pixels[0], 255);
        assert_eq!(pixels[1], 255);
        assert_eq!(pixels[2], 255);
        assert_eq!(pixels[3], 255);
    }

    #[test]
    fn paint_stroke_writes_pixels() {
        let mut doc = LowpolyDocument::new(default_fixture()).unwrap();
        let object_id = doc.fixture.active_object_id.clone();
        doc.paint_stroke(&object_id, 0, 0.5, 0.5, 4.0, [255, 0, 0, 255], 0.5, 1.0, false).unwrap();
        let composite = doc.composite_layers(&object_id).unwrap();
        let size = LOWPOLY_PAINT_TEXTURE_SIZE;
        let center = (size / 2 * size + size / 2) * 4;
        assert!(composite[center] > 200);
    }

    #[test]
    fn face_selection_vertex_ids_are_localized() {
        let mut doc = LowpolyDocument::new(default_fixture()).unwrap();
        doc.fixture.selection = LowpolySelection {
            targets: LowpolySelectionTargets {
                mesh: false,
                vertex: false,
                edge: false,
                face: true,
            },
            keys: vec!["lowpoly:obj-1:0:face:0".into()],
            mode: "face".into(),
            ids: vec![0],
            ..Default::default()
        };
        let verts = doc.selection_vertex_ids().unwrap();
        let total = doc.active_mesh().unwrap().vertex_count();
        assert!(!verts.is_empty());
        assert!(verts.len() < total);
    }

    #[test]
    fn face_translate_moves_only_selected_vertices() {
        let mut doc = LowpolyDocument::new(default_fixture()).unwrap();
        doc.fixture.selection = LowpolySelection {
            targets: LowpolySelectionTargets {
                mesh: false,
                vertex: false,
                edge: false,
                face: true,
            },
            keys: vec!["lowpoly:obj-1:0:face:0".into()],
            mode: "face".into(),
            ids: vec![0],
            ..Default::default()
        };
        let mesh = doc.active_mesh().unwrap();
        let face_verts = doc.selection_vertex_ids().unwrap();
        let other = (0..mesh.vertex_count())
            .map(|i| VertexId(i as u32))
            .find(|vid| !face_verts.contains(vid))
            .unwrap();
        let before_face = mesh.vertex_position(face_verts[0]).unwrap();
        let before_other = mesh.vertex_position(other).unwrap();
        drop(mesh);
        doc.active_mesh_mut()
            .unwrap()
            .move_vertices(&face_verts, Vec3::new(0.5, 0.0, 0.0))
            .unwrap();
        let mesh = doc.active_mesh().unwrap();
        let after_face = mesh.vertex_position(face_verts[0]).unwrap();
        let after_other = mesh.vertex_position(other).unwrap();
        assert!((after_face.x() - before_face.x() - 0.5).abs() < 1e-4);
        assert!((after_other.x() - before_other.x()).abs() < 1e-4);
        assert!((after_other.y() - before_other.y()).abs() < 1e-4);
        assert!((after_other.z() - before_other.z()).abs() < 1e-4);
    }
}

//#endregion Tests
