//! 🐚️ 🐚️ Animate present app commands command — `copy-prompt`.

#![allow(clippy::result_large_err)]

use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::{tile_morph_prompt_effect, PresentDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CopyPrompt
//#endregion 🔖️CopyPrompt

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "copy-prompt")]
pub struct CopyPrompt {}

pub fn handle(_payload: &CopyPrompt, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    Ok(Emit { effects: vec![tile_morph_prompt_effect(doc.snapshot)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::commands::export_video_from_deck;
    use crate::editor::animate::testkit::{present_app, present_app_with_registry};
    use crate::editor::animate::PresentCommand;
    use semio_framework_plugin::{testkit::meta, Effect};

    #[semio_framework_async_macros::async_test]
    async fn copy_prompt_is_shell_effect_not_view_mutation() {
        let mut app = present_app_with_registry().await;
        app.dispatch_typed(PresentCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 }), &meta("local")).await.expect("seed grid");
        let result = app.dispatch_typed(PresentCommand::CopyPrompt(CopyPrompt {}), &meta("local")).await.expect("copy prompt");
        assert!(result.mutations.is_empty(), "copyPrompt is a host effect, not a document operation");
        assert!(matches!(result.requested_effects.as_slice(), [Effect::DownloadMediaExport { mime_type, .. }] if mime_type == "text/markdown"), "copyPrompt emits exactly one media-export host effect carrying the morph prompt");
    }

    #[semio_framework_async_macros::async_test]
    async fn export_video_from_deck_reports_no_scene_hashes_as_download_error() {
        let mut app = present_app().await;
        let result = app.dispatch_typed(PresentCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck { output_dir: "output/animate-video".into(), scene_json: "{}".into() }), &meta("local")).await.expect("export");
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
