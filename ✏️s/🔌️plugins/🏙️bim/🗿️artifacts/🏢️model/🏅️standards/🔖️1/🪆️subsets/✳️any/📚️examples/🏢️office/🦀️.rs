//! 🏢️ Example `office`: a four-storey office on a six metre grid with columns, beams, cores, stairs and a curtain wall.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "office";
pub const ASSET_DIR: &str = "🏢️office";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Office Building", "Bürogebäude")
}
pub const ICON: &str = "building-2";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️office/🗣️.dsl.semio");
pub const SNAPSHOT_JSON: &str = include_str!("../../🖼️assets/🏢️office/📸️snapshot.json");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
