//! 🪟️ Initial navigation pose shared by FEM 3D editor and viewer windows.

/// 📐️ Initial FEM model orientation — frames the bundled 12 m × 6 m × 6.5 m demo hall from its
/// front-right corner; each open window retains its own later pose.
pub const INITIAL: crate::Viewport3dOrbit = crate::Viewport3dOrbit { position: [24.0, -20.0, 14.0], target: [6.0, 3.0, 3.0], zoom: 1.0, up: None };

/// 🎬️ Encodes the exact retained pose at the current world-scene wire boundary.
pub fn scene_camera_json(camera: &crate::Viewport3dOrbit) -> String {
    dsl::json::to_json_string(&dsl::ToValue::to_value(camera))
}
