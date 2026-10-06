use super::*;
use crate::editor::forms::unit_tests::context::{forms_app, settle};
use crate::editor::forms::FormsCommand;
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::PluginApp;
use ExportSnapshot;

/// 📤️ On a MOUNTED app `InvocationResult.requested_effects` is empty — the host effect rides the
/// settled publication receipt, not the dispatch return — so the download effect is read there.
#[semio_framework_async_macros::async_test]
async fn export_snapshot_downloads_the_forms_dsl_text() {
    let mut app = forms_app().await;
    app.dispatch_typed(FormsCommand::ExportSnapshot(ExportSnapshot {}), &meta("local")).await.expect("dispatch");
    let receipt = settle(&mut app).await;
    let (filename, data) = receipt.effects.iter().find_map(|effect| match effect {
        semio_framework::kernel::Effect::DownloadMediaExport { filename, data, .. } => Some((filename, data)),
        _ => None,
    }).expect("Export Form download");
    assert_eq!(filename, &format!("{}.forms", app.snapshot().unwrap().id));
    let decoded = forms_dsl::parse_dsl(data).expect("export is a valid native Forms document");
    assert_eq!(decoded, app.snapshot().unwrap());
}
