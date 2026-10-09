//! 🏡️ Example `house`: a detached house with basement, ground, upper and attic storeys, a bay wall, a gable roof and a U-stair.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;

pub const ID: &str = "house";
pub const ASSET_DIR: &str = "🏡️house";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Family House", "Einfamilienhaus")
}
pub const ICON: &str = "house";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏡️house/🗣️.dsl.semio");
pub const SNAPSHOT_JSON: &str = include_str!("../../🖼️assets/🏡️house/📸️snapshot.json");
pub const DERIVATIONS: &str = include_str!("../../🖼️assets/🏡️house/🧬️derivations.json");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
