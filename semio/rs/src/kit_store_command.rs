//! Top-level kit store command dispatch.
use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::kit::KitStore;
use crate::kit::KitStoreRef;
use crate::kit_alternative::{KitAlternative, KitAlternativeCommand, KitAlternativeCommandResult};
use crate::kit_change::{KitChange, KitChangeKind};
use crate::kit_checkpoint::{self, KitCheckpoint, KitCheckpointCommand, KitCheckpointCommandResult, MaterializedKit};
use crate::kit_draft::{Draft, KitDraftCommand, KitDraftCommandResult};
use crate::kit_session::{Session, SessionCommand, SessionCommandResult};
use crate::kit_transaction::{Transaction, TransactionCommand, TransactionCommandResult, TransactionState};
use crate::read_command::{self, ReadKitCommand, ReadKitCommandResult};
use crate::{error::Result, error::SemioError};

type KitFullDto = crate::kit::KitFullDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KitStoreCommand {
    ReadKitCommands { commands: Vec<ReadKitCommand> },
    NewSession,
    EndSession { id: Id },
    /// Branch: first checkpoint in the new alternative list.
    NewAlternative { from_checkpoint: Id, name: String },
    ExecuteSessionCommands { id: Id, commands: Vec<SessionCommand> },
    ExecuteKitCheckpointCommands { id: Id, commands: Vec<KitCheckpointCommand> },
    ExecuteKitAlternativeCommands { id: Id, commands: Vec<KitAlternativeCommand> },
    /// Run many commands in order (e.g. JSON array at WASM boundary).
    Batch { commands: Vec<KitStoreCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum KitStoreCommandResult {
    ReadKitCommands { results: Vec<ReadKitCommandResult> },
    NewSession { id: Id },
    EndSession { ok: bool },
    NewAlternative { id: Id },
    ExecuteSessionCommands { results: Vec<SessionCommandResult> },
    ExecuteKitCheckpointCommands { results: Vec<KitCheckpointCommandResult> },
    ExecuteKitAlternativeCommands { results: Vec<KitAlternativeCommandResult> },
    Batch { results: Vec<KitStoreCommandResult> },
    Nothing,
}

fn tip_alternative(store: &KitStore, alt: &Id) -> Option<Id> {
    store
        .alternatives
        .get(alt)
        .and_then(|a| a.checkpoints.last().cloned())
}

fn valid_draft_base(store: &KitStore, cp: Option<&Id>, alt: Option<&Id>) -> bool {
    match (alt, cp) {
        (None, None) => store.the_kit_head.is_none(),
        (None, Some(c)) => store.the_kit_head.as_ref() == Some(c),
        (Some(a), Some(c)) => tip_alternative(store, a).as_ref() == Some(c),
        (Some(_), None) => false,
    }
}

/// Replace the live graph from a full DTO while preserving VCS + event bus + legacy undo.
pub fn replace_graph_preserve(kit: &KitStoreRef, d: KitFullDto) -> Result<()> {
    KitStore::replace_from_full_dto(kit, d).map_err(|e| SemioError::InvalidOperation(e.to_string()))
}

pub fn the_kit_dto(store: &KitStore) -> KitFullDto {
    kit_checkpoint::materialize_dto(&store.initial, &store.checkpoints, store.the_kit_head.as_ref())
}

fn materialize_at(store: &KitStore, at: Option<&Id>) -> KitFullDto {
    kit_checkpoint::materialize_dto(&store.initial, &store.checkpoints, at)
}

/// Top-level VCS / command entry (call with the kit write lock when batching, or this locks internally).
pub fn execute(kit: &KitStoreRef, cmd: KitStoreCommand) -> Result<KitStoreCommandResult> {
    match cmd {
        KitStoreCommand::Batch { commands } => {
            let mut out = Vec::with_capacity(commands.len());
            for c in commands {
                out.push(execute(kit, c)?);
            }
            Ok(KitStoreCommandResult::Batch { results: out })
        }
        KitStoreCommand::ReadKitCommands { commands } => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let dto = the_kit_dto(&g);
            let results = read_command::read_kits(&dto, &commands)?;
            drop(g);
            Ok(KitStoreCommandResult::ReadKitCommands { results })
        }
        KitStoreCommand::NewSession => {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let id = Id::new_v7();
            g.sessions.insert(
                id.clone(),
                Session {
                    id: id.clone(),
                    drafts: Default::default(),
                },
            );
            Ok(KitStoreCommandResult::NewSession { id })
        }
        KitStoreCommand::EndSession { id } => {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.sessions.remove(&id);
            Ok(KitStoreCommandResult::EndSession { ok: true })
        }
        KitStoreCommand::NewAlternative { from_checkpoint, name } => {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            if !g.checkpoints.contains_key(&from_checkpoint) {
                return Err(SemioError::NotFound {
                    kind: "KitCheckpoint",
                    id: from_checkpoint,
                });
            }
            let aid = Id::new_v7();
            g.alternatives.insert(
                aid.clone(),
                KitAlternative {
                    id: aid.clone(),
                    name,
                    root: from_checkpoint.clone(),
                    checkpoints: vec![from_checkpoint],
                },
            );
            Ok(KitStoreCommandResult::NewAlternative { id: aid })
        }
        KitStoreCommand::ExecuteSessionCommands { id, commands } => {
            let mut results = Vec::new();
            for c in commands {
                results.push(exec_session(kit, &id, c)?);
            }
            Ok(KitStoreCommandResult::ExecuteSessionCommands { results })
        }
        KitStoreCommand::ExecuteKitCheckpointCommands { id, commands } => {
            let mut results = Vec::new();
            for c in commands {
                results.push(exec_checkpoint(kit, &id, c)?);
            }
            Ok(KitStoreCommandResult::ExecuteKitCheckpointCommands { results })
        }
        KitStoreCommand::ExecuteKitAlternativeCommands { id, commands } => {
            let mut results = Vec::new();
            for c in commands {
                results.push(exec_alternative(kit, &id, c)?);
            }
            Ok(KitStoreCommandResult::ExecuteKitAlternativeCommands { results })
        }
    }
}

fn exec_session(kit: &KitStoreRef, sid: &Id, cmd: SessionCommand) -> Result<SessionCommandResult> {
    match cmd {
        SessionCommand::ReadKitCommands { commands } => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let dto = the_kit_dto(&g);
            let results = read_command::read_kits(&dto, &commands)?;
            Ok(SessionCommandResult::ReadKitCommands { results })
        }
        SessionCommand::NewDraft {
            checkpoint_id,
            alternative_id,
        } => {
            let g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            if !g.sessions.contains_key(sid) {
                return Err(SemioError::InvalidOperation("unknown session".into()));
            }
            if !valid_draft_base(&g, checkpoint_id.as_ref(), alternative_id.as_ref()) {
                return Err(SemioError::InvalidOperation("stale or invalid draft base".into()));
            }
            let base = materialize_at(&g, checkpoint_id.as_ref());
            // Reset live graph to the materialized base for this draft.
            let dclone = base.clone();
            let aid = Id::new_v7();
            let draft = Draft {
                id: aid.clone(),
                parent_checkpoint: checkpoint_id.clone(),
                target_alternative: alternative_id.clone(),
                before: base,
                transactions: vec![],
                redo_transactions: vec![],
                open_transaction: None,
            };
            drop(g);
            replace_graph_preserve(kit, dclone)?;
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.sessions
                .get_mut(sid)
                .expect("session")
                .drafts
                .insert(aid.clone(), draft);
            Ok(SessionCommandResult::NewDraft { draft_id: aid })
        }
        SessionCommand::ExecuteKitDraftCommands { id: did, commands } => {
            let mut results = Vec::new();
            for c in commands {
                results.push(exec_draft(kit, sid, &did, c)?);
            }
            Ok(SessionCommandResult::ExecuteKitDraftCommands { results })
        }
    }
}

