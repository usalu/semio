//! 📏️ Main-window option — the line-height slider. Its command handler lives in
//! `🎮️commands/📏️set-line-height::set_line_height`.

use crate::editor::writer::config::WriterConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use crate::editor::writer::writer_action;
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

//#region 🔖️Measure
pub fn measure(config: &WriterConfig, labels: &WriterPlayLabels) -> WindowMeasure {
    let settings = &config.editor_settings;
    WindowMeasure::Slider {
        id: "writer-line-height-measure".into(),
        label: Some(labels.line_height.into()),
        value: settings.line_height as f64,
        min: 16.0,
        max: 40.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: writer_action("setEditorSetting", Some(json!({ "field": "lineHeight" }))),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::writer::terminology::writer_play_labels;

    #[semio_framework_async_macros::async_test]
    async fn the_slider_range_matches_the_command_handler_clamp() {
        let config = WriterConfig::default();
        match measure(&config, writer_play_labels(&config)) {
            WindowMeasure::Slider { min, max, .. } => assert!(min == 16.0 && max == 40.0),
            other => panic!("line-height measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
