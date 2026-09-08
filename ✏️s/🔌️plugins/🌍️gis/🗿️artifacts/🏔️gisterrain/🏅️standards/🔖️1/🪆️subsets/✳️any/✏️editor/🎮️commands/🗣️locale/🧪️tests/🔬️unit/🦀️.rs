
use super::*;
use crate::editor::gis3d::Gis3dCommand;
use crate::editor::gis3d::testkit::{app, dispatch};

/// 🗣️ `SetLocale` is not palette-declared but still dispatches cleanly end-to-end (command_id
/// mapping → `handle` → config store) — the same typed channel the shell uses to push locale.
#[semio_framework_async_macros::async_test]
async fn locale_command_dispatches_through_the_config_store() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })).await;
    assert!(result.mutations.is_empty(), "locale is config state, not a document edit");
}

#[semio_framework_async_macros::async_test]
async fn set_locale_is_not_declared_in_the_manifest() {
    let definition = crate::editor::gis3d::create_gis3d_app();
    assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "setLocale"), "locale is host-pushed, never palette-staged");
}
