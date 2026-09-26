//! ❌️ Non-compliant multi-failure masonry wall example.

use crate::En1996Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "multi-fail-masonry";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Multi-fail Masonry Wall", "Mauerwerkswand mit mehreren Versagen")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/❌️multi-fail-masonry/❌️multi-fail-masonry/🗣️.dsl.semio");

pub fn snapshot() -> En1996Snapshot {
    En1996Snapshot::noncompliant_multi_fail()
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
