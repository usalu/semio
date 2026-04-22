//! One transaction: a stack of [`crate::kit_change::KitChange`] with undo/redo.
use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::kit_change::KitChange;
use crate::read_command::{ReadKitCommand, ReadKitCommandResult};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransactionState {
    Open,
    Finalized,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Id,
    pub changes: Vec<KitChange>,
    pub redo_changes: Vec<KitChange>,
    pub state: TransactionState,
}

impl Transaction {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            changes: Vec::new(),
            redo_changes: Vec::new(),
            state: TransactionState::Open,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionCommand {
    ReadKitCommands { commands: Vec<ReadKitCommand> },
    ChangeKitCommands {
        commands: Vec<crate::change_command::ChangeKitCommand>,
    },
    Finalize,
    Abort,
    Undo,
    UndoAll,
    CanUndo,
    Redo,
    RedoAll,
    CanRedo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionCommandResult {
    ReadKitCommands { results: Vec<ReadKitCommandResult> },
    ChangeKitCommands { count: usize },
    Finalize { ok: bool },
    Abort { ok: bool },
    Undo { ok: bool },
    UndoAll { ok: bool },
    CanUndo { can: bool },
    Redo { ok: bool },
    RedoAll { ok: bool },
    CanRedo { can: bool },
    Nothing,
}

impl Transaction {
    pub fn can_undo(&self) -> bool {
        !self.changes.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_changes.is_empty()
    }
}
