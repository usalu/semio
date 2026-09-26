//! 📚️ Example `residential-method3`.
use crate::Din16798Snapshot;
use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "residential-method3";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Residential method 3", "Wohnen Methode 3")
}
pub const ICON: &str = "home";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏠residential-method3/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
pub fn snapshot() -> Din16798Snapshot {
    Din16798Snapshot::residential_method3()
}
