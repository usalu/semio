//! 🔁 Causal sync session: feeds remote {@link OpEnvelope}s into a document's vcs edit timeline.

use vcs::{reconcile_alternative, DocumentVcsStore, Operation};

//#region 🔖Errors
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SyncError {
    #[error("vcs error: {0}")]
    Vcs(String),
}
//#endregion 🔖Errors

//#region 🔖SyncSession
/// @emoji 🔁 Pairs a document's vcs store with the causal DAG that reconciles remote envelopes into it.
pub struct SyncSession<P, Op>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned,
    Op: Clone + serde::Serialize + serde::de::DeserializeOwned + Operation<P>,
{
    pub store: DocumentVcsStore<P, Op>,
}

impl<P, Op> SyncSession<P, Op>
where
    P: Clone + serde::Serialize + serde::de::DeserializeOwned,
    Op: Clone + serde::Serialize + serde::de::DeserializeOwned + Operation<P>,
{
    pub fn new(store: DocumentVcsStore<P, Op>) -> Self {
        Self { store }
    }

    /// @emoji 🕸️ Feeds a remote envelope through the store's causal DAG, materializing it (and any
    /// now-unblocked dependents) into the edit timeline.
    pub fn receive(&mut self, envelope: semio_framework_core::OpEnvelope) -> Result<(), SyncError> {
        self.store
            .ingest_remote(envelope)
            .map_err(|error| SyncError::Vcs(error.to_string()))
    }

    pub fn reconcile_branch(
        &mut self,
        alternative_name: &str,
        message: Option<String>,
        authors: Vec<vcs::Author>,
    ) -> Result<String, SyncError> {
        let mut envelope = self.store.envelope().clone();
        let alternative_id = reconcile_alternative(&mut envelope, alternative_name, message, authors)
            .map_err(|error| SyncError::Vcs(error.to_string()))?;
        let applied = self.store.applied_edit_ids().to_vec();
        let redo = self.store.redo_edit_ids().to_vec();
        self.store.set_state(envelope, applied, redo);
        Ok(alternative_id)
    }
}
//#endregion 🔖SyncSession

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use vcs::{create_document_vcs_envelope, op_envelope_from_edit, Edit, OperationDiff};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoProjection {
        n: i32,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoDiff {
        n: Option<i32>,
    }

    impl OperationDiff<DemoProjection> for DemoDiff {
        fn apply(&self, projection: &DemoProjection) -> DemoProjection {
            DemoProjection {
                n: self.n.unwrap_or(projection.n),
            }
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "op")]
    enum DemoOp {
        SetN { n: i32 },
    }

    impl Operation<DemoProjection> for DemoOp {
        type Diff = DemoDiff;

        fn diff(&self, _projection: &DemoProjection) -> DemoDiff {
            match self {
                DemoOp::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }

        fn backwards(&self, projection: &DemoProjection) -> Vec<Self> {
            vec![DemoOp::SetN { n: projection.n }]
        }
    }

    fn sample_op_envelope(edit_id: &str, n: i32) -> semio_framework_core::OpEnvelope {
        let edit = Edit {
            id: edit_id.into(),
            forwards: vec![DemoOp::SetN { n }],
            backwards: vec![DemoOp::SetN { n: 0 }],
            operation_meta: Vec::new(),
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "0".into(),
            finished_at: None,
        };
        let placeholder: vcs::DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        op_envelope_from_edit(&placeholder, &edit, Vec::new()).expect("op envelope")
    }

    #[test]
    fn receive_materializes_remote_envelope_into_the_edit_timeline() {
        let envelope: vcs::DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let store = DocumentVcsStore::new(envelope);
        let mut session = SyncSession::new(store);
        session.receive(sample_op_envelope("edit-1", 5)).expect("receive");
        assert_eq!(session.store.projection().expect("projection").n, 5);
        assert_eq!(session.store.envelope().vcs.edits.len(), 1);
    }

    #[test]
    fn receive_buffers_out_of_order_envelopes_until_dependencies_arrive() {
        let envelope: vcs::DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let store = DocumentVcsStore::new(envelope);
        let mut session = SyncSession::new(store);
        let mut second = sample_op_envelope("edit-2", 9);
        second.deps = vec![semio_framework_core::OperationId("edit-1".into())];
        session.receive(second).expect("receive second first");
        assert_eq!(session.store.envelope().vcs.edits.len(), 0, "buffered until edit-1 arrives");
        session.receive(sample_op_envelope("edit-1", 5)).expect("receive first");
        assert_eq!(session.store.envelope().vcs.edits.len(), 2, "both edits now applied");
        assert_eq!(session.store.projection().expect("projection").n, 9);
    }
}
