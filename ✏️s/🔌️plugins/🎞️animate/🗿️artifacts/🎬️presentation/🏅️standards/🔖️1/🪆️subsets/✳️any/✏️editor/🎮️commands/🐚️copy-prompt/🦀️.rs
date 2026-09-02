//! 🐚️ 🐚️ Animate presentation app commands command — `copy-prompt`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{tile_morph_prompt_effect, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CopyPrompt
//#endregion 🔖️CopyPrompt

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "copy-prompt")]
pub struct CopyPrompt {}

pub fn handle(_payload: &CopyPrompt, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    Ok(Emit { effects: vec![tile_morph_prompt_effect(doc.snapshot)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::commands::export_video_from_deck;
    use crate::editor::animate::testkit::{presentation_app, presentation_app_with_registry};
    use crate::editor::animate::PresentationCommand;
    use semio_framework_plugin::{testkit::meta, Effect};

    #[semio_framework_async_macros::async_test]
    async fn copy_prompt_is_shell_effect_not_view_mutation() {
        let mut app = presentation_app_with_registry().await;
        app.dispatch_typed(PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 }), &meta("local")).await.expect("seed grid");
        let result = app.dispatch_typed(PresentationCommand::CopyPrompt(CopyPrompt {}), &meta("local")).await.expect("copy prompt");
        assert!(result.mutations.is_empty(), "copyPrompt is a host effect, not a document operation");
        assert!(matches!(result.requested_effects.as_slice(), [Effect::DownloadMediaExport { mime_type, .. }] if mime_type == "text/markdown"), "copyPrompt emits exactly one media-export host effect carrying the morph prompt");
    }

    #[semio_framework_async_macros::async_test]
    async fn export_video_from_deck_reports_no_scene_hashes_as_download_error() {
        let mut app = presentation_app().await;
        let result = app.dispatch_typed(PresentationCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck { output_dir: "output/animate-video".into(), scene_json: "{}".into() }), &meta("local")).await.expect("export");
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
