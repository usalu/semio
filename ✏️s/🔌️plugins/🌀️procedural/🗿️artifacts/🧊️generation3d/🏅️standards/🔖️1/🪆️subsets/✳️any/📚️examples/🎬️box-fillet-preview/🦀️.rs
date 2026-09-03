//! 📚️ Example `box-fillet-preview`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "box-fillet-preview";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🧪️box-fillet-preview/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
