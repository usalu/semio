//! 🔩 Brepkit-backed implementation of [`kernel_3d_engine::BrepKernel`].

use std::f64::consts::TAU;

use async_trait::async_trait;
use brepkit_geometry::convert::{circle_to_nurbs, ellipse_to_nurbs, line_to_nurbs};
use brepkit_geometry::sampling::{sample_deflection, surface_grid};
use brepkit_io::stl::import_mesh;
use brepkit_math::curves::{Circle3D, Ellipse3D, Line3D};
use brepkit_math::frame::Frame3;
use brepkit_math::mat::Mat4;
use brepkit_math::nurbs::bezier_clip::curve_curve_intersect;
use brepkit_math::nurbs::curve::NurbsCurve;
use brepkit_math::nurbs::fitting::{approximate, interpolate};
use brepkit_math::nurbs::intersection::{intersect_curve_surface, intersect_nurbs_nurbs};
use brepkit_math::nurbs::surface::NurbsSurface;
use brepkit_math::nurbs::surface_fitting::interpolate_surface;
use brepkit_math::surfaces::{ConicalSurface, CylindricalSurface, SphericalSurface, ToroidalSurface};
use brepkit_math::vec::{Point3, Vec3 as BkVec3};
use brepkit_operations::blend_ops::fillet_v2;
use brepkit_operations::boolean::{boolean, compound_cut, BooleanOp};
use brepkit_operations::chamfer::{chamfer, chamfer_asymmetric};
use brepkit_operations::copy::copy_solid;
use brepkit_operations::defeature::defeature;
use brepkit_operations::draft::draft;
use brepkit_operations::extrude::extrude;
use brepkit_operations::fill_face::fill_coons_patch;
use brepkit_operations::fillet::{fillet_variable, FilletRadiusLaw};
use brepkit_operations::helix::{helical_sweep, make_helix_curve};
use brepkit_operations::loft::{loft, loft_smooth};
use brepkit_operations::measure;
use brepkit_operations::mesh_boolean::mesh_boolean;
use brepkit_operations::mirror::mirror;
use brepkit_operations::offset_face::offset_face;
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::pattern::{circular_pattern, grid_pattern, linear_pattern};
use brepkit_operations::pipe::pipe;
use brepkit_operations::primitives::{make_box, make_cone, make_convex_hull, make_cylinder, make_sphere, make_torus};
use brepkit_operations::revolve::revolve;
use brepkit_operations::section::section;
use brepkit_operations::sew::sew_faces;
use brepkit_operations::shell_op::shell;
use brepkit_operations::split::split;
use brepkit_operations::sweep::sweep;
use brepkit_operations::tessellate::{sample_solid_edges, tessellate_solid_with_tolerance, tessellate_with_tolerance};
use brepkit_operations::thicken::thicken;
use brepkit_operations::transform::transform_solid;
use brepkit_topology::builder;
use brepkit_topology::compound::CompoundId;
use brepkit_topology::edge::{Edge, EdgeCurve, EdgeId};
use brepkit_topology::explorer;
use brepkit_topology::face::{FaceId, FaceSurface};
use brepkit_topology::solid::SolidId;
use brepkit_topology::vertex::{Vertex, VertexId};
use brepkit_topology::wire::{OrientedEdge, Wire, WireId};
use brepkit_topology::{Topology, TopologyError};
use kernel_3d_engine::{BrepError, BrepKernel, BrepTopology, ClosestPoint, FaceGroup, GeometryHandle, GeometryKind, MeshTransfer, ParamDomain, PointClassification, Vec3};
use rayon::prelude::*;
use semio_framework_core::{MeshExporter, MeshImporter};

// #region Helpers
const TOL: f64 = 1e-6;

fn p3(v: Vec3) -> Point3 {
    Point3::new(v[0], v[1], v[2])
}

fn from_p3(p: Point3) -> Vec3 {
    [p.x(), p.y(), p.z()]
}

fn v3(v: Vec3) -> BkVec3 {
    BkVec3::new(v[0], v[1], v[2])
}

fn from_v3(v: BkVec3) -> Vec3 {
    [v.x(), v.y(), v.z()]
}
// #endregion Helpers

// #region 🔖Registry
enum KernelCurve {
    Line(Line3D, f64),
    Circle(Circle3D, f64, f64),
    Ellipse(Ellipse3D, f64, f64),
    Nurbs(NurbsCurve),
}

enum KernelSurface {
    Plane { origin: Point3, normal: BkVec3 },
    Cylinder(CylindricalSurface),
    Cone(ConicalSurface),
    Sphere(SphericalSurface),
    Torus(ToroidalSurface),
    Nurbs(NurbsSurface),
}

enum Entity {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire(WireId),
    Face(FaceId),
    Solid(SolidId),
    Compound(CompoundId),
    Curve(KernelCurve),
    Surface(KernelSurface),
}

struct Entry {
    kind: GeometryKind,
    entity: Entity,
}

pub struct BrepkitKernel {
    topo: Topology,
    seq: u32,
    registry: std::collections::HashMap<String, Entry>,
    /// 🐌➡️⚡ Coarse-tessellation cache for [`Self::boolean_mesh_sync`]'s torus fallback, keyed by
    /// `(SolidId, deflection_bits)` — repeated booleans against the same static operand (the
    /// slider-drag motivating case) skip re-tessellating that operand every call. Invalidated by
    /// [`Self::invalidate_solid_derived_caches`] wherever a `SolidId` is mutated in place.
    mesh_boolean_cache: std::collections::HashMap<(SolidId, u64), brepkit_operations::tessellate::TriangleMesh>,
}

impl Default for BrepkitKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl BrepkitKernel {
    pub fn new() -> Self {
        Self { topo: Topology::new(), seq: 0, registry: std::collections::HashMap::new(), mesh_boolean_cache: std::collections::HashMap::new() }
    }

    /// 🧹 Evicts derived-data caches for a `SolidId` that's about to be mutated in place
    /// (`translate`/`rotate`/`scale`/`heal_solid`/`convert_to_nurbs` reuse the same `SolidId`
    /// rather than registering a fresh one, unlike every other mutating operation).
    fn invalidate_solid_derived_caches(&mut self, solid: SolidId) {
        self.mesh_boolean_cache.retain(|(id, _), _| *id != solid);
    }

    /// ⚡ Tessellates a solid at `deflection`, reusing a cached mesh when available.
    fn cached_tessellate_solid(&mut self, solid: SolidId, deflection: f64) -> Result<brepkit_operations::tessellate::TriangleMesh, BrepError> {
        let key = (solid, deflection.to_bits());
        if let Some(mesh) = self.mesh_boolean_cache.get(&key) {
            return Ok(mesh.clone());
        }
        let mesh = tessellate_solid_with_tolerance(&self.topo, solid, deflection, 0.2).map_err(Self::map_err)?;
        self.mesh_boolean_cache.insert(key, mesh.clone());
        Ok(mesh)
    }

    fn register_entity(&mut self, kind: GeometryKind, entity: Entity) -> GeometryHandle {
        self.seq += 1;
        let handle = GeometryHandle::new(kind, self.seq);
        self.registry.insert(handle.as_str().to_string(), Entry { kind, entity });
        handle
    }

    fn register_solid(&mut self, solid: SolidId) -> GeometryHandle {
        self.register_entity(GeometryKind::Solid, Entity::Solid(solid))
    }

    fn entry(&self, handle: &GeometryHandle) -> Result<&Entry, BrepError> {
        self.registry.get(handle.as_str()).ok_or_else(|| BrepError::MissingHandle(handle.as_str().to_string()))
    }

