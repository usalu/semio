//! 📚️ Example conforming Blatt 60.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "blatt-60";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Blatt 60 conforming", "Blatt 60 konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/blatt-60/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🎩️example/🦀️.rs"]
mod example;