fn exec_draft(
    kit: &KitStoreRef,
    sid: &Id,
    did: &Id,
    cmd: KitDraftCommand,
) -> Result<KitDraftCommandResult> {
    match cmd {
        KitDraftCommand::ReadKitCommands { commands } => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let dto = g.to_full_dto();
            let results = read_command::read_kits(&dto, &commands)?;
            Ok(KitDraftCommandResult::ReadKitCommands { results })
        }
        KitDraftCommand::StartTransaction => {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let session = g.sessions.get_mut(sid).ok_or_else(|| {
                SemioError::InvalidOperation("unknown session for draft tx".into())
            })?;
            let d = session
                .drafts
                .get_mut(did)
                .ok_or_else(|| SemioError::InvalidOperation("unknown draft".into()))?;
            if d.open_transaction.is_some() {
                return Err(SemioError::InvalidOperation(
                    "transaction already open".into(),
                ));
            }
            let tid = Id::new_v7();
            d.open_transaction = Some(Transaction::new(tid.clone()));
            Ok(KitDraftCommandResult::StartTransaction { transaction_id: tid })
        }
        KitDraftCommand::ExecuteTransactionCommands { id: txid, commands } => {
            let mut results = Vec::new();
            for c in commands {
                results.push(exec_transaction(kit, sid, did, &txid, c)?);
            }
            Ok(KitDraftCommandResult::ExecuteTransactionCommands { results })
        }
        KitDraftCommand::FinalizeToKitCheckpoint { message } => finalize_draft(kit, sid, did, message),
        KitDraftCommand::Abort => {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            if let Some(session) = g.sessions.get_mut(sid) {
                session.drafts.remove(did);
            }
            Ok(KitDraftCommandResult::Abort { ok: true })
        }
        KitDraftCommand::Undo { count } => draft_undo(kit, sid, did, count),
        KitDraftCommand::Redo { count } => draft_redo(kit, sid, did, count),
        KitDraftCommand::CanUndo { count: _ } => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let can = g
                .sessions
                .get(sid)
                .and_then(|s| s.drafts.get(did))
                .map(|d| !d.transactions.is_empty())
                .unwrap_or(false);
            Ok(KitDraftCommandResult::CanUndo { can })
        }
        KitDraftCommand::CanRedo { count: _ } => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let can = g
                .sessions
                .get(sid)
                .and_then(|s| s.drafts.get(did))
                .map(|d| !d.redo_transactions.is_empty())
                .unwrap_or(false);
            Ok(KitDraftCommandResult::CanRedo { can })
        }
    }
}