    fn solid_id(&self, handle: &GeometryHandle) -> Result<SolidId, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Solid(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a solid", handle.as_str()))),
        }
    }

    fn face_id(&self, handle: &GeometryHandle) -> Result<FaceId, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Face(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a face", handle.as_str()))),
        }
    }

    fn edge_id(&self, handle: &GeometryHandle) -> Result<EdgeId, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Edge(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not an edge", handle.as_str()))),
        }
    }

    fn wire_id(&self, handle: &GeometryHandle) -> Result<WireId, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Wire(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a wire", handle.as_str()))),
        }
    }

    fn solid_ids_from_handle(&self, handle: &GeometryHandle) -> Result<Vec<SolidId>, BrepError> {
        match &self.entry(handle)?.entity {
            Entity::Solid(id) => Ok(vec![*id]),
            Entity::Compound(id) => Ok(self.topo.compound(*id).map_err(Self::map_topo_err)?.solids().to_vec()),
            _ => Err(BrepError::InvalidInput(format!("{} is not a solid or compound", handle.as_str()))),
        }
    }

    fn map_err(error: brepkit_operations::OperationsError) -> BrepError {
        BrepError::Operation(error.to_string())
    }

    fn map_topo_err(error: TopologyError) -> BrepError {
        BrepError::Operation(error.to_string())
    }

    fn map_io_err(error: brepkit_io::IoError) -> BrepError {
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

    fn solid_has_torus_surface(&self, solid: SolidId) -> bool {
        explorer::solid_faces(&self.topo, solid).map(|faces| faces.into_iter().any(|face_id| self.topo.face(face_id).ok().is_some_and(|face| matches!(face.surface(), FaceSurface::Torus(_))))).unwrap_or(false)
    }

    fn solid_bounds_overlap(&self, a: SolidId, b: SolidId) -> bool {
        let Some(aabb_a) = brepkit_operations::measure::solid_bounding_box(&self.topo, a).ok() else {
            return true;
        };
        let Some(aabb_b) = brepkit_operations::measure::solid_bounding_box(&self.topo, b).ok() else {
            return true;
        };
        let margin = brepkit_math::tolerance::Tolerance::new().linear;
        aabb_a.min.x() <= aabb_b.max.x() + margin
            && aabb_a.max.x() + margin >= aabb_b.min.x()
            && aabb_a.min.y() <= aabb_b.max.y() + margin
            && aabb_a.max.y() + margin >= aabb_b.min.y()
            && aabb_a.min.z() <= aabb_b.max.z() + margin
            && aabb_a.max.z() + margin >= aabb_b.min.z()
    }

    fn boolean_mesh_sync(&mut self, op: BooleanOp, a: SolidId, b: SolidId) -> Result<SolidId, BrepError> {
        // 🐌 Coarser than the default render deflection on purpose: this only feeds the
        // triangle-triangle boolean, not the final mesh, and a finer value multiplies the
        // CDT/mesh-boolean triangle count enough to turn torus-involving cuts into a
        // multi-second (wasm: ~20s) synchronous stall on the caller's thread.
        let deflection = 0.1;
        let tol = brepkit_math::tolerance::Tolerance::new();
        let mesh_a = self.cached_tessellate_solid(a, deflection)?;
        let mesh_b = self.cached_tessellate_solid(b, deflection)?;
        let mb = match mesh_boolean(&mesh_a, &mesh_b, op, tol.linear) {
            Ok(result) => result,
            Err(brepkit_operations::OperationsError::EmptyResult { .. }) if op == BooleanOp::Intersect => {
                return Ok(self.topo.add_empty_solid());
            }
            Err(error) => return Err(Self::map_err(error)),
        };
        import_mesh(&mut self.topo, &mb.mesh, tol.linear).map_err(Self::map_io_err)
    }

    fn boolean_sync(&mut self, op: BooleanOp, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let a_id = self.solid_id(a)?;
        let b_id = self.solid_id(b)?;
        let torus_involved = self.solid_has_torus_surface(a_id) || self.solid_has_torus_surface(b_id);
        let use_mesh = torus_involved && self.solid_bounds_overlap(a_id, b_id);
        let solid = if use_mesh { self.boolean_mesh_sync(op, a_id, b_id)? } else { boolean(&mut self.topo, op, a_id, b_id).map_err(Self::map_err)? };
        Ok(self.register_solid(solid))
    }

    fn edge_lines_flat(edges: &brepkit_operations::tessellate::EdgeLines) -> Vec<f32> {
        let mut flat = Vec::new();
        for index in 0..edges.offsets.len() {
            let start = edges.offsets[index];
            let end = edges.offsets.get(index + 1).copied().unwrap_or(edges.positions.len());
            let segment = &edges.positions[start..end];
            for pair in segment.windows(2) {
                let a = &pair[0];
                let b = &pair[1];
                flat.extend([a.x() as f32, a.y() as f32, a.z() as f32, b.x() as f32, b.y() as f32, b.z() as f32]);
            }
        }
        flat
    }

    fn sample_oriented_edge_lines(&self, edge: EdgeId, tol: f64) -> Result<Vec<f32>, BrepError> {
        let edge_data = self.topo.edge(edge).map_err(Self::map_topo_err)?;
        let start = self.topo.vertex(edge_data.start()).map_err(Self::map_topo_err)?.point();
        let end = self.topo.vertex(edge_data.end()).map_err(Self::map_topo_err)?.point();
        let delta = end - start;
        if delta.x() * delta.x() + delta.y() * delta.y() + delta.z() * delta.z() < 1e-18 {
            return Ok(Vec::new());
        }
        let nurbs = self.edge_to_nurbs(edge)?;
        let (a, b) = nurbs.domain();
        let samples = sample_deflection(&nurbs, a, b, tol);
        let mut edges = Vec::new();
        for pair in samples.windows(2) {
            let p0 = pair[0].1;
            let p1 = pair[1].1;
            edges.extend([p0.x() as f32, p0.y() as f32, p0.z() as f32, p1.x() as f32, p1.y() as f32, p1.z() as f32]);
        }
        Ok(edges)
    }

    fn sample_face_boundary_edge_lines(&self, face: FaceId, tol: f64) -> Result<Vec<f32>, BrepError> {
        let face_data = self.topo.face(face).map_err(Self::map_topo_err)?;
        let wire = self.topo.wire(face_data.outer_wire()).map_err(Self::map_topo_err)?;
        let mut edges = Vec::new();
        for oriented_edge in wire.edges() {
            edges.extend(self.sample_oriented_edge_lines(oriented_edge.edge(), tol)?);
        }
        Ok(edges)
    }

    fn curve_domain_inner(curve: &KernelCurve) -> ParamDomain {
        match curve {
            KernelCurve::Line(_, len) => ParamDomain { min: 0.0, max: *len },
            KernelCurve::Circle(_, a, b) | KernelCurve::Ellipse(_, a, b) => ParamDomain { min: *a, max: *b },
            KernelCurve::Nurbs(c) => {
                let (a, b) = c.domain();
                ParamDomain { min: a, max: b }
            }
        }
    }

    fn curve_evaluate(curve: &KernelCurve, t: f64) -> Point3 {
        match curve {
            KernelCurve::Line(line, _) => {
                let d = line.direction();
                let o = line.origin();
                Point3::new(o.x() + d.x() * t, o.y() + d.y() * t, o.z() + d.z() * t)
            }
            KernelCurve::Circle(c, _, _) => c.evaluate(t),
            KernelCurve::Ellipse(e, _, _) => e.evaluate(t),
            KernelCurve::Nurbs(c) => c.evaluate(t),
        }
    }

    fn curve_tangent_inner(curve: &KernelCurve, t: f64) -> BkVec3 {
        match curve {
            KernelCurve::Line(line, _) => line.tangent(),
            KernelCurve::Circle(c, _, _) => c.tangent(t),
            KernelCurve::Ellipse(e, _, _) => e.tangent(t),
            KernelCurve::Nurbs(c) => {
                let d = c.derivatives(t, 1);
                if d.len() > 1 {
                    d[1]
                } else {
                    BkVec3::new(1.0, 0.0, 0.0)
                }
            }
        }
    }

    fn curve_curvature_inner(curve: &KernelCurve, t: f64) -> f64 {
        match curve {
            KernelCurve::Line(_, _) => 0.0,
            KernelCurve::Circle(c, _, _) => 1.0 / c.radius(),
            KernelCurve::Ellipse(e, a, b) => {
                if let Ok(nurbs) = ellipse_to_nurbs(e, *a, *b) {
                    let d = nurbs.derivatives(t, 2);
                    if d.len() < 2 {
                        0.0
                    } else {
                        let tan = d[1];
                        let tan_len = tan.length();
                        if tan_len < 1e-15 {
                            0.0
                        } else {
                            let d2 = if d.len() > 2 { d[2] } else { BkVec3::new(0.0, 0.0, 0.0) };
                            tan.cross(d2).length() / tan_len.powi(3)
                        }
                    }
                } else {
                    0.0
                }
            }
            KernelCurve::Nurbs(c) => {
                let d = c.derivatives(t, 2);
                if d.len() < 2 {
                    return 0.0;
                }
                let tan = d[1];
                let tan_len = tan.length();
                if tan_len < 1e-15 {
                    return 0.0;
                }
                let d2 = if d.len() > 2 { d[2] } else { BkVec3::new(0.0, 0.0, 0.0) };
                tan.cross(d2).length() / tan_len.powi(3)
            }
        }
    }

    fn curve_to_nurbs(curve: &KernelCurve) -> Result<NurbsCurve, BrepError> {
        match curve {
            KernelCurve::Line(line, len) => {
                let end = line.evaluate(*len);
                line_to_nurbs(line.origin(), end).map_err(|e| BrepError::Operation(e.to_string()))
            }
            KernelCurve::Circle(c, a, b) => circle_to_nurbs(c, *a, *b).map_err(|e| BrepError::Operation(e.to_string())),
            KernelCurve::Ellipse(e, a, b) => ellipse_to_nurbs(e, *a, *b).map_err(|e| BrepError::Operation(e.to_string())),
            KernelCurve::Nurbs(c) => Ok(c.clone()),
        }
    }

    fn edge_to_nurbs(&self, edge: EdgeId) -> Result<NurbsCurve, BrepError> {
        let edge_data = self.topo.edge(edge).map_err(Self::map_topo_err)?;
        let start_pt = self.topo.vertex(edge_data.start()).map_err(Self::map_topo_err)?.point();
        let end_pt = self.topo.vertex(edge_data.end()).map_err(Self::map_topo_err)?.point();
        match edge_data.curve() {
            EdgeCurve::NurbsCurve(c) => Ok(c.clone()),
            EdgeCurve::Line => line_to_nurbs(start_pt, end_pt).map_err(|e| BrepError::Operation(e.to_string())),
            EdgeCurve::Circle(c) => {
                let (a, b) = if edge_data.start() == edge_data.end() {
                    (0.0, TAU)
                } else {
                    let ts = c.project(start_pt);
                    let mut te = c.project(end_pt);
                    if te <= ts {
                        te += TAU;
                    }
                    (ts, te)
                };
                circle_to_nurbs(c, a, b).map_err(|e| BrepError::Operation(e.to_string()))
            }
            EdgeCurve::Ellipse(e) => {
                let (a, b) = if edge_data.start() == edge_data.end() {
                    (0.0, TAU)
                } else {
                    let ts = e.project(start_pt);
                    let mut te = e.project(end_pt);
                    if te <= ts {
                        te += TAU;
                    }
                    (ts, te)
                };
                ellipse_to_nurbs(e, a, b).map_err(|e| BrepError::Operation(e.to_string()))
            }
        }
    }

    fn surface_to_nurbs(surface: &KernelSurface) -> Result<NurbsSurface, BrepError> {
        match surface {
            KernelSurface::Nurbs(s) => Ok(s.clone()),
            KernelSurface::Plane { origin, normal } => {
                let frame = Frame3::from_normal(*origin, *normal).map_err(|e| BrepError::Operation(e.to_string()))?;
                let u = frame.x;
                let v = frame.y;
                let grid = vec![vec![*origin, *origin + u, *origin + u + v, *origin + v], vec![*origin + u, *origin + u * 2.0, *origin + u * 2.0 + v, *origin + u + v]];
                interpolate_surface(&grid, 1, 1).map_err(|e| BrepError::Operation(e.to_string()))
            }
            KernelSurface::Cylinder(c) => c.to_nurbs(0.0, TAU).map_err(|e| BrepError::Operation(e.to_string())),
            KernelSurface::Cone(c) => c.to_nurbs(0.0, TAU).map_err(|e| BrepError::Operation(e.to_string())),
            KernelSurface::Sphere(s) => s.to_nurbs().map_err(|e| BrepError::Operation(e.to_string())),
            KernelSurface::Torus(t) => t.to_nurbs().map_err(|e| BrepError::Operation(e.to_string())),
        }
    }

    fn parse_points(points: &[Vec3]) -> Result<Vec<Point3>, BrepError> {
        Ok(points.iter().map(|p| p3(*p)).collect())
    }

    fn make_planar_face_points(&mut self, points: &[Point3]) -> Result<FaceId, BrepError> {
        builder::make_planar_face(&mut self.topo, points, TOL).map_err(Self::map_topo_err)
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

    pub fn convex_hull_sync(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        if pts.len() < 4 {
            return Err(BrepError::InvalidInput("convex hull needs at least 4 points".into()));
        }
        let solid = make_convex_hull(&mut self.topo, &pts).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn line_curve_sync(&mut self, start: Vec3, end: Vec3) -> Result<GeometryHandle, BrepError> {
        let a = p3(start);
        let b = p3(end);
        let dir = b - a;
        let len = dir.length();
        if len < 1e-12 {
            return Err(BrepError::InvalidInput("coincident line endpoints".into()));
        }
        let line = Line3D::new(a, dir).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Line(line, len))))
    }

    pub fn circle_curve_sync(&mut self, center: Vec3, normal: Vec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        let circle = Circle3D::new(p3(center), v3(normal), radius).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Circle(circle, 0.0, TAU))))
    }

    pub fn arc_curve_sync(&mut self, center: Vec3, normal: Vec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        let circle = Circle3D::new(p3(center), v3(normal), radius).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Circle(circle, start_angle, end_angle))))
    }

    pub fn ellipse_curve_sync(&mut self, center: Vec3, normal: Vec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        let ellipse = Ellipse3D::new(p3(center), v3(normal), semi_major, semi_minor).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Ellipse(ellipse, 0.0, TAU))))
    }

    pub fn polyline_wire_sync(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        if pts.len() < 2 {
            return Err(BrepError::InvalidInput("polyline needs at least 2 points".into()));
        }
        let n = pts.len();
        let verts: Vec<VertexId> = pts.iter().map(|p| self.topo.add_vertex(Vertex::new(*p, TOL))).collect();
        let edges: Vec<EdgeId> = (0..n - 1).map(|i| self.topo.add_edge(Edge::new(verts[i], verts[i + 1], EdgeCurve::Line))).collect();
        let oriented: Vec<OrientedEdge> = edges.iter().map(|&e| OrientedEdge::new(e, true)).collect();
        let wire = Wire::new(oriented, false).map_err(Self::map_topo_err)?;
        let wid = self.topo.add_wire(wire);
        Ok(self.register_entity(GeometryKind::Wire, Entity::Wire(wid)))
    }

    pub fn rectangle_wire_sync(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let pts = [Point3::new(-hw, -hh, 0.0), Point3::new(hw, -hh, 0.0), Point3::new(hw, hh, 0.0), Point3::new(-hw, hh, 0.0)];
        let wire = builder::make_polygon_wire(&mut self.topo, &pts, TOL).map_err(Self::map_topo_err)?;
        Ok(self.register_entity(GeometryKind::Wire, Entity::Wire(wire)))
    }

    pub fn regular_polygon_wire_sync(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        if sides < 3 {
            return Err(BrepError::InvalidInput("polygon needs at least 3 sides".into()));
        }
        let wire = builder::make_regular_polygon_wire(&mut self.topo, radius, sides, TOL).map_err(Self::map_topo_err)?;
        Ok(self.register_entity(GeometryKind::Wire, Entity::Wire(wire)))
    }

    pub fn interpolate_curve_sync(&mut self, points: &[Vec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        if pts.len() < 2 {
            return Err(BrepError::InvalidInput("interpolate needs at least 2 points".into()));
        }
        let curve = interpolate(&pts, degree).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Nurbs(curve))))
    }

    pub fn approximate_curve_sync(&mut self, points: &[Vec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        let curve = approximate(&pts, degree, control_points).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Nurbs(curve))))
    }

    pub fn helix_curve_sync(&mut self, origin: Vec3, axis: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let curve = make_helix_curve(p3(origin), v3(axis), radius, pitch, turns, 8).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Nurbs(curve))))
    }

    pub fn plane_surface_sync(&mut self, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        Ok(self.register_entity(GeometryKind::Surface, Entity::Surface(KernelSurface::Plane { origin: p3(origin), normal: v3(normal) })))
    }

    pub fn planar_face_from_points_sync(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        let pts = Self::parse_points(points)?;
        if pts.len() < 3 {
            return Err(BrepError::InvalidInput("planar face needs at least 3 points".into()));
        }
        let face = self.make_planar_face_points(&pts)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn planar_face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let wire_id = self.wire_id(wire)?;
        let face = builder::make_planar_face_from_wire(&mut self.topo, wire_id).map_err(Self::map_topo_err)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn nurbs_surface_from_grid_sync(&mut self, points: &[Vec<Vec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        let grid: Vec<Vec<Point3>> = points.iter().map(|row| row.iter().map(|p| p3(*p)).collect()).collect();
        let surface = interpolate_surface(&grid, degree_u, degree_v).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(self.register_entity(GeometryKind::Surface, Entity::Surface(KernelSurface::Nurbs(surface))))
    }

    pub fn coons_patch_sync(&mut self, curves: &[Vec<Vec3>]) -> Result<GeometryHandle, BrepError> {
        if curves.len() < 4 {
            return Err(BrepError::InvalidInput("coons patch needs 4 boundary curves".into()));
        }
        let polylines: Vec<Vec<Point3>> = curves.iter().map(|c| Self::parse_points(c)).collect::<Result<_, _>>()?;
        let face = fill_coons_patch(&mut self.topo, &polylines).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn offset_face_sync(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(face)?;
        let face = offset_face(&mut self.topo, face_id, distance, 16).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn thicken_face_sync(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(face)?;
        let solid = thicken(&mut self.topo, face_id, thickness).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn extrude_wire_sync(&mut self, wire: &GeometryHandle, vector: Vec3) -> Result<GeometryHandle, BrepError> {
        let distance = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
        if distance < 1e-12 {
            return Err(BrepError::InvalidInput("extrude vector magnitude must be positive".into()));
        }
        let direction = [vector[0] / distance, vector[1] / distance, vector[2] / distance];
        let face = self.planar_face_from_wire_sync(wire)?;
        self.extrude_sync(&face, direction, distance)
    }

    pub fn extrude_sync(&mut self, face: &GeometryHandle, direction: Vec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(face)?;
        let solid = extrude(&mut self.topo, face_id, v3(direction), distance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn revolve_sync(&mut self, face: &GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(face)?;
        let solid = revolve(&mut self.topo, face_id, p3(axis_origin), v3(axis_direction), angle).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn loft_sync(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        let face_ids: Vec<FaceId> = profiles.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = if smooth { loft_smooth(&mut self.topo, &face_ids).map_err(Self::map_err)? } else { loft(&mut self.topo, &face_ids).map_err(Self::map_err)? };
        Ok(self.register_solid(solid))
    }

    pub fn sweep_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(profile)?;
        let nurbs = match &self.entry(path)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            Entity::Wire(w) => {
                let wire = self.topo.wire(*w).map_err(Self::map_topo_err)?;
                let mut points = Vec::new();
                for oe in wire.edges() {
                    let edge = self.topo.edge(oe.edge()).map_err(Self::map_topo_err)?;
                    points.push(self.topo.vertex(edge.start()).map_err(Self::map_topo_err)?.point());
                    points.push(self.topo.vertex(edge.end()).map_err(Self::map_topo_err)?.point());
                }
                interpolate(&points, 3).map_err(|e| BrepError::Operation(e.to_string()))?
            }
            _ => return Err(BrepError::InvalidInput("sweep path must be curve, edge, or wire".into())),
        };
        let solid = sweep(&mut self.topo, face_id, &nurbs).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn pipe_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(profile)?;
        let path_curve = match &self.entry(path)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            _ => return Err(BrepError::InvalidInput("pipe path must be curve or edge".into())),
        };
        let guide_curve = if let Some(g) = guide {
            match &self.entry(g)?.entity {
                Entity::Curve(c) => Some(Self::curve_to_nurbs(c)?),
                Entity::Edge(e) => Some(self.edge_to_nurbs(*e)?),
                _ => None,
            }
        } else {
            None
        };
        let solid = pipe(&mut self.topo, face_id, &path_curve, guide_curve.as_ref()).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn helical_sweep_sync(&mut self, profile: &GeometryHandle, axis_origin: Vec3, axis_dir: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let face_id = self.face_id(profile)?;
        let solid = helical_sweep(&mut self.topo, face_id, p3(axis_origin), v3(axis_dir), radius, pitch, turns, 8).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn fuse_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.boolean_sync(BooleanOp::Fuse, a, b)
    }

    pub fn cut_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.boolean_sync(BooleanOp::Cut, a, b)
    }

    pub fn intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.boolean_sync(BooleanOp::Intersect, a, b)
    }

    pub fn compound_cut_sync(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let target_id = self.solid_id(target)?;
        let tool_ids: Vec<SolidId> = tools.iter().map(|h| self.solid_id(h)).collect::<Result<_, _>>()?;
        let solid = compound_cut(&mut self.topo, target_id, &tool_ids, brepkit_operations::boolean::BooleanOptions::default()).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn translate_sync(&mut self, shape: &GeometryHandle, offset: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        transform_solid(&mut self.topo, solid, &Mat4::translation(offset[0], offset[1], offset[2])).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn rotate_sync(&mut self, shape: &GeometryHandle, axis: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        transform_solid(&mut self.topo, solid, &Self::rotation_axis_matrix(axis, angle)?).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn scale_sync(&mut self, shape: &GeometryHandle, factor: f64, center: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let to_origin = Mat4::translation(-center[0], -center[1], -center[2]);
        let scale = Mat4::scale(factor, factor, factor);
        let back = Mat4::translation(center[0], center[1], center[2]);
        transform_solid(&mut self.topo, solid, &(back * scale * to_origin)).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn mirror_sync(&mut self, shape: &GeometryHandle, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        let solid_id = self.solid_id(shape)?;
        let solid = mirror(&mut self.topo, solid_id, p3(origin), v3(normal)).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn copy_shape_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let solid = copy_solid(&mut self.topo, solid).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn linear_pattern_sync(&mut self, shape: &GeometryHandle, direction: Vec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let compound = linear_pattern(&mut self.topo, solid, v3(direction), spacing, count).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Compound, Entity::Compound(compound)))
    }

    pub fn circular_pattern_sync(&mut self, shape: &GeometryHandle, axis: Vec3, count: usize) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let compound = circular_pattern(&mut self.topo, solid, v3(axis), count).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Compound, Entity::Compound(compound)))
    }

    #[allow(clippy::too_many_arguments, reason = "mirrors kernel_3d_engine::BrepKernel::grid_pattern's shape 1:1 (that trait is out of this crate's scope to restructure)")]
    pub fn grid_pattern_sync(&mut self, shape: &GeometryHandle, dir_x: Vec3, dir_y: Vec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let compound = grid_pattern(&mut self.topo, solid, v3(dir_x), v3(dir_y), spacing_x, spacing_y, count_x, count_y).map_err(Self::map_err)?;
        Ok(self.register_entity(GeometryKind::Compound, Entity::Compound(compound)))
    }

    pub fn fillet_sync(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let solid = fillet_v2(&mut self.topo, solid, &edges, radius).map_err(Self::map_err)?.solid;
        Ok(self.register_solid(solid))
    }

    pub fn fillet_variable_sync(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let laws: Vec<(EdgeId, FilletRadiusLaw)> = edges.iter().map(|&e| (e, FilletRadiusLaw::Linear { start: radius_start, end: radius_end })).collect();
        let solid = fillet_variable(&mut self.topo, solid, &laws).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn chamfer_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let solid = chamfer(&mut self.topo, solid, &edges, distance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    /// 🎯 Fillets only `edges` instead of every edge of the solid — brepkit's
    /// `fillet_v2` already accepts an explicit edge list, `fillet_sync` just always
    /// passes every edge; this exposes the selective-edge case directly.
    pub fn fillet_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edge_ids: Vec<EdgeId> = edges.iter().map(|handle| self.edge_id(handle)).collect::<Result<_, _>>()?;
        let solid = fillet_v2(&mut self.topo, solid, &edge_ids, radius).map_err(Self::map_err)?.solid;
        Ok(self.register_solid(solid))
    }

    /// 🎯 Chamfers only `edges` instead of every edge of the solid.
    pub fn chamfer_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edge_ids: Vec<EdgeId> = edges.iter().map(|handle| self.edge_id(handle)).collect::<Result<_, _>>()?;
        let solid = chamfer(&mut self.topo, solid, &edge_ids, distance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn chamfer_asymmetric_sync(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)?;
        let solid = chamfer_asymmetric(&mut self.topo, solid, &edges, d1, d2).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn shell_sync(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let open: Vec<FaceId> = open_faces.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = shell(&mut self.topo, solid, thickness, &open).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn draft_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: Vec3, neutral_point: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let face_ids: Vec<FaceId> = faces.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = draft(&mut self.topo, solid, &face_ids, v3(pull_direction), p3(neutral_point), angle).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn offset_solid_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let solid = offset_solid_v2(&mut self.topo, solid, distance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn defeature_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let face_ids: Vec<FaceId> = faces.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = defeature(&mut self.topo, solid, &face_ids).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn section_sync(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<Vec<GeometryHandle>, BrepError> {
        let solid_id = self.solid_id(solid)?;
        let result = section(&mut self.topo, solid_id, p3(plane_origin), v3(plane_normal)).map_err(Self::map_err)?;
        Ok(result.faces.into_iter().map(|f| self.register_entity(GeometryKind::Face, Entity::Face(f))).collect())
    }

    pub fn split_sync(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        let solid_id = self.solid_id(solid)?;
        let result = split(&mut self.topo, solid_id, p3(plane_origin), v3(plane_normal)).map_err(Self::map_err)?;
        Ok((self.register_solid(result.positive), self.register_solid(result.negative)))
    }

    pub fn curve_curve_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError> {
        let nurbs_a = match &self.entry(a)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            _ => return Err(BrepError::InvalidInput("curve a must be curve or edge".into())),
        };
        let nurbs_b = match &self.entry(b)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            _ => return Err(BrepError::InvalidInput("curve b must be curve or edge".into())),
        };
        let hits = curve_curve_intersect(&nurbs_a, &nurbs_b, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.iter().map(|h| from_p3(h.point)).collect())
    }

    pub fn curve_surface_intersect_sync(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError> {
        let nurbs_c = match &self.entry(curve)?.entity {
            Entity::Curve(c) => Self::curve_to_nurbs(c)?,
            Entity::Edge(e) => self.edge_to_nurbs(*e)?,
            _ => return Err(BrepError::InvalidInput("curve must be curve or edge".into())),
        };
        let nurbs_s = match &self.entry(surface)?.entity {
            Entity::Surface(s) => Self::surface_to_nurbs(s)?,
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Nurbs(ns) => ns.clone(),
                    other => Self::surface_to_nurbs(&match other {
                        FaceSurface::Plane { normal, d } => KernelSurface::Plane { origin: Point3::new(normal.x() * *d, normal.y() * *d, normal.z() * *d), normal: *normal },
                        FaceSurface::Cylinder(c) => KernelSurface::Cylinder(c.clone()),
                        FaceSurface::Cone(c) => KernelSurface::Cone(c.clone()),
                        FaceSurface::Sphere(s) => KernelSurface::Sphere(s.clone()),
                        FaceSurface::Torus(t) => KernelSurface::Torus(t.clone()),
                        FaceSurface::Nurbs(_) => unreachable!(),
                    })?,
                }
            }
            _ => return Err(BrepError::InvalidInput("surface must be surface or face".into())),
        };
        let hits = intersect_curve_surface(&nurbs_c, &nurbs_s, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.iter().map(|h| from_p3(h.point)).collect())
    }

    pub fn surface_surface_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, _tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let nurbs_a = match &self.entry(a)?.entity {
            Entity::Surface(s) => Self::surface_to_nurbs(s)?,
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Nurbs(ns) => ns.clone(),
                    other => Self::surface_to_nurbs(&match other {
                        FaceSurface::Plane { normal, d } => {
                            let origin = Point3::new(normal.x() * *d, normal.y() * *d, normal.z() * *d);
                            KernelSurface::Plane { origin, normal: *normal }
                        }
                        FaceSurface::Cylinder(c) => KernelSurface::Cylinder(c.clone()),
                        FaceSurface::Cone(c) => KernelSurface::Cone(c.clone()),
                        FaceSurface::Sphere(s) => KernelSurface::Sphere(s.clone()),
                        FaceSurface::Torus(t) => KernelSurface::Torus(t.clone()),
                        FaceSurface::Nurbs(_) => unreachable!(),
                    })?,
                }
            }
            _ => return Err(BrepError::InvalidInput("a must be surface or face".into())),
        };
        let nurbs_b = match &self.entry(b)?.entity {
            Entity::Surface(s) => Self::surface_to_nurbs(s)?,
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Nurbs(ns) => ns.clone(),
                    other => Self::surface_to_nurbs(&match other {
                        FaceSurface::Plane { normal, d } => {
                            let origin = Point3::new(normal.x() * *d, normal.y() * *d, normal.z() * *d);
                            KernelSurface::Plane { origin, normal: *normal }
                        }
                        FaceSurface::Cylinder(c) => KernelSurface::Cylinder(c.clone()),
                        FaceSurface::Cone(c) => KernelSurface::Cone(c.clone()),
                        FaceSurface::Sphere(s) => KernelSurface::Sphere(s.clone()),
                        FaceSurface::Torus(t) => KernelSurface::Torus(t.clone()),
                        FaceSurface::Nurbs(_) => unreachable!(),
                    })?,
                }
            }
            _ => return Err(BrepError::InvalidInput("b must be surface or face".into())),
        };
        let curves = intersect_nurbs_nurbs(&nurbs_a, &nurbs_b, 32, 0.0).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(curves.into_iter().map(|ic| self.register_entity(GeometryKind::Curve, Entity::Curve(KernelCurve::Nurbs(ic.curve)))).collect())
    }

    pub fn curve_point_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError> {
        match &self.entry(curve)?.entity {
            Entity::Curve(c) => Ok(from_p3(Self::curve_evaluate(c, parameter))),
            Entity::Edge(e) => {
                let edge = self.topo.edge(*e).map_err(Self::map_topo_err)?;
                let start = self.topo.vertex(edge.start()).map_err(Self::map_topo_err)?.point();
                let end = self.topo.vertex(edge.end()).map_err(Self::map_topo_err)?.point();
                match edge.curve() {
                    EdgeCurve::Line => {
                        let len = (end - start).length();
                        let frac = if len < 1e-15 { 0.0 } else { parameter / len };
                        Ok(from_p3(Point3::new(start.x() + (end.x() - start.x()) * frac, start.y() + (end.y() - start.y()) * frac, start.z() + (end.z() - start.z()) * frac)))
                    }
                    EdgeCurve::NurbsCurve(c) => Ok(from_p3(c.evaluate(parameter))),
                    EdgeCurve::Circle(c) => Ok(from_p3(c.evaluate(parameter))),
                    EdgeCurve::Ellipse(el) => Ok(from_p3(el.evaluate(parameter))),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", curve.as_str()))),
        }
    }

    pub fn curve_tangent_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError> {
        match &self.entry(curve)?.entity {
            Entity::Curve(c) => Ok(from_v3(Self::curve_tangent_inner(c, parameter))),
            Entity::Edge(e) => {
                let edge = self.topo.edge(*e).map_err(Self::map_topo_err)?;
                match edge.curve() {
                    EdgeCurve::Line => {
                        let start = self.topo.vertex(edge.start()).map_err(Self::map_topo_err)?.point();
                        let end = self.topo.vertex(edge.end()).map_err(Self::map_topo_err)?.point();
                        let dir = end - start;
                        let len = dir.length();
                        if len < 1e-15 {
                            Ok([1.0, 0.0, 0.0])
                        } else {
                            Ok([dir.x() / len, dir.y() / len, dir.z() / len])
                        }
                    }
                    EdgeCurve::NurbsCurve(c) => {
                        let d = c.derivatives(parameter, 1);
                        Ok(if d.len() > 1 { from_v3(d[1]) } else { [1.0, 0.0, 0.0] })
                    }
                    EdgeCurve::Circle(c) => Ok(from_v3(c.tangent(parameter))),
                    EdgeCurve::Ellipse(el) => Ok(from_v3(el.tangent(parameter))),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", curve.as_str()))),
        }
    }

    pub fn curve_domain_sync(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        match &self.entry(curve)?.entity {
            Entity::Curve(c) => Ok(Self::curve_domain_inner(c)),
            Entity::Edge(e) => {
                let edge = self.topo.edge(*e).map_err(Self::map_topo_err)?;
                match edge.curve() {
                    EdgeCurve::Line => {
                        let start = self.topo.vertex(edge.start()).map_err(Self::map_topo_err)?.point();
                        let end = self.topo.vertex(edge.end()).map_err(Self::map_topo_err)?.point();
                        Ok(ParamDomain { min: 0.0, max: (end - start).length() })
                    }
                    EdgeCurve::NurbsCurve(c) => {
                        let (a, b) = c.domain();
                        Ok(ParamDomain { min: a, max: b })
                    }
                    EdgeCurve::Circle(_) | EdgeCurve::Ellipse(_) => Ok(ParamDomain { min: 0.0, max: TAU }),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", curve.as_str()))),
        }
    }

    pub fn curve_curvature_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        match &self.entry(curve)?.entity {
            Entity::Curve(c) => Ok(Self::curve_curvature_inner(c, parameter)),
            Entity::Edge(e) => {
                let edge = self.topo.edge(*e).map_err(Self::map_topo_err)?;
                match edge.curve() {
                    EdgeCurve::Line => Ok(0.0),
                    EdgeCurve::Circle(c) => Ok(1.0 / c.radius()),
                    EdgeCurve::Ellipse(el) => {
                        let nurbs = ellipse_to_nurbs(el, 0.0, TAU).map_err(|e| BrepError::Operation(e.to_string()))?;
                        let d = nurbs.derivatives(parameter, 2);
                        if d.len() < 2 {
                            Ok(0.0)
                        } else {
                            let tan = d[1];
                            let tan_len = tan.length();
                            if tan_len < 1e-15 {
                                Ok(0.0)
                            } else {
                                let d2 = if d.len() > 2 { d[2] } else { BkVec3::new(0.0, 0.0, 0.0) };
                                Ok(tan.cross(d2).length() / tan_len.powi(3))
                            }
                        }
                    }
                    EdgeCurve::NurbsCurve(c) => {
                        let d = c.derivatives(parameter, 2);
                        if d.len() < 2 {
                            Ok(0.0)
                        } else {
                            let tan = d[1];
                            let tan_len = tan.length();
                            if tan_len < 1e-15 {
                                Ok(0.0)
                            } else {
                                let d2 = if d.len() > 2 { d[2] } else { BkVec3::new(0.0, 0.0, 0.0) };
                                Ok(tan.cross(d2).length() / tan_len.powi(3))
                            }
                        }
                    }
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", curve.as_str()))),
        }
    }

    pub fn surface_point_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError> {
        match &self.entry(surface)?.entity {
            Entity::Surface(KernelSurface::Plane { origin, normal }) => {
                let frame = Frame3::from_normal(*origin, *normal).map_err(|e| BrepError::Operation(e.to_string()))?;
                Ok(from_p3(frame.origin + frame.x * u + frame.y * v))
            }
            Entity::Surface(KernelSurface::Cylinder(c)) => Ok(from_p3(c.evaluate(u, v))),
            Entity::Surface(KernelSurface::Cone(c)) => Ok(from_p3(c.evaluate(u, v))),
            Entity::Surface(KernelSurface::Sphere(s)) => Ok(from_p3(s.evaluate(u, v))),
            Entity::Surface(KernelSurface::Torus(t)) => Ok(from_p3(t.evaluate(u, v))),
            Entity::Surface(KernelSurface::Nurbs(ns)) => Ok(from_p3(ns.evaluate(u, v))),
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Nurbs(ns) => Ok(from_p3(ns.evaluate(u, v))),
                    FaceSurface::Plane { normal, d } => {
                        let origin = Point3::new(normal.x() * *d, normal.y() * *d, normal.z() * *d);
                        let frame = Frame3::from_normal(origin, *normal).map_err(|e| BrepError::Operation(e.to_string()))?;
                        Ok(from_p3(frame.origin + frame.x * u + frame.y * v))
                    }
                    FaceSurface::Cylinder(c) => Ok(from_p3(c.evaluate(u, v))),
                    FaceSurface::Cone(c) => Ok(from_p3(c.evaluate(u, v))),
                    FaceSurface::Sphere(s) => Ok(from_p3(s.evaluate(u, v))),
                    FaceSurface::Torus(t) => Ok(from_p3(t.evaluate(u, v))),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a surface", surface.as_str()))),
        }
    }

    pub fn surface_normal_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError> {
        match &self.entry(surface)?.entity {
            Entity::Surface(KernelSurface::Plane { normal, .. }) => Ok(from_v3(*normal)),
            Entity::Surface(KernelSurface::Cylinder(c)) => Ok(from_v3(c.normal(u, v))),
            Entity::Surface(KernelSurface::Cone(c)) => Ok(from_v3(c.normal(u, v))),
            Entity::Surface(KernelSurface::Sphere(s)) => Ok(from_v3(s.normal(u, v))),
            Entity::Surface(KernelSurface::Torus(t)) => Ok(from_v3(t.normal(u, v))),
            Entity::Surface(KernelSurface::Nurbs(ns)) => {
                let d = ns.derivatives(u, v, 1);
                let du = d.get(1).and_then(|row| row.first()).copied();
                let dv = d.first().and_then(|row| row.get(1)).copied();
                if let (Some(du), Some(dv)) = (du, dv) {
                    Ok(from_v3(du.cross(dv).normalize().unwrap_or(BkVec3::new(0.0, 0.0, 1.0))))
                } else {
                    Ok([0.0, 0.0, 1.0])
                }
            }
            Entity::Face(f) => {
                let face = self.topo.face(*f).map_err(Self::map_topo_err)?;
                match face.surface() {
                    FaceSurface::Plane { normal, .. } => Ok(from_v3(*normal)),
                    FaceSurface::Nurbs(ns) => {
                        let d = ns.derivatives(u, v, 1);
                        let du = d.get(1).and_then(|row| row.first()).copied();
                        let dv = d.first().and_then(|row| row.get(1)).copied();
                        if let (Some(du), Some(dv)) = (du, dv) {
                            Ok(from_v3(du.cross(dv).normalize().unwrap_or(BkVec3::new(0.0, 0.0, 1.0))))
                        } else {
                            Ok([0.0, 0.0, 1.0])
                        }
                    }
                    FaceSurface::Cylinder(c) => Ok(from_v3(c.normal(u, v))),
                    FaceSurface::Cone(c) => Ok(from_v3(c.normal(u, v))),
                    FaceSurface::Sphere(s) => Ok(from_v3(s.normal(u, v))),
                    FaceSurface::Torus(t) => Ok(from_v3(t.normal(u, v))),
                }
            }
            _ => Err(BrepError::InvalidInput(format!("{} is not a surface", surface.as_str()))),
        }
    }

    pub fn volume_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let solids = self.solid_ids_from_handle(shape)?;
        let mut total = 0.0;
        for solid in solids {
            total += measure::solid_volume(&self.topo, solid, 0.1).map_err(Self::map_err)?;
        }
        Ok(total)
    }

    pub fn area_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match &self.entry(shape)?.entity {
            Entity::Face(f) => measure::face_area(&self.topo, *f, 0.1).map_err(Self::map_err),
            Entity::Solid(s) => measure::solid_surface_area(&self.topo, *s, 0.1).map_err(Self::map_err),
            Entity::Compound(c) => {
                let mut total = 0.0;
                for &s in self.topo.compound(*c).map_err(Self::map_topo_err)?.solids() {
                    total += measure::solid_surface_area(&self.topo, s, 0.1).map_err(Self::map_err)?;
                }
                Ok(total)
            }
            _ => Err(BrepError::InvalidInput(format!("{} cannot compute area", shape.as_str()))),
        }
    }

    pub fn length_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match &self.entry(shape)?.entity {
            Entity::Edge(e) => measure::edge_length(&self.topo, *e).map_err(Self::map_err),
            Entity::Wire(w) => {
                let wire = self.topo.wire(*w).map_err(Self::map_topo_err)?;
                let mut total = 0.0;
                for oe in wire.edges() {
                    total += measure::edge_length(&self.topo, oe.edge()).map_err(Self::map_err)?;
                }
                Ok(total)
            }
            Entity::Curve(c) => {
                let domain = Self::curve_domain_inner(c);
                let nurbs = Self::curve_to_nurbs(c)?;
                let (a, b) = nurbs.domain();
                let samples = sample_deflection(&nurbs, a, b, 0.01);
                Ok(if samples.len() < 2 {
                    domain.max - domain.min
                } else {
                    let mut len = 0.0;
                    for w in samples.windows(2) {
                        len += (w[1].1 - w[0].1).length();
                    }
                    len
                })
            }
            _ => Err(BrepError::InvalidInput(format!("{} cannot compute length", shape.as_str()))),
        }
    }

    pub fn center_of_mass_sync(&self, shape: &GeometryHandle) -> Result<Vec3, BrepError> {
        let solid = self.solid_id(shape)?;
        let com = measure::solid_center_of_mass(&self.topo, solid, 0.1).map_err(Self::map_err)?;
        Ok(from_p3(com))
    }

    pub fn bounding_box_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solids = self.solid_ids_from_handle(shape)?;
        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);
        for solid in solids {
            let aabb = measure::solid_bounding_box(&self.topo, solid).map_err(Self::map_err)?;
            min = Point3::new(min.x().min(aabb.min.x()), min.y().min(aabb.min.y()), min.z().min(aabb.min.z()));
            max = Point3::new(max.x().max(aabb.max.x()), max.y().max(aabb.max.y()), max.z().max(aabb.max.z()));
        }
        let dx = max.x() - min.x();
        let dy = max.y() - min.y();
        let dz = max.z() - min.z();
        let solid = make_box(&mut self.topo, dx.max(TOL), dy.max(TOL), dz.max(TOL)).map_err(Self::map_err)?;
        let cx = (min.x() + max.x()) / 2.0;
        let cy = (min.y() + max.y()) / 2.0;
        let cz = (min.z() + max.z()) / 2.0;
        transform_solid(&mut self.topo, solid, &Mat4::translation(cx, cy, cz)).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn distance_sync(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        let a_solids = self.solid_ids_from_handle(a)?;
        let b_solids = self.solid_ids_from_handle(b)?;
        let mut best = f64::MAX;
        for &sa in &a_solids {
            for &sb in &b_solids {
                let d = brepkit_operations::distance::solid_to_solid_distance(&self.topo, sa, sb).map_err(Self::map_err)?.distance;
                best = best.min(d);
            }
        }
        Ok(best)
    }

    pub fn closest_point_sync(&self, shape: &GeometryHandle, point: Vec3) -> Result<ClosestPoint, BrepError> {
        let solid = self.solid_id(shape)?;
        let result = brepkit_operations::distance::point_to_solid_distance(&self.topo, p3(point), solid).map_err(Self::map_err)?;
        Ok(ClosestPoint { distance: result.distance, point: from_p3(result.point_b), parameter: None, uv: None })
    }

    pub fn classify_point_sync(&self, solid: &GeometryHandle, point: Vec3) -> Result<PointClassification, BrepError> {
        let solid_id = self.solid_id(solid)?;
        let result = brepkit_operations::classify::classify_point(&self.topo, solid_id, p3(point), 0.1, TOL).map_err(Self::map_err)?;
        Ok(match result {
            brepkit_operations::classify::PointClassification::Inside => PointClassification::Inside,
            brepkit_operations::classify::PointClassification::Outside => PointClassification::Outside,
            brepkit_operations::classify::PointClassification::OnBoundary => PointClassification::OnBoundary,
        })
    }

    pub fn validate_sync(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        let solid = self.solid_id(shape)?;
        let report = brepkit_operations::validate::validate_solid_relaxed(&self.topo, solid).map_err(Self::map_err)?;
        if report.error_count() == 0 {
            Ok("valid".into())
        } else {
            Ok(format!("{} errors", report.error_count()))
        }
    }

    pub fn vertex_sync(&mut self, point: Vec3) -> Result<GeometryHandle, BrepError> {
        let id = self.topo.add_vertex(Vertex::new(p3(point), TOL));
        Ok(self.register_entity(GeometryKind::Vertex, Entity::Vertex(id)))
    }

    pub fn face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let wire_id = self.wire_id(wire)?;
        let face = builder::make_face_from_wire(&mut self.topo, wire_id).map_err(Self::map_topo_err)?;
        Ok(self.register_entity(GeometryKind::Face, Entity::Face(face)))
    }

    pub fn sew_faces_sync(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let face_ids: Vec<FaceId> = faces.iter().map(|h| self.face_id(h)).collect::<Result<_, _>>()?;
        let solid = sew_faces(&mut self.topo, &face_ids, tolerance).map_err(Self::map_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn heal_solid_sync(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        brepkit_operations::heal::heal_solid(&mut self.topo, solid, tolerance).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn convert_to_nurbs_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        brepkit_operations::heal::convert_to_bspline(&mut self.topo, solid).map_err(Self::map_err)?;
        self.invalidate_solid_derived_caches(solid);
        Ok(shape.clone())
    }

    pub fn deconstruct_sync(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        let solids = self.solid_ids_from_handle(shape)?;
        let mut vertex_ids = Vec::new();
        let mut edge_ids = Vec::new();
        let mut face_ids = Vec::new();
        let mut seen_vertices = std::collections::HashSet::new();
        let mut seen_edges = std::collections::HashSet::new();
        let mut seen_faces = std::collections::HashSet::new();
        for solid in solids {
            for vertex in explorer::solid_vertices(&self.topo, solid).map_err(Self::map_topo_err)? {
                if seen_vertices.insert(vertex.index()) {
                    vertex_ids.push(vertex);
                }
            }
            for edge in explorer::solid_edges(&self.topo, solid).map_err(Self::map_topo_err)? {
                if seen_edges.insert(edge.index()) {
                    edge_ids.push(edge);
                }
            }
            for face in explorer::solid_faces(&self.topo, solid).map_err(Self::map_topo_err)? {
                if seen_faces.insert(face.index()) {
                    face_ids.push(face);
                }
            }
        }
        Ok(BrepTopology {
            vertices: vertex_ids.into_iter().map(|id| self.register_entity(GeometryKind::Vertex, Entity::Vertex(id))).collect(),
            edges: edge_ids.into_iter().map(|id| self.register_entity(GeometryKind::Edge, Entity::Edge(id))).collect(),
            faces: face_ids.into_iter().map(|id| self.register_entity(GeometryKind::Face, Entity::Face(id))).collect(),
        })
    }

    pub fn export_step_sync(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError> {
        let mut solids = Vec::new();
        for h in shapes {
            solids.extend(self.solid_ids_from_handle(h)?);
        }
        brepkit_io::step::writer::write_step(&self.topo, &solids).map_err(Self::map_io_err)
    }

    pub fn export_stl_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let mut solids = Vec::new();
        for h in shapes {
            solids.extend(self.solid_ids_from_handle(h)?);
        }
        brepkit_io::stl::writer::write_stl(&self.topo, &solids, deflection, brepkit_io::stl::writer::StlFormat::Binary).map_err(Self::map_io_err)
    }

    pub fn export_obj_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        let mut solids = Vec::new();
        for h in shapes {
            solids.extend(self.solid_ids_from_handle(h)?);
        }
        brepkit_io::obj::write_obj(&self.topo, &solids, deflection).map_err(Self::map_io_err)
    }

    pub fn export_gltf_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let mut solids = Vec::new();
        for h in shapes {
            solids.extend(self.solid_ids_from_handle(h)?);
        }
        brepkit_io::gltf::write_glb(&self.topo, &solids, deflection).map_err(Self::map_io_err)
    }

    /// 🌉 Tessellates `handle` via `tessellate_sync`'s `MeshTransfer` path and converts it straight to framework-core `MeshData`; the bridge that lets `GlbExporter`/`GlbImporter` (hand-rolled, dependency-free) serve GLB for B-Rep solids instead of `brepkit_io::gltf::write_glb`.
    pub fn tessellate_to_mesh_data_sync(&self, handle: &GeometryHandle, tolerance: f64) -> Result<semio_framework_core::MeshData, BrepError> {
        let transfer = self.tessellate_sync(handle, tolerance)?;
        Ok(mesh_data_from_mesh_transfer(&transfer))
    }

    /// 🌉 GLB export standardized on the hand-rolled `GlbExporter` codec (see `tessellate_to_mesh_data_sync`), not `brepkit_io::gltf`.
    pub fn export_glb_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let mut mesh = semio_framework_core::MeshData::default();
        for handle in shapes {
            mesh.merge(&self.tessellate_to_mesh_data_sync(handle, deflection.max(1e-4))?);
        }
        semio_framework_core::GlbExporter.export(&mesh).map_err(BrepError::Operation)
    }

    /// 🌉 GLB import standardized on the hand-rolled `GlbImporter` codec, converted into a solid the same way `import_dwg_sync`/`import_stl_sync` do.
    pub fn import_glb_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mesh = semio_framework_core::GlbImporter.import(data).map_err(BrepError::Operation)?;
        let positions: Vec<brepkit_math::vec::Point3> = mesh.positions.as_chunks::<3>().0.iter().map(|c| brepkit_math::vec::Point3::new(c[0] as f64, c[1] as f64, c[2] as f64)).collect();
        let normals: Vec<brepkit_math::vec::Vec3> = mesh.normals.as_chunks::<3>().0.iter().map(|c| brepkit_math::vec::Vec3::new(c[0] as f64, c[1] as f64, c[2] as f64)).collect();
        let triangle_mesh = brepkit_operations::tessellate::TriangleMesh { positions, normals, indices: mesh.indices.clone() };
        let solid = import_mesh(&mut self.topo, &triangle_mesh, tolerance).map_err(Self::map_io_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn import_step_sync(&mut self, data: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        let solids = brepkit_io::step::reader::read_step(data, &mut self.topo).map_err(Self::map_io_err)?;
        Ok(solids.into_iter().map(|s| self.register_solid(s)).collect())
    }

    pub fn import_stl_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mesh = brepkit_io::stl::reader::read_stl(data).map_err(Self::map_io_err)?;
        let solid = import_mesh(&mut self.topo, &mesh, tolerance).map_err(Self::map_io_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn import_obj_sync(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mesh = brepkit_io::obj::read_obj(data).map_err(Self::map_io_err)?;
        let solid = import_mesh(&mut self.topo, &mesh, tolerance).map_err(Self::map_io_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn export_dwg_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let mut mesh = semio_framework_core::MeshData::default();
        for h in shapes {
            for solid in self.solid_ids_from_handle(h)? {
                let tri = tessellate_solid_with_tolerance(&self.topo, solid, deflection.max(1e-4), 0.2).map_err(Self::map_err)?;
                let base = (mesh.positions.len() / 3) as u32;
                for p in &tri.positions {
                    mesh.positions.extend_from_slice(&[p.x() as f32, p.y() as f32, p.z() as f32]);
                }
                mesh.indices.extend(tri.indices.iter().map(|i| i + base));
            }
        }
        let drawing = semio_framework_core::mesh_to_dwg_drawing(&mesh);
        semio_framework_core::dwg_to_bytes(&drawing).map_err(BrepError::Operation)
    }

    pub fn import_dwg_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let drawing = semio_framework_core::dwg_from_bytes(data).map_err(BrepError::Operation)?;
        let mesh = semio_framework_core::dwg_drawing_to_mesh(&drawing);
        let positions: Vec<brepkit_math::vec::Point3> = mesh.positions.as_chunks::<3>().0.iter().map(|c| brepkit_math::vec::Point3::new(c[0] as f64, c[1] as f64, c[2] as f64)).collect();
        let normals: Vec<brepkit_math::vec::Vec3> = mesh.normals.as_chunks::<3>().0.iter().map(|c| brepkit_math::vec::Vec3::new(c[0] as f64, c[1] as f64, c[2] as f64)).collect();
        let triangle_mesh = brepkit_operations::tessellate::TriangleMesh { positions, normals, indices: mesh.indices.clone() };
        let solid = import_mesh(&mut self.topo, &triangle_mesh, tolerance).map_err(Self::map_io_err)?;
        Ok(self.register_solid(solid))
    }

    pub fn kind_sync(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        Ok(self.entry(handle)?.kind)
    }

    pub fn tessellate_sync(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError> {
        let tol = tolerance.max(1e-4);
        let entry = self.entry(handle)?;
        match &entry.entity {
            Entity::Solid(solid) => {
                // 🧵 Tessellate each face in parallel (faces are independent); the triangle-index
                // merge below stays sequential since index offsets depend on prior faces' output.
                let faces = explorer::solid_faces(&self.topo, *solid).map_err(Self::map_topo_err)?;
                let face_meshes: Result<Vec<_>, BrepError> = faces.par_iter().map(|&face| tessellate_with_tolerance(&self.topo, face, tol, 0.2).map_err(Self::map_err).map(|mesh| (face, mesh))).collect();
                let mut transfer = MeshTransfer { position: Vec::new(), normal: Vec::new(), index: Vec::new(), edges: Vec::new(), points: Vec::new(), face_groups: Vec::new() };
                for (face, mesh) in face_meshes? {
                    let base = transfer.position.len() / 3;
                    transfer.position.extend(mesh.positions.iter().flat_map(|p| [p.x() as f32, p.y() as f32, p.z() as f32]));
                    transfer.normal.extend(mesh.normals.iter().flat_map(|n| [n.x() as f32, n.y() as f32, n.z() as f32]));
                    let tri_start = transfer.index.len() as u32;
                    let tri_count = mesh.indices.len() as u32;
                    for idx in mesh.indices {
                        transfer.index.push(idx + base as u32);
                    }
                    transfer.face_groups.push(FaceGroup { start: tri_start, count: tri_count, entity_id: face.index().to_string() });
                }
                let edges = sample_solid_edges(&self.topo, *solid, tol).map_err(Self::map_err)?;
                transfer.edges = Self::edge_lines_flat(&edges);
                Ok(transfer)
            }
            Entity::Compound(c) => {
                let mut transfer = MeshTransfer::default();
                for &solid in self.topo.compound(*c).map_err(Self::map_topo_err)?.solids() {
                    let mesh = tessellate_solid_with_tolerance(&self.topo, solid, tol, 0.2).map_err(Self::map_err)?;
                    let edges = sample_solid_edges(&self.topo, solid, tol).map_err(Self::map_err)?;
                    let base = transfer.position.len() / 3;
                    transfer.position.extend(mesh.positions.iter().flat_map(|p| [p.x() as f32, p.y() as f32, p.z() as f32]));
                    transfer.normal.extend(mesh.normals.iter().flat_map(|n| [n.x() as f32, n.y() as f32, n.z() as f32]));
                    let tri_start = transfer.index.len() as u32;
                    let tri_count = mesh.indices.len() as u32;
                    for idx in mesh.indices {
                        transfer.index.push(idx + base as u32);
                    }
                    transfer.edges.extend(Self::edge_lines_flat(&edges));
                    transfer.face_groups.push(FaceGroup { start: tri_start, count: tri_count, entity_id: handle.as_str().to_string() });
                }
                Ok(transfer)
            }
            Entity::Face(face) => {
                let mesh = tessellate_with_tolerance(&self.topo, *face, tol, 0.2).map_err(Self::map_err)?;
                let position: Vec<f32> = mesh.positions.iter().flat_map(|p| [p.x() as f32, p.y() as f32, p.z() as f32]).collect();
                let normal: Vec<f32> = mesh.normals.iter().flat_map(|n| [n.x() as f32, n.y() as f32, n.z() as f32]).collect();
                let triangle_count = mesh.indices.len() as u32;
                let edges = self.sample_face_boundary_edge_lines(*face, tol)?;
                Ok(MeshTransfer { position, normal, index: mesh.indices, edges, points: Vec::new(), face_groups: vec![FaceGroup { start: 0, count: triangle_count, entity_id: handle.as_str().to_string() }] })
            }
            Entity::Curve(c) => {
                let nurbs = Self::curve_to_nurbs(c)?;
                let (a, b) = nurbs.domain();
                let samples = sample_deflection(&nurbs, a, b, tol);
                let mut edges = Vec::new();
                for w in samples.windows(2) {
                    let p0 = w[0].1;
                    let p1 = w[1].1;
                    edges.extend([p0.x() as f32, p0.y() as f32, p0.z() as f32, p1.x() as f32, p1.y() as f32, p1.z() as f32]);
                }
                Ok(MeshTransfer { position: Vec::new(), normal: Vec::new(), index: Vec::new(), edges, points: Vec::new(), face_groups: Vec::new() })
            }
            Entity::Vertex(v) => {
                let p = self.topo.vertex(*v).map_err(Self::map_topo_err)?.point();
                Ok(MeshTransfer { position: Vec::new(), normal: Vec::new(), index: Vec::new(), edges: Vec::new(), points: vec![p.x() as f32, p.y() as f32, p.z() as f32], face_groups: Vec::new() })
            }
            Entity::Edge(_) | Entity::Wire(_) => {
                let mut edges = Vec::new();
                let edge_ids: Vec<EdgeId> = match &entry.entity {
                    Entity::Edge(e) => vec![*e],
                    Entity::Wire(w) => self.topo.wire(*w).map_err(Self::map_topo_err)?.edges().iter().map(|oe| oe.edge()).collect(),
                    _ => Vec::new(),
                };
                for edge_id in edge_ids {
                    edges.extend(self.sample_oriented_edge_lines(edge_id, tol)?);
                }
                Ok(MeshTransfer { position: Vec::new(), normal: Vec::new(), index: Vec::new(), edges, points: Vec::new(), face_groups: Vec::new() })
            }
            Entity::Surface(s) => {
                let nurbs = Self::surface_to_nurbs(s)?;
                let (ua, ub) = nurbs.domain_u();
                let (va, vb) = nurbs.domain_v();
                let grid = surface_grid(&nurbs, (ua, ub), (va, vb), 16, 16);
                let mut position = Vec::new();
                let mut normal = Vec::new();
                let mut index = Vec::new();
                let rows = grid.len();
                let cols = grid.first().map_or(0, Vec::len);
                for row in &grid {
                    for p in row {
                        position.extend([p.x() as f32, p.y() as f32, p.z() as f32]);
                        normal.extend([0.0, 0.0, 1.0]);
                    }
                }
                for r in 0..rows - 1 {
                    for c in 0..cols - 1 {
                        let i0 = r * cols + c;
                        let i1 = i0 + 1;
                        let i2 = (r + 1) * cols + c;
                        let i3 = i2 + 1;
                        index.extend([i0 as u32, i2 as u32, i1 as u32, i1 as u32, i2 as u32, i3 as u32]);
                    }
                }
                let triangle_count = index.len() as u32;
                Ok(MeshTransfer { position, normal, index, edges: Vec::new(), points: Vec::new(), face_groups: vec![FaceGroup { start: 0, count: triangle_count, entity_id: handle.as_str().to_string() }] })
            }
        }
    }

    pub fn dispose_sync(&mut self, handle: &GeometryHandle) {
        if let Some(Entry { entity: Entity::Solid(solid), .. }) = self.registry.remove(handle.as_str()) {
            self.invalidate_solid_derived_caches(solid);
        }
    }

    /// 🧹 Drops registry entries whose handles are not in the live reference set.
    pub fn retain_sync(&mut self, live: &std::collections::HashSet<String>) {
        let disposed_solids: Vec<SolidId> = self
            .registry
            .iter()
            .filter(|(handle, _)| !live.contains(handle.as_str()))
            .filter_map(|(_, entry)| match entry.entity {
                Entity::Solid(solid) => Some(solid),
                _ => None,
            })
            .collect();
        self.registry.retain(|handle, _| live.contains(handle));
        for solid in disposed_solids {
            self.invalidate_solid_derived_caches(solid);
        }
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
    async fn convex_hull(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        self.convex_hull_sync(points)
    }
    async fn line_curve(&mut self, start: Vec3, end: Vec3) -> Result<GeometryHandle, BrepError> {
        self.line_curve_sync(start, end)
    }
    async fn circle_curve(&mut self, center: Vec3, normal: Vec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.circle_curve_sync(center, normal, radius)
    }
    async fn arc_curve(&mut self, center: Vec3, normal: Vec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        self.arc_curve_sync(center, normal, radius, start_angle, end_angle)
    }
    async fn ellipse_curve(&mut self, center: Vec3, normal: Vec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        self.ellipse_curve_sync(center, normal, semi_major, semi_minor)
    }
    async fn polyline_wire(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        self.polyline_wire_sync(points)
    }
    async fn rectangle_wire(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.rectangle_wire_sync(width, height)
    }
    async fn regular_polygon_wire(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        self.regular_polygon_wire_sync(radius, sides)
    }
    async fn interpolate_curve(&mut self, points: &[Vec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        self.interpolate_curve_sync(points, degree)
    }
    async fn approximate_curve(&mut self, points: &[Vec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        self.approximate_curve_sync(points, degree, control_points)
    }
    async fn helix_curve(&mut self, origin: Vec3, axis: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helix_curve_sync(origin, axis, radius, pitch, turns)
    }
    async fn plane_surface(&mut self, origin: Vec3, normal: Vec3) -> Result<GeometryHandle, BrepError> {
        self.plane_surface_sync(origin, normal)
    }
    async fn planar_face_from_points(&mut self, points: &[Vec3]) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_points_sync(points)
    }
    async fn planar_face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_wire_sync(wire)
    }
    async fn nurbs_surface_from_grid(&mut self, points: &[Vec<Vec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        self.nurbs_surface_from_grid_sync(points, degree_u, degree_v)
    }
    async fn coons_patch(&mut self, curves: &[Vec<Vec3>]) -> Result<GeometryHandle, BrepError> {
        self.coons_patch_sync(curves)
    }
    async fn offset_face(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_face_sync(face, distance)
    }
    async fn thicken_face(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        self.thicken_face_sync(face, thickness)
    }
    async fn extrude_wire(&mut self, wire: &GeometryHandle, vector: Vec3) -> Result<GeometryHandle, BrepError> {
        self.extrude_wire_sync(wire, vector)
    }

    async fn extrude(&mut self, face: &GeometryHandle, direction: Vec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.extrude_sync(face, direction, distance)
    }
    async fn revolve(&mut self, face: &GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.revolve_sync(face, axis_origin, axis_direction, angle)
    }
    async fn loft(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        self.loft_sync(profiles, smooth)
    }
    async fn sweep(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.sweep_sync(profile, path)
    }
    async fn pipe(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        self.pipe_sync(profile, path, guide)
    }
    async fn helical_sweep(&mut self, profile: &GeometryHandle, axis_origin: Vec3, axis_dir: Vec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helical_sweep_sync(profile, axis_origin, axis_dir, radius, pitch, turns)
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
    async fn compound_cut(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.compound_cut_sync(target, tools)
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
    async fn copy_shape(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.copy_shape_sync(shape)
    }
    async fn linear_pattern(&mut self, shape: &GeometryHandle, direction: Vec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        self.linear_pattern_sync(shape, direction, spacing, count)
    }
    async fn circular_pattern(&mut self, shape: &GeometryHandle, axis: Vec3, count: usize) -> Result<GeometryHandle, BrepError> {
        self.circular_pattern_sync(shape, axis, count)
    }
    async fn grid_pattern(&mut self, shape: &GeometryHandle, dir_x: Vec3, dir_y: Vec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        self.grid_pattern_sync(shape, dir_x, dir_y, spacing_x, spacing_y, count_x, count_y)
    }
    async fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_sync(shape, radius)
    }
    async fn fillet_variable(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_variable_sync(shape, radius_start, radius_end)
    }
    async fn fillet_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_edges_sync(shape, edges, radius)
    }
    async fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_sync(shape, distance)
    }
    async fn chamfer_asymmetric(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_asymmetric_sync(shape, d1, d2)
    }
    async fn chamfer_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_edges_sync(shape, edges, distance)
    }
    async fn shell(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.shell_sync(shape, thickness, open_faces)
    }
    async fn draft(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: Vec3, neutral_point: Vec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.draft_sync(shape, faces, pull_direction, neutral_point, angle)
    }
    async fn offset_solid(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_solid_sync(shape, distance)
    }
    async fn defeature(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.defeature_sync(shape, faces)
    }
    async fn section(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<Vec<GeometryHandle>, BrepError> {
        self.section_sync(solid, plane_origin, plane_normal)
    }
    async fn split(&mut self, solid: &GeometryHandle, plane_origin: Vec3, plane_normal: Vec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        self.split_sync(solid, plane_origin, plane_normal)
    }
    async fn curve_curve_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError> {
        self.curve_curve_intersect_sync(a, b, tolerance)
    }
    async fn curve_surface_intersect(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<Vec3>, BrepError> {
        self.curve_surface_intersect_sync(curve, surface, tolerance)
    }
    async fn surface_surface_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        self.surface_surface_intersect_sync(a, b, tolerance)
    }
    async fn curve_point(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError> {
        self.curve_point_sync(curve, parameter)
    }
    async fn curve_tangent(&self, curve: &GeometryHandle, parameter: f64) -> Result<Vec3, BrepError> {
        self.curve_tangent_sync(curve, parameter)
    }
    async fn curve_domain(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        self.curve_domain_sync(curve)
    }
    async fn curve_curvature(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        self.curve_curvature_sync(curve, parameter)
    }
    async fn surface_point(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError> {
        self.surface_point_sync(surface, u, v)
    }
    async fn surface_normal(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<Vec3, BrepError> {
        self.surface_normal_sync(surface, u, v)
    }
    async fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.volume_sync(shape)
    }
    async fn area(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.area_sync(shape)
    }
    async fn length(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.length_sync(shape)
    }
    async fn center_of_mass(&self, shape: &GeometryHandle) -> Result<Vec3, BrepError> {
        self.center_of_mass_sync(shape)
    }
    async fn bounding_box(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.bounding_box_sync(shape)
    }
    async fn distance(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        self.distance_sync(a, b)
    }
    async fn closest_point(&self, shape: &GeometryHandle, point: Vec3) -> Result<ClosestPoint, BrepError> {
        self.closest_point_sync(shape, point)
    }
    async fn classify_point(&self, solid: &GeometryHandle, point: Vec3) -> Result<PointClassification, BrepError> {
        self.classify_point_sync(solid, point)
    }
    async fn validate(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        self.validate_sync(shape)
    }
    async fn vertex(&mut self, point: Vec3) -> Result<GeometryHandle, BrepError> {
        self.vertex_sync(point)
    }
    async fn face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.face_from_wire_sync(wire)
    }
    async fn sew_faces(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.sew_faces_sync(faces, tolerance)
    }
    async fn heal_solid(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.heal_solid_sync(shape, tolerance)
    }
    async fn convert_to_nurbs(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.convert_to_nurbs_sync(shape)
    }
    async fn deconstruct(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        self.deconstruct_sync(shape)
    }
    async fn export_step(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError> {
        self.export_step_sync(shapes)
    }
    async fn export_stl(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_stl_sync(shapes, deflection)
    }
    async fn export_obj(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        self.export_obj_sync(shapes, deflection)
    }
    async fn export_gltf(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_gltf_sync(shapes, deflection)
    }
    async fn import_step(&mut self, data: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        self.import_step_sync(data)
    }
    async fn import_stl(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_stl_sync(data, tolerance)
    }
    async fn import_obj(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_obj_sync(data, tolerance)
    }
    async fn export_dwg(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_dwg_sync(shapes, deflection)
    }
    async fn import_dwg(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_dwg_sync(data, tolerance)
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
    async fn retain(&mut self, live: &std::collections::HashSet<String>) {
        self.retain_sync(live);
    }
    async fn registry_len(&self) -> usize {
        self.registry_len()
    }
}
// #endregion 🔖Kernel

// #region 🔖MeshInterop
/// 🌉 Flattens a kernel `MeshTransfer` (position/normal/index/face_groups) into framework-core `MeshData`, reusing `mesh_from_indexed_with_face_groups` so picked triangles still resolve back to their B-Rep face id.
pub fn mesh_data_from_mesh_transfer(transfer: &MeshTransfer) -> semio_framework_core::MeshData {
    let face_groups: Vec<(u32, u32, u32)> = transfer.face_groups.iter().map(|group| (group.entity_id.parse().unwrap_or(0), group.start, group.count)).collect();
    let mut mesh = semio_framework_core::mesh_from_indexed_with_face_groups(&transfer.position, &transfer.normal, &transfer.index, &face_groups);
    mesh.edge_positions = transfer.edges.clone();
    if !mesh.edge_positions.is_empty() {
        let edge_count = mesh.edge_positions.len() / 6;
        mesh.edge_ids = (0..edge_count as u32).collect();
    }
    mesh
}

/// 🔌 Format-keyed solid export codec operating on `GeometryHandle`s directly (not tessellated `MeshData`) — thin wrappers around `BrepkitKernel`'s own STEP/STL/OBJ/GLB writers; no codec logic lives here.
pub trait SolidExporter: Send + Sync {
    fn format(&self) -> semio_framework_core::OsMediaFormat;
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
}

/// 🔌 Format-keyed solid import codec; see `SolidExporter`. Returns every solid the payload contained (STEP files may hold more than one).
pub trait SolidImporter: Send + Sync {
    fn format(&self) -> semio_framework_core::OsMediaFormat;
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError>;
}

pub struct StepSolidExporter;
impl SolidExporter for StepSolidExporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Step
    }
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], _deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_step_sync(shapes).map(|text| text.into_bytes())
    }
}

pub struct StepSolidImporter;
impl SolidImporter for StepSolidImporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Step
    }
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], _tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|error| BrepError::InvalidInput(error.to_string()))?;
        kernel.import_step_sync(text)
    }
}

