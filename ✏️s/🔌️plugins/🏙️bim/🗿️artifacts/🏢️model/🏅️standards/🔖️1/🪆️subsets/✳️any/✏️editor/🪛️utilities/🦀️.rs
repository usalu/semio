//! 🪛️ BIM utility registry: the tools a window arms, one row each. A utility is host-owned window state, never a document operation. A row names its icon, its ribbon group, the
//! window kinds that accept it and, for the tools with a hotkey, the key and the arming command (`arm-utility`) the keybinding table binds. Adding a tool is one row here and, for a
//! gesture, one arm of `crate::editor::bim::gestures::build_tool`.

use crate::editor::bim::entities::LabelOf;
use crate::editor::bim::modes::edit::windows::{plan, section, sheet, world};
use crate::editor::bim::terminology::BimLabels;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::UtilityRef;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Registry
/// 🪛️ One utility of the registry.
pub struct UtilityRow {
    pub id: &'static str,
    pub label: LabelOf,
    pub icon: &'static str,
    pub group: Option<&'static str>,
    pub windows: &'static [&'static str],
    pub keys: Option<&'static str>,
    pub arm: Option<&'static str>,
}

/// 🪛️ The utility armed in a fresh window.
pub const DEFAULT_UTILITY: &str = "select";

const PLAN_WORLD: &[&str] = &[plan::WINDOW_KIND_ID, world::WINDOW_KIND_ID];
const PLAN_ONLY: &[&str] = &[plan::WINDOW_KIND_ID];
const EVERYWHERE: &[&str] = &[plan::WINDOW_KIND_ID, world::WINDOW_KIND_ID, section::WINDOW_KIND_ID, sheet::WINDOW_KIND_ID];
const SHEET_ONLY: &[&str] = &[sheet::WINDOW_KIND_ID];

macro_rules! utilities {
    ($( $id:literal, $label:ident, $icon:literal, $group:literal, $windows:expr, $keys:expr, $arm:expr; )+) => {
        /// 🪛️ Every utility, in ribbon order; each one's label is a field of `BimLabels`.
        pub static UTILITIES: &[UtilityRow] = &[$( UtilityRow { id: $id, label: |labels| labels.$label, icon: $icon, group: Some($group), windows: $windows, keys: $keys, arm: $arm } ),+];
    };
}

utilities! {
    "select", utility_select, "mouse-pointer", "select", EVERYWHERE, Some("v"), Some("armSelect");
    "move", utility_move, "move", "select", PLAN_WORLD, Some("e"), Some("armMove");
    "rotate", utility_rotate, "rotate-ccw", "select", PLAN_WORLD, Some("q"), Some("armRotate");
    "wall", utility_wall, "brick-wall", "structure", PLAN_WORLD, Some("w"), Some("armWall");
    "wall-arc", utility_wall_arc, "spline", "structure", PLAN_WORLD, Some("a"), Some("armWallArc");
    "curtain-wall", utility_curtain_wall, "panels-top-left", "structure", PLAN_WORLD, Some("u"), Some("armCurtainWall");
    "column", utility_column, "columns", "structure", PLAN_WORLD, Some("c"), Some("armColumn");
    "beam", utility_beam, "minus", "structure", PLAN_WORLD, Some("b"), Some("armBeam");
    "slab", utility_slab, "layout-panel-top", "structure", PLAN_WORLD, Some("s"), Some("armSlab");
    "slab-walls", utility_slab_walls, "scan-line", "structure", PLAN_WORLD, Some("shift+s"), Some("armSlabWalls");
    "ceiling", utility_ceiling, "panel-top", "structure", PLAN_WORLD, Some("i"), Some("armCeiling");
    "ceiling-space", utility_ceiling_space, "scan-line", "structure", PLAN_WORLD, Some("shift+i"), Some("armCeilingSpace");
    "roof", utility_roof, "house", "structure", PLAN_WORLD, Some("r"), Some("armRoof");
    "window", utility_window, "app-window", "openings", PLAN_WORLD, Some("n"), Some("armWindow");
    "door", utility_door, "door-open", "openings", PLAN_WORLD, Some("d"), Some("armDoor");
    "opening", utility_opening, "square-dashed", "openings", PLAN_WORLD, Some("o"), Some("armOpening");
    "stair", utility_stair, "footprints", "circulation", PLAN_WORLD, Some("t"), Some("armStair");
    "railing", utility_railing, "fence", "circulation", PLAN_WORLD, Some("l"), Some("armRailing");
    "ramp", utility_ramp, "trending-up", "circulation", PLAN_WORLD, Some("shift+t"), Some("armRamp");
    "space", utility_space, "square-dashed", "spaces", PLAN_WORLD, Some("p"), Some("armSpace");
    "grid", utility_grid, "grid", "spaces", PLAN_WORLD, Some("g"), Some("armGrid");
    "measure", utility_measure, "ruler", "spaces", PLAN_WORLD, Some("m"), Some("armMeasure");
    "split-wall", utility_split_wall, "scissors", "structure", PLAN_WORLD, Some("shift+w"), Some("armSplitWall");
    "copy", utility_copy, "copy", "modify", PLAN_WORLD, Some("k"), Some("armCopy");
    "mirror", utility_mirror, "flip-horizontal-2", "modify", PLAN_WORLD, Some("shift+k"), Some("armMirror");
    "array", utility_array, "grid-3x3", "modify", PLAN_WORLD, Some("y"), Some("armArray");
    "array-radial", utility_array_radial, "orbit", "modify", PLAN_WORLD, Some("shift+y"), Some("armArrayRadial");
    "offset", utility_offset, "move-horizontal", "modify", PLAN_WORLD, Some("f"), Some("armOffset");
    "trim", utility_trim, "scissors-line-dashed", "modify", PLAN_WORLD, Some("x"), Some("armTrim");
    "extend", utility_extend, "move-diagonal", "modify", PLAN_WORLD, Some("shift+x"), Some("armExtend");
    "align", utility_align, "align-start-vertical", "modify", PLAN_WORLD, Some("z"), Some("armAlign");
    "split", utility_split, "split", "modify", PLAN_WORLD, Some("shift+z"), Some("armSplit");
    "dimension", utility_dimension, "move-horizontal", "annotate", PLAN_ONLY, Some("shift+d"), Some("armDimension");
    "tag", utility_tag, "tag", "annotate", PLAN_ONLY, Some("shift+g"), Some("armTag");
    "text-note", utility_text_note, "type", "annotate", PLAN_ONLY, Some("shift+n"), Some("armTextNote");
    "leader", utility_leader, "arrow-up-right", "annotate", PLAN_ONLY, Some("shift+l"), Some("armLeader");
    "viewport", utility_viewport, "frame", "sheet", SHEET_ONLY, Some("shift+v"), Some("armViewport");
}
//#endregion 🔖️Registry

//#region 🔖️Projection
/// 🧱️ The manifest definitions of every utility.
pub fn definitions() -> Vec<UtilityDefinition> {
    UTILITIES
        .iter()
        .map(|row| UtilityDefinition { group: row.group.map(Into::into), keys: row.keys.map(Into::into), ..UtilityDefinition::new(row.id, LocalizedLabel::native((row.label)(&BimLabels::NATIVE_EN).as_str(), (row.label)(&BimLabels::NATIVE_DE).as_str()), row.icon) })
        .collect()
}

/// ⌨️ The `(keys, arming command)` pair of every utility with a hotkey: a lower-case letter, or `shift+` and one for the variant of a tool.
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
