//! 📚️ Example `rectangle-wire-preview`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "rectangle-wire-preview";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
