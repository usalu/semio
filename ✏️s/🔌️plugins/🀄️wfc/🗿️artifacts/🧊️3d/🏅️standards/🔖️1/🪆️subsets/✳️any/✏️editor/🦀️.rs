//! ✏️ WFC 3D editor — the `ArtifactEditor` for `s.wfc.wfc3d@1/*`: two windows over one document, the
//! `wfc-graph` slot canvas (copied verbatim from `wfc2d`, so both artifacts present ONE window kind
//! to the shell) and the read-only `wfc-3d-preview` World3d pane.
//!
//! Every command maps 1:1 onto a real `Wfc3dMutation` builder from the schema tree — no synthetic
//! "set field" indirection, because the domain's own mutations are already exactly this granular.
//! The command families that are NOT document mutations (`change-camera`, `change-active-tile`) go to
//! the per-pane config instead: a camera is per-window view state, and the solve is an INFERENCE, so
//! letting either reach the document lanes would make a derived value look persisted and pollute undo.
//!
//! Collection inserts always land at the collection's CANONICAL SORTED position
//! (`canonical_*_index`), so a delete followed by its inverse puts the row back exactly where it was.

#![allow(clippy::result_large_err)]

use crate::editor::wfc3d::config::{wfc3d_active_tile_id, Wfc3dConfig, Wfc3dConfigMutation};
use crate::editor::wfc3d::modes::edit;
use crate::editor::wfc3d::modes::edit::windows::{graph, preview};
use crate::editor::wfc3d::transient::solved_transient;
use crate::mutations::{change_seed, change_tile_media, change_tile_weight, connect_slots, create_rule, create_slot, create_tile, delete_rule, delete_slot, delete_tile, disconnect_slots, move_slot, pin_slot, resize_slot, unpin_slot};
use crate::schema::snapshot::{canonical_edge_index, canonical_rule_index, canonical_slot_index, canonical_tile_index, GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d};
use crate::{Wfc3dMutation, Wfc3dSnapshot, WFC3D_DIALECT, WFC3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `Wfc3dMutation` kind a UI can
/// trigger, plus the two non-document verbs (camera, armed tile).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Wfc3dEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "create-slot")]
    CreateSlot { id: String, x: f64, y: f64, z: f64, width: f64, height: f64, depth: f64 },
    #[dsl(key = "delete-slot")]
    DeleteSlot { id: String },
    #[dsl(key = "move-slot")]
    MoveSlot { id: String, x: f64, y: f64, z: f64 },
    #[dsl(key = "resize-slot")]
    ResizeSlot { id: String, width: f64, height: f64, depth: f64 },
    #[dsl(key = "connect-slots")]
    ConnectSlots { id: String, from_slot_id: String, to_slot_id: String, relation: String },
    #[dsl(key = "disconnect-slots")]
    DisconnectSlots { id: String },
    #[dsl(key = "pin-slot")]
    PinSlot { id: String, tile_id: String },
    #[dsl(key = "unpin-slot")]
    UnpinSlot { id: String },
    #[dsl(key = "create-tile")]
    CreateTile { id: String, label: Option<String>, weight: f64 },
    #[dsl(key = "delete-tile")]
    DeleteTile { id: String },
    #[dsl(key = "change-tile-weight")]
    ChangeTileWeight { tile_id: String, weight: f64 },
    #[dsl(key = "change-tile-media")]
    ChangeTileMedia { tile_id: String },
    #[dsl(key = "create-rule")]
    CreateRule { id: String, tile_a_id: String, tile_b_id: String, relation: Option<String>, allowed: bool },
    #[dsl(key = "delete-rule")]
    DeleteRule { id: String },
    #[dsl(key = "change-camera")]
    ChangeCamera { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "change-active-tile")]
    ChangeActiveTile { tile_id: String },
}

impl protocol::OpBinary for Wfc3dEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// 🔗 The relation a connect gesture authors when the caller names none. The graph canvas drags a
/// wire between two `adjacent` ports; that is the relation it means.
pub const WFC_3D_DEFAULT_RELATION: &str = "adjacent";
//#endregion 🔖️Command

