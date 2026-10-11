//! 🚪️ Artifact text representations.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;

use super::super::GisMapViewerCamera;
impl GisMapViewerCamera {
    /// 🎬️ The exact `TiledMapScene::camera_json` string this camera stands for.
    pub fn scene_camera_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(self))
    }
}
