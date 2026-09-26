//! ⚠️ Non-compliant DE multi-part seismic example (EN 1998-1…6).

use crate::En1998Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "seismic-multipart-fail";
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = "";

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native(
        "Seismic multi-part (DE, failing)",
        "Erdbeben Mehrteil (DE, nicht konform)",
    )
}

pub fn snapshot() -> En1998Snapshot {
    En1998Snapshot::noncompliant_de_multipart()
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
