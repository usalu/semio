//! 🐚️ 🐚️ Animate present app commands command — `export-video-from-deck`.

use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::PresentDispatchCtx;
use crate::editor::animate::engine::export_video_from_scene;
use crate::editor::animate::engine::PresentScene;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, Effect};
use serde::{Deserialize, Serialize};

fn export_video_from_deck(scene: &PresentScene, output_dir: &str) -> Result<Vec<crate::editor::animate::engine::SceneAssetBundle>, crate::editor::animate::engine::PresentVideoExportError> {
    export_video_from_scene(scene, std::path::Path::new(output_dir))
}

//#region 🔖️ExportVideoFromDeck
//#endregion 🔖️ExportVideoFromDeck

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-video-from-deck")]
pub struct ExportVideoFromDeck {
    pub output_dir: String,
    pub scene_json: String,
}

pub fn handle(payload: &ExportVideoFromDeck, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let scene = serde_json::from_str::<PresentScene>(&payload.scene_json).unwrap_or_else(|_| PresentScene::empty("Deck export"));
    match export_video_from_deck(&scene, &payload.output_dir) {
        Ok(bundles) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: "animate-video-export.ops".into(), mime_type: "text/plain".into(), data: serde_json::to_string_pretty(&bundles).unwrap_or_else(|_| "[]".into()), encoding: None })),
        Err(error) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: "animate-video-export-error.txt".into(), mime_type: "text/plain".into(), data: error.to_string(), encoding: None })),
    }
}
