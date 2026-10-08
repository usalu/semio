//! 📚️ Example `demo`: one site, one building, two storeys, four walls of a two-layer wall type.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo House", "Demo-Haus")
}
pub const ICON: &str = "building";
pub const ASSET_DIR: &str = "🎬️demo";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio");
pub const SNAPSHOT_JSON: &str = include_str!("../../🖼️assets/🎬️demo/📸️snapshot.json");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
