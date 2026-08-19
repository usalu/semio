//! 📚️ Example `high-strength-connection`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "high-strength-connection";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("High Strength Connection", "High Strength Connection")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️high-strength-connection.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
