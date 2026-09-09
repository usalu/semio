use super::*;
use crate::editor::writer::terminology::writer_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_toggle_reflects_the_configs_show_line_numbers_flag() {
    let config = WriterMainWindowConfig { editor_settings: crate::WriterEditorSettings { show_line_numbers: false, ..Default::default() }, ..WriterMainWindowConfig::default() };
    match measure(&config, writer_play_labels(&semio_framework_plugin::ViewModel::default())) {
        WindowMeasure::Toggle { pressed, .. } => assert!(!pressed),
        other => panic!("line-numbers measure must be a toggle, got {other:?}"),
    }
}
