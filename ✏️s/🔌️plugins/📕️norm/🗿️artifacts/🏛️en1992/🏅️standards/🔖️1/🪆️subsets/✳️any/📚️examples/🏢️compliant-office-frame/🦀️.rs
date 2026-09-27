//! 📚️ Example `compliant-office-frame`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "compliant-office-frame";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Compliant Office Frame", "Konformes Bürotragwerk")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢compliant-office-frame/🏢compliant-office-frame/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
