//! 📚️ Example `noncompliant-office`.
use crate::Din16798Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "noncompliant-office";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Non-compliant office", "Nicht konformes Büro")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/⚠️noncompliant-office/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
pub fn snapshot() -> Din16798Snapshot {
    Din16798Snapshot::noncompliant_office()
}
