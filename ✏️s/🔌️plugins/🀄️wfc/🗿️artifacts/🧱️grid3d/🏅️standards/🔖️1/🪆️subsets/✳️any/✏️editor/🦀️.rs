//! ✏️ `s.wfc.grid3d` editor — two `World3d` windows over one document: the GRID (the persisted
//! problem, one box instance per cell) and the PREVIEW (the inferred assignment). Every document
//! verb maps 1:1 onto a real `Grid3dMutation` builder from the schema tree — no synthetic
//! "set field" indirection, since this domain's own mutations are already exactly that granular.
//!
//! 🖱️ `pickCell` is the one verb whose meaning depends on state: it reads the ARMED UTILITY of the
//! window it was dispatched in (`active_utility_by_window_id` first, focused window second — the
//! `🗒️note` fallback chain) and turns one pick into `pin-cell`, `mask-cell`, or nothing at all.

use crate::editor::grid3d::modes::edit;
use crate::editor::grid3d::modes::edit::tools::fill as fill_tool;
use crate::editor::grid3d::modes::edit::windows::{grid, preview};
use crate::editor::grid3d::window::{addressed_config, config_from_view, Grid3dWindowConfig};
use crate::mutations::{change_cell_sizes, change_periodicity, change_seed, change_tile_media, change_tile_weight, create_rule, create_tile, delete_rule, delete_tile, mask_cell, pin_cell, resize_grid, unmask_cell, unpin_cell};
use crate::schema::snapshot::{tile_index, Grid3dAxis, Grid3dCell, Grid3dColor, Grid3dDirection, Grid3dMesh, Grid3dPinnedCell, Grid3dRule, Grid3dTile, Grid3dTileMedia};
use crate::{Grid3dMutation, Grid3dSnapshot, WFC_GRID3D_DIALECT, WFC_GRID3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ToolRef, ToolRunJob, ToolRunJobRequest};
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
    #[dsl(key = "setActiveExample")]
    SetActiveExample { example_id: String },
    /// 🏁 Starts the interactive fill tool run; the finished cache lands in `Grid3dPreviewResidency`.
    #[dsl(key = "solve")]
    Solve,
    /// 🖱️ The host's own instance-pick verb, dispatched when the scene declares no `domainId` — the
    /// lane a WRITING utility runs on. It is `pickCell`'s twin rather than an alias, because
    /// `dispatch_typed_command_inner` rejects a command whose `command_id` is not the action it was
    /// admitted under.
    #[dsl(key = "worldSelect")]
    WorldSelect { cell_id: String },
    /// 🖱️ The host's DOMAINLESS hover lane, dispatched on every pointer move over a domainless scene.
    /// It is inert on purpose: a hover is per-frame state and writing it to the window config would put
    /// a store write on the pointer path. Declaring it is not optional — an undeclared `setHover` is
    /// refused once per pointer move and floods the console.
    #[dsl(key = "setHover")]
    SetHover { object_id: Option<String> },
    /// 🖱️ The domainless lane's OTHER pick verb: a click that hit no instance (the host clears the
    /// selection with `id: null`) or a sub-object component hit. Both are inert here — this window's
    /// meshes are whole cells with no addressable components, and clicking empty space is not an edit.
    #[dsl(key = "worldPick")]
    WorldPick { object_id: Option<String> },
}

impl protocol::OpBinary for Grid3dEditorCommand {
    /// 🧵️ The list `validate_tool_job_rows` joins against: `TOOL_JOB_IDS ∩ migrated` must EQUAL the
    /// `bounded_first_step_tool_proofs!` rows, or the app boots with `interactive-job.catalog-incomplete`.
    const TOOL_JOB_IDS: &'static [&'static str] = GRID3D_RETAINED_TOOL_IDS;

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

/// 📚️ The registered example an id names, or `None` for an id this artifact never published. An
/// unknown id FAULTS rather than silently opening an empty grid — the navbar picker sends the id it
/// read off the manifest, so a mismatch is a real registration bug and has to be loud.
pub fn example_snapshot(example_id: &str) -> Option<Grid3dSnapshot> {
    match example_id {
        crate::examples::blocks::ID => Some(crate::examples::blocks::snapshot()),
        crate::examples::pipes_3d::ID => Some(crate::examples::pipes_3d::snapshot()),
        _ => None,
    }
}