fn draft_undo(kit: &KitStoreRef, sid: &Id, did: &Id, count: i32) -> Result<KitDraftCommandResult> {
    let n = if count < 0 { i32::MAX } else { count } as usize;
    for _ in 0..n {
        let tx_opt = {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .sessions
                .get_mut(sid)
                .and_then(|s| s.drafts.get_mut(did))
                .ok_or_else(|| SemioError::InvalidOperation("no draft".into()))?;
            d.transactions.pop()
        };
        let Some(tx) = tx_opt else { break; };
        // Roll back composite: replay backward on full tx changes in reverse order
        for ch in tx.changes.iter().rev() {
            KitChange::apply_backward(ch, kit)
                .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
        }
        let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
        if let Some(d) = g.sessions.get_mut(sid).and_then(|s| s.drafts.get_mut(did)) {
            d.redo_transactions.push(tx);
        }
    }
    Ok(KitDraftCommandResult::Undo { ok: true })
}

fn draft_redo(kit: &KitStoreRef, sid: &Id, did: &Id, count: i32) -> Result<KitDraftCommandResult> {
    let n = if count < 0 { i32::MAX } else { count } as usize;
    for _ in 0..n {
        let tx_opt = {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .sessions
                .get_mut(sid)
                .and_then(|s| s.drafts.get_mut(did))
                .ok_or_else(|| SemioError::InvalidOperation("no draft".into()))?;
            d.redo_transactions.pop()
        };
        let Some(tx) = tx_opt else { break; };
        for ch in &tx.changes {
            KitChange::apply_forward(ch, kit)
                .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
        }
        let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
        if let Some(d) = g.sessions.get_mut(sid).and_then(|s| s.drafts.get_mut(did)) {
            d.transactions.push(tx);
        }
    }
    Ok(KitDraftCommandResult::Redo { ok: true })
}

