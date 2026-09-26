//! 📚️ Example `multi-fail-noncompliant`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "multi-fail-noncompliant";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Multi-Fail Noncompliant", "Mehrfachversagen nicht konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/⚠️multi-fail-noncompliant/⚠️multi-fail-noncompliant/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
