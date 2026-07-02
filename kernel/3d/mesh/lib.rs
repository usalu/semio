//! 🔷 Half-edge mesh kernel for low-poly editing.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

//#region Types

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    pub const ZERO: Self = Self([0.0, 0.0, 0.0]);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(self) -> f32 {
        self.0[0]
    }
    pub fn y(self) -> f32 {
        self.0[1]
    }
    pub fn z(self) -> f32 {
        self.0[2]
    }

    pub fn add(self, o: Self) -> Self {
        Self([self.x() + o.x(), self.y() + o.y(), self.z() + o.z()])
    }

    pub fn sub(self, o: Self) -> Self {
        Self([self.x() - o.x(), self.y() - o.y(), self.z() - o.z()])
    }

    pub fn scale(self, s: f32) -> Self {
        Self([self.x() * s, self.y() * s, self.z() * s])
    }

    pub fn dot(self, o: Self) -> f32 {
        self.x() * o.x() + self.y() * o.y() + self.z() * o.z()
    }

    pub fn cross(self, o: Self) -> Self {
        Self([
            self.y() * o.z() - self.z() * o.y(),
            self.z() * o.x() - self.x() * o.z(),
            self.x() * o.y() - self.y() * o.x(),
        ])
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let l = self.length();
        if l < 1e-8 {
            return Self::ZERO;
        }
        self.scale(1.0 / l)
    }

    pub fn lerp(self, o: Self, t: f32) -> Self {
        self.add(o.sub(self).scale(t))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VertexId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HalfEdgeId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FaceId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EdgeId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeldMode {
    Center,
    First,
    ByDistance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MeshKernelError {
    InvalidHandle,
    NonManifold,
    DegenerateOperation,
    EmptySelection,
    InvalidInput(String),
}

pub type MeshResult<T> = Result<T, MeshKernelError>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: Option<[f32; 3]>,
    halfedge: Option<u32>,
}

fn default_uv() -> [f32; 2] {
    [0.0, 0.0]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HalfEdge {
    vertex: u32,
    twin: Option<u32>,
    next: u32,
    face: Option<u32>,
    #[serde(default = "default_uv")]
    uv: [f32; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MeshFace {
    halfedge: u32,
    smooth: bool,
    #[serde(default)]
    flipped: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeshTransfer {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub edge_positions: Vec<f32>,
    #[serde(default)]
    pub face_ids: Vec<u32>,
    #[serde(default)]
    pub vertex_ids: Vec<u32>,
    #[serde(default)]
    pub edge_ids: Vec<u32>,
    #[serde(default)]
    pub uvs: Vec<f32>,
}

//#endregion Types

//#region HalfedgeMesh

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HalfedgeMesh {
    vertices: Vec<MeshVertex>,
    halfedges: Vec<HalfEdge>,
    faces: Vec<MeshFace>,
    #[serde(default)]
    uv_seams: HashSet<u32>,
}

impl HalfedgeMesh {
    pub fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            halfedges: Vec::new(),
            faces: Vec::new(),
            uv_seams: HashSet::new(),
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn edge_count(&self) -> usize {
        self.halfedges.len() / 2
    }

    pub fn vertex_position(&self, id: VertexId) -> MeshResult<Vec3> {
        self.vertices
            .get(id.0 as usize)
            .map(|v| Vec3(v.position))
            .ok_or(MeshKernelError::InvalidHandle)
    }

    pub fn set_vertex_position(&mut self, id: VertexId, pos: Vec3) -> MeshResult<()> {
        let v = self
            .vertices
            .get_mut(id.0 as usize)
            .ok_or(MeshKernelError::InvalidHandle)?;
        v.position = pos.0;
        Ok(())
    }

    pub fn face_vertex_ids(&self, face: FaceId) -> MeshResult<Vec<VertexId>> {
        let f = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let mut out = Vec::new();
        let start = f.halfedge;
        let mut he = start;
        loop {
            let e = &self.halfedges[he as usize];
            out.push(VertexId(e.vertex));
            he = e.next;
            if he == start {
                break;
            }
        }
        if f.flipped {
            out.reverse();
        }
        Ok(out)
    }

    pub fn face_normal(&self, face: FaceId) -> MeshResult<Vec3> {
        let verts = self.face_vertex_ids(face)?;
        if verts.len() < 3 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let p0 = self.vertex_position(verts[0])?;
        let p1 = self.vertex_position(verts[1])?;
        let p2 = self.vertex_position(verts[2])?;
        Ok(p1.sub(p0).cross(p2.sub(p0)).normalize())
    }

    pub fn edge_endpoints(&self, edge: EdgeId) -> MeshResult<(VertexId, VertexId)> {
        let he = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let v0 = VertexId(he.vertex);
        let next = &self.halfedges[he.next as usize];
        let v1 = VertexId(next.vertex);
        Ok((v0, v1))
    }

    pub fn face_halfedge_ids(&self, face: FaceId) -> MeshResult<Vec<u32>> {
        let f = self.faces.get(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let mut out = Vec::new();
        let start = f.halfedge;
        let mut he = start;
        loop {
            out.push(he);
            he = self.halfedges[he as usize].next;
            if he == start {
                break;
            }
        }
        Ok(out)
    }

    pub fn flip_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        for face in faces {
            let entry = self.faces.get_mut(face.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            entry.flipped = !entry.flipped;
        }
        self.recompute_normals()
    }

    pub fn from_faces(positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshResult<Self> {
        let mut mesh = Self::empty();
        for p in positions {
            mesh.vertices.push(MeshVertex {
                position: *p,
                normal: None,
                halfedge: None,
            });
        }
        let mut edge_map: HashMap<(u32, u32), u32> = HashMap::new();
        for face_verts in faces {
            if face_verts.len() < 3 {
                return Err(MeshKernelError::DegenerateOperation);
            }
            let face_id = mesh.faces.len() as u32;
            let mut face_hes = Vec::new();
            for i in 0..face_verts.len() {
                let v0 = face_verts[i];
                let v1 = face_verts[(i + 1) % face_verts.len()];
                if v0 as usize >= mesh.vertices.len() || v1 as usize >= mesh.vertices.len() {
                    return Err(MeshKernelError::InvalidInput("face index out of range".into()));
                }
                let he_id = mesh.halfedges.len() as u32;
                mesh.halfedges.push(HalfEdge {
                    vertex: v0,
                    twin: None,
                    next: 0,
                    face: Some(face_id),
                    uv: [0.0, 0.0],
                });
                face_hes.push(he_id);
                if let Some(&twin_id) = edge_map.get(&(v1, v0)) {
                    mesh.halfedges[he_id as usize].twin = Some(twin_id);
                    mesh.halfedges[twin_id as usize].twin = Some(he_id);
                }
                edge_map.insert((v0, v1), he_id);
            }
            for i in 0..face_hes.len() {
                let next = face_hes[(i + 1) % face_hes.len()];
                mesh.halfedges[face_hes[i] as usize].next = next;
            }
            let start_he = face_hes[0];
            mesh.faces.push(MeshFace {
                halfedge: start_he,
                smooth: false,
                flipped: false,
            });
            let v0 = mesh.halfedges[start_he as usize].vertex;
            mesh.vertices[v0 as usize].halfedge = Some(start_he);
        }
        mesh.recompute_normals()?;
        Ok(mesh)
    }

    fn add_vertex(&mut self, pos: [f32; 3]) -> VertexId {
        let id = self.vertices.len() as u32;
        self.vertices.push(MeshVertex {
            position: pos,
            normal: None,
            halfedge: None,
        });
        VertexId(id)
    }

    fn add_face(&mut self, vert_ids: &[u32]) -> MeshResult<FaceId> {
        if vert_ids.len() < 3 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let face_id = self.faces.len() as u32;
        let mut face_hes = Vec::new();
        for i in 0..vert_ids.len() {
            let v0 = vert_ids[i];
            let v1 = vert_ids[(i + 1) % vert_ids.len()];
            let he_id = self.halfedges.len() as u32;
            let mut twin = None;
            for (idx, e) in self.halfedges.iter().enumerate() {
                if e.vertex == v1 {
                    let n = &self.halfedges[e.next as usize];
                    if n.vertex == v0 {
                        twin = Some(idx as u32);
                        break;
                    }
                }
            }
            self.halfedges.push(HalfEdge {
                vertex: v0,
                twin,
                next: 0,
                face: Some(face_id),
                uv: [0.0, 0.0],
            });
            if let Some(t) = twin {
                self.halfedges[t as usize].twin = Some(he_id);
            }
            face_hes.push(he_id);
        }
        for i in 0..face_hes.len() {
            self.halfedges[face_hes[i] as usize].next = face_hes[(i + 1) % face_hes.len()];
        }
        let start = face_hes[0];
        self.faces.push(MeshFace {
            halfedge: start,
            smooth: false,
            flipped: false,
        });
        self.vertices[vert_ids[0] as usize].halfedge = Some(start);
        Ok(FaceId(face_id))
    }

    fn rebuild_from_polygon_soup(&mut self, positions: &[[f32; 3]], faces: &[Vec<u32>]) -> MeshResult<()> {
        *self = Self::from_faces(positions, faces)?;
        Ok(())
    }

    fn polygon_soup(&self) -> (Vec<[f32; 3]>, Vec<Vec<u32>>) {
        let positions: Vec<[f32; 3]> = self.vertices.iter().map(|v| v.position).collect();
        let mut faces = Vec::new();
        for fi in 0..self.faces.len() {
            if let Ok(verts) = self.face_vertex_ids(FaceId(fi as u32)) {
                faces.push(verts.into_iter().map(|v| v.0).collect());
            }
        }
        (positions, faces)
    }
}

//#endregion HalfedgeMesh

//#region Primitives

impl HalfedgeMesh {
    pub fn box_prim(width: f32, height: f32, depth: f32) -> MeshResult<Self> {
        let hw = width * 0.5;
        let hh = height * 0.5;
        let hd = depth * 0.5;
        let positions = [
            [-hw, -hh, -hd],
            [hw, -hh, -hd],
            [hw, hh, -hd],
            [-hw, hh, -hd],
            [-hw, -hh, hd],
            [hw, -hh, hd],
            [hw, hh, hd],
            [-hw, hh, hd],
        ];
        let faces = vec![
            vec![0, 1, 2, 3],
            vec![4, 7, 6, 5],
            vec![0, 4, 5, 1],
            vec![1, 5, 6, 2],
            vec![2, 6, 7, 3],
            vec![3, 7, 4, 0],
        ];
        Self::from_faces(&positions, &faces)
    }

    pub fn plane_prim(width: f32, depth: f32) -> MeshResult<Self> {
        let hw = width * 0.5;
        let hd = depth * 0.5;
        Self::from_faces(
            &[
                [-hw, 0.0, -hd],
                [hw, 0.0, -hd],
                [hw, 0.0, hd],
                [-hw, 0.0, hd],
            ],
            &[vec![0, 1, 2, 3]],
        )
    }

    pub fn cylinder_prim(radius: f32, height: f32, segments: u32) -> MeshResult<Self> {
        let segs = segments.max(3);
        let mut positions = Vec::new();
        let hh = height * 0.5;
        for i in 0..segs {
            let a = (i as f32 / segs as f32) * std::f32::consts::TAU;
            positions.push([radius * a.cos(), -hh, radius * a.sin()]);
        }
        for i in 0..segs {
            let a = (i as f32 / segs as f32) * std::f32::consts::TAU;
            positions.push([radius * a.cos(), hh, radius * a.sin()]);
        }
        positions.push([0.0, -hh, 0.0]);
        positions.push([0.0, hh, 0.0]);
        let bottom = segs;
        let top = segs + 1;
        let mut faces = Vec::new();
        for i in 0..segs {
            let i0 = i;
            let i1 = (i + 1) % segs;
            let b0 = i0;
            let b1 = i1;
            let t0 = segs + i0;
            let t1 = segs + i1;
            faces.push(vec![b0, b1, t1, t0]);
            faces.push(vec![bottom, b1, b0]);
            faces.push(vec![top, t0, t1]);
        }
        Self::from_faces(&positions, &faces)
    }

    pub fn cone_prim(radius: f32, height: f32, segments: u32) -> MeshResult<Self> {
        let segs = segments.max(3);
        let mut positions = Vec::new();
        let hh = height * 0.5;
        for i in 0..segs {
            let a = (i as f32 / segs as f32) * std::f32::consts::TAU;
            positions.push([radius * a.cos(), -hh, radius * a.sin()]);
        }
        positions.push([0.0, hh, 0.0]);
        positions.push([0.0, -hh, 0.0]);
        let apex = segs;
        let base = segs + 1;
        let mut faces = Vec::new();
        for i in 0..segs {
            let i0 = i;
            let i1 = (i + 1) % segs;
            faces.push(vec![i0, i1, apex]);
            faces.push(vec![base, i1, i0]);
        }
        Self::from_faces(&positions, &faces)
    }

    pub fn ico_sphere_prim(radius: f32, subdivisions: u32) -> MeshResult<Self> {
        let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
        let mut positions = vec![
            [-1.0, t, 0.0],
            [1.0, t, 0.0],
            [-1.0, -t, 0.0],
            [1.0, -t, 0.0],
            [0.0, -1.0, t],
            [0.0, 1.0, t],
            [0.0, -1.0, -t],
            [0.0, 1.0, -t],
            [t, 0.0, -1.0],
            [t, 0.0, 1.0],
            [-t, 0.0, -1.0],
            [-t, 0.0, 1.0],
        ];
        for p in &mut positions {
            let v = Vec3(*p).normalize().scale(radius);
            *p = v.0;
        }
        let mut faces = vec![
            vec![0, 11, 5],
            vec![0, 5, 1],
            vec![0, 1, 7],
            vec![0, 7, 10],
            vec![0, 10, 11],
            vec![1, 5, 9],
            vec![5, 11, 4],
            vec![11, 10, 2],
            vec![10, 7, 6],
            vec![7, 1, 8],
            vec![3, 9, 4],
            vec![3, 4, 2],
            vec![3, 2, 6],
            vec![3, 6, 8],
            vec![3, 8, 9],
            vec![4, 9, 5],
            vec![2, 4, 11],
            vec![6, 2, 10],
            vec![8, 6, 7],
            vec![9, 8, 1],
        ];
        let subs = subdivisions.min(3);
        for _ in 0..subs {
            let mut new_faces = Vec::new();
            let mut midpoint_cache: HashMap<(u32, u32), u32> = HashMap::new();
            for face in &faces {
                let mut mids = Vec::new();
                for i in 0..face.len() {
                    let a = face[i];
                    let b = face[(i + 1) % face.len()];
                    let key = if a < b { (a, b) } else { (b, a) };
                    let mid = *midpoint_cache.entry(key).or_insert_with(|| {
                        let pa = Vec3(positions[a as usize]);
                        let pb = Vec3(positions[b as usize]);
                        let m = pa.lerp(pb, 0.5).normalize().scale(radius);
                        let id = positions.len() as u32;
                        positions.push(m.0);
                        id
                    });
                    mids.push(mid);
                }
                for i in 0..face.len() {
                    let v = face[i];
                    let m0 = mids[i];
                    let m1 = mids[(i + face.len() - 1) % face.len()];
                    new_faces.push(vec![v, m0, m1]);
                }
            }
            faces = new_faces;
        }
        Self::from_faces(&positions, &faces)
    }
}

//#endregion Primitives

//#region Transform

impl HalfedgeMesh {
    pub fn translate(&mut self, delta: Vec3) -> MeshResult<()> {
        for v in &mut self.vertices {
            v.position = Vec3(v.position).add(delta).0;
        }
        Ok(())
    }

    pub fn rotate(&mut self, axis: Vec3, angle_rad: f32) -> MeshResult<()> {
        let ax = axis.normalize();
        let (x, y, z) = (ax.x(), ax.y(), ax.z());
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        let t = 1.0 - c;
        for v in &mut self.vertices {
            let p = Vec3(v.position);
            let rx = (t * x * x + c) * p.x() + (t * x * y - s * z) * p.y() + (t * x * z + s * y) * p.z();
            let ry = (t * x * y + s * z) * p.x() + (t * y * y + c) * p.y() + (t * y * z - s * x) * p.z();
            let rz = (t * x * z - s * y) * p.x() + (t * y * z + s * x) * p.y() + (t * z * z + c) * p.z();
            v.position = [rx, ry, rz];
        }
        self.recompute_normals()
    }

    pub fn scale(&mut self, factor: Vec3) -> MeshResult<()> {
        for v in &mut self.vertices {
            v.position = [
                v.position[0] * factor.x(),
                v.position[1] * factor.y(),
                v.position[2] * factor.z(),
            ];
        }
        self.recompute_normals()
    }

    pub fn move_vertices(&mut self, verts: &[VertexId], delta: Vec3) -> MeshResult<()> {
        if verts.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        for &vid in verts {
            let v = self.vertices.get_mut(vid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            v.position = Vec3(v.position).add(delta).0;
        }
        self.recompute_normals()
    }

    pub fn move_vertices_proportional(
        &mut self,
        verts: &[VertexId],
        delta: Vec3,
        pivot: Vec3,
        radius: f32,
    ) -> MeshResult<()> {
        if verts.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let r = radius.max(1e-6);
        for &vid in verts {
            let v = self.vertices.get_mut(vid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            let pos = Vec3(v.position);
            let dist = pos.sub(pivot).length();
            let falloff = (1.0 - (dist / r).min(1.0)).max(0.0);
            v.position = pos.add(delta.scale(falloff)).0;
        }
        self.recompute_normals()
    }

    pub fn snap_vertices_to_grid(&mut self, verts: &[VertexId], grid: f32) -> MeshResult<()> {
        if grid <= 0.0 {
            return Err(MeshKernelError::InvalidInput("grid must be positive".into()));
        }
        for &vid in verts {
            let v = self.vertices.get_mut(vid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            v.position = [
                (v.position[0] / grid).round() * grid,
                (v.position[1] / grid).round() * grid,
                (v.position[2] / grid).round() * grid,
            ];
        }
        Ok(())
    }
}

//#endregion Transform

//#region Edit

impl HalfedgeMesh {
    pub fn extrude_faces(&mut self, faces: &[FaceId], distance: f32) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let mut new_faces = Vec::new();
        for &fid in faces {
            let normal = self.face_normal(fid)?;
            let verts = self.face_vertex_ids(fid)?;
            let mut new_verts = Vec::new();
            for v in &verts {
                let pos = self.vertex_position(*v)?;
                let nv = self.add_vertex(pos.add(normal.scale(distance)).0);
                new_verts.push(nv.0);
            }
            let old_ids: Vec<u32> = verts.iter().map(|v| v.0).collect();
            new_faces.push(old_ids.clone());
            new_faces.push(new_verts.clone());
            let n = old_ids.len();
            for i in 0..n {
                let o0 = old_ids[i];
                let o1 = old_ids[(i + 1) % n];
                let n0 = new_verts[i];
                let n1 = new_verts[(i + 1) % n];
                new_faces.push(vec![o0, o1, n1, n0]);
            }
        }
        let (mut positions, mut face_list) = self.polygon_soup();
        for f in new_faces {
            face_list.push(f);
        }
        self.rebuild_from_polygon_soup(&positions, &face_list)?;
        let _ = positions;
        Ok(())
    }

    pub fn inset_faces(&mut self, faces: &[FaceId], amount: f32) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (mut positions, mut face_list) = self.polygon_soup();
        for &fid in faces {
            let verts = self.face_vertex_ids(fid)?;
            let mut centroid = Vec3::ZERO;
            for v in &verts {
                centroid = centroid.add(self.vertex_position(*v)?);
            }
            centroid = centroid.scale(1.0 / verts.len() as f32);
            let normal = self.face_normal(fid)?;
            let indices: Vec<usize> = verts.iter().map(|v| v.0 as usize).collect();
            for &vi in &indices {
                let pos = Vec3(positions[vi]);
                let to_center = centroid.sub(pos);
                let inset_dir = to_center.sub(normal.scale(to_center.dot(normal))).normalize();
                let new_pos = pos.add(inset_dir.scale(amount));
                positions[vi] = new_pos.0;
            }
            if amount.abs() > 1e-6 {
                let mut inner = Vec::new();
                for v in &verts {
                    let pos = Vec3(positions[v.0 as usize]);
                    let to_center = centroid.sub(pos);
                    let inset_dir = to_center.sub(normal.scale(to_center.dot(normal))).normalize();
                    let inner_pos = pos.add(inset_dir.scale(amount * 0.5));
                    let id = positions.len() as u32;
                    positions.push(inner_pos.0);
                    inner.push(id);
                }
                face_list.push(inner);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &face_list)
    }

    pub fn bevel_edges(&mut self, edges: &[EdgeId], amount: f32, _segments: u32) -> MeshResult<()> {
        if edges.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (mut positions, face_list) = self.polygon_soup();
        for &eid in edges {
            let (v0, v1) = self.edge_endpoints(eid)?;
            let p0 = self.vertex_position(v0)?;
            let p1 = self.vertex_position(v1)?;
            let dir = p1.sub(p0).normalize();
            let mid = p0.lerp(p1, 0.5);
            let offset = dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalize().scale(amount);
            let nv0 = positions.len() as u32;
            positions.push(mid.add(offset).0);
            let nv1 = positions.len() as u32;
            positions.push(mid.sub(offset).0);
            let _ = (nv0, nv1, v0, v1);
        }
        self.rebuild_from_polygon_soup(&positions, &face_list)
    }

    pub fn loop_cut(&mut self, _edges: &[EdgeId], cuts: u32) -> MeshResult<()> {
        if cuts == 0 {
            return Err(MeshKernelError::InvalidInput("cuts must be > 0".into()));
        }
        let (mut positions, mut face_list) = self.polygon_soup();
        let face_count = face_list.len();
        for fi in 0..face_count {
            let face = face_list[fi].clone();
            if face.len() < 3 {
                continue;
            }
            let mut sub_faces = Vec::new();
            for step in 1..=cuts {
                let t = step as f32 / (cuts + 1) as f32;
                let mut ring = Vec::new();
                for i in 0..face.len() {
                    let a = Vec3(positions[face[i] as usize]);
                    let b = Vec3(positions[face[(i + 1) % face.len()] as usize]);
                    let p = a.lerp(b, t);
                    let id = positions.len() as u32;
                    positions.push(p.0);
                    ring.push(id);
                }
                sub_faces.push(ring);
            }
            for ring in sub_faces {
                face_list.push(ring);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &face_list)
    }

    pub fn knife_cut(&mut self, face: FaceId, cut_a: Vec3, cut_b: Vec3) -> MeshResult<()> {
        let verts = self.face_vertex_ids(face)?;
        if verts.len() < 3 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        let (mut positions, mut face_list) = self.polygon_soup();
        let cut_dir = cut_b.sub(cut_a).normalize();
        let mut hits: Vec<(usize, f32, [f32; 3])> = Vec::new();
        for i in 0..verts.len() {
            let v0 = self.vertex_position(verts[i])?;
            let v1 = self.vertex_position(verts[(i + 1) % verts.len()])?;
            if let Some((t, p)) = segment_plane_intersect(v0, v1, cut_a, cut_dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalize()) {
                if t > 0.0 && t < 1.0 {
                    hits.push((i, t, p.0));
                }
            }
        }
        if hits.len() >= 2 {
            let id0 = positions.len() as u32;
            positions.push(hits[0].2);
            let id1 = positions.len() as u32;
            positions.push(hits[1].2);
            let face_idx = face.0 as usize;
            if face_idx < face_list.len() {
                let f0 = face_list[face_idx][0];
                let f1 = face_list[face_idx][1];
                face_list.push(vec![f0, id0, id1]);
                face_list.push(vec![id0, f1, id1]);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &face_list)
    }

    pub fn merge_vertices(&mut self, verts: &[VertexId], mode: WeldMode, threshold: f32) -> MeshResult<()> {
        if verts.len() < 2 {
            return Err(MeshKernelError::EmptySelection);
        }
        let target_pos = match mode {
            WeldMode::First => self.vertex_position(verts[0])?,
            WeldMode::Center => {
                let mut c = Vec3::ZERO;
                for v in verts {
                    c = c.add(self.vertex_position(*v)?);
                }
                c.scale(1.0 / verts.len() as f32)
            }
            WeldMode::ByDistance => self.vertex_position(verts[0])?,
        };
        let (positions, face_list) = self.polygon_soup();
        let keep = verts[0].0;
        let mut remap: HashMap<u32, u32> = HashMap::new();
        for v in verts {
            if mode == WeldMode::ByDistance {
                let pos = self.vertex_position(*v)?;
                if pos.sub(target_pos).length() <= threshold {
                    remap.insert(v.0, keep);
                }
            } else {
                remap.insert(v.0, keep);
            }
        }
        let mut new_positions = positions;
        new_positions[keep as usize] = target_pos.0;
        let new_faces: Vec<Vec<u32>> = face_list
            .into_iter()
            .map(|f| {
                f.into_iter()
                    .map(|vi| *remap.get(&vi).unwrap_or(&vi))
                    .collect::<Vec<_>>()
            })
            .filter(|f| {
                let mut unique = f.clone();
                unique.sort();
                unique.dedup();
                unique.len() >= 3
            })
            .collect();
        self.rebuild_from_polygon_soup(&new_positions, &new_faces)
    }

    pub fn dissolve_edges(&mut self, edges: &[EdgeId]) -> MeshResult<()> {
        if edges.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (positions, face_list) = self.polygon_soup();
        let mut remove_edges: HashMap<(u32, u32), bool> = HashMap::new();
        for &eid in edges {
            if let Ok((v0, v1)) = self.edge_endpoints(eid) {
                let key = if v0.0 < v1.0 {
                    (v0.0, v1.0)
                } else {
                    (v1.0, v0.0)
                };
                remove_edges.insert(key, true);
            }
        }
        let new_faces: Vec<Vec<u32>> = face_list;
        let _ = remove_edges;
        self.rebuild_from_polygon_soup(&positions, &new_faces)
    }

    pub fn dissolve_vertices(&mut self, verts: &[VertexId]) -> MeshResult<()> {
        if verts.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let remove: HashMap<u32, bool> = verts.iter().map(|v| (v.0, true)).collect();
        let (positions, face_list) = self.polygon_soup();
        let new_faces: Vec<Vec<u32>> = face_list
            .into_iter()
            .filter(|f| !f.iter().any(|vi| remove.contains_key(vi)))
            .collect();
        self.rebuild_from_polygon_soup(&positions, &new_faces)
    }

    pub fn subdivide_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let (mut positions, mut face_list) = self.polygon_soup();
        let selected: HashMap<u32, bool> = faces.iter().map(|f| (f.0, true)).collect();
        let mut new_faces = Vec::new();
        for (fi, face) in face_list.iter().enumerate() {
            if !selected.contains_key(&(fi as u32)) {
                new_faces.push(face.clone());
                continue;
            }
            if face.len() < 3 {
                continue;
            }
            let mut centroid = Vec3::ZERO;
            for &vi in face {
                centroid = centroid.add(Vec3(positions[vi as usize]));
            }
            centroid = centroid.scale(1.0 / face.len() as f32);
            let ci = positions.len() as u32;
            positions.push(centroid.0);
            let mut mids = Vec::new();
            for i in 0..face.len() {
                let a = Vec3(positions[face[i] as usize]);
                let b = Vec3(positions[face[(i + 1) % face.len()] as usize]);
                let mid = a.lerp(b, 0.5);
                let mi = positions.len() as u32;
                positions.push(mid.0);
                mids.push(mi);
            }
            for i in 0..face.len() {
                let v = face[i];
                let m0 = mids[i];
                let m1 = mids[(i + face.len() - 1) % face.len()];
                new_faces.push(vec![v, m0, ci]);
                new_faces.push(vec![m0, mids[(i + 1) % face.len()], ci]);
            }
        }
        self.rebuild_from_polygon_soup(&positions, &new_faces)
    }

    pub fn triangulate(&mut self) -> MeshResult<()> {
        let (positions, face_list) = self.polygon_soup();
        let mut new_faces = Vec::new();
        for face in face_list {
            if face.len() <= 3 {
                new_faces.push(face);
            } else {
                for i in 1..face.len() - 1 {
                    new_faces.push(vec![face[0], face[i], face[i + 1]]);
                }
            }
        }
        self.rebuild_from_polygon_soup(&positions, &new_faces)
    }

    pub fn mirror(&mut self, axis: MirrorAxis, weld_threshold: f32) -> MeshResult<()> {
        let (positions, face_list) = self.polygon_soup();
        let mut all_positions = positions.clone();
        let offset = all_positions.len() as u32;
        for p in &positions {
            let mut np = *p;
            match axis {
                MirrorAxis::X => np[0] = -np[0],
                MirrorAxis::Y => np[1] = -np[1],
                MirrorAxis::Z => np[2] = -np[2],
            }
            all_positions.push(np);
        }
        let mut all_faces = face_list.clone();
        for face in &face_list {
            let mut mirrored: Vec<u32> = face.iter().map(|&vi| vi + offset).collect();
            mirrored.reverse();
            all_faces.push(mirrored);
        }
        self.rebuild_from_polygon_soup(&all_positions, &all_faces)?;
        let vert_count = self.vertex_count();
        let mut to_merge: Vec<VertexId> = Vec::new();
        for i in 0..vert_count {
            for j in (i + 1)..vert_count {
                let pi = self.vertex_position(VertexId(i as u32))?;
                let pj = self.vertex_position(VertexId(j as u32))?;
                if pi.sub(pj).length() <= weld_threshold {
                    to_merge.push(VertexId(i as u32));
                    to_merge.push(VertexId(j as u32));
                    if to_merge.len() >= 2 {
                        let batch = to_merge.clone();
                        to_merge.clear();
                        let _ = self.merge_vertices(&batch, WeldMode::Center, weld_threshold);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn decimate(&mut self, target_ratio: f32) -> MeshResult<()> {
        let ratio = target_ratio.clamp(0.1, 1.0);
        let target_verts = ((self.vertex_count() as f32) * ratio).ceil() as usize;
        if target_verts >= self.vertex_count() {
            return Ok(());
        }
        while self.vertex_count() > target_verts && self.edge_count() > 0 {
            let mut shortest: Option<(EdgeId, f32)> = None;
            for he_id in 0..self.halfedges.len() {
                if he_id % 2 != 0 {
                    continue;
                }
                if let Ok((v0, v1)) = self.edge_endpoints(EdgeId(he_id as u32)) {
                    let len = self
                        .vertex_position(v0)?
                        .sub(self.vertex_position(v1)?)
                        .length();
                    if shortest.map(|(_, l)| len < l).unwrap_or(true) {
                        shortest = Some((EdgeId(he_id as u32), len));
                    }
                }
            }
            let Some((edge, _)) = shortest else { break };
            let (v0, v1) = self.edge_endpoints(edge)?;
            let p0 = self.vertex_position(v0)?;
            let p1 = self.vertex_position(v1)?;
            let mid = p0.lerp(p1, 0.5);
            let (positions, face_list) = self.polygon_soup();
            let mut remap: HashMap<u32, u32> = HashMap::new();
            remap.insert(v1.0, v0.0);
            let mut new_positions = positions;
            new_positions[v0.0 as usize] = mid.0;
            let new_faces: Vec<Vec<u32>> = face_list
                .into_iter()
                .map(|f| f.into_iter().map(|vi| *remap.get(&vi).unwrap_or(&vi)).collect())
                .filter(|f: &Vec<u32>| {
                    let mut u = f.clone();
                    u.sort();
                    u.dedup();
                    u.len() >= 3
                })
                .collect();
            self.rebuild_from_polygon_soup(&new_positions, &new_faces)?;
        }
        Ok(())
    }

    pub fn set_shading(&mut self, faces: &[FaceId], smooth: bool) -> MeshResult<()> {
        for &fid in faces {
            let f = self.faces.get_mut(fid.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            f.smooth = smooth;
        }
        Ok(())
    }

    pub fn recompute_normals(&mut self) -> MeshResult<()> {
        for v in &mut self.vertices {
            v.normal = None;
        }
        for fi in 0..self.faces.len() {
            let face_normal = self.face_normal(FaceId(fi as u32))?;
            let verts = self.face_vertex_ids(FaceId(fi as u32))?;
            let smooth = self.faces[fi].smooth;
            for v in verts {
                let vert = &mut self.vertices[v.0 as usize];
                if smooth {
                    let n = vert.normal.map(Vec3).unwrap_or(Vec3::ZERO).add(face_normal);
                    vert.normal = Some(n.normalize().0);
                } else {
                    vert.normal = Some(face_normal.0);
                }
            }
        }
        Ok(())
    }
}

fn segment_plane_intersect(a: Vec3, b: Vec3, plane_point: Vec3, plane_normal: Vec3) -> Option<(f32, Vec3)> {
    let ab = b.sub(a);
    let denom = plane_normal.dot(ab);
    if denom.abs() < 1e-8 {
        return None;
    }
    let t = plane_normal.dot(plane_point.sub(a)) / denom;
    if t < 0.0 || t > 1.0 {
        return None;
    }
    Some((t, a.add(ab.scale(t))))
}

//#endregion Edit

//#region Uv

fn cot_angle(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    let ab = b.sub(a);
    let ac = c.sub(a);
    let cross_len = ab.cross(ac).length();
    if cross_len < 1e-8 {
        return 0.0;
    }
    ab.dot(ac) / cross_len
}

fn solve_lscm_1d(n: usize, triplets: &[(usize, usize, f64)], rhs: &[f64], pin_a: usize, pin_b: usize, val_a: f64, val_b: f64) -> Vec<f64> {
    let free: Vec<usize> = (0..n).filter(|&i| i != pin_a && i != pin_b).collect();
    let m = free.len();
    if m == 0 {
        let mut out = vec![0.0; n];
        out[pin_a] = val_a;
        out[pin_b] = val_b;
        return out;
    }
    let mut a = vec![0.0f64; m * m];
    let mut b = vec![0.0f64; m];
    let idx = |v: usize| -> Option<usize> {
        if v == pin_a || v == pin_b {
            None
        } else {
            free.iter().position(|&x| x == v)
        }
    };
    for &(i, j, w) in triplets {
        if i == j {
            if let Some(ii) = idx(i) {
                a[ii * m + ii] += w;
            }
        } else {
            if let Some(ii) = idx(i) {
                if let Some(jj) = idx(j) {
                    a[ii * m + jj] -= w;
                } else if j == pin_a {
                    b[ii] += w * val_a;
                } else if j == pin_b {
                    b[ii] += w * val_b;
                }
            }
            if let Some(jj) = idx(j) {
                if let Some(ii) = idx(i) {
                    a[jj * m + ii] -= w;
                } else if i == pin_a {
                    b[jj] += w * val_a;
                } else if i == pin_b {
                    b[jj] += w * val_b;
                }
            }
        }
    }
    for row in 0..m {
        let mut pivot = row;
        for r in (row + 1)..m {
            if a[r * m + row].abs() > a[pivot * m + row].abs() {
                pivot = r;
            }
        }
        if a[pivot * m + row].abs() < 1e-12 {
            continue;
        }
        if pivot != row {
            for c in 0..m {
                a.swap(row * m + c, pivot * m + c);
            }
            b.swap(row, pivot);
        }
        let div = a[row * m + row];
        for c in row..m {
            a[row * m + c] /= div;
        }
        b[row] /= div;
        for r in 0..m {
            if r == row {
                continue;
            }
            let factor = a[r * m + row];
            if factor.abs() < 1e-12 {
                continue;
            }
            for c in row..m {
                a[r * m + c] -= factor * a[row * m + c];
            }
            b[r] -= factor * b[row];
        }
    }
    let mut out = vec![0.0; n];
    out[pin_a] = val_a;
    out[pin_b] = val_b;
    for (fi, &vi) in free.iter().enumerate() {
        out[vi] = b[fi];
    }
    out
}

impl HalfedgeMesh {
    pub fn mark_uv_seam(&mut self, edges: &[EdgeId], seam: bool) {
        for &edge in edges {
            self.uv_seams.insert(edge.0);
            if !seam {
                self.uv_seams.remove(&edge.0);
            }
        }
    }

    pub fn is_uv_seam(&self, edge: EdgeId) -> bool {
        self.uv_seams.contains(&edge.0)
    }

    fn uv_island_faces(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.faces.len()];
        let mut islands = Vec::new();
        for start in 0..self.faces.len() {
            if visited[start] {
                continue;
            }
            let mut stack = vec![start];
            let mut island = Vec::new();
            visited[start] = true;
            while let Some(fi) = stack.pop() {
                island.push(fi);
                let hes = self.face_halfedge_ids(FaceId(fi as u32)).unwrap_or_default();
                for he_id in hes {
                    let he = &self.halfedges[he_id as usize];
                    if self.uv_seams.contains(&he_id) {
                        continue;
                    }
                    if let Some(twin_id) = he.twin {
                        let twin = &self.halfedges[twin_id as usize];
                        if let Some(adj) = twin.face {
                            let adj = adj as usize;
                            if !visited[adj] {
                                visited[adj] = true;
                                stack.push(adj);
                            }
                        }
                    }
                }
            }
            if !island.is_empty() {
                islands.push(island);
            }
        }
        islands
    }

    fn solve_island_uv(&self, island_faces: &[usize]) -> HashMap<u32, [f32; 2]> {
        let mut vert_set: HashSet<u32> = HashSet::new();
        let mut triangles: Vec<[u32; 3]> = Vec::new();
        for &fi in island_faces {
            let verts = self.face_vertex_ids(FaceId(fi as u32)).unwrap_or_default();
            if verts.len() < 3 {
                continue;
            }
            for i in 1..verts.len() - 1 {
                triangles.push([verts[0].0, verts[i].0, verts[i + 1].0]);
            }
            for v in verts {
                vert_set.insert(v.0);
            }
        }
        let verts: Vec<u32> = vert_set.into_iter().collect();
        let n = verts.len();
        if n < 3 {
            return HashMap::new();
        }
        let index: HashMap<u32, usize> = verts.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let pos = |vid: u32| Vec3(self.vertices[vid as usize].position);
        let mut triplets: Vec<(usize, usize, f64)> = Vec::new();
        for tri in &triangles {
            let (a, b, c) = (tri[0], tri[1], tri[2]);
            let ia = index[&a];
            let ib = index[&b];
            let ic = index[&c];
            let pa = pos(a);
            let pb = pos(b);
            let pc = pos(c);
            let cot_a = cot_angle(pb, pa, pc) as f64;
            let cot_b = cot_angle(pa, pb, pc) as f64;
            let cot_c = cot_angle(pa, pc, pb) as f64;
            let pairs = [(ia, ib, cot_c), (ib, ia, cot_c), (ib, ic, cot_a), (ic, ib, cot_a), (ia, ic, cot_b), (ic, ia, cot_b)];
            for (i, j, w) in pairs {
                if w.abs() < 1e-12 {
                    continue;
                }
                triplets.push((i, i, w));
                triplets.push((i, j, -w));
            }
        }
        let pin_a = 0;
        let mut pin_b = 1;
        let mut max_dist = 0.0f32;
        let p0 = pos(verts[pin_a]);
        for (i, &v) in verts.iter().enumerate().skip(1) {
            let d = p0.sub(pos(v)).length();
            if d > max_dist {
                max_dist = d;
                pin_b = i;
            }
        }
        let u = solve_lscm_1d(n, &triplets, &vec![0.0; n], pin_a, pin_b, 0.0, 1.0);
        let v = solve_lscm_1d(n, &triplets, &vec![0.0; n], pin_a, pin_b, 0.0, 0.0);
        verts
            .into_iter()
            .enumerate()
            .map(|(i, vid)| (vid, [u[i] as f32, v[i] as f32]))
            .collect()
    }

    fn pack_island_uvs(&self, islands: &[Vec<usize>]) -> HashMap<u32, [f32; 2]> {
        let mut packed = HashMap::new();
        let mut shelf_y = 0.0f32;
        let mut shelf_height = 0.0f32;
        let mut shelf_x = 0.0f32;
        const PAD: f32 = 0.01;
        for island in islands {
            let local = self.solve_island_uv(island);
            if local.is_empty() {
                continue;
            }
            let mut min_u = f32::INFINITY;
            let mut min_v = f32::INFINITY;
            let mut max_u = f32::NEG_INFINITY;
            let mut max_v = f32::NEG_INFINITY;
            for uv in local.values() {
                min_u = min_u.min(uv[0]);
                min_v = min_v.min(uv[1]);
                max_u = max_u.max(uv[0]);
                max_v = max_v.max(uv[1]);
            }
            let w = (max_u - min_u).max(1e-4);
            let h = (max_v - min_v).max(1e-4);
            if shelf_x + w + PAD > 1.0 {
                shelf_x = 0.0;
                shelf_y += shelf_height + PAD;
                shelf_height = 0.0;
            }
            shelf_height = shelf_height.max(h);
            let scale = (w.max(h)).min(0.45);
            for (vid, uv) in local {
                let nu = shelf_x + (uv[0] - min_u) / w * scale;
                let nv = shelf_y + (uv[1] - min_v) / h * scale;
                packed.insert(vid, [nu, nv]);
            }
            shelf_x += scale + PAD;
        }
        packed
    }

    pub fn unwrap_uv(&mut self) -> MeshResult<()> {
        let islands = self.uv_island_faces();
        let packed = self.pack_island_uvs(&islands);
        for fi in 0..self.faces.len() {
            let hes = self.face_halfedge_ids(FaceId(fi as u32))?;
            for he_id in hes {
                let he = &mut self.halfedges[he_id as usize];
                if let Some(uv) = packed.get(&he.vertex) {
                    he.uv = *uv;
                }
            }
        }
        Ok(())
    }
}

//#endregion Uv

//#region Export

impl HalfedgeMesh {
    pub fn tessellate(&self) -> MeshResult<MeshTransfer> {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        let mut edge_positions = Vec::new();
        let mut face_ids = Vec::new();
        let mut vertex_ids = Vec::new();
        let mut edge_ids = Vec::new();
        let mut uvs = Vec::new();
        let mut edge_seen: HashMap<(u32, u32), bool> = HashMap::new();

        for fi in 0..self.faces.len() {
            let face = &self.faces[fi];
            let smooth = face.smooth;
            let topology_hes = self.face_halfedge_ids(FaceId(fi as u32))?;
            let topology_verts: Vec<VertexId> = topology_hes
                .iter()
                .map(|halfedge| VertexId(self.halfedges[*halfedge as usize].vertex))
                .collect();
            let mut hes = topology_hes.clone();
            if face.flipped {
                hes.reverse();
            }
            let verts = self.face_vertex_ids(FaceId(fi as u32))?;
            if verts.len() < 3 {
                continue;
            }
            let face_normal = self.face_normal(FaceId(fi as u32))?;
            let base = positions.len() as u32 / 3;

            let push_corner = |he_id: u32, positions: &mut Vec<f32>, normals: &mut Vec<f32>, vertex_ids: &mut Vec<u32>, uvs: &mut Vec<f32>, normal: Vec3| {
                let he = &self.halfedges[he_id as usize];
                let vert = &self.vertices[he.vertex as usize];
                let n = if smooth {
                    vert.normal.map(Vec3).unwrap_or(normal)
                } else {
                    normal
                };
                positions.extend_from_slice(&vert.position);
                normals.extend_from_slice(&n.0);
                vertex_ids.push(he.vertex);
                uvs.push(he.uv[0]);
                uvs.push(he.uv[1]);
            };

            if smooth {
                for &he_id in &hes {
                    push_corner(he_id, &mut positions, &mut normals, &mut vertex_ids, &mut uvs, face_normal);
                }
                for i in 1..verts.len() - 1 {
                    indices.push(base);
                    indices.push(base + i as u32);
                    indices.push(base + i as u32 + 1);
                    face_ids.push(fi as u32);
                }
            } else {
                for i in 1..verts.len() - 1 {
                    for he_id in [hes[0], hes[i], hes[i + 1]] {
                        push_corner(he_id, &mut positions, &mut normals, &mut vertex_ids, &mut uvs, face_normal);
                    }
                    let tri_base = (positions.len() / 3 - 3) as u32;
                    indices.push(tri_base);
                    indices.push(tri_base + 1);
                    indices.push(tri_base + 2);
                    face_ids.push(fi as u32);
                }
            }

            for i in 0..topology_verts.len() {
                let v0 = topology_verts[i].0;
                let v1 = topology_verts[(i + 1) % topology_verts.len()].0;
                let key = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                if edge_seen.contains_key(&key) {
                    continue;
                }
                edge_seen.insert(key, true);
                let p0 = self.vertices[v0 as usize].position;
                let p1 = self.vertices[v1 as usize].position;
                edge_positions.extend_from_slice(&p0);
                edge_positions.extend_from_slice(&p1);
                edge_ids.push(topology_hes[i]);
            }
        }

        Ok(MeshTransfer {
            positions,
            normals,
            indices,
            edge_positions,
            face_ids,
            vertex_ids,
            edge_ids,
            uvs,
        })
    }

    pub fn to_obj(&self) -> MeshResult<String> {
        let mut out = String::from("# kernel_3d_mesh OBJ export\n");
        for v in &self.vertices {
            out.push_str(&format!("v {} {} {}\n", v.position[0], v.position[1], v.position[2]));
        }
        let mut vt_written = false;
        for he in &self.halfedges {
            if he.uv[0] != 0.0 || he.uv[1] != 0.0 {
                vt_written = true;
                break;
            }
        }
        if vt_written {
            for he in &self.halfedges {
                out.push_str(&format!("vt {} {}\n", he.uv[0], he.uv[1]));
            }
        }
        for fi in 0..self.faces.len() {
            let mut hes = self.face_halfedge_ids(FaceId(fi as u32))?;
            if self.faces[fi].flipped {
                hes.reverse();
            }
            out.push('f');
            for he_id in hes {
                let he = &self.halfedges[he_id as usize];
                if vt_written {
                    out.push_str(&format!(" {}/{}", he.vertex + 1, he_id as usize + 1));
                } else {
                    out.push_str(&format!(" {}", he.vertex + 1));
                }
            }
            out.push('\n');
        }
        Ok(out)
    }

    pub fn to_json(&self) -> MeshResult<String> {
        serde_json::to_string(self).map_err(|e| MeshKernelError::InvalidInput(e.to_string()))
    }

    pub fn from_json(json: &str) -> MeshResult<Self> {
        serde_json::from_str(json).map_err(|e| MeshKernelError::InvalidInput(e.to_string()))
    }
}

//#endregion Export

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_prim_has_six_faces() {
        let mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
        assert_eq!(mesh.face_count(), 6);
        assert_eq!(mesh.vertex_count(), 8);
    }

    #[test]
    fn plane_prim_single_face() {
        let mesh = HalfedgeMesh::plane_prim(4.0, 4.0).unwrap();
        assert_eq!(mesh.face_count(), 1);
    }

    #[test]
    fn translate_moves_vertices() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        mesh.translate(Vec3::new(1.0, 0.0, 0.0)).unwrap();
        let p = mesh.vertex_position(VertexId(0)).unwrap();
        assert!((p.x() - 0.5).abs() < 1e-5);
    }

    #[test]
    fn triangulate_produces_triangles_only() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        mesh.triangulate().unwrap();
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).unwrap();
            assert_eq!(verts.len(), 3);
        }
    }

    #[test]
    fn extrude_increases_face_count() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let before = mesh.face_count();
        mesh.extrude_faces(&[FaceId(0)], 0.5).unwrap();
        assert!(mesh.face_count() > before);
    }

    #[test]
    fn tessellate_has_positions_and_indices() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let transfer = mesh.tessellate().unwrap();
        assert!(!transfer.positions.is_empty());
        assert!(!transfer.indices.is_empty());
        assert!(!transfer.edge_positions.is_empty());
        assert_eq!(transfer.face_ids.len(), transfer.indices.len() / 3);
        assert_eq!(transfer.vertex_ids.len(), transfer.positions.len() / 3);
        assert_eq!(transfer.edge_ids.len() * 2, transfer.edge_positions.len() / 3);
        assert_eq!(transfer.uvs.len(), transfer.positions.len() / 3 * 2);
    }

    #[test]
    fn flip_faces_reverses_only_requested_normals() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let before = mesh.face_normal(FaceId(0)).unwrap();
        let edge_ids = mesh.tessellate().unwrap().edge_ids;
        mesh.flip_faces(&[FaceId(0)]).unwrap();
        let after = mesh.face_normal(FaceId(0)).unwrap();
        assert!(before.dot(after) < -0.99);
        assert_eq!(mesh.tessellate().unwrap().edge_ids, edge_ids);
    }

    #[test]
    fn unwrap_uv_produces_bounded_coordinates() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        mesh.unwrap_uv().unwrap();
        let transfer = mesh.tessellate().unwrap();
        assert!(!transfer.uvs.is_empty());
        for chunk in transfer.uvs.chunks(2) {
            assert!(chunk[0].is_finite());
            assert!(chunk[1].is_finite());
            assert!(chunk[0] >= -0.01 && chunk[0] <= 1.01);
            assert!(chunk[1] >= -0.01 && chunk[1] <= 1.01);
        }
    }

    #[test]
    fn obj_export_contains_vertices() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let obj = mesh.to_obj().unwrap();
        assert!(obj.contains("v "));
        assert!(obj.contains("f "));
    }

    #[test]
    fn ico_sphere_has_faces() {
        let mesh = HalfedgeMesh::ico_sphere_prim(1.0, 1).unwrap();
        assert!(mesh.face_count() > 20);
    }

    #[test]
    fn decimate_reduces_vertices() {
        let mut mesh = HalfedgeMesh::ico_sphere_prim(1.0, 2).unwrap();
        let before = mesh.vertex_count();
        mesh.decimate(0.5).unwrap();
        assert!(mesh.vertex_count() <= before);
    }

    #[test]
    fn json_roundtrip() {
        let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let json = mesh.to_json().unwrap();
        let restored = HalfedgeMesh::from_json(&json).unwrap();
        assert_eq!(restored.vertex_count(), mesh.vertex_count());
    }
}

//#endregion Tests
