use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use crate::editor::shooting::terminology::ShootingLabels;
use crate::editor::shooting::modes::edit::windows::scene::options::center_model::*;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn center_model_toggle_starts_pressed() {
    let labels = shooting_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    match measure(labels) {
        WindowMeasure::Toggle { pressed, .. } => assert!(pressed),
        other => panic!("center-model measure must be a toggle, got {other:?}"),
    }
}
