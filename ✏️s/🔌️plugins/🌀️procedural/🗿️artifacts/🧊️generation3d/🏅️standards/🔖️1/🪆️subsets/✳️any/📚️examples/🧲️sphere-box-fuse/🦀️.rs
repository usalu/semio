//! 📚️ Example `sphere-box-fuse`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "sphere-box-fuse";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🧪️sphere-box-fuse/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