/// 🔁️ The sanctioned whole-document replace: a `LoadDocument` effect over a freshly packed snapshot
/// and an EDIT-FREE op log (`store::empty_document_spr`). Never a live `ArtifactEnvelope` minted just
/// to print it — such an envelope owns a bounded retirement authority whose `Drop` traps the guest
/// (`🧊️process3d`'s own boot `setActiveExample` incident).
pub fn reset_document_effect(document: &Grid3dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Grid3dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("grid3d", WFC_GRID3D_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Helpers

//#region 🔖️ActionBridge
/// 🎯️ The one boundary that reads the host's transport shape. React and wgpu dispatch a manifest
/// ACTION ID plus a `DslValue` argument bag; the trait default refuses every one of them
/// (`app.command.unsupported`), so without this bridge the whole declared action vocabulary — all
/// seventeen document verbs, the example picker and the world pick — is unreachable from the browser
/// and only the crate's own typed tests can drive the editor.
/// 🖱️ Both pick lanes land on the same verb: `pickCell` carries the cell key directly, while the
/// host's plugin-private `worldSelect` wraps it in a one-entry `ids` array.
fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Grid3dEditorCommand, Fault> {
    let field = |key: &str| args.and_then(|value| value.get(key));
    let text = |key: &str| field(key).and_then(dsl::DslValue::as_str).map(str::to_string);
    let float = |key: &str| field(key).and_then(dsl::DslValue::as_f64).filter(|number| number.is_finite());
    let count = |key: &str| float(key).filter(|number| *number >= 0.0).map(|number| number as u32);
    let flag = |key: &str| field(key).and_then(dsl::DslValue::as_bool);
    let floats = |key: &str| field(key).and_then(dsl::DslValue::as_array).map(|values| values.iter().filter_map(dsl::DslValue::as_f64).collect::<Vec<f64>>());
    let missing = |what: &str| Fault::from(format!("wfc.grid3d.action.missing-argument '{action}.{what}'"));
    let color = |prefix: &str| Grid3dColor {
        r: count(&format!("{prefix}r")).unwrap_or(255),
        g: count(&format!("{prefix}g")).unwrap_or(255),
        b: count(&format!("{prefix}b")).unwrap_or(255),
        a: count(&format!("{prefix}a")).unwrap_or(255),
    };
    match action {
        "changeSeed" => Ok(Grid3dEditorCommand::ChangeSeed { seed: float("seed").filter(|seed| *seed >= 0.0).map(|seed| seed as u64).ok_or_else(|| missing("seed"))? }),
        "resizeGrid" => Ok(Grid3dEditorCommand::ResizeGrid { width: count("width").ok_or_else(|| missing("width"))?, height: count("height").ok_or_else(|| missing("height"))?, depth: count("depth").ok_or_else(|| missing("depth"))? }),
        "changeCellSizes" => {
            let axis = text("axis").and_then(|token| [Grid3dAxis::X, Grid3dAxis::Y, Grid3dAxis::Z].into_iter().find(|axis| axis.label().eq_ignore_ascii_case(&token))).ok_or_else(|| missing("axis"))?;
            Ok(Grid3dEditorCommand::ChangeCellSizes { axis, sizes: floats("sizes").ok_or_else(|| missing("sizes"))? })
        }
        "changePeriodicity" => Ok(Grid3dEditorCommand::ChangePeriodicity { periodic_x: flag("periodicX").unwrap_or_default(), periodic_y: flag("periodicY").unwrap_or_default(), periodic_z: flag("periodicZ").unwrap_or_default() }),
        "createTile" => {
            let color = color("");
            Ok(Grid3dEditorCommand::CreateTile { id: text("id").ok_or_else(|| missing("id"))?, label: text("label"), weight: float("weight").unwrap_or(1.0), r: color.r, g: color.g, b: color.b, a: color.a })
        }
        "deleteTile" => Ok(Grid3dEditorCommand::DeleteTile { id: text("id").ok_or_else(|| missing("id"))? }),
        "changeTileWeight" => Ok(Grid3dEditorCommand::ChangeTileWeight { tile_id: text("tileId").or_else(|| text("id")).ok_or_else(|| missing("tileId"))?, weight: float("weight").ok_or_else(|| missing("weight"))? }),
        "changeTileColor" => {
            let color = color("");
            Ok(Grid3dEditorCommand::ChangeTileColor { tile_id: text("tileId").or_else(|| text("id")).ok_or_else(|| missing("tileId"))?, r: color.r, g: color.g, b: color.b, a: color.a })
        }
        "createRule" => Ok(Grid3dEditorCommand::CreateRule {
            id: text("id").ok_or_else(|| missing("id"))?,
            tile_a_id: text("tileAId").ok_or_else(|| missing("tileAId"))?,
            tile_b_id: text("tileBId").ok_or_else(|| missing("tileBId"))?,
            direction: text("direction").and_then(|token| Grid3dDirection::ALL.into_iter().find(|direction| direction.label().eq_ignore_ascii_case(&token))).ok_or_else(|| missing("direction"))?,
            allowed: flag("allowed").unwrap_or(true),
        }),
        "deleteRule" => Ok(Grid3dEditorCommand::DeleteRule { id: text("id").ok_or_else(|| missing("id"))? }),
        "pinCell" => Ok(Grid3dEditorCommand::PinCell { x: count("x").ok_or_else(|| missing("x"))?, y: count("y").ok_or_else(|| missing("y"))?, z: count("z").ok_or_else(|| missing("z"))?, tile_id: text("tileId").ok_or_else(|| missing("tileId"))? }),
        "unpinCell" => Ok(Grid3dEditorCommand::UnpinCell { x: count("x").ok_or_else(|| missing("x"))?, y: count("y").ok_or_else(|| missing("y"))?, z: count("z").ok_or_else(|| missing("z"))? }),
        "maskCell" => Ok(Grid3dEditorCommand::MaskCell { x: count("x").ok_or_else(|| missing("x"))?, y: count("y").ok_or_else(|| missing("y"))?, z: count("z").ok_or_else(|| missing("z"))? }),
        "unmaskCell" => Ok(Grid3dEditorCommand::UnmaskCell { x: count("x").ok_or_else(|| missing("x"))?, y: count("y").ok_or_else(|| missing("y"))?, z: count("z").ok_or_else(|| missing("z"))? }),
        "setActiveTile" => Ok(Grid3dEditorCommand::SetActiveTile { tile_id: text("tileId").or_else(|| text("id")).unwrap_or_default() }),
        "setCamera" => Ok(Grid3dEditorCommand::SetCamera {
            x: float("x").unwrap_or_default(),
            y: float("y").unwrap_or_default(),
            z: float("z").unwrap_or_default(),
            target_x: float("targetX").unwrap_or_default(),
            target_y: float("targetY").unwrap_or_default(),
            target_z: float("targetZ").unwrap_or_default(),
            zoom: float("zoom").unwrap_or(1.0),
        }),
        "pickCell" => Ok(Grid3dEditorCommand::PickCell { cell_id: text("cellId").or_else(|| text("id")).unwrap_or_default() }),
        grid::ACTION_SET_HOVER => Ok(Grid3dEditorCommand::SetHover { object_id: text("objectId").or_else(|| text("id")) }),
        grid::ACTION_WORLD_PICK => Ok(Grid3dEditorCommand::WorldPick { object_id: text("objectId") }),
        grid::ACTION_WORLD_SELECT => Ok(Grid3dEditorCommand::WorldSelect {
            cell_id: field("ids").and_then(dsl::DslValue::as_array).and_then(|ids| ids.first()).and_then(dsl::DslValue::as_str).map(str::to_string).or_else(|| text("id")).unwrap_or_default(),
        }),
        grid::ACTION_SET_ACTIVE_EXAMPLE => Ok(Grid3dEditorCommand::SetActiveExample {
            example_id: text("exampleId").or_else(|| text("id")).or_else(|| text("value")).unwrap_or_else(|| crate::examples::blocks::ID.to_string()),
        }),
        "solve" => Ok(Grid3dEditorCommand::Solve),
        _ => Err(Fault::from(format!("wfc.grid3d.action.unsupported '{action}'"))),
    }
}
//#endregion 🔖️ActionBridge

//#region 🔖️Reducer
/// ✏️ Dispatches straight onto the schema tree's own mutation builders. `setActiveTile` and
/// `setCamera` never touch the document — they address the exact window instance's own config — and a
/// pick resolves to a real document verb only when a writing utility is armed.
///
/// 🧵️ A free function rather than an inherent one, because BOTH dispatch routes call it: the direct
/// `ArtifactEditor::handle` and the retained `Grid3dCommandWork::step`, which is handed the view state
/// through its `ArtifactOwnedToolJobContext` and would otherwise have no way to reach it.
pub fn grid3d_command_emit(
    command: &Grid3dEditorCommand,
    doc: &ArtifactView<'_, Grid3dSnapshot>,
    cfg: &ConfigView<'_, NoConfig>,
    view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Grid3dMutation>, Fault> {
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
        Grid3dEditorCommand::PickCell { cell_id } | Grid3dEditorCommand::WorldSelect { cell_id } => {
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
        Grid3dEditorCommand::SetHover { .. } | Grid3dEditorCommand::WorldPick { .. } => return Ok(Emit::default()),
        Grid3dEditorCommand::Solve => {
            return Ok(Emit { effects: vec![fill_tool::start_effect()], description: Some("Solve".to_string()), ..Default::default() });
        }
        Grid3dEditorCommand::SetActiveExample { example_id } => {
            let Some(next_document) = example_snapshot(example_id) else {
                return Err(Fault::from(format!("wfc.grid3d.example.unknown '{example_id}'")));
            };
            if &next_document == document {
                return Ok(Emit::default());
            }
            return Ok(Emit { effects: vec![reset_document_effect(&next_document)], description: Some(format!("Load example {example_id}")), ..Default::default() });
        }
    };
    Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
}
//#endregion 🔖️Reducer

//#region 🧵️Retained
/// 🧵️ Every verb a pane may dispatch. `Migrated` is the only live classification, so EVERY dispatch
/// goes through the retained route: an id absent here carries no app-owned factory and answers
/// `interactive-job.missing-factory` — the verb is then dead in the running app however loudly the
/// window declares it.
pub const GRID3D_RETAINED_TOOL_IDS: &[&str] = &[
    "changeSeed",
    "resizeGrid",
    "changeCellSizes",
    "changePeriodicity",
    "createTile",
    "deleteTile",
    "changeTileWeight",
    "changeTileColor",
    "createRule",
    "deleteRule",
    "pinCell",
    "unpinCell",
    "maskCell",
    "unmaskCell",
    "pickCell",
    grid::ACTION_WORLD_SELECT,
    grid::ACTION_SET_HOVER,
    grid::ACTION_WORLD_PICK,
    "setActiveTile",
    "setCamera",
    grid::ACTION_SET_ACTIVE_EXAMPLE,
    "solve",
];

const GRID3D_RETAINED_PAYLOAD_SCHEMA: &str = "wfc.grid3d.tool-command.v1";
const GRID3D_RETAINED_RAW_BYTES: usize = 65_536;
const GRID3D_RETAINED_WORK_ITEMS: usize = 4_096;

/// 📄️ One document-lane publication row.
const fn artifact_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] }
}