pub struct StlSolidExporter;
impl SolidExporter for StlSolidExporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Stl
    }
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_stl_sync(shapes, deflection)
    }
}

pub struct StlSolidImporter;
impl SolidImporter for StlSolidImporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Stl
    }
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        kernel.import_stl_sync(bytes, tolerance).map(|handle| vec![handle])
    }
}

pub struct ObjSolidExporter;
impl SolidExporter for ObjSolidExporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Obj
    }
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_obj_sync(shapes, deflection).map(|text| text.into_bytes())
    }
}

pub struct ObjSolidImporter;
impl SolidImporter for ObjSolidImporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Obj
    }
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|error| BrepError::InvalidInput(error.to_string()))?;
        kernel.import_obj_sync(text, tolerance).map(|handle| vec![handle])
    }
}

pub struct GlbSolidExporter;
impl SolidExporter for GlbSolidExporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Glb
    }
    fn export(&self, kernel: &BrepkitKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_glb_sync(shapes, deflection)
    }
}

pub struct GlbSolidImporter;
impl SolidImporter for GlbSolidImporter {
    fn format(&self) -> semio_framework_core::OsMediaFormat {
        semio_framework_core::OsMediaFormat::Glb
    }
    fn import(&self, kernel: &mut BrepkitKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        kernel.import_glb_sync(bytes, tolerance).map(|handle| vec![handle])
    }
}
// #endregion 🔖MeshInterop

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
    fn box_tessellation_emits_one_face_group_per_topological_face() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let mesh = kernel.tessellate_sync(&solid, 0.1).unwrap();
        assert_eq!(mesh.face_groups.len(), 6, "a box has 6 planar faces");
        let entity_ids: std::collections::HashSet<&str> = mesh.face_groups.iter().map(|g| g.entity_id.as_str()).collect();
        assert_eq!(entity_ids.len(), 6, "face group entity ids must be distinct per face");
        let triangle_count = (mesh.index.len() / 3) as u32;
        let mut covered = vec![false; triangle_count as usize];
        for group in &mesh.face_groups {
            assert!(group.count > 0, "every face group must contain at least one triangle");
            for tri in (group.start / 3)..(group.start / 3 + group.count / 3) {
                assert!(!covered[tri as usize], "face groups must not overlap");
                covered[tri as usize] = true;
            }
        }
        assert!(covered.into_iter().all(|hit| hit), "face groups must partition every triangle");
    }

    #[test]
    fn dwg_export_import_round_trips_a_box() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let bytes = kernel.export_dwg_sync(&[solid], 0.1).unwrap();
        assert!(!bytes.is_empty());
        let imported = kernel.import_dwg_sync(&bytes, 0.1).unwrap();
        let mesh = kernel.tessellate_sync(&imported, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
    }

    #[test]
    fn glb_export_import_round_trips_a_box_through_the_mesh_codec() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let original_volume = kernel.volume_sync(&solid).unwrap();
        let bytes = kernel.export_glb_sync(&[solid], 0.1).unwrap();
        assert!(!bytes.is_empty());
        let imported = kernel.import_glb_sync(&bytes, 0.1).unwrap();
        let imported_volume = kernel.volume_sync(&imported).unwrap();
        assert!((imported_volume - original_volume).abs() < original_volume * 0.05, "volume should survive the GLB round trip: original={original_volume} imported={imported_volume}");
        let mesh = kernel.tessellate_sync(&imported, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
        let triangle_count = mesh.index.len() / 3;
        assert!((6..=5000).contains(&triangle_count), "a re-tessellated box should stay in a sane triangle-count range, got {triangle_count}");
    }

    #[test]
    fn glb_tessellation_bridge_produces_reasonable_mesh_data() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let mesh_data = kernel.tessellate_to_mesh_data_sync(&solid, 0.1).unwrap();
        assert!(!mesh_data.positions.is_empty(), "tessellation bridge must produce vertex positions");
        assert!(!mesh_data.indices.is_empty(), "tessellation bridge must produce triangle indices");
        assert_eq!(mesh_data.indices.len() % 3, 0, "indices must form complete triangles");
        let triangle_count = mesh_data.triangle_count();
        assert!((6..=5000).contains(&triangle_count), "a box tessellated through the GLB bridge should stay in a sane triangle-count range, got {triangle_count}");

        let bytes = semio_framework_core::GlbExporter.export(&mesh_data).unwrap();
        assert!(!bytes.is_empty());
        let reimported = semio_framework_core::GlbImporter.import(&bytes).unwrap();
        let reimported_triangles = reimported.indices.len() / 3;
        assert_eq!(reimported_triangles, triangle_count, "GLB codec must preserve triangle count through export/import");
        assert_eq!(reimported.positions.len(), mesh_data.positions.len(), "GLB codec must preserve vertex position count through export/import");
    }

    #[test]
    fn tessellate_to_mesh_data_carries_face_ids() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let mesh = kernel.tessellate_to_mesh_data_sync(&solid, 0.1).unwrap();
        assert_eq!(mesh.face_ids.len(), mesh.triangle_count());
        assert!(mesh.edge_positions.len() >= 6);
        assert_eq!(mesh.edge_positions.len() % 6, 0);
        assert_eq!(mesh.edge_ids.len(), mesh.edge_positions.len() / 6);
    }

    #[test]
    fn tessellate_face_carries_boundary_edge_positions() {
        let mut kernel = BrepkitKernel::new();
        let wire = kernel
            .polyline_wire_sync(&[[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 2.0, 0.0], [0.0, 0.0, 0.0]])
            .unwrap();
        let face = kernel.planar_face_from_wire_sync(&wire).unwrap();
        let mesh = kernel.tessellate_to_mesh_data_sync(&face, 0.1).unwrap();
        assert!(mesh.triangle_count() > 0);
        assert!(mesh.edge_positions.len() >= 6);
        assert_eq!(mesh.edge_positions.len() % 6, 0);
    }

    type SolidCodec = (Box<dyn SolidExporter>, Box<dyn SolidImporter>, f64);

    #[test]
    fn solid_exporters_and_importers_round_trip_a_box_per_format() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let original_volume = kernel.volume_sync(&solid).unwrap();
        // 🔩 STEP is exact NURBS round-trip; STL/OBJ/GLB reimport a re-tessellated mesh, so allow a
        // small deflection-driven volume error instead of an exact match.
        let codecs: Vec<SolidCodec> = vec![
            (Box::new(StepSolidExporter), Box::new(StepSolidImporter), 1e-6),
            (Box::new(StlSolidExporter), Box::new(StlSolidImporter), 0.05),
            (Box::new(ObjSolidExporter), Box::new(ObjSolidImporter), 0.05),
            (Box::new(GlbSolidExporter), Box::new(GlbSolidImporter), 0.05),
        ];
        for (exporter, importer, tolerance) in codecs {
            let format = exporter.format();
            assert_eq!(format, importer.format());
            let bytes = exporter.export(&kernel, std::slice::from_ref(&solid), 0.1).expect("export");
            assert!(!bytes.is_empty(), "{format:?} export must not be empty");
            let imported = importer.import(&mut kernel, &bytes, 0.1).expect("import");
            assert!(!imported.is_empty(), "{format:?} import must yield at least one solid");
            let mut imported_volume = 0.0;
            for handle in &imported {
                imported_volume += kernel.volume_sync(handle).unwrap();
            }
            assert!((imported_volume - original_volume).abs() < original_volume * tolerance, "{format:?} round trip should preserve volume: original={original_volume} imported={imported_volume}");
            for handle in &imported {
                let mesh = kernel.tessellate_sync(handle, 0.1).unwrap();
                assert!(!mesh.position.is_empty(), "{format:?} round-tripped solid must still tessellate to a non-empty mesh");
                assert!(!mesh.index.is_empty(), "{format:?} round-tripped solid must still tessellate to non-empty indices");
            }
        }
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
    fn sphere_cut_cylinder_completes() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(2.8).unwrap();
        let cylinder = kernel.cylinder_prim_sync(0.5, 4.0).unwrap();
        let cut = kernel.cut_sync(&sphere, &cylinder).unwrap();
        let volume = kernel.volume_sync(&cut).unwrap();
        assert!(volume > 0.0);
        assert!(volume < 92.0);
    }

    #[test]
    fn sphere_cut_disjoint_torus_completes() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(2.8).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        let moved = kernel.translate_sync(&torus, [20.0, 0.0, 0.0]).unwrap();
        let cut = kernel.cut_sync(&sphere, &moved).unwrap();
        let volume = kernel.volume_sync(&cut).unwrap();
        assert!((volume - 92.0).abs() < 2.0);
    }

    #[test]
    fn sphere_cut_intersecting_torus_completes() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(2.8).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        let cut = kernel.cut_sync(&sphere, &torus).unwrap();
        let volume = kernel.volume_sync(&cut).unwrap();
        assert!(volume > 0.0);
        assert!(volume < 92.0);
        let mesh = kernel.tessellate_sync(&cut, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
    }

    #[test]
    fn fixture_sphere_cut_torus_volume_is_less_than_sphere() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(2.2).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        let sphere_vol = kernel.volume_sync(&sphere).unwrap();
        let intersect = kernel.intersect_sync(&sphere, &torus).unwrap();
        let intersect_vol = kernel.volume_sync(&intersect).unwrap();
        let cut = kernel.cut_sync(&sphere, &torus).unwrap();
        let cut_vol = kernel.volume_sync(&cut).unwrap();
        let cut_tris = kernel.tessellate_sync(&cut, 0.1).unwrap().index.len() / 3;
        assert!(intersect_vol > 0.0, "sphere and torus should overlap, intersect_vol={intersect_vol}");
        assert!((sphere_vol - cut_vol - intersect_vol).abs() < sphere_vol * 0.15, "cut+intersect should approximate sphere: sphere={sphere_vol} cut={cut_vol} intersect={intersect_vol}");
        assert!(cut_vol < sphere_vol * 0.85, "cut vol {cut_vol} should be well below sphere vol {sphere_vol}");
        assert!(cut_tris > 800, "cut mesh should retain enough triangles for a visible torus tunnel, got {cut_tris}");
    }

    #[test]
    fn fixture_sphere_cut_torus_at_slider_max_completes() {
        let mut kernel = BrepkitKernel::new();
        let sphere = kernel.sphere_prim_sync(10.0).unwrap();
        let torus = kernel.torus_prim_sync(2.0, 0.5).unwrap();
        let cut = kernel.cut_sync(&sphere, &torus).unwrap();
        let volume = kernel.volume_sync(&cut).unwrap();
        assert!(volume > 0.0);
        let mesh = kernel.tessellate_sync(&cut, 0.1).unwrap();
        assert!(!mesh.position.is_empty());
        assert!(!mesh.index.is_empty());
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

    #[test]
    fn line_curve_evaluate() {
        let mut kernel = BrepkitKernel::new();
        let line = kernel.line_curve_sync([0.0, 0.0, 0.0], [2.0, 0.0, 0.0]).unwrap();
        let mid = kernel.curve_point_sync(&line, 1.0).unwrap();
        assert!((mid[0] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn extrude_rectangle_volume() {
        let mut kernel = BrepkitKernel::new();
        let wire = kernel.rectangle_wire_sync(2.0, 2.0).unwrap();
        let face = kernel.planar_face_from_wire_sync(&wire).unwrap();
        let solid = kernel.extrude_sync(&face, [0.0, 0.0, 1.0], 3.0).unwrap();
        let vol = kernel.volume_sync(&solid).unwrap();
        assert!((vol - 12.0).abs() < 0.5);
    }

    #[test]
    fn section_box_returns_faces() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 2.0, 2.0).unwrap();
        let faces = kernel.section_sync(&solid, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).unwrap();
        assert!(!faces.is_empty());
    }

    #[test]
    fn box_surface_area() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).unwrap();
        let area = kernel.area_sync(&solid).unwrap();
        assert!((area - 52.0).abs() < 1.0);
    }

    #[test]
    fn step_export_import_roundtrip_stub() {
        let mut kernel = BrepkitKernel::new();
        let solid = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
        let step = kernel.export_step_sync(std::slice::from_ref(&solid)).unwrap();
        assert!(step.contains("ISO-10303"));
        let imported = kernel.import_step_sync(&step).unwrap();
        assert_eq!(imported.len(), 1);
        assert!(kernel.volume_sync(&imported[0]).unwrap() > 0.0);
    }

    #[test]
    fn curve_tessellation_produces_edges() {
        let mut kernel = BrepkitKernel::new();
        let curve = kernel.line_curve_sync([0.0, 0.0, 0.0], [10.0, 0.0, 0.0]).unwrap();
        let mesh = kernel.tessellate_sync(&curve, 0.1).unwrap();
        assert!(!mesh.edges.is_empty());
    }

    #[test]
    fn sweep_wire_profile_produces_tube_mesh() {
        let mut kernel = BrepkitKernel::new();
        let path_wire = kernel.polyline_wire_sync(&[[0.0, 0.0, 0.0], [4.0, 0.0, 0.0]]).unwrap();
        let profile_wire = kernel.regular_polygon_wire_sync(0.08, 8).unwrap();
        let profile_face = kernel.planar_face_from_wire_sync(&profile_wire).unwrap();
        let solid = kernel.sweep_sync(&profile_face, &path_wire).unwrap();
        let mesh = kernel.tessellate_sync(&solid, 0.1).unwrap();
        assert!(mesh.position.len() > 36);
        assert!(mesh.index.len() > 12);
    }
}
// #endregion 🔖Tests
