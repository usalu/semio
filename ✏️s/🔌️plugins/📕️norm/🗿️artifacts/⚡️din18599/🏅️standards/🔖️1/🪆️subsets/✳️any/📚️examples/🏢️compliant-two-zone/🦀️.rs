//! 📚️ Example `compliant-two-zone`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "compliant-two-zone";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Compliant Two-Zone House", "Konformes Zwei-Zonen-Haus")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️compliant-two-zone/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

//#region 🔢️TaxonomyMounts
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🔢️TaxonomyMounts