//#region 🔖️GraphView
/// 🕸️ The `wfc-graph` window's whole view of this document — the projection lives HERE, not in the
/// window, which is what lets `wfc2d` and `wfc3d` share one window file byte for byte. `z` is
/// deliberately dropped: the graph canvas is a 2d plan of an arbitrary graph, and the third axis is
/// the preview pane's business.
pub struct Wfc3dGraphView<'a>(pub &'a Wfc3dSnapshot);

impl graph::SlotGraphView for Wfc3dGraphView<'_> {
    fn graph_slots(&self) -> Vec<graph::GraphSlotView> {
        self.0
            .slots
            .iter()
            .map(|slot| graph::GraphSlotView { id: slot.id.clone(), x: slot.x, y: slot.y, width: slot.width, height: slot.height, pinned_tile_id: slot.pinned_tile_id.clone() })
            .collect()
    }
    fn graph_edges(&self) -> Vec<graph::GraphEdgeView> {
        self.0.edges.iter().map(|edge| graph::GraphEdgeView { id: edge.id.clone(), from_slot_id: edge.from_slot_id.clone(), to_slot_id: edge.to_slot_id.clone(), relation: edge.relation.clone() }).collect()
    }
}
//#endregion 🔖️GraphView

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Wfc3dEditor;

impl ArtifactEditor for Wfc3dEditor {
    type Snapshot = Wfc3dSnapshot;
    type Mutation = Wfc3dMutation;
    type Config = Wfc3dConfig;
    type ConfigMutation = Wfc3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Wfc3dEditorCommand;

    const DIALECT: Dialect = WFC3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC3D_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Wfc3dSnapshot {
        crate::examples::two_room_corridor::snapshot()
    }

    /// ✏️ Dispatches straight onto the schema tree's own mutation builders. A pin gesture reads the
    /// PANE's armed tile rather than a global one, so two panes can pin different tiles.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation>, Fault> {
        command_emit(command, doc.snapshot, cfg.snapshot)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc.snapshot, cfg.snapshot)
    }
}

