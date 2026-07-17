//! 🥅 Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link ActionBus}.

pub mod action_bus {
// #region action_bus
//! 🎯 Action routing between renderer and app controllers.

use serde_json::Value;
use std::collections::HashMap;

pub trait ActionHandler: Send {
    fn id(&self) -> &str;
    fn handle(&mut self, action: &str, args: Option<&Value>) -> Vec<String>;
}

pub struct ActionBus {
    controllers: HashMap<String, Box<dyn ActionHandler>>,
}

impl Default for ActionBus {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionBus {
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Box<dyn ActionHandler>) {
        let id = handler.id().to_string();
        self.controllers.insert(id, handler);
    }

    pub fn unregister(&mut self, controller_id: &str) {
        self.controllers.remove(controller_id);
    }

    pub fn dispatch(&mut self, controller_id: &str, action: &str, args: Option<&Value>) -> Vec<String> {
        self.controllers
            .get_mut(controller_id)
            .map(|handler| handler.handle(action, args))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoHandler {
        id: String,
    }

    impl ActionHandler for EchoHandler {
        fn id(&self) -> &str {
            &self.id
        }

        fn handle(&mut self, action: &str, _args: Option<&Value>) -> Vec<String> {
            vec![format!("{action}:ok")]
        }
    }

    #[test]
    fn dispatches_to_registered_handler() {
        let mut bus = ActionBus::new();
        bus.register(Box::new(EchoHandler { id: "app".into() }));
        let ops = bus.dispatch("app", "ping", None);
        assert_eq!(ops, vec!["ping:ok"]);
    }
}
// #endregion action_bus
}

pub mod mesh {
// #region mesh
//! 🔺 Shared mesh geometry: primitives, compact JSON, OBJ/GLB interchange.

use serde::{Deserialize, Serialize};

//#region MeshData
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshData {
    #[serde(default)]
    pub positions: Vec<f32>,
    #[serde(default)]
    pub normals: Vec<f32>,
    #[serde(default)]
    pub colors: Vec<f32>,
    #[serde(default)]
    pub indices: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uvs: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub face_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vertex_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_positions: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_uvs: Vec<f32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_is_seam: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint_texture_base64: Option<String>,
}

impl MeshData {
    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn compute_normals(&mut self) {
        let count = self.vertex_count();
        self.normals = vec![0.0; count * 3];
        for tri in self.indices.chunks_exact(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p0 = [self.positions[i0 * 3], self.positions[i0 * 3 + 1], self.positions[i0 * 3 + 2]];
            let p1 = [self.positions[i1 * 3], self.positions[i1 * 3 + 1], self.positions[i1 * 3 + 2]];
            let p2 = [self.positions[i2 * 3], self.positions[i2 * 3 + 1], self.positions[i2 * 3 + 2]];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [
                e0[1] * e1[2] - e0[2] * e1[1],
                e0[2] * e1[0] - e0[0] * e1[2],
                e0[0] * e1[1] - e0[1] * e1[0],
            ];
            for &idx in tri {
                let i = idx as usize * 3;
                self.normals[i] += n[0];
                self.normals[i + 1] += n[1];
                self.normals[i + 2] += n[2];
            }
        }
        for chunk in self.normals.chunks_exact_mut(3) {
            let len = (chunk[0] * chunk[0] + chunk[1] * chunk[1] + chunk[2] * chunk[2]).sqrt();
            if len > 1e-8 {
                chunk[0] /= len;
                chunk[1] /= len;
                chunk[2] /= len;
            }
        }
    }

    pub fn aabb(&self) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for chunk in self.positions.chunks_exact(3) {
            for axis in 0..3 {
                min[axis] = min[axis].min(chunk[axis]);
                max[axis] = max[axis].max(chunk[axis]);
            }
        }
        (min, max)
    }

    pub fn merge(&mut self, other: &MeshData) {
        let base = self.vertex_count() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.colors.extend_from_slice(&other.colors);
        self.indices
            .extend(other.indices.iter().map(|index| index + base));
    }
}
//#endregion MeshData

//#region Primitives
fn push_triangle(mesh: &mut MeshData, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

pub fn mesh_box(width: f32, height: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hh = height * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    let faces = [
        ([-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]),
        ([hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]),
        ([-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]),
        ([-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]),
        ([hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]),
        ([-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]),
    ];
    for (a, b, c, d) in faces {
        push_triangle(&mut mesh, a, b, c);
        push_triangle(&mut mesh, a, c, d);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_plane(width: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

pub fn mesh_uv_sphere(radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::PI;
        let phi1 = v1 * std::f32::consts::PI;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = sphere_point(radius, phi0, theta0);
            let p10 = sphere_point(radius, phi0, theta1);
            let p01 = sphere_point(radius, phi1, theta0);
            let p11 = sphere_point(radius, phi1, theta1);
            if ring > 0 {
                push_triangle(&mut mesh, p00, p10, p11);
            }
            if ring + 1 < rings {
                push_triangle(&mut mesh, p00, p11, p01);
            }
        }
    }
    mesh.compute_normals();
    mesh
}

fn sphere_point(radius: f32, phi: f32, theta: f32) -> [f32; 3] {
    let sin_phi = phi.sin();
    [
        radius * sin_phi * theta.cos(),
        radius * phi.cos(),
        radius * sin_phi * theta.sin(),
    ]
}

pub fn mesh_ico_sphere(radius: f32, subdivisions: u32) -> MeshData {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        normalize3([-1.0, t, 0.0]),
        normalize3([1.0, t, 0.0]),
        normalize3([-1.0, -t, 0.0]),
        normalize3([1.0, -t, 0.0]),
        normalize3([0.0, -1.0, t]),
        normalize3([0.0, 1.0, t]),
        normalize3([0.0, -1.0, -t]),
        normalize3([0.0, 1.0, -t]),
        normalize3([t, 0.0, -1.0]),
        normalize3([t, 0.0, 1.0]),
        normalize3([-t, 0.0, -1.0]),
        normalize3([-t, 0.0, 1.0]),
    ];
    let mut faces = vec![
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = std::collections::HashMap::new();
        for face in &faces {
            let a = midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[
                [face[0], a, c],
                [face[1], b, a],
                [face[2], c, b],
                [a, b, c],
            ]);
        }
        faces = next;
    }
    let mut mesh = MeshData::default();
    for face in faces {
        let a = scale3(verts[face[0] as usize], radius);
        let b = scale3(verts[face[1] as usize], radius);
        let c = scale3(verts[face[2] as usize], radius);
        push_triangle(&mut mesh, a, b, c);
    }
    mesh.compute_normals();
    mesh
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn midpoint(
    verts: &mut Vec<[f32; 3]>,
    cache: &mut std::collections::HashMap<(u32, u32), u32>,
    a: u32,
    b: u32,
) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = normalize3([
        (verts[a as usize][0] + verts[b as usize][0]) * 0.5,
        (verts[a as usize][1] + verts[b as usize][1]) * 0.5,
        (verts[a as usize][2] + verts[b as usize][2]) * 0.5,
    ]);
    let index = verts.len() as u32;
    verts.push(mid);
    cache.insert(key, index);
    index
}

pub fn mesh_cylinder(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let half = height * 0.5;
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p00 = [radius * a0.cos(), -half, radius * a0.sin()];
        let p01 = [radius * a1.cos(), -half, radius * a1.sin()];
        let p10 = [radius * a0.cos(), half, radius * a0.sin()];
        let p11 = [radius * a1.cos(), half, radius * a1.sin()];
        push_triangle(&mut mesh, p00, p01, p11);
        push_triangle(&mut mesh, p00, p11, p10);
        push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00);
        push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_cone(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        push_triangle(&mut mesh, apex, p1, p0);
        push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_torus(major_radius: f32, minor_radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::TAU;
        let phi1 = v1 * std::f32::consts::TAU;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = torus_point(major_radius, minor_radius, phi0, theta0);
            let p10 = torus_point(major_radius, minor_radius, phi0, theta1);
            let p01 = torus_point(major_radius, minor_radius, phi1, theta0);
            let p11 = torus_point(major_radius, minor_radius, phi1, theta1);
            push_triangle(&mut mesh, p00, p10, p11);
            push_triangle(&mut mesh, p00, p11, p01);
        }
    }
    mesh.compute_normals();
    mesh
}

fn torus_point(major: f32, minor: f32, phi: f32, theta: f32) -> [f32; 3] {
    let r = major + minor * theta.cos();
    [r * phi.cos(), minor * theta.sin(), r * phi.sin()]
}

pub fn mesh_from_kind(kind: &str) -> MeshData {
    match kind {
        "vortex-marker" => mesh_ico_sphere(0.12, 1),
        "vertex-marker" => mesh_ico_sphere(1.0, 1),
        "sphere" | "uvSphere" => mesh_uv_sphere(0.5, 16, 12),
        "icoSphere" => mesh_ico_sphere(0.5, 1),
        "plane" => mesh_plane(1.0, 1.0),
        "cylinder" => mesh_cylinder(0.5, 1.0, 16),
        "cone" => mesh_cone(0.5, 1.0, 16),
        "torus" => mesh_torus(0.5, 0.15, 16, 12),
        _ => mesh_box(1.0, 1.0, 1.0),
    }
}

/** @emoji 🔩 Builds mesh data from indexed brep tessellation buffers. */
pub fn mesh_from_indexed(positions: &[f32], normals: &[f32], indices: &[u32]) -> MeshData {
    let mut mesh = MeshData {
        positions: positions.to_vec(),
        normals: normals.to_vec(),
        indices: indices.to_vec(),
        ..MeshData::default()
    };
    if mesh.normals.is_empty() && !mesh.positions.is_empty() {
        mesh.compute_normals();
    }
    mesh
}

/** @emoji 🧩 Like `mesh_from_indexed`, but also stamps `face_ids` per triangle from `(face id, triangle start, triangle count)`
 * groups — lets a picked triangle resolve back to the brep face it came from. Plain tuples (not the kernel's `FaceGroup`)
 * so this crate doesn't need to depend on the kernel engine crate; callers convert their own group type. */
pub fn mesh_from_indexed_with_face_groups(positions: &[f32], normals: &[f32], indices: &[u32], face_groups: &[(u32, u32, u32)]) -> MeshData {
    let mut mesh = mesh_from_indexed(positions, normals, indices);
    if !face_groups.is_empty() {
        let triangle_count = indices.len() / 3;
        let mut face_ids = vec![0u32; triangle_count];
        for &(face_id, start, count) in face_groups {
            let start_tri = (start / 3) as usize;
            let count_tri = (count / 3) as usize;
            for tri in start_tri..(start_tri + count_tri).min(triangle_count) {
                face_ids[tri] = face_id;
            }
        }
        mesh.face_ids = face_ids;
    }
    mesh
}
//#endregion Primitives

//#region Obj
pub fn mesh_to_obj(mesh: &MeshData, object_name: &str) -> String {
    let mut out = format!("o {object_name}\n");
    for chunk in mesh.positions.chunks_exact(3) {
        out.push_str(&format!("v {} {} {}\n", chunk[0], chunk[1], chunk[2]));
    }
    if mesh.normals.len() == mesh.positions.len() {
        for chunk in mesh.normals.chunks_exact(3) {
            out.push_str(&format!("vn {} {} {}\n", chunk[0], chunk[1], chunk[2]));
        }
    }
    for tri in mesh.indices.chunks_exact(3) {
        let a = tri[0] + 1;
        let b = tri[1] + 1;
        let c = tri[2] + 1;
        if mesh.normals.len() == mesh.positions.len() {
            out.push_str(&format!("f {a}//{a} {b}//{b} {c}//{c}\n"));
        } else {
            out.push_str(&format!("f {a} {b} {c}\n"));
        }
    }
    out
}

/// 🔤 Hand-parses OBJ text (`v`/`vn`/`f` lines) back into `MeshData`; fan-triangulates n-gon faces and falls back to computed normals when the file has no `vn` lines or a mismatched vertex/normal count. Round-trips `mesh_to_obj`'s own output losslessly; general third-party OBJ interop is unvalidated.
pub fn mesh_from_obj(text: &str) -> Result<MeshData, String> {
    let mut positions: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut vertex_count = 0usize;
    let mut normal_count = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let tag = match parts.next() {
            Some(tag) => tag,
            None => continue,
        };
        match tag {
            "v" => {
                let coords: Vec<f32> = parts.filter_map(|value| value.parse().ok()).collect();
                if coords.len() < 3 {
                    return Err("obj: malformed v line".into());
                }
                positions.extend_from_slice(&coords[..3]);
                vertex_count += 1;
            }
            "vn" => {
                let coords: Vec<f32> = parts.filter_map(|value| value.parse().ok()).collect();
                if coords.len() < 3 {
                    return Err("obj: malformed vn line".into());
                }
                normals.extend_from_slice(&coords[..3]);
                normal_count += 1;
            }
            "f" => {
                let mut face: Vec<usize> = Vec::new();
                for token in parts {
                    let raw_index = token.split('/').next().ok_or_else(|| "obj: malformed face token".to_string())?;
                    let raw: i64 = raw_index.parse().map_err(|_| "obj: malformed face index".to_string())?;
                    face.push(obj_resolve_index(raw, vertex_count)?);
                }
                if face.len() < 3 {
                    continue;
                }
                for i in 1..face.len() - 1 {
                    indices.push(face[0] as u32);
                    indices.push(face[i] as u32);
                    indices.push(face[i + 1] as u32);
                }
            }
            _ => {}
        }
    }
    let mut mesh = MeshData {
        positions,
        indices,
        ..MeshData::default()
    };
    if normal_count == vertex_count && normal_count > 0 {
        mesh.normals = normals;
    } else {
        mesh.compute_normals();
    }
    Ok(mesh)
}

fn obj_resolve_index(raw: i64, count: usize) -> Result<usize, String> {
    if raw > 0 {
        Ok((raw - 1) as usize)
    } else if raw < 0 {
        let index = count as i64 + raw;
        if index < 0 {
            Err("obj: negative vertex index out of range".into())
        } else {
            Ok(index as usize)
        }
    } else {
        Err("obj: zero vertex index".into())
    }
}
//#endregion Obj

//#region Glb
pub fn mesh_to_glb(mesh: &MeshData) -> Vec<u8> {
    let positions = f32_slice_to_bytes(&mesh.positions);
    let normals = if mesh.normals.len() == mesh.positions.len() {
        f32_slice_to_bytes(&mesh.normals)
    } else {
        let mut copy = mesh.clone();
        copy.compute_normals();
        f32_slice_to_bytes(&copy.normals)
    };
    let indices = u32_slice_to_bytes(&mesh.indices);
    let bin = [positions.as_slice(), normals.as_slice(), indices.as_slice()].concat();
    let padded_bin = pad_to_4(bin);
    let positions_len = positions.len();
    let normals_len = normals.len();
    let indices_len = indices.len();
    let positions_offset = 0usize;
    let normals_offset = positions_offset + positions_len;
    let indices_offset = normals_offset + normals_len;
    let json = format!(
        r#"{{
  "asset": {{"version": "2.0"}},
  "scene": 0,
  "scenes": [{{"nodes": [0]}}],
  "nodes": [{{"mesh": 0}}],
  "meshes": [{{
    "primitives": [{{
      "attributes": {{"POSITION": 0, "NORMAL": 1}},
      "indices": 2,
      "mode": 4
    }}]
  }}],
  "accessors": [
    {{"bufferView": 0, "componentType": 5126, "count": {}, "type": "VEC3", "min": {}, "max": {}}},
    {{"bufferView": 1, "componentType": 5126, "count": {}, "type": "VEC3"}},
    {{"bufferView": 2, "componentType": 5125, "count": {}, "type": "SCALAR"}}
  ],
  "bufferViews": [
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}}
  ],
  "buffers": [{{"byteLength": {}}}]
}}"#,
        mesh.vertex_count(),
        json_vec3_min(&mesh.positions),
        json_vec3_max(&mesh.positions),
        mesh.vertex_count(),
        mesh.indices.len(),
        positions_offset,
        positions_len,
        normals_offset,
        normals_len,
        indices_offset,
        indices_len,
        padded_bin.len()
    );
    let mut json_bytes = json.into_bytes();
    while json_bytes.len() % 4 != 0 {
        json_bytes.push(b' ');
    }
    let total_len = 12 + 8 + json_bytes.len() + 8 + padded_bin.len();
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&(2u32).to_le_bytes());
    out.extend_from_slice(&(total_len as u32).to_le_bytes());
    out.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json_bytes);
    out.extend_from_slice(&(padded_bin.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\x00");
    out.extend_from_slice(&padded_bin);
    out
}

pub fn mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    if bytes.len() < 12 || &bytes[0..4] != b"glTF" {
        return Err("invalid glb header".into());
    }
    let mut offset = 12usize;
    let mut json = None;
    let mut bin = None;
    while offset + 8 <= bytes.len() {
        let chunk_len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        let chunk_type = &bytes[offset + 4..offset + 8];
        offset += 8;
        let end = offset + chunk_len;
        if end > bytes.len() {
            break;
        }
        let chunk = &bytes[offset..end];
        if chunk_type == b"JSON" {
            json = Some(String::from_utf8_lossy(chunk).to_string());
        } else if chunk_type == b"BIN\x00" {
            bin = Some(chunk.to_vec());
        }
        offset = end;
    }
    let json = json.ok_or_else(|| "glb missing json chunk".to_string())?;
    let bin = bin.ok_or_else(|| "glb missing bin chunk".to_string())?;
    let root: serde_json::Value = serde_json::from_str(&json).map_err(|err| err.to_string())?;
    let accessors = root
        .get("accessors")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing accessors".to_string())?;
    let buffer_views = root
        .get("bufferViews")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing bufferViews".to_string())?;
    let meshes = root
        .get("meshes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "glb missing meshes".to_string())?;
    let primitive = meshes[0]
        .get("primitives")
        .and_then(|v| v.as_array())
        .and_then(|v| v.first())
        .ok_or_else(|| "glb missing primitive".to_string())?;
    let position_accessor = primitive
        .get("attributes")
        .and_then(|v| v.get("POSITION"))
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "glb missing POSITION".to_string())? as usize;
    let normal_accessor = primitive
        .get("attributes")
        .and_then(|v| v.get("NORMAL"))
        .and_then(|v| v.as_u64());
    let index_accessor = primitive
        .get("indices")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "glb missing indices".to_string())? as usize;
    let positions = read_accessor_f32_vec3(&accessors[position_accessor], &buffer_views, &bin)?;
    let normals = if let Some(index) = normal_accessor {
        read_accessor_f32_vec3(&accessors[index as usize], &buffer_views, &bin)?
    } else {
        Vec::new()
    };
    let indices = read_accessor_u32(&accessors[index_accessor], &buffer_views, &bin)?;
    let mut mesh = MeshData {
        positions,
        normals,
        colors: Vec::new(),
        indices,
        ..Default::default()
    };
    if mesh.normals.is_empty() {
        mesh.compute_normals();
    }
    Ok(mesh)
}

fn read_accessor_f32_vec3(
    accessor: &serde_json::Value,
    buffer_views: &[serde_json::Value],
    bin: &[u8],
) -> Result<Vec<f32>, String> {
    let count = accessor.get("count").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let view_index = accessor.get("bufferView").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let byte_offset = accessor
        .get("byteOffset")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let view = &buffer_views[view_index];
    let view_offset = view.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let start = view_offset + byte_offset;
    let mut out = Vec::with_capacity(count * 3);
    for index in 0..count {
        let base = start + index * 12;
        if base + 12 > bin.len() {
            break;
        }
        for axis in 0..3 {
            let value = f32::from_le_bytes(bin[base + axis * 4..base + axis * 4 + 4].try_into().unwrap());
            out.push(value);
        }
    }
    Ok(out)
}

