use super::*;

#[test]
fn labels_resolve_native_english_and_german_from_the_shared_view_state() {
    assert_eq!(generation3d_labels(&semio_framework_plugin::ViewModel::default()).widgets.as_str(), "Widgets");
    assert_eq!(generation3d_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).widgets.as_str(), "Elemente");
}

#[test]
fn unknown_catalog_kind_falls_back_to_the_id_itself() {
    let labels = generation3d_labels(&semio_framework_plugin::ViewModel::default());
    assert_eq!(generation3d_catalog_label("bogusKind", labels), "bogusKind");
}