/// 🪟️ One per-window-instance config row — the camera pose and the armed tile are pane state, never
/// document state, and each belongs to exactly one window instance.
const fn window_config_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] }
}

/// 🛣️ One lane per route, read off each arm of `grid3d_command_emit`. The example picker is
/// `HostOnly`: it answers a whole-document `Effect::LoadDocument`, which is not an edit, so
/// re-picking the open example mints no phantom undo entry.
const GRID3D_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    artifact_route("changeSeed"),
    artifact_route("resizeGrid"),
    artifact_route("changeCellSizes"),
    artifact_route("changePeriodicity"),
    artifact_route("createTile"),
    artifact_route("deleteTile"),
    artifact_route("changeTileWeight"),
    artifact_route("changeTileColor"),
    artifact_route("createRule"),
    artifact_route("deleteRule"),
    artifact_route("pinCell"),
    artifact_route("unpinCell"),
    artifact_route("maskCell"),
    artifact_route("unmaskCell"),
    artifact_route("pickCell"),
    artifact_route(grid::ACTION_WORLD_SELECT),
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: grid::ACTION_SET_HOVER, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: grid::ACTION_WORLD_PICK, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    window_config_route("setActiveTile"),
    window_config_route("setCamera"),
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: grid::ACTION_SET_ACTIVE_EXAMPLE, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "solve", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
];

