//! 📚️ Example `compliant-detached`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "compliant-detached";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Compliant Detached House", "Konformes freistehendes Wohnhaus")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/✅️compliant-detached/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

//#region 🪢️TaxonomyMounts
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
