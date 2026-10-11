//! 🪟️ `sizes`: the size an authored opening resolves to, the override-else-type rule of the opening record over the window or door type it names.

use crate::{ModelSnapshot, Opening, OpeningKind};

/// 📐️ Size and sill of an opening after the override-else-type rule.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetainedClone)]
pub struct Resolved {
    pub width: f64,
    pub height: f64,
    pub sill: f64,
    pub type_found: bool,
}

/// 📐️ Width, height and sill: an explicit width or height wins over the type; the sill override replaces the sill of the type (a window defaults to its type sill, a door or void to zero).
pub fn resolve_size(snapshot: &ModelSnapshot, opening: &Opening) -> Resolved {
    let (kind_width, kind_height, kind_sill, type_found) = match &opening.kind {
        OpeningKind::Window { window_type } => snapshot.window_types.get(window_type).map_or((0.0, 0.0, 0.0, false), |kind| (kind.width, kind.height, kind.sill, true)),
        OpeningKind::Door { door_type } => snapshot.door_types.get(door_type).map_or((0.0, 0.0, 0.0, false), |kind| (kind.width, kind.height, 0.0, true)),
        OpeningKind::Void { width, height } => (*width, *height, 0.0, true),
    };
    Resolved { width: opening.width.unwrap_or(kind_width), height: opening.height.unwrap_or(kind_height), sill: opening.sill_override.unwrap_or(kind_sill), type_found }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
