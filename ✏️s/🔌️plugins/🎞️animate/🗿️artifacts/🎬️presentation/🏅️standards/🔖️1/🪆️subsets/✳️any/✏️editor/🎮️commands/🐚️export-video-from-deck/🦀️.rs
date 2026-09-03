//! 🐚️ 🐚️ Animate presentation app commands command — `export-video-from-deck`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::engine::export_video_from_scene;
use crate::editor::animate::engine::PresentationScene;
use crate::editor::animate::PresentationDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

async fn export_video_from_deck(scene: &PresentationScene, output_dir: &str) -> Result<Vec<crate::editor::animate::engine::SceneAssetBundle>, crate::editor::animate::engine::PresentationVideoExportError> {
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

pub async fn handle_async(payload: &ExportVideoFromDeck) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let scene: PresentationScene = dsl::os_pack::json::parse(&payload.scene_json)
        .ok()
        .and_then(|value| dsl::FromValue::from_value(dsl::os_pack::json::to_dsl_value(&value)).ok())
        .unwrap_or_else(|| PresentationScene::empty("Deck export"));
    match export_video_from_deck(&scene, &payload.output_dir).await {
        Ok(bundles) => Ok(Emit {
            effects: vec![Effect::DownloadMediaExport {
                filename: "animate-video-export.ops".into(),
                mime_type: "text/plain".into(),
                data: {
                    let value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&bundles));
                    dsl::os_pack::json::to_string_pretty(&value)
                },
                encoding: None,
            }],
            ..Default::default()
        }),
        Err(error) => Ok(Emit { effects: vec![Effect::DownloadMediaExport { filename: "animate-video-export-error.txt".into(), mime_type: "text/plain".into(), data: error.to_string(), encoding: None }], ..Default::default() }),
    }
}

pub fn handle(_payload: &ExportVideoFromDeck, _doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("animate.video.export.async-route"), "video export must be dispatched through the editor async boundary"))
}
