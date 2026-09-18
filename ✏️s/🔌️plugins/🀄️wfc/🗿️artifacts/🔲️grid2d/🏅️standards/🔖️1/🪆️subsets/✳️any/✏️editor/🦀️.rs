//! ✏️ 2D-grid editor — the authored `ArtifactEditor` surface for `s.wfc.grid2d@1/*`. Two panes:
//! `🔲️grid` (the interactive Board2d over the authored grid) and `👁️preview` (the Canvas2d render of
//! the SOLVED assignment). Every document command maps 1:1 onto a real `Grid2dMutation` builder
//! from the schema tree — no synthetic "set field" indirection, because the domain's own mutations
//! are already exactly that granular. Camera/grid/active-tile/solve-cache commands write the pane's
//! own `Grid2dWindowConfig` instead, never the document.

use crate::editor::grid2d::modes::edit;
use crate::editor::grid2d::modes::edit::windows::{grid, preview};
use crate::editor::grid2d::window::{self, Grid2dWindowConfig};
use crate::mutations::{
    change_cell_size, change_periodicity, change_seed, change_tile_media, change_tile_weight, create_rule, create_tile, delete_rule, delete_tile, mask_cell, pin_cell, resize_grid, unmask_cell, unpin_cell,
};
use crate::schema::inferences::solve_with_job;
use crate::schema::snapshot::{WfcAdjacencyRule2d, WfcTile2d, WfcTileMedia2d};
use crate::{Grid2dMutation, Grid2dSnapshot, WFC_GRID2D_DIALECT, WFC_GRID2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient,
    NoTransientMutation, ViewModel,
};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

//#region 🔖️ActiveUtility
/// 🧰️ The utility armed for the window being rendered or dispatched: the React host arms utilities
/// per window INSTANCE (`active_utility_by_window_id`) and mirrors only the shell's ACTIVE window
/// into the flat `active_utility_id`, so a pane that is not the active window would otherwise read
/// `None` and run every click as a plain pick.
pub fn grid2d_active_utility(view: &ViewModel) -> &str {
    view.window_id
        .as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or(grid::UTILITY_SELECT)
}
//#endregion 🔖️ActiveUtility

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `Grid2dMutation` kind plus the
/// pane-local view verbs (camera, grid chrome, active tile, solve).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Grid2dEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "resize-grid")]
    ResizeGrid { width: u32, height: u32 },
    #[dsl(key = "change-cell-size")]
    ChangeCellSize { cell_width: f64, cell_height: f64 },
    #[dsl(key = "change-periodicity")]
    ChangePeriodicity { periodic_x: bool, periodic_y: bool },
    #[dsl(key = "create-tile")]
    CreateTile { id: String, label: Option<String>, weight: f64 },
    #[dsl(key = "delete-tile")]
    DeleteTile { id: String },
    #[dsl(key = "change-tile-weight")]
    ChangeTileWeight { id: String, weight: f64 },
    #[dsl(key = "change-tile-media")]
    ChangeTileMedia { id: String, media_json: String },
    #[dsl(key = "create-rule")]
    CreateRule { id: String, tile_a_id: String, tile_b_id: String, direction: String, allowed: bool },
    #[dsl(key = "delete-rule")]
    DeleteRule { id: String },
    #[dsl(key = "pin-cell")]
    PinCell { x: u32, y: u32, tile_id: String },
    #[dsl(key = "unpin-cell")]
    UnpinCell { x: u32, y: u32 },
    #[dsl(key = "mask-cell")]
    MaskCell { x: u32, y: u32 },
    #[dsl(key = "unmask-cell")]
    UnmaskCell { x: u32, y: u32 },
    /// 🖱️ One board click, resolved against the pane's ARMED utility: `pin` writes the active tile,
    /// `mask` toggles the hole, `select` changes nothing.
    #[dsl(key = "pick-cell")]
    PickCell { x: u32, y: u32 },
    #[dsl(key = "set-active-tile")]
    SetActiveTile { tile_id: String },
    #[dsl(key = "set-camera")]
    SetCamera { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "set-grid-visible")]
    SetGridVisible { visible: bool },
    #[dsl(key = "set-grid-snap-enabled")]
    SetGridSnapEnabled { enabled: bool },
    #[dsl(key = "set-grid-factor")]
    SetGridFactor { factor: f64 },
    /// 🏁 Runs the `s.wfc.grid2d.solve` inference and caches its commit in THIS pane's window
    /// config. The document is never touched: the solve is derived, never persisted.
    #[dsl(key = "solve")]
    Solve,
}