fn read_accessor_u32(
    accessor: &serde_json::Value,
    buffer_views: &[serde_json::Value],
    bin: &[u8],
) -> Result<Vec<u32>, String> {
    let count = accessor.get("count").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let view_index = accessor.get("bufferView").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let byte_offset = accessor
        .get("byteOffset")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let view = &buffer_views[view_index];
    let view_offset = view.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let start = view_offset + byte_offset;
    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let base = start + index * 4;
        if base + 4 > bin.len() {
            break;
        }
        out.push(u32::from_le_bytes(bin[base..base + 4].try_into().unwrap()));
    }
    Ok(out)
}

fn f32_slice_to_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn u32_slice_to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while data.len() % 4 != 0 {
        data.push(0);
    }
    data
}

fn json_vec3_min(positions: &[f32]) -> String {
    let (min, _) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb();
    format!("[{}, {}, {}]", min[0], min[1], min[2])
}

fn json_vec3_max(positions: &[f32]) -> String {
    let (_, max) = MeshData {
        positions: positions.to_vec(),
        ..Default::default()
    }
    .aabb();
    format!("[{}, {}, {}]", max[0], max[1], max[2])
}
//#endregion Glb

//#region Stl
/// 🧱 Hand-rolled binary STL: 80-byte header, `u32` little-endian triangle count, then per triangle a `f32x3` facet normal, three `f32x3` vertices, and a `u16` attribute-byte-count (written as 0). No vertex dedupe, matching the binary STL convention of one independent triangle per record.
pub fn mesh_to_stl(mesh: &MeshData) -> Vec<u8> {
    let triangle_count = mesh.triangle_count() as u32;
    let mut out = Vec::with_capacity(80 + 4 + triangle_count as usize * 50);
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&triangle_count.to_le_bytes());
    for tri in mesh.indices.chunks_exact(3) {
        let p0 = stl_vertex(&mesh.positions, tri[0]);
        let p1 = stl_vertex(&mesh.positions, tri[1]);
        let p2 = stl_vertex(&mesh.positions, tri[2]);
        let normal = stl_face_normal(p0, p1, p2);
        for component in normal {
            out.extend_from_slice(&component.to_le_bytes());
        }
        for vertex in [p0, p1, p2] {
            for component in vertex {
                out.extend_from_slice(&component.to_le_bytes());
            }
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}

pub fn mesh_from_stl(bytes: &[u8]) -> Result<MeshData, String> {
    if bytes.len() < 84 {
        return Err("stl: truncated header".into());
    }
    let triangle_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let expected_len = 84 + triangle_count * 50;
    if bytes.len() < expected_len {
        return Err("stl: truncated triangle data".into());
    }
    let mut mesh = MeshData::default();
    for triangle in 0..triangle_count {
        let base = 84 + triangle * 50;
        let mut normal = [0f32; 3];
        for axis in 0..3 {
            normal[axis] = f32::from_le_bytes(bytes[base + axis * 4..base + axis * 4 + 4].try_into().unwrap());
        }
        let vertex_base = base + 12;
        for corner in 0..3 {
            let corner_base = vertex_base + corner * 12;
            let mut position = [0f32; 3];
            for axis in 0..3 {
                position[axis] = f32::from_le_bytes(bytes[corner_base + axis * 4..corner_base + axis * 4 + 4].try_into().unwrap());
            }
            let index = (mesh.positions.len() / 3) as u32;
            mesh.positions.extend_from_slice(&position);
            mesh.normals.extend_from_slice(&normal);
            mesh.indices.push(index);
        }
    }
    Ok(mesh)
}

fn stl_vertex(positions: &[f32], index: u32) -> [f32; 3] {
    let base = index as usize * 3;
    [positions[base], positions[base + 1], positions[base + 2]]
}

fn stl_face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [
        e0[1] * e1[2] - e0[2] * e1[1],
        e0[2] * e1[0] - e0[0] * e1[2],
        e0[0] * e1[1] - e0[1] * e1[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 1e-8 {
        [n[0] / len, n[1] / len, n[2] / len]
    } else {
        [0.0, 0.0, 0.0]
    }
}
//#endregion Stl

//#region MediaFormat
/// 🗂️ OS-level media export/import format. Lives here (not in `framework/product/os/core`) because `framework/core` sits below `framework/product/os/core` in the dependency graph — `os/core` depends on `framework-core`, never the reverse — so the `MeshExporter`/`MeshImporter` traits below, and every OS registration site, share one definition; `framework/product/os/core` re-exports it verbatim.
#[derive(Clone, Debug, PartialEq)]
pub enum OsMediaFormat {
    Svg,
    Png,
    Obj,
    Glb,
    Stl,
    Step,
    Dwg,
}

impl OsMediaFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Svg => "svg",
            Self::Png => "png",
            Self::Obj => "obj",
            Self::Glb => "glb",
            Self::Stl => "stl",
            Self::Step => "step",
            Self::Dwg => "dwg",
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Svg => "image/svg+xml",
            Self::Png => "image/png",
            Self::Obj => "model/obj",
            Self::Glb => "model/gltf-binary",
            Self::Stl => "model/stl",
            Self::Step => "model/step",
            Self::Dwg => "image/vnd.dwg",
        }
    }

    /// @emoji 🔢 Whether this format's payload is base64-encoded binary rather than plain text.
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Png | Self::Glb | Self::Stl | Self::Dwg)
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "svg" => Some(Self::Svg),
            "png" => Some(Self::Png),
            "obj" => Some(Self::Obj),
            "glb" => Some(Self::Glb),
            "stl" => Some(Self::Stl),
            "step" => Some(Self::Step),
            "dwg" => Some(Self::Dwg),
            _ => None,
        }
    }
}
//#endregion MediaFormat

//#region MeshCodec
/// 🔌 Format-keyed mesh export codec; concrete implementations below are zero-dependency (hand-rolled OBJ/GLB/STL). B-Rep apps additionally get `SolidExporter` (kernel/3d/brep/rs) which wraps the real kernel's STEP/STL/OBJ writers, and reuse `GlbExporter`/`GlbImporter` here via a tessellation bridge so GLB is the same codec everywhere.
pub trait MeshExporter: Send + Sync {
    fn format(&self) -> OsMediaFormat;
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String>;
}

/// 🔌 Format-keyed mesh import codec; see `MeshExporter`.
pub trait MeshImporter: Send + Sync {
    fn format(&self) -> OsMediaFormat;
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String>;
}

pub struct ObjExporter;
impl MeshExporter for ObjExporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Obj
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_obj(mesh, "mesh").into_bytes())
    }
}

pub struct ObjImporter;
impl MeshImporter for ObjImporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Obj
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        mesh_from_obj(text)
    }
}

pub struct GlbExporter;
impl MeshExporter for GlbExporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Glb
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_glb(mesh))
    }
}

pub struct GlbImporter;
impl MeshImporter for GlbImporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Glb
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_glb(bytes)
    }
}

pub struct StlExporter;
impl MeshExporter for StlExporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Stl
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_stl(mesh))
    }
}

pub struct StlImporter;
impl MeshImporter for StlImporter {
    fn format(&self) -> OsMediaFormat {
        OsMediaFormat::Stl
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_stl(bytes)
    }
}
//#endregion MeshCodec

//#region Dwg
/// 📐 Hand-rolled DWG codec: a self-contained, round-trippable binary interchange format using the AC1015 (R2000) file magic and an R2000-flavored section-locator/CRC/handle container (bit primitives BS/BL/BD/handle refs per https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf). Entity/header field layouts are a semio-defined subset chosen for lossless round-tripping through this codec; byte-exact third-party AutoCAD/ODA interop needs follow-up validation against a real DWG viewer.

//#region DwgTypes
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DwgDrawing {
    pub layers: Vec<DwgLayer>,
    pub entities: Vec<DwgEntity>,
    pub extmin: [f64; 3],
    pub extmax: [f64; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct DwgLayer {
    pub name: String,
    pub color: u8,
}

impl Default for DwgLayer {
    fn default() -> Self {
        Self { name: "0".to_string(), color: 7 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DwgColor {
    ByLayer,
    ByBlock,
    Index(u8),
}

impl DwgColor {
    fn to_bs(self) -> u16 {
        match self {
            DwgColor::ByLayer => 256,
            DwgColor::ByBlock => 0,
            DwgColor::Index(index) => index as u16,
        }
    }

    fn from_bs(value: u16) -> Self {
        match value {
            256 => DwgColor::ByLayer,
            0 => DwgColor::ByBlock,
            other => DwgColor::Index(other as u8),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DwgEntity {
    pub layer: usize,
    pub color: DwgColor,
    pub geometry: DwgGeometry,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DwgGeometry {
    Line { start: [f64; 3], end: [f64; 3] },
    Point { at: [f64; 3] },
    Circle { center: [f64; 3], radius: f64, normal: [f64; 3] },
    Arc { center: [f64; 3], radius: f64, start_angle: f64, end_angle: f64, normal: [f64; 3] },
    Ellipse { center: [f64; 3], major_axis: [f64; 3], ratio: f64, start_param: f64, end_param: f64, normal: [f64; 3] },
    LwPolyline { closed: bool, elevation: f64, vertices: Vec<[f64; 2]>, bulges: Vec<f64> },
    Spline { degree: u32, control_points: Vec<[f64; 3]>, knots: Vec<f64>, weights: Vec<f64> },
    Text { at: [f64; 3], height: f64, rotation: f64, content: String },
    Face3d { corners: [[f64; 3]; 4] },
    Polyline3d { closed: bool, vertices: Vec<[f64; 3]> },
    PolyfaceMesh { vertices: Vec<[f64; 3]>, faces: Vec<[i32; 4]> },
}

impl DwgDrawing {
    pub fn ensure_layer(&mut self, name: &str) -> usize {
        if let Some(index) = self.layers.iter().position(|layer| layer.name == name) {
            return index;
        }
        self.layers.push(DwgLayer { name: name.to_string(), color: 7 });
        self.layers.len() - 1
    }

    fn recompute_extents(&mut self) {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        let touch = |p: [f64; 3], min: &mut [f64; 3], max: &mut [f64; 3]| {
            for axis in 0..3 {
                min[axis] = min[axis].min(p[axis]);
                max[axis] = max[axis].max(p[axis]);
            }
        };
        for entity in &self.entities {
            match &entity.geometry {
                DwgGeometry::Line { start, end } => {
                    touch(*start, &mut min, &mut max);
                    touch(*end, &mut min, &mut max);
                }
                DwgGeometry::Point { at } => touch(*at, &mut min, &mut max),
                DwgGeometry::Circle { center, radius, .. } | DwgGeometry::Arc { center, radius, .. } => {
                    touch([center[0] - radius, center[1] - radius, center[2]], &mut min, &mut max);
                    touch([center[0] + radius, center[1] + radius, center[2]], &mut min, &mut max);
                }
                DwgGeometry::Ellipse { center, major_axis, .. } => {
                    let r = (major_axis[0] * major_axis[0] + major_axis[1] * major_axis[1]).sqrt();
                    touch([center[0] - r, center[1] - r, center[2]], &mut min, &mut max);
                    touch([center[0] + r, center[1] + r, center[2]], &mut min, &mut max);
                }
                DwgGeometry::LwPolyline { vertices, elevation, .. } => {
                    for v in vertices {
                        touch([v[0], v[1], *elevation], &mut min, &mut max);
                    }
                }
                DwgGeometry::Spline { control_points, .. } | DwgGeometry::Polyline3d { vertices: control_points, .. } => {
                    for p in control_points {
                        touch(*p, &mut min, &mut max);
                    }
                }
                DwgGeometry::PolyfaceMesh { vertices, .. } => {
                    for p in vertices {
                        touch(*p, &mut min, &mut max);
                    }
                }
                DwgGeometry::Text { at, .. } => touch(*at, &mut min, &mut max),
                DwgGeometry::Face3d { corners } => {
                    for p in corners {
                        touch(*p, &mut min, &mut max);
                    }
                }
            }
        }
        if min[0].is_finite() {
            self.extmin = min;
            self.extmax = max;
        }
    }
}
//#endregion DwgTypes

//#region DwgBits
struct DwgBitWriter {
    bytes: Vec<u8>,
    bit: u8,
}

impl DwgBitWriter {
    fn new() -> Self {
        Self { bytes: Vec::new(), bit: 0 }
    }

    fn write_bit(&mut self, value: bool) {
        if self.bit == 0 {
            self.bytes.push(0);
        }
        if value {
            let last = self.bytes.len() - 1;
            self.bytes[last] |= 1 << (7 - self.bit);
        }
        self.bit = (self.bit + 1) % 8;
    }

    fn write_bits(&mut self, value: u64, count: u8) {
        for i in (0..count).rev() {
            self.write_bit((value >> i) & 1 != 0);
        }
    }

    fn write_b(&mut self, value: bool) {
        self.write_bit(value);
    }

    fn write_bb(&mut self, value: u8) {
        self.write_bits(value as u64, 2);
    }

    fn write_rc(&mut self, value: u8) {
        self.write_bits(value as u64, 8);
    }

    fn write_rs(&mut self, value: u16) {
        self.write_rc((value & 0xFF) as u8);
        self.write_rc((value >> 8) as u8);
    }

    fn write_rl(&mut self, value: u32) {
        self.write_rs((value & 0xFFFF) as u16);
        self.write_rs((value >> 16) as u16);
    }

    fn write_rd(&mut self, value: f64) {
        let bits = value.to_bits();
        self.write_rl((bits & 0xFFFF_FFFF) as u32);
        self.write_rl((bits >> 32) as u32);
    }

    fn write_bs(&mut self, value: u16) {
        match value {
            0 => self.write_bb(2),
            256 => self.write_bb(3),
            v if v <= 0xFF => {
                self.write_bb(1);
                self.write_rc(v as u8);
            }
            v => {
                self.write_bb(0);
                self.write_rs(v);
            }
        }
    }

    fn write_bl(&mut self, value: u32) {
        match value {
            0 => self.write_bb(2),
            v if v <= 0xFF => {
                self.write_bb(1);
                self.write_rc(v as u8);
            }
            v => {
                self.write_bb(0);
                self.write_rl(v);
            }
        }
    }

    fn write_bd(&mut self, value: f64) {
        if value == 0.0 {
            self.write_bb(2);
        } else if value == 1.0 {
            self.write_bb(1);
        } else {
            self.write_bb(0);
            self.write_rd(value);
        }
    }

    fn write_2rd(&mut self, v: [f64; 2]) {
        self.write_rd(v[0]);
        self.write_rd(v[1]);
    }

    fn write_3bd(&mut self, v: [f64; 3]) {
        self.write_bd(v[0]);
        self.write_bd(v[1]);
        self.write_bd(v[2]);
    }

    fn write_3rd(&mut self, v: [f64; 3]) {
        self.write_rd(v[0]);
        self.write_rd(v[1]);
        self.write_rd(v[2]);
    }

    fn write_be(&mut self, normal: [f64; 3]) {
        if normal == [0.0, 0.0, 1.0] {
            self.write_b(true);
        } else {
            self.write_b(false);
            self.write_3bd(normal);
        }
    }

    fn write_t(&mut self, text: &str) {
        let bytes = text.as_bytes();
        let len = bytes.len().min(0xFFFF);
        self.write_rs(len as u16);
        for &b in &bytes[..len] {
            self.write_rc(b);
        }
    }

    fn write_ms(&mut self, mut value: u32) {
        loop {
            let mut chunk = (value & 0x7FFF) as u16;
            value >>= 15;
            if value != 0 {
                chunk |= 0x8000;
                self.write_rs(chunk);
            } else {
                self.write_rs(chunk);
                break;
            }
        }
    }

    fn write_handle(&mut self, code: u8, handle: u64) {
        let mut bytes = Vec::new();
        let mut v = handle;
        while v != 0 {
            bytes.insert(0, (v & 0xFF) as u8);
            v >>= 8;
        }
        self.write_rc((code << 4) | bytes.len() as u8);
        for b in bytes {
            self.write_rc(b);
        }
    }

    fn pad_to_byte(&mut self) {
        while self.bit != 0 {
            self.write_bit(false);
        }
    }

    fn bit_len(&self) -> usize {
        self.bytes.len() * 8 - if self.bit == 0 { 0 } else { 8 - self.bit as usize }
    }
}

struct DwgBitReader<'a> {
    bytes: &'a [u8],
    byte_pos: usize,
    bit: u8,
}

impl<'a> DwgBitReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, byte_pos: 0, bit: 0 }
    }

    fn read_bit(&mut self) -> Result<bool, String> {
        if self.byte_pos >= self.bytes.len() {
            return Err("dwg bitstream underflow".to_string());
        }
        let value = (self.bytes[self.byte_pos] >> (7 - self.bit)) & 1 != 0;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.byte_pos += 1;
        }
        Ok(value)
    }

    fn read_bits(&mut self, count: u8) -> Result<u64, String> {
        let mut value = 0u64;
        for _ in 0..count {
            value = (value << 1) | self.read_bit()? as u64;
        }
        Ok(value)
    }

    fn read_b(&mut self) -> Result<bool, String> {
        self.read_bit()
    }

    fn read_bb(&mut self) -> Result<u8, String> {
        Ok(self.read_bits(2)? as u8)
    }

    fn read_rc(&mut self) -> Result<u8, String> {
        Ok(self.read_bits(8)? as u8)
    }

    fn read_rs(&mut self) -> Result<u16, String> {
        let lo = self.read_rc()? as u16;
        let hi = self.read_rc()? as u16;
        Ok(lo | (hi << 8))
    }

    fn read_rl(&mut self) -> Result<u32, String> {
        let lo = self.read_rs()? as u32;
        let hi = self.read_rs()? as u32;
        Ok(lo | (hi << 16))
    }

    fn read_rd(&mut self) -> Result<f64, String> {
        let lo = self.read_rl()? as u64;
        let hi = self.read_rl()? as u64;
        Ok(f64::from_bits(lo | (hi << 32)))
    }

    fn read_bs(&mut self) -> Result<u16, String> {
        match self.read_bb()? {
            0 => self.read_rs(),
            1 => Ok(self.read_rc()? as u16),
            2 => Ok(0),
            _ => Ok(256),
        }
    }

    fn read_bl(&mut self) -> Result<u32, String> {
        match self.read_bb()? {
            0 => self.read_rl(),
            1 => Ok(self.read_rc()? as u32),
            2 => Ok(0),
            _ => Err("invalid BL flag".to_string()),
        }
    }

    fn read_bd(&mut self) -> Result<f64, String> {
        match self.read_bb()? {
            0 => self.read_rd(),
            1 => Ok(1.0),
            2 => Ok(0.0),
            _ => Err("invalid BD flag".to_string()),
        }
    }

    fn read_2rd(&mut self) -> Result<[f64; 2], String> {
        Ok([self.read_rd()?, self.read_rd()?])
    }

    fn read_3bd(&mut self) -> Result<[f64; 3], String> {
        Ok([self.read_bd()?, self.read_bd()?, self.read_bd()?])
    }

    fn read_be(&mut self) -> Result<[f64; 3], String> {
        if self.read_b()? {
            Ok([0.0, 0.0, 1.0])
        } else {
            self.read_3bd()
        }
    }

    fn read_t(&mut self) -> Result<String, String> {
        let len = self.read_rs()? as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.read_rc()?);
        }
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    fn read_ms(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        let mut shift = 0;
        loop {
            let chunk = self.read_rs()?;
            value |= ((chunk & 0x7FFF) as u32) << shift;
            shift += 15;
            if chunk & 0x8000 == 0 {
                break;
            }
        }
        Ok(value)
    }

    fn read_handle(&mut self) -> Result<(u8, u64), String> {
        let head = self.read_rc()?;
        let code = head >> 4;
        let len = head & 0x0F;
        let mut value = 0u64;
        for _ in 0..len {
            value = (value << 8) | self.read_rc()? as u64;
        }
        Ok((code, value))
    }

    fn pad_to_byte(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.byte_pos += 1;
        }
    }
}

fn dwg_crc16(seed: u16, data: &[u8]) -> u16 {
    let mut crc = seed;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}
//#endregion DwgBits

//#region DwgObjects
const DWG_TYPE_LAYER: u16 = 51;
const DWG_TYPE_LINE: u16 = 19;
const DWG_TYPE_POINT: u16 = 27;
const DWG_TYPE_CIRCLE: u16 = 18;
const DWG_TYPE_ARC: u16 = 17;
const DWG_TYPE_ELLIPSE: u16 = 35;
const DWG_TYPE_LWPOLYLINE: u16 = 77;
const DWG_TYPE_SPLINE: u16 = 36;
const DWG_TYPE_TEXT: u16 = 1;
const DWG_TYPE_FACE3D: u16 = 28;
const DWG_TYPE_POLYLINE3D: u16 = 16;
const DWG_TYPE_POLYLINE_PFACE: u16 = 29;

const HANDLE_MODEL_SPACE: u64 = 0x10;
const HANDLE_LAYER_BASE: u64 = 0x20;
const HANDLE_ENTITY_BASE: u64 = 0x1000;

fn dwg_write_object(out: &mut Vec<u8>, object_type: u16, handle: u64, body: &mut DwgBitWriter, handles: &mut DwgBitWriter) {
    let bitsize = body.bit_len() as u32;
    body.pad_to_byte();
    handles.pad_to_byte();

    let mut framed = DwgBitWriter::new();
    framed.write_bs(object_type);
    framed.write_rl(bitsize);
    framed.write_handle(0, handle);
    framed.pad_to_byte();
    for byte in &body.bytes {
        framed.bytes.push(*byte);
    }
    for byte in &handles.bytes {
        framed.bytes.push(*byte);
    }

    let payload = framed.bytes;
    let mut sized = DwgBitWriter::new();
    sized.write_ms(payload.len() as u32);
    sized.pad_to_byte();

    out.extend_from_slice(&sized.bytes);
    out.extend_from_slice(&payload);
    let crc = dwg_crc16(0xC0C1, &payload);
    out.extend_from_slice(&crc.to_le_bytes());
}

fn dwg_encode_entity_common(body: &mut DwgBitWriter, handles: &mut DwgBitWriter, layer_handle: u64, color: DwgColor) {
    body.write_bb(0);
    body.write_bl(0);
    body.write_b(true);
    body.write_bs(color.to_bs());
    body.write_bd(1.0);
    body.write_bb(0);
    body.write_bb(0);
    body.write_bs(0);
    body.write_rc(29);

    handles.write_handle(3, HANDLE_MODEL_SPACE);
    handles.write_handle(5, layer_handle);
}

fn dwg_decode_entity_common(reader: &mut DwgBitReader) -> Result<DwgColor, String> {
    let _entmode = reader.read_bb()?;
    let _numreactors = reader.read_bl()?;
    let _nolinks = reader.read_b()?;
    let color = DwgColor::from_bs(reader.read_bs()?);
    let _ltype_scale = reader.read_bd()?;
    let _ltype_flags = reader.read_bb()?;
    let _plotstyle_flags = reader.read_bb()?;
    let _invisibility = reader.read_bs()?;
    let _lineweight = reader.read_rc()?;
    Ok(color)
}

fn dwg_decode_entity_handles(reader: &mut DwgBitReader) -> Result<u64, String> {
    reader.pad_to_byte();
    let (_owner_code, _owner) = reader.read_handle()?;
    let (_layer_code, layer_handle) = reader.read_handle()?;
    Ok(layer_handle)
}

fn dwg_encode_entity(objects_bytes: &mut Vec<u8>, object_map: &mut Vec<(u64, usize)>, next_handle: &mut u64, layer_handle: u64, entity: &DwgEntity) {
    let handle = *next_handle;
    *next_handle += 1;
    let mut body = DwgBitWriter::new();
    let mut handles = DwgBitWriter::new();
    dwg_encode_entity_common(&mut body, &mut handles, layer_handle, entity.color);

    let object_type = match &entity.geometry {
        DwgGeometry::Line { start, end } => {
            body.write_3bd(*start);
            body.write_3bd(*end);
            DWG_TYPE_LINE
        }
        DwgGeometry::Point { at } => {
            body.write_3bd(*at);
            DWG_TYPE_POINT
        }
        DwgGeometry::Circle { center, radius, normal } => {
            body.write_3bd(*center);
            body.write_bd(*radius);
            body.write_be(*normal);
            DWG_TYPE_CIRCLE
        }
        DwgGeometry::Arc { center, radius, start_angle, end_angle, normal } => {
            body.write_3bd(*center);
            body.write_bd(*radius);
            body.write_bd(*start_angle);
            body.write_bd(*end_angle);
            body.write_be(*normal);
            DWG_TYPE_ARC
        }
        DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal } => {
            body.write_3bd(*center);
            body.write_3bd(*major_axis);
            body.write_be(*normal);
            body.write_bd(*ratio);
            body.write_bd(*start_param);
            body.write_bd(*end_param);
            DWG_TYPE_ELLIPSE
        }
        DwgGeometry::Text { at, height, rotation, content } => {
            body.write_3bd(*at);
            body.write_bd(*height);
            body.write_bd(*rotation);
            body.write_t(content);
            DWG_TYPE_TEXT
        }
        DwgGeometry::Face3d { corners } => {
            for corner in corners {
                body.write_3bd(*corner);
            }
            DWG_TYPE_FACE3D
        }
        DwgGeometry::LwPolyline { closed, elevation, vertices, bulges } => {
            body.write_b(*closed);
            body.write_bd(*elevation);
            body.write_bl(vertices.len() as u32);
            for (i, v) in vertices.iter().enumerate() {
                body.write_2rd(*v);
                body.write_bd(bulges.get(i).copied().unwrap_or(0.0));
            }
            DWG_TYPE_LWPOLYLINE
        }
        DwgGeometry::Spline { degree, control_points, knots, weights } => {
            body.write_bl(*degree);
            body.write_bl(control_points.len() as u32);
            for p in control_points {
                body.write_3bd(*p);
            }
            body.write_bl(knots.len() as u32);
            for k in knots {
                body.write_rd(*k);
            }
            body.write_bl(weights.len() as u32);
            for w in weights {
                body.write_rd(*w);
            }
            DWG_TYPE_SPLINE
        }
        DwgGeometry::Polyline3d { closed, vertices } => {
            body.write_b(*closed);
            body.write_bl(vertices.len() as u32);
            for v in vertices {
                body.write_3bd(*v);
            }
            DWG_TYPE_POLYLINE3D
        }
        DwgGeometry::PolyfaceMesh { vertices, faces } => {
            body.write_bl(vertices.len() as u32);
            for v in vertices {
                body.write_3bd(*v);
            }
            body.write_bl(faces.len() as u32);
            for face in faces {
                for idx in face {
                    body.write_bl(idx.unsigned_abs());
                    body.write_b(*idx < 0);
                }
            }
            DWG_TYPE_POLYLINE_PFACE
        }
    };

    let offset = objects_bytes.len();
    dwg_write_object(objects_bytes, object_type, handle, &mut body, &mut handles);
    object_map.push((handle, offset));
}

fn dwg_decode_entity(object_type: u16, reader: &mut DwgBitReader) -> Result<Option<(u64, DwgColor, DwgGeometry)>, String> {
    match object_type {
        DWG_TYPE_LINE => {
            let color = dwg_decode_entity_common(reader)?;
            let start = reader.read_3bd()?;
            let end = reader.read_3bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Line { start, end })))
        }
        DWG_TYPE_POINT => {
            let color = dwg_decode_entity_common(reader)?;
            let at = reader.read_3bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Point { at })))
        }
        DWG_TYPE_CIRCLE => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let radius = reader.read_bd()?;
            let normal = reader.read_be()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Circle { center, radius, normal })))
        }
        DWG_TYPE_ARC => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let radius = reader.read_bd()?;
            let start_angle = reader.read_bd()?;
            let end_angle = reader.read_bd()?;
            let normal = reader.read_be()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Arc { center, radius, start_angle, end_angle, normal })))
        }
        DWG_TYPE_ELLIPSE => {
            let color = dwg_decode_entity_common(reader)?;
            let center = reader.read_3bd()?;
            let major_axis = reader.read_3bd()?;
            let normal = reader.read_be()?;
            let ratio = reader.read_bd()?;
            let start_param = reader.read_bd()?;
            let end_param = reader.read_bd()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal })))
        }
        DWG_TYPE_TEXT => {
            let color = dwg_decode_entity_common(reader)?;
            let at = reader.read_3bd()?;
            let height = reader.read_bd()?;
            let rotation = reader.read_bd()?;
            let content = reader.read_t()?;
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Text { at, height, rotation, content })))
        }
        DWG_TYPE_FACE3D => {
            let color = dwg_decode_entity_common(reader)?;
            let corners = [reader.read_3bd()?, reader.read_3bd()?, reader.read_3bd()?, reader.read_3bd()?];
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Face3d { corners })))
        }
        DWG_TYPE_LWPOLYLINE => {
            let color = dwg_decode_entity_common(reader)?;
            let closed = reader.read_b()?;
            let elevation = reader.read_bd()?;
            let count = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(count);
            let mut bulges = Vec::with_capacity(count);
            for _ in 0..count {
                vertices.push(reader.read_2rd()?);
                bulges.push(reader.read_bd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::LwPolyline { closed, elevation, vertices, bulges })))
        }
        DWG_TYPE_SPLINE => {
            let color = dwg_decode_entity_common(reader)?;
            let degree = reader.read_bl()?;
            let cp_count = reader.read_bl()? as usize;
            let mut control_points = Vec::with_capacity(cp_count);
            for _ in 0..cp_count {
                control_points.push(reader.read_3bd()?);
            }
            let knot_count = reader.read_bl()? as usize;
            let mut knots = Vec::with_capacity(knot_count);
            for _ in 0..knot_count {
                knots.push(reader.read_rd()?);
            }
            let weight_count = reader.read_bl()? as usize;
            let mut weights = Vec::with_capacity(weight_count);
            for _ in 0..weight_count {
                weights.push(reader.read_rd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Spline { degree, control_points, knots, weights })))
        }
        DWG_TYPE_POLYLINE3D => {
            let color = dwg_decode_entity_common(reader)?;
            let closed = reader.read_b()?;
            let count = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(count);
            for _ in 0..count {
                vertices.push(reader.read_3bd()?);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::Polyline3d { closed, vertices })))
        }
        DWG_TYPE_POLYLINE_PFACE => {
            let color = dwg_decode_entity_common(reader)?;
            let vcount = reader.read_bl()? as usize;
            let mut vertices = Vec::with_capacity(vcount);
            for _ in 0..vcount {
                vertices.push(reader.read_3bd()?);
            }
            let fcount = reader.read_bl()? as usize;
            let mut faces = Vec::with_capacity(fcount);
            for _ in 0..fcount {
                let mut face = [0i32; 4];
                for slot in face.iter_mut() {
                    let magnitude = reader.read_bl()? as i32;
                    let negative = reader.read_b()?;
                    *slot = if negative { -magnitude } else { magnitude };
                }
                faces.push(face);
            }
            let layer_handle = dwg_decode_entity_handles(reader)?;
            Ok(Some((layer_handle, color, DwgGeometry::PolyfaceMesh { vertices, faces })))
        }
        _ => Ok(None),
    }
}
//#endregion DwgObjects

