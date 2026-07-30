//! ⚙️ Puzzle 3d app — headless compute (constitutional: engine).
#![allow(clippy::missing_errors_doc, reason = "Internal puzzle 3d WASM bundle.")]

use puzzle_3d::{Puzzle3dError, Puzzle3dProjection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//#region 🔒GeometryAdapter
/// 🔒 Thin wrappers over `nalgebra`/`parry3d` — the one interface boundary this crate depends on.

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec3d(nalgebra::Vector3<f32>);

impl Vec3d {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(nalgebra::Vector3::new(x, y, z))
    }
    fn x(&self) -> f32 {
        self.0.x
    }
    fn y(&self) -> f32 {
        self.0.y
    }
    fn z(&self) -> f32 {
        self.0.z
    }
    fn amax(&self) -> f32 {
        self.0.amax()
    }
}

impl std::ops::Add for Vec3d {
    type Output = Vec3d;
    fn add(self, rhs: Self) -> Self {
        Vec3d(self.0 + rhs.0)
    }
}

impl std::ops::Mul<f32> for Vec3d {
    type Output = Vec3d;
    fn mul(self, rhs: f32) -> Self {
        Vec3d(self.0 * rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point3d(nalgebra::Point3<f32>);

impl Point3d {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(nalgebra::Point3::new(x, y, z))
    }
    fn x(&self) -> f32 {
        self.0.x
    }
    fn y(&self) -> f32 {
        self.0.y
    }
    fn z(&self) -> f32 {
        self.0.z
    }
    fn inf(&self, other: &Self) -> Self {
        Self(self.0.inf(&other.0))
    }
    fn sup(&self, other: &Self) -> Self {
        Self(self.0.sup(&other.0))
    }
    fn coords(&self) -> Vec3d {
        Vec3d(self.0.coords)
    }
    fn from_coords(v: Vec3d) -> Self {
        Self(nalgebra::Point3::from(v.0))
    }
}

impl std::ops::Sub for Point3d {
    type Output = Vec3d;
    fn sub(self, rhs: Self) -> Vec3d {
        Vec3d(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, Debug)]
struct Rotation3d(nalgebra::UnitQuaternion<f32>);

impl Rotation3d {
    fn identity() -> Self {
        Self(nalgebra::UnitQuaternion::identity())
    }
    /// 🔓 Builds from CAD's `[i, j, k, w]` quaternion convention.
    fn from_ijkw(i: f32, j: f32, k: f32, w: f32) -> Self {
        Self(nalgebra::UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(w, i, j, k)))
    }
    fn to_ijkw(self) -> (f32, f32, f32, f32) {
        let q = self.0.quaternion();
        (q.i, q.j, q.k, q.w)
    }
    fn rotation_between(from: Vec3d, to: Vec3d) -> Option<Self> {
        nalgebra::UnitQuaternion::rotation_between(&from.0, &to.0).map(Self)
    }
    fn apply(&self, v: Vec3d) -> Vec3d {
        Vec3d(self.0 * v.0)
    }
}

#[derive(Clone, Copy, Debug)]
struct Pose3d(nalgebra::Isometry3<f32>);

impl Pose3d {
    fn identity() -> Self {
        Self(nalgebra::Isometry3::identity())
    }
    fn from_parts(translation: Vec3d, rotation: Rotation3d) -> Self {
        Self(nalgebra::Isometry3::from_parts(translation.0.into(), rotation.0))
    }
    fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }
    fn transform_point(&self, point: &Point3d) -> Point3d {
        Point3d(self.0 * point.0)
    }
    fn compose(&self, other: &Self) -> Self {
        Self(self.0 * other.0)
    }
}

struct CollisionShape(parry3d::shape::SharedShape);

impl CollisionShape {
    fn from_triangle_mesh(vertices: &[Point3d], indices: Vec<[u32; 3]>) -> Self {
        let verts: Vec<nalgebra::Point3<f32>> = vertices.iter().map(|p| p.0).collect();
        let mesh = parry3d::shape::TriMesh::with_flags(verts, indices, parry3d::shape::TriMeshFlags::ORIENTED | parry3d::shape::TriMeshFlags::MERGE_DUPLICATE_VERTICES);
        Self(parry3d::shape::SharedShape::new(mesh))
    }
    fn contains_point(&self, pose: &Pose3d, point: &Point3d) -> bool {
        self.0.contains_point(&pose.0, &point.0)
    }
}

fn shapes_intersect(pose_a: &Pose3d, a: &CollisionShape, pose_b: &Pose3d, b: &CollisionShape) -> bool {
    parry3d::query::intersection_test(&pose_a.0, &*a.0, &pose_b.0, &*b.0).unwrap_or(false)
}
//#endregion 🔒GeometryAdapter

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use wasm_bindgen::prelude::*;

const SURFACE_CONTACT_MAX_AABB_VOLUME: f64 = 1e-4;
const BRUSH_COLLISION_MESH_MIN_EXTENT: f64 = 2.0;
const BRUSH_PLACEMENT_PARALLEL_TOLERANCE: f64 = 1e-6;
const DEFAULT_CABLE_KIND_ID: &str = "cable.link";
const FILL_COUNT_MAX: usize = 1000;

type Quat = [f64; 4];
type Vec3 = [f64; 3];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrushHostRules {
    #[serde(default)]
    reject_capital_on_tambour: bool,
    #[serde(default)]
    reject_last_single_storey_on_mid_tambour: bool,
    #[serde(default)]
    door_tambour_requires_door_capsule: bool,
    #[serde(default = "default_door_capsule_min_abs_x")]
    door_capsule_min_abs_x: f64,
    #[serde(default = "default_door_capsule_max_abs_y")]
    door_capsule_max_abs_y: f64,
}

fn default_door_capsule_min_abs_x() -> f64 {
    0.9
}

fn default_door_capsule_max_abs_y() -> f64 {
    1.6
}

impl Default for BrushHostRules {
    fn default() -> Self {
        Self {
            reject_capital_on_tambour: true,
            reject_last_single_storey_on_mid_tambour: true,
            door_tambour_requires_door_capsule: true,
            door_capsule_min_abs_x: default_door_capsule_min_abs_x(),
            door_capsule_max_abs_y: default_door_capsule_max_abs_y(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct BrushKindWeights {
    #[serde(default)]
    object_weights: HashMap<String, f64>,
    #[serde(default)]
    vortex_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KindCompatEntry {
    source: String,
    target: String,
    #[serde(default)]
    bidirectional: bool,
    #[serde(default)]
    important: bool,
    specificity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ObjectKindVortexTemplate {
    #[serde(rename = "vortexKind", default)]
    vortex_kind: Option<String>,
    position: Vec3,
    direction: Option<Vec3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ObjectKind {
    id: String,
    #[serde(rename = "meshUrl", default)]
    mesh_url: Option<String>,
    #[serde(default)]
    scale: Option<serde_json::Value>,
    #[serde(default)]
    vortices: Vec<ObjectKindVortexTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VortexKindCatalog {
    id: String,
    #[serde(rename = "defaultCableKind", default)]
    default_cable_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CableKindCatalog {
    id: String,
    #[serde(rename = "defaultAttractionKind", default)]
    default_attraction_kind: Option<String>,
}

/// 🗂️ The compile-time-catalog side of a scene: object/vortex/cable kind rows, reachable through
/// `apply_brush_placement_to_fixture`'s public signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindCatalogBundle {
    #[serde(default)]
    objects: Vec<ObjectKind>,
    #[serde(default)]
    vortices: Vec<VortexKindCatalog>,
    #[serde(default)]
    cables: Vec<CableKindCatalog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VortexProps {
    id: String,
    #[serde(rename = "vortexKind", default)]
    vortex_kind: Option<String>,
    position: Vec3,
    direction: Option<Vec3>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FixtureObject {
    id: String,
    #[serde(rename = "objectKind", default)]
    object_kind: Option<String>,
    #[serde(rename = "meshUrl", default)]
    mesh_url: Option<String>,
    origin: Vec3,
    orientation: Option<Quat>,
    scale: Option<serde_json::Value>,
    #[serde(default)]
    vortices: Vec<VortexProps>,
    /// 🪣 Live-viewport-only tag (never persisted to the document): this object's 0-based position in
    /// the fill plan's sequence, so the viewport can reveal/hide planned pieces by drag position without
    /// a WASM round trip. Set only on `compose_fill_display`'s output, stripped from committed fixtures.
    #[serde(rename = "revealIndex", default, skip_serializing_if = "Option::is_none")]
    reveal_index: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttractionProps {
    #[serde(default)]
    id: String,
    attracting: String,
    attracted: String,
    #[serde(default)]
    gap: f64,
    #[serde(default)]
    shift: f64,
    #[serde(default)]
    rise: f64,
    #[serde(default)]
    rotation: f64,
    #[serde(default)]
    turn: f64,
    #[serde(default)]
    tilt: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorldVolumeProps {
    id: String,
    origin: Vec3,
    #[serde(default)]
    orientation: Option<Quat>,
    #[serde(default)]
    scale: Option<serde_json::Value>,
}

/// 🏗️ A puzzle-3d scene's object/attraction/target-volume state, reachable through
/// `apply_brush_placement_to_fixture`'s public signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    #[serde(default)]
    attractions: Vec<AttractionProps>,
    #[serde(default)]
    objects: Vec<FixtureObject>,
    #[serde(default, rename = "targetVolumes")]
    target_volumes: Vec<WorldVolumeProps>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SceneConfig {
    fixture: Fixture,
    #[serde(rename = "kindCatalogs", default)]
    kind_catalogs: Option<KindCatalogBundle>,
    #[serde(rename = "kindCompatibility", default)]
    kind_compatibility: Vec<KindCompatEntry>,
    #[serde(rename = "overlapBudget", default)]
    overlap_budget: f64,
    #[serde(default)]
    seed: u32,
    #[serde(rename = "hostRules", default)]
    host_rules: BrushHostRules,
    #[serde(default)]
    weights: BrushKindWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushCompatibleCandidate {
    object_kind_id: String,
    source_vortex_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushPreviewState {
    pub target_vortex_full_id: String,
    pub object_kind_id: String,
    pub source_vortex_index: usize,
    pub mesh_url: String,
    pub origin: Vec3,
    pub orientation: Quat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrushCollisionFreeResult {
    free: Vec<BrushCompatibleCandidate>,
    unknown_pending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrushPlacePayload {
    target_vortex_full_id: String,
    object_kind_id: String,
    source_vortex_index: usize,
    origin: Vec3,
    orientation: Quat,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillBuildProgress {
    count: usize,
    applied_count: usize,
    max_count: usize,
    done: bool,
    #[serde(default)]
    appended_objects: Vec<FixtureObject>,
    #[serde(default)]
    appended_attractions: Vec<AttractionProps>,
    #[serde(default)]
    sequence: Vec<BrushPlacePayload>,
}

#[derive(Debug, Clone)]
struct AttractionVortexContext {
    object_kind: Option<String>,
    vortex_kind: Option<String>,
}

struct CollisionMeshPart {
    shape: CollisionShape,
    local_pose: Pose3d,
}

struct CollisionBody {
    parts: Vec<CollisionMeshPart>,
    local_bounds_min: Point3d,
    local_bounds_max: Point3d,
}

#[derive(Clone)]
struct PlacedCollisionEntry {
    object_id: String,
    mesh_url: String,
    world: Pose3d,
}

#[derive(Clone)]
struct BrushFillVortexTarget {
    full_id: String,
    object_id: String,
    object_kind: Option<String>,
    vortex_kind: Option<String>,
    vortex_index: usize,
}

#[derive(Debug, Clone)]
enum PrecomputeTask {
    BrushTarget(String),
    FillStep,
}

fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}

fn normalize_vec3(v: Vec3) -> Vec3 {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-9 {
        return [0.0, 0.0, -1.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

fn vec3_dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn vec3_cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn negate_vec3(v: Vec3) -> Vec3 {
    [-v[0], -v[1], -v[2]]
}

fn vec3_scale(v: Vec3, scale: &Option<serde_json::Value>) -> Vec3 {
    match scale {
        None => v,
        Some(serde_json::Value::Number(n)) => {
            let s = n.as_f64().unwrap_or(1.0);
            [v[0] * s, v[1] * s, v[2] * s]
        }
        Some(serde_json::Value::Array(arr)) if arr.len() >= 3 => {
            let sx = arr[0].as_f64().unwrap_or(1.0);
            let sy = arr[1].as_f64().unwrap_or(1.0);
            let sz = arr[2].as_f64().unwrap_or(1.0);
            [v[0] * sx, v[1] * sy, v[2] * sz]
        }
        _ => v,
    }
}

fn unit_quat_from_cad(q: Quat) -> Rotation3d {
    Rotation3d::from_ijkw(q[0] as f32, q[1] as f32, q[2] as f32, q[3] as f32)
}

fn quat_rotate_vec(q: Quat, v: Vec3) -> Vec3 {
    let uq = unit_quat_from_cad(q);
    let rotated = uq.apply(Vec3d::new(v[0] as f32, v[1] as f32, v[2] as f32));
    [rotated.x() as f64, rotated.y() as f64, rotated.z() as f64]
}

fn quaternion_from_180_degree_axis(axis: Vec3) -> Quat {
    let unit = normalize_vec3(axis);
    [unit[0], unit[1], unit[2], 0.0]
}

fn anti_parallel_brush_orientation(target_dir: Vec3) -> Quat {
    let z_axis: Vec3 = [0.0, 0.0, 1.0];
    if target_dir[2].abs() < BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        return quaternion_from_180_degree_axis(z_axis);
    }
    let axis = vec3_cross(z_axis, target_dir);
    if (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt() < BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        return quaternion_from_180_degree_axis([1.0, 0.0, 0.0]);
    }
    quaternion_from_180_degree_axis(axis)
}

fn pose_isometry(origin: Vec3, orientation: Quat, _scale: &Option<serde_json::Value>) -> Pose3d {
    let q = unit_quat_from_cad(orientation);
    let t = Vec3d::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
    Pose3d::from_parts(t, q)
}

fn compute_brush_placement_pose(
    source_local_position: Vec3,
    source_local_direction: Vec3,
    scale: &Option<serde_json::Value>,
    target_world_position: Vec3,
    target_world_direction: Vec3,
    reference_orientation: Option<Quat>,
    use_host_orientation: bool,
) -> (Vec3, Quat) {
    let scaled_local = vec3_scale(source_local_position, scale);
    let local_dir = normalize_vec3(source_local_direction);
    let target_dir = normalize_vec3(target_world_direction);
    if use_host_orientation {
        if let Some(host_orientation) = reference_orientation {
            let world_source_dir = normalize_vec3(quat_rotate_vec(host_orientation, local_dir));
            if vec3_dot(world_source_dir, target_dir) < -BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
                let origin = vec3_sub(target_world_position, quat_rotate_vec(host_orientation, scaled_local));
                return (origin, host_orientation);
            }
        }
    }
    let desired_world_dir = negate_vec3(target_dir);
    let orientation = if vec3_dot(local_dir, desired_world_dir) < -1.0 + BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        anti_parallel_brush_orientation(target_dir)
    } else {
        let from = Vec3d::new(local_dir[0] as f32, local_dir[1] as f32, local_dir[2] as f32);
        let to = Vec3d::new(desired_world_dir[0] as f32, desired_world_dir[1] as f32, desired_world_dir[2] as f32);
        let q = Rotation3d::rotation_between(from, to).unwrap_or(Rotation3d::identity());
        let (i, j, k, w) = q.to_ijkw();
        [i as f64, j as f64, k as f64, w as f64]
    };
    let origin = vec3_sub(target_world_position, quat_rotate_vec(orientation, scaled_local));
    (origin, orientation)
}

fn collision_body_from_buffers(positions: &[f32], indices: &[u32]) -> Option<CollisionBody> {
    if positions.len() < 9 || indices.len() < 3 {
        return None;
    }
    let mut verts: Vec<Point3d> = Vec::with_capacity(positions.len() / 3);
    let mut min = Point3d::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3d::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for chunk in positions.as_chunks::<3>().0 {
        let rp = Point3d::new(chunk[0], chunk[1], chunk[2]);
        verts.push(rp);
        min = min.inf(&rp);
        max = max.sup(&rp);
    }
    let extent = (max - min).amax();
    if !extent.is_finite() || extent < BRUSH_COLLISION_MESH_MIN_EXTENT as f32 {
        return None;
    }
    let mut tris: Vec<[u32; 3]> = Vec::with_capacity(indices.len() / 3);
    for chunk in indices.as_chunks::<3>().0 {
        tris.push(*chunk);
    }
    let shape = CollisionShape::from_triangle_mesh(&verts, tris);
    Some(CollisionBody { parts: vec![CollisionMeshPart { shape, local_pose: Pose3d::identity() }], local_bounds_min: min, local_bounds_max: max })
}

fn world_bounds(body: &CollisionBody, world: &Pose3d) -> (Point3d, Point3d) {
    let corners = [
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_min.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_min.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_max.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_max.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_min.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_min.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_max.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_max.y(), body.local_bounds_max.z()),
    ];
    let mut min = Point3d::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3d::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for corner in corners {
        let w = world.transform_point(&corner);
        min = min.inf(&w);
        max = max.sup(&w);
    }
    (min, max)
}

fn volume_scale_vec(scale: &Option<serde_json::Value>) -> [f32; 3] {
    match scale {
        Some(serde_json::Value::Number(n)) => {
            let s = n.as_f64().unwrap_or(1.0) as f32;
            [s, s, s]
        }
        Some(serde_json::Value::Array(values)) if values.len() == 3 => {
            let read = |index: usize| values.get(index).and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
            [read(0), read(1), read(2)]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

fn world_volumes_contain_aabb(volumes: &[WorldVolumeProps], min: Point3d, max: Point3d) -> bool {
    if volumes.is_empty() {
        return true;
    }
    let corners = [
        min,
        Point3d::new(max.x(), min.y(), min.z()),
        Point3d::new(min.x(), max.y(), min.z()),
        Point3d::new(max.x(), max.y(), min.z()),
        Point3d::new(min.x(), min.y(), max.z()),
        Point3d::new(max.x(), min.y(), max.z()),
        Point3d::new(min.x(), max.y(), max.z()),
        max,
    ];
    for volume in volumes {
        let scale = volume_scale_vec(&volume.scale);
        let world = pose_isometry(volume.origin, volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &None);
        let inv = world.inverse();
        let hx = 0.5 + 1e-3;
        let hy = 0.5 + 1e-3;
        let hz = 0.5 + 1e-3;
        let mut inside = true;
        for corner in corners {
            let relative = inv.transform_point(&corner);
            let local = Point3d::new(relative.x() / scale[0], relative.y() / scale[1], relative.z() / scale[2]);
            if local.x().abs() > hx || local.y().abs() > hy || local.z().abs() > hz {
                inside = false;
                break;
            }
        }
        if inside {
            return true;
        }
    }
    false
}

fn point_inside_body(body: &CollisionBody, world: &Pose3d, point: Point3d) -> bool {
    let local = world.inverse().transform_point(&point);
    for part in &body.parts {
        let part_local = part.local_pose.inverse().transform_point(&local);
        if part.shape.contains_point(&part.local_pose, &part_local) {
            return true;
        }
    }
    false
}

fn bodies_intersect(a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> bool {
    let (amin, amax) = world_bounds(a, world_a);
    let (bmin, bmax) = world_bounds(b, world_b);
    if amax.x() < bmin.x() || bmax.x() < amin.x() || amax.y() < bmin.y() || bmax.y() < amin.y() || amax.z() < bmin.z() || bmax.z() < amin.z() {
        return false;
    }
    for part_a in &a.parts {
        let pose_a = world_a.compose(&part_a.local_pose);
        for part_b in &b.parts {
            let pose_b = world_b.compose(&part_b.local_pose);
            if shapes_intersect(&pose_a, &part_a.shape, &pose_b, &part_b.shape) {
                return true;
            }
        }
    }
    let center = Point3d::from_coords((amin.coords() + amax.coords() + bmin.coords() + bmax.coords()) * 0.25);
    point_inside_body(a, world_a, center) && point_inside_body(b, world_b, center)
}

fn solid_overlap_volume(a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d, sample_count: usize, overlap_budget: f64) -> f64 {
    let (amin, amax) = world_bounds(a, world_a);
    let (bmin, bmax) = world_bounds(b, world_b);
    let imin = Point3d::new(amin.x().max(bmin.x()), amin.y().max(bmin.y()), amin.z().max(bmin.z()));
    let imax = Point3d::new(amax.x().min(bmax.x()), amax.y().min(bmax.y()), amax.z().min(bmax.z()));
    if imax.x() < imin.x() || imax.y() < imin.y() || imax.z() < imin.z() {
        return 0.0;
    }
    if !bodies_intersect(a, world_a, b, world_b) {
        return 0.0;
    }
    let size = imax - imin;
    let box_vol = size.x() as f64 * size.y() as f64 * size.z() as f64;
    if box_vol <= SURFACE_CONTACT_MAX_AABB_VOLUME {
        return 0.0;
    }
    let mut inside_both = 0usize;
    let mut state = 0x9e3779b9u32;
    for _ in 0..sample_count {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let rx = (state as f64) / (u32::MAX as f64);
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let ry = (state as f64) / (u32::MAX as f64);
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let rz = (state as f64) / (u32::MAX as f64);
        let p = Point3d::new(imin.x() + (imax.x() - imin.x()) * rx as f32, imin.y() + (imax.y() - imin.y()) * ry as f32, imin.z() + (imax.z() - imin.z()) * rz as f32);
        if point_inside_body(a, world_a, p) && point_inside_body(b, world_b, p) {
            inside_both += 1;
            if (inside_both as f64 / sample_count as f64) * box_vol > overlap_budget {
                return overlap_budget + 1.0;
            }
        }
    }
    (inside_both as f64 / sample_count as f64) * box_vol
}

fn puzzle3d_vortex_port_shape(vortex_kind: &str) -> Option<&'static str> {
    if vortex_kind.contains(" circular ") {
        Some("circular")
    } else if vortex_kind.contains(" rectangular ") {
        Some("rectangular")
    } else {
        None
    }
}

fn puzzle3d_vortex_port_shapes_compatible(source: &str, target: &str) -> bool {
    match (puzzle3d_vortex_port_shape(source), puzzle3d_vortex_port_shape(target)) {
        (None, _) | (_, None) => true,
        (Some(a), Some(b)) => a == b,
    }
}

fn puzzle3d_single_letter_port_family(vortex_kind: &str) -> Option<char> {
    let head = vortex_kind.split('-').next()?;
    if head.len() == 1 {
        let ch = head.chars().next()?;
        if ch.is_ascii_lowercase() {
            return Some(ch);
        }
    }
    None
}

fn puzzle3d_single_letter_port_families_compatible(source: &str, target: &str) -> bool {
    match (puzzle3d_single_letter_port_family(source), puzzle3d_single_letter_port_family(target)) {
        (None, _) | (_, None) => true,
        (Some(a), Some(b)) => a == b,
    }
}

fn catalog_vortex_by_id<'a>(catalogs: &'a KindCatalogBundle, vortex_kind: &str) -> Option<&'a VortexKindCatalog> {
    catalogs.vortices.iter().find(|v| v.id == vortex_kind)
}

fn catalog_cable_by_id<'a>(catalogs: &'a KindCatalogBundle, cable_kind: &str) -> Option<&'a CableKindCatalog> {
    catalogs.cables.iter().find(|w| w.id == cable_kind)
}

fn resolve_cable_kind_for_vortex(vortex_kind: &str, catalogs: &KindCatalogBundle) -> String {
    catalog_vortex_by_id(catalogs, vortex_kind).and_then(|v| v.default_cable_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_CABLE_KIND_ID.to_string())
}

fn resolve_attraction_kind_for_cable(cable_kind: &str, catalogs: &KindCatalogBundle) -> String {
    catalog_cable_by_id(catalogs, cable_kind).and_then(|c| c.default_attraction_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
}

fn compat_pair_matches(rule: &KindCompatEntry, a: &str, b: &str) -> bool {
    (rule.source == a && rule.target == b) || (rule.bidirectional && rule.source == b && rule.target == a)
}

fn specificity_rank(spec: Option<&str>) -> i32 {
    match spec {
        Some("general") => 0,
        Some("object") => 1,
        Some("attraction") => 2,
        Some("cable") => 3,
        Some("vortex") => 4,
        _ => 4,
    }
}

fn attraction_gesture_rule_applies(rule: &KindCompatEntry, attracting: &AttractionVortexContext, attracted: &AttractionVortexContext, catalogs: &KindCatalogBundle) -> bool {
    let cable_src = resolve_cable_kind_for_vortex(attracting.vortex_kind.as_deref().unwrap_or(""), catalogs);
    let cable_tgt = resolve_cable_kind_for_vortex(attracted.vortex_kind.as_deref().unwrap_or(""), catalogs);
    let attraction_src = resolve_attraction_kind_for_cable(&cable_src, catalogs);
    let attraction_tgt = resolve_attraction_kind_for_cable(&cable_tgt, catalogs);
    let sn = attracting.object_kind.as_deref().unwrap_or("");
    let tn = attracted.object_kind.as_deref().unwrap_or("");
    let sv = attracting.vortex_kind.as_deref().unwrap_or("");
    let tv = attracted.vortex_kind.as_deref().unwrap_or("");
    match rule.specificity.as_deref().unwrap_or("vortex") {
        "general" => compat_pair_matches(rule, sv, tv),
        "object" => compat_pair_matches(rule, sn, tn),
        "attraction" => compat_pair_matches(rule, &attraction_src, &attraction_tgt),
        "vortex" => compat_pair_matches(rule, sv, tv),
        "cable" => compat_pair_matches(rule, &cable_src, &cable_tgt),
        _ => compat_pair_matches(rule, sv, tv),
    }
}

fn vortices_attraction_compatible_for_drag(attracting: &AttractionVortexContext, attracted: &AttractionVortexContext, rules: &[KindCompatEntry], catalogs: &KindCatalogBundle) -> bool {
    let sv = attracting.vortex_kind.as_deref().unwrap_or("");
    let tv = attracted.vortex_kind.as_deref().unwrap_or("");
    if !puzzle3d_vortex_port_shapes_compatible(sv, tv) {
        return false;
    }
    if !puzzle3d_single_letter_port_families_compatible(sv, tv) {
        return false;
    }
    if rules.is_empty() {
        return true;
    }
    let mut matched: Vec<&KindCompatEntry> = rules.iter().filter(|r| attraction_gesture_rule_applies(r, attracting, attracted, catalogs)).collect();
    if matched.is_empty() {
        return false;
    }
    if matched.iter().any(|r| r.important) {
        matched.retain(|r| r.important);
    } else {
        let max_rank = matched.iter().map(|r| specificity_rank(r.specificity.as_deref())).max().unwrap_or(4);
        matched.retain(|r| specificity_rank(r.specificity.as_deref()) == max_rank);
    }
    !matched.is_empty()
}

fn brush_stack_vortex_base(vortex_kind: &str) -> Option<&str> {
    if let Some(base) = vortex_kind.strip_suffix(" bottom") {
        Some(base)
    } else if let Some(base) = vortex_kind.strip_suffix(" top") {
        Some(base)
    } else {
        None
    }
}

fn brush_stack_bottom_top_pair(source: &str, target: &str) -> bool {
    let (Some(sb), Some(tb)) = (brush_stack_vortex_base(source), brush_stack_vortex_base(target)) else {
        return false;
    };
    source.ends_with(" bottom") && target.ends_with(" top") && sb == tb
}

fn brush_stack_top_bottom_pair(source: &str, target: &str) -> bool {
    let (Some(sb), Some(tb)) = (brush_stack_vortex_base(source), brush_stack_vortex_base(target)) else {
        return false;
    };
    source.ends_with(" top") && target.ends_with(" bottom") && sb == tb
}

fn brush_stack_mate_pair(source: &str, target: &str) -> bool {
    if !puzzle3d_vortex_port_shapes_compatible(source, target) {
        return false;
    }
    brush_stack_bottom_top_pair(source, target) || brush_stack_top_bottom_pair(source, target)
}

fn brush_candidate_rank(candidate: &BrushCompatibleCandidate, template: &ObjectKindVortexTemplate, target: &AttractionVortexContext) -> i64 {
    let mut score: i64 = 0;
    let target_kind = target.vortex_kind.as_deref().unwrap_or("");
    let source_kind = template.vortex_kind.as_deref().unwrap_or("");
    if candidate.object_kind_id == target.object_kind.as_deref().unwrap_or("") {
        score += 10_000;
    }
    if brush_stack_mate_pair(source_kind, target_kind) {
        score += 5_000;
    }
    if source_kind == target_kind && !brush_stack_mate_pair(source_kind, target_kind) {
        score -= 4_000;
    }
    if target_kind.ends_with(" top") && !brush_stack_mate_pair(source_kind, target_kind) {
        score -= 2_000;
    }
    if target_kind.ends_with(" bottom") && !source_kind.ends_with(" top") {
        score -= 2_000;
    }
    if target_kind.contains("tambour circular") || target_kind.contains("tambour rectangular") {
        let host_kind = target.object_kind.as_deref().unwrap_or("");
        let mid_tambour_host = host_kind == "Tambour" || host_kind == "Cylindric Tambour";
        if candidate.object_kind_id.contains("Capital") {
            score -= 50_000;
        } else if candidate.object_kind_id.contains("Cylindric") && candidate.object_kind_id.contains("Tambour") {
            score += 11_000;
        }
        if mid_tambour_host && (candidate.object_kind_id.contains("Last Storey") || candidate.object_kind_id.contains("Single Storey")) {
            score -= 30_000;
        }
        if mid_tambour_host && candidate.object_kind_id == "Cylindric Tambour" {
            score += 5_000;
        }
    }
    score
}

fn host_accepts_candidate(rules: &BrushHostRules, target: &AttractionVortexContext, candidate: &BrushCompatibleCandidate, template: &ObjectKindVortexTemplate) -> bool {
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    if rules.reject_capital_on_tambour && (target_vk.contains("tambour circular") || target_vk.contains("tambour rectangular")) && candidate.object_kind_id.contains("Capital") {
        return false;
    }
    let host_kind = target.object_kind.as_deref().unwrap_or("");
    if rules.reject_last_single_storey_on_mid_tambour
        && (target_vk.contains("tambour circular") || target_vk.contains("tambour rectangular"))
        && (host_kind == "Tambour" || host_kind == "Cylindric Tambour")
        && (candidate.object_kind_id.contains("Last Storey") || candidate.object_kind_id.contains("Single Storey"))
    {
        return false;
    }
    if !rules.door_tambour_requires_door_capsule || !target_vk.contains("door tambour") {
        return true;
    }
    let source_vk = template.vortex_kind.as_deref().unwrap_or("");
    if !source_vk.contains("door capsule") {
        return false;
    }
    let x = template.position[0].abs();
    let y = template.position[1].abs();
    x >= rules.door_capsule_min_abs_x && y < rules.door_capsule_max_abs_y
}

fn brush_placement_uses_host_orientation(target: &AttractionVortexContext, source_vk: &str, candidate_kind: &str) -> bool {
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    if brush_stack_mate_pair(source_vk, target_vk) {
        return false;
    }
    if source_vk != target_vk {
        return false;
    }
    candidate_kind == target.object_kind.as_deref().unwrap_or("")
}

fn catalog_object_kind_by_id<'a>(catalogs: &'a KindCatalogBundle, id: &str) -> Option<&'a ObjectKind> {
    catalogs.objects.iter().find(|k| k.id == id)
}

fn resolve_object_kind_mesh_url(kind_id: &str, catalogs: &KindCatalogBundle, fixture: &Fixture) -> Option<String> {
    if let Some(kind) = catalog_object_kind_by_id(catalogs, kind_id) {
        if let Some(url) = kind.mesh_url.as_ref().filter(|u| !u.is_empty()) {
            return Some(url.clone());
        }
    }
    fixture.objects.iter().find(|o| o.object_kind.as_deref() == Some(kind_id)).and_then(|o| o.mesh_url.clone())
}

fn brush_compatible_candidates(target: &AttractionVortexContext, catalogs: &KindCatalogBundle, rules: &[KindCompatEntry], host_rules: &BrushHostRules) -> Vec<BrushCompatibleCandidate> {
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    let stack_top_target = target_vk.ends_with(" top");
    let stack_bottom_target = target_vk.ends_with(" bottom");
    let mut scored: Vec<(BrushCompatibleCandidate, i64)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for kind in &catalogs.objects {
        if kind.mesh_url.as_ref().map(|u| u.is_empty()).unwrap_or(true) || kind.vortices.is_empty() {
            continue;
        }
        for (source_vortex_index, template) in kind.vortices.iter().enumerate() {
            let source_vk = template.vortex_kind.as_deref().unwrap_or("");
            if stack_top_target && !brush_stack_mate_pair(source_vk, target_vk) {
                continue;
            }
            if stack_bottom_target && !brush_stack_mate_pair(source_vk, target_vk) {
                continue;
            }
            let attracting = AttractionVortexContext { object_kind: Some(kind.id.clone()), vortex_kind: Some(source_vk.to_string()) };
            if !vortices_attraction_compatible_for_drag(&attracting, target, rules, catalogs) {
                continue;
            }
            let candidate = BrushCompatibleCandidate { object_kind_id: kind.id.clone(), source_vortex_index };
            if !host_accepts_candidate(host_rules, target, &candidate, template) {
                continue;
            }
            let key = format!("{}\u{1}{}", candidate.object_kind_id, candidate.source_vortex_index);
            if !seen.insert(key) {
                continue;
            }
            let rank = brush_candidate_rank(&candidate, template, target);
            scored.push((candidate, rank));
        }
    }
    scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.object_kind_id.cmp(&b.0.object_kind_id)).then_with(|| a.0.source_vortex_index.cmp(&b.0.source_vortex_index)));
    scored.into_iter().map(|(c, _)| c).collect()
}

fn blocked_vortex_full_ids(attractions: &[AttractionProps]) -> std::collections::HashSet<String> {
    let mut s = std::collections::HashSet::new();
    for a in attractions {
        s.insert(a.attracting.clone());
        s.insert(a.attracted.clone());
    }
    s
}

fn vortex_world_from_object(obj: &FixtureObject, vortex_index: usize) -> Option<(Vec3, Vec3)> {
    let vortex = obj.vortices.get(vortex_index)?;
    let orientation = obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let position = vec3_add(obj.origin, quat_rotate_vec(orientation, vortex.position));
    let direction = normalize_vec3(quat_rotate_vec(orientation, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
    Some((position, direction))
}

fn enumerate_brush_fill_vortex_targets(fixture: &Fixture) -> Vec<BrushFillVortexTarget> {
    let blocked = blocked_vortex_full_ids(&fixture.attractions);
    let mut out = Vec::new();
    for obj in &fixture.objects {
        for (i, vortex) in obj.vortices.iter().enumerate() {
            let full_id = puzzle3d_vortex_full_id(&obj.id, &vortex.id);
            if !blocked.contains(&full_id) {
                out.push(BrushFillVortexTarget { full_id, object_id: obj.id.clone(), object_kind: obj.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone(), vortex_index: i });
            }
        }
    }
    out
}

fn brush_kind_weight_value(weights: &HashMap<String, f64>, id: &str) -> f64 {
    weights.get(id).copied().unwrap_or(1.0)
}

fn brush_candidate_suggestion_weight(candidate: &BrushCompatibleCandidate, weights: &BrushKindWeights, catalogs: &KindCatalogBundle) -> f64 {
    let vortex_kind = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
    brush_kind_weight_value(&weights.object_weights, &candidate.object_kind_id) * brush_kind_weight_value(&weights.vortex_weights, vortex_kind)
}

fn brush_target_vortex_allows_suggestion(vortex_kind: Option<&str>, weights: &BrushKindWeights) -> bool {
    brush_kind_weight_value(&weights.vortex_weights, vortex_kind.unwrap_or("")) > 0.0
}

fn fill_vortex_target_weight(target: &BrushFillVortexTarget, weights: &BrushKindWeights) -> f64 {
    brush_kind_weight_value(&weights.vortex_weights, target.vortex_kind.as_deref().unwrap_or(""))
}

fn weighted_sample_without_replacement<T, F>(items: &[T], weight_of: F, rng_state: &mut u32) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> f64,
{
    let eligible: Vec<T> = items.iter().filter(|item| weight_of(item) > 0.0).cloned().collect();
    if eligible.len() < 2 {
        return eligible;
    }
    let mut remaining = eligible;
    let mut out = Vec::new();
    while !remaining.is_empty() {
        let w_list: Vec<f64> = remaining.iter().map(&weight_of).collect();
        let total: f64 = w_list.iter().sum();
        if total <= 0.0 {
            break;
        }
        let mut r = fill_rng(rng_state) * total;
        let mut pick = remaining.len() - 1;
        for (i, weight) in w_list.iter().enumerate() {
            r -= weight;
            if r <= 0.0 {
                pick = i;
                break;
            }
        }
        out.push(remaining[pick].clone());
        remaining.remove(pick);
    }
    out
}

fn fill_rng(rng_state: &mut u32) -> f64 {
    *rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
    *rng_state as f64 / 4_294_967_296.0
}

fn weighted_order_fill_vortex_targets(targets: &[BrushFillVortexTarget], weights: &BrushKindWeights, rng_state: &mut u32) -> Vec<BrushFillVortexTarget> {
    weighted_sample_without_replacement(targets, |target| fill_vortex_target_weight(target, weights), rng_state)
}

fn weighted_order_brush_compatible_candidates(candidates: &[BrushCompatibleCandidate], weights: &BrushKindWeights, catalogs: &KindCatalogBundle, rng_state: &mut u32) -> Vec<BrushCompatibleCandidate> {
    weighted_sample_without_replacement(candidates, |candidate| brush_candidate_suggestion_weight(candidate, weights, catalogs), rng_state)
}

fn fill_candidate_diversity_score(candidate: &BrushCompatibleCandidate, target_vortex_index: usize, target_object_kind: Option<&str>) -> i64 {
    if target_object_kind != Some(candidate.object_kind_id.as_str()) {
        return 0;
    }
    1000 + (candidate.source_vortex_index as i64 - target_vortex_index as i64).unsigned_abs() as i64 * 100
}

fn order_brush_fill_compatible_candidates(
    candidates: &[BrushCompatibleCandidate],
    target_vortex_kind: Option<&str>,
    target_vortex_index: usize,
    target_object_kind: Option<&str>,
    catalogs: &KindCatalogBundle,
    weights: &BrushKindWeights,
    rng_state: &mut u32,
) -> Vec<BrushCompatibleCandidate> {
    let allowed: Vec<BrushCompatibleCandidate> = candidates.iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, weights, catalogs) > 0.0).cloned().collect();
    let target = target_vortex_kind.unwrap_or("");
    let mut cross = Vec::new();
    let mut same = Vec::new();
    for candidate in allowed {
        let source_vk = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
        if source_vk != target || brush_stack_mate_pair(source_vk, target) {
            cross.push(candidate);
        } else {
            same.push(candidate);
        }
    }
    cross.sort_by(|left, right| {
        fill_candidate_diversity_score(right, target_vortex_index, target_object_kind)
            .cmp(&fill_candidate_diversity_score(left, target_vortex_index, target_object_kind))
            .then_with(|| left.object_kind_id.cmp(&right.object_kind_id))
            .then_with(|| left.source_vortex_index.cmp(&right.source_vortex_index))
    });
    let mut same_sorted = same;
    same_sorted.sort_by(|left, right| left.object_kind_id.cmp(&right.object_kind_id).then_with(|| left.source_vortex_index.cmp(&right.source_vortex_index)));
    cross.extend(weighted_order_brush_compatible_candidates(&same_sorted, weights, catalogs, rng_state));
    cross
}

/// 🎯 A target vortex's world-space pose, bundled so `brush_preview_from_candidate` stays under clippy's arg-count limit.
#[derive(Clone, Copy)]
struct TargetVortexWorld {
    position: Vec3,
    direction: Vec3,
    reference_orientation: Option<Quat>,
}

fn brush_preview_from_candidate(target_full_id: &str, candidate: &BrushCompatibleCandidate, target: &AttractionVortexContext, world: TargetVortexWorld, catalogs: &KindCatalogBundle, fixture: &Fixture) -> Option<BrushPreviewState> {
    let kind = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id)?;
    let template = kind.vortices.get(candidate.source_vortex_index)?;
    let mesh_url = resolve_object_kind_mesh_url(&candidate.object_kind_id, catalogs, fixture)?;
    let source_vk = template.vortex_kind.as_deref().unwrap_or("");
    let use_host = brush_placement_uses_host_orientation(target, source_vk, &candidate.object_kind_id);
    let (origin, orientation) = compute_brush_placement_pose(template.position, template.direction.unwrap_or([0.0, 0.0, -1.0]), &kind.scale, world.position, world.direction, world.reference_orientation, use_host);
    Some(BrushPreviewState { target_vortex_full_id: target_full_id.to_string(), object_kind_id: kind.id.clone(), source_vortex_index: candidate.source_vortex_index, mesh_url, origin, orientation, scale: kind.scale.clone() })
}

/// ⏱ Monotonic-enough wall clock in milliseconds — `Date.now()` only for wasm-bindgen web targets.
/// WASI P2 program components (`target_env = "p2"`) must not call `js_sys` (no wasm-bindgen imports);
/// they share the native `Instant` path so precompute budgets still advance.
/// @link https://docs.rs/js-sys/latest/js_sys/struct.Date.html#method.now
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn puzzle3d_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
fn puzzle3d_now_ms() -> f64 {
    use std::sync::OnceLock;
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    START.get_or_init(std::time::Instant::now).elapsed().as_secs_f64() * 1000.0
}

/// 🪫 Soft wall-clock ceiling for a single `precompute_step` call — a `FillStep` task's own collision
/// search cost is otherwise unbounded per call, so this only caps how many *additional* tasks beyond
/// the first are attempted once time runs out; the first task in a call always runs so a tick always
/// makes forward progress.
const PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS: f64 = 12.0;

struct FillBuilder {
    base: Fixture,
    fixture: Fixture,
    applied_count: usize,
    sequence: Vec<BrushPlacePayload>,
    appended_objects: Vec<FixtureObject>,
    appended_attractions: Vec<AttractionProps>,
    placed: Vec<PlacedCollisionEntry>,
    candidate_cache: HashMap<String, Vec<BrushCompatibleCandidate>>,
    seed_object_ids: std::collections::HashSet<String>,
    rng_state: u32,
    stalled: bool,
    max_count: usize,
}

impl FillBuilder {
    fn new(base: Fixture, seed: u32, meshes: &HashMap<String, CollisionBody>, catalogs: &KindCatalogBundle) -> Self {
        let seed_object_ids: std::collections::HashSet<String> = base.objects.iter().map(|o| o.id.clone()).collect();
        let mut placed = Vec::new();
        for obj in &base.objects {
            if let Some(mesh_url) = resolve_object_kind_mesh_url(obj.object_kind.as_deref().unwrap_or(""), catalogs, &base) {
                if meshes.contains_key(&mesh_url) {
                    placed.push(PlacedCollisionEntry { object_id: obj.id.clone(), mesh_url, world: pose_isometry(obj.origin, obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &obj.scale) });
                }
            }
        }
        Self {
            base: base.clone(),
            fixture: base,
            applied_count: 0,
            sequence: Vec::new(),
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            placed,
            candidate_cache: HashMap::new(),
            seed_object_ids,
            rng_state: seed,
            stalled: false,
            max_count: FILL_COUNT_MAX,
        }
    }

    fn progress(&self) -> FillBuildProgress {
        FillBuildProgress {
            count: self.sequence.len(),
            applied_count: self.applied_count,
            max_count: self.max_count,
            done: self.stalled || self.sequence.len() >= self.max_count,
            appended_objects: self.appended_objects.clone(),
            appended_attractions: self.appended_attractions.clone(),
            sequence: self.sequence.clone(),
        }
    }
}

struct Puzzle3dEngine {
    scene: Option<SceneConfig>,
    /// 🧊 Raw JSON of the last `set_scene` call, so a resync with byte-identical config (every action
    /// re-syncs the session, see `sync_precompute_session`) can skip `rebuild_queue` instead of wiping
    /// `brush_cache`/`fill`/`queue` and restarting suggestion+fill precompute from zero every time.
    scene_json: Option<String>,
    meshes: HashMap<String, CollisionBody>,
    brush_cache: HashMap<String, BrushCollisionFreeResult>,
    fill: Option<FillBuilder>,
    queue: Vec<PrecomputeTask>,
}

impl Puzzle3dEngine {
    fn new() -> Self {
        Self { scene: None, scene_json: None, meshes: HashMap::new(), brush_cache: HashMap::new(), fill: None, queue: Vec::new() }
    }

    fn rebuild_queue(&mut self) {
        self.queue.clear();
        self.brush_cache.clear();
        if let Some(scene) = &self.scene {
            for target in enumerate_brush_fill_vortex_targets(&scene.fixture) {
                self.queue.push(PrecomputeTask::BrushTarget(target.full_id));
            }
            for _ in 0..FILL_COUNT_MAX {
                self.queue.push(PrecomputeTask::FillStep);
            }
            let catalogs = scene.kind_catalogs.clone().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
            self.fill = Some(FillBuilder::new(scene.fixture.clone(), scene.seed, &self.meshes, &catalogs));
        } else {
            self.fill = None;
        }
    }

    /// 🎚 Distribution-weight edits must not `rebuild_queue()` — applied fill objects stay, only the
    /// unapplied planning tail is discarded and re-enqueued for background `fillBuildTick` planning.
    fn soft_replan_fill_tail(&mut self) {
        let Some(fill) = &mut self.fill else {
            return;
        };
        let applied = fill.applied_count;
        fill.sequence.truncate(applied);
        fill.appended_objects.truncate(applied);
        fill.appended_attractions.truncate(applied);
        fill.fixture = fill.base.clone();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        let retained_ids: std::collections::HashSet<&str> = fill.fixture.objects.iter().map(|object| object.id.as_str()).collect();
        fill.placed.retain(|entry| retained_ids.contains(entry.object_id.as_str()));
        fill.candidate_cache.clear();
        fill.stalled = false;
        self.queue.retain(|task| !matches!(task, PrecomputeTask::FillStep));
        self.queue.extend((applied..fill.max_count).map(|_| PrecomputeTask::FillStep));
    }

    fn update_kind_weights(&mut self, object_weights: HashMap<String, f64>, vortex_weights: HashMap<String, f64>) {
        if let Some(scene) = &mut self.scene {
            scene.weights.object_weights = object_weights;
            scene.weights.vortex_weights = vortex_weights;
            if let Ok(normalized) = serde_json::to_string(scene) {
                self.scene_json = Some(normalized);
            }
        }
        self.brush_cache.clear();
        self.soft_replan_fill_tail();
    }

    /// 🪣 True when `fixture` is the fill plan's base plus zero-or-more applied fill objects — i.e. the
    /// live document after `setFillCount`, which must NOT rebuild the precompute session or the slider
    /// loses its ability to remove/replan those objects.
    fn is_fill_applied_projection(fixture: &Fixture, fill: &FillBuilder) -> bool {
        let plan_objects: std::collections::HashSet<&str> = fill.appended_objects.iter().map(|object| object.id.as_str()).collect();
        let plan_attractions: std::collections::HashSet<&str> = fill.appended_attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        let base_objects: std::collections::HashSet<&str> = fill.base.objects.iter().map(|object| object.id.as_str()).collect();
        let base_attractions: std::collections::HashSet<&str> = fill.base.attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        let base_volumes: std::collections::HashSet<&str> = fill.base.target_volumes.iter().map(|volume| volume.id.as_str()).collect();
        let incoming_objects: std::collections::HashSet<&str> = fixture.objects.iter().map(|object| object.id.as_str()).filter(|id| !plan_objects.contains(id)).collect();
        let incoming_attractions: std::collections::HashSet<&str> = fixture.attractions.iter().map(|attraction| attraction.id.as_str()).filter(|id| !plan_attractions.contains(id)).collect();
        let incoming_volumes: std::collections::HashSet<&str> = fixture.target_volumes.iter().map(|volume| volume.id.as_str()).collect();
        incoming_objects == base_objects && incoming_attractions == base_attractions && incoming_volumes == base_volumes
    }

    fn strip_fill_plan_from_fixture(fixture: &mut Fixture, fill: &FillBuilder) {
        let plan_objects: std::collections::HashSet<&str> = fill.appended_objects.iter().map(|object| object.id.as_str()).collect();
        let plan_attractions: std::collections::HashSet<&str> = fill.appended_attractions.iter().map(|attraction| attraction.id.as_str()).collect();
        fixture.objects.retain(|object| !plan_objects.contains(object.id.as_str()));
        fixture.attractions.retain(|attraction| !plan_attractions.contains(attraction.id.as_str()));
    }

    fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        let mut scene: SceneConfig = serde_json::from_str(json)?;
        // 🪣 After the fill slider materializes objects into the document, every incidental action
        // (hover, pick, mesh register sync, …) re-feeds that applied projection here. Treating it as a
        // brand-new scene used to `rebuild_queue()` and bake the filled objects into `fill.base`, after
        // which the slider could neither remove them nor replan a fresh tail.
        if self.fill.as_ref().is_some_and(|fill| Self::is_fill_applied_projection(&scene.fixture, fill)) {
            if let Some(fill) = &self.fill {
                Self::strip_fill_plan_from_fixture(&mut scene.fixture, fill);
            }
            let normalized = serde_json::to_string(&scene)?;
            if let Some(current) = &mut self.scene {
                current.overlap_budget = scene.overlap_budget;
                current.seed = scene.seed;
                current.weights = scene.weights;
                current.kind_catalogs = scene.kind_catalogs;
                current.kind_compatibility = scene.kind_compatibility;
                current.host_rules = scene.host_rules;
            }
            self.scene_json = Some(normalized);
            return Ok(());
        }
        let normalized = serde_json::to_string(&scene)?;
        if self.scene_json.as_deref() == Some(normalized.as_str()) {
            return Ok(());
        }
        self.scene = Some(scene);
        self.scene_json = Some(normalized);
        self.rebuild_queue();
        Ok(())
    }

    fn register_mesh(&mut self, url: String, positions: Vec<f32>, indices: Vec<u32>) {
        if let Some(body) = collision_body_from_buffers(&positions, &indices) {
            self.meshes.insert(url, body);
            // 🧊 A newly registered/replaced mesh invalidates any cached brush candidates/fill progress
            // computed against a different (or fallback-box) body for this url.
            self.rebuild_queue();
        }
    }

    fn has_mesh(&self, url: &str) -> bool {
        self.meshes.contains_key(url)
    }

    /// 🧊 Drops a cached brush-candidate entry and re-queues that vortex at the front so a just-opened
    /// suggestion popup is not stuck on a stale empty / pending result.
    fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.brush_cache.remove(vortex_full_id);
        self.queue.retain(|task| !matches!(task, PrecomputeTask::BrushTarget(id) if id == vortex_full_id));
        self.queue.insert(0, PrecomputeTask::BrushTarget(vortex_full_id.to_string()));
    }

    /// 🧊 Recomputes and caches brush candidates for one vortex immediately (used when opening / accepting
    /// the suggestion popup so the UI does not wait on the background queue).
    fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        let result = self.compute_brush_cache_entry(vortex_full_id);
        self.brush_cache.insert(vortex_full_id.to_string(), result);
    }

    fn preview_collides(meshes: &HashMap<String, CollisionBody>, preview: &BrushPreviewState, placed: &[PlacedCollisionEntry], overlap_budget: f64, sample_count: usize) -> Option<bool> {
        let preview_body = meshes.get(&preview.mesh_url)?;
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        for entry in placed {
            let other = meshes.get(&entry.mesh_url)?;
            let vol = solid_overlap_volume(preview_body, &preview_world, other, &entry.world, sample_count, overlap_budget);
            if vol > overlap_budget {
                return Some(true);
            }
        }
        Some(false)
    }

    fn brush_collision_free(&self, target_full_id: &str, candidates: &[BrushCompatibleCandidate], overlap_budget: f64) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true };
        };
        let empty_catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let catalogs = scene.kind_catalogs.as_ref().unwrap_or(&empty_catalogs);
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, vortex_index, _)) = target_obj else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false };
        };
        let Some((position, direction)) = vortex_world_from_object(host, vortex_index) else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[vortex_index].vortex_kind.clone() };
        let host_id = host.id.clone();
        let placed: Vec<PlacedCollisionEntry> = scene
            .fixture
            .objects
            .iter()
            .filter(|obj| obj.id != host_id)
            .filter_map(|obj| {
                let mesh_url = resolve_object_kind_mesh_url(obj.object_kind.as_deref().unwrap_or(""), catalogs, &scene.fixture)?;
                if !self.meshes.contains_key(&mesh_url) {
                    return None;
                }
                Some(PlacedCollisionEntry { object_id: obj.id.clone(), mesh_url, world: pose_isometry(obj.origin, obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &obj.scale) })
            })
            .collect();
        let mut free = Vec::new();
        let mut unknown_pending = false;
        for candidate in candidates {
            let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
            let Some(preview) = brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, catalogs, &scene.fixture) else {
                continue;
            };
            if !self.meshes.contains_key(&preview.mesh_url) {
                unknown_pending = true;
                continue;
            }
            match Self::preview_collides(&self.meshes, &preview, &placed, overlap_budget, 1024) {
                None => unknown_pending = true,
                Some(true) => {}
                Some(false) => free.push(candidate.clone()),
            }
        }
        BrushCollisionFreeResult { free, unknown_pending }
    }

    fn compute_brush_cache_entry(&self, target_full_id: &str) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: true };
        };
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|o| {
            o.vortices.iter().enumerate().find_map(|(i, v)| {
                let full_id = puzzle3d_vortex_full_id(&o.id, &v.id);
                if full_id == target_full_id {
                    Some((o, i, v))
                } else {
                    None
                }
            })
        });
        let Some((host, _, vortex)) = target_obj else {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false };
        };
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone() };
        if !brush_target_vortex_allows_suggestion(vortex.vortex_kind.as_deref(), &scene.weights) {
            return BrushCollisionFreeResult { free: vec![], unknown_pending: false };
        }
        let compatible = brush_compatible_candidates(&target_ctx, &catalogs, &scene.kind_compatibility, &scene.host_rules);
        let compatible: Vec<BrushCompatibleCandidate> = compatible.into_iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &catalogs) > 0.0).collect();
        self.brush_collision_free(target_full_id, &compatible, scene.overlap_budget)
    }

    pub fn brush_preview_json(&self, target_full_id: &str, candidate_index: usize) -> Option<String> {
        let scene = self.scene.as_ref()?;
        let result = self.brush_cache.get(target_full_id).cloned().unwrap_or_else(|| self.compute_brush_cache_entry(target_full_id));
        if result.free.is_empty() {
            return None;
        }
        let candidate = &result.free[candidate_index % result.free.len()];
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let target_obj = scene.fixture.objects.iter().find_map(|object| {
            object.vortices.iter().enumerate().find_map(|(index, vortex)| {
                let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                if full_id == target_full_id {
                    Some((object, index))
                } else {
                    None
                }
            })
        })?;
        let (host, vortex_index) = target_obj;
        let (position, direction) = vortex_world_from_object(host, vortex_index)?;
        let target_ctx = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[vortex_index].vortex_kind.clone() };
        let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
        let preview = brush_preview_from_candidate(target_full_id, candidate, &target_ctx, world, &catalogs, &scene.fixture)?;
        serde_json::to_string(&preview).ok()
    }

    fn precompute_step(&mut self, budget: u32) -> bool {
        let start = puzzle3d_now_ms();
        let mut remaining = budget as usize;
        let mut steps_done = 0usize;
        while remaining > 0 {
            if steps_done > 0 && puzzle3d_now_ms() - start >= PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS {
                break;
            }
            let task = match self.queue.first().cloned() {
                Some(t) => t,
                None => return false,
            };
            match task {
                PrecomputeTask::BrushTarget(full_id) => {
                    let result = self.compute_brush_cache_entry(&full_id);
                    self.brush_cache.insert(full_id, result);
                    self.queue.remove(0);
                }
                PrecomputeTask::FillStep => {
                    if !self.fill_step_one() {
                        while matches!(self.queue.first(), Some(PrecomputeTask::FillStep)) {
                            self.queue.remove(0);
                        }
                    } else {
                        self.queue.remove(0);
                    }
                }
            }
            steps_done += 1;
            remaining -= 1;
        }
        !self.queue.is_empty()
    }

    fn fill_step_one(&mut self) -> bool {
        let Some(scene) = &self.scene else {
            return false;
        };
        let Some(fill) = &mut self.fill else {
            return false;
        };
        if fill.stalled || fill.sequence.len() >= fill.max_count {
            fill.stalled = true;
            return false;
        }
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] });
        let overlap_budget = scene.overlap_budget;
        let weights = scene.weights.clone();
        let kind_compatibility = scene.kind_compatibility.clone();
        let host_rules = scene.host_rules.clone();
        let free_targets = enumerate_brush_fill_vortex_targets(&fill.fixture);
        if free_targets.is_empty() {
            fill.stalled = true;
            return false;
        }
        let seed_targets: Vec<_> = free_targets.iter().filter(|t| fill.seed_object_ids.contains(&t.object_id)).cloned().collect();
        let frontier_targets: Vec<_> = free_targets.iter().filter(|t| !fill.seed_object_ids.contains(&t.object_id)).cloned().collect();
        let ordered_targets: Vec<_> = weighted_order_fill_vortex_targets(&seed_targets, &weights, &mut fill.rng_state).into_iter().chain(weighted_order_fill_vortex_targets(&frontier_targets, &weights, &mut fill.rng_state)).collect();
        if ordered_targets.is_empty() {
            fill.stalled = true;
            return false;
        }
        let target_start = fill.sequence.len() % ordered_targets.len();
        for target_offset in 0..ordered_targets.len() {
            let target = &ordered_targets[(target_start + target_offset) % ordered_targets.len()];
            let Some(host) = fill.fixture.objects.iter().find(|o| o.id == target.object_id) else {
                continue;
            };
            let Some((position, direction)) = vortex_world_from_object(host, target.vortex_index) else {
                continue;
            };
            let target_ctx = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
            let key = format!("{}\u{1}{}", target.object_kind.as_deref().unwrap_or(""), target.vortex_kind.as_deref().unwrap_or(""));
            let compatible = fill.candidate_cache.entry(key).or_insert_with(|| brush_compatible_candidates(&target_ctx, &catalogs, &kind_compatibility, &host_rules)).clone();
            if compatible.is_empty() {
                continue;
            }
            let ordered_candidates = order_brush_fill_compatible_candidates(&compatible, target.vortex_kind.as_deref(), target.vortex_index, target.object_kind.as_deref(), &catalogs, &weights, &mut fill.rng_state);
            if ordered_candidates.is_empty() {
                continue;
            }
            for candidate in &ordered_candidates {
                let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
                let Some(preview) = brush_preview_from_candidate(&target.full_id, candidate, &target_ctx, world, &catalogs, &fill.fixture) else {
                    continue;
                };
                if !self.meshes.contains_key(&preview.mesh_url) {
                    continue;
                }
                let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
                if let Some(body) = self.meshes.get(&preview.mesh_url) {
                    let (min, max) = world_bounds(body, &preview_world);
                    if !world_volumes_contain_aabb(&scene.fixture.target_volumes, min, max) {
                        continue;
                    }
                }
                let placed_snapshot: Vec<PlacedCollisionEntry> = fill.placed.iter().filter(|entry| entry.object_id != target.object_id).cloned().collect();
                match Self::preview_collides(&self.meshes, &preview, &placed_snapshot, overlap_budget, 512) {
                    None | Some(true) => continue,
                    Some(false) => {}
                }
                let payload = BrushPlacePayload {
                    target_vortex_full_id: preview.target_vortex_full_id.clone(),
                    object_kind_id: preview.object_kind_id.clone(),
                    source_vortex_index: preview.source_vortex_index,
                    origin: preview.origin,
                    orientation: preview.orientation,
                    scale: preview.scale.clone(),
                };
                let next_fixture = apply_brush_placement_to_fixture(&fill.fixture, &payload, &catalogs);
                if next_fixture.objects.len() == fill.fixture.objects.len() {
                    continue;
                }
                // 🔒 Infallible: the length check above proves `apply_brush_placement_to_fixture` actually
                // appended (rather than returning `fixture.clone()` unchanged), and it only ever appends
                // exactly one object together with exactly one attraction, never one without the other.
                let mut placed_object = next_fixture.objects.last().cloned().expect("objects grew, so last() is Some");
                if let Some(mesh_url) = resolve_object_kind_mesh_url(placed_object.object_kind.as_deref().unwrap_or(""), &catalogs, &next_fixture) {
                    if self.meshes.contains_key(&mesh_url) {
                        fill.placed.push(PlacedCollisionEntry { object_id: placed_object.id.clone(), mesh_url, world: pose_isometry(placed_object.origin, placed_object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &placed_object.scale) });
                    }
                }
                let new_attraction = next_fixture.attractions.last().cloned().expect("attractions grew alongside objects, so last() is Some");
                fill.fixture = next_fixture;
                fill.sequence.push(payload);
                // 🪣 Tag with its sequence position so `compose_fill_display` can expose it as `revealIndex`.
                placed_object.reveal_index = Some(fill.appended_objects.len());
                fill.appended_objects.push(placed_object);
                fill.appended_attractions.push(new_attraction);
                return true;
            }
        }
        fill.stalled = true;
        false
    }

    /// 🔽 Moving the count down (or up) only changes which prefix of the already-planned sequence is
    /// applied to the document — the plan (`sequence`/`appended_*`/`placed`/`fixture`) is prefix-stable
    /// and is never discarded here, so a jittery drag can never force expensive replanning.
    fn apply_fill_count(&mut self, count: usize) -> Option<Fixture> {
        let fill = self.fill.as_mut()?;
        let count = count.min(fill.sequence.len());
        fill.applied_count = count;
        let mut fixture = fill.base.clone();
        // 🪣 `revealIndex` is a live-viewport-only hint (see `compose_fill_display`) — never persist it
        // to the committed document projection.
        fixture.objects.extend(fill.appended_objects.iter().take(count).cloned().map(|mut object| {
            object.reveal_index = None;
            object
        }));
        fixture.attractions.extend(fill.appended_attractions.iter().take(count).cloned());
        Some(fixture)
    }

    /// 🪣 Read-only prefix of the precomputed fill plan for live viewport show/hide — does not mutate
    /// `applied_count`, the queue, or the document projection.
    fn compose_fill_display(&self, count: usize) -> Option<Fixture> {
        let fill = self.fill.as_ref()?;
        let visible = count.min(fill.sequence.len());
        let mut fixture = fill.base.clone();
        fixture.objects.extend(fill.appended_objects.iter().take(visible).cloned());
        fixture.attractions.extend(fill.appended_attractions.iter().take(visible).cloned());
        Some(fixture)
    }

    fn apply_brush_placement(&mut self, payload: &BrushPlacePayload) -> Option<Fixture> {
        let catalogs = self.scene.as_ref()?.kind_catalogs.as_ref()?.clone();
        let fixture = &self.scene.as_ref()?.fixture;
        let next = apply_brush_placement_to_fixture(fixture, payload, &catalogs);
        if next.objects.len() == fixture.objects.len() {
            return None;
        }
        if let Some(scene) = &mut self.scene {
            scene.fixture = next.clone();
        }
        self.rebuild_queue();
        Some(next)
    }
}

