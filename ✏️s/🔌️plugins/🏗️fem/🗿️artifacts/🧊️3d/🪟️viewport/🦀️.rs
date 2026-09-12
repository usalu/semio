//! 🪟️ Initial navigation pose shared by FEM 3D editor and viewer windows.

/// 📐️ Initial FEM model orientation; each open window retains its own later pose.
pub const INITIAL: crate::Viewport3dOrbit = crate::Viewport3dOrbit { position: [4.0, -4.0, 3.0], target: [0.0; 3], zoom: 1.0, up: None };

/// 🎬️ Encodes the exact retained pose at the current world-scene wire boundary.
pub fn scene_camera_json(camera: &crate::Viewport3dOrbit) -> String {
    dsl::json::to_json_string(&dsl::ToValue::to_value(camera))
}
