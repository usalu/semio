//! 🔤️ Main-window option — the font-size slider. Its command handler lives in
//! `🎮️commands/⚙️set-font-px::set_font_px`.

use crate::editor::writer::config::WriterConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use crate::editor::writer::writer_action;
use semio_framework_plugin::WindowMeasure;

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
        on_change: writer_action("setEditorSetting", Some(dsl::DslValue::object([("field".into(), dsl::DslValue::String("fontPx".into()))]))),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
