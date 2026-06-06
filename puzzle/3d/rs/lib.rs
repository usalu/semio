//! 🧩 Puzzle 3d brush/fill precompute: parry3d solid collision, candidate enumeration, greedy fill.
#![allow(clippy::missing_errors_doc, reason = "Internal puzzle 3d WASM bundle.")]

use nalgebra::{Isometry3, Point3, Quaternion, UnitQuaternion, Vector3};
use parry3d::query::intersection_test;
use parry3d::shape::{SharedShape, TriMesh, TriMeshFlags};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const DEFAULT_OVERLAP_BUDGET: f64 = 0.02;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KindCatalogBundle {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AttractionProps {
    attracting: String,
    attracted: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FixtureV1 {
    #[serde(default)]
    attractions: Vec<AttractionProps>,
    #[serde(default)]
    objects: Vec<FixtureObject>,
    #[serde(default, rename = "targetVolumes")]
    target_volumes: Vec<WorldVolumeProps>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SceneConfig {
    fixture: FixtureV1,
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
struct BrushPreviewState {
    target_vortex_full_id: String,
    object_kind_id: String,
    source_vortex_index: usize,
    mesh_url: String,
    origin: Vec3,
    orientation: Quat,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrushCollisionFreeResult {
    free: Vec<BrushCompatibleCandidate>,
    unknown_pending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrushPlacePayload {
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
struct FillBuildProgress {
    count: usize,
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
    object_id: String,
    object_kind: Option<String>,
    vortex_kind: Option<String>,
}

struct CollisionMeshPart {
    shape: SharedShape,
    local_pose: Isometry3<f32>,
}

struct CollisionBody {
    parts: Vec<CollisionMeshPart>,
    local_bounds_min: Point3<f32>,
    local_bounds_max: Point3<f32>,
}

#[derive(Clone)]
struct PlacedCollisionEntry {
    object_id: String,
    mesh_url: String,
    world: Isometry3<f32>,
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
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
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

fn unit_quat_from_cad(q: Quat) -> UnitQuaternion<f32> {
    UnitQuaternion::from_quaternion(Quaternion::new(q[3] as f32, q[0] as f32, q[1] as f32, q[2] as f32))
}

fn quat_rotate_vec(q: Quat, v: Vec3) -> Vec3 {
    let uq = unit_quat_from_cad(q);
    let rotated = uq * Vector3::new(v[0] as f32, v[1] as f32, v[2] as f32);
    [rotated.x as f64, rotated.y as f64, rotated.z as f64]
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

fn pose_isometry(origin: Vec3, orientation: Quat, _scale: &Option<serde_json::Value>) -> Isometry3<f32> {
    let q = unit_quat_from_cad(orientation);
    let t = Vector3::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
    Isometry3::from_parts(t.into(), q)
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
        let from = Vector3::new(local_dir[0] as f32, local_dir[1] as f32, local_dir[2] as f32);
        let to = Vector3::new(desired_world_dir[0] as f32, desired_world_dir[1] as f32, desired_world_dir[2] as f32);
        let q = UnitQuaternion::rotation_between(&from, &to).unwrap_or(UnitQuaternion::identity());
        let quat = q.quaternion();
        [quat.i as f64, quat.j as f64, quat.k as f64, quat.w as f64]
    };
    let origin = vec3_sub(target_world_position, quat_rotate_vec(orientation, scaled_local));
    (origin, orientation)
}

fn collision_body_from_buffers(positions: &[f32], indices: &[u32]) -> Option<CollisionBody> {
    if positions.len() < 9 || indices.len() < 3 {
        return None;
    }
    let mut verts: Vec<Point3<f32>> = Vec::with_capacity(positions.len() / 3);
    let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for chunk in positions.chunks_exact(3) {
        let rp = Point3::new(chunk[0], chunk[1], chunk[2]);
        verts.push(rp);
        min = min.inf(&rp);
        max = max.sup(&rp);
    }
    let extent = (max - min).amax();
    if !extent.is_finite() || extent < BRUSH_COLLISION_MESH_MIN_EXTENT as f32 {
        return None;
    }
    let mut tris: Vec<[u32; 3]> = Vec::with_capacity(indices.len() / 3);
    for chunk in indices.chunks_exact(3) {
        tris.push([chunk[0], chunk[1], chunk[2]]);
    }
    let tri_mesh = TriMesh::with_flags(verts, tris, TriMeshFlags::ORIENTED | TriMeshFlags::MERGE_DUPLICATE_VERTICES);
    let shape = SharedShape::new(tri_mesh);
    Some(CollisionBody {
        parts: vec![CollisionMeshPart {
            shape,
            local_pose: Isometry3::identity(),
        }],
        local_bounds_min: min,
        local_bounds_max: max,
    })
}

fn world_bounds(body: &CollisionBody, world: &Isometry3<f32>) -> (Point3<f32>, Point3<f32>) {
    let corners = [
        Point3::new(body.local_bounds_min.x, body.local_bounds_min.y, body.local_bounds_min.z),
        Point3::new(body.local_bounds_max.x, body.local_bounds_min.y, body.local_bounds_min.z),
        Point3::new(body.local_bounds_min.x, body.local_bounds_max.y, body.local_bounds_min.z),
        Point3::new(body.local_bounds_max.x, body.local_bounds_max.y, body.local_bounds_min.z),
        Point3::new(body.local_bounds_min.x, body.local_bounds_min.y, body.local_bounds_max.z),
        Point3::new(body.local_bounds_max.x, body.local_bounds_min.y, body.local_bounds_max.z),
        Point3::new(body.local_bounds_min.x, body.local_bounds_max.y, body.local_bounds_max.z),
        Point3::new(body.local_bounds_max.x, body.local_bounds_max.y, body.local_bounds_max.z),
    ];
    let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for corner in corners {
        let w = world * corner;
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

fn world_volumes_contain_aabb(volumes: &[WorldVolumeProps], min: Point3<f32>, max: Point3<f32>) -> bool {
    if volumes.is_empty() {
        return true;
    }
    let corners = [
        min,
        Point3::new(max.x, min.y, min.z),
        Point3::new(min.x, max.y, min.z),
        Point3::new(max.x, max.y, min.z),
        Point3::new(min.x, min.y, max.z),
        Point3::new(max.x, min.y, max.z),
        Point3::new(min.x, max.y, max.z),
        max,
    ];
    for volume in volumes {
        let scale = volume_scale_vec(&volume.scale);
        let hx = scale[0] * 0.5 + 1e-3;
        let hy = scale[1] * 0.5 + 1e-3;
        let hz = scale[2] * 0.5 + 1e-3;
        let world = pose_isometry(
            volume.origin,
            volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
            &None,
        );
        let inv = world.inverse();
        let mut inside = true;
        for corner in corners {
            let local = inv * corner;
            if local.x.abs() > hx || local.y.abs() > hy || local.z.abs() > hz {
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

fn point_inside_body(body: &CollisionBody, world: &Isometry3<f32>, point: Point3<f32>) -> bool {
    let local = world.inverse() * point;
    for part in &body.parts {
        let part_local = part.local_pose.inverse() * local;
        if part.shape.contains_point(&part.local_pose, &part_local) {
            return true;
        }
    }
    false
}

fn bodies_intersect(a: &CollisionBody, world_a: &Isometry3<f32>, b: &CollisionBody, world_b: &Isometry3<f32>) -> bool {
    let (amin, amax) = world_bounds(a, world_a);
    let (bmin, bmax) = world_bounds(b, world_b);
    if amax.x < bmin.x || bmax.x < amin.x || amax.y < bmin.y || bmax.y < amin.y || amax.z < bmin.z || bmax.z < amin.z {
        return false;
    }
    for part_a in &a.parts {
        let pose_a = world_a * part_a.local_pose;
        for part_b in &b.parts {
            let pose_b = world_b * part_b.local_pose;
            if intersection_test(&pose_a, &*part_a.shape, &pose_b, &*part_b.shape).unwrap_or(false) {
                return true;
            }
        }
    }
    let center = Point3::from((amin.coords + amax.coords + bmin.coords + bmax.coords) * 0.25);
    point_inside_body(a, world_a, center) && point_inside_body(b, world_b, center)
}

fn solid_overlap_volume(
    a: &CollisionBody,
    world_a: &Isometry3<f32>,
    b: &CollisionBody,
    world_b: &Isometry3<f32>,
    sample_count: usize,
    overlap_budget: f64,
) -> f64 {
    let (amin, amax) = world_bounds(a, world_a);
    let (bmin, bmax) = world_bounds(b, world_b);
    let imin = Point3::new(amin.x.max(bmin.x), amin.y.max(bmin.y), amin.z.max(bmin.z));
    let imax = Point3::new(amax.x.min(bmax.x), amax.y.min(bmax.y), amax.z.min(bmax.z));
    if imax.x < imin.x || imax.y < imin.y || imax.z < imin.z {
        return 0.0;
    }
    if !bodies_intersect(a, world_a, b, world_b) {
        return 0.0;
    }
    let size = imax - imin;
    let box_vol = size.x as f64 * size.y as f64 * size.z as f64;
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
        let p = Point3::new(
            imin.x + (imax.x - imin.x) * rx as f32,
            imin.y + (imax.y - imin.y) * ry as f32,
            imin.z + (imax.z - imin.z) * rz as f32,
        );
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
    match (
        puzzle3d_single_letter_port_family(source),
        puzzle3d_single_letter_port_family(target),
    ) {
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
    catalog_vortex_by_id(catalogs, vortex_kind)
        .and_then(|v| v.default_cable_kind.as_ref())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_CABLE_KIND_ID.to_string())
}

fn resolve_attraction_kind_for_cable(cable_kind: &str, catalogs: &KindCatalogBundle) -> String {
    catalog_cable_by_id(catalogs, cable_kind)
        .and_then(|c| c.default_attraction_kind.as_ref())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_default()
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

fn attraction_gesture_rule_applies(
    rule: &KindCompatEntry,
    attracting: &AttractionVortexContext,
    attracted: &AttractionVortexContext,
    catalogs: &KindCatalogBundle,
) -> bool {
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

fn vortices_attraction_compatible_for_drag(
    attracting: &AttractionVortexContext,
    attracted: &AttractionVortexContext,
    rules: &[KindCompatEntry],
    catalogs: &KindCatalogBundle,
) -> bool {
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
    let mut matched: Vec<&KindCompatEntry> = rules
        .iter()
        .filter(|r| attraction_gesture_rule_applies(r, attracting, attracted, catalogs))
        .collect();
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
        if mid_tambour_host
            && (candidate.object_kind_id.contains("Last Storey") || candidate.object_kind_id.contains("Single Storey"))
        {
            score -= 30_000;
        }
        if mid_tambour_host && candidate.object_kind_id == "Cylindric Tambour" {
            score += 5_000;
        }
    }
    score
}

fn host_accepts_candidate(
    rules: &BrushHostRules,
    target: &AttractionVortexContext,
    candidate: &BrushCompatibleCandidate,
    template: &ObjectKindVortexTemplate,
) -> bool {
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    if rules.reject_capital_on_tambour
        && (target_vk.contains("tambour circular") || target_vk.contains("tambour rectangular"))
        && candidate.object_kind_id.contains("Capital")
    {
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

fn resolve_object_kind_mesh_url(kind_id: &str, catalogs: &KindCatalogBundle, fixture: &FixtureV1) -> Option<String> {
    if let Some(kind) = catalog_object_kind_by_id(catalogs, kind_id) {
        if let Some(url) = kind.mesh_url.as_ref().filter(|u| !u.is_empty()) {
            return Some(url.clone());
        }
    }
    fixture
        .objects
        .iter()
        .find(|o| o.object_kind.as_deref() == Some(kind_id))
        .and_then(|o| o.mesh_url.clone())
}

fn brush_compatible_candidates(
    target: &AttractionVortexContext,
    catalogs: &KindCatalogBundle,
    rules: &[KindCompatEntry],
    host_rules: &BrushHostRules,
) -> Vec<BrushCompatibleCandidate> {
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
            let attracting = AttractionVortexContext {
                object_id: "__brush__".to_string(),
                object_kind: Some(kind.id.clone()),
                vortex_kind: Some(source_vk.to_string()),
            };
            if !vortices_attraction_compatible_for_drag(&attracting, target, rules, catalogs) {
                continue;
            }
            let candidate = BrushCompatibleCandidate {
                object_kind_id: kind.id.clone(),
                source_vortex_index,
            };
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
    scored.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| a.0.object_kind_id.cmp(&b.0.object_kind_id))
            .then_with(|| a.0.source_vortex_index.cmp(&b.0.source_vortex_index))
    });
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
    let direction = normalize_vec3(quat_rotate_vec(
        orientation,
        vortex.direction.unwrap_or([0.0, 0.0, -1.0]),
    ));
    Some((position, direction))
}

fn enumerate_brush_fill_vortex_targets(fixture: &FixtureV1) -> Vec<BrushFillVortexTarget> {
    let blocked = blocked_vortex_full_ids(&fixture.attractions);
    let mut out = Vec::new();
    for obj in &fixture.objects {
        for (i, vortex) in obj.vortices.iter().enumerate() {
            let full_id = puzzle3d_vortex_full_id(&obj.id, &vortex.id);
            if !blocked.contains(&full_id) {
                out.push(BrushFillVortexTarget {
                    full_id,
                    object_id: obj.id.clone(),
                    object_kind: obj.object_kind.clone(),
                    vortex_kind: vortex.vortex_kind.clone(),
                    vortex_index: i,
                });
            }
        }
    }
    out
}

fn brush_kind_weight_value(weights: &HashMap<String, f64>, id: &str) -> f64 {
    weights.get(id).copied().unwrap_or(1.0)
}

fn brush_candidate_suggestion_weight(
    candidate: &BrushCompatibleCandidate,
    weights: &BrushKindWeights,
    catalogs: &KindCatalogBundle,
) -> f64 {
    let vortex_kind = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id)
        .and_then(|kind| kind.vortices.get(candidate.source_vortex_index))
        .and_then(|template| template.vortex_kind.as_deref())
        .unwrap_or("");
    brush_kind_weight_value(&weights.object_weights, &candidate.object_kind_id)
        * brush_kind_weight_value(&weights.vortex_weights, vortex_kind)
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

fn weighted_order_fill_vortex_targets(
    targets: &[BrushFillVortexTarget],
    weights: &BrushKindWeights,
    rng_state: &mut u32,
) -> Vec<BrushFillVortexTarget> {
    weighted_sample_without_replacement(targets, |target| fill_vortex_target_weight(target, weights), rng_state)
}

fn weighted_order_brush_compatible_candidates(
    candidates: &[BrushCompatibleCandidate],
    weights: &BrushKindWeights,
    catalogs: &KindCatalogBundle,
    rng_state: &mut u32,
) -> Vec<BrushCompatibleCandidate> {
    weighted_sample_without_replacement(
        candidates,
        |candidate| brush_candidate_suggestion_weight(candidate, weights, catalogs),
        rng_state,
    )
}

fn fill_candidate_diversity_score(
    candidate: &BrushCompatibleCandidate,
    target_vortex_index: usize,
    target_object_kind: Option<&str>,
) -> i64 {
    if target_object_kind != Some(candidate.object_kind_id.as_str()) {
        return 0;
    }
    1000
        + (candidate.source_vortex_index as i64 - target_vortex_index as i64)
            .unsigned_abs() as i64
            * 100
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
    let allowed: Vec<BrushCompatibleCandidate> = candidates
        .iter()
        .filter(|candidate| brush_candidate_suggestion_weight(candidate, weights, catalogs) > 0.0)
        .cloned()
        .collect();
    let target = target_vortex_kind.unwrap_or("");
    let mut cross = Vec::new();
    let mut same = Vec::new();
    for candidate in allowed {
        let source_vk = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id)
            .and_then(|kind| kind.vortices.get(candidate.source_vortex_index))
            .and_then(|template| template.vortex_kind.as_deref())
            .unwrap_or("");
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
    same_sorted.sort_by(|left, right| {
        left.object_kind_id
            .cmp(&right.object_kind_id)
            .then_with(|| left.source_vortex_index.cmp(&right.source_vortex_index))
    });
    cross.extend(weighted_order_brush_compatible_candidates(
        &same_sorted,
        weights,
        catalogs,
        rng_state,
    ));
    cross
}

fn brush_preview_from_candidate(
    target_full_id: &str,
    candidate: &BrushCompatibleCandidate,
    target: &AttractionVortexContext,
    target_world_position: Vec3,
    target_world_direction: Vec3,
    reference_orientation: Option<Quat>,
    catalogs: &KindCatalogBundle,
    fixture: &FixtureV1,
) -> Option<BrushPreviewState> {
    let kind = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id)?;
    let template = kind.vortices.get(candidate.source_vortex_index)?;
    let mesh_url = resolve_object_kind_mesh_url(&candidate.object_kind_id, catalogs, fixture)?;
    let source_vk = template.vortex_kind.as_deref().unwrap_or("");
    let use_host = brush_placement_uses_host_orientation(target, source_vk, &candidate.object_kind_id);
    let (origin, orientation) = compute_brush_placement_pose(
        template.position,
        template.direction.unwrap_or([0.0, 0.0, -1.0]),
        &kind.scale,
        target_world_position,
        target_world_direction,
        reference_orientation,
        use_host,
    );
    Some(BrushPreviewState {
        target_vortex_full_id: target_full_id.to_string(),
        object_kind_id: kind.id.clone(),
        source_vortex_index: candidate.source_vortex_index,
        mesh_url,
        origin,
        orientation,
        scale: kind.scale.clone(),
    })
}

struct FillBuilder {
    fixture: FixtureV1,
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
    fn new(base: FixtureV1, seed: u32, meshes: &HashMap<String, CollisionBody>, catalogs: &KindCatalogBundle) -> Self {
        let seed_object_ids: std::collections::HashSet<String> = base.objects.iter().map(|o| o.id.clone()).collect();
        let mut placed = Vec::new();
        for obj in &base.objects {
            if let Some(mesh_url) = resolve_object_kind_mesh_url(obj.object_kind.as_deref().unwrap_or(""), catalogs, &base) {
                if meshes.contains_key(&mesh_url) {
                    placed.push(PlacedCollisionEntry {
                        object_id: obj.id.clone(),
                        mesh_url,
                        world: pose_isometry(obj.origin, obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &obj.scale),
                    });
                }
            }
        }
        Self {
            fixture: base,
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

    fn rng(&mut self) -> f64 {
        fill_rng(&mut self.rng_state)
    }

    fn progress(&self) -> FillBuildProgress {
        FillBuildProgress {
            count: self.sequence.len(),
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
    meshes: HashMap<String, CollisionBody>,
    brush_cache: HashMap<String, BrushCollisionFreeResult>,
    fill: Option<FillBuilder>,
    queue: Vec<PrecomputeTask>,
}

impl Puzzle3dEngine {
    fn new() -> Self {
        Self {
            scene: None,
            meshes: HashMap::new(),
            brush_cache: HashMap::new(),
            fill: None,
            queue: Vec::new(),
        }
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
            let catalogs = scene.kind_catalogs.clone().unwrap_or(KindCatalogBundle {
                objects: vec![],
                vortices: vec![],
                cables: vec![],
            });
            self.fill = Some(FillBuilder::new(scene.fixture.clone(), scene.seed, &self.meshes, &catalogs));
        } else {
            self.fill = None;
        }
    }

    fn set_scene(&mut self, json: &str) -> Result<(), String> {
        let scene: SceneConfig = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.scene = Some(scene);
        self.rebuild_queue();
        Ok(())
    }

    fn register_mesh(&mut self, url: String, positions: Vec<f32>, indices: Vec<u32>) {
        if let Some(body) = collision_body_from_buffers(&positions, &indices) {
            self.meshes.insert(url, body);
        }
    }

    fn preview_collides(
        meshes: &HashMap<String, CollisionBody>,
        preview: &BrushPreviewState,
        placed: &[PlacedCollisionEntry],
        overlap_budget: f64,
        sample_count: usize,
    ) -> Option<bool> {
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

    fn brush_collision_free(
        &self,
        target_full_id: &str,
        candidates: &[BrushCompatibleCandidate],
        overlap_budget: f64,
    ) -> BrushCollisionFreeResult {
        let Some(scene) = &self.scene else {
            return BrushCollisionFreeResult {
                free: vec![],
                unknown_pending: true,
            };
        };
        let empty_catalogs = KindCatalogBundle {
            objects: vec![],
            vortices: vec![],
            cables: vec![],
        };
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
            return BrushCollisionFreeResult {
                free: vec![],
                unknown_pending: false,
            };
        };
        let Some((position, direction)) = vortex_world_from_object(host, vortex_index) else {
            return BrushCollisionFreeResult {
                free: vec![],
                unknown_pending: false,
            };
        };
        let target_ctx = AttractionVortexContext {
            object_id: host.id.clone(),
            object_kind: host.object_kind.clone(),
            vortex_kind: host.vortices[vortex_index].vortex_kind.clone(),
        };
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
                Some(PlacedCollisionEntry {
                    object_id: obj.id.clone(),
                    mesh_url,
                    world: pose_isometry(obj.origin, obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &obj.scale),
                })
            })
            .collect();
        let mut free = Vec::new();
        let mut unknown_pending = false;
        for candidate in candidates {
            let Some(preview) = brush_preview_from_candidate(
                target_full_id,
                candidate,
                &target_ctx,
                position,
                direction,
                host.orientation,
                catalogs,
                &scene.fixture,
            ) else {
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
            return BrushCollisionFreeResult {
                free: vec![],
                unknown_pending: true,
            };
        };
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle {
            objects: vec![],
            vortices: vec![],
            cables: vec![],
        });
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
            return BrushCollisionFreeResult {
                free: vec![],
                unknown_pending: false,
            };
        };
        let target_ctx = AttractionVortexContext {
            object_id: host.id.clone(),
            object_kind: host.object_kind.clone(),
            vortex_kind: vortex.vortex_kind.clone(),
        };
        if !brush_target_vortex_allows_suggestion(vortex.vortex_kind.as_deref(), &scene.weights) {
            return BrushCollisionFreeResult {
                free: vec![],
                unknown_pending: false,
            };
        }
        let compatible = brush_compatible_candidates(&target_ctx, &catalogs, &scene.kind_compatibility, &scene.host_rules);
        let compatible: Vec<BrushCompatibleCandidate> = compatible
            .into_iter()
            .filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &catalogs) > 0.0)
            .collect();
        self.brush_collision_free(target_full_id, &compatible, scene.overlap_budget)
    }

    fn precompute_step(&mut self, budget: u32) -> bool {
        let mut remaining = budget as usize;
        while remaining > 0 {
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
        let catalogs = scene.kind_catalogs.as_ref().cloned().unwrap_or(KindCatalogBundle {
            objects: vec![],
            vortices: vec![],
            cables: vec![],
        });
        let overlap_budget = scene.overlap_budget;
        let weights = scene.weights.clone();
        let kind_compatibility = scene.kind_compatibility.clone();
        let host_rules = scene.host_rules.clone();
        let free_targets = enumerate_brush_fill_vortex_targets(&fill.fixture);
        if free_targets.is_empty() {
            fill.stalled = true;
            return false;
        }
        let seed_targets: Vec<_> = free_targets
            .iter()
            .filter(|t| fill.seed_object_ids.contains(&t.object_id))
            .cloned()
            .collect();
        let frontier_targets: Vec<_> = free_targets
            .iter()
            .filter(|t| !fill.seed_object_ids.contains(&t.object_id))
            .cloned()
            .collect();
        let ordered_targets: Vec<_> = weighted_order_fill_vortex_targets(&seed_targets, &weights, &mut fill.rng_state)
            .into_iter()
            .chain(weighted_order_fill_vortex_targets(
                &frontier_targets,
                &weights,
                &mut fill.rng_state,
            ))
            .collect();
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
            let target_ctx = AttractionVortexContext {
                object_id: target.object_id.clone(),
                object_kind: target.object_kind.clone(),
                vortex_kind: target.vortex_kind.clone(),
            };
            let key = format!(
                "{}\u{1}{}",
                target.object_kind.as_deref().unwrap_or(""),
                target.vortex_kind.as_deref().unwrap_or("")
            );
            let compatible = fill
                .candidate_cache
                .entry(key)
                .or_insert_with(|| {
                    brush_compatible_candidates(&target_ctx, &catalogs, &kind_compatibility, &host_rules)
                })
                .clone();
            if compatible.is_empty() {
                continue;
            }
            let ordered_candidates = order_brush_fill_compatible_candidates(
                &compatible,
                target.vortex_kind.as_deref(),
                target.vortex_index,
                target.object_kind.as_deref(),
                &catalogs,
                &weights,
                &mut fill.rng_state,
            );
            if ordered_candidates.is_empty() {
                continue;
            }
            for candidate in &ordered_candidates {
                let Some(preview) = brush_preview_from_candidate(
                    &target.full_id,
                    candidate,
                    &target_ctx,
                    position,
                    direction,
                    host.orientation,
                    &catalogs,
                    &fill.fixture,
                ) else {
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
                let placed_snapshot: Vec<PlacedCollisionEntry> = fill
                    .placed
                    .iter()
                    .filter(|entry| entry.object_id != target.object_id)
                    .cloned()
                    .collect();
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
                let placed_object = next_fixture.objects.last().cloned().unwrap();
                if let Some(mesh_url) = resolve_object_kind_mesh_url(placed_object.object_kind.as_deref().unwrap_or(""), &catalogs, &next_fixture) {
                    if self.meshes.contains_key(&mesh_url) {
                        fill.placed.push(PlacedCollisionEntry {
                            object_id: placed_object.id.clone(),
                            mesh_url,
                            world: pose_isometry(
                                placed_object.origin,
                                placed_object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                                &placed_object.scale,
                            ),
                        });
                    }
                }
                let new_attraction = next_fixture.attractions.last().cloned().unwrap();
                fill.fixture = next_fixture;
                fill.sequence.push(payload);
                fill.appended_objects.push(placed_object);
                fill.appended_attractions.push(new_attraction);
                return true;
            }
        }
        fill.stalled = true;
        false
    }
}

fn apply_brush_placement_to_fixture(fixture: &FixtureV1, payload: &BrushPlacePayload, catalogs: &KindCatalogBundle) -> FixtureV1 {
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
    let vortices: Vec<VortexProps> = kind
        .vortices
        .iter()
        .enumerate()
        .map(|(index, entry)| VortexProps {
            id: format!("{object_id}:v{index}"),
            vortex_kind: entry.vortex_kind.clone(),
            position: entry.position,
            direction: entry.direction,
        })
        .collect();
    let attracting = puzzle3d_vortex_full_id(&object_id, &vortices[payload.source_vortex_index].id);
    let _attraction_id = format!("attraction-{attracting}-{}", payload.target_vortex_full_id);
    let mut next = fixture.clone();
    if next.attractions.iter().any(|a| a.attracting == attracting || a.attracted == payload.target_vortex_full_id) {
        return fixture.clone();
    }
    next.attractions.push(AttractionProps {
        attracting,
        attracted: payload.target_vortex_full_id.clone(),
    });
    next.objects.push(FixtureObject {
        id: object_id,
        object_kind: Some(kind.id.clone()),
        mesh_url: Some(mesh_url),
        origin: payload.origin,
        orientation: Some(payload.orientation),
        scale: payload.scale.clone().or(kind.scale.clone()),
        vortices,
    });
    let _ = template;
    next
}

fn uuid_simple() -> String {
    let mut state = (js_sys_time_now() as u64) ^ 0xdead_beef_cafe_babe;
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

#[cfg(not(target_arch = "wasm32"))]
fn js_sys_time_now() -> f64 {
    0.0
}

#[cfg(target_arch = "wasm32")]
fn js_sys_time_now() -> f64 {
    js_sys::Date::now()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct Puzzle3dPrecomputeSession {
    engine: Puzzle3dEngine,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Puzzle3dPrecomputeSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: Puzzle3dEngine::new(),
        }
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), JsValue> {
        self.engine.set_scene(json).map_err(|e| JsValue::from_str(&e))
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh(url.to_string(), positions.to_vec(), indices.to_vec());
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.engine.precompute_step(budget)
    }

    pub fn brush_candidates(&self, vortex_full_id: &str) -> String {
        if let Some(hit) = self.engine.brush_cache.get(vortex_full_id) {
            return serde_json::to_string(hit).unwrap_or_else(|_| "{}".to_string());
        }
        let result = self.engine.compute_brush_cache_entry(vortex_full_id);
        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn fill_progress(&self) -> String {
        let progress = self
            .engine
            .fill
            .as_ref()
            .map(|f| f.progress())
            .unwrap_or(FillBuildProgress {
                count: 0,
                max_count: FILL_COUNT_MAX,
                done: true,
                appended_objects: vec![],
                appended_attractions: vec![],
                sequence: vec![],
            });
        serde_json::to_string(&progress).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Puzzle3dPrecomputeSession {
    engine: Puzzle3dEngine,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brush_candidates_allow_separated_boxes() {
        let mut engine = Puzzle3dEngine::new();
        let positions: Vec<f32> = vec![
            -4.0, -4.0, -4.0, 4.0, -4.0, -4.0, 4.0, 4.0, -4.0, -4.0, 4.0, -4.0, -4.0, -4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0,
        ];
        let indices: Vec<u32> = vec![
            0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2,
        ];
        engine.register_mesh("/test/obstacle.glb".to_string(), positions.clone(), indices.clone());
        engine.register_mesh("/test/preview.glb".to_string(), positions, indices);
        let scene = SceneConfig {
            fixture: FixtureV1 {
                attractions: vec![],
                objects: vec![
                    FixtureObject {
                        id: "obstacle".to_string(),
                        object_kind: Some("Kind".to_string()),
                        mesh_url: Some("/test/obstacle.glb".to_string()),
                        origin: [0.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        vortices: vec![VortexProps {
                            id: "v0".to_string(),
                            vortex_kind: Some("port-a".to_string()),
                            position: [0.0, 0.0, 0.0],
                            direction: Some([0.0, 0.0, -1.0]),
                        }],
                    },
                    FixtureObject {
                        id: "host".to_string(),
                        object_kind: Some("Host".to_string()),
                        mesh_url: Some("/test/unregistered.glb".to_string()),
                        origin: [12.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                        vortices: vec![VortexProps {
                            id: "v0".to_string(),
                            vortex_kind: Some("port-a".to_string()),
                            position: [0.0, 0.0, 0.0],
                            direction: Some([0.0, 0.0, -1.0]),
                        }],
                    },
                ],
            },
            kind_catalogs: Some(KindCatalogBundle {
                objects: vec![ObjectKind {
                    id: "Kind".to_string(),
                    mesh_url: Some("/test/preview.glb".to_string()),
                    scale: None,
                    vortices: vec![ObjectKindVortexTemplate {
                        vortex_kind: Some("port-b".to_string()),
                        position: [0.0, 0.0, 0.0],
                        direction: Some([0.0, 0.0, -1.0]),
                    }],
                }],
                vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None }],
                cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None }],
            }),
            kind_compatibility: vec![KindCompatEntry {
                source: "port-b".to_string(),
                target: "port-a".to_string(),
                bidirectional: true,
                important: false,
                specificity: Some("vortex".to_string()),
            }],
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
                    ObjectKindVortexTemplate {
                        vortex_kind: Some("c-b".to_string()),
                        position: [0.0, 0.0, 0.0],
                        direction: Some([0.0, 0.0, 1.0]),
                    },
                    ObjectKindVortexTemplate {
                        vortex_kind: Some("b-s".to_string()),
                        position: [0.0, 0.0, 0.0],
                        direction: Some([0.0, 0.0, 1.0]),
                    },
                ],
            }],
            vortices: vec![
                VortexKindCatalog { id: "c-b".to_string(), default_cable_kind: None },
                VortexKindCatalog { id: "c-t".to_string(), default_cable_kind: None },
                VortexKindCatalog { id: "b-s".to_string(), default_cable_kind: None },
            ],
            cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None }],
        };
        let candidates = vec![
            BrushCompatibleCandidate {
                object_kind_id: "Placed".to_string(),
                source_vortex_index: 0,
            },
            BrushCompatibleCandidate {
                object_kind_id: "Placed".to_string(),
                source_vortex_index: 1,
            },
        ];
        let mut weights = BrushKindWeights::default();
        weights.vortex_weights.insert("c-b".to_string(), 0.0);
        weights.vortex_weights.insert("c-t".to_string(), 0.0);
        weights.vortex_weights.insert("b-s".to_string(), 1.0);
        weights.object_weights.insert("Placed".to_string(), 1.0);
        let mut rng = 7u32;
        let ordered = order_brush_fill_compatible_candidates(
            &candidates,
            Some("b-s"),
            1,
            Some("Host"),
            &catalogs,
            &weights,
            &mut rng,
        );
        assert_eq!(ordered.len(), 1);
        assert_eq!(ordered[0].source_vortex_index, 1);
        let targets = vec![
            BrushFillVortexTarget {
                full_id: "host:v0".to_string(),
                object_id: "host".to_string(),
                object_kind: Some("Host".to_string()),
                vortex_kind: Some("c-t".to_string()),
                vortex_index: 0,
            },
            BrushFillVortexTarget {
                full_id: "host:v1".to_string(),
                object_id: "host".to_string(),
                object_kind: Some("Host".to_string()),
                vortex_kind: Some("b-s".to_string()),
                vortex_index: 1,
            },
        ];
        let target_ordered = weighted_order_fill_vortex_targets(&targets, &weights, &mut rng);
        assert_eq!(target_ordered.len(), 1);
        assert_eq!(target_ordered[0].vortex_kind.as_deref(), Some("b-s"));
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Puzzle3dPrecomputeSession {
    pub fn new() -> Self {
        Self {
            engine: Puzzle3dEngine::new(),
        }
    }

    pub fn set_scene(&mut self, json: &str) -> Result<(), String> {
        self.engine.set_scene(json)
    }

    pub fn register_mesh(&mut self, url: &str, positions: &[f32], indices: &[u32]) {
        self.engine.register_mesh(url.to_string(), positions.to_vec(), indices.to_vec());
    }

    pub fn precompute_step(&mut self, budget: u32) -> bool {
        self.engine.precompute_step(budget)
    }

    pub fn brush_candidates(&self, vortex_full_id: &str) -> String {
        if let Some(hit) = self.engine.brush_cache.get(vortex_full_id) {
            return serde_json::to_string(hit).unwrap_or_else(|_| "{}".to_string());
        }
        let result = self.engine.compute_brush_cache_entry(vortex_full_id);
        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn fill_progress(&self) -> String {
        let progress = self
            .engine
            .fill
            .as_ref()
            .map(|f| f.progress())
            .unwrap_or(FillBuildProgress {
                count: 0,
                max_count: FILL_COUNT_MAX,
                done: true,
                appended_objects: vec![],
                appended_attractions: vec![],
                sequence: vec![],
            });
        serde_json::to_string(&progress).unwrap_or_else(|_| "{}".to_string())
    }
}
