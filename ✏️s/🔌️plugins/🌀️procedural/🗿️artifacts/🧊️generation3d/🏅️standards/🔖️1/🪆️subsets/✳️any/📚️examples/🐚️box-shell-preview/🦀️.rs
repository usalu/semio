//! 📚️ Example `box-shell-preview`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "box-shell-preview";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
