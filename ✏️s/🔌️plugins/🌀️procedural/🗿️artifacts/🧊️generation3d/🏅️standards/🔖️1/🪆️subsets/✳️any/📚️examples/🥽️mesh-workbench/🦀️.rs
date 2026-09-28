//! 🥽️ Mesh creation, face editing, measurement, and faceted preview in one graph.
use semio_framework_plugin::{ExampleSource, LocalizedLabel};
pub const ID: &str = "mesh-workbench";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Mesh Workbench", "Netzwerkstatt") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🥽️mesh-workbench/🗣️.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }
