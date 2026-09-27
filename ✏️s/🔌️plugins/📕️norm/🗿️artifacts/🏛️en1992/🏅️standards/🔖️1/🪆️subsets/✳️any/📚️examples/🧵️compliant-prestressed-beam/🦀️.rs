//! 📚️ Example `compliant-prestressed-beam`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "compliant-prestressed-beam";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Compliant Prestressed Beam", "Konformer vorgespannter Träger")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🧵compliant-prestressed-beam/🧵compliant-prestressed-beam/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
