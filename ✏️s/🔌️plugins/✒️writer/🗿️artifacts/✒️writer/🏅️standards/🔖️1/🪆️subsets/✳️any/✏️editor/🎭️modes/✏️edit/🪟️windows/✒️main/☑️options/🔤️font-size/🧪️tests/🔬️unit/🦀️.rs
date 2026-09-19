use super::*;
use crate::editor::writer::terminology::writer_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_slider_range_matches_the_command_handler_clamp() {
    let config = WriterMainWindowConfig::default();
    match measure(&config, writer_play_labels(&semio_framework_plugin::ViewModel::default())) {
        WindowMeasure::Slider { min, max, .. } => assert!(min == 10.0 && max == 24.0),
        other => panic!("font-size measure must be a slider, got {other:?}"),
    }
}