impl protocol::OpBinary for Grid2dEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command

//#region 🔖️Args
fn arg<'a>(args: Option<&'a dsl::DslValue>, key: &str) -> Option<&'a dsl::DslValue> {
    match args {
        Some(dsl::DslValue::Object(entries)) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value),
        _ => None,
    }
}

fn arg_f64(args: Option<&dsl::DslValue>, key: &str, fallback: f64) -> f64 {
    match arg(args, key) {
        Some(dsl::DslValue::Number(number)) => number.as_f64(),
        Some(dsl::DslValue::String(text)) => text.parse().unwrap_or(fallback),
        _ => fallback,
    }
}

fn arg_u32(args: Option<&dsl::DslValue>, key: &str, fallback: u32) -> u32 {
    let value = arg_f64(args, key, f64::from(fallback));
    if value.is_finite() && value >= 0.0 {
        value as u32
    } else {
        fallback
    }
}

fn arg_u64(args: Option<&dsl::DslValue>, key: &str, fallback: u64) -> u64 {
    let value = arg_f64(args, key, fallback as f64);
    if value.is_finite() && value >= 0.0 {
        value as u64
    } else {
        fallback
    }
}

fn arg_bool(args: Option<&dsl::DslValue>, key: &str, fallback: bool) -> bool {
    match arg(args, key) {
        Some(dsl::DslValue::Bool(value)) => *value,
        Some(dsl::DslValue::String(text)) => text == "true",
        _ => fallback,
    }
}

fn arg_string(args: Option<&dsl::DslValue>, key: &str) -> String {
    match arg(args, key) {
        Some(dsl::DslValue::String(text)) => text.clone(),
        Some(other) => dsl::json::to_json_string(other),
        None => String::new(),
    }
}
//#endregion 🔖️Args

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Grid2dEditor;

impl Grid2dEditor {
    /// 🀄️ The tile a pin gesture writes: the pane's active tile when it still exists, else the first
    /// authored tile — a cursor that has gone stale must fall back to a real entry, never render
    /// nothing (the "Empty canvas despite real data" fault class).
    fn active_tile(document: &Grid2dSnapshot, config: &Grid2dWindowConfig) -> Option<String> {
        if !config.active_tile_id.is_empty() && document.tiles.iter().any(|tile| tile.id == config.active_tile_id) {
            return Some(config.active_tile_id.clone());
        }
        document.tiles.first().map(|tile| tile.id.clone())
    }

    fn config_emit(view_state: Option<&ViewModel>, config: Grid2dWindowConfig, description: &str) -> Result<Emit<Grid2dMutation>, Fault> {
        let view = view_state.ok_or_else(|| Fault::from("wfc-grid2d-window-required"))?;
        let mutation = window::addressed_config(view, config)?;
        Ok(Emit { window_config_mutations: vec![mutation], description: Some(description.to_string()), ..Default::default() })
    }
}

impl ArtifactEditor for Grid2dEditor {
    type Snapshot = Grid2dSnapshot;
    type Mutation = Grid2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Grid2dEditorCommand;

