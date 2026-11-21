//! 📚️ Example failing Blatt 7.

use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;

pub const ID: &str = "blatt-7-fail";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Part 7 fan with wrong record count", "Ventilator nach Blatt 7 mit falscher Datensatzanzahl")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/blatt-7-fail/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🎩️example/🦀️.rs"]
mod example;
