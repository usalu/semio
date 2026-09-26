//! 📚️ Example `cooled-office`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "cooled-office";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Cooled Office Building", "Gekühltes Bürogebäude")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/❄️cooled-office/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

//#region 🔢️TaxonomyMounts
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🔢️TaxonomyMounts
