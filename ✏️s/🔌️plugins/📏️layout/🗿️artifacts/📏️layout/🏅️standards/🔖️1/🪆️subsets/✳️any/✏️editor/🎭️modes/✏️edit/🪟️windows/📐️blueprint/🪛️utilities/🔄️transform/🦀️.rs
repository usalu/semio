//! 🔄️ Blueprint utility — Transform: arms the world-space gumball on the selected frames.

use semio_framework_plugin::{LocalizedLabel, UtilityDefinition};

pub const UTILITY_ID: &str = "transform";

/// 🎛️ Handle set the overlay draws. Every axis stays on; the host gumball reads the same flags
/// from the `meta:gumball` layer.
pub struct TransformUtilityOptions {
    pub move_axes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

/// 🧱️ Stitched into the blueprint window and the layout app manifest.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition { allows_actions_while_active: true, ..UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Transform", "Transformieren"), "move") }
}

/// 🎛️ Move, rotate, axis scale, and uniform scale are all armed.
pub fn options() -> TransformUtilityOptions {
    TransformUtilityOptions { move_axes: true, rotate: true, scale_axes: true, scale_uniform: true }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
