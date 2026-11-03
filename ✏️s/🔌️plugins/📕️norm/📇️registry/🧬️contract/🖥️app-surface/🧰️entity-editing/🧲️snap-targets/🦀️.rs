//! 🧲️ Scene snap targets: member ends and midpoints of the scene become snap targets of a translating handle, so a drag lands on existing geometry without a typed value. See [`super`].

use semio_s_plugin_norm_structural_scene::{Geometry, Handle, HandleKind, Path3, StructuralScene, Vec3};

fn along(axis: &Vec3, anchor: &Vec3, point: &Vec3) -> f64 {
    let length = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
    if length <= f64::EPSILON {
        return f64::NAN;
    }
    ((point.x - anchor.x) * axis.x + (point.y - anchor.y) * axis.y + (point.z - anchor.z) * axis.z) / length
}

fn mid(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 { x: (a.x + b.x) / 2.0, y: (a.y + b.y) / 2.0, z: (a.z + b.z) / 2.0 }
}

/// 📍️ The ends and the segment midpoints of every member axis of the scene except the handle's own entity.
pub fn member_points(scene: &StructuralScene, except: &str) -> Vec<Vec3> {
    let mut points = Vec::new();
    for entity in scene.entities.iter().filter(|entity| entity.entity.id != except) {
        if let Geometry::Member(member) = &entity.geometry {
            if let Path3::Polyline(line) = &member.axis {
                points.extend(line.points.iter().cloned());
                points.extend(line.points.windows(2).map(|pair| mid(&pair[0], &pair[1])));
            }
        }
    }
    points
}

/// 🧲️ The drag parameters at which a translating handle meets any of `points`: `s` is the distance in metres along the handle axis from its anchor, inside the limits, sorted and without duplicates. Other handle kinds have none.
pub fn targets_from_points(points: &[Vec3], handle: &Handle) -> Vec<f64> {
    let HandleKind::Translate { axis } = &handle.kind else { return Vec::new() };
    let mut targets: Vec<f64> = points.iter().map(|point| along(axis, &handle.anchor, point)).filter(|s| s.is_finite() && handle.limits.as_ref().is_none_or(|limits| limits.contains(*s))).collect();
    targets.sort_by(f64::total_cmp);
    targets.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    targets
}

/// 🧲️ [`targets_from_points`] over the member ends and midpoints of the scene except the handle's own entity.
pub fn scene_snap_targets(scene: &StructuralScene, handle: &Handle) -> Vec<f64> {
    targets_from_points(&member_points(scene, &handle.entity.id), handle)
}

/// 🧲️ The handle with the scene targets merged into its snap rule (existing targets kept), ready for the drag tool.
pub fn with_scene_targets(mut handle: Handle, scene: &StructuralScene) -> Handle {
    let mut merged = std::mem::take(&mut handle.snap.targets);
    merged.extend(scene_snap_targets(scene, &handle));
    merged.sort_by(f64::total_cmp);
    merged.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    handle.snap.targets = merged;
    handle
}
