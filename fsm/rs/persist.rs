//! 💾 Versioned persistence — stable string state ids, fingerprint-checked restore.
//!
//! Never serializes JS promises, futures, callbacks, timer handles, or actor
//! references — only the logical configuration, history and context survive.

use crate::kernel::{Snapshot, Status};
use crate::{Configuration, Machine, NodeId};

//#region 🔖Persist

/// 💾 A machine's logical state, addressed by stable string ids so adding/renumbering
/// states never invalidates a previously persisted snapshot.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PersistedSnapshot {
    pub version: u32,
    pub fingerprint: u64,
    pub states: Vec<String>,
    pub history: Vec<(String, Vec<String>)>,
    pub done: bool,
}

/// 💾 Why [`restore`] could not rebuild a [`Snapshot`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreError {
    /// No [`Migration`] bridges the persisted fingerprint to the current machine.
    FingerprintMismatch,
    /// A persisted stable id no longer exists in the current machine.
    UnknownStableId(String),
}

/// 💾 Migrates a [`PersistedSnapshot`] captured under an older machine fingerprint.
pub trait Migration {
    /// The fingerprint this migration accepts as input.
    fn from_version(&self) -> u64;
    /// Produces a [`PersistedSnapshot`] valid under a newer fingerprint.
    fn migrate(&self, snapshot: PersistedSnapshot) -> PersistedSnapshot;
}

/// 💾 Captures a running [`Snapshot`] as a portable, stable-id-addressed value.
pub fn persist<M: Machine>(snapshot: &Snapshot<M>) -> PersistedSnapshot {
    let def = M::definition();
    let states = snapshot.configuration.iter_ones().map(|id| def.nodes[id.0 as usize].stable_id.to_string()).collect();
    let history = snapshot
        .history_entries()
        .iter()
        .map(|(owner, ids)| {
            (
                def.nodes[owner.0 as usize].stable_id.to_string(),
                ids.iter().map(|id| def.nodes[id.0 as usize].stable_id.to_string()).collect(),
            )
        })
        .collect();
    PersistedSnapshot {
        version: 1,
        fingerprint: def.fingerprint,
        states,
        history,
        done: matches!(snapshot.status, Status::Done(_)),
    }
}

fn stable_id_to_node(def_nodes: &[crate::kernel::NodeDef], stable_id: &str) -> Result<NodeId, RestoreError> {
    def_nodes
        .iter()
        .position(|n| n.stable_id == stable_id)
        .map(|idx| NodeId(idx as u16))
        .ok_or_else(|| RestoreError::UnknownStableId(stable_id.to_string()))
}

/// 💾 Rebuilds a [`Snapshot`] from a [`PersistedSnapshot`], applying `migrations` in
/// sequence until the fingerprint matches the current machine, then re-resolving
/// stable ids back to dense [`NodeId`]s. `context` is supplied by the caller since
/// the consumer's `Context` may itself need domain-specific deserialization.
pub fn restore<M: Machine>(persisted: &PersistedSnapshot, context: M::Context, migrations: &[&dyn Migration]) -> Result<Snapshot<M>, RestoreError> {
    let def = M::definition();
    let mut current = persisted.clone();
    while current.fingerprint != def.fingerprint {
        let next = migrations.iter().find(|m| m.from_version() == current.fingerprint);
        match next {
            Some(m) => current = m.migrate(current),
            None => return Err(RestoreError::FingerprintMismatch),
        }
    }
    let mut configuration = M::Config::default();
    for stable_id in &current.states {
        configuration.set(stable_id_to_node(def.nodes, stable_id)?);
    }
    let mut history = Vec::new();
    for (owner_id, ids) in &current.history {
        let owner = stable_id_to_node(def.nodes, owner_id)?;
        let mut resolved = Vec::new();
        for id in ids {
            resolved.push(stable_id_to_node(def.nodes, id)?);
        }
        history.push((owner, resolved));
    }
    let status = Status::Running;
    Ok(Snapshot::from_parts(configuration, context, status, history))
}

//#endregion 🔖Persist

//#region 🧪Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::{init, macrostep};
    use crate::testing::support::{unit_toggle_definition, UnitToggleContext, UnitToggleEvent, UnitToggleMachine};

    #[test]
    fn persist_then_restore_round_trips_active_state() {
        let _ = unit_toggle_definition();
        let mut sink = Vec::new();
        let mut snapshot = init::<UnitToggleMachine>((), &mut sink);
        let mut inspector = crate::inspect::NullInspector;
        macrostep(&mut snapshot, UnitToggleEvent::Flip, &mut sink, &mut inspector);
        assert!(snapshot.matches("on"));

        let persisted = persist(&snapshot);
        assert_eq!(persisted.fingerprint, UnitToggleMachine::definition().fingerprint);
        assert!(persisted.states.iter().any(|s| s == "on"));

        let restored = restore::<UnitToggleMachine>(&persisted, UnitToggleContext::default(), &[]).expect("restore should succeed");
        assert!(restored.matches("on"));
    }

    #[test]
    fn restore_rejects_fingerprint_mismatch_without_migration() {
        let mut sink = Vec::new();
        let snapshot = init::<UnitToggleMachine>((), &mut sink);
        let mut persisted = persist(&snapshot);
        persisted.fingerprint = 9999;
        let result = restore::<UnitToggleMachine>(&persisted, UnitToggleContext::default(), &[]);
        assert!(matches!(result, Err(RestoreError::FingerprintMismatch)));
    }

    struct BumpFingerprint;
    impl Migration for BumpFingerprint {
        fn from_version(&self) -> u64 {
            9999
        }
        fn migrate(&self, mut snapshot: PersistedSnapshot) -> PersistedSnapshot {
            snapshot.fingerprint = UnitToggleMachine::definition().fingerprint;
            snapshot
        }
    }

    #[test]
    fn restore_applies_migration_chain_until_fingerprint_matches() {
        let mut sink = Vec::new();
        let snapshot = init::<UnitToggleMachine>((), &mut sink);
        let mut persisted = persist(&snapshot);
        persisted.fingerprint = 9999;
        let migration = BumpFingerprint;
        let migrations: &[&dyn Migration] = &[&migration];
        let restored = restore::<UnitToggleMachine>(&persisted, UnitToggleContext::default(), migrations).expect("migration should bridge fingerprint");
        assert!(restored.matches("off"));
    }
}

//#endregion 🧪Tests
