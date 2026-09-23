use super::*;
use crate::editor::animate::commands::export_video_from_deck;
use crate::editor::animate::unit_tests::context::{presentation_app, presentation_app_with_registry};
use crate::editor::animate::PresentationCommand;
use semio_framework_plugin::{artifact_app_laws::meta, Effect};

#[semio_framework_async_macros::async_test]
async fn copy_prompt_is_shell_effect_not_view_mutation() {
    let mut app = presentation_app_with_registry().await;
    crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 })).await;
    let result = crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::CopyPrompt(CopyPrompt {})).await;
    assert!(result.mutations.is_empty(), "copyPrompt is a host effect, not a document operation");
    assert!(matches!(result.requested_effects.as_slice(), [Effect::DownloadMediaExport { mime_type, .. }] if mime_type == "text/markdown"), "copyPrompt emits exactly one media-export host effect carrying the morph prompt");
}

/// 🎥️ `exportVideoFromDeck` compiles scene assets into a filesystem directory, so the manifest classifies
/// it `BatchOnlyPendingRewrite`: the UI route must refuse it before any handler runs, and the batch
/// route (`handle_async`) is where the export — and its empty-deck error report — lives.
#[semio_framework_async_macros::async_test]
async fn export_video_from_deck_reports_no_scene_hashes_as_download_error() {
    let payload = export_video_from_deck::ExportVideoFromDeck { output_dir: "output/animate-video".into(), scene_json: "{}".into() };
    let mut app = presentation_app().await;
    let refused = app.dispatch_typed(PresentationCommand::ExportVideoFromDeck(payload.clone()), &meta("local")).await.expect_err("a batch-only export is never UI-dispatchable");
    assert_eq!(refused.code.0, "interactive-job.not-ui-safe");
    let emit = export_video_from_deck::handle_async(&payload).await.expect("batch export route");
    match emit.effects.as_slice() {
        [Effect::DownloadMediaExport { filename, mime_type, data, .. }] => {
            assert_eq!(filename, "animate-video-export-error.txt");
            assert_eq!(mime_type, "text/plain");
            assert!(!data.is_empty());
        }
        other => panic!("expected a single download error effect, got {other:?}"),
    }
}
