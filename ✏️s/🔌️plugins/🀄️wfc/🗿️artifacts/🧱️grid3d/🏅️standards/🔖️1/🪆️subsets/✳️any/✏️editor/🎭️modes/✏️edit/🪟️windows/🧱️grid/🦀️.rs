//! 🧱️ `s.wfc.grid3d` editor — the GRID window: one translucent box instance per cell, placed at the
//! cumulative NON-UNIFORM offsets its own `cellSizesX/Y/Z` declare and scaled to its own box. Pinned
//! cells are drawn in their tile's colour, masked cells in a dark cage.
//!
//! 🖱️ A pick (framework `interactionSelect`) carries the cell key `x:y:z` as the instance id, and the
//! ARMED UTILITY decides what it means: `pin` pins the window's active tile, `mask` carves the cell
//! out, `select` only reports it. Nothing else in this window writes the document.

use crate::editor::grid3d::window::Grid3dWindowConfig;
use crate::schema::scene_internals;
use crate::Grid3dSnapshot;
use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::{
    world3d_camera_json, world3d_scene, world3d_selection_json, ActionDefinition, ActionKind, BuiltNode, LocalizedLabel, UiAssemblyResult, UtilityDefinition, WindowKindDefinition, WindowOptions, WorldSunConfig,
};
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid3d-grid";
pub const BODY_KEY: &str = "wfc.grid3d.grid";
pub const SURFACE_ID: &str = "wfc.grid3d.grid";
pub const UTILITY_SELECT: &str = "select";
pub const UTILITY_PIN: &str = "pin";
pub const UTILITY_MASK: &str = "mask";
pub const INTERACTION_DOMAIN: &str = "wfc.grid3d.cells";
pub const INTERACTION_GRANULARITY_CELL: &str = "cell";
pub const HOVER_CHANNEL: &str = "wfc.grid3d.cell";
//#endregion 🔖️Constants

