//! 📚️ Example `rectangle-extrude-volume`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "rectangle-extrude-volume";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
