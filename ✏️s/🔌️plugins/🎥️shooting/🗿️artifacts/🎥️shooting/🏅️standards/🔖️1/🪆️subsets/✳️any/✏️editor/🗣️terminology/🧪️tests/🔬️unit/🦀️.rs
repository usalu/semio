use crate::editor::shooting::terminology::*;

#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_native_english_by_default() {
    assert_eq!(shooting_play_labels(&semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).shots.as_str(), "Shots");
}

#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_german_from_the_shared_view_model() {
    let view_state = semio_plugin::ViewModel { locale: semio_ui_locale::Locale::De, ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::De, semio_ui_locale::Terminology::Native) };
    assert_eq!(shooting_play_labels(&view_state).shots.as_str(), "Aufnahmen");
}