//#region DwgWrite
const DWG_FILE_HEADER_LEN: usize = 55;
const DWG_SENTINEL_HEADER_VARS_BEGIN: [u8; 16] = [0xCF, 0x7B, 0x1F, 0x23, 0xFD, 0xDE, 0x38, 0xA9, 0x5F, 0x7C, 0x68, 0xB8, 0x4E, 0x6D, 0x33, 0x5F];
const DWG_SENTINEL_HEADER_VARS_END: [u8; 16] = [0x30, 0x84, 0xE0, 0xDC, 0x02, 0x21, 0xC7, 0x56, 0xA0, 0x83, 0x97, 0x47, 0xB1, 0x92, 0xCC, 0xA0];
const DWG_SENTINEL_CLASSES_BEGIN: [u8; 16] = [0x8D, 0xA1, 0xC4, 0xB8, 0xC4, 0xA9, 0xF8, 0xC5, 0xC0, 0xDC, 0xF4, 0x5F, 0xE7, 0xCF, 0xB6, 0x8A];
const DWG_SENTINEL_CLASSES_END: [u8; 16] = [0x72, 0x5E, 0x3B, 0x47, 0x3B, 0x56, 0x07, 0x3A, 0x3F, 0x23, 0x0B, 0xA0, 0x18, 0x30, 0x49, 0x75];
const DWG_SENTINEL_FILE_HEADER_END: [u8; 16] = [0x95, 0xA0, 0x4E, 0x28, 0x99, 0x82, 0x1A, 0xE5, 0x5E, 0x41, 0xE0, 0x5F, 0x9D, 0x3A, 0x4D, 0x00];

/// 📐 Serializes a drawing to a semio DWG (AC1015-flavored) byte stream.
pub fn dwg_to_bytes(drawing: &DwgDrawing) -> Result<Vec<u8>, String> {
    let mut drawing = drawing.clone();
    if drawing.layers.is_empty() {
        drawing.layers.push(DwgLayer::default());
    }
    drawing.recompute_extents();

    let layer_handles: Vec<u64> = (0..drawing.layers.len()).map(|i| HANDLE_LAYER_BASE + i as u64).collect();
    let mut objects_bytes = Vec::new();
    let mut object_map: Vec<(u64, usize)> = Vec::new();

    for (i, layer) in drawing.layers.iter().enumerate() {
        let handle = layer_handles[i];
        let mut body = DwgBitWriter::new();
        body.write_t(&layer.name);
        body.write_rc(layer.color);
        let mut handles = DwgBitWriter::new();
        let offset = objects_bytes.len();
        dwg_write_object(&mut objects_bytes, DWG_TYPE_LAYER, handle, &mut body, &mut handles);
        object_map.push((handle, offset));
    }

    let mut next_handle = HANDLE_ENTITY_BASE;
    for entity in &drawing.entities {
        let layer_handle = layer_handles.get(entity.layer).copied().unwrap_or(layer_handles[0]);
        dwg_encode_entity(&mut objects_bytes, &mut object_map, &mut next_handle, layer_handle, entity);
    }

    let mut header_body = DwgBitWriter::new();
    header_body.write_3rd(drawing.extmin);
    header_body.write_3rd(drawing.extmax);
    header_body.write_handle(0, next_handle);
    header_body.pad_to_byte();
    let header_payload = header_body.bytes;
    let header_crc = dwg_crc16(0xC0C1, &header_payload);

    let mut header_section = Vec::new();
    header_section.extend_from_slice(&DWG_SENTINEL_HEADER_VARS_BEGIN);
    header_section.extend_from_slice(&(header_payload.len() as u32).to_le_bytes());
    header_section.extend_from_slice(&header_payload);
    header_section.extend_from_slice(&header_crc.to_le_bytes());
    header_section.extend_from_slice(&DWG_SENTINEL_HEADER_VARS_END);

    let mut classes_section = Vec::new();
    classes_section.extend_from_slice(&DWG_SENTINEL_CLASSES_BEGIN);
    classes_section.extend_from_slice(&0u32.to_le_bytes());
    classes_section.extend_from_slice(&dwg_crc16(0xC0C1, &[]).to_le_bytes());
    classes_section.extend_from_slice(&DWG_SENTINEL_CLASSES_END);

    let header_vars_offset = DWG_FILE_HEADER_LEN;
    let classes_offset = header_vars_offset + header_section.len();
    let objects_offset = classes_offset + classes_section.len();
    let object_map_offset = objects_offset + objects_bytes.len();

    let mut map_section = Vec::new();
    map_section.extend_from_slice(&(object_map.len() as u32).to_le_bytes());
    for (handle, local_offset) in &object_map {
        map_section.extend_from_slice(&handle.to_le_bytes());
        map_section.extend_from_slice(&((objects_offset + local_offset) as u64).to_le_bytes());
    }
    let map_crc = dwg_crc16(0xC0C1, &map_section);
    map_section.extend_from_slice(&map_crc.to_le_bytes());

    let mut file_header = Vec::new();
    file_header.extend_from_slice(b"AC1015");
    file_header.extend_from_slice(&3u32.to_le_bytes());
    let locators: [(u8, u32, u32); 3] = [
        (0, header_vars_offset as u32, header_section.len() as u32),
        (1, classes_offset as u32, classes_section.len() as u32),
        (2, object_map_offset as u32, map_section.len() as u32),
    ];
    for (num, seeker, size) in locators {
        file_header.push(num);
        file_header.extend_from_slice(&seeker.to_le_bytes());
        file_header.extend_from_slice(&size.to_le_bytes());
    }
    let locator_crc = dwg_crc16(0, &file_header) ^ 0x8461;
    file_header.extend_from_slice(&locator_crc.to_le_bytes());
    file_header.extend_from_slice(&DWG_SENTINEL_FILE_HEADER_END);
    debug_assert_eq!(file_header.len(), DWG_FILE_HEADER_LEN);

    let mut out = Vec::with_capacity(object_map_offset + map_section.len());
    out.extend_from_slice(&file_header);
    out.extend_from_slice(&header_section);
    out.extend_from_slice(&classes_section);
    out.extend_from_slice(&objects_bytes);
    out.extend_from_slice(&map_section);
    Ok(out)
}
//#endregion DwgWrite

//#region DwgRead
/// 📐 Parses a semio DWG (AC1015-flavored) byte stream, tolerating and skipping unrecognized or malformed objects.
pub fn dwg_from_bytes(bytes: &[u8]) -> Result<DwgDrawing, String> {
    if bytes.len() < 6 || &bytes[0..6] != b"AC1015" {
        let found = String::from_utf8_lossy(bytes.get(0..6).unwrap_or(b"??????")).to_string();
        return Err(format!("unsupported dwg version '{found}': only AC1015 (R2000) is supported"));
    }
    if bytes.len() < DWG_FILE_HEADER_LEN {
        return Err("dwg file header truncated".to_string());
    }
    let section_count = u32::from_le_bytes(bytes[6..10].try_into().unwrap()) as usize;
    let mut cursor = 10usize;
    let mut locators: Vec<(u8, usize, usize)> = Vec::new();
    for _ in 0..section_count.min(16) {
        if cursor + 9 > bytes.len() {
            return Err("dwg section locator truncated".to_string());
        }
        let num = bytes[cursor];
        let seeker = u32::from_le_bytes(bytes[cursor + 1..cursor + 5].try_into().unwrap()) as usize;
        let size = u32::from_le_bytes(bytes[cursor + 5..cursor + 9].try_into().unwrap()) as usize;
        locators.push((num, seeker, size));
        cursor += 9;
    }

    let (_, map_offset, map_size) = *locators
        .iter()
        .find(|(num, _, _)| *num == 2)
        .ok_or_else(|| "dwg missing object map locator".to_string())?;
    if map_offset + map_size > bytes.len() || map_size < 4 {
        return Err("dwg object map out of bounds".to_string());
    }
    let map_bytes = &bytes[map_offset..map_offset + map_size];
    let count = u32::from_le_bytes(map_bytes[0..4].try_into().unwrap()) as usize;
    let mut entries = Vec::with_capacity(count);
    let mut pos = 4usize;
    for _ in 0..count {
        if pos + 16 > map_bytes.len() {
            break;
        }
        let handle = u64::from_le_bytes(map_bytes[pos..pos + 8].try_into().unwrap());
        let address = u64::from_le_bytes(map_bytes[pos + 8..pos + 16].try_into().unwrap()) as usize;
        entries.push((handle, address));
        pos += 16;
    }

    let mut layers: Vec<DwgLayer> = Vec::new();
    let mut layer_handle_index: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut pending_entities: Vec<(u64, DwgColor, DwgGeometry)> = Vec::new();

    for (handle, address) in &entries {
        if *address >= bytes.len() {
            continue;
        }
        let mut sizer = DwgBitReader::new(&bytes[*address..]);
        let payload_len = match sizer.read_ms() {
            Ok(v) => v as usize,
            Err(_) => continue,
        };
        sizer.pad_to_byte();
        let payload_start = address + sizer.byte_pos;
        if payload_start + payload_len > bytes.len() {
            continue;
        }
        let payload = &bytes[payload_start..payload_start + payload_len];
        let mut reader = DwgBitReader::new(payload);
        let object_type = match reader.read_bs() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let _bitsize = match reader.read_rl() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if reader.read_handle().is_err() {
            continue;
        }
        reader.pad_to_byte();

        if object_type == DWG_TYPE_LAYER {
            if let (Ok(name), Ok(color)) = (reader.read_t(), reader.read_rc()) {
                layer_handle_index.insert(*handle, layers.len());
                layers.push(DwgLayer { name, color });
            }
            continue;
        }

        if let Ok(Some((layer_handle, color, geometry))) = dwg_decode_entity(object_type, &mut reader) {
            pending_entities.push((layer_handle, color, geometry));
        }
    }

    if layers.is_empty() {
        layers.push(DwgLayer::default());
    }

    let entities = pending_entities
        .into_iter()
        .map(|(layer_handle, color, geometry)| DwgEntity {
            layer: layer_handle_index.get(&layer_handle).copied().unwrap_or(0),
            color,
            geometry,
        })
        .collect();

    let mut drawing = DwgDrawing { layers, entities, extmin: [0.0; 3], extmax: [0.0; 3] };
    drawing.recompute_extents();
    Ok(drawing)
}
//#endregion DwgRead

