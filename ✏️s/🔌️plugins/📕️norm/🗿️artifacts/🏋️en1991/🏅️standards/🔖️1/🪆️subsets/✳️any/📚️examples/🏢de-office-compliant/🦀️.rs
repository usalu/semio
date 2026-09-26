//! 📚️ Example `de-office-compliant`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "de-office-compliant";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("DE Office Compliant", "DE Büro konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢de-office-compliant/🏢de-office-compliant/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
