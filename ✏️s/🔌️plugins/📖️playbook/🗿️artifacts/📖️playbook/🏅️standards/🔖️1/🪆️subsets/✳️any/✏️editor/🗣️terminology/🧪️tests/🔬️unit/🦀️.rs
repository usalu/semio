use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    let english = playbook_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let german = playbook_play_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::De, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) });
    assert_eq!((english.kind_arg.as_str(), english.files_step.as_str(), english.files_block.as_str()), ("Kind", "Step", "Block"));
    assert_eq!((german.kind_arg.as_str(), german.files_step.as_str(), german.files_block.as_str()), ("Art", "Schritt", "Baustein"));
}
