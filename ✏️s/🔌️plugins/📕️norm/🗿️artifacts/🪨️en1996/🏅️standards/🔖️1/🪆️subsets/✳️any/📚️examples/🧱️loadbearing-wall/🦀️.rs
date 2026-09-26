//! 🧱 Compliant clay load-bearing wall example (DIN EN 1996 DE-NA).

use crate::En1996Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "loadbearing-wall";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Load-bearing Clay Wall", "Tragende Ziegelwand")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio");

pub fn snapshot() -> En1996Snapshot {
    En1996Snapshot::compliant_clay_wall()
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
