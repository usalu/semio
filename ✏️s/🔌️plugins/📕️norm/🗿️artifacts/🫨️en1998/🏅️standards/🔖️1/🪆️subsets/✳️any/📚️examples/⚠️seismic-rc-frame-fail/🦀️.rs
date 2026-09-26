//! ⚠️ Non-compliant DE office RC frame example.

use crate::En1998Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "seismic-rc-frame-fail";
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = "";

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Seismic RC frame (DE zone 2, failing)", "Erdbeben-Stahlbetonrahmen (DE Zone 2, nicht konform)")
}

pub fn snapshot() -> En1998Snapshot {
    En1998Snapshot::noncompliant_de_office()
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
