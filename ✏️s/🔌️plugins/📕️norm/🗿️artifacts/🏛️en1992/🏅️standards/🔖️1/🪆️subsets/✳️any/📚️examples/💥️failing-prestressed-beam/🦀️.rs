//! 📚️ Example `failing-prestressed-beam`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "failing-prestressed-beam";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Failing Prestressed Beam", "Versagender vorgespannter Träger")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/💥failing-prestressed-beam/💥failing-prestressed-beam/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