/// 📏️ One bounded first step per route, admitted only while the whole addressable document plus this
/// edit fit inside `GRID3D_RETAINED_WORK_ITEMS`.
fn grid3d_retained_extent(command: &Grid3dEditorCommand, snapshot: &Grid3dSnapshot) -> Option<usize> {
    if !GRID3D_RETAINED_TOOL_IDS.contains(&<Grid3dEditor as ArtifactEditor>::command_id(command)) {
        return None;
    }
    let items = snapshot.tiles.len().checked_add(snapshot.rules.len())?.checked_add(snapshot.pinned.len())?.checked_add(snapshot.masked.len())?.checked_add(1)?;
    (items <= GRID3D_RETAINED_WORK_ITEMS).then_some(1)
}

/// 🧵️ The ONE bounded work step every grid3d route runs — the pure `grid3d_command_emit` reducer,
/// handed the view state through the job context so the pick verbs can still read the armed utility
/// and the config verbs can still address their own window instance.
struct Grid3dCommandWork {
    tool_id: &'static str,
    completed: bool,
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Grid3dEditor>> for Grid3dCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Grid3dEditorCommand,
        snapshot: &Grid3dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<Grid3dEditor>>>,
    ) -> Option<usize> {
        (!self.completed && <Grid3dEditor as ArtifactEditor>::command_id(command) == self.tool_id).then(|| grid3d_retained_extent(command, snapshot)).flatten()
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<Grid3dEditor>>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<Grid3dEditor>>, Fault> {
        if self.completed || <Grid3dEditor as ArtifactEditor>::command_id(input.command) != self.tool_id {
            return Err(Fault::from("wfc.grid3d.retained.route"));
        }
        let emit = grid3d_command_emit(
            input.command,
            &ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone()),
            &ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) },
            input.context.and_then(|context| context.view_state.as_ref()),
        )?;
        self.completed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(emit))
    }
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block binds this exact
/// Rust type to `EditorApp<Grid3dEditor>`, which is what turns a bare (and therefore dispatch-dead)
/// bounded proof into an exact-owner proof.
struct Grid3dRetainedCommandJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl Grid3dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GRID3D_RETAINED_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Grid3dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<Grid3dEditor>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<Grid3dEditor>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GRID3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::bounded_first_step(GRID3D_RETAINED_RAW_BYTES, GRID3D_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GRID3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("bounded WFC 3D grid command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Grid3dRetainedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<Grid3dEditor>;
    const TOOL_IDS: &'static [&'static str] = GRID3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC_GRID3D_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = GRID3D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️Retained

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

    /// 👥️🫧️ The close ladder retires the presence and transient ROOTS through installed factories and
    /// refuses to teardown without them; this editor carries no presence at all, so the `No*` factories
    /// are its exact terminal.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🗃️ The bounded retirement catalog every store lane is released THROUGH: a store built without
    /// owners answers `artifact store has no owner-supplied bounded disposer` the moment the close
    /// ladder reaches it, so the owners and the disposer below are one declaration in two halves.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<NoDraft, NoDraftMutation>())
    }

    /// ♻️ The instance close ladder walks one owned store lane per stage and faults the whole close
    /// with `interactive-job.close-owned-disposer-missing` the moment a lane answers `None`, so an
    /// editor must declare how EVERY store it types is released, not only the ones it edits.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn initial_snapshot() -> Grid3dSnapshot {
        crate::examples::blocks::snapshot()
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        crate::editor::grid3d::window::register_config(registry)
    }

    /// 🏷️ The manifest action id this command was declared under. The trait default answers one
    /// opaque `"typed-command"` for every variant, which makes a history row unaddressable and breaks
    /// the declared-action/command round trip.
    fn command_id(command: &Self::Command) -> &'static str {
        match command {
            Grid3dEditorCommand::ChangeSeed { .. } => "changeSeed",
            Grid3dEditorCommand::ResizeGrid { .. } => "resizeGrid",
            Grid3dEditorCommand::ChangeCellSizes { .. } => "changeCellSizes",
            Grid3dEditorCommand::ChangePeriodicity { .. } => "changePeriodicity",
            Grid3dEditorCommand::CreateTile { .. } => "createTile",
            Grid3dEditorCommand::DeleteTile { .. } => "deleteTile",
            Grid3dEditorCommand::ChangeTileWeight { .. } => "changeTileWeight",
            Grid3dEditorCommand::ChangeTileColor { .. } => "changeTileColor",
            Grid3dEditorCommand::CreateRule { .. } => "createRule",
            Grid3dEditorCommand::DeleteRule { .. } => "deleteRule",
            Grid3dEditorCommand::PinCell { .. } => "pinCell",
            Grid3dEditorCommand::UnpinCell { .. } => "unpinCell",
            Grid3dEditorCommand::MaskCell { .. } => "maskCell",
            Grid3dEditorCommand::UnmaskCell { .. } => "unmaskCell",
            Grid3dEditorCommand::PickCell { .. } => "pickCell",
            Grid3dEditorCommand::WorldSelect { .. } => grid::ACTION_WORLD_SELECT,
            Grid3dEditorCommand::SetHover { .. } => grid::ACTION_SET_HOVER,
            Grid3dEditorCommand::WorldPick { .. } => grid::ACTION_WORLD_PICK,
            Grid3dEditorCommand::SetActiveTile { .. } => "setActiveTile",
            Grid3dEditorCommand::SetCamera { .. } => "setCamera",
            Grid3dEditorCommand::SetActiveExample { .. } => grid::ACTION_SET_ACTIVE_EXAMPLE,
            Grid3dEditorCommand::Solve => "solve",
        }
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        command_from_action(action, args)
    }

    /// 📬️ The document lane's one-item retained preparation. Without one, EVERY route declaring
    /// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract
    /// and stays dispatch-dead — the live playground answers "typed command 'worldSelect' declares the
    /// unsupported artifact publication lane" on the first pick.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("grid3d-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn build_tool_run_job(request: ToolRunJobRequest<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        fill_tool::build_run_job(request)
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Grid3dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GRID3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let tool_id = <Self as ArtifactEditor>::command_id(&request.command);
        if tool_id != request.tool_id {
            return Err(Fault::from("wfc.grid3d.retained.tool-mismatch"));
        }
        if grid3d_retained_extent(&request.command, &request.snapshot).is_none() {
            return Err(Fault::from("wfc.grid3d.retained.extent"));
        }
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = Box::new(Grid3dCommandWork { tool_id, completed: false });
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            <Self as ArtifactEditor>::command_id,
            GRID3D_RETAINED_RAW_BYTES,
            GRID3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Grid3dEditor>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.wfc.grid3d@1/*#editor",
        artifact_schema: "s.wfc.grid3d",
        factory: "Grid3dRetainedCommandJobFactory",
        factory_type: Grid3dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "changeSeed", "resizeGrid", "changeCellSizes", "changePeriodicity",
            "createTile", "deleteTile", "changeTileWeight", "changeTileColor",
            "createRule", "deleteRule",
            "pinCell", "unpinCell", "maskCell", "unmaskCell",
            "pickCell", "worldSelect", "setHover", "worldPick", "setActiveTile", "setCamera", "setActiveExample", "solve"
        ]
    }

    /// 🔁️ Admits the envelope a `LoadDocument` effect hands the store. The trait default REFUSES it
    /// (`Err(envelope)`), which faults every whole-document swap at the archive-load boundary — and
    /// the example picker is exactly such a swap.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, WFC_GRID3D_DOCUMENT_SCHEMA, operation, generation))
    }

    /// ✏️ Every dispatch route — the direct one and the retained one — runs the very same reducer,
    /// so a verb cannot mean two things depending on how it arrived.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        grid3d_command_emit(command, doc, cfg, view_state)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let window_config: Grid3dWindowConfig = config_from_view(cfg);
        let window_id = view_state.window_id.as_deref().or(view_state.focused_window_id.as_deref()).unwrap_or(body_key);
        match body_key {
            grid::BODY_KEY => grid::render(doc.snapshot, &window_config, &[], None, grid3d_active_utility(Some(view_state))).map(semio_framework_plugin::built_to_component_tree),
            preview::BODY_KEY => preview::render(doc.snapshot, &window_config, window_id, doc.tool_run()).map(semio_framework_plugin::built_to_component_tree),
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
        .tool(fill_tool::definition())
        .mode_tools(edit::GRID3D_EDIT_MODE_ID, vec![semio_framework::io::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))])
        .default_layout(edit::layout())
        .action_destructive("deleteTile")
        .action_destructive("deleteRule")
        .action_destructive(grid::ACTION_SET_ACTIVE_EXAMPLE)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
