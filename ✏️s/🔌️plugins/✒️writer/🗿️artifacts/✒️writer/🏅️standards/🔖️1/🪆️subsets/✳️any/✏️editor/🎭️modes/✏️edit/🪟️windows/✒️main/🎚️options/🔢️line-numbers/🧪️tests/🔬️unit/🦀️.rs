
use super::*;
use crate::editor::writer::terminology::writer_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_toggle_reflects_the_configs_show_line_numbers_flag() {
    let config = WriterConfig { editor_settings: crate::editor::writer::config::WriterEditorSettings { show_line_numbers: false, ..Default::default() }, ..WriterConfig::default() };
    match measure(&config, writer_play_labels(&config)) {
        WindowMeasure::Toggle { pressed, .. } => assert!(!pressed),
        other => panic!("line-numbers measure must be a toggle, got {other:?}"),
    }
}
