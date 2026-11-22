//! 📚️ Published scope example for Blatt 22.

use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;

pub const ID: &str = "blatt-22";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Blatt 22 Burner", "Blatt 22 Brenner")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/blatt-22/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🎩️example/🦀️.rs"]
mod example;
