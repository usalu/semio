//! 🔷 Lowpoly core: mesh editing session wrapping kernel_3d_mesh.

use kernel_3d_mesh::{
    FaceId, HalfedgeMesh, MeshKernelError, MirrorAxis, Vec3, VertexId, WeldMode,
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
pub struct LowpolySelection {
    pub mode: String,
    pub ids: Vec<u32>,
}

impl Default for LowpolySelection {
    fn default() -> Self {
        Self {
            mode: "object".into(),
            ids: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyObject {
    pub id: String,
    pub name: String,
    pub transform: LowpolyTransform,
    pub smooth_shading: bool,
    pub mesh_json: String,
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
    let mesh_json = mesh.to_json().unwrap_or_else(|_| "{}".into());
    LowpolyFixture {
        schema: "lowpoly.fixture".into(),
        objects: vec![LowpolyObject {
            id: "obj-1".into(),
            name: "Rock".into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh_json,
        }],
        active_object_id: "obj-1".into(),
        selection: LowpolySelection {
            mode: "object".into(),
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
}

impl LowpolyDocument {
    fn new(fixture: LowpolyFixture) -> Result<Self, String> {
        let mut doc = Self {
            fixture,
            meshes: Vec::new(),
            next_object_serial: 100,
        };
        doc.reload_meshes()?;
        Ok(doc)
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

    fn add_primitive(&mut self, kind: &str) -> Result<String, String> {
        let mesh = match kind {
            "box" => HalfedgeMesh::box_prim(1.0, 1.0, 1.0),
            "plane" => HalfedgeMesh::plane_prim(2.0, 2.0),
            "cylinder" => HalfedgeMesh::cylinder_prim(0.5, 1.0, 12),
            "cone" => HalfedgeMesh::cone_prim(0.5, 1.0, 12),
            "ico_sphere" => HalfedgeMesh::ico_sphere_prim(0.5, 1),
            _ => return Err(format!("unknown primitive: {kind}")),
        }
        .map_err(|e| format!("{e:?}"))?;
        self.next_object_serial += 1;
        let id = format!("obj-{}", self.next_object_serial);
        let mesh_json = mesh.to_json().map_err(|e| format!("{e:?}"))?;
        self.fixture.objects.push(LowpolyObject {
            id: id.clone(),
            name: kind.into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh_json,
        });
        self.meshes.push(mesh);
        self.fixture.active_object_id = id.clone();
        self.fixture.selection = LowpolySelection {
            mode: "object".into(),
            ids: vec![(self.fixture.objects.len() - 1) as u32],
        };
        Ok(id)
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
        self.doc = LowpolyDocument::new(fixture).map_err(|e| JsValue::from_str(&e))?;
        Ok(())
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
        self.doc.fixture.selection = LowpolySelection {
            mode: mode.into(),
            ids,
        };
        Ok(())
    }

    #[wasm_bindgen(js_name = tessellateActive)]
    pub fn tessellate_active(&self) -> Result<String, JsValue> {
        let mesh = self.doc.active_mesh().map_err(|e| JsValue::from_str(&e))?;
        let transfer = mesh.tessellate().map_err(|e| JsValue::from_str(&map_err(e)))?;
        serde_json::to_string(&serde_json::json!({
            "positions": transfer.positions,
            "normals": transfer.normals,
            "indices": transfer.indices,
            "edgePositions": transfer.edge_positions,
        }))
        .map_err(|e| JsValue::from_str(&e.to_string()))
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
    pub fn translate_selection(&mut self, dx: f32, dy: f32, dz: f32) -> Result<(), JsValue> {
        let delta = Vec3::new(dx, dy, dz);
        let mode = self.doc.fixture.selection.mode.clone();
        let verts = self.doc.selected_vertex_ids();
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        match mode.as_str() {
            "vertex" => {
                mesh.move_vertices(&verts, delta).map_err(|e| JsValue::from_str(&map_err(e)))?;
            }
            "object" | _ => {
                mesh.translate(delta).map_err(|e| JsValue::from_str(&map_err(e)))?;
            }
        }
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = rotateSelection)]
    pub fn rotate_selection(&mut self, ax: f32, ay: f32, az: f32, angle: f32) -> Result<(), JsValue> {
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.rotate(Vec3::new(ax, ay, az), angle).map_err(|e| JsValue::from_str(&map_err(e)))?;
        self.doc.sync_meshes_to_fixture().map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = scaleSelection)]
    pub fn scale_selection(&mut self, sx: f32, sy: f32, sz: f32) -> Result<(), JsValue> {
        let mesh = self.doc.active_mesh_mut().map_err(|e| JsValue::from_str(&e))?;
        mesh.scale(Vec3::new(sx, sy, sz)).map_err(|e| JsValue::from_str(&map_err(e)))?;
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
}

//#endregion Tests
