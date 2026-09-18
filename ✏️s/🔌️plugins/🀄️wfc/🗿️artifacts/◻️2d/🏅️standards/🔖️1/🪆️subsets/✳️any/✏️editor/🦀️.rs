//! ✏️ WFC 2D editor — the `ArtifactEditor` for `s.wfc.wfc2d@1/*`: two windows over one document, the
//! `wfc-graph` slot canvas and the read-only `wfc-2d-preview`.
//!
//! Every command maps 1:1 onto a real `Wfc2dMutation` builder from the schema tree — no synthetic
//! "set field" indirection, because the domain's own mutations are already exactly this granular.
//! The two command families that are NOT document mutations (`change-camera`, `change-active-tile`)
//! go to the per-pane config instead: a camera is per-window view state. The SOLVE goes to the app
//! transient — it is an INFERENCE, so letting it reach the document lanes would make a derived value
//! look persisted and would pollute undo — and reaches the preview window through
//! `ArtifactEditor::render_with_request_context`, the only render entry point that carries that lane.

#![allow(clippy::result_large_err)]

use crate::editor::wfc2d::config::{wfc2d_active_tile_id, Wfc2dConfig, Wfc2dConfigMutation};
use crate::editor::wfc2d::modes::edit;
use crate::editor::wfc2d::modes::edit::windows::{graph, preview};
use crate::editor::wfc2d::transient::{Wfc2dTransient, Wfc2dTransientMutation};
use crate::mutations::{change_seed, change_tile_media, change_tile_weight, connect_slots, create_rule, create_slot, create_tile, delete_rule, delete_slot, delete_tile, disconnect_slots, move_slot, pin_slot, resize_slot, unpin_slot};
use crate::schema::snapshot::{Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dTile, Wfc2dTileMedia};
use crate::{Wfc2dMutation, Wfc2dSnapshot, WFC_2D_DIALECT, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `Wfc2dMutation` kind a UI can
/// trigger, plus the three non-document verbs (camera, armed tile, solve result).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Wfc2dEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "create-slot")]
    CreateSlot { id: String, x: f64, y: f64, width: f64, height: f64 },
    #[dsl(key = "delete-slot")]
    DeleteSlot { id: String },
    #[dsl(key = "move-slot")]
    MoveSlot { id: String, x: f64, y: f64 },
    #[dsl(key = "resize-slot")]
    ResizeSlot { id: String, width: f64, height: f64 },
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

impl protocol::OpBinary for Wfc2dEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command

//#region 🔖️GraphView
/// 🕸️ The `wfc-graph` window's whole view of this document — see that window's own doc comment for
/// why the projection lives here and not there (so `wfc3d` can copy the window file verbatim).
pub struct Wfc2dGraphView<'a>(pub &'a Wfc2dSnapshot);

impl graph::SlotGraphView for Wfc2dGraphView<'_> {
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
pub struct Wfc2dEditor;

/// ✏️ The whole command→emission rule set, as a free function over plain values. `ArtifactEditor::handle`
/// only adapts the framework's view bundle onto this: `InteractionView` has `pub(crate)` fields, so
/// this crate's own tests cannot construct one and would otherwise have no way to exercise dispatch
/// at all (cad's own test-harness precedent, `📐️cad/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`).
///
/// A pin gesture reads the PANE's armed tile rather than a global one, so two panes can pin different
/// tiles. Camera and armed-tile verbs go to the pane config and never touch the document lanes.
pub fn dispatch(command: &Wfc2dEditorCommand, document: &Wfc2dSnapshot, config: &Wfc2dConfig) -> Result<Emit<Wfc2dMutation, Wfc2dConfigMutation>, Fault> {
    let (mutation, description) = match command {
        Wfc2dEditorCommand::ChangeCamera { x, y, zoom } => {
            let mutations = vec![Wfc2dConfigMutation::ChangeCamera(crate::editor::wfc2d::config::ChangeCamera { x: *x, y: *y, zoom: *zoom })];
            return Ok(Emit { config_mutations: mutations, description: Some("Change camera".into()), ..Default::default() });
        }
        Wfc2dEditorCommand::ChangeActiveTile { tile_id } => {
            let mutations = vec![Wfc2dConfigMutation::ChangeActiveTile(crate::editor::wfc2d::config::ChangeActiveTile { tile_id: tile_id.clone() })];
            return Ok(Emit { config_mutations: mutations, description: Some("Arm tile".into()), ..Default::default() });
        }
        Wfc2dEditorCommand::ChangeSeed { seed } => (change_seed(*seed), format!("Change seed to {seed}")),
        Wfc2dEditorCommand::CreateSlot { id, x, y, width, height } => (create_slot(Wfc2dSlot { id: id.clone(), x: *x, y: *y, width: *width, height: *height, pinned_tile_id: None }), format!("Create slot {id}")),
        Wfc2dEditorCommand::DeleteSlot { id } => (delete_slot(id.clone()), format!("Delete slot {id}")),
        Wfc2dEditorCommand::MoveSlot { id, x, y } => (move_slot(id.clone(), *x, *y), format!("Move slot {id}")),
        Wfc2dEditorCommand::ResizeSlot { id, width, height } => (resize_slot(id.clone(), *width, *height), format!("Resize slot {id}")),
        Wfc2dEditorCommand::ConnectSlots { id, from_slot_id, to_slot_id, relation } => (
            connect_slots(Wfc2dSlotEdge {
                id: id.clone(),
                from_slot_id: from_slot_id.clone(),
                to_slot_id: to_slot_id.clone(),
                relation: if relation.is_empty() { crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.into() } else { relation.clone() },
            }),
            format!("Connect {from_slot_id} to {to_slot_id}"),
        ),
        Wfc2dEditorCommand::DisconnectSlots { id } => (disconnect_slots(id.clone()), format!("Disconnect {id}")),
        Wfc2dEditorCommand::PinSlot { id, tile_id } => {
            let tile = if tile_id.is_empty() { wfc2d_active_tile_id(config, document) } else { Some(tile_id.clone()) };
            let Some(tile) = tile else {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::Plugin, semio_framework_plugin::FaultCode::new("wfc2d.tile.unknown-pin"), "No tile is armed and the document declares none."));
            };
            (pin_slot(id.clone(), tile.clone()), format!("Pin {id} to {tile}"))
        }
        Wfc2dEditorCommand::UnpinSlot { id } => (unpin_slot(id.clone()), format!("Unpin slot {id}")),
        Wfc2dEditorCommand::CreateTile { id, label, weight } => (create_tile(Wfc2dTile { id: id.clone(), label: label.clone(), weight: *weight, media: Wfc2dTileMedia::default() }), format!("Create tile {id}")),
        Wfc2dEditorCommand::DeleteTile { id } => (delete_tile(id.clone()), format!("Delete tile {id}")),
        Wfc2dEditorCommand::ChangeTileWeight { tile_id, weight } => (change_tile_weight(tile_id.clone(), *weight), format!("Change weight of {tile_id}")),
        Wfc2dEditorCommand::ChangeTileMedia { tile_id } => (change_tile_media(tile_id.clone(), Wfc2dTileMedia::default()), format!("Clear media of {tile_id}")),
        Wfc2dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, relation, allowed } => (
            create_rule(Wfc2dRule { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), relation: relation.clone(), allowed: *allowed }),
            format!("Create rule {id}"),
        ),
        Wfc2dEditorCommand::DeleteRule { id } => (delete_rule(id.clone()), format!("Delete rule {id}")),
    };
    Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
}

