//! 🖱️ Blueprint utility — Select: the default pointer. Click and shift-click picking stay on the
//! canvas pointer commands; this utility does not arm the transform gumball.

use semio_framework_plugin::{LocalizedLabel, UtilityCategory, UtilityDefinition};

pub const UTILITY_ID: &str = "select";

/// 🧱️ Stitched into the blueprint window and the layout app manifest.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Selection), allows_actions_while_active: true, ..UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer") }
}
