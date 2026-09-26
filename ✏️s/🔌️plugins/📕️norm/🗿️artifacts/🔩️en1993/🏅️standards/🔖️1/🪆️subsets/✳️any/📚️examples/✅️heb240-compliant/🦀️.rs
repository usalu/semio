//! 📚️ Example `heb240-compliant` — realistic compliant HEB 240 S355 frame.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "heb240-compliant";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("HEB 240 Compliant Frame", "HEB 240 konformer Rahmen")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/✅️heb240-compliant/✅️heb240-compliant/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
