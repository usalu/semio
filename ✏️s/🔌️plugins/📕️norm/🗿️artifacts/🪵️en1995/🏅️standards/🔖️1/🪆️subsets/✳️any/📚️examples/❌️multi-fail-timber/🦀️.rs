//! 📚️ Non-compliant multi-fail timber example.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "multi-fail-timber";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Non-Compliant Multi-Fail Timber", "Nicht nachweisbarer Mehrfachversagen-Holzbau")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/❌️multi-fail-timber/❌️multi-fail-timber/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
