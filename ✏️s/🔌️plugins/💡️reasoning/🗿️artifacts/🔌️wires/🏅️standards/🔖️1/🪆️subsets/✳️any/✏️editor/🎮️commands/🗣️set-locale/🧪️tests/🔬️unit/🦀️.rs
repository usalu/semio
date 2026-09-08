
use super::*;
use crate::editor::wires::testkit::{dispatch, metabolism_app, render};
use crate::editor::wires::{WIRES_PLAY_BODY_DOCUMENT, WiresCommand};

/// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
/// `ViewModel.locale` threaded through `render` (the trait dropped `ViewModel` entirely).
#[semio_framework_async_macros::async_test]
async fn wires_labels_resolve_native_in_german() {
    let mut app = metabolism_app().await;
    dispatch(&mut app, WiresCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
    let json = render(&mut app, WIRES_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("Identitäten"));
    assert!(json.contains("Beziehungen"));
}
