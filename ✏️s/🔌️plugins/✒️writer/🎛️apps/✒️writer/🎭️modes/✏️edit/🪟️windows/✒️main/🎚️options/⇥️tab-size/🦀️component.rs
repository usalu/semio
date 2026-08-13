//! ⇥️ Main-window option — the tab-size slider. Its command handler lives in
//! `🎮️commands/⚙️toggle-line-numbers::set_tab_size`.

use crate::apps::writer::config::WriterConfig;
use crate::apps::writer::terminology::WriterPlayLabels;
use crate::apps::writer::writer_action;
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

//#region 🔖️Measure
pub fn measure(config: &WriterConfig, labels: &WriterPlayLabels) -> WindowMeasure {
    let settings = &config.editor_settings;
    WindowMeasure::Slider {
        id: "writer-tab-size-measure".into(),
        label: Some(labels.tab_size.into()),
        value: settings.tab_size as f64,
        min: 1.0,
        max: 8.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: writer_action("setEditorSetting", Some(json!({ "field": "tabSize" }))),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::writer::terminology::writer_play_labels;

    #[test]
    fn the_slider_range_matches_the_command_handler_clamp() {
        let config = WriterConfig::default();
        match measure(&config, writer_play_labels(&config)) {
            WindowMeasure::Slider { min, max, .. } => assert!(min == 1.0 && max == 8.0),
            other => panic!("tab-size measure must be a slider, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