//#region 🔖️Utilities
/// 🧰️ The three things a pick in this window can mean. No utility is armed until the host presses
/// one, and `select` is the inert default the framework falls back to.
pub fn utilities() -> Vec<UtilityDefinition> {
    vec![
        UtilityDefinition::new(UTILITY_SELECT, LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer"),
        UtilityDefinition::new(UTILITY_PIN, LocalizedLabel::native("Pin", "Anheften"), "lock"),
        UtilityDefinition::new(UTILITY_MASK, LocalizedLabel::native("Mask", "Ausblenden"), "square-dashed"),
    ]
}
//#endregion 🔖️Utilities

//#region 🕹️Interaction
/// 🕹️ The interaction domain this window's picks ride: ONE granularity, the cell, because a grid pick
/// can only ever mean a cell. A window kind that references a domain the app never declared is
/// refused outright by the plugin builder (`app-definition.invalid: … references undeclared
/// interaction …`), so the declaration and the reference live side by side in this file.
pub fn interaction() -> semio_framework_plugin::InteractionDefinition {
    semio_framework_plugin::InteractionDefinition {
        id: INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Cells", "Zellen"),
        granularities: vec![semio_framework_plugin::GranularityDefinition { id: INTERACTION_GRANULARITY_CELL.into(), label: LocalizedLabel::native("Cell", "Zelle"), icon_id: "box".into() }],
        hierarchy: semio_framework_plugin::HierarchyProvider::Topology,
        hover: semio_framework_plugin::HoverSpec { enabled: true, transitive: false, channels: vec![HOVER_CHANNEL.into()], broadcast: true },
        selection: semio_framework_plugin::SelectionSpec {
            modes: vec![semio_framework_plugin::SelectionMode::Multiple, semio_framework_plugin::SelectionMode::Single],
            methods: vec![semio_framework_plugin::SelectionMethod::Pick, semio_framework_plugin::SelectionMethod::Rectangle],
            merges: vec![semio_framework_plugin::MergeMode::Replace, semio_framework_plugin::MergeMode::Additive, semio_framework_plugin::MergeMode::Subtractive],
            transitive: false,
            broadcast: true,
        },
    }
}
//#endregion 🕹️Interaction

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::grid3d::create_grid3d_editor`.
pub fn definition() -> WindowKindDefinition {
    let mut definition = WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Grid", "Raster"),
        body_key: BODY_KEY.into(),
        surface_kind: semio_framework_plugin::SurfaceKind::World3d,
        icon_id: "grid-3x3".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: vec![UTILITY_SELECT.into(), UTILITY_PIN.into(), UTILITY_MASK.into()],
        interactions: vec![semio_framework_plugin::InteractionRef::new(INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    };
    definition.actions.extend([
        ActionDefinition::bounded_catalog("changeSeed", LocalizedLabel::native("Change Seed", "Seed ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("resizeGrid", LocalizedLabel::native("Resize Grid", "Raster ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("changeCellSizes", LocalizedLabel::native("Change Cell Sizes", "Zellgrößen ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("changePeriodicity", LocalizedLabel::native("Change Periodicity", "Periodizität ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("createTile", LocalizedLabel::native("Create Tile", "Kachel erstellen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("deleteTile", LocalizedLabel::native("Delete Tile", "Kachel löschen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("changeTileWeight", LocalizedLabel::native("Change Tile Weight", "Kachelgewicht ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("changeTileColor", LocalizedLabel::native("Change Tile Colour", "Kachelfarbe ändern"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("createRule", LocalizedLabel::native("Create Rule", "Regel erstellen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("deleteRule", LocalizedLabel::native("Delete Rule", "Regel löschen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("pinCell", LocalizedLabel::native("Pin Cell", "Zelle anheften"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("unpinCell", LocalizedLabel::native("Unpin Cell", "Zelle lösen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("maskCell", LocalizedLabel::native("Mask Cell", "Zelle ausblenden"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("unmaskCell", LocalizedLabel::native("Unmask Cell", "Zelle einblenden"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("setActiveTile", LocalizedLabel::native("Arm Tile", "Kachel wählen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("pickCell", LocalizedLabel::native("Pick Cell", "Zelle wählen"), ActionKind::Mutation),
        ActionDefinition::bounded_catalog("setCamera", LocalizedLabel::native("Set Camera", "Kamera setzen"), ActionKind::Mutation),
    ]);
    for action in &mut definition.actions {
        action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    }
    definition
}
//#endregion 🔖️Definition

//#region 📷️Camera
/// 📷️ How many grid spans the opening pose sits back from the grid's centre — the same framing seed
/// puzzle 3d uses, so a fresh document opens on a pose the user can orbit instead of one inside the
/// geometry.
const FRAMING_SPANS: f64 = 1.9;
const FRAMING_MINIMUM_DISTANCE: f64 = 2.5;

/// 📷️ The pose a pane opens on before any gesture: framed on what this document actually spans.
/// Idempotent — a pane the user (or a stored `WindowConfig`) already posed is left exactly as it is.
pub fn framed_camera(document: &Grid3dSnapshot, config: &Grid3dWindowConfig) -> ([f64; 3], [f64; 3]) {
    if !config.camera_unset() {
        return (config.position(), config.target());
    }
    let extent = scene_internals::grid_extent(document);
    let target = [extent[0] * 0.5, extent[1] * 0.5, extent[2] * 0.5];
    let span = extent[0].max(extent[1]).max(extent[2]).max(1.0);
    let distance = (span * FRAMING_SPANS).max(FRAMING_MINIMUM_DISTANCE);
    ([target[0] + distance, target[1] - distance, target[2] + distance * 0.75], target)
}
//#endregion 📷️Camera

//#region 🔖️Render
/// 🧱️ One instanced `World3d` scene over the whole cell grid. `selection_json` reports the armed
/// utility's current cell so the host paints it without the document ever storing hover state.
pub fn render(document: &Grid3dSnapshot, config: &Grid3dWindowConfig, selected: &[String], hovered: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let (position, target) = framed_camera(document, config);
    let scene = semio_framework_ui::wgpu::World3dScene {
        domain_id: Some(INTERACTION_DOMAIN.into()),
        ..world3d_scene(
            world3d_camera_json(position, target, 45.0),
            scene_internals::grid_meshes_json(document),
            scene_internals::grid_instances_json(document),
            world3d_selection_json("interactionSelect", selected, hovered),
            &WorldSunConfig::default(),
        )
    };
    semio_framework_plugin::scene_surface(SURFACE_ID, SurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