fn exec_transaction(
    kit: &KitStoreRef,
    sid: &Id,
    did: &Id,
    txid: &Id,
    cmd: TransactionCommand,
) -> Result<TransactionCommandResult> {
    match cmd {
        TransactionCommand::ReadKitCommands { commands } => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let dto = g.to_full_dto();
            let results = read_command::read_kits(&dto, &commands)?;
            Ok(TransactionCommandResult::ReadKitCommands { results })
        }
        TransactionCommand::ChangeKitCommands { commands: chs } => {
            let mut n = 0usize;
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            {
                let _ = g
                    .sessions
                    .get(sid)
                    .and_then(|s| s.drafts.get(did))
                    .and_then(|d| d.open_transaction.as_ref())
                    .filter(|t| t.id == *txid && t.state == TransactionState::Open)
                    .ok_or_else(|| {
                        SemioError::InvalidOperation("no open matching transaction".into())
                    })?;
            }
            for c in &chs {
                let before = g.to_full_dto();
                let kind = crate::change_command::apply_change_kit_command(&mut g, c)?;
                let after = g.to_full_dto();
                if let Some(mut kc) = KitChange::between(&before, &after) {
                    kc.kind = kind;
                    if let Some(d) = g.sessions.get_mut(sid).and_then(|s| s.drafts.get_mut(did)) {
                        if let Some(ot) = d.open_transaction.as_mut() {
                            if ot.id == *txid {
                                ot.changes.push(kc);
                                ot.redo_changes.clear();
                                n += 1;
                            }
                        }
                    }
                }
            }
            Ok(TransactionCommandResult::ChangeKitCommands { count: n })
        }
        TransactionCommand::Finalize => {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .sessions
                .get_mut(sid)
                .and_then(|s| s.drafts.get_mut(did))
                .ok_or_else(|| SemioError::InvalidOperation("no draft".into()))?;
            let mut tx = d
                .open_transaction
                .take()
                .filter(|t| t.id == *txid)
                .ok_or_else(|| SemioError::InvalidOperation("no tx to finalize".into()))?;
            tx.state = TransactionState::Finalized;
            d.transactions.push(tx);
            Ok(TransactionCommandResult::Finalize { ok: true })
        }
        TransactionCommand::Abort => {
            let to_undo: Vec<KitChange> = {
                let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                let d = g
                    .sessions
                    .get_mut(sid)
                    .and_then(|s| s.drafts.get_mut(did))
                    .ok_or_else(|| SemioError::InvalidOperation("no draft".into()))?;
                if d.open_transaction.as_ref().map(|t| t.id.clone()) != Some(txid.clone()) {
                    return Err(SemioError::InvalidOperation("tx id mismatch abort".into()));
                }
                d.open_transaction
                    .as_ref()
                    .map(|t| t.changes.clone())
                    .unwrap_or_default()
            };
            for ch in to_undo.iter().rev() {
                KitChange::apply_backward(ch, kit)
                    .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
            }
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            if let Some(d) = g
                .sessions
                .get_mut(sid)
                .and_then(|s| s.drafts.get_mut(did))
            {
                d.open_transaction = None;
            }
            Ok(TransactionCommandResult::Abort { ok: true })
        }
        TransactionCommand::Undo => transaction_undo(kit, sid, did, txid, false),
        TransactionCommand::UndoAll => transaction_undo(kit, sid, did, txid, true),
        TransactionCommand::Redo => transaction_redo(kit, sid, did, txid, false),
        TransactionCommand::RedoAll => transaction_redo(kit, sid, did, txid, true),
        TransactionCommand::CanUndo => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let can = g
                .sessions
                .get(sid)
                .and_then(|s| s.drafts.get(did))
                .and_then(|d| d.open_transaction.as_ref())
                .filter(|t| t.id == *txid)
                .map(|t| t.can_undo())
                .unwrap_or(false);
            Ok(TransactionCommandResult::CanUndo { can })
        }
        TransactionCommand::CanRedo => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let can = g
                .sessions
                .get(sid)
                .and_then(|s| s.drafts.get(did))
                .and_then(|d| d.open_transaction.as_ref())
                .filter(|t| t.id == *txid)
                .map(|t| t.can_redo())
                .unwrap_or(false);
            Ok(TransactionCommandResult::CanRedo { can })
        }
    }
}