/// 🖼️ The whole render rule set, likewise free of the framework's view bundle so a test can call it.
pub fn render_body(body_key: &str, document: &Wfc2dSnapshot, config: &Wfc2dConfig, transient: &Wfc2dTransient) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    match body_key {
        graph::WFC_GRAPH_BODY => {
            let camera = graph::GraphCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom };
            graph::render(&Wfc2dGraphView(document), camera, &[]).map(semio_framework_plugin::built_to_component_tree)
        }
        preview::WFC_2D_PREVIEW_BODY => preview::render(document, transient, config.camera_x, config.camera_y, config.camera_zoom).map(semio_framework_plugin::built_to_component_tree),
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}

impl ArtifactEditor for Wfc2dEditor {
    type Snapshot = Wfc2dSnapshot;
    type Mutation = Wfc2dMutation;
    type Config = Wfc2dConfig;
    type ConfigMutation = Wfc2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = Wfc2dTransient;
    type TransientMutation = Wfc2dTransientMutation;
    type Command = Wfc2dEditorCommand;

    const DIALECT: Dialect = WFC_2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_2D_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Wfc2dSnapshot {
        crate::examples::two_room_corridor::document()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation>, Fault> {
        dispatch(command, doc.snapshot, cfg.snapshot)
    }

    /// 🫧️ The bare `render` is the trait's transient-LESS entry point: its signature carries no
    /// transient lane at all, so this path can only paint what the DOCUMENT says (authored pins).
    /// Every host request goes through `render_with_request_context` below, which does carry the
    /// lane; this exists for callers that have no request context to offer.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc.snapshot, cfg.snapshot, &Wfc2dTransient::default())
    }

    /// 🫧️ The HOST-FACING render: the framework hands the app-local transient store in here, so this
    /// is where the solved assignment actually reaches `wfc-2d-preview`. Delegates to the free
    /// `render_with_transient` below so a test can drive the same code without an `InteractionView`
    /// (whose fields are `pub(crate)` in the framework crate and cannot be built from here).
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_with_transient(body_key, doc, cfg, transient)
    }
}

/// 🫧️ The exact projection `render_with_request_context` performs, minus the two arguments a test
/// cannot construct. `transient.snapshot` IS the app-local `Wfc2dTransient` the solve job publishes
/// into, so the preview window paints the LAST SOLVE here and authored pins only where the solve has
/// not reached yet.
pub fn render_with_transient(
    body_key: &str,
    doc: &ArtifactView<'_, Wfc2dSnapshot>,
    cfg: &ConfigView<'_, Wfc2dConfig>,
    transient: &semio_framework_plugin::TransientView<'_, Wfc2dTransient>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    render_body(body_key, doc.snapshot, cfg.snapshot, transient.snapshot)
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_wfc2d_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(WFC_2D_DIALECT)
        .document(["semio", "wfc", "2d"])
        .icon_id("network")
        .mode_def(edit::definition())
        .default_mode_id(edit::WFC_2D_EDIT_MODE_ID)
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
