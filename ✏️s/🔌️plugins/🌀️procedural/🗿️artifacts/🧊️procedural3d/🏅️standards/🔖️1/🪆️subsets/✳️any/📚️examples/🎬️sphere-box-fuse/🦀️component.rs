//! 📚️ Example `sphere-box-fuse`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "sphere-box-fuse";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Sphere Box Fuse", "Sphere Box Fuse")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️sphere-box-fuse.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
