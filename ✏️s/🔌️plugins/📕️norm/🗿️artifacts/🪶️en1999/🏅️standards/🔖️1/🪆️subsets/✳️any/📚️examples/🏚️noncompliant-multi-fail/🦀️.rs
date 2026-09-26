//! 🏚️ Non-compliant multi-fail EN 1999 subject.

use crate::En1999Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "noncompliant-multi-fail";

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Non-compliant aluminium beam", "Nicht nachweisbarer Aluminiumträger")
}

pub const ICON: &str = "file";

/// 📄 Bundled hierarchical DSL for the multi-fail aluminium subject.
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏚️noncompliant-multi-fail/🏚️noncompliant-multi-fail/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

pub fn snapshot() -> En1999Snapshot {
    En1999Snapshot::noncompliant_multi_fail()
}