fn transaction_undo(
    kit: &KitStoreRef,
    sid: &Id,
    did: &Id,
    txid: &Id,
    all: bool,
) -> Result<TransactionCommandResult> {
    loop {
        let done = {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .sessions
                .get_mut(sid)
                .and_then(|s| s.drafts.get_mut(did))
                .ok_or_else(|| SemioError::InvalidOperation("no draft".into()))?;
            let tx = d
                .open_transaction
                .as_mut()
                .filter(|t| t.id == *txid)
                .ok_or_else(|| SemioError::InvalidOperation("no tx for undo".into()))?;
            if tx.changes.is_empty() {
                return Ok(if all {
                    TransactionCommandResult::UndoAll { ok: true }
                } else {
                    TransactionCommandResult::Undo { ok: true }
                });
            }
            let ch = tx.changes.pop().expect("pop");
            Some(ch)
        };
        let Some(ch) = done else { break; };
        KitChange::apply_backward(&ch, kit).map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
        {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .sessions
                .get_mut(sid)
                .and_then(|s| s.drafts.get_mut(did))
                .expect("d");
            let tx = d.open_transaction.as_mut().expect("ot");
            tx.redo_changes.push(ch);
        }
        if !all {
            return Ok(TransactionCommandResult::Undo { ok: true });
        }
    }
    Ok(TransactionCommandResult::UndoAll { ok: true })
}

fn transaction_redo(
    kit: &KitStoreRef,
    sid: &Id,
    did: &Id,
    txid: &Id,
    all: bool,
) -> Result<TransactionCommandResult> {
    loop {
        let done = {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .sessions
                .get_mut(sid)
                .and_then(|s| s.drafts.get_mut(did))
                .ok_or_else(|| SemioError::InvalidOperation("no draft".into()))?;
            let tx = d
                .open_transaction
                .as_mut()
                .filter(|t| t.id == *txid)
                .ok_or_else(|| SemioError::InvalidOperation("no tx for redo".into()))?;
            if tx.redo_changes.is_empty() {
                return Ok(if all {
                    TransactionCommandResult::RedoAll { ok: true }
                } else {
                    TransactionCommandResult::Redo { ok: true }
                });
            }
            let ch = tx.redo_changes.pop().expect("rpop");
            Some(ch)
        };
        let Some(ch) = done else { break; };
        KitChange::apply_forward(&ch, kit).map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
        {
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .sessions
                .get_mut(sid)
                .and_then(|s| s.drafts.get_mut(did))
                .expect("d");
            let tx = d.open_transaction.as_mut().expect("ot");
            tx.changes.push(ch);
        }
        if !all {
            return Ok(TransactionCommandResult::Redo { ok: true });
        }
    }
    Ok(TransactionCommandResult::RedoAll { ok: true })
}

fn finalize_draft(
    kit: &KitStoreRef,
    sid: &Id,
    did: &Id,
    message: String,
) -> Result<KitDraftCommandResult> {
    let (parent, alt, before, after) = {
        let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
        let d = g
            .sessions
            .get(sid)
            .and_then(|s| s.drafts.get(did))
            .ok_or_else(|| SemioError::InvalidOperation("no draft to finalize".into()))?;
        if d.open_transaction.is_some() {
            return Err(SemioError::InvalidOperation(
                "open transaction must be closed before FinalizeToKitCheckpoint".into(),
            ));
        }
        let after = g.to_full_dto();
        (
            d.parent_checkpoint.clone(),
            d.target_alternative.clone(),
            d.before.clone(),
            after,
        )
    };
    let kc = KitChange::between(&before, &after).ok_or_else(|| {
        SemioError::InvalidOperation("no change to finalize in draft".into())
    })?;
    let ch = {
        let mut t = kc;
        t.kind = KitChangeKind::Inferred;
        t
    };
    let new_id = Id::new_v7();
    let h = kit_checkpoint::hash_checkpoint(parent.as_ref(), &new_id, std::slice::from_ref(&ch));
    let cp = KitCheckpoint {
        id: new_id.clone(),
        parent: parent.clone(),
        changes: vec![ch],
        message: Some(message),
        time: None,
        authors: vec![],
        hash: h,
        release: None,
    };
    {
        let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
        g.checkpoints.insert(new_id.clone(), cp);
        g.children
            .entry(parent.clone())
            .or_default()
            .push(new_id.clone());
        if let Some(aid) = alt {
            if let Some(a) = g.alternatives.get_mut(&aid) {
                a.checkpoints.push(new_id.clone());
            } else {
                return Err(SemioError::InvalidOperation("target alternative missing".into()));
            }
        } else {
            g.the_kit_head = Some(new_id.clone());
        }
        g.sessions
            .get_mut(sid)
            .expect("s")
            .drafts
            .remove(did);
    }
    Ok(KitDraftCommandResult::FinalizeToKitCheckpoint {
        checkpoint_id: new_id,
    })
}

