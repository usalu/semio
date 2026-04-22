//! Client session: owns drafts.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::id::Id;
use crate::kit_draft::{Draft, KitDraftCommand, KitDraftCommandResult};
use crate::read_command::{ReadKitCommand, ReadKitCommandResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Id,
    pub drafts: HashMap<Id, Draft>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionCommand {
    ReadKitCommands { commands: Vec<ReadKitCommand> },
    NewDraft {
        checkpoint_id: Option<Id>,
        alternative_id: Option<Id>,
    },
    ExecuteKitDraftCommands {
        id: Id,
        commands: Vec<KitDraftCommand>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionCommandResult {
    ReadKitCommands { results: Vec<ReadKitCommandResult> },
    NewDraft { draft_id: Id },
    ExecuteKitDraftCommands { results: Vec<KitDraftCommandResult> },
    Nothing,
}
