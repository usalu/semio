//! 🏠️ Aluminium roof purlin example — compliant EN 1999 subject.

use crate::En1999Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "aluminium-roof-purlin";

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Aluminium roof purlin", "Aluminium-Dachpfette")
}

pub const ICON: &str = "file";

pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio");

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

pub fn snapshot() -> En1999Snapshot {
    En1999Snapshot::compliant_roof_purlin()
}
