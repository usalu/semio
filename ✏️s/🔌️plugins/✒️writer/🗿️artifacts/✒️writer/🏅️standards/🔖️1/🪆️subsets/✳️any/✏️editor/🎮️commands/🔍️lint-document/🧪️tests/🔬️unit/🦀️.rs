use super::LintDocument;
use crate::editor::writer::commands::set_active_example;
use crate::editor::writer::unit_tests::context::{dispatch, new_app_with_registry};
use crate::editor::writer::WriterCommand;
use crate::{writer_text, WriterSnapshot};
use semio_framework::kernel::Effect;

#[semio_framework_async_macros::async_test]
async fn lint_is_a_view_action_and_example_default_materializes() {
    let mut app = new_app_with_registry().await;
    // lintDocument is a declared View action: registry kind discipline requires it emit no operations.
    let result = dispatch(&mut app, WriterCommand::LintDocument(LintDocument {})).await;
    assert!(result.mutations.is_empty(), "lint re-runs diagnostics into runtime, never the document");
    // setActiveExample fired with the declared default example (`crate::examples::demo::ID`, the ONE
    // id this subset publishes — `"jack"` was never published and fell through to the EMPTY document) — whole-document replace
    // is not an in-history mutation (`SetSnapshot` is banned outright), so this surfaces as a
    // `Effect::LoadDocument`, not a live `app.snapshot()` change (see `reset_document_effect`).
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("expected a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let projection = <WriterSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert!(!writer_text(&projection).is_empty(), "jack default materialized from the registry");
}
