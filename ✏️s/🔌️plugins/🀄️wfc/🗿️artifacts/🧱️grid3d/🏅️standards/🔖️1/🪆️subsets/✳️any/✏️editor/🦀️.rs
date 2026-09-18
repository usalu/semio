//! ✏️ `s.wfc.grid3d` editor — two `World3d` windows over one document: the GRID (the persisted
//! problem, one box instance per cell) and the PREVIEW (the inferred assignment). Every document
//! verb maps 1:1 onto a real `Grid3dMutation` builder from the schema tree — no synthetic
//! "set field" indirection, since this domain's own mutations are already exactly that granular.
//!
//! 🖱️ `pickCell` is the one verb whose meaning depends on state: it reads the ARMED UTILITY of the
//! window it was dispatched in (`active_utility_by_window_id` first, focused window second — the
//! `🗒️note` fallback chain) and turns one pick into `pin-cell`, `mask-cell`, or nothing at all.

use crate::editor::grid3d::modes::edit;
use crate::editor::grid3d::modes::edit::windows::{grid, preview};
use crate::editor::grid3d::window::{addressed_config, config_from_view, Grid3dWindowConfig};
use crate::mutations::{change_cell_sizes, change_periodicity, change_seed, change_tile_media, change_tile_weight, create_rule, create_tile, delete_rule, delete_tile, mask_cell, pin_cell, resize_grid, unmask_cell, unpin_cell};
use crate::schema::snapshot::{tile_index, Grid3dAxis, Grid3dCell, Grid3dColor, Grid3dDirection, Grid3dMesh, Grid3dPinnedCell, Grid3dRule, Grid3dTile, Grid3dTileMedia};
use crate::{Grid3dMutation, Grid3dSnapshot, WFC_GRID3D_DIALECT, WFC_GRID3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per document verb, plus the three the surface
/// itself owns: `pickCell` (utility-dependent), `setActiveTile` and `setCamera`, which write the
/// per-window config rather than the document.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Grid3dEditorCommand {
    #[dsl(key = "changeSeed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "resizeGrid")]
    ResizeGrid { width: u32, height: u32, depth: u32 },
    #[dsl(key = "changeCellSizes")]
    ChangeCellSizes { axis: Grid3dAxis, sizes: Vec<f64> },
    #[dsl(key = "changePeriodicity")]
    ChangePeriodicity { periodic_x: bool, periodic_y: bool, periodic_z: bool },
    #[dsl(key = "createTile")]
    CreateTile { id: String, label: Option<String>, weight: f64, r: u32, g: u32, b: u32, a: u32 },
    #[dsl(key = "deleteTile")]
    DeleteTile { id: String },
    #[dsl(key = "changeTileWeight")]
    ChangeTileWeight { tile_id: String, weight: f64 },
    #[dsl(key = "changeTileColor")]
    ChangeTileColor { tile_id: String, r: u32, g: u32, b: u32, a: u32 },
    #[dsl(key = "createRule")]
    CreateRule { id: String, tile_a_id: String, tile_b_id: String, direction: Grid3dDirection, allowed: bool },
    #[dsl(key = "deleteRule")]
    DeleteRule { id: String },
    #[dsl(key = "pinCell")]
    PinCell { x: u32, y: u32, z: u32, tile_id: String },
    #[dsl(key = "unpinCell")]
    UnpinCell { x: u32, y: u32, z: u32 },
    #[dsl(key = "maskCell")]
    MaskCell { x: u32, y: u32, z: u32 },
    #[dsl(key = "unmaskCell")]
    UnmaskCell { x: u32, y: u32, z: u32 },
    #[dsl(key = "pickCell")]
    PickCell { cell_id: String },
    #[dsl(key = "setActiveTile")]
    SetActiveTile { tile_id: String },
    #[dsl(key = "setCamera")]
    SetCamera { x: f64, y: f64, z: f64, target_x: f64, target_y: f64, target_z: f64, zoom: f64 },
}

impl protocol::OpBinary for Grid3dEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command

