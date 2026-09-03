//! 📚️ Example `face-sweep-extrude`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "face-sweep-extrude";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Face Sweep Extrude", "Face Sweep Extrude")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🧪️face-sweep-extrude/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
