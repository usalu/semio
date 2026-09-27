//! 📚️ Example conforming Blatt 53.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "blatt-53";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Blatt 53 conforming", "Blatt 53 konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/blatt-53/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🎩️example/🦀️.rs"]
mod example;
