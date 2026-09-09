use super::*;

#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_native_english_by_default() {
    assert_eq!(shooting_play_labels(&semio_framework_plugin::ViewModel::default()).shots.as_str(), "Shots");
}

#[semio_framework_async_macros::async_test]
async fn shooting_labels_resolve_german_from_the_shared_view_model() {
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    assert_eq!(shooting_play_labels(&view_state).shots.as_str(), "Aufnahmen");
}