//#region 🔖️Helpers
/// 🧰️ The utility armed for the window being dispatched: the React host arms utilities PER WINDOW
/// INSTANCE (`active_utility_by_window_id`) and mirrors only the shell's ACTIVE window into the flat
/// `active_utility_id`, so a pane that is not the active window would otherwise read `None` and run
/// every pick as a plain select (`🗒️note`'s precedent, ticket 26/09/17/NOTE-PLUGIN-END-TO-END).
pub fn grid3d_active_utility(view: Option<&semio_framework_plugin::ViewModel>) -> &str {
    let Some(view) = view else { return grid::UTILITY_SELECT };
    view.window_id
        .as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or(grid::UTILITY_SELECT)
}

/// 🧊️ Splits a `x:y:z` cell key back into coordinates. A key the grid never emitted answers `None`,
/// so a forged pick refuses instead of writing a cell outside the grid.
pub fn parse_cell_id(cell_id: &str) -> Option<(u32, u32, u32)> {
    let mut parts = cell_id.split(':');
    let x = parts.next()?.parse().ok()?;
    let y = parts.next()?.parse().ok()?;
    let z = parts.next()?.parse().ok()?;
    parts.next().is_none().then_some((x, y, z))
}

/// 🗿️ A unit-box tile in one colour — what `createTile` and `changeTileColor` author, since the
/// editor has no mesh-authoring affordance of its own (a real mesh arrives through
/// `change-tile-media` with a `MeshChild` handle).
fn box_media(color: Grid3dColor) -> Grid3dTileMedia {
    Grid3dTileMedia::Mesh { mesh: Grid3dMesh { positions: Vec::new(), indices: Vec::new(), color: Some(color) } }
}
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Grid3dEditor;

impl ArtifactEditor for Grid3dEditor {
    type Snapshot = Grid3dSnapshot;
    type Mutation = Grid3dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Grid3dEditorCommand;

    const DIALECT: Dialect = WFC_GRID3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_GRID3D_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Grid3dSnapshot {
        crate::examples::blocks::snapshot()
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        crate::editor::grid3d::window::register_config(registry)
    }

