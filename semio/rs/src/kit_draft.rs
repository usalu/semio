//! Draft: stack of finalized transactions on top of a checkpoint tip, plus optional open transaction.
use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::kit::KitFullDto;
use crate::kit_transaction::{Transaction, TransactionCommand, TransactionCommandResult};
use crate::read_command::{ReadKitCommand, ReadKitCommandResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Draft {
    pub id: Id,
    /// `None` = base is [`KitStore.initial`] only (no checkpoint yet on that line).
    pub parent_checkpoint: Option<Id>,
    /// When set, commits extend this alternative's checkpoint list instead of `the_kit_head`.
    pub target_alternative: Option<Id>,
    pub before: KitFullDto,
    pub transactions: Vec<Transaction>,
    pub redo_transactions: Vec<Transaction>,
    /// Open transaction for `ChangeKitCommands` (at most one).
    pub open_transaction: Option<Transaction>,
}

impl Draft {
    pub fn open_tx_id(&self) -> Option<&Id> {
        self.open_transaction.as_ref().map(|t| &t.id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KitDraftCommand {
    ReadKitCommands { commands: Vec<ReadKitCommand> },
    StartTransaction,
    FinalizeToKitCheckpoint { message: String },
    Abort,
    Undo { count: i32 },
    CanUndo { count: i32 },
    Redo { count: i32 },
    CanRedo { count: i32 },
    ExecuteTransactionCommands {
        id: Id,
        commands: Vec<TransactionCommand>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KitDraftCommandResult {
    ReadKitCommands { results: Vec<ReadKitCommandResult> },
    StartTransaction { transaction_id: Id },
    FinalizeToKitCheckpoint { checkpoint_id: Id },
    Abort { ok: bool },
    Undo { ok: bool },
    CanUndo { can: bool },
    Redo { ok: bool },
    CanRedo { can: bool },
    ExecuteTransactionCommands { results: Vec<TransactionCommandResult> },
    Nothing,
}