pub fn apply_brush_placement_to_fixture(fixture: &Fixture, payload: &BrushPlacePayload, catalogs: &KindCatalogBundle) -> Fixture {
    let Some(kind) = catalog_object_kind_by_id(catalogs, &payload.object_kind_id) else {
        return fixture.clone();
    };
    let Some(template) = kind.vortices.get(payload.source_vortex_index) else {
        return fixture.clone();
    };
    let Some(mesh_url) = resolve_object_kind_mesh_url(&payload.object_kind_id, catalogs, fixture) else {
        return fixture.clone();
    };
    let object_id = format!("puzzle3d.brush.{}", uuid_simple());
    let vortices: Vec<VortexProps> = kind.vortices.iter().enumerate().map(|(index, entry)| VortexProps { id: format!("{object_id}:v{index}"), vortex_kind: entry.vortex_kind.clone(), position: entry.position, direction: entry.direction }).collect();
    // 🌲 The new object attaches as `attracted`: the pre-existing target vortex it's docking onto stays the
    // resolution root. Params start at zero (a bare port-to-port docking); `puzzle3d_rederive_all_attractions`
    // (puzzle/plugin/rs/d3/mod.rs) rederives them from this placement's actual pose right after merge, so the
    // object never visibly jumps when the directed-attraction resolver runs.
    let attracted = puzzle3d_vortex_full_id(&object_id, &vortices[payload.source_vortex_index].id);
    let attraction_id = format!("attraction-{}-{attracted}", payload.target_vortex_full_id);
    let mut next = fixture.clone();
    if next.attractions.iter().any(|a| a.attracting == payload.target_vortex_full_id || a.attracted == attracted) {
        return fixture.clone();
    }
    next.attractions.push(AttractionProps { id: attraction_id, attracting: payload.target_vortex_full_id.clone(), attracted, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
    next.objects.push(FixtureObject { id: object_id, object_kind: Some(kind.id.clone()), mesh_url: Some(mesh_url), origin: payload.origin, orientation: Some(payload.orientation), scale: payload.scale.clone().or(kind.scale.clone()), vortices, reveal_index: None });
    let _ = template;
    next
}

/// 🔢 Guarantees uniqueness across calls even when `js_sys_time_now()` is frozen (native/test builds
/// always return `0.0`) or two calls land in the same millisecond (rapid-fire suggestion acceptance) —
/// without this, brush-placed objects collided on `object_id` and therefore on every derived vortex id,
/// so hover/suggestion-target lookups (keyed on those ids) silently applied to the wrong object.
static PUZZLE3D_UUID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn uuid_simple() -> String {
    let counter = PUZZLE3D_UUID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut state = (js_sys_time_now() as u64) ^ 0xdead_beef_cafe_babe ^ counter.wrapping_mul(0x9e37_79b9_7f4a_7c15).wrapping_add(1);
    let mut out = String::with_capacity(36);
    for (i, _) in (0..16).enumerate() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let byte = (state >> 33) as u8;
        if i == 4 || i == 6 || i == 8 || i == 10 {
            out.push('-');
        }
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
fn js_sys_time_now() -> f64 {
    0.0
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn js_sys_time_now() -> f64 {
    js_sys::Date::now()
}

/// 🔤 Parses `.puzzle3d` DSL text (`Puzzle3dProjection`'s `dsl::DslDocument` grammar) into the same camelCase JSON shape callers previously got from a hand-authored `*.3d.json` fixture — lets non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating the DSL grammar.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen(js_name = puzzle3dParseDslJson)]
pub fn puzzle3d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
    use store::DocumentDsl;
    let projection = Puzzle3dProjection::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&projection).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
pub struct Puzzle3dPrecomputeSession {
    engine: Puzzle3dEngine,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl Default for Puzzle3dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
impl Puzzle3dPrecomputeSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { engine: Puzzle3dEngine::new() }
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), JsValue> {
        self.engine.set_scene(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh(url.to_string(), positions.to_vec(), indices.to_vec());
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.engine.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.engine.precompute_step(budget)
    }

    pub fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.invalidate_brush_target(vortex_full_id);
    }

    pub fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        self.engine.refresh_brush_candidates(vortex_full_id);
    }

    pub fn brush_candidates(&self, vortex_full_id: &str) -> String {
        if let Some(hit) = self.engine.brush_cache.get(vortex_full_id) {
            return serde_json::to_string(hit).unwrap_or_else(|_| "{}".to_string());
        }
        let result = self.engine.compute_brush_cache_entry(vortex_full_id);
        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn brush_preview_json(&self, vortex_full_id: &str, candidate_index: usize) -> Option<String> {
        self.engine.brush_preview_json(vortex_full_id, candidate_index)
    }

    pub fn fill_progress(&self) -> String {
        let progress = self.engine.fill.as_ref().map(|f| f.progress()).unwrap_or(FillBuildProgress { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true, appended_objects: vec![], appended_attractions: vec![], sequence: vec![] });
        serde_json::to_string(&progress).unwrap_or_else(|_| "{}".to_string())
    }

    /// 🪣 O(1) planned-count readout for the render/tick hot path — avoids a `fill_progress` JSON
    /// round trip just to read `sequence.len()`.
    pub fn fill_available_count(&self) -> u32 {
        self.engine.fill.as_ref().map(|fill| fill.sequence.len() as u32).unwrap_or(0)
    }

    pub fn fill_is_done(&self) -> bool {
        self.engine.fill.as_ref().map(|fill| fill.stalled || fill.sequence.len() >= fill.max_count).unwrap_or(true)
    }

    pub fn apply_brush_placement_json(&mut self, payload_json: &str) -> Result<String, JsValue> {
        let payload: BrushPlacePayload = serde_json::from_str(payload_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let fixture = self.engine.apply_brush_placement(&payload).ok_or_else(|| JsValue::from_str("brush placement rejected"))?;
        serde_json::to_string(&fixture).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn apply_fill_count(&mut self, count: u32) -> Result<String, JsValue> {
        let fixture = self.engine.apply_fill_count(count as usize).ok_or_else(|| JsValue::from_str("fill session unavailable"))?;
        serde_json::to_string(&fixture).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn compose_fill_display(&self, count: u32) -> Result<String, JsValue> {
        let fixture = self.engine.compose_fill_display(count as usize).ok_or_else(|| JsValue::from_str("fill session unavailable"))?;
        serde_json::to_string(&fixture).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn update_kind_weights(&mut self, object_weights: &str, vortex_weights: &str) -> Result<(), JsValue> {
        let object_weights: HashMap<String, f64> = serde_json::from_str(object_weights).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let vortex_weights: HashMap<String, f64> = serde_json::from_str(vortex_weights).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.engine.update_kind_weights(object_weights, vortex_weights);
        Ok(())
    }
}

#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
pub struct Puzzle3dPrecomputeSession {
    engine: Puzzle3dEngine,
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_OVERLAP_BUDGET: f64 = 0.02;

    /// 🔗 Keeps the example fixture's scene-authored kind catalog in sync with the compile-time `puzzle3d-default` manifest.
    #[test]
    fn concrete_forest_kind_catalog_matches_puzzle3d_default_manifest() {
        use store::DocumentDsl;
        let fixture = Puzzle3dProjection::parse_dsl(include_str!("../../../../../../../../../✏️s/🔌plugin/🧩puzzle/🎛️app/🧊3d/⚡️implementation/🦀rust/📚example/🧩concrete-forest.puzzle3d")).unwrap();
        let catalogs: KindCatalogBundle = serde_json::from_value(serde_json::to_value(&fixture.meta.kind_catalogs).unwrap()).unwrap();
        let manifest = mathematical_graph_manifest::manifest_by_id("puzzle3d-default").expect("puzzle3d-default manifest must be registered");
        let wire_kind_ids: std::collections::BTreeSet<_> = manifest.wire_kinds.iter().map(|row| row.id.as_str()).collect();
        let edge_kind_ids: std::collections::BTreeSet<_> = manifest.edge_kinds.iter().map(|row| row.id.as_str()).collect();
        for vortex in &catalogs.vortices {
            if let Some(default_cable_kind) = &vortex.default_cable_kind {
                assert!(wire_kind_ids.contains(default_cable_kind.as_str()), "vortex kind {:?} references unknown wire kind {default_cable_kind:?}", vortex.id);
            }
        }
        for cable in &catalogs.cables {
            if let Some(default_attraction_kind) = &cable.default_attraction_kind {
                assert!(edge_kind_ids.contains(default_attraction_kind.as_str()), "cable kind {:?} references unknown edge kind {default_attraction_kind:?}", cable.id);
            }
        }
    }

    #[test]
    fn world_volumes_contain_aabb_respects_oriented_box() {
        let volumes = vec![WorldVolumeProps { id: "v1".to_string(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(serde_json::json!([4.0, 4.0, 4.0])) }];
        let min = Point3d::new(-1.0, -1.0, -1.0);
        let max = Point3d::new(1.0, 1.0, 1.0);
        assert!(world_volumes_contain_aabb(&volumes, min, max));
        let outside_min = Point3d::new(-3.0, -3.0, -3.0);
        let outside_max = Point3d::new(3.0, 3.0, 3.0);
        assert!(!world_volumes_contain_aabb(&volumes, outside_min, outside_max));
    }

    #[test]
    fn brush_candidates_allow_separated_boxes() {
        let mut engine = Puzzle3dEngine::new();
        let positions: Vec<f32> = vec![-4.0, -4.0, -4.0, 4.0, -4.0, -4.0, 4.0, 4.0, -4.0, -4.0, 4.0, -4.0, -4.0, -4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0];
        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
        engine.register_mesh("/test/obstacle.glb".to_string(), positions.clone(), indices.clone());
        engine.register_mesh("/test/preview.glb".to_string(), positions, indices);
        let scene = SceneConfig {
            fixture: Fixture {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![
                    FixtureObject {
                        id: "obstacle".to_string(),
                        object_kind: Some("Kind".to_string()),
                        mesh_url: Some("/test/obstacle.glb".to_string()),
                        origin: [0.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                        reveal_index: None,
                    },
                    FixtureObject {
                        id: "host".to_string(),
                        object_kind: Some("Host".to_string()),
                        mesh_url: Some("/test/unregistered.glb".to_string()),
                        origin: [12.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                        reveal_index: None,
                    },
                ],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![ObjectKind {
                    id: "Kind".to_string(),
                    mesh_url: Some("/test/preview.glb".to_string()),
                    scale: None,
                    vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                }],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None }],
                cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None }],
            }),
            kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
            overlap_budget: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        engine.scene = Some(scene);
        let result = engine.compute_brush_cache_entry("host:v0");
        assert!(!result.unknown_pending, "expected mesh-ready result");
        assert_eq!(result.free.len(), 1, "expected one collision-free candidate");
    }

    #[test]
    fn fill_distribution_excludes_zero_weight_vortices() {
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind {
                id: "Placed".to_string(),
                mesh_url: Some("/test/placed.glb".to_string()),
                scale: None,
                vortices: vec![
                    ObjectKindVortexTemplate { vortex_kind: Some("c-b".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]) },
                    ObjectKindVortexTemplate { vortex_kind: Some("b-s".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]) },
                ],
            }],
            vortices: vec![VortexKindCatalog { id: "c-b".to_string(), default_cable_kind: None }, VortexKindCatalog { id: "c-t".to_string(), default_cable_kind: None }, VortexKindCatalog { id: "b-s".to_string(), default_cable_kind: None }],
            cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None }],
        };
        let candidates = vec![BrushCompatibleCandidate { object_kind_id: "Placed".to_string(), source_vortex_index: 0 }, BrushCompatibleCandidate { object_kind_id: "Placed".to_string(), source_vortex_index: 1 }];
        let mut weights = BrushKindWeights::default();
        weights.vortex_weights.insert("c-b".to_string(), 0.0);
        weights.vortex_weights.insert("c-t".to_string(), 0.0);
        weights.vortex_weights.insert("b-s".to_string(), 1.0);
        weights.object_weights.insert("Placed".to_string(), 1.0);
        let mut rng = 7u32;
        let ordered = order_brush_fill_compatible_candidates(&candidates, Some("b-s"), 1, Some("Host"), &catalogs, &weights, &mut rng);
        assert_eq!(ordered.len(), 1);
        assert_eq!(ordered[0].source_vortex_index, 1);
        let targets = vec![
            BrushFillVortexTarget { full_id: "host:v0".to_string(), object_id: "host".to_string(), object_kind: Some("Host".to_string()), vortex_kind: Some("c-t".to_string()), vortex_index: 0 },
            BrushFillVortexTarget { full_id: "host:v1".to_string(), object_id: "host".to_string(), object_kind: Some("Host".to_string()), vortex_kind: Some("b-s".to_string()), vortex_index: 1 },
        ];
        let target_ordered = weighted_order_fill_vortex_targets(&targets, &weights, &mut rng);
        assert_eq!(target_ordered.len(), 1);
        assert_eq!(target_ordered[0].vortex_kind.as_deref(), Some("b-s"));
    }

    #[test]
    fn brush_placement_emits_attraction_with_id_and_directed_root() {
        let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind {
                id: "Placed".to_string(),
                mesh_url: Some("/test/placed.glb".to_string()),
                scale: None,
                vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
            }],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None }],
            cables: vec![],
        };
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".to_string(), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let next = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
        assert_eq!(next.attractions.len(), 1, "brush placement should append exactly one attraction");
        let attraction = &next.attractions[0];
        assert!(!attraction.id.is_empty(), "brush-placed attraction must carry a non-empty id (regression: engine attractions with no id were silently dropped by fixture_from_engine_json)");
        assert_eq!(attraction.attracting, "host:v0", "the pre-existing target vortex must stay the resolution root");
        assert!(attraction.attracted.starts_with(&format!("{}:", next.objects[0].id)), "the newly placed object's vortex must be the attracted (non-root) side");
        assert_eq!(attraction.gap, 0.0);
        assert_eq!(attraction.rotation, 0.0);
    }

    /// 🪪 Regression: `uuid_simple()` used to seed only from the (frozen-in-native-builds) clock, so two
    /// brush placements in a row minted the *same* object id — colliding vortex ids then made hover and
    /// suggestion-target lookups (keyed on those ids) silently apply to the wrong object.
    #[test]
    fn successive_brush_placements_never_collide_on_object_id() {
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind {
                id: "Placed".to_string(),
                mesh_url: Some("/test/placed.glb".to_string()),
                scale: None,
                vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
            }],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None }],
            cables: vec![],
        };
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".to_string(), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let mut fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
        let mut ids = std::collections::HashSet::new();
        for i in 0..8 {
            fixture = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
            let placed = fixture.objects.last().expect("placement should append an object");
            assert!(ids.insert(placed.id.clone()), "brush placement #{i} minted a duplicate object id {:?}", placed.id);
            // Successive placements target the same fixed `host:v0`, so only the first actually attaches;
            // reset attractions so every iteration re-exercises `apply_brush_placement_to_fixture` fresh.
            fixture.attractions.clear();
        }
    }

    fn single_object_scene_json() -> String {
        let scene = SceneConfig {
            fixture: Fixture {
                attractions: vec![],
                target_volumes: vec![],
                objects: vec![FixtureObject {
                    id: "host".to_string(),
                    object_kind: Some("Host".to_string()),
                    mesh_url: Some("/test/host.glb".to_string()),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
                    reveal_index: None,
                }],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![ObjectKind { id: "Host".to_string(), mesh_url: Some("/test/host.glb".to_string()), scale: None, vortices: vec![] }],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None }],
                cables: vec![],
            }),
            kind_compatibility: vec![],
            overlap_budget: DEFAULT_OVERLAP_BUDGET,
            seed: 1,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        serde_json::to_string(&scene).unwrap()
    }

    /// 🪪 Regression: `set_scene` used to unconditionally `rebuild_queue()`, wiping `brush_cache`/`fill`
    /// progress on every resync — `sync_precompute_session` (puzzle/plugin/rs/lib.rs) calls `set_scene`
    /// on *every* action, so this made suggestion/fill precompute restart from zero on every single tick,
    /// freezing the UI. A resync with byte-identical scene JSON must be a no-operation.
    #[test]
    fn compose_fill_display_is_read_only_and_matches_apply_prefix() {
        let object =
            |id: &str| FixtureObject { id: id.to_string(), object_kind: Some("Placed".to_string()), mesh_url: Some("/test/placed.glb".to_string()), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, vortices: vec![], reveal_index: None };
        let attraction = |index: usize| AttractionProps { id: format!("a{index}"), attracting: format!("p{index}:v0"), attracted: format!("p{}:v0", index + 1), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 };
        let payload = |index: usize| BrushPlacePayload { target_vortex_full_id: format!("p{index}:v0"), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [index as f64, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let base = Fixture { objects: vec![object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base.clone(), 7, &HashMap::new(), &catalogs);
        fill.applied_count = 2;
        fill.sequence = (0..5).map(payload).collect();
        fill.appended_objects = (0..5).map(|index| object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..5).map(attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        let mut engine = Puzzle3dEngine::new();
        engine.fill = Some(fill);

        let display = engine.compose_fill_display(4).expect("compose display");
        assert_eq!(display.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0", "p1", "p2", "p3"]);
        assert_eq!(engine.fill.as_ref().expect("fill").applied_count, 2, "compose must not mutate applied_count");

        let applied = engine.apply_fill_count(4).expect("apply fill count");
        assert_eq!(applied.objects.len(), display.objects.len());
        assert_eq!(engine.fill.as_ref().expect("fill").applied_count, 4);
    }

    #[test]
    fn fill_options_paths_are_millisecond_scale() {
        let object =
            |id: &str| FixtureObject { id: id.to_string(), object_kind: Some("Placed".to_string()), mesh_url: Some("/test/placed.glb".to_string()), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, vortices: vec![], reveal_index: None };
        let attraction = |index: usize| AttractionProps { id: format!("a{index}"), attracting: format!("p{index}:v0"), attracted: format!("p{}:v0", index + 1), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 };
        let payload = |index: usize| BrushPlacePayload { target_vortex_full_id: format!("p{index}:v0"), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [index as f64, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let base = Fixture { objects: vec![object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base.clone(), 7, &HashMap::new(), &catalogs);
        fill.applied_count = 0;
        fill.sequence = (0..10).map(payload).collect();
        fill.appended_objects = (0..10).map(|index| object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..10).map(attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());

        let mut engine = Puzzle3dEngine::new();
        let base_scene = SceneConfig {
            fixture: base.clone(),
            kind_catalogs: Some(catalogs),
            kind_compatibility: vec![],
            overlap_budget: 0.0,
            seed: 7,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        engine.set_scene(&serde_json::to_string(&base_scene).unwrap()).expect("seed");
        engine.fill = Some(fill);

        let count_start = std::time::Instant::now();
        let _ = engine.apply_fill_count(5).expect("apply fill count");
        let count_ms = count_start.elapsed().as_secs_f64() * 1000.0;
        println!("[DEBUG] apply_fill_count(5): {count_ms:.3}ms");
        assert!(count_ms < 5.0, "fill count apply took {count_ms}ms");
        assert_eq!(engine.fill.as_ref().expect("fill").applied_count, 5);

        let queue_before = engine.queue.len();
        let weight_start = std::time::Instant::now();
        let mut object_weights = HashMap::new();
        object_weights.insert("Placed".to_string(), 1.0);
        let mut vortex_weights = HashMap::new();
        vortex_weights.insert("c-b".to_string(), 0.5);
        vortex_weights.insert("b-s".to_string(), 0.5);
        engine.update_kind_weights(object_weights, vortex_weights);
        let weight_ms = weight_start.elapsed().as_secs_f64() * 1000.0;
        println!("[DEBUG] update_kind_weights: {weight_ms:.3}ms queue_before={queue_before} queue_after={}", engine.queue.len());
        assert!(weight_ms < 50.0, "weight update took {weight_ms}ms");
        let fill = engine.fill.as_ref().expect("fill");
        let fill_steps = engine.queue.iter().filter(|task| matches!(task, PrecomputeTask::FillStep)).count();
        assert_eq!(fill_steps, fill.max_count - fill.applied_count, "weight update must soft-replan the tail without a full queue wipe");
        assert_eq!(fill.applied_count, 5, "applied fill objects must survive weight edits");
    }

    #[test]
    fn apply_fill_count_downward_move_keeps_the_plan_intact() {
        // 🔽 Moving the count DOWN must never discard the already-planned sequence/appended objects/
        // placed entries or re-enqueue FillSteps — only `applied_count` (and the returned document-prefix
        // fixture) may change. Otherwise a jittery drag forces expensive replanning on every dip.
        let object = |id: &str| FixtureObject { id: id.to_string(), object_kind: Some("Placed".to_string()), mesh_url: Some("/test/placed.glb".to_string()), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, vortices: vec![], reveal_index: None };
        let attraction = |index: usize| AttractionProps { id: format!("a{index}"), attracting: format!("p{index}:v0"), attracted: format!("p{}:v0", index + 1), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 };
        let payload = |index: usize| BrushPlacePayload { target_vortex_full_id: format!("p{index}:v0"), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [index as f64, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let base = Fixture { objects: vec![object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base.clone(), 7, &HashMap::new(), &catalogs);
        fill.applied_count = 0;
        fill.sequence = (0..10).map(payload).collect();
        fill.appended_objects = (0..10).map(|index| object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..10).map(attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        fill.placed = fill
            .appended_objects
            .iter()
            .map(|object| PlacedCollisionEntry { object_id: object.id.clone(), mesh_url: "/test/placed.glb".into(), world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) })
            .collect();

        let mut engine = Puzzle3dEngine::new();
        let base_scene = SceneConfig { fixture: base.clone(), kind_catalogs: Some(catalogs), kind_compatibility: vec![], overlap_budget: 0.0, seed: 7, host_rules: BrushHostRules::default(), weights: BrushKindWeights::default() };
        engine.set_scene(&serde_json::to_string(&base_scene).unwrap()).expect("seed");
        engine.fill = Some(fill);

        engine.apply_fill_count(8).expect("apply up to 8");
        let queue_before = engine.queue.len();
        let placed_before = engine.fill.as_ref().unwrap().placed.len();
        let sequence_before = engine.fill.as_ref().unwrap().sequence.len();

        engine.apply_fill_count(3).expect("apply down to 3");
        let fill = engine.fill.as_ref().expect("fill");
        assert_eq!(fill.applied_count, 3);
        assert_eq!(fill.sequence.len(), sequence_before, "the plan is prefix-stable — downward moves never truncate it");
        assert_eq!(fill.appended_objects.len(), sequence_before);
        assert_eq!(fill.appended_attractions.len(), sequence_before);
        assert_eq!(fill.placed.len(), placed_before, "placed collision entries survive a downward move");
        assert_eq!(engine.queue.len(), queue_before, "no FillSteps get re-enqueued on a downward move");

        let fixture = engine.apply_fill_count(7).expect("apply back up to 7");
        assert_eq!(fixture.objects.len(), base.objects.len() + 7, "moving back up is instant — the plan was never discarded");
    }

    #[test]
    fn update_kind_weights_soft_replans_tail_without_rebuilding_queue() {
        let mut engine = Puzzle3dEngine::new();
        let json = single_object_scene_json();
        engine.set_scene(&json).expect("seed scene");
        let queue_len_after_seed = engine.queue.len();
        engine.precompute_step(8);
        let queue_len_after_step = engine.queue.len();
        assert!(queue_len_after_step < queue_len_after_seed);

        let mut object_weights = HashMap::new();
        object_weights.insert("Host".to_string(), 0.25);
        object_weights.insert("Placed".to_string(), 0.75);
        let mut vortex_weights = HashMap::new();
        vortex_weights.insert("c-b".to_string(), 0.5);
        vortex_weights.insert("b-s".to_string(), 0.5);
        engine.update_kind_weights(object_weights, vortex_weights);

        assert_eq!(engine.fill.as_ref().map(|fill| fill.applied_count).unwrap_or(0), 0, "weight-only edits must not change applied count");
        assert_eq!(engine.fill.as_ref().map(|fill| fill.sequence.len()).unwrap_or(0), 0, "planned tail must be discarded for replanning");
        assert!(engine.queue.len() >= queue_len_after_step, "fill steps must be re-enqueued without a full queue wipe");
        assert!(engine.queue.iter().any(|task| matches!(task, PrecomputeTask::FillStep)), "fill planning must continue after weight edits");
    }

    #[test]
    fn set_scene_with_identical_json_preserves_precompute_progress() {
        let mut engine = Puzzle3dEngine::new();
        let json = single_object_scene_json();
        engine.set_scene(&json).expect("first set_scene should succeed");
        let queue_len_before = engine.queue.len();
        assert!(queue_len_before > 0, "rebuild_queue should have enqueued at least the fill steps");
        engine.precompute_step(4);
        let queue_len_after_step = engine.queue.len();
        assert!(queue_len_after_step < queue_len_before, "precompute_step should have drained some queue items");

        engine.set_scene(&json).expect("resync with identical json should succeed");
        assert_eq!(engine.queue.len(), queue_len_after_step, "identical scene JSON must not rebuild (wipe) the queue");

        // A genuinely different scene (different object count) must still rebuild.
        let mut scene: serde_json::Value = serde_json::from_str(&json).unwrap();
        scene["fixture"]["objects"].as_array_mut().unwrap().push(serde_json::json!({ "id": "extra", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [5.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0], "vortices": [] }));
        let changed_json = serde_json::to_string(&scene).unwrap();
        engine.set_scene(&changed_json).expect("set_scene with a genuinely different scene should succeed");
        assert_ne!(engine.queue.len(), queue_len_after_step, "a changed scene must rebuild the queue");
    }

    #[test]
    fn decreasing_fill_count_keeps_the_plan_intact_and_does_not_replan() {
        // 🔽 Downward moves are prefix-stable (see `apply_fill_count`) — the plan/sequence/appended
        // objects/queue must never be discarded or re-enqueued just because the applied prefix shrank;
        // that used to force expensive replanning on every jittery drag dip.
        let object =
            |id: &str| FixtureObject { id: id.to_string(), object_kind: Some("Placed".to_string()), mesh_url: Some("/test/placed.glb".to_string()), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, vortices: vec![], reveal_index: None };
        let attraction = |index: usize| AttractionProps { id: format!("a{index}"), attracting: format!("p{index}:v0"), attracted: format!("p{}:v0", index + 1), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 };
        let payload = |index: usize| BrushPlacePayload { target_vortex_full_id: format!("p{index}:v0"), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [index as f64, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let base = Fixture { objects: vec![object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base.clone(), 7, &HashMap::new(), &catalogs);
        fill.applied_count = 3;
        fill.sequence = (0..3).map(payload).collect();
        fill.appended_objects = (0..3).map(|index| object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..3).map(attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        fill.stalled = true;
        let rng_state = fill.rng_state;
        let mut engine = Puzzle3dEngine::new();
        engine.fill = Some(fill);

        let fixture = engine.apply_fill_count(1).expect("fill session");
        assert_eq!(fixture.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0"], "the returned document prefix reflects the new applied count");
        let fill = engine.fill.as_ref().expect("fill builder");
        assert_eq!(fill.appended_objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["p0", "p1", "p2"], "the full plan survives — a downward move never discards the tail");
        assert_eq!(fill.sequence.len(), 3, "the planned sequence is never truncated by a downward move");
        assert_eq!(fill.applied_count, 1);
        assert!(fill.stalled, "apply_fill_count never touches stalled — only actual planning (fill_step_one) does");
        assert_eq!(fill.rng_state, rng_state, "no replanning happens, so the random stream is untouched");
        assert!(engine.queue.is_empty(), "no FillSteps get enqueued by a downward move");

        let fixture = engine.apply_fill_count(0).expect("zero fill count");
        assert_eq!(fixture.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"], "zero applies nothing to the document");
        assert_eq!(engine.fill.as_ref().expect("fill builder").sequence.len(), 3, "even at count 0, the plan is preserved for instant re-apply");
    }

    #[test]
    fn set_scene_with_applied_fill_projection_preserves_slider_session() {
        let object =
            |id: &str| FixtureObject { id: id.to_string(), object_kind: Some("Placed".to_string()), mesh_url: Some("/test/placed.glb".to_string()), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, vortices: vec![], reveal_index: None };
        let attraction = |index: usize| AttractionProps { id: format!("a{index}"), attracting: format!("p{index}:v0"), attracted: format!("p{}:v0", index + 1), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 };
        let payload = |index: usize| BrushPlacePayload { target_vortex_full_id: format!("p{index}:v0"), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [index as f64, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let base = Fixture { objects: vec![object("base")], attractions: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let mut fill = FillBuilder::new(base.clone(), 7, &HashMap::new(), &catalogs);
        fill.applied_count = 3;
        fill.sequence = (0..3).map(payload).collect();
        fill.appended_objects = (0..3).map(|index| object(&format!("p{index}"))).collect();
        fill.appended_attractions = (0..3).map(attraction).collect();
        fill.fixture.objects.extend(fill.appended_objects.iter().cloned());
        fill.fixture.attractions.extend(fill.appended_attractions.iter().cloned());
        fill.stalled = true;

        let mut engine = Puzzle3dEngine::new();
        let base_scene = SceneConfig {
            fixture: base.clone(),
            kind_catalogs: Some(catalogs),
            kind_compatibility: vec![],
            overlap_budget: 0.0,
            seed: 7,
            host_rules: BrushHostRules::default(),
            weights: BrushKindWeights::default(),
        };
        let base_json = serde_json::to_string(&base_scene).unwrap();
        engine.set_scene(&base_json).expect("seed base scene");
        // 🪣 Replace the fresh FillBuilder from rebuild_queue with the already-applied session under test.
        engine.fill = Some(fill);

        let mut applied_scene = base_scene.clone();
        applied_scene.fixture.objects.extend((0..3).map(|index| object(&format!("p{index}"))));
        applied_scene.fixture.attractions.extend((0..3).map(attraction));
        // 🪪 Pose drift on the base object (attraction rederive) must not count as a new scene.
        applied_scene.fixture.objects[0].origin = [1.0, 2.0, 3.0];
        let applied_json = serde_json::to_string(&applied_scene).unwrap();
        engine.set_scene(&applied_json).expect("re-syncing the applied fill projection must succeed");

        let fill = engine.fill.as_ref().expect("fill session must survive the applied-projection re-sync");
        assert_eq!(fill.applied_count, 3, "applied fill count must survive incidental set_scene syncs");
        assert_eq!(fill.sequence.len(), 3, "planned fill sequence must survive incidental set_scene syncs");
        assert_eq!(fill.base.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"]);

        let reduced = engine.apply_fill_count(1).expect("decreasing after sync");
        assert_eq!(reduced.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base", "p0"], "slider must still be able to remove fill objects after a document re-sync");
        let cleared = engine.apply_fill_count(0).expect("clear after sync");
        assert_eq!(cleared.objects.iter().map(|object| object.id.as_str()).collect::<Vec<_>>(), vec!["base"]);
    }

    /// 🪪 Regression: registering a mesh must invalidate any cached brush candidates computed against a
    /// different (e.g. fallback-box) body for the same url, but a no-operation re-registration must not matter
    /// once the cache already reflects the current mesh set (the everyday case: every action re-seeds the
    /// fallback body, and `sync_precompute_session` already guards that with `has_mesh`).
    #[test]
    fn register_mesh_invalidates_cached_precompute_state() {
        let mut engine = Puzzle3dEngine::new();
        engine.set_scene(&single_object_scene_json()).expect("set_scene should succeed");
        let queue_len_before = engine.queue.len();
        let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
        engine.register_mesh("/test/host.glb".to_string(), positions, indices);
        assert_eq!(engine.queue.len(), queue_len_before, "registering a mesh rebuilds the same scene, so the queue shape is unchanged, but the cache/fill must have been recomputed fresh");
        assert!(engine.brush_cache.is_empty(), "rebuild_queue clears brush_cache; a stale entry from before the real mesh arrived must not survive");
    }

    fn unit_cube_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
        (
            vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
            vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2],
        )
    }

    /// 🧊 Same box as `unit_cube_mesh_buffers` but with outward-facing (CCW-from-outside) winding, needed
    /// for tests that rely on `CollisionShape::contains_point` actually reporting interior points as inside.
    fn outward_wound_unit_cube_mesh_buffers() -> (Vec<f32>, Vec<u32>) {
        (
            vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
            vec![0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 5, 4, 0, 1, 5, 2, 7, 6, 2, 3, 7, 0, 7, 3, 0, 4, 7, 1, 6, 5, 1, 2, 6],
        )
    }

    #[test]
    fn vec3d_and_point3d_basic_ops() {
        let a = Vec3d::new(1.0, 2.0, 3.0);
        let b = Vec3d::new(4.0, -1.0, 0.5);
        let sum = a + b;
        assert_eq!((sum.x(), sum.y(), sum.z()), (5.0, 1.0, 3.5));
        let scaled = a * 2.0;
        assert_eq!((scaled.x(), scaled.y(), scaled.z()), (2.0, 4.0, 6.0));
        assert_eq!(Vec3d::new(-5.0, 3.0, -1.0).amax(), 5.0);

        let p1 = Point3d::new(1.0, 5.0, -2.0);
        let p2 = Point3d::new(4.0, 2.0, 3.0);
        let inf = p1.inf(&p2);
        let sup = p1.sup(&p2);
        assert_eq!((inf.x(), inf.y(), inf.z()), (1.0, 2.0, -2.0));
        assert_eq!((sup.x(), sup.y(), sup.z()), (4.0, 5.0, 3.0));
        let diff = p2 - p1;
        assert_eq!((diff.x(), diff.y(), diff.z()), (3.0, -3.0, 5.0));
        let back = Point3d::from_coords(diff);
        assert_eq!((back.x(), back.y(), back.z()), (3.0, -3.0, 5.0));
    }

    #[test]
    fn rotation3d_identity_ijkw_roundtrip_and_apply() {
        let identity = Rotation3d::identity();
        assert_eq!(identity.to_ijkw(), (0.0, 0.0, 0.0, 1.0));
        let q = Rotation3d::from_ijkw(0.0, 0.0, 0.0, 1.0);
        let v = Vec3d::new(1.0, 0.0, 0.0);
        let rotated = q.apply(v);
        assert_eq!((rotated.x(), rotated.y(), rotated.z()), (1.0, 0.0, 0.0));
    }

    #[test]
    fn rotation3d_rotation_between_none_for_antiparallel() {
        let from = Vec3d::new(1.0, 0.0, 0.0);
        let to = Vec3d::new(-1.0, 0.0, 0.0);
        assert!(Rotation3d::rotation_between(from, to).is_none(), "opposite vectors have no unique rotation axis");
        let to2 = Vec3d::new(0.0, 1.0, 0.0);
        assert!(Rotation3d::rotation_between(from, to2).is_some());
    }

    #[test]
    fn pose3d_compose_inverse_transform_point() {
        let rotation = Rotation3d::from_ijkw(0.0, 0.0, 0.0, 1.0);
        let translation = Vec3d::new(1.0, 2.0, 3.0);
        let pose = Pose3d::from_parts(translation, rotation);
        let point = Point3d::new(0.0, 0.0, 0.0);
        let transformed = pose.transform_point(&point);
        assert_eq!((transformed.x(), transformed.y(), transformed.z()), (1.0, 2.0, 3.0));
        let back = pose.inverse().transform_point(&transformed);
        assert!(back.x().abs() < 1e-6 && back.y().abs() < 1e-6 && back.z().abs() < 1e-6);
        let composed = pose.compose(&Pose3d::identity());
        let composed_point = composed.transform_point(&point);
        assert_eq!((composed_point.x(), composed_point.y(), composed_point.z()), (1.0, 2.0, 3.0));
    }

    #[test]
    fn normalize_vec3_handles_zero_length() {
        assert_eq!(normalize_vec3([0.0, 0.0, 0.0]), [0.0, 0.0, -1.0]);
        let n = normalize_vec3([3.0, 0.0, 4.0]);
        assert!((n[0] - 0.6).abs() < 1e-9 && (n[2] - 0.8).abs() < 1e-9);
    }

    #[test]
    fn vec3_math_helpers() {
        assert_eq!(vec3_dot([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]), 32.0);
        assert_eq!(vec3_cross([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]), [0.0, 0.0, 1.0]);
        assert_eq!(vec3_add([1.0, 2.0, 3.0], [1.0, 1.0, 1.0]), [2.0, 3.0, 4.0]);
        assert_eq!(vec3_sub([1.0, 2.0, 3.0], [1.0, 1.0, 1.0]), [0.0, 1.0, 2.0]);
        assert_eq!(negate_vec3([1.0, -2.0, 3.0]), [-1.0, 2.0, -3.0]);
    }

    #[test]
    fn vec3_scale_variants() {
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &None), [1.0, 2.0, 3.0]);
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &Some(serde_json::json!(2.0))), [2.0, 4.0, 6.0]);
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &Some(serde_json::json!([2.0, 3.0, 4.0]))), [2.0, 6.0, 12.0]);
        assert_eq!(vec3_scale([1.0, 2.0, 3.0], &Some(serde_json::json!("bogus"))), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn quat_rotate_vec_and_180_degree_axis() {
        let identity: Quat = [0.0, 0.0, 0.0, 1.0];
        let rotated = quat_rotate_vec(identity, [1.0, 2.0, 3.0]);
        assert!((rotated[0] - 1.0).abs() < 1e-9 && (rotated[1] - 2.0).abs() < 1e-9 && (rotated[2] - 3.0).abs() < 1e-9);
        let q = quaternion_from_180_degree_axis([0.0, 0.0, 2.0]);
        assert_eq!(q, [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn anti_parallel_brush_orientation_branches() {
        let in_plane = anti_parallel_brush_orientation([1.0, 0.0, 0.0]);
        assert_eq!(in_plane, quaternion_from_180_degree_axis([0.0, 0.0, 1.0]), "near-planar target direction falls back to the z axis");
        let along_z = anti_parallel_brush_orientation([0.0, 0.0, 1.0]);
        assert_eq!(along_z, quaternion_from_180_degree_axis([1.0, 0.0, 0.0]), "target parallel to z has an undefined cross axis, falls back to x");
        let general = anti_parallel_brush_orientation([0.0, 1.0, 0.5]);
        assert_eq!(general, quaternion_from_180_degree_axis([-1.0, 0.0, 0.0]), "general case uses cross(z, target)");
    }

    #[test]
    fn compute_brush_placement_pose_host_orientation_branch() {
        let host_orientation: Quat = [0.0, 0.0, 0.0, 1.0];
        let (origin, orientation) = compute_brush_placement_pose([1.0, 0.0, 0.0], [0.0, 0.0, -1.0], &None, [10.0, 0.0, 0.0], [0.0, 0.0, 1.0], Some(host_orientation), true);
        assert_eq!(orientation, host_orientation, "an antiparallel host-source direction keeps the host's own orientation");
        assert_eq!(origin, [9.0, 0.0, 0.0]);
    }

    #[test]
    fn compute_brush_placement_pose_falls_back_without_reference_orientation() {
        let (_, orientation) = compute_brush_placement_pose([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], &None, [0.0, 0.0, 0.0], [0.0, 0.0, -1.0], None, true);
        let anti = anti_parallel_brush_orientation([0.0, 0.0, -1.0]);
        assert_eq!(orientation, anti, "use_host_orientation with no reference orientation must fall through to the general path");
    }

    #[test]
    fn compute_brush_placement_pose_general_rotation_between() {
        let (origin, orientation) = compute_brush_placement_pose([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], &None, [5.0, 0.0, 0.0], [1.0, 0.0, 0.0], None, false);
        let rotated = quat_rotate_vec(orientation, [0.0, 0.0, -1.0]);
        assert!((rotated[0] + 1.0).abs() < 1e-4, "local dir must rotate onto the desired world dir: {rotated:?}");
        assert!((origin[0] - 5.0).abs() < 1e-6);
    }

    #[test]
    fn collision_body_from_buffers_rejects_too_few_or_degenerate() {
        assert!(collision_body_from_buffers(&[0.0; 6], &[0, 1, 2]).is_none(), "fewer than 3 vertices must be rejected");
        assert!(collision_body_from_buffers(&[0.0; 9], &[0, 1]).is_none(), "fewer than one triangle's worth of indices must be rejected");
        let tiny_positions: Vec<f32> = vec![0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.1, 0.0];
        assert!(collision_body_from_buffers(&tiny_positions, &[0, 1, 2]).is_none(), "extent below the minimum collision mesh extent must be rejected");
    }

    #[test]
    fn collision_body_from_buffers_accepts_valid_mesh() {
        let (positions, indices) = unit_cube_mesh_buffers();
        let scaled: Vec<f32> = positions.iter().map(|c| c * 4.0).collect();
        let body = collision_body_from_buffers(&scaled, &indices).expect("valid mesh should build a body");
        assert_eq!(body.parts.len(), 1);
        assert_eq!((body.local_bounds_min.x(), body.local_bounds_max.x()), (-4.0, 4.0));
    }

    #[test]
    fn world_bounds_transforms_local_aabb_corners() {
        let (positions, indices) = unit_cube_mesh_buffers();
        let body = collision_body_from_buffers(&positions, &indices).expect("body");
        let pose = Pose3d::from_parts(Vec3d::new(10.0, 0.0, 0.0), Rotation3d::identity());
        let (min, max) = world_bounds(&body, &pose);
        assert_eq!((min.x(), max.x()), (9.0, 11.0));
    }

    #[test]
    fn world_volumes_contain_aabb_empty_and_multi_volume() {
        assert!(world_volumes_contain_aabb(&[], Point3d::new(-1.0, -1.0, -1.0), Point3d::new(1.0, 1.0, 1.0)), "no target volumes means unconstrained");
        let volumes = vec![
            WorldVolumeProps { id: "far".into(), origin: [100.0, 0.0, 0.0], orientation: None, scale: None },
            WorldVolumeProps { id: "near".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: Some(serde_json::json!(4.0)) },
        ];
        assert!(world_volumes_contain_aabb(&volumes, Point3d::new(-1.0, -1.0, -1.0), Point3d::new(1.0, 1.0, 1.0)), "any single containing volume is enough");
    }

    #[test]
    fn bodies_intersect_and_solid_overlap_volume_reject_disjoint_aabbs() {
        let (positions, indices) = unit_cube_mesh_buffers();
        let body = collision_body_from_buffers(&positions, &indices).expect("body");
        let pose_a = Pose3d::identity();
        let pose_b = Pose3d::from_parts(Vec3d::new(100.0, 0.0, 0.0), Rotation3d::identity());
        assert!(!bodies_intersect(&body, &pose_a, &body, &pose_b));
        assert_eq!(solid_overlap_volume(&body, &pose_a, &body, &pose_b, 64, 0.02), 0.0);
    }

    #[test]
    fn solid_overlap_volume_reports_positive_overlap_for_coincident_bodies() {
        let (positions, indices) = outward_wound_unit_cube_mesh_buffers();
        let scaled: Vec<f32> = positions.iter().map(|c| c * 4.0).collect();
        let body = collision_body_from_buffers(&scaled, &indices).expect("body");
        let pose = Pose3d::identity();
        assert!(point_inside_body(&body, &pose, Point3d::new(0.0, 0.0, 0.0)), "the box's own center must be inside itself");
        let overlap = solid_overlap_volume(&body, &pose, &body, &pose, 256, 0.0);
        assert!(overlap > 0.0, "two fully coincident solid bodies must report a positive overlap: {overlap}");
    }

    #[test]
    fn vortex_port_shape_and_compatibility() {
        assert_eq!(puzzle3d_vortex_port_shape("foo circular bar"), Some("circular"));
        assert_eq!(puzzle3d_vortex_port_shape("foo rectangular bar"), Some("rectangular"));
        assert_eq!(puzzle3d_vortex_port_shape("plain"), None);
        assert!(puzzle3d_vortex_port_shapes_compatible("plain", "foo circular bar"));
        assert!(puzzle3d_vortex_port_shapes_compatible("foo circular bar", "baz circular qux"));
        assert!(!puzzle3d_vortex_port_shapes_compatible("foo circular bar", "baz rectangular qux"));
    }

    #[test]
    fn single_letter_port_family_and_compatibility() {
        assert_eq!(puzzle3d_single_letter_port_family("a-socket"), Some('a'));
        assert_eq!(puzzle3d_single_letter_port_family("ab-socket"), None);
        assert_eq!(puzzle3d_single_letter_port_family("A-socket"), None);
        assert_eq!(puzzle3d_single_letter_port_family("plain"), None);
        assert!(puzzle3d_single_letter_port_families_compatible("plain", "a-socket"));
        assert!(puzzle3d_single_letter_port_families_compatible("a-socket", "a-plug"));
        assert!(!puzzle3d_single_letter_port_families_compatible("a-socket", "b-plug"));
    }

    #[test]
    fn resolve_cable_and_attraction_kind_defaults_and_lookup() {
        let catalogs = KindCatalogBundle {
            objects: vec![],
            vortices: vec![VortexKindCatalog { id: "vk".into(), default_cable_kind: Some("  cable.custom  ".into()) }, VortexKindCatalog { id: "vk-empty".into(), default_cable_kind: Some("   ".into()) }],
            cables: vec![CableKindCatalog { id: "cable.custom".into(), default_attraction_kind: Some("attraction.custom".into()) }],
        };
        assert_eq!(resolve_cable_kind_for_vortex("vk", &catalogs), "cable.custom");
        assert_eq!(resolve_cable_kind_for_vortex("vk-empty", &catalogs), DEFAULT_CABLE_KIND_ID);
        assert_eq!(resolve_cable_kind_for_vortex("missing", &catalogs), DEFAULT_CABLE_KIND_ID);
        assert_eq!(resolve_attraction_kind_for_cable("cable.custom", &catalogs), "attraction.custom");
        assert_eq!(resolve_attraction_kind_for_cable("missing", &catalogs), "");
    }

    #[test]
    fn compat_pair_matches_and_specificity_rank() {
        let rule = KindCompatEntry { source: "a".into(), target: "b".into(), bidirectional: false, important: false, specificity: None };
        assert!(compat_pair_matches(&rule, "a", "b"));
        assert!(!compat_pair_matches(&rule, "b", "a"));
        let bidi = KindCompatEntry { bidirectional: true, ..rule };
        assert!(compat_pair_matches(&bidi, "b", "a"));
        assert_eq!(specificity_rank(Some("general")), 0);
        assert_eq!(specificity_rank(Some("object")), 1);
        assert_eq!(specificity_rank(Some("attraction")), 2);
        assert_eq!(specificity_rank(Some("cable")), 3);
        assert_eq!(specificity_rank(Some("vortex")), 4);
        assert_eq!(specificity_rank(Some("unknown")), 4);
        assert_eq!(specificity_rank(None), 4);
    }

    #[test]
    fn attraction_gesture_rule_applies_specificity_branches() {
        let catalogs = KindCatalogBundle {
            objects: vec![],
            vortices: vec![VortexKindCatalog { id: "sv".into(), default_cable_kind: Some("cable.a".into()) }, VortexKindCatalog { id: "tv".into(), default_cable_kind: Some("cable.b".into()) }],
            cables: vec![CableKindCatalog { id: "cable.a".into(), default_attraction_kind: Some("attr.a".into()) }, CableKindCatalog { id: "cable.b".into(), default_attraction_kind: Some("attr.b".into()) }],
        };
        let attracting = AttractionVortexContext { object_kind: Some("ObjA".into()), vortex_kind: Some("sv".into()) };
        let attracted = AttractionVortexContext { object_kind: Some("ObjB".into()), vortex_kind: Some("tv".into()) };
        let rule_for = |source: &str, target: &str, specificity: Option<&str>| KindCompatEntry { source: source.into(), target: target.into(), bidirectional: false, important: false, specificity: specificity.map(String::from) };
        assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", Some("general")), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("ObjA", "ObjB", Some("object")), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("attr.a", "attr.b", Some("attraction")), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("cable.a", "cable.b", Some("cable")), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", None), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", Some("weird")), &attracting, &attracted, &catalogs));
        assert!(!attraction_gesture_rule_applies(&rule_for("sv", "other", Some("general")), &attracting, &attracted, &catalogs));
    }

    #[test]
    fn vortices_attraction_compatible_for_drag_branches() {
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let a_circ = AttractionVortexContext { object_kind: None, vortex_kind: Some("x circular y".into()) };
        let a_rect = AttractionVortexContext { object_kind: None, vortex_kind: Some("x rectangular y".into()) };
        assert!(!vortices_attraction_compatible_for_drag(&a_circ, &a_rect, &[], &catalogs), "incompatible port shapes must reject regardless of rules");

        let a_letter = AttractionVortexContext { object_kind: None, vortex_kind: Some("a-socket".into()) };
        let b_letter = AttractionVortexContext { object_kind: None, vortex_kind: Some("b-plug".into()) };
        assert!(!vortices_attraction_compatible_for_drag(&a_letter, &b_letter, &[], &catalogs), "mismatched single-letter families must reject");

        let sv = AttractionVortexContext { object_kind: None, vortex_kind: Some("sv".into()) };
        let tv = AttractionVortexContext { object_kind: None, vortex_kind: Some("tv".into()) };
        assert!(vortices_attraction_compatible_for_drag(&sv, &tv, &[], &catalogs), "no rules means compatible");

        let unrelated = KindCompatEntry { source: "sv".into(), target: "other".into(), bidirectional: false, important: false, specificity: Some("general".into()) };
        assert!(!vortices_attraction_compatible_for_drag(&sv, &tv, &[unrelated], &catalogs), "no matching rule must reject");

        let low = KindCompatEntry { source: "sv".into(), target: "tv".into(), bidirectional: false, important: false, specificity: Some("general".into()) };
        let important = KindCompatEntry { important: true, ..low.clone() };
        assert!(vortices_attraction_compatible_for_drag(&sv, &tv, &[low, important], &catalogs), "an important match among matched rules must keep it compatible");
    }

    #[test]
    fn brush_stack_pair_helpers() {
        assert_eq!(brush_stack_vortex_base("column bottom"), Some("column"));
        assert_eq!(brush_stack_vortex_base("column top"), Some("column"));
        assert_eq!(brush_stack_vortex_base("column"), None);
        assert!(brush_stack_bottom_top_pair("column bottom", "column top"));
        assert!(!brush_stack_bottom_top_pair("column top", "column bottom"));
        assert!(brush_stack_top_bottom_pair("column top", "column bottom"));
        assert!(!brush_stack_top_bottom_pair("column bottom", "column top"));
        assert!(brush_stack_mate_pair("column bottom", "column top"));
        assert!(brush_stack_mate_pair("column top", "column bottom"));
        assert!(!brush_stack_mate_pair("column bottom", "beam top"));
        assert!(!brush_stack_mate_pair("x circular column bottom", "x rectangular column top"), "incompatible port shapes must reject even a stack mate pair");
    }

    #[test]
    fn brush_candidate_rank_scores_kind_match_and_stack_and_tambour_rules() {
        let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
        let template = ObjectKindVortexTemplate { vortex_kind: Some("column bottom".into()), position: [0.0, 0.0, 0.0], direction: None };
        let same_kind = BrushCompatibleCandidate { object_kind_id: "Host".into(), source_vortex_index: 0 };
        let score = brush_candidate_rank(&same_kind, &template, &target);
        assert_eq!(score, 15_000, "matching object kind (+10000) plus a stack mate pair (+5000)");

        let target_tambour = AttractionVortexContext { object_kind: Some("Tambour".into()), vortex_kind: Some("door tambour circular".into()) };
        let capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("door tambour circular".into()), position: [0.0, 0.0, 0.0], direction: None };
        let capital = BrushCompatibleCandidate { object_kind_id: "Capital".into(), source_vortex_index: 0 };
        assert!(brush_candidate_rank(&capital, &capsule_template, &target_tambour) < 0, "capital on tambour must be penalized");

        let cylindric = BrushCompatibleCandidate { object_kind_id: "Cylindric Tambour".into(), source_vortex_index: 0 };
        assert!(brush_candidate_rank(&cylindric, &capsule_template, &target_tambour) > 0, "cylindric tambour stacking onto a mid-tambour host should score positively");
    }

    #[test]
    fn host_accepts_candidate_rule_branches() {
        let rules = BrushHostRules::default();
        let target = AttractionVortexContext { object_kind: Some("Tambour".into()), vortex_kind: Some("door tambour circular".into()) };
        let door_capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("door capsule".into()), position: [1.0, 0.0, 0.0], direction: None };

        let capital = BrushCompatibleCandidate { object_kind_id: "Capital".into(), source_vortex_index: 0 };
        assert!(!host_accepts_candidate(&rules, &target, &capital, &door_capsule_template), "reject_capital_on_tambour must reject Capital");

        let storey = BrushCompatibleCandidate { object_kind_id: "Last Storey".into(), source_vortex_index: 0 };
        assert!(!host_accepts_candidate(&rules, &target, &storey, &door_capsule_template), "reject_last_single_storey_on_mid_tambour must reject Last Storey on a Tambour host");

        let door_ok = BrushCompatibleCandidate { object_kind_id: "Door".into(), source_vortex_index: 0 };
        assert!(host_accepts_candidate(&rules, &target, &door_ok, &door_capsule_template), "a door capsule far enough on x and close enough on y must be accepted");

        let non_capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("not a capsule".into()), position: [1.0, 0.0, 0.0], direction: None };
        assert!(!host_accepts_candidate(&rules, &target, &door_ok, &non_capsule_template), "a door tambour target requires a door-capsule source vortex");

        let close_template = ObjectKindVortexTemplate { vortex_kind: Some("door capsule".into()), position: [0.1, 0.0, 0.0], direction: None };
        assert!(!host_accepts_candidate(&rules, &target, &door_ok, &close_template), "the door capsule position must satisfy the minimum absolute x");

        let door_rule_off = BrushHostRules { door_tambour_requires_door_capsule: false, ..BrushHostRules::default() };
        assert!(host_accepts_candidate(&door_rule_off, &target, &door_ok, &non_capsule_template), "disabling door_tambour_requires_door_capsule accepts regardless of the source vortex kind");
    }

    #[test]
    fn brush_placement_uses_host_orientation_branches() {
        let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
        assert!(!brush_placement_uses_host_orientation(&target, "column bottom", "Host"), "stack mate pairs never use host orientation");
        assert!(!brush_placement_uses_host_orientation(&target, "other", "Host"), "different vortex kinds never use host orientation");
        assert!(brush_placement_uses_host_orientation(&target, "column top", "Host"), "matching vortex kind and object kind uses host orientation");
        assert!(!brush_placement_uses_host_orientation(&target, "column top", "OtherKind"), "matching vortex kind but a different candidate kind rejects host orientation");
    }

    #[test]
    fn resolve_object_kind_mesh_url_prefers_catalog_then_falls_back_to_fixture() {
        let catalogs = KindCatalogBundle { objects: vec![ObjectKind { id: "Kind".into(), mesh_url: Some("/catalog.glb".into()), scale: None, vortices: vec![] }], vortices: vec![], cables: vec![] };
        let fixture = Fixture { attractions: vec![], target_volumes: vec![], objects: vec![] };
        assert_eq!(resolve_object_kind_mesh_url("Kind", &catalogs, &fixture), Some("/catalog.glb".to_string()));

        let empty_catalogs = KindCatalogBundle { objects: vec![ObjectKind { id: "Kind".into(), mesh_url: Some("".into()), scale: None, vortices: vec![] }], vortices: vec![], cables: vec![] };
        let fixture_with_object = Fixture {
            attractions: vec![],
            target_volumes: vec![],
            objects: vec![FixtureObject { id: "o1".into(), object_kind: Some("Kind".into()), mesh_url: Some("/fixture.glb".into()), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, vortices: vec![], reveal_index: None }],
        };
        assert_eq!(resolve_object_kind_mesh_url("Kind", &empty_catalogs, &fixture_with_object), Some("/fixture.glb".to_string()));
        assert_eq!(resolve_object_kind_mesh_url("Missing", &empty_catalogs, &fixture_with_object), None);
    }

    #[test]
    fn brush_compatible_candidates_filters_and_sorts() {
        let catalogs = KindCatalogBundle {
            objects: vec![
                ObjectKind { id: "NoMesh".into(), mesh_url: None, scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), position: [0.0, 0.0, 0.0], direction: None }] },
                ObjectKind { id: "NoVortices".into(), mesh_url: Some("/a.glb".into()), scale: None, vortices: vec![] },
                ObjectKind { id: "Match".into(), mesh_url: Some("/b.glb".into()), scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), position: [0.0, 0.0, 0.0], direction: None }] },
            ],
            vortices: vec![],
            cables: vec![],
        };
        let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("sv".into()) };
        let candidates = brush_compatible_candidates(&target, &catalogs, &[], &BrushHostRules::default());
        assert_eq!(candidates.len(), 1, "kinds with no mesh url or no vortices must be excluded: {candidates:?}");
        assert_eq!(candidates[0].object_kind_id, "Match");
    }

    #[test]
    fn brush_compatible_candidates_stack_target_only_matches_mates() {
        let catalogs = KindCatalogBundle {
            objects: vec![
                ObjectKind { id: "Mate".into(), mesh_url: Some("/a.glb".into()), scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("column bottom".into()), position: [0.0, 0.0, 0.0], direction: None }] },
                ObjectKind { id: "NotMate".into(), mesh_url: Some("/b.glb".into()), scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("beam".into()), position: [0.0, 0.0, 0.0], direction: None }] },
            ],
            vortices: vec![],
            cables: vec![],
        };
        let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
        let candidates = brush_compatible_candidates(&target, &catalogs, &[], &BrushHostRules::default());
        assert_eq!(candidates.len(), 1, "a stack-top target must only match stack mates: {candidates:?}");
        assert_eq!(candidates[0].object_kind_id, "Mate");
    }

    #[test]
    fn blocked_vortex_full_ids_and_enumeration_excludes_them() {
        let attractions = vec![AttractionProps { id: "a1".into(), attracting: "host:v0".into(), attracted: "guest:v0".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 }];
        let blocked = blocked_vortex_full_ids(&attractions);
        assert!(blocked.contains("host:v0") && blocked.contains("guest:v0"));

        let fixture = Fixture {
            attractions,
            target_volumes: vec![],
            objects: vec![
                FixtureObject { id: "host".into(), object_kind: Some("Host".into()), mesh_url: None, origin: [0.0, 0.0, 0.0], orientation: None, scale: None, vortices: vec![VortexProps { id: "v0".into(), vortex_kind: None, position: [0.0, 0.0, 0.0], direction: None }], reveal_index: None },
                FixtureObject { id: "free".into(), object_kind: Some("Free".into()), mesh_url: None, origin: [0.0, 0.0, 0.0], orientation: None, scale: None, vortices: vec![VortexProps { id: "v0".into(), vortex_kind: None, position: [0.0, 0.0, 0.0], direction: None }], reveal_index: None },
            ],
        };
        let targets = enumerate_brush_fill_vortex_targets(&fixture);
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].full_id, "free:v0");
    }

    #[test]
    fn vortex_world_from_object_none_for_missing_index() {
        let object = FixtureObject { id: "o".into(), object_kind: None, mesh_url: None, origin: [1.0, 2.0, 3.0], orientation: None, scale: None, vortices: vec![], reveal_index: None };
        assert!(vortex_world_from_object(&object, 0).is_none());
    }

    #[test]
    fn weight_lookup_helpers_default_to_one_or_gate_on_zero() {
        let mut weights = BrushKindWeights::default();
        weights.object_weights.insert("A".into(), 2.0);
        weights.vortex_weights.insert("v".into(), 0.0);
        assert_eq!(brush_kind_weight_value(&weights.object_weights, "A"), 2.0);
        assert_eq!(brush_kind_weight_value(&weights.object_weights, "missing"), 1.0);
        assert!(!brush_target_vortex_allows_suggestion(Some("v"), &weights));
        assert!(brush_target_vortex_allows_suggestion(Some("other"), &weights));
        assert!(brush_target_vortex_allows_suggestion(None, &weights));

        let target = BrushFillVortexTarget { full_id: "f".into(), object_id: "o".into(), object_kind: None, vortex_kind: Some("v".into()), vortex_index: 0 };
        assert_eq!(fill_vortex_target_weight(&target, &weights), 0.0);
    }

    #[test]
    fn weighted_sample_without_replacement_edge_cases() {
        let items = vec![1, 2, 3];
        let mut rng = 42u32;
        let single: Vec<i32> = weighted_sample_without_replacement(&[1], |_| 1.0, &mut rng);
        assert_eq!(single, vec![1]);
        let all_zero: Vec<i32> = weighted_sample_without_replacement(&items, |_| 0.0, &mut rng);
        assert!(all_zero.is_empty(), "all-zero weights leave nothing eligible");
        let sampled = weighted_sample_without_replacement(&items, |_| 1.0, &mut rng);
        let mut sorted = sampled.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, items, "every eligible item appears exactly once");
    }

    #[test]
    fn fill_rng_is_deterministic_for_a_given_seed() {
        let mut a = 123u32;
        let mut b = 123u32;
        for _ in 0..5 {
            assert_eq!(fill_rng(&mut a), fill_rng(&mut b));
        }
        assert_ne!(a, 123);
    }

    #[test]
    fn fill_candidate_diversity_score_rewards_distance_within_same_kind() {
        let candidate = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 3 };
        assert_eq!(fill_candidate_diversity_score(&candidate, 0, Some("Other")), 0, "a different target object kind never scores");
        assert_eq!(fill_candidate_diversity_score(&candidate, 0, Some("Kind")), 1000 + 300);
        assert_eq!(fill_candidate_diversity_score(&candidate, 3, Some("Kind")), 1000);
    }

    #[test]
    fn brush_preview_from_candidate_none_branches() {
        let catalogs = KindCatalogBundle { objects: vec![ObjectKind { id: "Kind".into(), mesh_url: Some("/mesh.glb".into()), scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), position: [0.0, 0.0, 0.0], direction: None }] }], vortices: vec![], cables: vec![] };
        let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
        let target_ctx = AttractionVortexContext { object_kind: None, vortex_kind: None };
        let world = TargetVortexWorld { position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, -1.0], reference_orientation: None };

        let missing_kind = BrushCompatibleCandidate { object_kind_id: "Missing".into(), source_vortex_index: 0 };
        assert!(brush_preview_from_candidate("t", &missing_kind, &target_ctx, world, &catalogs, &fixture).is_none());

        let bad_index = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 5 };
        assert!(brush_preview_from_candidate("t", &bad_index, &target_ctx, world, &catalogs, &fixture).is_none());

        let empty_mesh_catalogs = KindCatalogBundle { objects: vec![ObjectKind { id: "Kind".into(), mesh_url: None, scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), position: [0.0, 0.0, 0.0], direction: None }] }], vortices: vec![], cables: vec![] };
        let ok_candidate = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 0 };
        assert!(brush_preview_from_candidate("t", &ok_candidate, &target_ctx, world, &empty_mesh_catalogs, &fixture).is_none(), "a missing mesh url must yield no preview");

        let preview = brush_preview_from_candidate("t", &ok_candidate, &target_ctx, world, &catalogs, &fixture).expect("a valid candidate should produce a preview");
        assert_eq!(preview.mesh_url, "/mesh.glb");
        assert_eq!(preview.object_kind_id, "Kind");
    }

    #[test]
    fn apply_brush_placement_to_fixture_rejects_missing_kind_template_or_mesh() {
        let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle { objects: vec![ObjectKind { id: "Kind".into(), mesh_url: None, scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), position: [0.0, 0.0, 0.0], direction: None }] }], vortices: vec![], cables: vec![] };

        let missing_kind = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Missing".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_kind, &catalogs).objects.len(), 0);

        let missing_template = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 9, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_template, &catalogs).objects.len(), 0);

        let missing_mesh = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_mesh, &catalogs).objects.len(), 0, "no resolvable mesh url means the placement must be rejected");
    }

    #[test]
    fn apply_brush_placement_to_fixture_rejects_duplicate_attraction_target() {
        let catalogs = KindCatalogBundle { objects: vec![ObjectKind { id: "Kind".into(), mesh_url: Some("/mesh.glb".into()), scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), position: [0.0, 0.0, 0.0], direction: None }] }], vortices: vec![], cables: vec![] };
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let fixture = Fixture { attractions: vec![AttractionProps { id: "a".into(), attracting: "host:v0".into(), attracted: "other:v0".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 }], objects: vec![], target_volumes: vec![] };
        let next = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
        assert_eq!(next.objects.len(), 0, "a target vortex that is already attracting must reject the placement");
    }

    #[test]
    fn engine_precompute_step_and_fill_step_false_with_no_scene() {
        let mut engine = Puzzle3dEngine::new();
        assert!(!engine.precompute_step(10));
        assert!(!engine.fill_step_one());
    }

    #[test]
    fn engine_apply_brush_placement_none_without_scene_or_catalogs() {
        let mut engine = Puzzle3dEngine::new();
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert!(engine.apply_brush_placement(&payload).is_none(), "no scene means no placement");

        engine.set_scene(&single_object_scene_json()).expect("seed");
        if let Some(scene) = &mut engine.scene {
            scene.kind_catalogs = None;
        }
        assert!(engine.apply_brush_placement(&payload).is_none(), "no catalogs means no placement");
    }

    #[test]
    fn engine_has_mesh_invalidate_and_refresh_brush_candidates() {
        let mut engine = Puzzle3dEngine::new();
        engine.set_scene(&single_object_scene_json()).expect("seed");
        assert!(!engine.has_mesh("/test/host.glb"));
        let (positions, indices) = unit_cube_mesh_buffers();
        engine.register_mesh("/test/host.glb".to_string(), positions, indices);
        assert!(engine.has_mesh("/test/host.glb"));

        engine.invalidate_brush_target("host:v0");
        match engine.queue.first() {
            Some(PrecomputeTask::BrushTarget(id)) => assert_eq!(id.as_str(), "host:v0"),
            other => panic!("expected the invalidated target requeued at the front, got {other:?}"),
        }
        assert!(!engine.brush_cache.contains_key("host:v0"));

        engine.refresh_brush_candidates("host:v0");
        assert!(engine.brush_cache.contains_key("host:v0"));
        assert_eq!(engine.brush_preview_json("host:v0", 0), None, "the catalog's Host kind has no vortices, so there are no free candidates");
    }

    #[test]
    fn precompute_session_native_wrapper_exercises_public_methods() {
        let mut session = Puzzle3dPrecomputeSession::default();
        session.set_scene(&single_object_scene_json()).expect("set_scene");
        assert!(!session.has_mesh("/test/host.glb"));
        let (positions, indices) = unit_cube_mesh_buffers();
        session.register_mesh("/test/host.glb", &positions, &indices);
        assert!(session.has_mesh("/test/host.glb"));
        assert!(!session.fill_is_done(), "a freshly (re)seeded fill session has not stalled or hit max_count yet");

        session.precompute_step(50);
        session.invalidate_brush_target("host:v0");
        session.refresh_brush_candidates("host:v0");
        let candidates_json = session.brush_candidates("host:v0");
        assert!(candidates_json.contains("free"));
        assert!(session.brush_preview_json("host:v0", 0).is_none());

        assert!(session.fill_progress().contains("maxCount"));
        assert_eq!(session.fill_available_count(), 0);

        let mut object_weights = HashMap::new();
        object_weights.insert("Host".to_string(), 1.0);
        session.update_kind_weights_rust(object_weights, HashMap::new());

        assert!(session.apply_brush_placement_rust("not json").is_err());

        let fixture_json = session.apply_fill_count_rust(0).expect("fill session available");
        assert!(fixture_json.contains("\"host\""));
        let display_json = session.compose_fill_display_rust(0).expect("fill session available");
        assert!(display_json.contains("\"host\""));
    }

    #[test]
    fn precompute_session_native_wrapper_errors_without_scene() {
        let mut session = Puzzle3dPrecomputeSession::new();
        assert!(session.apply_fill_count_rust(0).is_err());
        assert!(session.compose_fill_display_rust(0).is_err());
        assert!(session.apply_brush_placement_rust(r#"{"targetVortexFullId":"a","objectKindId":"b","sourceVortexIndex":0,"origin":[0,0,0],"orientation":[0,0,0,1]}"#).is_err());
        assert!(session.fill_is_done());
        assert_eq!(session.fill_available_count(), 0);
    }
}

#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
impl Default for Puzzle3dPrecomputeSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
impl Puzzle3dPrecomputeSession {
    pub fn new() -> Self {
        Self { engine: Puzzle3dEngine::new() }
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), Puzzle3dError> {
        self.engine.set_scene(json)
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh(url.to_string(), positions.to_vec(), indices.to_vec());
    }

    pub fn has_mesh(&self, url: &str) -> bool {
        self.engine.has_mesh(url)
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.engine.precompute_step(budget)
    }

    pub fn invalidate_brush_target(&mut self, vortex_full_id: &str) {
        self.engine.invalidate_brush_target(vortex_full_id);
    }

    pub fn refresh_brush_candidates(&mut self, vortex_full_id: &str) {
        self.engine.refresh_brush_candidates(vortex_full_id);
    }

    pub fn brush_candidates(&self, vortex_full_id: &str) -> String {
        if let Some(hit) = self.engine.brush_cache.get(vortex_full_id) {
            return serde_json::to_string(hit).unwrap_or_else(|_| "{}".to_string());
        }
        let result = self.engine.compute_brush_cache_entry(vortex_full_id);
        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn brush_preview_json(&self, vortex_full_id: &str, candidate_index: usize) -> Option<String> {
        self.engine.brush_preview_json(vortex_full_id, candidate_index)
    }

    pub fn fill_progress(&self) -> String {
        let progress = self.engine.fill.as_ref().map(|f| f.progress()).unwrap_or(FillBuildProgress { count: 0, applied_count: 0, max_count: FILL_COUNT_MAX, done: true, appended_objects: vec![], appended_attractions: vec![], sequence: vec![] });
        serde_json::to_string(&progress).unwrap_or_else(|_| "{}".to_string())
    }

    /// 🪣 O(1) planned-count readout for the render/tick hot path — avoids a `fill_progress` JSON
    /// round trip just to read `sequence.len()`.
    pub fn fill_available_count(&self) -> u32 {
        self.engine.fill.as_ref().map(|fill| fill.sequence.len() as u32).unwrap_or(0)
    }

    pub fn fill_is_done(&self) -> bool {
        self.engine.fill.as_ref().map(|fill| fill.stalled || fill.sequence.len() >= fill.max_count).unwrap_or(true)
    }

    pub fn apply_brush_placement_rust(&mut self, payload_json: &str) -> Result<String, Puzzle3dError> {
        let payload: BrushPlacePayload = serde_json::from_str(payload_json)?;
        let fixture = self.engine.apply_brush_placement(&payload).ok_or(Puzzle3dError::BrushPlacementRejected)?;
        Ok(serde_json::to_string(&fixture)?)
    }

    pub fn apply_fill_count_rust(&mut self, count: u32) -> Result<String, Puzzle3dError> {
        let fixture = self.engine.apply_fill_count(count as usize).ok_or(Puzzle3dError::FillSessionUnavailable)?;
        Ok(serde_json::to_string(&fixture)?)
    }

    pub fn compose_fill_display_rust(&self, count: u32) -> Result<String, Puzzle3dError> {
        let fixture = self.engine.compose_fill_display(count as usize).ok_or(Puzzle3dError::FillSessionUnavailable)?;
        Ok(serde_json::to_string(&fixture)?)
    }

    pub fn update_kind_weights_rust(&mut self, object_weights: HashMap<String, f64>, vortex_weights: HashMap<String, f64>) {
        self.engine.update_kind_weights(object_weights, vortex_weights);
    }
}


//#region 🔖DocumentHelpers
pub fn empty_puzzle3d_projection() -> Puzzle3dProjection {
    Puzzle3dProjection::default()
}
//#endregion 🔖DocumentHelpers