    /// ✏️ Dispatches straight onto the schema tree's own mutation builders. `setActiveTile` and
    /// `setCamera` never touch the document — they address the exact window instance's own config —
    /// and `pickCell` resolves to a real document verb only when a writing utility is armed.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let document = doc.snapshot;
        let window_config = config_from_view(cfg);
        let (mutation, description) = match command {
            Grid3dEditorCommand::ChangeSeed { seed } => (change_seed(*seed), format!("Change seed to {seed}")),
            Grid3dEditorCommand::ResizeGrid { width, height, depth } => (resize_grid(*width, *height, *depth), format!("Resize grid to {width}×{height}×{depth}")),
            Grid3dEditorCommand::ChangeCellSizes { axis, sizes } => (change_cell_sizes(*axis, sizes.clone()), format!("Change {} cell sizes", axis.label())),
            Grid3dEditorCommand::ChangePeriodicity { periodic_x, periodic_y, periodic_z } => (change_periodicity(*periodic_x, *periodic_y, *periodic_z), "Change periodicity".to_string()),
            Grid3dEditorCommand::CreateTile { id, label, weight, r, g, b, a } => {
                (create_tile(Grid3dTile { id: id.clone(), label: label.clone(), weight: *weight, media: box_media(Grid3dColor { r: *r, g: *g, b: *b, a: *a }) }), format!("Create tile {id}"))
            }
            Grid3dEditorCommand::DeleteTile { id } => (delete_tile(id.clone()), format!("Delete tile {id}")),
            Grid3dEditorCommand::ChangeTileWeight { tile_id, weight } => (change_tile_weight(tile_id.clone(), *weight), format!("Change weight of {tile_id}")),
            Grid3dEditorCommand::ChangeTileColor { tile_id, r, g, b, a } => {
                let color = Grid3dColor { r: *r, g: *g, b: *b, a: *a };
                let media = match tile_index(document, tile_id).map(|index| &document.tiles[index].media) {
                    Some(Grid3dTileMedia::Mesh { mesh }) => Grid3dTileMedia::Mesh { mesh: Grid3dMesh { positions: mesh.positions.clone(), indices: mesh.indices.clone(), color: Some(color) } },
                    _ => box_media(color),
                };
                (change_tile_media(tile_id.clone(), media), format!("Change colour of {tile_id}"))
            }
            Grid3dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, direction, allowed } => {
                (create_rule(Grid3dRule { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), direction: *direction, allowed: *allowed }), format!("Create rule {id}"))
            }
            Grid3dEditorCommand::DeleteRule { id } => (delete_rule(id.clone()), format!("Delete rule {id}")),
            Grid3dEditorCommand::PinCell { x, y, z, tile_id } => (pin_cell(Grid3dPinnedCell { x: *x, y: *y, z: *z, tile_id: tile_id.clone() }), format!("Pin cell {x}:{y}:{z}")),
            Grid3dEditorCommand::UnpinCell { x, y, z } => (unpin_cell(*x, *y, *z), format!("Unpin cell {x}:{y}:{z}")),
            Grid3dEditorCommand::MaskCell { x, y, z } => (mask_cell(Grid3dCell { x: *x, y: *y, z: *z }), format!("Mask cell {x}:{y}:{z}")),
            Grid3dEditorCommand::UnmaskCell { x, y, z } => (unmask_cell(*x, *y, *z), format!("Unmask cell {x}:{y}:{z}")),
            Grid3dEditorCommand::PickCell { cell_id } => {
                let Some((x, y, z)) = parse_cell_id(cell_id) else {
                    return Err(Fault::from(format!("wfc.grid3d.cell.unknown-cell '{cell_id}'")));
                };
                match grid3d_active_utility(view_state) {
                    grid::UTILITY_PIN => {
                        if window_config.active_tile_id.is_empty() {
                            return Err(Fault::from("wfc.grid3d.tile.no-armed-tile"));
                        }
                        (pin_cell(Grid3dPinnedCell { x, y, z, tile_id: window_config.active_tile_id }), format!("Pin cell {cell_id}"))
                    }
                    grid::UTILITY_MASK => (mask_cell(Grid3dCell { x, y, z }), format!("Mask cell {cell_id}")),
                    _ => return Ok(Emit::default()),
                }
            }
            Grid3dEditorCommand::SetActiveTile { tile_id } => {
                let view = view_state.ok_or_else(|| Fault::from("wfc-grid3d-window-required"))?;
                let mut next = window_config;
                next.active_tile_id.clone_from(tile_id);
                return Ok(Emit { window_config_mutations: vec![addressed_config(view, next)?], description: Some(format!("Arm tile {tile_id}")), ..Default::default() });
            }
            Grid3dEditorCommand::SetCamera { x, y, z, target_x, target_y, target_z, zoom } => {
                let view = view_state.ok_or_else(|| Fault::from("wfc-grid3d-window-required"))?;
                let mut next = window_config;
                next.camera_x = *x;
                next.camera_y = *y;
                next.camera_z = *z;
                next.target_x = *target_x;
                next.target_y = *target_y;
                next.target_z = *target_z;
                next.camera_zoom = *zoom;
                return Ok(Emit { window_config_mutations: vec![addressed_config(view, next)?], description: Some("Set camera".into()), ..Default::default() });
            }
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let window_config: Grid3dWindowConfig = config_from_view(cfg);
        let window_id = view_state.window_id.as_deref().or(view_state.focused_window_id.as_deref()).unwrap_or(body_key);
        match body_key {
            grid::BODY_KEY => grid::render(doc.snapshot, &window_config, &[], None).map(semio_framework_plugin::built_to_component_tree),
            preview::BODY_KEY => preview::render(doc.snapshot, &window_config, window_id).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_grid3d_editor() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(WFC_GRID3D_DIALECT).document(["semio", "wfc", "grid3d"]).artifact_kind(crate::artifact_kind()).icon_id("box").interaction(grid::interaction());
    for utility in grid::utilities() {
        builder = builder.utility(utility);
    }
    builder
        .mode_def(edit::definition())
        .default_mode_id(edit::GRID3D_EDIT_MODE_ID)
        .window_kind_def(grid::definition())
        .window_kind_def(preview::definition())
        .window_kind_utilities(grid::WINDOW_KIND_ID, vec![grid::UTILITY_SELECT.into(), grid::UTILITY_PIN.into(), grid::UTILITY_MASK.into()])
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
