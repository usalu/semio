use super::*;
use crate::editor::animate::unit_tests::context::presentation_app_with_registry;
use crate::editor::animate::PresentationCommand;
use semio_framework_plugin::Effect;

#[semio_framework_async_macros::async_test]
async fn copy_prompt_is_shell_effect_not_view_mutation() {
    let mut app = presentation_app_with_registry().await;
    crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::SeedGrid(crate::editor::animate::commands::seed_grid::SeedGrid { rows: 2, columns: 2 })).await;
    let result = crate::editor::animate::unit_tests::context::dispatch(&mut app, PresentationCommand::CopyPrompt(CopyPrompt {})).await;
    assert!(result.mutations.is_empty(), "copyPrompt is a host effect, not a document operation");
    assert!(matches!(result.requested_effects.as_slice(), [Effect::DownloadMediaExport { mime_type, .. }] if mime_type == "text/markdown"), "copyPrompt emits exactly one media-export host effect carrying the morph prompt");
}