//#region DwgBridges
/// 🔺 Wraps mesh data as a single polyface-mesh drawing.
pub fn mesh_to_dwg_drawing(mesh: &MeshData) -> DwgDrawing {
    let vertices: Vec<[f64; 3]> = mesh.positions.chunks_exact(3).map(|c| [c[0] as f64, c[1] as f64, c[2] as f64]).collect();
    let faces: Vec<[i32; 4]> = mesh
        .indices
        .chunks_exact(3)
        .map(|tri| [tri[0] as i32 + 1, tri[1] as i32 + 1, tri[2] as i32 + 1, tri[2] as i32 + 1])
        .collect();
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices, faces } });
    drawing.recompute_extents();
    drawing
}

/// 🔺 Collects polyface-mesh and 3dface entities into mesh data.
pub fn dwg_drawing_to_mesh(drawing: &DwgDrawing) -> MeshData {
    let mut mesh = MeshData::default();
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::PolyfaceMesh { vertices, faces } => {
                let base = mesh.vertex_count() as u32;
                for v in vertices {
                    mesh.positions.extend_from_slice(&[v[0] as f32, v[1] as f32, v[2] as f32]);
                }
                for face in faces {
                    let idx: Vec<u32> = face.iter().map(|i| (i.unsigned_abs().saturating_sub(1)) + base).collect();
                    if face[2] == face[3] {
                        mesh.indices.extend_from_slice(&[idx[0], idx[1], idx[2]]);
                    } else {
                        mesh.indices.extend_from_slice(&[idx[0], idx[1], idx[2]]);
                        mesh.indices.extend_from_slice(&[idx[0], idx[2], idx[3]]);
                    }
                }
            }
            DwgGeometry::Face3d { corners } => {
                let base = mesh.vertex_count() as u32;
                for c in corners {
                    mesh.positions.extend_from_slice(&[c[0] as f32, c[1] as f32, c[2] as f32]);
                }
                mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
                if corners[3] != corners[2] {
                    mesh.indices.extend_from_slice(&[base, base + 2, base + 3]);
                }
            }
            _ => {}
        }
    }
    mesh.compute_normals();
    mesh
}

/// ✏️ Path segment mirror of the 2d kernel's PathSegment (kernel/2d/engine/rs/lib.rs), kept dependency-free.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DwgPathSegment {
    Move { to: [f64; 2] },
    Line { to: [f64; 2] },
    Quad { ctrl: [f64; 2], to: [f64; 2] },
    Cubic { ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2] },
    Arc { rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: [f64; 2] },
    Close,
}

fn arc_bulge(from: [f64; 2], to: [f64; 2], radius: f64, sweep: bool) -> f64 {
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let chord = (dx * dx + dy * dy).sqrt();
    if chord < 1e-9 || radius < 1e-9 {
        return 0.0;
    }
    let included_angle = 2.0 * (chord * 0.5 / radius).clamp(-1.0, 1.0).asin();
    let bulge = (included_angle / 4.0).tan();
    if sweep {
        bulge
    } else {
        -bulge
    }
}

fn bulge_to_segment(from: [f64; 2], to: [f64; 2], bulge: f64) -> DwgPathSegment {
    if bulge.abs() < 1e-9 {
        return DwgPathSegment::Line { to };
    }
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let chord = (dx * dx + dy * dy).sqrt();
    let included_angle = 4.0 * bulge.atan();
    let denom = (2.0 * (included_angle / 2.0).sin()).abs();
    let radius = if denom > 1e-9 { chord / denom } else { 0.0 };
    DwgPathSegment::Arc { rx: radius, ry: radius, rotation: 0.0, large_arc: included_angle.abs() > std::f64::consts::PI, sweep: bulge > 0.0, to }
}

/// ✏️ Converts flattened path segments to dwg entities: line/close runs to lwpolylines with bulge arcs, curves to splines.
pub fn paths_to_dwg_drawing(paths: &[Vec<DwgPathSegment>]) -> DwgDrawing {
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    for path in paths {
        let mut vertices: Vec<[f64; 2]> = Vec::new();
        let mut bulges: Vec<f64> = Vec::new();
        let mut closed = false;
        let mut cursor = [0.0, 0.0];
        let mut start = [0.0, 0.0];
        for segment in path {
            match segment {
                DwgPathSegment::Move { to } => {
                    if !vertices.is_empty() {
                        drawing.entities.push(DwgEntity {
                            layer,
                            color: DwgColor::ByLayer,
                            geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices: vertices.clone(), bulges: bulges.clone() },
                        });
                        vertices.clear();
                        bulges.clear();
                        closed = false;
                    }
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                    start = *to;
                }
                DwgPathSegment::Line { to } => {
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                }
                DwgPathSegment::Quad { ctrl, to } => {
                    let c1 = [cursor[0] + 2.0 / 3.0 * (ctrl[0] - cursor[0]), cursor[1] + 2.0 / 3.0 * (ctrl[1] - cursor[1])];
                    let c2 = [to[0] + 2.0 / 3.0 * (ctrl[0] - to[0]), to[1] + 2.0 / 3.0 * (ctrl[1] - to[1])];
                    let spline_points = [cursor, c1, c2, *to];
                    drawing.entities.push(DwgEntity {
                        layer,
                        color: DwgColor::ByLayer,
                        geometry: DwgGeometry::Spline {
                            degree: 3,
                            control_points: spline_points.iter().map(|p| [p[0], p[1], 0.0]).collect(),
                            knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                            weights: vec![1.0; 4],
                        },
                    });
                    cursor = *to;
                }
                DwgPathSegment::Cubic { ctrl1, ctrl2, to } => {
                    let spline_points = [cursor, *ctrl1, *ctrl2, *to];
                    drawing.entities.push(DwgEntity {
                        layer,
                        color: DwgColor::ByLayer,
                        geometry: DwgGeometry::Spline {
                            degree: 3,
                            control_points: spline_points.iter().map(|p| [p[0], p[1], 0.0]).collect(),
                            knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                            weights: vec![1.0; 4],
                        },
                    });
                    cursor = *to;
                }
                DwgPathSegment::Arc { rx, sweep, to, .. } => {
                    let bulge = arc_bulge(cursor, *to, *rx, *sweep);
                    if let Some(last) = bulges.last_mut() {
                        *last = bulge;
                    }
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                }
                DwgPathSegment::Close => {
                    closed = true;
                    cursor = start;
                }
            }
        }
        if !vertices.is_empty() {
            drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices, bulges } });
        }
    }
    drawing.recompute_extents();
    drawing
}

/// ✏️ Converts drawing entities back to path segments, one path per entity.
pub fn dwg_drawing_to_paths(drawing: &DwgDrawing) -> Vec<Vec<DwgPathSegment>> {
    let mut paths = Vec::new();
    for entity in &drawing.entities {
        match &entity.geometry {
            DwgGeometry::LwPolyline { closed, vertices, bulges, .. } => {
                if vertices.is_empty() {
                    continue;
                }
                let mut segments = vec![DwgPathSegment::Move { to: vertices[0] }];
                for i in 1..vertices.len() {
                    let from = vertices[i - 1];
                    let to = vertices[i];
                    let bulge = bulges.get(i - 1).copied().unwrap_or(0.0);
                    segments.push(bulge_to_segment(from, to, bulge));
                }
                if *closed && vertices.len() > 1 {
                    let bulge = bulges.last().copied().unwrap_or(0.0);
                    segments.push(bulge_to_segment(vertices[vertices.len() - 1], vertices[0], bulge));
                    segments.push(DwgPathSegment::Close);
                }
                paths.push(segments);
            }
            DwgGeometry::Spline { degree, control_points, .. } if *degree == 3 && control_points.len() == 4 => {
                paths.push(vec![
                    DwgPathSegment::Move { to: [control_points[0][0], control_points[0][1]] },
                    DwgPathSegment::Cubic {
                        ctrl1: [control_points[1][0], control_points[1][1]],
                        ctrl2: [control_points[2][0], control_points[2][1]],
                        to: [control_points[3][0], control_points[3][1]],
                    },
                ]);
            }
            DwgGeometry::Circle { center, radius, .. } => {
                paths.push(vec![
                    DwgPathSegment::Move { to: [center[0] + radius, center[1]] },
                    DwgPathSegment::Arc { rx: *radius, ry: *radius, rotation: 0.0, large_arc: true, sweep: true, to: [center[0] - radius, center[1]] },
                    DwgPathSegment::Arc { rx: *radius, ry: *radius, rotation: 0.0, large_arc: true, sweep: true, to: [center[0] + radius, center[1]] },
                    DwgPathSegment::Close,
                ]);
            }
            _ => {}
        }
    }
    paths
}
//#endregion DwgBridges
//#endregion Dwg

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_has_triangles() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        assert_eq!(mesh.triangle_count(), 12);
        assert_eq!(mesh.normals.len(), mesh.positions.len());
    }

    #[test]
    fn obj_contains_faces() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        let obj = mesh_to_obj(&mesh, "box");
        assert!(obj.contains("o box"));
        assert!(obj.contains("f "));
    }

    #[test]
    fn glb_round_trip() {
        let mesh = mesh_uv_sphere(1.0, 8, 6);
        let glb = mesh_to_glb(&mesh);
        let decoded = mesh_from_glb(&glb).expect("decode glb");
        assert_eq!(decoded.vertex_count(), mesh.vertex_count());
        assert_eq!(decoded.indices.len(), mesh.indices.len());
    }

    #[test]
    fn primitive_kinds() {
        assert!(mesh_from_kind("sphere").vertex_count() > 0);
        assert!(mesh_from_kind("box").vertex_count() > 0);
    }

    #[test]
    fn obj_round_trip() {
        let mesh = mesh_uv_sphere(1.0, 8, 6);
        let obj = mesh_to_obj(&mesh, "sphere");
        let decoded = mesh_from_obj(&obj).expect("decode obj");
        assert_eq!(decoded.vertex_count(), mesh.vertex_count());
        assert_eq!(decoded.indices.len(), mesh.indices.len());
    }

    #[test]
    fn stl_round_trip() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        let stl = mesh_to_stl(&mesh);
        assert_eq!(stl.len(), 80 + 4 + mesh.triangle_count() * 50);
        let decoded = mesh_from_stl(&stl).expect("decode stl");
        assert_eq!(decoded.triangle_count(), mesh.triangle_count());
        assert_eq!(decoded.positions.len(), mesh.triangle_count() * 9);
    }

    #[test]
    fn media_format_round_trips_str_and_binary_flags() {
        for format in [
            OsMediaFormat::Svg,
            OsMediaFormat::Png,
            OsMediaFormat::Obj,
            OsMediaFormat::Glb,
            OsMediaFormat::Stl,
            OsMediaFormat::Step,
            OsMediaFormat::Dwg,
        ] {
            assert_eq!(OsMediaFormat::parse(format.as_str()), Some(format.clone()));
        }
        assert!(OsMediaFormat::Glb.is_binary());
        assert!(OsMediaFormat::Stl.is_binary());
        assert!(!OsMediaFormat::Step.is_binary());
        assert!(!OsMediaFormat::Obj.is_binary());
    }

    #[test]
    fn mesh_exporters_and_importers_round_trip_through_the_trait_objects() {
        let mesh = mesh_box(1.0, 1.0, 1.0);
        let codecs: Vec<(Box<dyn MeshExporter>, Box<dyn MeshImporter>)> = vec![
            (Box::new(ObjExporter), Box::new(ObjImporter)),
            (Box::new(GlbExporter), Box::new(GlbImporter)),
            (Box::new(StlExporter), Box::new(StlImporter)),
        ];
        for (exporter, importer) in codecs {
            assert_eq!(exporter.format(), importer.format());
            let bytes = exporter.export(&mesh).expect("export");
            let decoded = importer.import(&bytes).expect("import");
            assert_eq!(decoded.triangle_count(), mesh.triangle_count());
        }
    }

    /// 🔺 Small shared-vertex tetrahedron fixture (4 verts, 4 triangles) used by the format round-trip tests below — small enough to assert exact positions/indices, but with enough shared vertices to exercise indexed (not per-face-duplicated) geometry.
    fn tetra_mesh_fixture() -> MeshData {
        let mut mesh = MeshData {
            positions: vec![
                0.0, 0.0, 0.0, // v0
                1.0, 0.0, 0.0, // v1
                0.0, 1.0, 0.0, // v2
                0.0, 0.0, 1.0, // v3
            ],
            indices: vec![0, 1, 2, 0, 1, 3, 0, 2, 3, 1, 2, 3],
            ..MeshData::default()
        };
        mesh.compute_normals();
        mesh
    }

    fn assert_positions_close(a: &[f32], b: &[f32]) {
        assert_eq!(a.len(), b.len(), "position array length mismatch");
        for (x, y) in a.iter().zip(b.iter()) {
            assert!((x - y).abs() < 1e-4, "position mismatch: {x} vs {y}");
        }
    }

    #[test]
    fn obj_round_trip_preserves_positions_and_indices() {
        let mesh = tetra_mesh_fixture();
        let bytes = ObjExporter.export(&mesh).expect("export obj");
        let decoded = ObjImporter.import(&bytes).expect("import obj");
        assert_positions_close(&decoded.positions, &mesh.positions);
        assert_eq!(decoded.indices, mesh.indices);
    }

    #[test]
    fn glb_round_trip_preserves_positions_and_indices() {
        let mesh = tetra_mesh_fixture();
        let bytes = GlbExporter.export(&mesh).expect("export glb");
        let decoded = GlbImporter.import(&bytes).expect("import glb");
        assert_positions_close(&decoded.positions, &mesh.positions);
        assert_eq!(decoded.indices, mesh.indices);
    }

    #[test]
    fn stl_round_trip_preserves_triangle_geometry() {
        let mesh = tetra_mesh_fixture();
        let bytes = StlExporter.export(&mesh).expect("export stl");
        let decoded = StlImporter.import(&bytes).expect("import stl");
        assert_eq!(decoded.triangle_count(), mesh.triangle_count());
        // STL has no vertex sharing, so indices are trivially [0, 1, 2, 3, ...]; compare
        // per-triangle corner positions against the original indexed mesh instead.
        for (triangle, decoded_tri) in mesh.indices.chunks_exact(3).zip(decoded.indices.chunks_exact(3)) {
            for (&original_index, &decoded_index) in triangle.iter().zip(decoded_tri.iter()) {
                let original = &mesh.positions[original_index as usize * 3..original_index as usize * 3 + 3];
                let decoded_position = &decoded.positions[decoded_index as usize * 3..decoded_index as usize * 3 + 3];
                assert_positions_close(decoded_position, original);
            }
        }
    }

    #[test]
    fn mesh_from_indexed_with_face_groups_stamps_per_triangle_face_ids() {
        let positions: Vec<f32> = (0..6 * 3 * 3).map(|i| i as f32).collect();
        let indices: Vec<u32> = (0..18).collect();
        let face_groups = [(101u32, 0u32, 6u32), (202u32, 6u32, 12u32)];
        let mesh = mesh_from_indexed_with_face_groups(&positions, &[], &indices, &face_groups);
        assert_eq!(mesh.face_ids.len(), 6);
        assert_eq!(&mesh.face_ids[0..2], &[101, 101]);
        assert_eq!(&mesh.face_ids[2..6], &[202, 202, 202, 202]);
    }

    #[test]
    fn mesh_from_indexed_with_face_groups_empty_groups_leaves_face_ids_empty() {
        let positions: Vec<f32> = (0..9).map(|i| i as f32).collect();
        let indices: Vec<u32> = vec![0, 1, 2];
        let mesh = mesh_from_indexed_with_face_groups(&positions, &[], &indices, &[]);
        assert!(mesh.face_ids.is_empty());
    }

    #[test]
    fn dwg_bit_primitives_round_trip_at_unaligned_offsets() {
        let mut writer = DwgBitWriter::new();
        writer.write_bit(true);
        writer.write_bit(false);
        writer.write_bit(true);
        writer.write_bs(0);
        writer.write_bs(256);
        writer.write_bs(42);
        writer.write_bs(12345);
        writer.write_bl(0);
        writer.write_bl(200);
        writer.write_bl(70000);
        writer.write_bd(0.0);
        writer.write_bd(1.0);
        writer.write_bd(3.14159);
        writer.write_ms(70000);
        writer.write_handle(5, 0x1234);
        writer.write_t("héllo");
        writer.pad_to_byte();

        let mut reader = DwgBitReader::new(&writer.bytes);
        assert!(reader.read_bit().unwrap());
        assert!(!reader.read_bit().unwrap());
        assert!(reader.read_bit().unwrap());
        assert_eq!(reader.read_bs().unwrap(), 0);
        assert_eq!(reader.read_bs().unwrap(), 256);
        assert_eq!(reader.read_bs().unwrap(), 42);
        assert_eq!(reader.read_bs().unwrap(), 12345);
        assert_eq!(reader.read_bl().unwrap(), 0);
        assert_eq!(reader.read_bl().unwrap(), 200);
        assert_eq!(reader.read_bl().unwrap(), 70000);
        assert_eq!(reader.read_bd().unwrap(), 0.0);
        assert_eq!(reader.read_bd().unwrap(), 1.0);
        assert_eq!(reader.read_bd().unwrap(), 3.14159);
        assert_eq!(reader.read_ms().unwrap(), 70000);
        assert_eq!(reader.read_handle().unwrap(), (5, 0x1234));
        assert_eq!(reader.read_t().unwrap(), "héllo");
    }

    #[test]
    fn dwg_crc16_matches_seed_on_empty_input() {
        assert_eq!(dwg_crc16(0xC0C1, &[]), 0xC0C1);
        assert_ne!(dwg_crc16(0xC0C1, &[1, 2, 3]), 0xC0C1);
    }

    #[test]
    fn dwg_writer_produces_a_structurally_valid_container() {
        let bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode empty drawing");
        assert_eq!(&bytes[0..6], b"AC1015");
        let section_count = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        assert_eq!(section_count, 3);
        assert_eq!(&bytes[DWG_FILE_HEADER_LEN - 16..DWG_FILE_HEADER_LEN], &DWG_SENTINEL_FILE_HEADER_END);
    }

    #[test]
    fn dwg_full_entity_set_round_trips() {
        let mut drawing = DwgDrawing::default();
        let layer_a = drawing.ensure_layer("outline");
        let layer_b = drawing.ensure_layer("solids");
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::Index(3), geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [10.0, 5.0, 0.0] } });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 3.0] } });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByBlock, geometry: DwgGeometry::Circle { center: [0.0, 0.0, 0.0], radius: 5.0, normal: [0.0, 0.0, 1.0] } });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::Index(1),
            geometry: DwgGeometry::Arc { center: [0.0, 0.0, 0.0], radius: 3.0, start_angle: 0.0, end_angle: 1.57, normal: [0.0, 0.0, 1.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::Index(2),
            geometry: DwgGeometry::Ellipse { center: [1.0, 1.0, 0.0], major_axis: [4.0, 0.0, 0.0], ratio: 0.5, start_param: 0.0, end_param: 6.28, normal: [0.0, 0.0, 1.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], bulges: vec![0.0, 0.5, 0.0] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_a,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Spline {
                degree: 3,
                control_points: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [3.0, 2.0, 0.0], [4.0, 0.0, 0.0]],
                knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                weights: vec![1.0; 4],
            },
        });
        drawing.entities.push(DwgEntity { layer: layer_a, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [0.0, 0.0, 0.0], height: 2.5, rotation: 0.0, content: "semio".to_string() } });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Face3d { corners: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::Polyline3d { closed: false, vertices: vec![[0.0, 0.0, 0.0], [0.0, 0.0, 5.0], [1.0, 0.0, 5.0]] },
        });
        drawing.entities.push(DwgEntity {
            layer: layer_b,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] },
        });

        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded = dwg_from_bytes(&bytes).expect("decode");

        assert_eq!(decoded.entities.len(), drawing.entities.len());
        assert_eq!(decoded.layers.len(), drawing.layers.len());
        for (original, round_tripped) in drawing.entities.iter().zip(decoded.entities.iter()) {
            assert_eq!(original.geometry, round_tripped.geometry);
            assert_eq!(original.color, round_tripped.color);
            assert_eq!(drawing.layers[original.layer].name, decoded.layers[round_tripped.layer].name);
        }
    }

    #[test]
    fn dwg_mesh_bridge_round_trips_triangle_count_and_positions() {
        let mesh = mesh_box(2.0, 2.0, 2.0);
        let drawing = mesh_to_dwg_drawing(&mesh);
        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded_drawing = dwg_from_bytes(&bytes).expect("decode");
        let decoded_mesh = dwg_drawing_to_mesh(&decoded_drawing);
        assert_eq!(decoded_mesh.triangle_count(), mesh.triangle_count());
        assert_eq!(decoded_mesh.vertex_count(), mesh.vertex_count());
    }

    #[test]
    fn dwg_path_bridge_round_trips_cubic_control_points_exactly() {
        let paths = vec![vec![
            DwgPathSegment::Move { to: [0.0, 0.0] },
            DwgPathSegment::Line { to: [5.0, 0.0] },
            DwgPathSegment::Cubic { ctrl1: [6.0, 1.0], ctrl2: [7.0, 3.0], to: [5.0, 4.0] },
            DwgPathSegment::Close,
        ]];
        let drawing = paths_to_dwg_drawing(&paths);
        let bytes = dwg_to_bytes(&drawing).expect("encode");
        let decoded = dwg_from_bytes(&bytes).expect("decode");
        let round_tripped_paths = dwg_drawing_to_paths(&decoded);

        let cubic_found = round_tripped_paths.iter().flatten().any(|segment| {
            matches!(segment, DwgPathSegment::Cubic { ctrl1, ctrl2, to }
                if (ctrl1[0] - 6.0).abs() < 1e-9 && (ctrl2[1] - 3.0).abs() < 1e-9 && (to[1] - 4.0).abs() < 1e-9)
        });
        assert!(cubic_found, "expected the exact cubic control points to survive the dwg round trip");

        let line_found = round_tripped_paths.iter().flatten().any(|segment| matches!(segment, DwgPathSegment::Line { to } if (to[0] - 5.0).abs() < 1e-9));
        assert!(line_found, "expected the polyline segment to survive the dwg round trip");
    }

    #[test]
    fn dwg_rejects_unsupported_version() {
        let mut bytes = dwg_to_bytes(&DwgDrawing::default()).expect("encode");
        bytes[0..6].copy_from_slice(b"AC1018");
        let err = dwg_from_bytes(&bytes).expect_err("should reject non-R2000 version");
        assert!(err.contains("AC1018"));
    }

    #[test]
    fn dwg_reader_skips_unknown_object_types_without_failing() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 1.0, 1.0] } });
        let mut bytes = dwg_to_bytes(&drawing).expect("encode");

        let mut bogus_body = DwgBitWriter::new();
        bogus_body.write_rc(0xFF);
        let mut bogus_handles = DwgBitWriter::new();
        let bogus_offset = bytes.len();
        dwg_write_object(&mut bytes, 900, 0x9999, &mut bogus_body, &mut bogus_handles);

        let map_locator_pos = 10 + 2 * 9;
        let map_offset = u32::from_le_bytes(bytes[map_locator_pos + 1..map_locator_pos + 5].try_into().unwrap());
        let map_size = u32::from_le_bytes(bytes[map_locator_pos + 5..map_locator_pos + 9].try_into().unwrap());
        let mut new_entry = Vec::new();
        new_entry.extend_from_slice(&0x9999u64.to_le_bytes());
        new_entry.extend_from_slice(&(bogus_offset as u64).to_le_bytes());
        let insert_at = map_offset as usize + 4;
        for (i, b) in new_entry.iter().enumerate() {
            bytes.insert(insert_at + i, *b);
        }
        let new_count = u32::from_le_bytes(bytes[map_offset as usize..map_offset as usize + 4].try_into().unwrap()) + 1;
        bytes[map_offset as usize..map_offset as usize + 4].copy_from_slice(&new_count.to_le_bytes());
        let new_size = map_size + new_entry.len() as u32;
        bytes[map_locator_pos + 5..map_locator_pos + 9].copy_from_slice(&new_size.to_le_bytes());

        let decoded = dwg_from_bytes(&bytes).expect("reader should tolerate the unknown object type");
        assert_eq!(decoded.entities.len(), 1);
    }
}
// #endregion mesh
}

