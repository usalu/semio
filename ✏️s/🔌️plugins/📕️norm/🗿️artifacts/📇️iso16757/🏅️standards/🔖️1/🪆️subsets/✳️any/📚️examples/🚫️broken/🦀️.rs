//! 📚️ Example `broken` — catalogue with many ISO 16757 violations.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "broken";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Broken catalogue", "Fehlerhafter Katalog")
}
pub const ICON: &str = "file-warning";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🚫️broken/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/📚️example/🦀️.rs"]
mod example;
