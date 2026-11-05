//! 📷️ Camera presets (iso, plan, elevations), fit to selection and fit to the worst check.

use super::config::CameraPreset;
use super::parts::entity_points;
use super::pose::*;
use super::scene::{Aabb, StructuralScene};

pub const FIELD_OF_VIEW: f64 = 45.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraPose {
    pub position: V3,
    pub target: V3,
    pub fov: f64,
}

impl CameraPose {
    pub fn json(&self) -> String {
        semio_plugin::world3d_camera_json(self.position, self.target, self.fov)
    }
}

pub fn preset_direction(preset: CameraPreset) -> V3 {
    match preset {
        CameraPreset::Iso => unit([1.0, -1.0, 0.8]),
        CameraPreset::Plan => unit([0.0, -0.001, 1.0]),
        CameraPreset::ElevationFront => [0.0, -1.0, 0.0],
        CameraPreset::ElevationBack => [0.0, 1.0, 0.0],
        CameraPreset::ElevationLeft => [-1.0, 0.0, 0.0],
        CameraPreset::ElevationRight => [1.0, 0.0, 0.0],
    }
}

pub fn bounds_of(points: &[V3]) -> Option<Aabb> {
    Aabb::of_points(points.iter().map(|p| super::scene::Vec3::new(p[0], p[1], p[2])).collect::<Vec<_>>().iter())
}

/// 🎯️ Camera looking at the centre of `bounds` along the preset direction, far enough that the bounding sphere fits the field of view.
pub fn fit(bounds: &Aabb, preset: CameraPreset) -> CameraPose {
    let (lo, hi) = (v3(&bounds.min), v3(&bounds.max));
    let centre = mul(add(lo, hi), 0.5);
    let radius = (len(sub(hi, lo)) / 2.0).max(0.5);
    let distance = radius / (FIELD_OF_VIEW.to_radians() / 2.0).sin() * 1.1;
    CameraPose { position: add(centre, mul(preset_direction(preset), distance)), target: centre, fov: FIELD_OF_VIEW }
}

pub fn preset_pose(scene: &StructuralScene, preset: CameraPreset) -> CameraPose {
    fit(&scene.bounds, preset)
}

/// 🎯️ Fit to the union of the selected entities; falls back to the scene bounds when nothing selectable matches.
pub fn fit_selection(scene: &StructuralScene, ids: &[String], preset: CameraPreset) -> CameraPose {
    let points: Vec<V3> = scene.entities.iter().filter(|e| ids.contains(&e.entity.id)).flat_map(|e| entity_points(&e.geometry)).collect();
    bounds_of(&points).map_or_else(|| preset_pose(scene, preset), |b| fit(&b, preset))
}

/// 🚨️ Entity with the highest utilisation ratio, if any entity carries one.
pub fn worst_entity(scene: &StructuralScene) -> Option<&str> {
    scene.entities.iter().filter_map(|e| e.utilisation.as_ref().map(|u| (e.entity.id.as_str(), u.ratio))).max_by(|a, b| a.1.total_cmp(&b.1)).map(|(id, _)| id)
}

pub fn fit_worst(scene: &StructuralScene, preset: CameraPreset) -> CameraPose {
    fit_selection(scene, &worst_entity(scene).map(|id| vec![id.to_string()]).unwrap_or_default(), preset)
}