pub mod platform {
// #region platform
//! 🖥️ Root shell: apps, URI chrome, panel toggles, and shared action bus.

use crate::action_bus::ActionBus;
use crate::ui::AppDefinition;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PanelVisibility {
    pub left_side_panel: bool,
    pub right_side_panel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct PlatformSpec {
    pub id: String,
    pub name: String,
    pub default_active_app_id: Option<String>,
    pub initial_panel_visibility: Option<PanelVisibility>,
}

pub struct Platform {
    pub action_bus: ActionBus,
    pub apps: Vec<AppDefinition>,
    pub active_app_id: String,
    pub generation: u64,
    pub chrome_generation: u64,
    pub uri: String,
    pub panel_visibility: PanelVisibility,
    pub id: String,
    pub name: String,
    generation_counter: AtomicU64,
    chrome_generation_counter: AtomicU64,
}

impl Platform {
    pub fn new(spec: Option<PlatformSpec>) -> Self {
        let spec = spec.unwrap_or_default();
        let panel_visibility = spec.initial_panel_visibility.clone().unwrap_or_default();
        Self {
            action_bus: ActionBus::new(),
            apps: Vec::new(),
            active_app_id: spec.default_active_app_id.clone().unwrap_or_default(),
            generation: 0,
            chrome_generation: 0,
            uri: "/".into(),
            panel_visibility,
            id: spec.id,
            name: spec.name,
            generation_counter: AtomicU64::new(0),
            chrome_generation_counter: AtomicU64::new(0),
        }
    }

    pub fn add_app(&mut self, app: AppDefinition) {
        if self.active_app_id.is_empty() {
            self.active_app_id = app.id.clone();
        }
        self.apps.push(app);
        self.notify();
    }

    pub fn get_active_app(&self) -> Option<&AppDefinition> {
        self.apps
            .iter()
            .find(|app| app.id == self.active_app_id)
            .or_else(|| self.apps.first())
    }

    pub fn set_active_app_id(&mut self, id: String) {
        if self.active_app_id == id {
            return;
        }
        self.active_app_id = id;
        self.notify_chrome();
    }

    pub fn set_panel_visibility(&mut self, next: PanelVisibility) {
        if self.panel_visibility == next {
            return;
        }
        self.panel_visibility = next;
        self.notify_chrome();
    }

    pub fn notify(&mut self) {
        self.generation = self.generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }

    pub fn notify_chrome(&mut self) {
        self.chrome_generation = self.chrome_generation_counter.fetch_add(1, Ordering::SeqCst) + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{ModeDefinition, WindowKindDefinition};

    #[test]
    fn adds_first_app_as_active() {
        let mut platform = Platform::new(None);
        platform.add_app(AppDefinition {
            id: "draw-play".into(),
            label: "Draw".into(),
            document: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: crate::ui::Modes::one(ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
                utilities: Vec::new(),
                layout_id: None,
                commands: Vec::new(),
            }),
            default_mode_id: "edit".into(),
            window_kinds: crate::ui::WindowKinds::one(WindowKindDefinition {
                id: "composite".into(),
                label: "Canvas".into(),
                body_key: "composite".into(),
                surface_kind: ui_wgpu::SurfaceKind::Canvas2d,
                icon_id: None,
                options: ui_wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                params_schema: None,
                document_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            actions: vec![],
            utilities: vec![],
            commands: vec![],
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            introduction: None,
            dialogs: Vec::new(),
        });
        assert_eq!(platform.active_app_id, "draw-play");
    }
}
// #endregion platform
}

pub mod ui {
// #region ui
//! 🧩 App manifest (`AppDefinition`/`ModeDefinition`/`WindowKindDefinition`/`PluginManifest`/`ViewState`)
//! and kernel types shared by plugins and renderers; the declarative `UiNode` component model itself
//! lives in `ui_wgpu`'s `component` region.

use serde::{Deserialize, Serialize};
use ui_wgpu::{ActionDescriptor, NamedLayout, SurfaceKind, WindowLayout, WindowOptions};

//#region 🔖Manifest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Keybinding {
    pub keys: String,
    pub action: ActionDescriptor,
}

/// @emoji 🗂️ Classifies a declared action by how it interacts with VCS history.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ActionKind {
    /// Mutates the document — dispatched as VCS operations with a true inverse, recorded in history.
    Operation,
    /// Ephemeral view state (camera, selection, hover, active utility) — not recorded in history.
    View,
    /// Framework-provided undo/redo/checkpoint/alternative — auto-injected, never app-declared.
    History,
    /// Shell-only effect (navigate, export, spawn) — no document mutation.
    Shell,
}

//#region 🔖ActionArgs
/// @emoji 🔘 One selectable option of a `Select` argument control — the persisted `value` and its
/// human `label`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ActionArgOption {
    pub value: String,
    pub label: String,
}

impl ActionArgOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into() }
    }
}

/// @emoji 🎚️ Declarative input control for one action argument — a lean manifest-altitude enum,
/// deliberately NOT `ui_wgpu::UiControlNode` (whose variants embed live values and immediate-dispatch
/// wiring). Renderers map each variant onto a staged form field. Tagged with `kind` to mirror the
/// sibling `UtilityNode`/`UiControlNode` declarative-tree convention.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActionArgControl {
    Text {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        placeholder: Option<String>,
    },
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        step: Option<f64>,
    },
    Slider {
        min: f64,
        max: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        unit: Option<String>,
    },
    Toggle,
    Select {
        options: Vec<ActionArgOption>,
    },
    Vec3,
    IconSelect {
        classifier_kind: String,
    },
}

/// @emoji 📝 Declares one argument of an action: its `id` (the JSON key sent in `ActionDescriptor.args`),
/// human `label`, input `control`, whether it is `required`, an optional `default` value, and an optional
/// `description`. An empty `ActionDefinition.args` (the common case) means a no-argument action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ActionArgDef {
    pub id: String,
    pub label: String,
    pub control: ActionArgControl,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub description: Option<String>,
}

impl ActionArgDef {
    fn with_control(id: impl Into<String>, label: impl Into<String>, control: ActionArgControl) -> Self {
        Self { id: id.into(), label: label.into(), control, required: false, default: None, description: None }
    }

    /// @emoji 🔤 A free-text argument.
    pub fn text(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::with_control(id, label, ActionArgControl::Text { placeholder: None })
    }

    /// @emoji 🔢 A numeric argument (unbounded stepper by default).
    pub fn number(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::with_control(id, label, ActionArgControl::Number { min: None, max: None, step: None })
    }

    /// @emoji 🎚️ A bounded slider argument.
    pub fn slider(id: impl Into<String>, label: impl Into<String>, min: f64, max: f64) -> Self {
        Self::with_control(id, label, ActionArgControl::Slider { min, max, step: None, unit: None })
    }

    /// @emoji 🔘 A boolean toggle argument.
    pub fn toggle(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::with_control(id, label, ActionArgControl::Toggle)
    }

    /// @emoji 🔽 A single-choice select argument.
    pub fn select(id: impl Into<String>, label: impl Into<String>, options: Vec<ActionArgOption>) -> Self {
        Self::with_control(id, label, ActionArgControl::Select { options })
    }

    /// @emoji 🧭 A three-component vector argument.
    pub fn vec3(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::with_control(id, label, ActionArgControl::Vec3)
    }

    /// @emoji ❗ Marks the argument as required — execution is blocked until it has an effective value.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// @emoji 🎁 Sets the default effective value used when nothing is staged.
    pub fn default_value(mut self, value: impl Into<serde_json::Value>) -> Self {
        self.default = Some(value.into());
        self
    }

    /// @emoji 💬 Attaches a description shown alongside the field.
    pub fn describe(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}
//#endregion 🔖ActionArgs

/// @emoji 📇 Declares one action an app can receive via `ActionDescriptor.action`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ActionDefinition {
    pub id: String,
    pub label: String,
    pub kind: ActionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub icon_id: Option<String>,
    /// 📝 Typed argument declarations. Empty (the common case) = a no-argument action; serde-defaults
    /// to empty so manifests/fixtures without this field still deserialize.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<ActionArgDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub keys: Option<String>,
    #[serde(default)]
    pub in_palette: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub category: Option<String>,
}

impl ActionDefinition {
    pub fn new(id: impl Into<String>, label: impl Into<String>, kind: ActionKind) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind,
            icon_id: None,
            args: Vec::new(),
            keys: None,
            in_palette: true,
            category: None,
        }
    }

    /// @emoji 📝 Attaches typed argument declarations to this action.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }
}

/// @emoji 🕹️ The six framework-owned History actions, auto-injected into every `AppDefinition`.
pub fn history_action_definitions() -> Vec<ActionDefinition> {
    vec![
        ActionDefinition {
            keys: Some("mod+z".into()),
            ..ActionDefinition::new("undo", "Undo", ActionKind::History)
        },
        ActionDefinition {
            keys: Some("mod+shift+z".into()),
            ..ActionDefinition::new("redo", "Redo", ActionKind::History)
        },
        ActionDefinition::new("commitCheckpoint", "Commit Checkpoint", ActionKind::History),
        ActionDefinition::new("createAlternative", "Create Alternative", ActionKind::History),
        ActionDefinition::new("switchAlternative", "Switch Alternative", ActionKind::History),
        ActionDefinition::new("checkoutCheckpoint", "Checkout Checkpoint", ActionKind::History),
    ]
}

/// @emoji 🧰 The framework-owned action id apps dispatch to activate a utility — auto-injected as a View
/// action into any `AppDefinition` that declares utilities (mirrors `history_action_definitions`).
pub const SET_ACTIVE_UTILITY_ACTION_ID: &str = "setActiveUtility";

/// @emoji 🧰 The framework-injected `setActiveUtility` View action (never in the palette): switches the
/// host-owned active utility of a window kind. `utilityId` is required; `windowKindId` is contextual (the
/// shell fills it from the focused window when absent).
pub fn set_active_utility_action_definition() -> ActionDefinition {
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new(SET_ACTIVE_UTILITY_ACTION_ID, "Set Active Utility", ActionKind::View)
    }
    .with_args([
        ActionArgDef::text("utilityId", "Utility").required(),
        ActionArgDef::text("windowKindId", "Window"),
    ])
}

/// @emoji 🎓 The framework-owned action id apps dispatch to (re)start an app's introduction —
/// auto-injected as a shell-intercepted View action into any
/// `AppDefinition` that declares one (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).
pub const START_INTRODUCTION_ACTION_ID: &str = "startIntroduction";

/// @emoji 🎓 The framework-injected `startIntroduction` View action: fully shell-intercepted (never
/// forwarded to the plugin), it resets playback to the first step of `AppDefinition.introduction`.
/// Unlike ordinary app actions this stays out of the action palette because the shell exposes the
/// dedicated `Introduce App` command.
pub fn start_introduction_action_definition() -> ActionDefinition {
    ActionDefinition {
        in_palette: false,
        ..ActionDefinition::new(START_INTRODUCTION_ACTION_ID, "Introduce App", ActionKind::View)
    }
}

/// 📇 A validated reference into an app's `AppDefinition.actions` registry — prevents windows/modes
/// from silently inheriting "every app action" by making the scoping explicit and typed. Distinct
/// from `kernel::ActionId` (a dispatched-invocation identifier); this one names a *declaration*.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct ActionRef(String);

impl ActionRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ActionRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ActionRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

//#region 🔖Utilities
/// @emoji 🧰 Declares one interactive utility (a live-preview pointer mode) an app exposes. Distinct from
/// an `ActionDefinition`: exactly one utility is active per window kind at a time, and activation is
/// host-owned session view state (`ViewState.active_utility_id`), never a document field or VCS op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct UtilityDefinition {
    pub id: String,
    pub label: String,
    pub icon_id: String,
    /// 🧺 Visual toolbar collection this utility groups into; `None` = a flat top-level toolbar entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub keys: Option<String>,
    /// 🖱️ CSS/winit cursor name applied to the window body while this utility is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub category: Option<ui_wgpu::UtilityCategory>,
    /// 🚦 Whether window-scoped actions stay enabled while this utility is active. Defaults to `false`
    /// (matching today's whitelist-based gating where an active utility suppresses the action panel);
    /// set `true` for passive view utilities (e.g. cad `cad.play.view.*`) that should not gate actions.
    #[serde(default)]
    pub allows_actions_while_active: bool,
}

impl UtilityDefinition {
    /// @emoji 🧰 A utility with sensible defaults (no group/keys/cursor/category, gates actions while active).
    pub fn new(id: impl Into<String>, label: impl Into<String>, icon_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon_id: icon_id.into(),
            group: None,
            keys: None,
            cursor: None,
            category: None,
            allows_actions_while_active: false,
        }
    }
}

/// @emoji 🧰 A validated reference into an app's `AppDefinition.utilities` registry — the utility mirror of
/// `ActionRef`, scoping utilities to window kinds/modes with a typed, resolvable id.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct UtilityRef(String);

impl UtilityRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for UtilityRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for UtilityRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖Utilities

//#region 🔖Commands
/// @emoji 🗂️ Where a command is offered. There are no window-level commands — window-scoped verbs
/// stay `ActionDefinition`/`UtilityDefinition`; a command is scoped to the os shell, a plugin, an app, or
/// one of an app's modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum CommandScope {
    Os,
    Plugin,
    App,
    Mode,
}

