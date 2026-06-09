//! 🔩 Brepkit-backed implementation of [`geometry_brep_engine::BrepKernel`].

use async_trait::async_trait;
use brepkit_math::mat::Mat4;
use brepkit_math::vec::{Point3, Vec3 as BkVec3};
use brepkit_operations::boolean::{boolean, BooleanOp};
use brepkit_operations::chamfer::chamfer;
use brepkit_operations::fillet::fillet_rolling_ball;
use brepkit_operations::measure::solid_volume;
use brepkit_operations::mirror::mirror;
use brepkit_operations::primitives::{make_box, make_cone, make_cylinder, make_sphere, make_torus};
use brepkit_operations::tessellate::{sample_solid_edges, tessellate_solid_with_tolerance};
use brepkit_operations::transform::transform_solid;
use brepkit_topology::explorer;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use brepkit_topology::TopologyError;
use geometry_brep_engine::{BrepError, BrepKernel, FaceGroup, GeometryHandle, GeometryKind, MeshTransfer, Vec3};

// #region 🔖Registry
struct Entry {
    kind: GeometryKind,
    solid: SolidId,
}

pub struct BrepkitKernel {
    topo: Topology,
    seq: u32,
    registry: std::collections::HashMap<String, Entry>,
}

impl Default for BrepkitKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl BrepkitKernel {
    pub fn new() -> Self {
        Self {
            topo: Topology::new(),
            seq: 0,
            registry: std::collections::HashMap::new(),
        }
    }

    fn register_solid(&mut self, solid: SolidId) -> GeometryHandle {
        self.seq += 1;
        let handle = GeometryHandle::new(GeometryKind::Solid, self.seq);
        self.registry.insert(handle.as_str().to_string(), Entry { kind: GeometryKind::Solid, solid });
        handle
    }

    fn solid_id(&self, handle: &GeometryHandle) -> Result<SolidId, BrepError> {
        let entry = self.registry.get(handle.as_str()).ok_or_else(|| BrepError::MissingHandle(handle.as_str().to_string()))?;
        if entry.kind != GeometryKind::Solid {
            return Err(BrepError::InvalidInput(format!("{} is not a solid", handle.as_str())));
        }
        Ok(entry.solid)
    }

    fn map_err(error: brepkit_operations::OperationsError) -> BrepError {
        BrepError::Operation(error.to_string())
    }

    fn map_topo_err(error: TopologyError) -> BrepError {
        BrepError::Operation(error.to_string())
    }

