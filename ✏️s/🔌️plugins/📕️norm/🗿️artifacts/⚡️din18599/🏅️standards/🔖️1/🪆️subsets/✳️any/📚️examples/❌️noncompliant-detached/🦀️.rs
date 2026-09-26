//! 📚️ Example `noncompliant-detached`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "noncompliant-detached";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Non-Compliant Detached House", "Nicht konformes freistehendes Wohnhaus")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/❌️noncompliant-detached/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

//#region 🪢️TaxonomyMounts
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