/// @emoji 🎛️ Declares one command: a scoped, categorized verb offered in the footer command panel.
/// Handling a command may emit VCS-tracked operations exactly like an operation-kind action — see
/// `DocumentApp::handle_command`/`ActionEmit`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandDefinition {
    pub id: String,
    pub label: String,
    pub scope: CommandScope,
    /// 🗂️ Footer category tab this command groups under (an open id, e.g. "document", "appearance").
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub icon_id: Option<String>,
    /// 📝 Reuses `ActionArgDef` — one staged-form contract shared by actions, dialogs, and commands.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<ActionArgDef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub keys: Option<String>,
    #[serde(default)]
    pub in_palette: bool,
}

impl CommandDefinition {
    pub fn new(id: impl Into<String>, label: impl Into<String>, scope: CommandScope, category: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            scope,
            category: category.into(),
            icon_id: None,
            args: Vec::new(),
            keys: None,
            in_palette: true,
        }
    }

    /// @emoji 📝 Attaches typed argument declarations to this command.
    pub fn with_args(mut self, args: impl IntoIterator<Item = ActionArgDef>) -> Self {
        self.args = args.into_iter().collect();
        self
    }
}

/// 🎛️ A validated reference into an app's `AppDefinition.commands` registry — the command mirror of
/// `ActionRef`/`UtilityRef`. Only ever names a `Mode`-scope command (see `ModeDefinition.commands`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct CommandRef(String);

impl CommandRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CommandRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for CommandRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
//#endregion 🔖Commands

//#region 🔖Introduction
/// @emoji 🎓 A first-run walkthrough an app declares to introduce its UI, utilities, and actions to a
/// first-time user. Rendered as an ordered sequence of `IntroductionStepDefinition`s over a full-screen
/// glass veil; the shell owns playback (start/advance/skip) as ephemeral chrome state, never the
/// document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct IntroductionDefinition {
    pub title: String,
    pub steps: Vec<IntroductionStepDefinition>,
}

/// @emoji 🪜 One step of an `IntroductionDefinition`: an info box pointing at `anchor`, with an
/// `emphasis` treatment cut out of the glass veil and an `advance` condition that completes the step.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct IntroductionStepDefinition {
    pub id: String,
    pub title: String,
    pub body: String,
    pub anchor: IntroductionAnchor,
    #[serde(default)]
    pub emphasis: IntroductionEmphasis,
    #[serde(default)]
    pub placement: IntroductionPlacement,
    #[serde(default)]
    pub advance: IntroductionAdvance,
}

impl IntroductionStepDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<String>, body: impl Into<String>, anchor: IntroductionAnchor) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
            anchor,
            emphasis: IntroductionEmphasis::default(),
            placement: IntroductionPlacement::default(),
            advance: IntroductionAdvance::default(),
        }
    }

    /// @emoji 🔦 Overrides how the anchor is emphasized against the glass veil.
    pub fn emphasis(mut self, emphasis: IntroductionEmphasis) -> Self {
        self.emphasis = emphasis;
        self
    }

    /// @emoji 📍 Overrides where the info box is placed relative to the anchor.
    pub fn placement(mut self, placement: IntroductionPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// @emoji 👉 Makes the step complete when the user performs `advance` instead of pressing Next.
    pub fn advance_on(mut self, advance: IntroductionAdvance) -> Self {
        self.advance = advance;
        self
    }
}

/// @emoji 🎯 Renderer-agnostic reference to the UI element an `IntroductionStepDefinition` points at —
/// no CSS/DOM types leak into the contract; each renderer maps a variant onto its own element lookup
/// (the React shell resolves these to `data-*` selectors).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum IntroductionAnchor {
    /// 🖥️ No specific element — the whole screen (paired with `IntroductionPlacement::Center`).
    Screen,
    Navbar,
    Footer,
    /// 🪟 References `AppDefinition.windowKinds[].id`.
    WindowKind(String),
    /// 🧰 References `AppDefinition.utilities`.
    Utility(UtilityRef),
    /// 📇 References `AppDefinition.actions`.
    Action(ActionRef),
    /// 📑 References a declared `PanelTabDefinition.id()`.
    PanelTab(String),
    /// 🪝 Escape hatch: a well-known `data-slot` name, unvalidated.
    Slot(String),
}

/// @emoji 🔦 How a step's anchor is treated against the full-screen glass veil.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum IntroductionEmphasis {
    /// 🌫️ No cutout — the anchor stays veiled (used with `IntroductionAnchor::Screen`).
    None,
    /// 🕳️ The anchor is cut out of the veil, shown normally and interactive.
    #[default]
    Cutout,
    /// ✨ Cutout plus an animated ring around the anchor.
    Highlight,
}

/// @emoji 📍 Where the info box is placed relative to its anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum IntroductionPlacement {
    #[default]
    Auto,
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

/// @emoji 👉 What completes an introduction step. `Next` needs the info box's Next button; `Action`/
/// `Utility` complete as soon as the user dispatches that action or activates that utility, teaching by doing.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum IntroductionAdvance {
    #[default]
    Next,
    /// 📇 References `AppDefinition.actions`.
    Action(ActionRef),
    /// 🧰 References `AppDefinition.utilities`.
    Utility(UtilityRef),
}
//#endregion 🔖Introduction

//#region 🔖Dialog
/// @emoji 🗨️ A declared modal form dialog: a glass veil covers the screen and an info box (styled
/// identically to the introduction walkthrough box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)
/// presents `args` as a staged form. Submit dispatches `submit_action` with the merged effective
/// args; empty `args` degenerates to a message/confirm dialog. Opened only via
/// `HostEffect::OpenDialog`; the shell owns open/close as ephemeral chrome state, never the document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct DialogDefinition {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<ActionArgDef>,
    /// 📇 References `AppDefinition.actions` — dispatched with the merged effective args on submit.
    pub submit_action: ActionRef,
    pub submit_label: String,
    /// 📇 Optional `AppDefinition.actions` reference dispatched on any dismissal (Escape, veil
    /// click, or the Cancel button).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub cancel_action: Option<ActionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub cancel_label: Option<String>,
}

impl DialogDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<String>, submit_action: ActionRef) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: None,
            args: Vec::new(),
            submit_action,
            submit_label: "OK".into(),
            cancel_action: None,
            cancel_label: None,
        }
    }

    /// @emoji 📝 Attaches explanatory body text shown below the title.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// @emoji 🧾 Attaches the staged-form field declarations.
    pub fn args(mut self, args: Vec<ActionArgDef>) -> Self {
        self.args = args;
        self
    }

    /// @emoji ✅ Overrides the submit button label (default "OK").
    pub fn submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// @emoji ❌ Overrides the cancel button label (default "Cancel", applied by the renderer).
    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }

    /// @emoji 🚪 Declares an action dispatched on any dismissal (Escape, veil click, Cancel button).
    pub fn on_cancel(mut self, action: ActionRef) -> Self {
        self.cancel_action = Some(action);
        self
    }
}
//#endregion 🔖Dialog

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    pub label: String,
    /// 🧰 Utilities available while this mode is active — references `AppDefinition.utilities` ids.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub utilities: Vec<UtilityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub layout_id: Option<String>,
    /// 🎛️ Mode-scope commands active while this mode is active — references `AppDefinition.commands`
    /// ids (each of which must declare `scope: CommandScope::Mode`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandRef>,
}

/// 🚫 A non-empty, order-preserving list — construction-time enforcement replaces what used to be a
/// runtime `assert!` deep inside `AppBuilder::build_definition`. The first entry is the implicit
/// fallback default when nothing else specifies one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "Vec<T>", into = "Vec<T>", bound = "T: Clone + Serialize + serde::de::DeserializeOwned")]
pub struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn one(first: T) -> Self {
        Self { first, rest: Vec::new() }
    }

    pub fn new(first: T, rest: Vec<T>) -> Self {
        Self { first, rest }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        std::iter::once(&mut self.first).chain(self.rest.iter_mut())
    }

    pub fn first_mut(&mut self) -> &mut T {
        &mut self.first
    }
}

impl<T> std::ops::Index<usize> for NonEmptyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index == 0 {
            &self.first
        } else {
            &self.rest[index - 1]
        }
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Chain<std::iter::Once<&'a T>, std::slice::Iter<'a, T>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(&self.first).chain(self.rest.iter())
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = String;
    fn try_from(mut values: Vec<T>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            return Err("expected a non-empty list, got zero entries".to_string());
        }
        let first = values.remove(0);
        Ok(Self { first, rest: values })
    }
}

impl<T: Clone> From<NonEmptyVec<T>> for Vec<T> {
    fn from(value: NonEmptyVec<T>) -> Self {
        std::iter::once(value.first).chain(value.rest).collect()
    }
}

/// 🚫 Every app has at least one mode — `protocol/module/procedural` and any other single-purpose app
/// must declare an explicit mode (e.g. `"default"`) instead of the zero-mode state the type system
/// now makes unrepresentable.
pub type Modes = NonEmptyVec<ModeDefinition>;

/// 🚫 Every app has at least one window kind — mirrors `Modes`, formerly a runtime `assert!` in
/// `AppBuilder::build_definition`.
pub type WindowKinds = NonEmptyVec<WindowKindDefinition>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDefinition {
    pub id: String,
    pub label: String,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub icon_id: Option<String>,
    /// 🎛️ Always-present chrome facets (was: separately-optional `measures`/`engagement`).
    #[serde(default)]
    pub options: WindowOptions,
    /// 📇 Actions this window kind accepts — references `AppDefinition.actions` ids. Mandatory,
    /// may be empty, never absent; replaces the previous implicit "every app action applies to
    /// every window" behavior.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionRef>,
    /// 🧰 Utilities this window kind accepts — references `AppDefinition.utilities` ids. Empty = no utilities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub utilities: Vec<UtilityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub params_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub document_projection_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub input_event_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub output_schema: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum PanelGroup {
    Workbench,
    Details,
    Display,
    Settings,
}

impl PanelGroup {
    /// 🧭 The dock anchor this group defaults to. Groups only ever map to the four corner anchors —
    /// the two middle anchors (`top-middle`/`bottom-middle`) start empty and are user-populated via
    /// drag-and-drop or a dock skeleton override, never via a `PanelGroup`.
    pub fn anchor(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "top-left",
            PanelGroup::Details => "top-right",
            PanelGroup::Display => "bottom-left",
            PanelGroup::Settings => "bottom-right",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PanelGroup::Workbench => "workbench",
            PanelGroup::Details => "details",
            PanelGroup::Display => "display",
            PanelGroup::Settings => "settings",
        }
    }
}

/// 🌳 Closes the informal `FRAMEWORK_CATEGORY_*`/`*_TAB_ID` string-constant convention that used to
/// live in the renderer: every panel tab is either a framework-predefined kind (compile-time
/// exhaustive) or an app-declared custom tab (open id, still required to be unique/non-empty,
/// validated at construction by `AppBuilder`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind", content = "id")]
pub enum PanelTabKind {
    WorkbenchCategory,
    DisplayCategory,
    DetailsCategory,
    SettingsCategory,
    DisplayWindows,
    DisplayLayout,
    SettingsGeneral,
    SettingsTheme,
    /// 🧩 App-declared tab — id is app-namespaced (e.g. `"puzzle.catalogue"`).
    App(String),
}

impl PanelTabKind {
    /// 🔤 Flat string key for code that needs one, e.g. React `key=` props.
    pub fn id_str(&self) -> &str {
        match self {
            PanelTabKind::WorkbenchCategory => "framework.category.workbench",
            PanelTabKind::DisplayCategory => "framework.category.display",
            PanelTabKind::DetailsCategory => "framework.category.details",
            PanelTabKind::SettingsCategory => "framework.category.settings",
            PanelTabKind::DisplayWindows => "framework.display.windows",
            PanelTabKind::DisplayLayout => "framework.display.layout",
            PanelTabKind::SettingsGeneral => "framework.settings.general",
            PanelTabKind::SettingsTheme => "framework.settings.theme",
            PanelTabKind::App(id) => id.as_str(),
        }
    }
}

/// 🌳 A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it). Exactly one of the two is set; `group` is only meaningful on root (non-nested) entries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PanelTabDefinition {
    pub kind: PanelTabKind,
    pub label: String,
    pub group: PanelGroup,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub body_key: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PanelTabDefinition>,
}

impl PanelTabDefinition {
    pub fn id(&self) -> &str {
        self.kind.id_str()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct AppDefinition {
    pub id: String,
    pub label: String,
    pub document: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub icon_id: Option<String>,
    pub controller_id: String,
    /// 🚧 `Modes` is `NonEmptyVec<ModeDefinition>`, whose `serde(try_from/into = "Vec<T>")` wire
    /// format is a flat array — not the `{ first, rest }` shape ts-rs would infer from the struct
    /// fields, so the wire-accurate array shape is supplied directly instead of deriving `TS` on
    /// `NonEmptyVec` itself.
    #[cfg_attr(feature = "typegen", ts(type = "ModeDefinition[]"))]
    pub modes: Modes,
    pub default_mode_id: String,
    /// 🚧 See `modes` above — `WindowKinds` is `NonEmptyVec<WindowKindDefinition>`.
    #[cfg_attr(feature = "typegen", ts(type = "WindowKindDefinition[]"))]
    pub window_kinds: WindowKinds,
    pub panel_tabs: Vec<PanelTabDefinition>,
    pub keybindings: Vec<Keybinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionDefinition>,
    /// 🧰 The interactive utilities this app exposes (referenced by `WindowKindDefinition.utilities`/`ModeDefinition.utilities`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub utilities: Vec<UtilityDefinition>,
    /// 🎛️ App- and mode-scope commands this app exposes (referenced by `ModeDefinition.commands` for
    /// `Mode`-scope entries; `App`-scope entries apply whenever the app is focused).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub named_layouts: Vec<NamedLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub default_layout: Option<WindowLayout>,
    /// 🗣️ Terminology ids this app declares beyond the implicit "native" default.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terminologies: Vec<String>,
    /// 🎓 This app's first-run walkthrough, if it declares one — see `IntroductionDefinition`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub introduction: Option<IntroductionDefinition>,
    /// 🗨️ The modal form dialogs this app can open via `HostEffect::OpenDialog`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dialogs: Vec<DialogDefinition>,
}

/// 🧭 Resolves the dock layout a mode should present.
pub fn resolve_layout_for_mode(app: &AppDefinition, mode_id: &str) -> Option<WindowLayout> {
    let mode = app.modes.iter().find(|mode| mode.id == mode_id)?;
    if let Some(layout_id) = &mode.layout_id {
        if let Some(named) = app.named_layouts.iter().find(|entry| entry.id == *layout_id) {
            return Some(named.layout.clone());
        }
    }
    app.default_layout.clone()
}

//#region 🔖action-args
/// @emoji 🧮 Computes the effective argument map for an action: for each declared arg, the staged value
/// if present, else its declared `default`, else omitted. Renderers stage edits locally and pass them
/// here; the contract enforcer ({@link VcsDocumentApp}) materializes defaults before dispatch so plugins
/// never re-implement default-filling.
pub fn effective_action_args(
    defs: &[ActionArgDef],
    staged: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut effective = serde_json::Map::new();
    for def in defs {
        if let Some(value) = staged.get(&def.id) {
            effective.insert(def.id.clone(), value.clone());
        } else if let Some(default) = &def.default {
            effective.insert(def.id.clone(), default.clone());
        }
    }
    effective
}

/// @emoji ❗ Returns the ids of required args that are still unset in `effective`. "Unset" means absent,
/// JSON `null`, or an empty string (covers a blank Text/Select/IconSelect); `false`, `0`, and `[]` are
/// valid values for Toggle/Number/Slider/Vec3 and never count as unset.
pub fn missing_required_args(
    defs: &[ActionArgDef],
    effective: &serde_json::Map<String, serde_json::Value>,
) -> Vec<String> {
    defs.iter()
        .filter(|def| def.required)
        .filter(|def| match effective.get(&def.id) {
            None | Some(serde_json::Value::Null) => true,
            Some(serde_json::Value::String(text)) => text.is_empty(),
            Some(_) => false,
        })
        .map(|def| def.id.clone())
        .collect()
}

/// @emoji 🚦 Whether an action is eligible to appear in a window's Actions panel — excludes the six
/// framework History actions (rendered by the History rail) and the injected `setActiveUtility` (an
/// internal View action wired to the toolbar, never the panel).
fn action_is_panel_eligible(action: &ActionDefinition) -> bool {
    action.kind != ActionKind::History && action.id != SET_ACTIVE_UTILITY_ACTION_ID
}

/// @emoji 📇 Resolves the actions a window kind presents in its panel. Explicit `window_kind.actions`
/// refs resolve in declared order; additionally, any panel-eligible app action referenced by *no*
/// window kind is an "orphan" that appears on every window (the scoping fallback that prevents blank
/// panels mid-migration — Architecture Decision 8). A window that scopes nothing therefore shows every
/// orphan; once a plugin scopes an action to some window, it stops being an orphan and appears only
/// where scoped. Unresolvable refs are skipped (the builder validates them at construction time).
pub fn resolve_window_actions<'a>(
    app: &'a AppDefinition,
    window_kind: &WindowKindDefinition,
) -> Vec<&'a ActionDefinition> {
    let referenced: std::collections::HashSet<&str> = app
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter().map(ActionRef::as_str))
        .collect();
    let mut resolved: Vec<&'a ActionDefinition> = Vec::new();
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for action_ref in &window_kind.actions {
        if let Some(action) = app.actions.iter().find(|action| action.id == action_ref.as_str()) {
            if seen.insert(action.id.as_str()) {
                resolved.push(action);
            }
        }
    }
    for action in &app.actions {
        if action_is_panel_eligible(action)
            && !referenced.contains(action.id.as_str())
            && seen.insert(action.id.as_str())
        {
            resolved.push(action);
        }
    }
    resolved
}
//#endregion 🔖action-args

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ProgramDefinition {
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub document: Vec<String>,
    pub yields: String,
}

/// 🪜 Formats a canonical app document for chrome.
pub fn app_document_label(document: &[String]) -> String {
    document.join(" · ")
}

/// 🗂️ Formats a window tab within its canonical app document.
pub fn app_window_document_label(app: &AppDefinition, window_label: &str) -> String {
    let mut document = app.document.clone();
    let normalized_window = window_label.trim().to_lowercase();
    let normalized_app = app.label.trim().to_lowercase();
    if !normalized_window.is_empty()
        && normalized_window != normalized_app
        && document.last().is_none_or(|segment| segment.to_lowercase() != normalized_window)
    {
        document.push(normalized_window);
    }
    app_document_label(&document)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub id: String,
    pub label: String,
    pub document_json: String,
    pub app_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Contribution {
    /// 🧩 A module contributing an extension block kind to a protocol-list (Blockly-like) builder host app.
    ProtocolBlockKind {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "blockKind"))]
        block_kind: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        #[cfg_attr(feature = "typegen", ts(rename = "defaultValueJson"))]
        default_value_json: String,
        #[cfg_attr(feature = "typegen", ts(rename = "paramsBodyKey"))]
        params_body_key: String,
        #[cfg_attr(feature = "typegen", ts(rename = "previewBodyKey"))]
        preview_body_key: String,
    },
    /// 🧩 A sourcing module contributing a typology tree and catalogue object kinds to a sourcing host app.
    SourcingModule {
        #[cfg_attr(feature = "typegen", ts(rename = "appId"))]
        app_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "moduleId"))]
        module_id: String,
        label: String,
        #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
        icon_id: String,
        #[cfg_attr(feature = "typegen", ts(rename = "typologyJson"))]
        typology_json: String,
        #[cfg_attr(feature = "typegen", ts(rename = "kindsJson"))]
        kinds_json: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub plugin_id: String,
    pub label: String,
    pub version: String,
    pub apps: Vec<AppDefinition>,
    pub programs: Vec<ProgramDefinition>,
    pub examples: Vec<ExampleDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<kernel::CapabilityRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributions: Vec<Contribution>,
    /// 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandDefinition>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ViewState {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_mode_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_window_kind_id: Option<String>,
    /// 🧰 The host-owned active utility for the active window kind (never a document field, never a VCS op).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub active_utility_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub panel_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub contributions_json: Option<String>,
    /// 🗣️ Active UI locale (e.g. "en", "de"); plugins resolve their own label set from this.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub locale: Option<String>,
    /// 🗣️ Active terminology id ("native" default, or an app-declared alternative term set).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub terminology: Option<String>,
}