    fn rotation_axis_matrix(axis: Vec3, angle: f64) -> Result<Mat4, BrepError> {
        let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        if len < 1e-12 {
            return Err(BrepError::InvalidInput("zero rotation axis".into()));
        }
        let (x, y, z) = (axis[0] / len, axis[1] / len, axis[2] / len);
        let (s, c) = angle.sin_cos();
        let one_c = 1.0 - c;
        Ok(Mat4([
            [one_c * x * x + c, one_c * x * y - s * z, one_c * x * z + s * y, 0.0],
            [one_c * x * y + s * z, one_c * y * y + c, one_c * y * z - s * x, 0.0],
            [one_c * x * z - s * y, one_c * y * z + s * x, one_c * z * z + c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    fn edge_lines_flat(edges: &brepkit_operations::tessellate::EdgeLines) -> Vec<f32> {
        let mut flat = Vec::new();
        for index in 0..edges.offsets.len() {
            let start = edges.offsets[index] as usize;
            let end = edges.offsets.get(index + 1).copied().unwrap_or(edges.positions.len()) as usize;
            let segment = &edges.positions[start..end];
            for pair in segment.windows(2) {
                let a = &pair[0];
                let b = &pair[1];
                flat.extend([a.x() as f32, a.y() as f32, a.z() as f32, b.x() as f32, b.y() as f32, b.z() as f32]);
            }
        }
        flat
    }
}
// #endregion 🔖Registry

impl BrepkitKernel {
    pub fn box_prim_sync(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_box(&mut self.topo, width, depth, height).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn sphere_prim_sync(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_sphere(&mut self.topo, radius, 24).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn cylinder_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_cylinder(&mut self.topo, radius, height).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn cone_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_cone(&mut self.topo, radius, 0.0, height).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn torus_prim_sync(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_torus(&mut self.topo, major, minor, 24).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn fuse_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let a_id = self.solid_id(a)?;
        let b_id = self.solid_id(b)?;
        let solid = boolean(&mut self.topo, BooleanOp::Fuse, a_id, b_id).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn cut_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let a_id = self.solid_id(a)?;
        let b_id = self.solid_id(b)?;
        let solid = boolean(&mut self.topo, BooleanOp::Cut, a_id, b_id).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let a_id = self.solid_id(a)?;
        let b_id = self.solid_id(b)?;
        let solid = boolean(&mut self.topo, BooleanOp::Intersect, a_id, b_id).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn translate_sync(&mut self, shape: &GeometryHandle, offset: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let matrix = Mat4::translation(offset[0], offset[1], offset[2]);
        transform_solid(&mut self.topo, solid, &matrix).map_err(Self::map_err)?;
        Ok(shape.clone())
    }

    pub fn rotate_sync(&mut self, shape: &GeometryHandle, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let matrix = Self::rotation_axis_matrix(axis, angle)?;
        transform_solid(&mut self.topo, solid, &matrix).map_err(Self::map_err)?;
        Ok(shape.clone())
    }

    pub fn scale_sync(&mut self, shape: &GeometryHandle, factor: f64, center: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let to_origin = Mat4::translation(-center[0], -center[1], -center[2]);
        let scale = Mat4::scale(factor, factor, factor);
        let back = Mat4::translation(center[0], center[1], center[2]);
        let matrix = back * scale * to_origin;
        transform_solid(&mut self.topo, solid, &matrix).map_err(Self::map_err)?;
        Ok(shape.clone())
    }

    pub fn mirror_sync(&mut self, shape: &GeometryHandle, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid_id = self.solid_id(shape)?;
        let solid = mirror(
            &mut self.topo,
            solid_id,
            Point3::new(origin[0], origin[1], origin[2]),
            BkVec3::new(normal[0], normal[1], normal[2]),
        )
        .map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn fillet_sync(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let filleted = fillet_rolling_ball(&mut self.topo, solid, &edges, radius).map_err(Self::map_err)?;
        Ok(self.register_solid(filleted))
    }

    pub fn chamfer_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let chamfered = chamfer(&mut self.topo, solid, &edges, distance).map_err(Self::map_err)?;
        Ok(self.register_solid(chamfered))
    }

    pub fn volume_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let solid = self.solid_id(shape)?;
        solid_volume(&self.topo, solid, 0.1).map_err(Self::map_err)
    }

    pub fn kind_sync(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        Ok(self.registry.get(handle.as_str()).map(|entry| entry.kind).ok_or_else(|| BrepError::MissingHandle(handle.as_str().to_string()))?)
    }

    pub fn tessellate_sync(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError> {
        let solid = self.solid_id(handle)?;
        let tol = tolerance.max(1e-4);
        let mesh = tessellate_solid_with_tolerance(&self.topo, solid, tol, 0.2).map_err(Self::map_err)?;
        let edges = sample_solid_edges(&self.topo, solid, tol).map_err(Self::map_err)?;
        let position: Vec<f32> = mesh.positions.iter().flat_map(|point| [point.x() as f32, point.y() as f32, point.z() as f32]).collect();
        let normal: Vec<f32> = mesh.normals.iter().flat_map(|vector| [vector.x() as f32, vector.y() as f32, vector.z() as f32]).collect();
        let index = mesh.indices;
        let triangle_count = index.len() as u32;
        Ok(MeshTransfer {
            position,
            normal,
            index,
            edges: Self::edge_lines_flat(&edges),
            face_groups: vec![FaceGroup { start: 0, count: triangle_count, entity_id: handle.as_str().to_string() }],
        })
    }

    pub fn dispose_sync(&mut self, handle: &GeometryHandle) {
        self.registry.remove(handle.as_str());
    }

    /// 🧹 Drops registry entries whose handles are not in the live reference set.
    pub fn retain_sync(&mut self, live: &std::collections::HashSet<String>) {
        self.registry.retain(|handle, _| live.contains(handle));
    }

    pub fn registry_len(&self) -> usize {
        self.registry.len()
    }
}

#[async_trait(?Send)]
impl BrepKernel for BrepkitKernel {
    async fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.box_prim_sync(width, depth, height)
    }

    async fn sphere_prim(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.sphere_prim_sync(radius)
    }

    async fn cylinder_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cylinder_prim_sync(radius, height)
    }

    async fn cone_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cone_prim_sync(radius, height)
    }

    async fn torus_prim(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        self.torus_prim_sync(major, minor)
    }

    async fn fuse(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.fuse_sync(a, b)
    }

    async fn cut(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.cut_sync(a, b)
    }

    async fn intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.intersect_sync(a, b)
    }

    async fn translate(&mut self, shape: &GeometryHandle, offset: Vec3) -> Result<GeometryHandle, BrepError> {
        self.translate_sync(shape, offset)
    }

    async fn rotate(&mut self, shape: &GeometryHandle, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.rotate_sync(shape, axis, angle)
    }

    async fn scale(&mut self, shape: &GeometryHandle, factor: f64, center: Vec3) -> Result<GeometryHandle, BrepError> {
        self.scale_sync(shape, factor, center)
    }

    async fn mirror(&mut self, shape: &GeometryHandle, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        self.mirror_sync(shape, origin, normal)
    }

    async fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_sync(shape, radius)
    }

    async fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_sync(shape, distance)
    }

    async fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.volume_sync(shape)
    }

    async fn kind(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        self.kind_sync(handle)
    }

    async fn tessellate(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError> {
        self.tessellate_sync(handle, tolerance)
    }

    async fn dispose(&mut self, handle: &GeometryHandle) {
        self.dispose_sync(handle);
    }
}
// #endregion 🔖Kernel

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_and_tessellate() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let mesh = kernel.tessellate_sync(&solid, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
        assert_eq!(kernel.volume_sync(&solid).unwrap(), 24.0);
    }

    #[test]
    fn fillet_and_translate() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let filleted = kernel.fillet_sync(&solid, 0.1).unwrap();
        let moved = kernel.translate_sync(&filleted, [1.0, 0.0, 0.0]).unwrap();
        let mesh = kernel.tessellate_sync(&moved, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
    }

    #[test]
    fn retain_sync_drops_unreferenced_handles() {
        let mut kernel = BrepkitKernel::new();
        let kept = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let orphan = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        assert_eq!(kernel.registry_len(), 2);
        let live = std::collections::HashSet::from([kept.as_str().to_string()]);
        kernel.retain_sync(&live);
        assert_eq!(kernel.registry_len(), 1);
        assert!(kernel.tessellate_sync(&kept, 0.1).is_ok());
        assert!(kernel.tessellate_sync(&orphan, 0.1).is_err());
    }
}
// #endregion 🔖Tests
