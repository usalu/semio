//! 🎛️ Command vocabulary of the WFC 3D editor: the command enum, its wire ids and the tool and example constants. See [`super`].

use super::*;

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `Wfc3dMutation` kind a UI can
/// trigger, plus the two non-document verbs (camera, armed tile).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
pub enum Wfc3dEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "create-slot")]
    CreateSlot { id: String, x: f64, y: f64, z: f64, width: f64, height: f64, depth: f64 },
    #[dsl(key = "delete-slot")]
    DeleteSlot { id: String },
    /// 🚚️ `z` is OPTIONAL because the `wfc-graph` window is shared with `wfc2d` and its staged form
    /// therefore offers only `x`/`y`. An absent `z` means "keep the slot's authored height", never
    /// "zero" — a 2d form must not be able to flatten a stack it cannot even see.
    #[dsl(key = "move-slot")]
    MoveSlot { id: String, x: f64, y: f64, z: Option<f64> },
    /// 📐️ `depth` is optional for the same reason, with the same meaning.
    #[dsl(key = "resize-slot")]
    ResizeSlot { id: String, width: f64, height: f64, depth: Option<f64> },
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
    #[dsl(key = "solve")]
    Solve,
    #[dsl(key = "commit-fill")]
    CommitFill { payload_json: String },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    /// 🕹️ The NodeGraph canvas' OWN gesture channel: a released node drag (the node-graph gesture record), a
    /// completed, cut or deleted wire and a deleted node all arrive as `nodeGraphEdit` rows, never as
    /// `move-slot`/`connect-slots` directly. One release is ONE drag-tool transaction of relative `drag-slots` leaves.
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { operations_json: String },
    /// 🎥️ The canvas' settled camera. Its own verb rather than `change-camera`, because a command's
    /// id must equal the action it was admitted under.
    #[dsl(key = "node-graph-viewport")]
    NodeGraphViewport { x: f64, y: f64, zoom: f64 },
}

impl protocol::OpBinary for Wfc3dEditorCommand {
    /// 🧵️ The list `validate_tool_job_rows` joins against: `TOOL_JOB_IDS ∩ migrated` must EQUAL the
    /// `bounded_first_step_tool_proofs!` rows, or the app boots with `interactive-job.catalog-incomplete`.
    const TOOL_JOB_IDS: &'static [&'static str] = WFC_3D_RETAINED_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// 🪪️ The exact tool id one command dispatches as — `dispatch_typed_command_inner` rejects a command
/// whose id does not equal the admitted action, so this is the single naming authority for the
/// manifest actions, the retained roster and the publication contracts alike.
pub fn wfc3d_command_id(command: &Wfc3dEditorCommand) -> &'static str {
    match command {
        Wfc3dEditorCommand::ChangeSeed { .. } => "change-seed",
        Wfc3dEditorCommand::CreateSlot { .. } => "create-slot",
        Wfc3dEditorCommand::DeleteSlot { .. } => "delete-slot",
        Wfc3dEditorCommand::MoveSlot { .. } => "move-slot",
        Wfc3dEditorCommand::ResizeSlot { .. } => "resize-slot",
        Wfc3dEditorCommand::ConnectSlots { .. } => "connect-slots",
        Wfc3dEditorCommand::DisconnectSlots { .. } => "disconnect-slots",
        Wfc3dEditorCommand::PinSlot { .. } => "pin-slot",
        Wfc3dEditorCommand::UnpinSlot { .. } => "unpin-slot",
        Wfc3dEditorCommand::CreateTile { .. } => "create-tile",
        Wfc3dEditorCommand::DeleteTile { .. } => "delete-tile",
        Wfc3dEditorCommand::ChangeTileWeight { .. } => "change-tile-weight",
        Wfc3dEditorCommand::ChangeTileMedia { .. } => "change-tile-media",
        Wfc3dEditorCommand::CreateRule { .. } => "create-rule",
        Wfc3dEditorCommand::DeleteRule { .. } => "delete-rule",
        Wfc3dEditorCommand::ChangeCamera { .. } => "change-camera",
        Wfc3dEditorCommand::ChangeActiveTile { .. } => "change-active-tile",
        Wfc3dEditorCommand::Solve => "solve",
        Wfc3dEditorCommand::CommitFill { .. } => fill::COMMIT_FILL_ACTION_ID,
        Wfc3dEditorCommand::SetActiveExample { .. } => WFC_3D_SET_ACTIVE_EXAMPLE,
        Wfc3dEditorCommand::NodeGraphEdit { .. } => WFC_3D_NODE_GRAPH_EDIT,
        Wfc3dEditorCommand::NodeGraphViewport { .. } => WFC_3D_NODE_GRAPH_VIEWPORT,
    }
}

/// 🔗 The relation a connect gesture authors when the caller names none. The graph canvas drags a
/// wire between two `adjacent` ports; that is the relation it means.
pub const WFC_3D_DEFAULT_RELATION: &str = "adjacent";

/// 📏️ Canvas units one authored slot unit occupies in the `wfc-graph` pane. A slot is a BOX in
/// metres, so a document authored at `width: 1.0` would paint a one-pixel node at viewport zoom 1 and
/// three slots a metre apart would overlap inside the canvas' own minimum node size — the pane was
/// legible only as a smudge. The scale is the caller's, never the shared window's: the window file
/// stays byte-identical to `wfc2d`'s, and dividing a released drag's canvas offset by it is what turns the drag back
/// into a relative `drag-slots` in document units.
pub const WFC_3D_GRAPH_UNIT: f64 = 120.0;

/// 🕹️ The interaction domain the graph canvas selects and hovers in. Declaring ONE domain is what
/// makes the framework inject `interactionSelect`/`interactionHover`/`clearSelection` onto every
/// window kind; without it the shell drops every pointer gesture as `undeclared-action`.
pub const WFC_3D_INTERACTION_GRAPH: &str = "slot";
//#endregion 🔖️Command
