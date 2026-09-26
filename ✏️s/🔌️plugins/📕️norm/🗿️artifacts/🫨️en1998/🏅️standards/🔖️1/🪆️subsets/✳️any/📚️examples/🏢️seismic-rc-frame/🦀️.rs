//! 🏢️ Compliant DE office RC frame example.

use crate::En1998Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "seismic-rc-frame";
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio");

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Seismic RC frame (DE zone 2, compliant)", "Erdbeben-Stahlbetonrahmen (DE Zone 2, konform)")
}

pub fn snapshot() -> En1998Snapshot {
    En1998Snapshot::compliant_de_office()
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
