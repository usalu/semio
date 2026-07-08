//! 🔁 Operation DAG sync: causal envelope exchange over vcs edit timelines.

use semio_framework_core::OpEnvelope;
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use vcs::{reconcile_alternative, DocumentVcsEnvelope, VcsError};

//#region 🔖Errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SyncError {
    #[error("duplicate operation id: {0}")]
    Duplicate(String),
    #[error("missing dependency: {0}")]
    MissingDependency(String),
    #[error("vcs error: {0}")]
    Vcs(String),
}
//#endregion 🔖Errors

//#region 🔖OpDag
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpDag {
    envelopes: HashMap<String, OpEnvelope>,
    applied: HashSet<String>,
    pending: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InsertResult {
    Applied,
    Pending,
    AlreadyApplied,
}

impl OpDag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, envelope: OpEnvelope) -> Result<InsertResult, SyncError> {
        let id = envelope.id.0.clone();
        if self.applied.contains(&id) {
            return Ok(InsertResult::AlreadyApplied);
        }
        if self.envelopes.contains_key(&id) {
            return Err(SyncError::Duplicate(id));
        }
        for dependency in &envelope.deps {
            if !self.applied.contains(&dependency.0) && !self.envelopes.contains_key(&dependency.0) {
                self.envelopes.insert(id.clone(), envelope);
                if !self.pending.contains(&id) {
                    self.pending.push(id);
                }
                return Ok(InsertResult::Pending);
            }
        }
        self.envelopes.insert(id.clone(), envelope);
        self.mark_applied(&id);
        self.drain_ready();
        Ok(InsertResult::Applied)
    }

    pub fn ready(&self) -> Vec<&OpEnvelope> {
        self.pending
            .iter()
            .filter_map(|id| self.envelopes.get(id))
            .filter(|envelope| {
                envelope
                    .deps
                    .iter()
                    .all(|dependency| self.applied.contains(&dependency.0))
            })
            .collect()
    }

    pub fn applied_ids(&self) -> Vec<String> {
        self.applied.iter().cloned().collect()
    }

    fn mark_applied(&mut self, id: &str) {
        self.applied.insert(id.to_string());
        self.pending.retain(|pending| pending != id);
    }

    fn drain_ready(&mut self) {
        loop {
            let ready: Vec<String> = self
                .pending
                .iter()
                .filter(|id| {
                    self.envelopes
                        .get(*id)
                        .is_some_and(|envelope| {
                            envelope
                                .deps
                                .iter()
                                .all(|dependency| self.applied.contains(&dependency.0))
                        })
                })
                .cloned()
                .collect();
            if ready.is_empty() {
                break;
            }
            for id in ready {
                self.mark_applied(&id);
            }
        }
    }
}
//#endregion 🔖OpDag

//#region 🔖SyncSession
pub struct SyncSession<P, Op>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned,
    Op: Clone + serde::Serialize + serde::de::DeserializeOwned,
{
    pub dag: OpDag,
    pub envelope: DocumentVcsEnvelope<P, Op>,
}

impl<P, Op> SyncSession<P, Op>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned,
    Op: Clone + serde::Serialize + serde::de::DeserializeOwned + vcs::Operation<P>,
{
    pub fn new(envelope: DocumentVcsEnvelope<P, Op>) -> Self {
        Self {
            dag: OpDag::new(),
            envelope,
        }
    }

    pub fn receive(&mut self, envelope: OpEnvelope) -> Result<InsertResult, SyncError> {
        self.dag.insert(envelope)
    }

    pub fn reconcile_branch(
        &mut self,
        alternative_name: &str,
        message: Option<String>,
        authors: Vec<vcs::Author>,
    ) -> Result<String, SyncError> {
        reconcile_alternative(&mut self.envelope, alternative_name, message, authors)
            .map_err(|error| SyncError::Vcs(error.to_string()))
    }
}
//#endregion 🔖SyncSession

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_core::{
        ActorId, InverseOperation, ModelDiff, ModelId, OperationId, PayloadHash, SchemaId, SchemaVersion,
        UndoPolicy,
    };
    use serde_json::json;

    fn sample_envelope(id: &str, deps: Vec<&str>) -> OpEnvelope {
        OpEnvelope {
            id: OperationId(id.into()),
            actor: ActorId("actor-1".into()),
            model: ModelId("model-1".into()),
            schema_version: SchemaVersion("test.v1".into()),
            deps: deps.into_iter().map(|dep| OperationId(dep.into())).collect(),
            payload_hash: PayloadHash("hash".into()),
            diff: ModelDiff {
                schema_id: SchemaId("diff.v1".into()),
                payload: json!({"value": id}),
            },
            inverse: InverseOperation {
                target_operation: OperationId(id.into()),
                inverse_diff: ModelDiff {
                    schema_id: SchemaId("diff.v1".into()),
                    payload: json!({}),
                },
                base_version: semio_framework_core::ModelVersion(0),
                dependencies: Vec::new(),
                undo_policy: UndoPolicy::ExactBaseOnly,
            },
        }
    }

    #[test]
    fn inserts_pending_until_dependencies_arrive() {
        let mut dag = OpDag::new();
        assert!(matches!(
            dag.insert(sample_envelope("op-2", vec!["op-1"])),
            Ok(InsertResult::Pending)
        ));
        assert!(matches!(
            dag.insert(sample_envelope("op-1", vec![])),
            Ok(InsertResult::Applied)
        ));
        assert_eq!(dag.applied_ids().len(), 2);
    }
}