/// 🗣️ Locale/terminology-aware label patch for an already-instantiated app's manifest, resolved fresh per `ViewState`
/// (unlike `AppDefinition`, which is assembled once at plugin-load time and cannot itself react to locale changes).
/// The shell merges this over the static `AppDefinition` labels by id; ids absent from a map keep their static English label.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct AppLabelsOverlay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub app_label: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub window_kind_labels: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub panel_tab_labels: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mode_labels: std::collections::HashMap<String, String>,
    /// 🗣️ Locale-aware overrides for `AppDefinition.actions[].label` (operations/view-actions/shell-actions), keyed by action id — covers the command palette and any other UI surfacing an action's static English label.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub action_labels: std::collections::HashMap<String, String>,
    /// 🗣️ Locale-aware overrides for `AppDefinition.utilities[].label` (toolbar tools), keyed by utility id.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub utility_labels: std::collections::HashMap<String, String>,
    /// 🗣️ Locale-aware overrides for `AppDefinition.examples[].label` (example/fixture picker), keyed by example id.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub example_labels: std::collections::HashMap<String, String>,
    /// 🗣️ Locale-aware overrides for action-arg labels and their select-option labels, keyed `"{actionId}.{argId}"` and `"{actionId}.{argId}.option.{value}"`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub action_arg_labels: std::collections::HashMap<String, String>,
    /// 🗣️ Locale-aware overrides for `DialogDefinition` text, keyed `"{dialogId}.title"` / `".body"` / `".submit"`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub dialog_labels: std::collections::HashMap<String, String>,
    /// 🗣️ Locale-aware overrides for `IntroductionDefinition` text, keyed `"intro.title"` / `"intro.step.{stepId}.title"` / `".body"`.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub introduction_labels: std::collections::HashMap<String, String>,
}

impl AppLabelsOverlay {
    /// 🗣️ Starts an overlay pre-populated with the well-known framework panel-tab labels (Document/Catalogue/Inspection/Parameters) for every panel tab id supplied; apps then extend it with their own window-kind/mode labels.
    pub fn with_framework_panel_tabs(panel_tab_ids: impl IntoIterator<Item = impl Into<String>>, is_de: bool) -> Self {
        let mut overlay = Self::default();
        for id in panel_tab_ids {
            let id = id.into();
            if let Some(label) = ui_wgpu::framework_panel_tab_label(&id, is_de) {
                overlay.panel_tab_labels.insert(id, label.into());
            }
        }
        overlay
    }
}

//#region 🔖Kernel
pub mod kernel {
//! 🧠 Local-first action kernel contracts: actions, operations, capabilities, window I/O.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ui_wgpu::UiNode;

//#region 🔖Identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AssetHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityToken(pub u128);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PluginInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationId(pub String);

/// 🪪 Identifies one dispatched invocation — of an action *or* a command; both route through the same
/// `KernelOperation`/`UndoGroup` history bookkeeping.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InvocationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AppInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActorId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaId(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentVersion(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowKindId(pub String);
//#endregion 🔖Identifiers

//#region 🔖HybridLogicalTimestamp
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HybridLogicalTimestamp {
    pub actor: u64,
    pub physical_ms: u64,
    pub logical: u64,
}

impl HybridLogicalTimestamp {
    pub fn new(actor: u64, physical_ms: u64) -> Self {
        Self {
            actor,
            physical_ms,
            logical: 0,
        }
    }

    pub fn tick(&mut self, physical_ms: u64) {
        if physical_ms > self.physical_ms {
            self.physical_ms = physical_ms;
            self.logical = 0;
        } else {
            self.logical = self.logical.saturating_add(1);
        }
    }

    pub fn merge(&mut self, other: &Self) {
        if other.physical_ms > self.physical_ms {
            self.physical_ms = other.physical_ms;
            self.logical = other.logical;
        } else if other.physical_ms == self.physical_ms && other.logical > self.logical {
            self.logical = other.logical;
        }
        self.logical = self.logical.saturating_add(1);
    }

