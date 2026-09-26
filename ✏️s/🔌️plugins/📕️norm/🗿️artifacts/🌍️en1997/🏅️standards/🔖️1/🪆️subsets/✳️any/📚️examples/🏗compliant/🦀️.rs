//! 📚️ Example `compliant`.
use semio_framework_plugin::{ExampleSource, LocalizedLabel};
pub const ID: &str = "compliant";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Compliant project", "Konformes Projekt")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏗compliant/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
