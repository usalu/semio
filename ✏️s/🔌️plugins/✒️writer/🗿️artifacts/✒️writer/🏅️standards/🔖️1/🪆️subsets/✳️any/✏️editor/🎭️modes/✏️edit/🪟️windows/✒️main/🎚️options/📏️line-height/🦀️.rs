//! 📏️ Main-window option — the line-height slider. Its command handler lives in
//! `🎮️commands/📏️set-line-height::set_line_height`.

use crate::editor::writer::modes::edit::windows::main::config::WriterMainWindowConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use crate::editor::writer::writer_action;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(config: &WriterMainWindowConfig, labels: &WriterPlayLabels) -> WindowMeasure {
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
        on_change: writer_action("setEditorSetting", Some(dsl::DslValue::object([("field".into(), dsl::DslValue::String("lineHeight".into()))]))),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
