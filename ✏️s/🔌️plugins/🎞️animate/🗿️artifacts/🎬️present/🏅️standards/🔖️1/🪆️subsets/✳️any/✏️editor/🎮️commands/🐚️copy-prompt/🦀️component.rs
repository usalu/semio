//! 🐚️ 🐚️ Animate present app commands command — `copy-prompt`.

use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::engine::export_video_from_scene;
use crate::editor::animate::engine::PresentScene;
use crate::editor::animate::{tile_morph_prompt_effect, PresentDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use serde::{Deserialize, Serialize};

async fn export_video_from_deck(scene: &PresentScene, output_dir: &str) -> Result<Vec<crate::editor::animate::engine::SceneAssetBundle>, crate::editor::animate::engine::PresentVideoExportError> {
    export_video_from_scene(scene, std::path::Path::new(output_dir))
}

//#region 🔖️CopyPrompt
//#endregion 🔖️CopyPrompt

//#region 🔖️ExportVideoFromDeck
//#endregion 🔖️ExportVideoFromDeck

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "copy-prompt")]
pub struct CopyPrompt {}

pub async fn handle(_payload: &CopyPrompt, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit::effect(tile_morph_prompt_effect(doc.snapshot)))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::commands::export_video_from_deck;
    use crate::editor::animate::testkit::{present_app, present_app_with_registry};
    use crate::editor::animate::PresentCommand;
    use semio_framework_plugin::testkit::meta;

    #[semio_framework_async_macros::async_test]
    async fn copy_prompt_is_shell_effect_not_view_mutation() {
        let mut app = present_app_with_registry();
        app.dispatch_typed(PresentCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 }), &meta("local")).expect("seed grid");
        let result = app.dispatch_typed(PresentCommand::CopyPrompt(CopyPrompt {}), &meta("local")).expect("copy prompt");
        assert!(result.mutations.is_empty(), "copyPrompt is a host effect, not a document operation");
        assert!(matches!(result.requested_effects.as_slice(), [Effect::DownloadMediaExport { mime_type, .. }] if mime_type == "text/markdown"), "copyPrompt emits exactly one media-export host effect carrying the morph prompt");
    }

    #[semio_framework_async_macros::async_test]
    async fn export_video_from_deck_reports_no_scene_hashes_as_download_error() {
        let mut app = present_app();
        let result = app.dispatch_typed(PresentCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck { output_dir: "output/animate-video".into(), scene_json: "{}".into() }), &meta("local")).expect("export");
        match result.requested_effects.as_slice() {
            [Effect::DownloadMediaExport { filename, mime_type, data, .. }] => {
                assert_eq!(filename, "animate-video-export-error.txt");
                assert_eq!(mime_type, "text/plain");
                assert!(!data.is_empty());
            }
            other => panic!("expected a single download error effect, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