    pub fn cmp_key(&self) -> (u64, u64) {
        (self.physical_ms, self.logical)
    }
}
//#endregion 🔖HybridLogicalTimestamp

//#region 🔖Capability
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Rights {
    Read,
    Write,
    Invoke,
    Open,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ResourceKind {
    Document,
    Projection,
    Window,
    Asset,
    Network,
    Backbone,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Scope {
    Instance,
    App,
    Plugin,
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CapabilityRequirement {
    pub resource: ResourceKind,
    pub rights: Rights,
    pub scope: Scope,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub subject: PluginInstanceId,
    pub resource: ResourceId,
    pub rights: Rights,
    pub scope: Scope,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityGrant {
    pub token: CapabilityToken,
    pub capability: Capability,
}
//#endregion 🔖Capability

//#region 🔖Invocation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDef {
    pub id: ActionId,
    pub input_schema: SchemaId,
    pub output_schema: SchemaId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_capabilities: Vec<CapabilityRequirement>,
    pub deterministic: bool,
    pub produces_operations: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub id: InvocationId,
    pub app: AppInstanceId,
    pub action: ActionId,
    pub input: Value,
    pub actor: ActorId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causal_context: Vec<OperationId>,
}

/// @emoji 🎛️ A dispatched invocation of a `CommandDefinition` — the command mirror of `ActionInvocation`.
/// No `causal_context`: commands are not chained off a prior op the way an action's follow-up can be.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandInvocation {
    pub id: InvocationId,
    pub app: AppInstanceId,
    pub command: CommandId,
    pub input: Value,
    pub actor: ActorId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub level: String,
    pub message: String,
}

// 🪪 `rename_all` on an enum only renames variant tags ("setActiveUtility"), not the fields *inside* each
// struct-variant — those need `rename_all_fields` (serde 1.0.126+) or every multi-word field here
// (window_kind_id, mime_type, program_id, ...) silently serializes as snake_case, breaking any TS side
// that destructures camelCase (confirmed live: `SetActiveUtility` was shipping `window_kind_id`/`utility_id`,
// so the host-owned utility switch after `openVortexSuggestions` never applied and the brush preview never
// rendered).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum HostEffect {
    OpenWindow { kind: WindowKindId, params: Value },
    CloseWindow { window: WindowHandle },
    Notify { message: String },
    RequestSync,
    /// @emoji 🧭 Navigates the shell to a URI (studio/instance/document route).
    Navigate { uri: String },
    /// @emoji 🗂️ Replaces the active studio/window panel state with a serialized panel JSON.
    SetPanel { panel_json: String },
    /// @emoji ⬇️ Downloads an in-memory media export as a file (base64 or utf-8 `data`).
    DownloadMediaExport {
        filename: String,
        mime_type: String,
        data: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
    },
    /// @emoji 🖼️ Renders one or more icon-scene requests to images and downloads each.
    IconRenderExport { items: Vec<IconRenderExportItem> },
    /// @emoji 📤 Asks the shell to open a file picker and re-dispatch `import_action` with the
    /// picked file's contents as `{ payload, name }` args.
    RequestFileOpen {
        accept: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        read_as: Option<String>,
        import_action: String,
    },
    /// @emoji ✨ Spawns a plugin instance (idempotent on `os_instance_id`) without focusing it.
    SpawnPluginInstance {
        program_id: String,
        app_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        os_instance_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        document_json: Option<String>,
    },
    /// @emoji 🪟 Spawns (if needed) and focuses/navigates to a plugin instance.
    OpenPluginInstance {
        program_id: String,
        app_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        os_instance_id: Option<String>,
    },
    /// @emoji 🧰 Programmatically switches the host-owned active utility of a window kind — the effect
    /// form of `setActiveUtility`, letting a plugin change utilities without a user click.
    SetActiveUtility { window_kind_id: String, utility_id: String },
    /// @emoji 🗨️ Opens a declared `AppDefinition.dialogs` entry; `args` (an object keyed by arg id)
    /// pre-seeds the staged form. Kernel-altitude — plain `String`/`Value`, no manifest types.
    OpenDialog {
        dialog_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        args: Option<Value>,
    },
}

/// @emoji 🖼️ One icon-render export request: the destination filename plus the opaque icon-scene
/// render request forwarded to the shell's `iconRenderPort`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconRenderExportItem {
    pub filename: String,
    pub request: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppEvent {
    pub kind: String,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDiff {
    pub schema_id: SchemaId,
    pub payload: Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UndoPolicy {
    ExactBaseOnly,
    TransformAgainstConcurrent,
    SemanticUndo,
    CompensatingAction,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseOperation {
    pub target_operation: OperationId,
    pub inverse_diff: DocumentDiff,
    pub base_version: DocumentVersion,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<OperationId>,
    pub undo_policy: UndoPolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelOperation {
    pub id: OperationId,
    pub document: DocumentHandle,
    pub base_version: DocumentVersion,
    pub invocation_id: InvocationId,
    pub diff: DocumentDiff,
    pub inverse: InverseOperation,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<OperationId>,
    pub author: ActorId,
    pub timestamp: HybridLogicalTimestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoGroup {
    pub invocation_id: InvocationId,
    pub operations: Vec<OperationId>,
    pub inverse_operations: Vec<InverseOperation>,
}

/// @emoji 🐢 What part of the shell's rendered UI an action actually invalidates — lets `refresh-ui`
/// skip re-rendering/re-fetching sections nothing touched. Absent from JSON (older/unmodified plugins)
/// deserializes to `Full`, so any plugin that never sets this keeps today's whole-shell-refresh
/// behavior exactly. `None` means "nothing to re-render at all" (e.g. a pure telemetry/heartbeat action).
// 🐢 `rename_all = "camelCase"` alone only renames the *variant* names (Full/None/Partial ->
// full/none/partial via `tag = "kind"`) — it does NOT cascade into a struct variant's own fields, which
// would otherwise serialize as snake_case (`window_bodies`) and silently desync from the TS
// `UiDirtyScope` type's camelCase `windowBodies`. `rename_all_fields` is the attribute that renames
// fields *within* variants; both are needed together.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum UiDirtyScope {
    #[default]
    Full,
    None,
    Partial {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        window_bodies: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        panel_bodies: Vec<String>,
        #[serde(default)]
        utilities: bool,
        #[serde(default)]
        engagements: bool,
        #[serde(default)]
        measures: bool,
        #[serde(default)]
        labels: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvocationResult {
    pub output: Value,
    pub operations: Vec<KernelOperation>,
    pub inverse_group: UndoGroup,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_effects: Vec<HostEffect>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<AppEvent>,
    #[serde(default)]
    pub ui_scope: UiDirtyScope,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionContext {
    pub invocation: ActionInvocation,
    pub document_projection: Value,
    pub view_state: super::ViewState,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub granted_capabilities: Vec<CapabilityGrant>,
}

/// @emoji 🎛️ Context for a dispatched `CommandInvocation` — the command mirror of `ActionContext`.
/// No `document_projection`/`granted_capabilities`: `VcsDocumentApp` owns the store directly and
/// commands don't yet carry a capability grant model (mirrors actions' current state).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandContext {
    pub invocation: CommandInvocation,
    pub view_state: super::ViewState,
}
//#endregion 🔖Invocation

//#region 🔖Sync
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PayloadHash(pub String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpEnvelope {
    pub id: OperationId,
    pub actor: ActorId,
    pub document: DocumentId,
    pub schema_version: SchemaVersion,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deps: Vec<OperationId>,
    pub payload_hash: PayloadHash,
    pub diff: DocumentDiff,
    pub inverse: InverseOperation,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OpDagError {
    #[error("duplicate operation id: {0}")]
    Duplicate(String),
}

/// @emoji 🕸️ Causal DAG of exchanged {@link OpEnvelope}s: buffers envelopes until their deps are applied.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpDag {
    envelopes: std::collections::HashMap<String, OpEnvelope>,
    applied: std::collections::HashSet<String>,
    applied_order: Vec<String>,
    drained: usize,
    pending: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InsertResult {
    Applied,
    Pending,
    AlreadyApplied,
}

impl OpDag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, envelope: OpEnvelope) -> Result<InsertResult, OpDagError> {
        let id = envelope.id.0.clone();
        if self.applied.contains(&id) {
            return Ok(InsertResult::AlreadyApplied);
        }
        if self.envelopes.contains_key(&id) {
            return Err(OpDagError::Duplicate(id));
        }
        for dependency in &envelope.deps {
            if !self.applied.contains(&dependency.0) && !self.envelopes.contains_key(&dependency.0) {
                self.envelopes.insert(id.clone(), envelope);
                if !self.pending.contains(&id) {
                    self.pending.push(id);
                }
                return Ok(InsertResult::Pending);
            }
        }
        self.envelopes.insert(id.clone(), envelope);
        self.mark_applied(&id);
        self.drain_ready();
        Ok(InsertResult::Applied)
    }

    pub fn ready(&self) -> Vec<&OpEnvelope> {
        self.pending
            .iter()
            .filter_map(|id| self.envelopes.get(id))
            .filter(|envelope| {
                envelope
                    .deps
                    .iter()
                    .all(|dependency| self.applied.contains(&dependency.0))
            })
            .collect()
    }

    pub fn applied_ids(&self) -> Vec<String> {
        self.applied.iter().cloned().collect()
    }

    /// @emoji 🧺 Drains envelopes applied since the last drain, in causal application order.
    pub fn drain_applied_envelopes(&mut self) -> Vec<OpEnvelope> {
        let fresh: Vec<String> = self.applied_order[self.drained..].to_vec();
        self.drained = self.applied_order.len();
        fresh
            .iter()
            .filter_map(|id| self.envelopes.get(id).cloned())
            .collect()
    }

    fn mark_applied(&mut self, id: &str) {
        self.applied.insert(id.to_string());
        self.applied_order.push(id.to_string());
        self.pending.retain(|pending| pending != id);
    }

    fn drain_ready(&mut self) {
        loop {
            let ready: Vec<String> = self
                .pending
                .iter()
                .filter(|id| {
                    self.envelopes
                        .get(*id)
                        .is_some_and(|envelope| {
                            envelope
                                .deps
                                .iter()
                                .all(|dependency| self.applied.contains(&dependency.0))
                        })
                })
                .cloned()
                .collect();
            if ready.is_empty() {
                break;
            }
            for id in ready {
                self.mark_applied(&id);
            }
        }
    }
}

//#region 🔖HubProtocol
/// @emoji 📡 Presence roster entry broadcast to every peer connected to a document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresencePeer {
    pub actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    pub connected_at_ms: i64,
    /// @emoji 🪪 Authenticated hub user id, when this peer connected with an `AuthSession` rather than an anonymous share token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// @emoji 🎚️ The peer's resolved studio role (`"owner"`/`"member"`/`"viewer"`), present alongside `user_id`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// @emoji 📨 Client→server hub wire frames; the counterpart is {@link HubServerFrame}.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum HubClientFrame {
    Hello {
        actor: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        token: Option<String>,
        since_version: i64,
    },
    Ops {
        envelopes: Vec<OpEnvelope>,
    },
    PutEnvelope {
        version: i64,
        envelope: Value,
    },
    Presence {
        peer: PresencePeer,
    },
    Bye,
}

/// @emoji 📬 Server→client hub wire frames; the counterpart is {@link HubClientFrame}.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum HubServerFrame {
    Welcome {
        version: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        envelope: Option<Value>,
        presence: Vec<PresencePeer>,
        backlog: Vec<OpEnvelope>,
    },
    Ops {
        version: i64,
        envelopes: Vec<OpEnvelope>,
        origin: String,
    },
    SnapshotReplaced {
        version: i64,
        envelope: Value,
    },
    Presence {
        peers: Vec<PresencePeer>,
    },
    Ack {
        op_id: String,
        version: i64,
    },
    Conflict {
        message: String,
    },
    Error {
        message: String,
    },
}
//#endregion 🔖HubProtocol

#[cfg(test)]
mod op_dag_tests {
    use super::*;

    fn sample_envelope(id: &str, deps: Vec<&str>) -> OpEnvelope {
        OpEnvelope {
            id: OperationId(id.into()),
            actor: ActorId("actor-1".into()),
            document: DocumentId("document-1".into()),
            schema_version: SchemaVersion("test.v1".into()),
            deps: deps.into_iter().map(|dep| OperationId(dep.into())).collect(),
            payload_hash: PayloadHash("hash".into()),
            diff: DocumentDiff {
                schema_id: SchemaId("diff.v1".into()),
                payload: serde_json::json!({"value": id}),
            },
            inverse: InverseOperation {
                target_operation: OperationId(id.into()),
                inverse_diff: DocumentDiff {
                    schema_id: SchemaId("diff.v1".into()),
                    payload: serde_json::json!({}),
                },
                base_version: DocumentVersion(0),
                dependencies: Vec::new(),
                undo_policy: UndoPolicy::ExactBaseOnly,
            },
        }
    }

    #[test]
    fn inserts_pending_until_dependencies_arrive() {
        let mut dag = OpDag::new();
        assert!(matches!(
            dag.insert(sample_envelope("op-2", vec!["op-1"])),
            Ok(InsertResult::Pending)
        ));
        assert!(matches!(
            dag.insert(sample_envelope("op-1", vec![])),
            Ok(InsertResult::Applied)
        ));
        assert_eq!(dag.applied_ids().len(), 2);
    }

    #[test]
    fn drains_applied_envelopes_in_causal_order() {
        let mut dag = OpDag::new();
        dag.insert(sample_envelope("op-2", vec!["op-1"])).unwrap();
        dag.insert(sample_envelope("op-1", vec![])).unwrap();
        let drained = dag.drain_applied_envelopes();
        assert_eq!(
            drained.iter().map(|envelope| envelope.id.0.clone()).collect::<Vec<_>>(),
            vec!["op-1".to_string(), "op-2".to_string()]
        );
        assert!(dag.drain_applied_envelopes().is_empty(), "second drain yields nothing new");
        dag.insert(sample_envelope("op-3", vec![])).unwrap();
        let drained = dag.drain_applied_envelopes();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].id.0, "op-3");
    }
}
//#endregion 🔖Sync

//#region 🔖Window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Appearance {
    pub mode: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEvent {
    pub kind: String,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionRequest {
    pub invocation: ActionInvocation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDef {
    pub id: WindowKindId,
    pub params_schema: SchemaId,
    pub document_projection_schema: SchemaId,
    pub input_event_schema: SchemaId,
    pub output_schema: SchemaId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<CapabilityRequirement>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowInput {
    pub window: WindowHandle,
    pub params: Value,
    pub document_projection: Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<WindowEvent>,
    pub size: PhysicalSize,
    pub scale_factor: f64,
    pub appearance: Appearance,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOutput {
    pub ui: UiNode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionRequest>,
}
//#endregion 🔖Window

//#region 🔖MergeStrategy
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DocumentKind {
    PlainRecord,
    OrderedSequence,
    TextSequence,
    TombstonedGraph,
    ContentAddressedBlob,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MergeStrategyKind {
    LwwRegister,
    OrderedSequence,
    TextSequence,
    TombstonedGraphSet,
    ContentAddressedBlob,
}

impl DocumentKind {
    pub fn merge_strategy(&self) -> MergeStrategyKind {
        match self {
            DocumentKind::PlainRecord => MergeStrategyKind::LwwRegister,
            DocumentKind::OrderedSequence => MergeStrategyKind::OrderedSequence,
            DocumentKind::TextSequence => MergeStrategyKind::TextSequence,
            DocumentKind::TombstonedGraph => MergeStrategyKind::TombstonedGraphSet,
            DocumentKind::ContentAddressedBlob => MergeStrategyKind::ContentAddressedBlob,
        }
    }
}
//#endregion 🔖MergeStrategy
}
//#endregion 🔖Kernel

#[cfg(test)]
mod app_document_tests {
    use super::app_document_label;

    //#region 🔖UiDirtyScopeTests
    /// 🐢 Regression: `rename_all = "camelCase"` on an enum only renames *variant* names via `tag`, not
    /// the fields inside a struct variant — those need `rename_all_fields` too, or `Partial`'s fields
    /// silently serialize as snake_case (`window_bodies`) while the TS `UiDirtyScope` type expects
    /// camelCase (`windowBodies`), desyncing the wire contract without any compile-time signal.
    #[test]
    fn ui_dirty_scope_partial_serializes_fields_as_camel_case() {
        use crate::kernel::UiDirtyScope;
        let scope = UiDirtyScope::Partial {
            window_bodies: vec!["a".into()],
            panel_bodies: vec!["b".into()],
            utilities: true,
            engagements: true,
            measures: false,
            labels: false,
        };
        let json = serde_json::to_string(&scope).unwrap();
        assert!(json.contains("\"windowBodies\""), "{json}");
        assert!(json.contains("\"panelBodies\""), "{json}");
        assert!(!json.contains("window_bodies"), "{json}");
        assert!(!json.contains("panel_bodies"), "{json}");
    }

    #[test]
    fn ui_dirty_scope_defaults_to_full() {
        use crate::kernel::UiDirtyScope;
        assert_eq!(UiDirtyScope::default(), UiDirtyScope::Full);
        assert_eq!(serde_json::to_string(&UiDirtyScope::Full).unwrap(), "{\"kind\":\"full\"}");
        // Absent from JSON (an older plugin that never sets it) must also deserialize to Full.
        #[derive(serde::Deserialize)]
        struct Wrapper {
            #[serde(default)]
            ui_scope: UiDirtyScope,
        }
        let parsed: Wrapper = serde_json::from_str("{}").unwrap();
        assert_eq!(parsed.ui_scope, UiDirtyScope::Full);
    }
    //#endregion UiDirtyScopeTests

    #[test]
    fn formats_app_document_for_chrome() {
        assert_eq!(
            app_document_label(&["semio".into(), "puzzle".into(), "3d".into()]),
            "semio · puzzle · 3d"
        );
    }

    //#region 🔖ActionArgsAndUtilitiesTests
    use crate::ui::{
        effective_action_args, missing_required_args, resolve_window_actions, ActionArgControl, ActionArgDef,
        ActionArgOption, ActionDefinition, ActionKind, ActionRef, AppDefinition, CommandDefinition, CommandRef,
        CommandScope, DialogDefinition, IntroductionAdvance,
        IntroductionAnchor, Modes, UtilityDefinition, UtilityRef, WindowKindDefinition, WindowKinds,
        SET_ACTIVE_UTILITY_ACTION_ID,
    };
    use crate::ui::kernel::HostEffect;
    use serde_json::json;

    #[test]
    fn action_arg_def_builder_chain() {
        let arg = ActionArgDef::slider("scale", "Scale", 0.0, 4.0)
            .required()
            .default_value(1.0)
            .describe("scale factor");
        assert_eq!(arg.id, "scale");
        assert!(arg.required);
        assert_eq!(arg.default, Some(json!(1.0)));
        assert_eq!(arg.description.as_deref(), Some("scale factor"));
        assert!(matches!(arg.control, ActionArgControl::Slider { min, max, .. } if min == 0.0 && max == 4.0));
    }

    #[test]
    fn effective_args_prefer_staged_then_default() {
        let defs = vec![
            ActionArgDef::text("a", "A").default_value("da"),
            ActionArgDef::text("b", "B").default_value("db"),
            ActionArgDef::text("c", "C"),
        ];
        let mut staged = serde_json::Map::new();
        staged.insert("a".into(), json!("staged-a"));
        let effective = effective_action_args(&defs, &staged);
        assert_eq!(effective.get("a"), Some(&json!("staged-a")), "staged wins");
        assert_eq!(effective.get("b"), Some(&json!("db")), "default fills in");
        assert!(!effective.contains_key("c"), "no staged, no default ⇒ omitted");
    }

    #[test]
    fn missing_required_args_treats_unset_select_as_missing() {
        let defs = vec![
            ActionArgDef::select("mode", "Mode", vec![ActionArgOption::new("x", "X")]).required(),
            ActionArgDef::toggle("flag", "Flag").required(),
        ];
        // Nothing staged, no defaults: both required ids are missing.
        let empty = serde_json::Map::new();
        let effective = effective_action_args(&defs, &empty);
        let missing = missing_required_args(&defs, &effective);
        assert!(missing.contains(&"mode".to_string()));
        assert!(missing.contains(&"flag".to_string()));

        // An empty-string select value still counts as unset; `false` for a toggle is a real value.
        let mut effective = serde_json::Map::new();
        effective.insert("mode".into(), json!(""));
        effective.insert("flag".into(), json!(false));
        let missing = missing_required_args(&defs, &effective);
        assert_eq!(missing, vec!["mode".to_string()], "empty-string select is unset; false toggle is set");
    }

    #[test]
    fn utility_definition_and_utility_ref_construction() {
        let utility = UtilityDefinition::new("brush", "Brush", "icon.brush");
        assert_eq!(utility.id, "brush");
        assert!(!utility.allows_actions_while_active, "default gates actions while active");
        assert_eq!(UtilityRef::new("brush").as_str(), "brush");
        assert_eq!(UtilityRef::from("brush").as_str(), "brush");
    }

    fn app_with(actions: Vec<ActionDefinition>, window_actions: Vec<ActionRef>) -> AppDefinition {
        AppDefinition {
            id: "a".into(),
            label: "A".into(),
            document: vec!["semio".into(), "a".into()],
            icon_id: None,
            controller_id: "a".into(),
            modes: Modes::one(crate::ui::ModeDefinition {
                id: "edit".into(),
                label: "Edit".into(),
                utilities: Vec::new(),
                layout_id: None,
                commands: Vec::new(),
            }),
            default_mode_id: "edit".into(),
            window_kinds: WindowKinds::one(WindowKindDefinition {
                id: "main".into(),
                label: "Main".into(),
                body_key: "a.main".into(),
                surface_kind: ui_wgpu::SurfaceKind::Canvas2d,
                icon_id: None,
                options: ui_wgpu::WindowOptions::default(),
                actions: window_actions,
                utilities: Vec::new(),
                params_schema: None,
                document_projection_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            actions,
            utilities: vec![],
            commands: vec![],
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            introduction: None,
            dialogs: Vec::new(),
        }
    }

    #[test]
    fn resolve_window_actions_explicit_scoping() {
        let app = app_with(
            vec![
                ActionDefinition::new("add", "Add", ActionKind::Operation),
                ActionDefinition::new("remove", "Remove", ActionKind::Operation),
            ],
            vec![ActionRef::new("add")],
        );
        let window = app.window_kinds.first();
        let resolved: Vec<&str> = resolve_window_actions(&app, window).iter().map(|a| a.id.as_str()).collect();
        // `add` is explicitly scoped here; `remove` is referenced by no window ⇒ orphan ⇒ also appears.
        assert_eq!(resolved, vec!["add", "remove"]);
    }

    #[test]
    fn resolve_window_actions_excludes_history_and_set_active_utility_orphans() {
        let app = app_with(
            vec![
                ActionDefinition::new("undo", "Undo", ActionKind::History),
                crate::ui::set_active_utility_action_definition(),
                ActionDefinition::new("add", "Add", ActionKind::Operation),
            ],
            vec![],
        );
        let window = app.window_kinds.first();
        let resolved: Vec<&str> = resolve_window_actions(&app, window).iter().map(|a| a.id.as_str()).collect();
        assert_eq!(resolved, vec!["add"], "history + setActiveUtility are never panel-eligible orphans");
        assert!(!resolved.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
    }

    #[test]
    fn action_definition_deserializes_without_args_field() {
        // Forward-compat: legacy JSON with no `args`/`category` still deserializes with empty defaults.
        let action: ActionDefinition =
            serde_json::from_str(r#"{"id":"x","label":"X","kind":"operation","inPalette":true}"#).unwrap();
        assert!(action.args.is_empty());
    }

    #[test]
    fn window_kind_deserializes_without_utilities_field() {
        let window: WindowKindDefinition = serde_json::from_str(
            r#"{"id":"main","label":"Main","bodyKey":"a.main","surfaceKind":"canvas-2d"}"#,
        )
        .unwrap();
        assert!(window.utilities.is_empty());
        assert!(window.actions.is_empty());
    }

    #[test]
    fn action_arg_control_serializes_tagged() {
        let control = ActionArgControl::Select { options: vec![ActionArgOption::new("x", "X")] };
        let json = serde_json::to_string(&control).unwrap();
        assert!(json.contains("\"kind\":\"select\""), "tagged with kind: {json}");
        let round: ActionArgControl = serde_json::from_str(&json).unwrap();
        assert_eq!(round, control);
    }

    #[test]
    fn introduction_anchor_round_trips_tagged() {
        for anchor in [
            IntroductionAnchor::Screen,
            IntroductionAnchor::Navbar,
            IntroductionAnchor::WindowKind("main".into()),
            IntroductionAnchor::Utility(UtilityRef::new("brush")),
            IntroductionAnchor::Action(ActionRef::new("add")),
            IntroductionAnchor::PanelTab("puzzle.catalogue".into()),
            IntroductionAnchor::Slot("navbar".into()),
        ] {
            let json = serde_json::to_string(&anchor).unwrap();
            assert!(json.contains("\"kind\":"), "tagged with kind: {json}");
            let round: IntroductionAnchor = serde_json::from_str(&json).unwrap();
            assert_eq!(round, anchor);
        }
    }

    #[test]
    fn introduction_advance_round_trips_tagged() {
        for advance in [
            IntroductionAdvance::Next,
            IntroductionAdvance::Action(ActionRef::new("add")),
            IntroductionAdvance::Utility(UtilityRef::new("brush")),
        ] {
            let json = serde_json::to_string(&advance).unwrap();
            let round: IntroductionAdvance = serde_json::from_str(&json).unwrap();
            assert_eq!(round, advance);
        }
        assert_eq!(IntroductionAdvance::default(), IntroductionAdvance::Next);
    }

    #[test]
    fn dialog_definition_round_trips_camel_case_with_defaults() {
        let dialog = DialogDefinition::new("confirm-delete", "Delete?", ActionRef::new("deleteSelection"));
        let json = serde_json::to_string(&dialog).unwrap();
        assert!(json.contains("\"submitAction\":\"deleteSelection\""), "{json}");
        assert!(json.contains("\"submitLabel\":\"OK\""), "{json}");
        assert!(!json.contains("cancelAction"), "omitted when unset: {json}");
        let round: DialogDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(round, dialog);
    }

    #[test]
    fn dialog_definition_builder_chain() {
        let dialog = DialogDefinition::new("addObject", "Add Object", ActionRef::new("addObjectKind"))
            .body("Choose a kind")
            .args(vec![ActionArgDef::text("objectKind", "Kind")])
            .submit_label("Add")
            .cancel_label("Nevermind")
            .on_cancel(ActionRef::new("closeDialog"));
        assert_eq!(dialog.body.as_deref(), Some("Choose a kind"));
        assert_eq!(dialog.args.len(), 1);
        assert_eq!(dialog.submit_label, "Add");
        assert_eq!(dialog.cancel_label.as_deref(), Some("Nevermind"));
        assert_eq!(dialog.cancel_action, Some(ActionRef::new("closeDialog")));
    }

    #[test]
    fn command_definition_round_trips_camel_case_with_defaults() {
        let command = CommandDefinition::new("os.setThemeId", "Set Theme", CommandScope::Os, "appearance")
            .with_args(vec![ActionArgDef::text("themeId", "Theme").required()]);
        let json = serde_json::to_string(&command).unwrap();
        assert!(json.contains("\"scope\":\"os\""), "{json}");
        assert!(json.contains("\"category\":\"appearance\""), "{json}");
        assert!(json.contains("\"inPalette\":true"), "{json}");
        assert!(!json.contains("iconId"), "omitted when unset: {json}");
        assert!(!json.contains("keys"), "omitted when unset: {json}");
        let round: CommandDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(round, command);
    }

    #[test]
    fn command_ref_only_resolves_mode_scope_commands() {
        // 🎛️ CommandScope has no Ord/discriminant helper beyond PartialEq — this pins the four
        // variants' camelCase wire tags so a future variant addition can't silently reorder them.
        for (scope, tag) in [
            (CommandScope::Os, "\"os\""),
            (CommandScope::Plugin, "\"plugin\""),
            (CommandScope::App, "\"app\""),
            (CommandScope::Mode, "\"mode\""),
        ] {
            assert_eq!(serde_json::to_string(&scope).unwrap(), tag);
        }
        assert_eq!(CommandRef::new("mode.focus").as_str(), "mode.focus");
        assert_eq!(CommandRef::from("mode.focus").as_str(), "mode.focus");
    }

    #[test]
    fn open_dialog_effect_round_trips_camel_case() {
        let effect = HostEffect::OpenDialog { dialog_id: "addObject".into(), args: None };
        let json = serde_json::to_string(&effect).unwrap();
        assert_eq!(json, r#"{"openDialog":{"dialogId":"addObject"}}"#);
        let round: HostEffect = serde_json::from_str(&json).unwrap();
        assert_eq!(round, effect);
    }
    //#endregion 🔖ActionArgsAndUtilitiesTests

    #[cfg(feature = "typegen")]
    #[test]
    fn exports_typescript_bindings() {
        use ts_rs::TS;
        ui_wgpu::ActionDescriptor::export().unwrap();
        ui_wgpu::WindowLayoutWindowNode::export().unwrap();
        ui_wgpu::WindowLayoutStackNode::export().unwrap();
        ui_wgpu::WindowLayoutAxisNode::export().unwrap();
        ui_wgpu::WindowLayoutChild::export().unwrap();
        ui_wgpu::WindowLayoutRoot::export().unwrap();
        ui_wgpu::WindowLayout::export().unwrap();
        ui_wgpu::NamedLayout::export().unwrap();
        ui_wgpu::component::layout::MeasureSelectItem::export().unwrap();
        ui_wgpu::WindowMeasure::export().unwrap();
        ui_wgpu::component::layout::WindowEngagementOption::export().unwrap();
        ui_wgpu::component::layout::WindowEngagementInput::export().unwrap();
        ui_wgpu::component::layout::WindowEngagementStatus::export().unwrap();
        ui_wgpu::component::layout::WindowEngagementPossible::export().unwrap();
        ui_wgpu::component::layout::WindowEngagementRingOption::export().unwrap();
        ui_wgpu::component::layout::WindowEngagementToggleGroupOption::export().unwrap();
        ui_wgpu::component::layout::WindowEngagementSelectItem::export().unwrap();
        ui_wgpu::WindowEngagementControl::export().unwrap();
        ui_wgpu::WindowEngagement::export().unwrap();
        ui_wgpu::WindowEngagementSlot::export().unwrap();
        ui_wgpu::WindowOptions::export().unwrap();
        ui_wgpu::SurfaceKind::export().unwrap();
        ui_wgpu::UtilityCategory::export().unwrap();
        crate::ui::Keybinding::export().unwrap();
        crate::ui::ActionKind::export().unwrap();
        crate::ui::ActionArgOption::export().unwrap();
        crate::ui::ActionArgControl::export().unwrap();
        crate::ui::ActionArgDef::export().unwrap();
        crate::ui::ActionDefinition::export().unwrap();
        crate::ui::ActionRef::export().unwrap();
        crate::ui::UtilityDefinition::export().unwrap();
        crate::ui::UtilityRef::export().unwrap();
        crate::ui::CommandScope::export().unwrap();
        crate::ui::CommandDefinition::export().unwrap();
        crate::ui::CommandRef::export().unwrap();
        crate::ui::ModeDefinition::export().unwrap();
        crate::ui::WindowKindDefinition::export().unwrap();
        crate::ui::PanelGroup::export().unwrap();
        crate::ui::PanelTabKind::export().unwrap();
        crate::ui::PanelTabDefinition::export().unwrap();
        crate::ui::IntroductionDefinition::export().unwrap();
        crate::ui::IntroductionStepDefinition::export().unwrap();
        crate::ui::IntroductionAnchor::export().unwrap();
        crate::ui::IntroductionEmphasis::export().unwrap();
        crate::ui::IntroductionPlacement::export().unwrap();
        crate::ui::IntroductionAdvance::export().unwrap();
        crate::ui::DialogDefinition::export().unwrap();
        crate::ui::AppDefinition::export().unwrap();
        crate::ui::ProgramDefinition::export().unwrap();
        crate::ui::ExampleDefinition::export().unwrap();
        crate::ui::Contribution::export().unwrap();
        crate::ui::PluginManifest::export().unwrap();
        crate::ui::ViewState::export().unwrap();
        crate::ui::AppLabelsOverlay::export().unwrap();
        crate::ui::kernel::CapabilityRequirement::export().unwrap();
        crate::ui::kernel::Rights::export().unwrap();
        crate::ui::kernel::ResourceKind::export().unwrap();
        crate::ui::kernel::Scope::export().unwrap();
    }
}
//#endregion 🔖Manifest

// #endregion ui
}


pub use action_bus::{ActionBus, ActionHandler};
// 🧩 The declarative component model (layout/utilities/UiNode) lives in `ui_wgpu` now — re-import
// honestly (not a re-export) wherever this crate's manifest/kernel types need it; see `pub mod ui`.
pub use mesh::{
    mesh_box, mesh_cone, mesh_cylinder, mesh_from_glb, mesh_from_indexed, mesh_from_indexed_with_face_groups, mesh_from_kind, mesh_ico_sphere,
    mesh_plane, mesh_to_glb, mesh_to_obj, mesh_from_obj, mesh_to_stl, mesh_from_stl, mesh_torus, mesh_uv_sphere, MeshData,
    dwg_drawing_to_mesh, dwg_drawing_to_paths, dwg_from_bytes, dwg_to_bytes, mesh_to_dwg_drawing, paths_to_dwg_drawing,
    DwgColor, DwgDrawing, DwgEntity, DwgGeometry, DwgLayer, DwgPathSegment,
    MeshExporter, MeshImporter, ObjExporter, ObjImporter, GlbExporter, GlbImporter, StlExporter, StlImporter,
    OsMediaFormat,
};
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use ui::*;
pub use ui::kernel::{
    ActorId, AppEvent, AppInstanceId, AssetHandle, Capability, CapabilityGrant, CapabilityRequirement,
    CapabilityToken, ActionContext, ActionDef, ActionId, ActionInvocation, CommandContext, CommandId, CommandInvocation,
    ActionRequest, InvocationId, InvocationResult, Diagnostic, HostEffect, HubClientFrame, HubServerFrame, HybridLogicalTimestamp, IconRenderExportItem, InverseOperation,
    InsertResult, KernelOperation, MergeStrategyKind, DocumentDiff, DocumentHandle, DocumentId, DocumentKind,
    DocumentVersion, OpDag, OpDagError, OpEnvelope, OperationId, PayloadHash, PhysicalSize, PluginInstanceId, PresencePeer,
    ResourceId, ResourceKind, Appearance, Rights, SchemaId, SchemaVersion, Scope, UndoGroup, UndoPolicy,
    WindowEvent, WindowHandle, WindowInput, WindowKindDef, WindowKindId, WindowOutput,
};
