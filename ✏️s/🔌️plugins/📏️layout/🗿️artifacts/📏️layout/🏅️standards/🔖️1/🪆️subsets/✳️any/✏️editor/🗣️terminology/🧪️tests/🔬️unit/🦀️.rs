
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(layout_labels(&semio_framework_plugin::ViewModel::default()).frames.as_str(), "Frames");
    assert_eq!(layout_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).frames.as_str(), "Rahmen");
}

#[semio_framework_async_macros::async_test]
async fn catalogue_kind_label_resolves_known_kinds_and_falls_back_to_the_id() {
    let labels = layout_labels(&semio_framework_plugin::ViewModel::default());
    assert_eq!(catalogue_kind_label("rect", labels), labels.kind_rect.into());
    assert_eq!(catalogue_kind_label("bogus", labels), Label::data("bogus"));
}

#[semio_framework_async_macros::async_test]
async fn preflight_msg_fills_positional_placeholders_in_order() {
    let labels = layout_labels(&semio_framework_plugin::ViewModel::default());
    assert_eq!(preflight_msg(labels.preflight_font_missing, &["Comic Sans", "frame-1"]), "Font Comic Sans used by frame-1 is not available");
}
