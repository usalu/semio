//! 🔢️ Main-window option — the line-numbers toggle. Its command handler lives in
//! `🎮️commands/🔢️toggle-line-numbers::toggle_line_numbers`.

use crate::editor::writer::config::WriterConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use crate::editor::writer::writer_action;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(config: &WriterConfig, labels: &WriterPlayLabels) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: "writer-line-numbers-measure".into(),
        icon_id: "list-ordered".into(),
        label: Some(labels.line_numbers.into()),
        pressed: config.editor_settings.show_line_numbers,
        text: None,
        on_change: writer_action("toggleLineNumbers", None),
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
