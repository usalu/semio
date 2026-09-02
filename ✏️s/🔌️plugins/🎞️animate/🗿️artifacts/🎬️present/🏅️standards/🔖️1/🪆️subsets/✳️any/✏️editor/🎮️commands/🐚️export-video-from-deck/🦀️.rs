//! 🐚️ 🐚️ Animate present app commands command — `export-video-from-deck`.

#![allow(clippy::result_large_err)]

use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::engine::export_video_from_scene;
use crate::editor::animate::engine::PresentScene;
use crate::editor::animate::PresentDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

async fn export_video_from_deck(scene: &PresentScene, output_dir: &str) -> Result<Vec<crate::editor::animate::engine::SceneAssetBundle>, crate::editor::animate::engine::PresentVideoExportError> {
    export_video_from_scene(scene, std::path::Path::new(output_dir)).await
}

//#region 🔖️ExportVideoFromDeck
//#endregion 🔖️ExportVideoFromDeck

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-video-from-deck")]
pub struct ExportVideoFromDeck {
    pub output_dir: String,
    pub scene_json: String,
}

pub async fn handle_async(payload: &ExportVideoFromDeck) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let scene: PresentScene = serde_json::from_str::<serde_json::Value>(&payload.scene_json)
        .ok()
        .and_then(|value| dsl::FromValue::from_value(value.into()).ok())
        .unwrap_or_else(|| PresentScene::empty("Deck export"));
    match export_video_from_deck(&scene, &payload.output_dir).await {
        Ok(bundles) => Ok(Emit {
            effects: vec![Effect::DownloadMediaExport {
                filename: "animate-video-export.ops".into(),
                mime_type: "text/plain".into(),
                data: {
                    let value: serde_json::Value = dsl::ToValue::to_value(&bundles).into();
                    serde_json::to_string_pretty(&value).unwrap_or_else(|_| "[]".into())
                },
                encoding: None,
            }],
            ..Default::default()
        }),
        Err(error) => Ok(Emit { effects: vec![Effect::DownloadMediaExport { filename: "animate-video-export-error.txt".into(), mime_type: "text/plain".into(), data: error.to_string(), encoding: None }], ..Default::default() }),
    }
}

pub fn handle(_payload: &ExportVideoFromDeck, _doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("animate.video.export.async-route"), "video export must be dispatched through the editor async boundary"))
}
