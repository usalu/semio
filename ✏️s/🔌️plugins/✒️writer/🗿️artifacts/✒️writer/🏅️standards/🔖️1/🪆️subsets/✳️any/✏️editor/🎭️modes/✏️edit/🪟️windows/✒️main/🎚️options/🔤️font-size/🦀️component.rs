//! 🔤️ Main-window option — the font-size slider. Its command handler lives in
//! `🎮️commands/⚙️toggle-line-numbers::set_font_px`.

use crate::editor::writer::config::WriterConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use crate::editor::writer::writer_action;
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

//#region 🔖️Measure
pub fn measure(config: &WriterConfig, labels: &WriterPlayLabels) -> WindowMeasure {
    let settings = &config.editor_settings;
    WindowMeasure::Slider {
        id: "writer-font-size-measure".into(),
        label: Some(labels.font_size.into()),
        value: settings.font_px as f64,
        min: 10.0,
        max: 24.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: writer_action("setEditorSetting", Some(json!({ "field": "fontPx" }))),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::writer::terminology::writer_play_labels;

    #[test]
    fn the_slider_range_matches_the_command_handler_clamp() {
        let config = WriterConfig::default();
        match measure(&config, writer_play_labels(&config)) {
            WindowMeasure::Slider { min, max, .. } => assert!(min == 10.0 && max == 24.0),
            other => panic!("font-size measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
