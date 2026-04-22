//! Named ordered list of checkpoints branching from the main kit line.
use serde::{Deserialize, Serialize};

use crate::id::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KitAlternative {
    pub id: Id,
    pub name: String,
    /// First checkpoint on the main (or shared) line this line extends from.
    pub root: Id,
    /// Ordered checkpoint ids (may share ids with other alternatives).
    pub checkpoints: Vec<Id>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KitAlternativeCommand {
    ReadKitCommands {
        commands: Vec<crate::read_command::ReadKitCommand>,
    },
    UnifyKitCheckpointsToSingleKitCheckpoint { message: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KitAlternativeCommandResult {
    ReadKitCommands {
        results: Vec<crate::read_command::ReadKitCommandResult>,
    },
    UnifyKitCheckpointsToSingleKitCheckpoint { new_checkpoint_id: Id },
    Nothing,
}
