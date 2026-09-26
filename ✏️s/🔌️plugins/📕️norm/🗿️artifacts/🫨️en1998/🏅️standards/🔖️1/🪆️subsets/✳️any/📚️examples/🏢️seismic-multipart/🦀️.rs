//! 🏢️ Compliant DE multi-part seismic example (EN 1998-1…6).

use crate::En1998Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "seismic-multipart";
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️seismic-multipart/🏢️seismic-multipart/🗣️.dsl.semio");

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native(
        "Seismic multi-part (DE, compliant)",
        "Erdbeben Mehrteil (DE, konform)",
    )
}

pub fn snapshot() -> En1998Snapshot {
    En1998Snapshot::compliant_de_multipart()
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
