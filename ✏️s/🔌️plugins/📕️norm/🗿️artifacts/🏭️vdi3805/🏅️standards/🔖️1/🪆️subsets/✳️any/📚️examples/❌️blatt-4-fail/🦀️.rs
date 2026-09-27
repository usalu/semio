//! 📚️ Example failing Blatt 4.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "blatt-4-fail";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Blatt 4 nonconforming", "Blatt 4 nicht konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/blatt-4-fail/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🎩️example/🦀️.rs"]
mod example;