fn exec_checkpoint(kit: &KitStoreRef, cpid: &Id, cmd: KitCheckpointCommand) -> Result<KitCheckpointCommandResult> {
    match cmd {
        KitCheckpointCommand::ReadKitCommands { commands } => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let dto = materialize_at(&g, Some(cpid));
            let results = read_command::read_kits(&dto, &commands)?;
            Ok(KitCheckpointCommandResult::ReadKitCommands { results })
        }
        KitCheckpointCommand::MarkAsRelease => {
            let init = {
                let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.initial.clone()
            };
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let path =
                kit_checkpoint::checkpoint_chain_root_to_leaf(cpid, &g.checkpoints);
            let mut cl: Vec<KitChange> = Vec::new();
            for id in &path {
                if let Some(c) = g.checkpoints.get(id) {
                    for ch in &c.changes {
                        cl.push(ch.clone());
                    }
                }
            }
            let snapshot =
                kit_checkpoint::materialize_dto(&init, &g.checkpoints, Some(cpid));
            let cp = g.checkpoints.get_mut(cpid).ok_or_else(|| SemioError::NotFound {
                kind: "KitCheckpoint",
                id: cpid.clone(),
            })?;
            let mk = MaterializedKit {
                initial: init,
                change_list: cl,
                computed: Some(snapshot),
            };
            cp.release = Some(mk);
            Ok(KitCheckpointCommandResult::MarkAsRelease { ok: true })
        }
    }
}

fn exec_alternative(kit: &KitStoreRef, aid: &Id, cmd: KitAlternativeCommand) -> Result<KitAlternativeCommandResult> {
    match cmd {
        KitAlternativeCommand::ReadKitCommands { commands } => {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let tip = g
                .alternatives
                .get(aid)
                .and_then(|a| a.checkpoints.last());
            let dto = if let Some(t) = tip {
                materialize_at(&g, Some(t))
            } else {
                the_kit_dto(&g)
            };
            let results = read_command::read_kits(&dto, &commands)?;
            Ok(KitAlternativeCommandResult::ReadKitCommands { results })
        }
        KitAlternativeCommand::UnifyKitCheckpointsToSingleKitCheckpoint { message } => {
            let (root, before_dto, after_dto) = {
                let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
                let alt = g.alternatives.get(aid).ok_or_else(|| {
                    SemioError::NotFound {
                        kind: "KitAlternative",
                        id: aid.clone(),
                    }
                })?;
                if alt.checkpoints.is_empty() {
                    return Err(SemioError::InvalidOperation("empty alternative".into()));
                }
                let r = alt.checkpoints[0].clone();
                let t = alt
                    .checkpoints
                    .last()
                    .cloned()
                    .expect("len checked");
                let a = materialize_at(&g, Some(&r));
                let b = materialize_at(&g, Some(&t));
                (r, a, b)
            };
            let ch = KitChange::between(&before_dto, &after_dto)
                .ok_or_else(|| SemioError::InvalidOperation("no delta to unify".into()))?;
            let new_id = Id::new_v7();
            let h = kit_checkpoint::hash_checkpoint(Some(&root), &new_id, std::slice::from_ref(&ch));
            let mut ch2 = ch;
            ch2.kind = KitChangeKind::UnifyCheckpoints;
            let cp = KitCheckpoint {
                id: new_id.clone(),
                parent: Some(root.clone()),
                changes: vec![ch2],
                message: Some(message),
                time: None,
                authors: vec![],
                hash: h,
                release: None,
            };
            let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.checkpoints.insert(new_id.clone(), cp);
            g.children
                .entry(Some(root.clone()))
                .or_default()
                .push(new_id.clone());
            if let Some(alt) = g.alternatives.get_mut(aid) {
                alt.checkpoints = vec![root, new_id.clone()];
            }
            Ok(
                KitAlternativeCommandResult::UnifyKitCheckpointsToSingleKitCheckpoint {
                    new_checkpoint_id: new_id,
                },
            )
        }
    }
}
