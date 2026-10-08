//! 🪛️ BIM utility registry: the tools a window arms, one row each. A utility is host-owned window state, never a document operation. A row names its icon, its ribbon group, the
//! window kinds that accept it and, for the tools with a hotkey, the key and the arming command (`arm-utility`) the keybinding table binds. Adding a tool is one row here and, for a
//! gesture, one arm of `crate::editor::bim::gestures::build_tool`.

use crate::editor::bim::modes::edit::windows::{plan, section, world};
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::UtilityRef;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Registry
/// 🪛️ One utility of the registry.
pub struct UtilityRow {
    pub id: &'static str,
    pub label: (&'static str, &'static str),
    pub icon: &'static str,
    pub group: Option<&'static str>,
    pub windows: &'static [&'static str],
    pub keys: Option<&'static str>,
    pub arm: Option<&'static str>,
}

/// 🪛️ The utility armed in a fresh window.
pub const DEFAULT_UTILITY: &str = "select";

const PLAN_WORLD: &[&str] = &[plan::WINDOW_KIND_ID, world::WINDOW_KIND_ID];
const EVERYWHERE: &[&str] = &[plan::WINDOW_KIND_ID, world::WINDOW_KIND_ID, section::WINDOW_KIND_ID];

macro_rules! utilities {
    ($( $id:literal, ($en:literal, $de:literal), $icon:literal, $group:literal, $windows:expr, $keys:expr, $arm:expr; )+) => {
        /// 🪛️ Every utility, in ribbon order.
        pub static UTILITIES: &[UtilityRow] = &[$( UtilityRow { id: $id, label: ($en, $de), icon: $icon, group: Some($group), windows: $windows, keys: $keys, arm: $arm } ),+];
    };
}

utilities! {
    "select", ("Select", "Auswählen"), "mouse-pointer", "select", EVERYWHERE, Some("v"), Some("armSelect");
    "move", ("Move", "Verschieben"), "move", "select", PLAN_WORLD, None, None;
    "rotate", ("Rotate", "Drehen"), "rotate-ccw", "select", PLAN_WORLD, None, None;
    "wall", ("Wall", "Wand"), "brick-wall", "structure", PLAN_WORLD, Some("w"), Some("armWall");
    "wall-arc", ("Arc wall", "Bogenwand"), "spline", "structure", PLAN_WORLD, Some("a"), Some("armWallArc");
    "curtain-wall", ("Curtain wall", "Vorhangfassade"), "panels-top-left", "structure", PLAN_WORLD, Some("u"), Some("armCurtainWall");
    "column", ("Column", "Stütze"), "columns", "structure", PLAN_WORLD, Some("c"), Some("armColumn");
    "beam", ("Beam", "Träger"), "minus", "structure", PLAN_WORLD, Some("b"), Some("armBeam");
    "slab", ("Slab", "Decke"), "layout-panel-top", "structure", PLAN_WORLD, Some("s"), Some("armSlab");
    "slab-walls", ("Slab from walls", "Decke aus Wänden"), "scan-line", "structure", PLAN_WORLD, None, None;
    "roof", ("Roof", "Dach"), "house", "structure", PLAN_WORLD, Some("r"), Some("armRoof");
    "window", ("Window", "Fenster"), "app-window", "openings", PLAN_WORLD, Some("n"), Some("armWindow");
    "door", ("Door", "Tür"), "door-open", "openings", PLAN_WORLD, Some("d"), Some("armDoor");
    "opening", ("Opening", "Öffnung"), "square-dashed", "openings", PLAN_WORLD, Some("o"), Some("armOpening");
    "stair", ("Stair", "Treppe"), "footprints", "circulation", PLAN_WORLD, Some("t"), Some("armStair");
    "railing", ("Railing", "Geländer"), "fence", "circulation", PLAN_WORLD, Some("l"), Some("armRailing");
    "space", ("Space", "Raum"), "square-dashed", "spaces", PLAN_WORLD, Some("p"), Some("armSpace");
    "grid", ("Grid line", "Rasterlinie"), "grid", "spaces", PLAN_WORLD, Some("g"), Some("armGrid");
    "measure", ("Measure", "Messen"), "ruler", "spaces", PLAN_WORLD, Some("m"), Some("armMeasure");
}
//#endregion 🔖️Registry

//#region 🔖️Projection
/// 🧱️ The manifest definitions of every utility.
pub fn definitions() -> Vec<UtilityDefinition> {
    UTILITIES
        .iter()
        .map(|row| UtilityDefinition { group: row.group.map(Into::into), keys: row.keys.map(Into::into), ..UtilityDefinition::new(row.id, LocalizedLabel::native(row.label.0, row.label.1), row.icon) })
        .collect()
}

/// ⌨️ The `(keys, arming command)` pair of every utility with a hotkey.
pub fn keybindings() -> Vec<(&'static str, &'static str)> {
    UTILITIES.iter().filter_map(|row| row.keys.zip(row.arm)).collect()
}

/// 🪟️ The utilities one window kind accepts.
pub fn for_window(window: &str) -> Vec<UtilityRef> {
    UTILITIES.iter().filter(|row| row.windows.contains(&window)).map(|row| UtilityRef::from(row.id)).collect()
}

/// 🌅️ The first arm of a window: the default utility when the window accepts it.
pub fn initial() -> Option<UtilityRef> {
    Some(UtilityRef::from(DEFAULT_UTILITY))
}

/// 🪛️ The utility a view has armed, else the default.
pub fn active(view_state: &semio_framework_plugin::ViewModel) -> &str {
    view_state.active_utility_id.as_deref().unwrap_or(DEFAULT_UTILITY)
}

/// 🖱️ Whether a utility starts a drawing session that needs the 3D window's ground picks (everything but plain selection).
pub fn draws(utility: &str) -> bool {
    utility != DEFAULT_UTILITY
}
//#endregion 🔖️Projection

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
