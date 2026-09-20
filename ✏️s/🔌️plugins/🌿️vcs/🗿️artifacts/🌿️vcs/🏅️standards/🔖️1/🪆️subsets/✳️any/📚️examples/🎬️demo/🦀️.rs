//! 📚️ Example `demo`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🧬️ The document this example IS, decoded from its own asset so the DSL file stays the single
/// source of truth. `setActiveExample` packs this into the `Effect::LoadDocument` it emits.
pub fn snapshot() -> crate::VcsSnapshot {
    <crate::VcsSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).unwrap_or_default()
}

//#region 🪢️TaxonomyMounts
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
