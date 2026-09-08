
use super::*;
use crate::editor::writer::terminology::writer_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_slider_range_matches_the_command_handler_clamp() {
    let config = WriterConfig::default();
    match measure(&config, writer_play_labels(&config)) {
        WindowMeasure::Slider { min, max, .. } => assert!(min == 1.0 && max == 8.0),
        other => panic!("tab-size measure must be a slider, got {other:?}"),
    }
}