    const DIALECT: Dialect = WFC_GRID2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_GRID2D_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Grid2dSnapshot {
        crate::examples::grid2d::pipes::document()
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        Ok(match action {
            "change-seed" => Grid2dEditorCommand::ChangeSeed { seed: arg_u64(args, "seed", 0) },
            "resize-grid" => Grid2dEditorCommand::ResizeGrid { width: arg_u32(args, "width", 1), height: arg_u32(args, "height", 1) },
            "change-cell-size" => Grid2dEditorCommand::ChangeCellSize { cell_width: arg_f64(args, "cellWidth", 1.0), cell_height: arg_f64(args, "cellHeight", 1.0) },
            "change-periodicity" => Grid2dEditorCommand::ChangePeriodicity { periodic_x: arg_bool(args, "periodicX", false), periodic_y: arg_bool(args, "periodicY", false) },
            "create-tile" => Grid2dEditorCommand::CreateTile { id: arg_string(args, "id"), label: None, weight: arg_f64(args, "weight", 1.0) },
            "delete-tile" => Grid2dEditorCommand::DeleteTile { id: arg_string(args, "id") },
            "change-tile-weight" => Grid2dEditorCommand::ChangeTileWeight { id: arg_string(args, "id"), weight: arg_f64(args, "weight", 1.0) },
            "change-tile-media" => Grid2dEditorCommand::ChangeTileMedia { id: arg_string(args, "id"), media_json: arg_string(args, "mediaJson") },
            "create-rule" => Grid2dEditorCommand::CreateRule {
                id: arg_string(args, "id"),
                tile_a_id: arg_string(args, "tileAId"),
                tile_b_id: arg_string(args, "tileBId"),
                direction: arg_string(args, "direction"),
                allowed: arg_bool(args, "allowed", true),
            },
            "delete-rule" => Grid2dEditorCommand::DeleteRule { id: arg_string(args, "id") },
            "pin-cell" => Grid2dEditorCommand::PinCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0), tile_id: arg_string(args, "tileId") },
            "unpin-cell" => Grid2dEditorCommand::UnpinCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0) },
            "mask-cell" => Grid2dEditorCommand::MaskCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0) },
            "unmask-cell" => Grid2dEditorCommand::UnmaskCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0) },
            "pick-cell" => Grid2dEditorCommand::PickCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0) },
            "set-active-tile" => Grid2dEditorCommand::SetActiveTile { tile_id: arg_string(args, "tileId") },
            "set-camera" => Grid2dEditorCommand::SetCamera { x: arg_f64(args, "x", 0.0), y: arg_f64(args, "y", 0.0), zoom: arg_f64(args, "zoom", 1.0) },
            "set-grid-visible" => Grid2dEditorCommand::SetGridVisible { visible: arg_bool(args, "visible", true) },
            "set-grid-snap-enabled" => Grid2dEditorCommand::SetGridSnapEnabled { enabled: arg_bool(args, "enabled", true) },
            "set-grid-factor" => Grid2dEditorCommand::SetGridFactor { factor: arg_f64(args, "factor", 1.0) },
            "solve" => Grid2dEditorCommand::Solve,
            other => return Err(Fault::from(format!("wfc-grid2d-unknown-action:{other}"))),
        })
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let window_config = window::config_from_view(cfg);
        let (mutation, description) = match command {
            Grid2dEditorCommand::ChangeSeed { seed } => (change_seed(*seed), format!("Change seed to {seed}")),
            Grid2dEditorCommand::ResizeGrid { width, height } => (resize_grid(*width, *height), format!("Resize grid to {width}×{height}")),
            Grid2dEditorCommand::ChangeCellSize { cell_width, cell_height } => (change_cell_size(*cell_width, *cell_height), "Change cell size".to_string()),
            Grid2dEditorCommand::ChangePeriodicity { periodic_x, periodic_y } => (change_periodicity(*periodic_x, *periodic_y), "Change periodicity".to_string()),
            Grid2dEditorCommand::CreateTile { id, label, weight } => {
                (create_tile(WfcTile2d { id: id.clone(), label: label.clone(), weight: *weight, media: WfcTileMedia2d::default() }), format!("Create tile {id}"))
            }
            Grid2dEditorCommand::DeleteTile { id } => (delete_tile(id.clone()), format!("Delete tile {id}")),
            Grid2dEditorCommand::ChangeTileWeight { id, weight } => (change_tile_weight(id.clone(), *weight), format!("Change weight of {id}")),
            Grid2dEditorCommand::ChangeTileMedia { id, media_json } => {
                let media: WfcTileMedia2d = protocol::json::from_json_str(media_json).map_err(|error| Fault::from(format!("wfc-grid2d-invalid-media:{error}")))?;
                (change_tile_media(id.clone(), media), format!("Change media of {id}"))
            }
            Grid2dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, direction, allowed } => {
                let direction = crate::schema::snapshot::text::direction_from_token(direction).map_err(|error| Fault::from(format!("wfc-grid2d-invalid-direction:{error}")))?;
                (create_rule(WfcAdjacencyRule2d { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), direction, allowed: *allowed }), format!("Create rule {id}"))
            }
            Grid2dEditorCommand::DeleteRule { id } => (delete_rule(id.clone()), format!("Delete rule {id}")),
            Grid2dEditorCommand::PinCell { x, y, tile_id } => (pin_cell(*x, *y, tile_id.clone()), format!("Pin cell ({x}, {y})")),
            Grid2dEditorCommand::UnpinCell { x, y } => (unpin_cell(*x, *y), format!("Unpin cell ({x}, {y})")),
            Grid2dEditorCommand::MaskCell { x, y } => (mask_cell(*x, *y), format!("Mask cell ({x}, {y})")),
            Grid2dEditorCommand::UnmaskCell { x, y } => (unmask_cell(*x, *y), format!("Unmask cell ({x}, {y})")),
            Grid2dEditorCommand::PickCell { x, y } => {
                let utility = view_state.map_or(grid::UTILITY_SELECT, grid2d_active_utility);
                match utility {
                    grid::UTILITY_PIN => {
                        let tile = Self::active_tile(doc.snapshot, &window_config).ok_or_else(|| Fault::from("wfc-grid2d-no-tile-to-pin"))?;
                        (pin_cell(*x, *y, tile), format!("Pin cell ({x}, {y})"))
                    }
                    grid::UTILITY_MASK => {
                        if doc.snapshot.masked.iter().any(|cell| cell.x == *x && cell.y == *y) {
                            (unmask_cell(*x, *y), format!("Unmask cell ({x}, {y})"))
                        } else {
                            (mask_cell(*x, *y), format!("Mask cell ({x}, {y})"))
                        }
                    }
                    _ => return Ok(Emit::default()),
                }
            }
            Grid2dEditorCommand::SetActiveTile { tile_id } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { active_tile_id: tile_id.clone(), ..window_config }, "Set active tile");
            }
            Grid2dEditorCommand::SetCamera { x, y, zoom } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { camera_x: *x, camera_y: *y, camera_zoom: if *zoom > 0.0 { *zoom } else { 1.0 }, ..window_config }, "Set camera");
            }
            Grid2dEditorCommand::SetGridVisible { visible } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { grid_visible: *visible, ..window_config }, "Set grid visibility");
            }
            Grid2dEditorCommand::SetGridSnapEnabled { enabled } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { grid_snap_enabled: *enabled, ..window_config }, "Set grid snapping");
            }
            Grid2dEditorCommand::SetGridFactor { factor } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { grid_factor: if *factor > 0.0 { *factor } else { 1.0 }, ..window_config }, "Set grid factor");
            }
            Grid2dEditorCommand::Solve => {
                let commit = solve_with_job(doc.snapshot).map_err(|error| Fault::from(format!("wfc-grid2d-solve:{error}")))?;
                return Self::config_emit(view_state, Grid2dWindowConfig { solve_json: protocol::json::to_json_string(&commit), ..window_config }, "Solve");
            }
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let config = window::config_from_view(cfg);
        match body_key {
            grid::BODY_KEY => grid::render(doc.snapshot, &config, grid2d_active_utility(view_state)).map(semio_framework_plugin::built_to_component_tree),
            preview::BODY_KEY => preview::render(doc.snapshot, &config).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window::register_config(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window::register_transient(registry)
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_grid2d_editor() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(WFC_GRID2D_DIALECT)
        .document(["semio", "wfc", "grid2d"])
        .icon_id("puzzle")
        .mode_def(edit::definition())
        .default_mode_id(edit::GRID2D_EDIT_MODE_ID)
        .window_kind_def(grid::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout());
    for utility in edit::utilities() {
        builder = builder.utility(utility);
    }
    builder.window_kind_utilities(grid::WINDOW_KIND_ID, vec![grid::UTILITY_SELECT.into(), grid::UTILITY_PIN.into(), grid::UTILITY_MASK.into()]).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
