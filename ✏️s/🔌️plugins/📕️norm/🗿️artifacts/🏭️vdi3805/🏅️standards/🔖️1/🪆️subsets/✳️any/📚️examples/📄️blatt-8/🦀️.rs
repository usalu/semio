//! 📚️ Example conforming Blatt 8.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "blatt-8";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Blatt 8 conforming", "Blatt 8 konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/blatt-8/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🎩️example/🦀️.rs"]
mod example;
