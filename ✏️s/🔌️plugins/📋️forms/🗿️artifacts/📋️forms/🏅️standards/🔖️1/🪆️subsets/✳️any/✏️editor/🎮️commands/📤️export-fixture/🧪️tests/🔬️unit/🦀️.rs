use super::*;
use crate::editor::forms::unit_tests::context::{forms_app, settle};
use crate::editor::forms::FormsCommand;
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::PluginApp;
use ExportFixture;

/// 📤️ On a MOUNTED app `InvocationResult.requested_effects` is empty — the host effect rides the
/// settled publication receipt, not the dispatch return — so the download effect is read there.
#[semio_framework_async_macros::async_test]
async fn export_fixture_downloads_the_forms_dsl_text() {
    let mut app = forms_app().await;
    app.dispatch_typed(FormsCommand::ExportFixture(ExportFixture {}), &meta("local")).await.expect("dispatch");
    let receipt = settle(&mut app).await;
    assert!(!receipt.effects.is_empty(), "exportFixture must emit a host effect");
}
