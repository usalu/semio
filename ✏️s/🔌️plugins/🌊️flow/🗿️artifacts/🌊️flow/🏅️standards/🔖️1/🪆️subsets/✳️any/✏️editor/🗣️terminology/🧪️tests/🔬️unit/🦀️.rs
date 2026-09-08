
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(flow_play_labels(&semio_framework_plugin::ViewModel::default()).synapses.as_str(), "Synapses");
    assert_eq!(flow_play_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).synapses.as_str(), "Synapsen");
}

#[semio_framework_async_macros::async_test]
async fn unknown_extension_ids_fall_back_to_runtime_data_labels() {
    let labels = flow_play_labels(&semio_framework_plugin::ViewModel::default());
    assert_eq!(flow_extension_label("auto-layout", "Auto Layout", labels), labels.extension_auto_layout.into());
    assert_eq!(flow_extension_label("third-party", "Third Party", labels), Label::data("Third Party"));
    assert_eq!(flow_extension_action_title_label("flow.extension.evaluate", "Evaluate Fixture", labels), labels.extension_action_evaluate_fixture.into());
    assert_eq!(flow_extension_action_title_label("third.party", "Do Thing", labels), Label::data("Do Thing"));
}
