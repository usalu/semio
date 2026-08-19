//! 📚️ Example `composite-bridge-girder`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "composite-bridge-girder";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Composite Bridge Girder", "Composite Bridge Girder")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️composite-bridge-girder.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
