//! 📚️ Example for the published Blatt 16 scope.

use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;

pub const ID: &str = "blatt-16";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Blatt 16 scope example", "Blatt 16 Anwendungsbeispiel")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/blatt-16/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}


#[cfg(test)]
#[path = "🧪️tests/🎩️example/🦀️.rs"]
mod example;
