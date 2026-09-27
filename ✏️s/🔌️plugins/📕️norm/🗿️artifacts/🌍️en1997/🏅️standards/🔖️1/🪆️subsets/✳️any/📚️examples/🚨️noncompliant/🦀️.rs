//! 📚️ Example `noncompliant`.
use semio_framework_plugin::{ExampleSource, LocalizedLabel};
pub const ID: &str = "noncompliant";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Non-compliant project", "Nicht konformes Projekt")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🚨noncompliant/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
