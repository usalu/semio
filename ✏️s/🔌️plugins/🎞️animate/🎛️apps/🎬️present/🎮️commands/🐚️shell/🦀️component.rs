//! 🐚️ Animate present app commands — host shell effects: copy-prompt, export-video-from-deck. Both
//! export round-trip through the host; neither ever emits document or config operations.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::tile_morph_prompt_effect;
use crate::artifacts::present::engine::export_video_from_scene;
use crate::artifacts::present::engine::PresentScene;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentDeck;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

fn export_video_from_deck(scene: &PresentScene, output_dir: &str) -> Result<Vec<crate::artifacts::present::engine::SceneAssetBundle>, crate::artifacts::present::engine::PresentError> {
    export_video_from_scene(scene, std::path::Path::new(output_dir))
}

//#region 🔖️CopyPrompt
pub mod copy_prompt {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "copy-prompt")]
    pub struct CopyPrompt {}

    pub fn handle(_payload: &CopyPrompt, doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        Ok(Emit::effect(tile_morph_prompt_effect(doc.projection)))
    }
}
//#endregion 🔖️CopyPrompt

//#region 🔖️ExportVideoFromDeck
pub mod export_video_from_deck {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-video-from-deck")]
    pub struct ExportVideoFromDeck {
        pub output_dir: String,
        pub scene_json: String,
    }

    pub fn handle(payload: &ExportVideoFromDeck, _doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        let scene = serde_json::from_str::<PresentScene>(&payload.scene_json).unwrap_or_else(|_| PresentScene::empty("Deck export"));
        match export_video_from_deck(&scene, &payload.output_dir) {
            Ok(bundles) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: "animate-video-export.ops".into(), mime_type: "text/plain".into(), data: serde_json::to_string_pretty(&bundles).unwrap_or_else(|_| "[]".into()), encoding: None })),
            Err(error) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: "animate-video-export-error.txt".into(), mime_type: "text/plain".into(), data: error.to_string(), encoding: None })),
        }
    }
}
//#endregion 🔖️ExportVideoFromDeck

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{present_app, present_app_with_registry};
    use crate::apps::present::PresentCommand;
    use semio_framework_plugin::testkit::meta;

    #[test]
    fn copy_prompt_is_shell_effect_not_view_mutation() {
        let mut app = present_app_with_registry();
        app.dispatch_typed(PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 2, columns: 2 }), &meta("local")).expect("seed grid");
        let result = app.dispatch_typed(PresentCommand::CopyPrompt(copy_prompt::CopyPrompt {}), &meta("local")).expect("copy prompt");
        assert!(result.document_mutations.is_empty(), "copyPrompt is a host effect, not a document operation");
        assert!(matches!(result.requested_effects.as_slice(), [HostEffect::DownloadMediaExport { mime_type, .. }] if mime_type == "text/markdown"), "copyPrompt emits exactly one media-export host effect carrying the morph prompt");
    }

    #[test]
    fn export_video_from_deck_reports_no_scene_hashes_as_download_error() {
        let mut app = present_app();
        let result = app.dispatch_typed(PresentCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck { output_dir: "output/animate-video".into(), scene_json: "{}".into() }), &meta("local")).expect("export");
        match result.requested_effects.as_slice() {
            [HostEffect::DownloadMediaExport { filename, mime_type, data, .. }] => {
                assert_eq!(filename, "animate-video-export-error.txt");
                assert_eq!(mime_type, "text/plain");
                assert!(!data.is_empty());
            }
            other => panic!("expected a single download error effect, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
