//! 📚️ Solar catalogue with incorrect record count.

use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;

pub const ID: &str = "blatt-19-fail";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Solar catalogue with incorrect record count", "Solarkatalog mit falscher Datensatzanzahl")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/blatt-19-fail/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🎩️example/🦀️.rs"]
mod example;
