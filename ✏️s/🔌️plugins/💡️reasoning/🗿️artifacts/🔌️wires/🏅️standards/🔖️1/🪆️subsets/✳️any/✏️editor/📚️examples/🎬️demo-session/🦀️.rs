//! 📚️ Example `demo-session`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo-session";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo Session", "Demo-Sitzung")
}
pub const ICON: &str = "play";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
