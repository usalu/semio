//! 📚️ Example `glulam-footbridge`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "glulam-footbridge";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Glulam Footbridge", "Glulam Footbridge")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️glulam-footbridge.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