/// ✏️ The whole command→mutation mapping, as a PURE function of the document and the pane config —
/// `handle` is the thin `ArtifactView`/`ConfigView` adapter over it. Split out so the mapping is
/// reachable from a unit test: `ArtifactView`'s own fields are private, so nothing outside the
/// framework can build one.
pub fn command_emit(command: &Wfc3dEditorCommand, document: &Wfc3dSnapshot, config: &Wfc3dConfig) -> Result<Emit<Wfc3dMutation, Wfc3dConfigMutation>, Fault> {
    {
        let (mutation, description) = match command {
            Wfc3dEditorCommand::ChangeCamera { x, y, zoom } => {
                let emitted = vec![Wfc3dConfigMutation::ChangeCamera(crate::editor::wfc3d::config::ChangeCamera { x: *x, y: *y, zoom: *zoom })];
                return Ok(Emit { config_mutations: emitted, description: Some("Change camera".into()), ..Default::default() });
            }
            Wfc3dEditorCommand::ChangeActiveTile { tile_id } => {
                let emitted = vec![Wfc3dConfigMutation::ChangeActiveTile(crate::editor::wfc3d::config::ChangeActiveTile { tile_id: tile_id.clone() })];
                return Ok(Emit { config_mutations: emitted, description: Some("Arm tile".into()), ..Default::default() });
            }
            Wfc3dEditorCommand::ChangeSeed { seed } => (change_seed(*seed), format!("Change seed to {seed}")),
            Wfc3dEditorCommand::CreateSlot { id, x, y, z, width, height, depth } => (
                create_slot(
                    canonical_slot_index(document, id),
                    Slot3d { id: id.clone(), x: *x, y: *y, z: *z, width: positive_or(*width), height: positive_or(*height), depth: positive_or(*depth), pinned_tile_id: None },
                ),
                format!("Create slot {id}"),
            ),
            Wfc3dEditorCommand::DeleteSlot { id } => (delete_slot(id.clone()), format!("Delete slot {id}")),
            Wfc3dEditorCommand::MoveSlot { id, x, y, z } => (move_slot(id.clone(), *x, *y, *z), format!("Move slot {id}")),
            Wfc3dEditorCommand::ResizeSlot { id, width, height, depth } => (resize_slot(id.clone(), positive_or(*width), positive_or(*height), positive_or(*depth)), format!("Resize slot {id}")),
            Wfc3dEditorCommand::ConnectSlots { id, from_slot_id, to_slot_id, relation } => (
                connect_slots(
                    canonical_edge_index(document, id),
                    SlotEdge { id: id.clone(), from_slot_id: from_slot_id.clone(), to_slot_id: to_slot_id.clone(), relation: if relation.is_empty() { WFC_3D_DEFAULT_RELATION.into() } else { relation.clone() } },
                ),
                format!("Connect {from_slot_id} to {to_slot_id}"),
            ),
            Wfc3dEditorCommand::DisconnectSlots { id } => (disconnect_slots(id.clone()), format!("Disconnect {id}")),
            Wfc3dEditorCommand::PinSlot { id, tile_id } => {
                let tile = if tile_id.is_empty() { wfc3d_active_tile_id(config, document) } else { Some(tile_id.clone()) };
                let Some(tile) = tile else { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.tile.unknown-pin"), "No tile is armed and the document declares none.")) };
                (pin_slot(id.clone(), tile.clone()), format!("Pin {id} to {tile}"))
            }
            Wfc3dEditorCommand::UnpinSlot { id } => (unpin_slot(id.clone()), format!("Unpin slot {id}")),
            Wfc3dEditorCommand::CreateTile { id, label, weight } => (
                create_tile(canonical_tile_index(document, id), Tile { id: id.clone(), label: label.clone(), weight: positive_or(*weight), media: TileMedia3d::default() }),
                format!("Create tile {id}"),
            ),
            Wfc3dEditorCommand::DeleteTile { id } => (delete_tile(id.clone()), format!("Delete tile {id}")),
            Wfc3dEditorCommand::ChangeTileWeight { tile_id, weight } => (change_tile_weight(tile_id.clone(), positive_or(*weight)), format!("Change weight of {tile_id}")),
            Wfc3dEditorCommand::ChangeTileMedia { tile_id } => (change_tile_media(tile_id.clone(), crate::unit_box_media(None)), format!("Reset media of {tile_id}")),
            Wfc3dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, relation, allowed } => (
                create_rule(canonical_rule_index(document, id), GraphRule { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), relation: relation.clone(), allowed: *allowed }),
                format!("Create rule {id}"),
            ),
            Wfc3dEditorCommand::DeleteRule { id } => (delete_rule(id.clone()), format!("Delete rule {id}")),
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
    }
}

/// 🖼️ The whole per-window render, as a PURE function of the document and the pane config — the same
/// split, for the same reason.
pub fn render_body(body_key: &str, document: &Wfc3dSnapshot, config: &Wfc3dConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    match body_key {
        graph::WFC_GRAPH_BODY => {
            let camera = graph::GraphCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom };
            graph::render(&Wfc3dGraphView(document), camera, &[]).map(semio_framework_plugin::built_to_component_tree)
        }
        preview::WFC_3D_PREVIEW_BODY => preview::render(document, &solved_transient(document), config.camera_zoom).map(semio_framework_plugin::built_to_component_tree),
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}

/// 📐️ A slot extent or tile weight the UI left at zero still has to be positive, or the mutation's
/// own invariant guard refuses it — the editor supplies the honest unit default rather than emitting
/// a command it knows will be rejected.
fn positive_or(value: f64) -> f64 {
    if value > 0.0 {
        value
    } else {
        1.0
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_wfc3d_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(WFC3D_DIALECT)
        .document(["semio", "wfc", "3d"])
        .icon_id("network")
        .mode_def(edit::definition())
        .default_mode_id(edit::WFC_3D_EDIT_MODE_ID)
        .window_kind_def(graph::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
