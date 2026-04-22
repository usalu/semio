//! Checkpoints and materialized kit snapshots on the main / shared tree.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::hash::HashWriter;
use crate::id::Id;
use crate::kit::KitFullDto;
use crate::kit_change::KitChange;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedKit {
    pub initial: KitFullDto,
    pub change_list: Vec<KitChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub computed: Option<KitFullDto>,
}

impl MaterializedKit {
    pub fn compute(&self) -> KitFullDto {
        self.change_list.iter().fold(self.initial.clone(), |acc, c| {
            crate::kit_change::KitChange::apply_forward_dto(&acc, c)
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KitCheckpoint {
    pub id: Id,
    pub parent: Option<Id>,
    pub changes: Vec<KitChange>,
    pub message: Option<String>,
    pub time: Option<String>,
    pub authors: Vec<Id>,
    pub hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<MaterializedKit>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KitCheckpointCommand {
    ReadKitCommands {
        commands: Vec<crate::read_command::ReadKitCommand>,
    },
    MarkAsRelease,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KitCheckpointCommandResult {
    ReadKitCommands {
        results: Vec<crate::read_command::ReadKitCommandResult>,
    },
    MarkAsRelease { ok: bool },
    Nothing,
}

/// Chain from root to `at` (inclusive): walk parents from `at` to `None`, reverse.
pub fn checkpoint_chain_root_to_leaf(
    at: &Id,
    checkpoints: &HashMap<Id, KitCheckpoint>,
) -> Vec<Id> {
    let mut out = vec![at.clone()];
    let mut cur = at.clone();
    loop {
        let parent = checkpoints
            .get(&cur)
            .and_then(|c| c.parent.as_ref().cloned());
        match parent {
            None => break,
            Some(p) if out.contains(&p) => break,
            Some(p) => {
                out.push(p.clone());
                cur = p;
            }
        }
    }
    out.reverse();
    out
}

pub fn materialize_dto(
    initial: &KitFullDto,
    checkpoints: &HashMap<Id, KitCheckpoint>,
    at: Option<&Id>,
) -> KitFullDto {
    let Some(at_id) = at else {
        return initial.clone();
    };
    let path = checkpoint_chain_root_to_leaf(at_id, checkpoints);
    let mut s = initial.clone();
    for cid in &path {
        if let Some(cp) = checkpoints.get(cid) {
            for ch in &cp.changes {
                s = crate::kit_change::KitChange::apply_forward_dto(&s, ch);
            }
        }
    }
    s
}

pub fn hash_checkpoint(
    parent: Option<&Id>,
    new_id: &Id,
    changes: &[KitChange],
) -> String {
    let mut w = HashWriter::new();
    w.tag("kit_cp");
    if let Some(p) = parent {
        w.str(p.as_str());
    } else {
        w.str("");
    }
    w.str(new_id.as_str());
    w.str(
        &changes
            .iter()
            .map(|c| serde_json::to_string(c).unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n"),
    );
    w.finalize()
}
