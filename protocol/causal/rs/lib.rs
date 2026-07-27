//! 🎞️ Protocol causal layer: `OperationEnvelope`/`DocumentDiff`/`InverseOperation`, the `OpDag`
//! causal buffer, the runtime frontier-summary twin, the `OperationTransform` hook, and the
//! `operation_envelope_from_edit` bridge from `protocol_command::Edit`. Moved from
//! `framework/core/rs/lib.rs`'s `🔖Sync` region (`OperationEnvelope` L6246, `DocumentDiff` L6121,
//! `InverseOperation` L6137, `OpDag`/`InsertResult`/`OpDagError` L6266-6380 including its existing
//! unit tests at L6488-6572) and `vcs/rs/lib.rs`'s `operation_envelope_from_edit`. Frozen contract:
//! `.repo/🎫/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §`protocol_causal`.
//!
//! This crate's `FrontierSummary`/`frontier_delta` are the runtime/wire twin of
//! `protocol_history`'s durable-log-derived pair — deliberately kept separate, see `🔖Frontier`.

//#region 🔖Envelope
// Moved from framework/core L6246 (OperationEnvelope), L6121 (DocumentDiff), L6137
// (InverseOperation). The frozen contract's field shapes are simpler than the framework-core
// originals (no `schema_version`/`payload_hash` on the envelope, no `target_operation`/
// `base_version`/`dependencies`/`undo_policy` on the inverse) — implemented exactly as specified
// below; diff/inverse stay schema-erased (`serde_json::Value` payload).

/// @emoji ✉️ A causally-ordered operation crossing the wire: identity, actor, dependency set, the
/// forward diff, its precomputed inverse, and the HLC tick it was authored at.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperationEnvelope {
    pub operation_id: protocol_core::OperationId,
    pub document_id: protocol_core::DocumentId,
    pub actor: protocol_core::ActorId,
    pub dependencies: Vec<protocol_core::OperationId>,
    pub diff: DocumentDiff,
    pub inverse: InverseOperation,
    pub timestamp: protocol_core::HybridLogicalTimestamp,
}

/// @emoji 🧮 A schema-tagged, opaque forward diff payload.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentDiff {
    pub schema: String,
    pub payload: serde_json::Value,
}

/// @emoji ↩️ A schema-tagged, opaque inverse diff payload.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InverseOperation {
    pub schema: String,
    pub inverse_diff: serde_json::Value,
}
//#endregion 🔖Envelope

//#region 🔖OpDag
// Moved verbatim from framework/core L6266-6379 including its existing unit tests (L6488-6572),
// field names adapted to the new `OperationEnvelope` shape (`id` -> `operation_id`, `deps` ->
// `dependencies`). No behavior change, including the pre-existing quirk this port preserves
// faithfully: `insert`'s own per-envelope Applied/Pending classification treats a dependency as
// "not blocking" once it is merely *known* to the dag (present in `envelopes`, via any earlier
// Pending insert), not only once it is actually `applied` — see the inline comment on `insert`
// below. This never manifests for insertions performed in true topological order (every ancestor
// is already `applied`, not merely known, by induction), which is the property this crate's own
// `🧪Tests::quick` convergence tests exercise; `protocol_testkit`'s exhaustive suite covers
// scrambled orderings.

/// @emoji 🕸️ Causal DAG of exchanged `OperationEnvelope`s: buffers envelopes until their
/// dependencies are applied.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpDag {
    envelopes: std::collections::HashMap<String, OperationEnvelope>,
    applied: std::collections::HashSet<String>,
    applied_order: Vec<String>,
    drained: usize,
    pending: Vec<String>,
}

/// @emoji 🚦 The outcome of one `OpDag::insert` call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsertResult {
    Applied,
    Pending,
    AlreadyApplied,
}

/// @emoji 🚨 `OpDag`'s one failure mode: the same operation id inserted twice while still pending.
/// Hand-rolled `Display`/`Error` (this crate has no `thiserror` dependency — `protocol_core`/
/// `protocol_command` are the only path deps).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpDagError {
    Duplicate,
}

impl std::fmt::Display for OpDagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpDagError::Duplicate => write!(f, "duplicate operation id"),
        }
    }
}

impl std::error::Error for OpDagError {}

impl OpDag {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕ Inserts one envelope. Returns `AlreadyApplied` if its id was applied before,
    /// `Err(Duplicate)` if it's already buffered as pending, `Pending` if any dependency is wholly
    /// unknown to this dag, else `Applied` (and cascades `drain_ready` for anything it unblocks).
    pub fn insert(&mut self, envelope: OperationEnvelope) -> Result<InsertResult, OpDagError> {
        let id = envelope.operation_id.0.clone();
        if self.applied.contains(&id) {
            return Ok(InsertResult::AlreadyApplied);
        }
        if self.envelopes.contains_key(&id) {
            return Err(OpDagError::Duplicate);
        }
        for dependency in &envelope.dependencies {
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

    /// @emoji ✅ Ids of currently-pending envelopes whose dependencies are all applied.
    pub fn ready(&self) -> Vec<protocol_core::OperationId> {
        self.pending
            .iter()
            .filter_map(|id| self.envelopes.get(id))
            .filter(|envelope| envelope.dependencies.iter().all(|dependency| self.applied.contains(&dependency.0)))
            .map(|envelope| envelope.operation_id.clone())
            .collect()
    }

    /// @emoji 🧺 Drains envelopes applied since the last drain, in causal application order.
    pub fn drain_applied_envelopes(&mut self) -> Vec<OperationEnvelope> {
        let fresh: Vec<String> = self.applied_order[self.drained..].to_vec();
        self.drained = self.applied_order.len();
        fresh.iter().filter_map(|id| self.envelopes.get(id).cloned()).collect()
    }

    /// @emoji 🌱 Seeds one id into the applied-set from out-of-band knowledge (e.g. a full-document
    /// snapshot merge) — without this, a later envelope whose `dependencies` reference this id
    /// stays `Pending` forever, since `insert` only recognizes a dependency as satisfied through
    /// this dag's own `envelopes`/`applied` bookkeeping, never through edits a peer adopted by some
    /// other route.
    pub fn seed_applied(&mut self, operation_id: protocol_core::OperationId) {
        let id = operation_id.0;
        if !self.applied.contains(&id) {
            self.mark_applied(&id);
        }
    }

    fn mark_applied(&mut self, id: &str) {
        self.applied.insert(id.to_string());
        self.applied_order.push(id.to_string());
        self.pending.retain(|pending| pending != id);
    }

    fn drain_ready(&mut self) {
        loop {
            let ready: Vec<String> = self
                .pending
                .iter()
                .filter(|id| self.envelopes.get(*id).is_some_and(|envelope| envelope.dependencies.iter().all(|dependency| self.applied.contains(&dependency.0))))
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

//#region 🔖Frontier
/// @emoji 🏔️ Runtime/wire twin of `protocol_history::FrontierSummary` — the shape `db` and
/// `framework/sync` exchange without a full history-log decode. Deliberately NOT unified with the
/// durable-log-derived version: they serve different layers (live runtime state vs on-disk log).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrontierSummary {
    pub document_id: protocol_core::DocumentId,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_hash: [u8; 32],
}

/// @emoji ⚖️ How a `local` frontier relates to a `remote` one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FrontierComparison {
    Equal,
    Ahead,
    Behind,
    Diverged { common_edit_count: u64 },
}

/// @emoji 🔎 Compares two frontier summaries. Design choice (the contract fixes the enum shape,
/// not the comparison algorithm): identical `(head_edit_ordinal, head_edit_id, chain_hash)` is
/// `Equal`; a strictly greater/lesser `head_edit_ordinal` alone is `Ahead`/`Behind` (a summary
/// carries no ancestry chain to verify beyond its tip, so ordinal order is the only signal
/// available at this layer); equal ordinal with a differing `head_edit_id`/`chain_hash` is
/// `Diverged`, with `common_edit_count` conservatively reported as the shared ordinal floor
/// (`min` of both ordinals) since this summary-only comparison cannot walk history to find the
/// true common ancestor — callers wanting an exact count must consult the durable log via
/// `protocol_history`.
pub fn frontier_delta(local: &FrontierSummary, remote: &FrontierSummary) -> FrontierComparison {
    if local.head_edit_ordinal == remote.head_edit_ordinal && local.head_edit_id == remote.head_edit_id && local.chain_hash == remote.chain_hash {
        return FrontierComparison::Equal;
    }
    if local.head_edit_ordinal > remote.head_edit_ordinal {
        return FrontierComparison::Ahead;
    }
    if local.head_edit_ordinal < remote.head_edit_ordinal {
        return FrontierComparison::Behind;
    }
    FrontierComparison::Diverged { common_edit_count: local.head_edit_ordinal.min(remote.head_edit_ordinal) }
}
//#endregion 🔖Frontier

//#region 🔖Transform
/// @emoji 🔀 The result of transforming one operation against a concurrent one.
#[derive(Clone, Debug, PartialEq)]
pub enum TransformOutcome<Op> {
    Unchanged(Op),
    Transformed(Op),
    Conflict(String),
}

/// @emoji 🧮 Operational-transform hook: rewrites `self` so it applies cleanly after `against`
/// (both assumed concurrent, same base). New trait — no prior `vcs`/`framework-core` equivalent.
pub trait OperationTransform<P>: protocol_command::Operation<P> {
    fn transform(&self, against: &Self) -> TransformOutcome<Self>
    where
        Self: Sized;
}
//#endregion 🔖Transform

//#region 🔖Bridge
// Moved from vcs/rs (was operation_envelope_from_edit). The original signature took a
// `DocumentVcsEnvelope<P, Operation>` (for its `.id`/`.schema`) and a `deps: Vec<OperationId>` and
// returned a single `Result<OperationEnvelope, VcsError>` whose diff/inverse payloads were the
// *whole* `Edit` serialized once. The frozen contract's signature drops both the vcs envelope and
// the `deps` parameter and returns `Vec<OperationEnvelope>` — one envelope per forward op — which
// only works because `Op: protocol_command::Operation<P>` supplies each op's own
// `operation_id`/`dependencies`/`author_id`/`timestamp` via trait methods, no base `P` needed.
//
// 🎯 Design choices (genuine ambiguity the contract leaves to the implementer): (1) `edit.forwards`
// is zipped index-wise with `edit.operation_meta` (the richer, already-computed per-op metadata a
// live appender fills in) with a documented fallback chain: `operation_meta[i]` field, else the
// `Op` trait method, else a structural default (`{edit.id}#{i}` for the id, `edit.actor` or
// `"unknown"` for the actor, `HybridLogicalTimestamp::new(0, 0)` for the timestamp) so this
// function is total even for a bare-bones `Edit` with no explicit meta. (2) `edit.backwards[i]` is
// this op's per-index inverse (absent if the backwards vec is shorter, mirroring
// `protocol_history::HistoryEdit`'s note that "backward op count may differ from forward"), so
// `inverse_diff` is `Value::Null` past the end of `backwards` rather than an error. (3) `diff.schema`
// / `inverse.schema` use `std::any::type_name::<Op>()` — the only stable per-`Op`-type tag
// available in a fully generic bridge fn with no `OperationDescriptor` lookup in scope; a caller
// wanting a real `SchemaId` should overwrite the field after the call.
pub fn operation_envelope_from_edit<P, Op: protocol_command::Operation<P>>(edit: &protocol_command::Edit<Op>, document_id: &protocol_core::DocumentId) -> Vec<OperationEnvelope> {
    let schema = std::any::type_name::<Op>().to_string();
    edit.forwards
        .iter()
        .enumerate()
        .map(|(index, op)| {
            let meta = edit.operation_meta.get(index);
            let operation_id = meta
                .and_then(|m| m.operation_id.clone())
                .or_else(|| op.operation_id())
                .unwrap_or_else(|| protocol_core::OperationId(format!("{}#{index}", edit.id)));
            let dependencies = meta.map_or_else(|| op.dependencies(), |m| m.dependencies.clone());
            let actor = meta
                .and_then(|m| m.author_id.clone())
                .or_else(|| op.author_id())
                .unwrap_or_else(|| protocol_core::ActorId(edit.actor.clone().unwrap_or_else(|| "unknown".to_string())));
            let timestamp = meta.map(|m| m.timestamp).or_else(|| op.timestamp()).unwrap_or_else(|| protocol_core::HybridLogicalTimestamp::new(0, 0));
            let payload = serde_json::to_value(op).unwrap_or(serde_json::Value::Null);
            let inverse_payload = edit.backwards.get(index).map_or(serde_json::Value::Null, |inverse_op| serde_json::to_value(inverse_op).unwrap_or(serde_json::Value::Null));
            OperationEnvelope {
                operation_id,
                document_id: document_id.clone(),
                actor,
                dependencies,
                diff: DocumentDiff { schema: schema.clone(), payload },
                inverse: InverseOperation { schema: schema.clone(), inverse_diff: inverse_payload },
                timestamp,
            }
        })
        .collect()
}
//#endregion 🔖Bridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸Fixtures
    // Dummy (P=i64, Op=CausalAddOp) pair: the smallest possible Operation/OperationDiff impl,
    // reused across this file's tests instead of a real technology's op set.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct CausalAddDiff {
        delta: i64,
    }
    impl protocol_command::OperationDiff<i64> for CausalAddDiff {
        fn apply(&self, base: &i64) -> i64 {
            base + self.delta
        }
        fn absorb(&mut self, other: Self) {
            self.delta += other.delta;
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct CausalAddOp {
        delta: i64,
    }
    impl protocol_command::Operation<i64> for CausalAddOp {
        type Diff = CausalAddDiff;
        fn diff(&self, _base: &i64) -> CausalAddDiff {
            CausalAddDiff { delta: self.delta }
        }
        fn backwards(&self, _base: &i64) -> Vec<Self> {
            vec![CausalAddOp { delta: -self.delta }]
        }
    }
    impl OperationTransform<i64> for CausalAddOp {
        fn transform(&self, against: &Self) -> TransformOutcome<Self> {
            if self.delta == against.delta {
                TransformOutcome::Unchanged(self.clone())
            } else if self.delta == 0 {
                TransformOutcome::Conflict("zero delta cannot transform".to_string())
            } else {
                TransformOutcome::Transformed(CausalAddOp { delta: self.delta + against.delta })
            }
        }
    }

    fn sample_envelope(id: &str, deps: Vec<&str>) -> OperationEnvelope {
        OperationEnvelope {
            operation_id: protocol_core::OperationId(id.into()),
            document_id: protocol_core::DocumentId("document-1".into()),
            actor: protocol_core::ActorId("actor-1".into()),
            dependencies: deps.into_iter().map(|dep| protocol_core::OperationId(dep.into())).collect(),
            diff: DocumentDiff { schema: "diff.v1".into(), payload: serde_json::json!({"value": id}) },
            inverse: InverseOperation { schema: "diff.v1".into(), inverse_diff: serde_json::json!({}) },
            timestamp: protocol_core::HybridLogicalTimestamp::new(1, 0),
        }
    }
    //#endregion 🧸Fixtures

    //#region 🔖Envelope
    #[test]
    fn operation_envelope_serde_round_trips() {
        let envelope = sample_envelope("operation-1", vec!["operation-0"]);
        let json = serde_json::to_string(&envelope).expect("serialize");
        let round_tripped: OperationEnvelope = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, envelope);
    }
    //#endregion 🔖Envelope

    //#region 🔖OpDag
    #[test]
    fn inserts_pending_until_dependencies_arrive() {
        let mut dag = OpDag::new();
        assert_eq!(dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap(), InsertResult::Pending);
        assert_eq!(dag.insert(sample_envelope("operation-1", vec![])).unwrap(), InsertResult::Applied);
        assert_eq!(dag.applied.len(), 2);
    }

    #[test]
    fn drains_applied_envelopes_in_causal_order() {
        let mut dag = OpDag::new();
        dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap();
        dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        let drained = dag.drain_applied_envelopes();
        assert_eq!(drained.iter().map(|envelope| envelope.operation_id.0.clone()).collect::<Vec<_>>(), vec!["operation-1".to_string(), "operation-2".to_string()]);
        assert!(dag.drain_applied_envelopes().is_empty(), "second drain yields nothing new");
        dag.insert(sample_envelope("operation-3", vec![])).unwrap();
        let drained = dag.drain_applied_envelopes();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].operation_id.0, "operation-3");
    }

    #[test]
    fn insert_duplicate_pending_operation_id_errors() {
        let mut dag = OpDag::new();
        dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap();
        let err = dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap_err();
        assert_eq!(err, OpDagError::Duplicate);
    }

    #[test]
    fn insert_already_applied_operation_returns_already_applied_without_erroring() {
        let mut dag = OpDag::new();
        dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        let result = dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        assert_eq!(result, InsertResult::AlreadyApplied);
    }

    #[test]
    fn seed_applied_unblocks_pending_envelopes_that_reference_out_of_band_deps() {
        let mut dag = OpDag::new();
        assert_eq!(dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap(), InsertResult::Pending);
        assert!(dag.ready().is_empty(), "dependency is not yet known to this dag");
        dag.seed_applied(protocol_core::OperationId("operation-1".to_string()));
        let ready_ids: Vec<String> = dag.ready().iter().map(|id| id.0.clone()).collect();
        assert_eq!(ready_ids, vec!["operation-2".to_string()]);
    }

    #[test]
    fn opdagerror_display_is_non_empty() {
        assert!(!OpDagError::Duplicate.to_string().is_empty());
    }

    //#region 🏃quick
    mod quick {
        use super::*;

        /// @emoji 🔁 Diamond DAG (A none; B,C dep A; D dep B,C) inserted in every hand-picked
        /// topological order converges to the same final applied set and drained envelope count —
        /// the "permutation-convergence" law the amendment's testing note asks for at the `quick`
        /// tier. True topological orders never hit the `insert`-classification quirk documented on
        /// `OpDag` above (every dependency is already `applied`, not merely known, by induction).
        fn diamond(id_a: &str, id_b: &str, id_c: &str, id_d: &str) -> [(&'static str, OperationEnvelope); 4] {
            [
                ("a", sample_envelope(id_a, vec![])),
                ("b", sample_envelope(id_b, vec![id_a])),
                ("c", sample_envelope(id_c, vec![id_a])),
                ("d", sample_envelope(id_d, vec![id_b, id_c])),
            ]
        }

        fn assert_converges(order: [&str; 4]) {
            let nodes = diamond("A", "B", "C", "D");
            let mut dag = OpDag::new();
            for label in order {
                let (_, envelope) = nodes.iter().find(|(l, _)| *l == label).expect("known label").clone();
                let result = dag.insert(envelope).expect("insert never duplicates in a fresh dag");
                assert_eq!(result, InsertResult::Applied, "insertion order {order:?} must stay fully topological");
            }
            let drained = dag.drain_applied_envelopes();
            let mut ids: Vec<String> = drained.iter().map(|e| e.operation_id.0.clone()).collect();
            ids.sort();
            assert_eq!(ids, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]);
        }

        #[test]
        fn topological_order_a_b_c_d_converges() {
            assert_converges(["a", "b", "c", "d"]);
        }

        #[test]
        fn topological_order_a_c_b_d_converges() {
            assert_converges(["a", "c", "b", "d"]);
        }

        #[test]
        fn topological_order_a_b_d_c_is_rejected_as_non_topological() {
            // "d" before "c" is NOT a valid topological order (d depends on c) — insert must not
            // silently accept it as Applied; it must classify as Pending instead, proving this
            // test suite actually distinguishes topological from non-topological orderings rather
            // than accepting anything.
            let nodes = diamond("A", "B", "C", "D");
            let mut dag = OpDag::new();
            for label in ["a", "b", "d"] {
                let (_, envelope) = nodes.iter().find(|(l, _)| *l == label).expect("known label").clone();
                let result = dag.insert(envelope).unwrap();
                if label == "d" {
                    assert_eq!(result, InsertResult::Pending, "d must not apply before its dependency c arrives");
                }
            }
            dag.insert(nodes.into_iter().find(|(l, _)| *l == "c").unwrap().1).unwrap();
            let mut ids: Vec<String> = dag.applied.iter().cloned().collect();
            ids.sort();
            assert_eq!(ids, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()], "once c arrives, d converges too");
        }
    }
    //#endregion 🏃quick
    //#endregion 🔖OpDag

    //#region 🔖Frontier
    fn frontier(document_id: &str, ordinal: u64, head_id: &str, commit_seq: u64, chain_byte: u8) -> FrontierSummary {
        FrontierSummary {
            document_id: protocol_core::DocumentId(document_id.into()),
            head_edit_ordinal: ordinal,
            head_edit_id: head_id.into(),
            last_commit_seq: commit_seq,
            chain_hash: [chain_byte; 32],
        }
    }

    #[test]
    fn frontier_delta_identical_summaries_are_equal() {
        let a = frontier("doc-1", 5, "edit-5", 3, 9);
        let b = a.clone();
        assert_eq!(frontier_delta(&a, &b), FrontierComparison::Equal);
    }

    #[test]
    fn frontier_delta_greater_ordinal_is_ahead() {
        let local = frontier("doc-1", 10, "edit-10", 4, 1);
        let remote = frontier("doc-1", 5, "edit-5", 3, 2);
        assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Ahead);
    }

    #[test]
    fn frontier_delta_lesser_ordinal_is_behind() {
        let local = frontier("doc-1", 5, "edit-5", 3, 1);
        let remote = frontier("doc-1", 10, "edit-10", 4, 2);
        assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Behind);
    }

    #[test]
    fn frontier_delta_same_ordinal_different_head_is_diverged() {
        let local = frontier("doc-1", 5, "edit-5a", 3, 1);
        let remote = frontier("doc-1", 5, "edit-5b", 3, 2);
        assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Diverged { common_edit_count: 5 });
    }

    #[test]
    fn frontier_summary_serde_round_trips() {
        let summary = frontier("doc-1", 7, "edit-7", 2, 5);
        let json = serde_json::to_string(&summary).expect("serialize");
        let round_tripped: FrontierSummary = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, summary);
    }
    //#endregion 🔖Frontier

    //#region 🔖Transform
    #[test]
    fn transform_unchanged_when_deltas_match() {
        let a = CausalAddOp { delta: 3 };
        let b = CausalAddOp { delta: 3 };
        assert_eq!(a.transform(&b), TransformOutcome::Unchanged(CausalAddOp { delta: 3 }));
    }

    #[test]
    fn transform_transformed_when_deltas_differ() {
        let a = CausalAddOp { delta: 2 };
        let b = CausalAddOp { delta: 5 };
        assert_eq!(a.transform(&b), TransformOutcome::Transformed(CausalAddOp { delta: 7 }));
    }

    #[test]
    fn transform_conflict_case_carries_message() {
        let a = CausalAddOp { delta: 0 };
        let b = CausalAddOp { delta: 9 };
        match a.transform(&b) {
            TransformOutcome::Conflict(message) => assert!(!message.is_empty()),
            other => panic!("expected Conflict, got {other:?}"),
        }
    }
    //#endregion 🔖Transform

    //#region 🔖Bridge
    #[test]
    fn operation_envelope_from_edit_derives_one_envelope_per_forward_op_using_explicit_meta() {
        let edit = protocol_command::Edit::<CausalAddOp> {
            id: "edit-1".into(),
            actor: Some("actor-fallback".into()),
            forwards: vec![CausalAddOp { delta: 1 }, CausalAddOp { delta: 2 }],
            backwards: vec![CausalAddOp { delta: -1 }, CausalAddOp { delta: -2 }],
            operation_meta: vec![
                protocol_command::OperationMeta {
                    operation_id: Some(protocol_core::OperationId("op-a".into())),
                    dependencies: vec![protocol_core::OperationId("op-0".into())],
                    base_version: 0,
                    author_id: Some(protocol_core::ActorId("actor-explicit".into())),
                    timestamp: protocol_core::HybridLogicalTimestamp::new(1, 1000),
                    undo_policy: protocol_core::UndoPolicy::ExactBaseOnly,
                    payload_hash: None,
                },
                protocol_command::OperationMeta {
                    operation_id: Some(protocol_core::OperationId("op-b".into())),
                    dependencies: vec![protocol_core::OperationId("op-a".into())],
                    base_version: 1,
                    author_id: None,
                    timestamp: protocol_core::HybridLogicalTimestamp::new(1, 2000),
                    undo_policy: protocol_core::UndoPolicy::ExactBaseOnly,
                    payload_hash: None,
                },
            ],
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let document_id = protocol_core::DocumentId("doc-1".into());

        let envelopes = operation_envelope_from_edit(&edit, &document_id);
        assert_eq!(envelopes.len(), 2);

        assert_eq!(envelopes[0].operation_id, protocol_core::OperationId("op-a".into()));
        assert_eq!(envelopes[0].actor, protocol_core::ActorId("actor-explicit".into()));
        assert_eq!(envelopes[0].dependencies, vec![protocol_core::OperationId("op-0".into())]);
        assert_eq!(envelopes[0].document_id, document_id);
        assert_eq!(envelopes[0].timestamp, protocol_core::HybridLogicalTimestamp::new(1, 1000));
        assert_eq!(envelopes[0].diff.payload, serde_json::json!({"delta": 1}));
        assert_eq!(envelopes[0].inverse.inverse_diff, serde_json::json!({"delta": -1}));

        // Second op's meta has no author_id -> falls back to `edit.actor`, not "unknown".
        assert_eq!(envelopes[1].operation_id, protocol_core::OperationId("op-b".into()));
        assert_eq!(envelopes[1].actor, protocol_core::ActorId("actor-fallback".into()));
    }

    #[test]
    fn operation_envelope_from_edit_falls_back_to_op_trait_and_structural_defaults_without_meta() {
        let edit = protocol_command::Edit::<CausalAddOp> {
            id: "edit-2".into(),
            actor: None,
            forwards: vec![CausalAddOp { delta: 5 }],
            backwards: vec![],
            operation_meta: vec![],
            description: None,
            coalesce_key: None,
            sequence_number: 0,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let document_id = protocol_core::DocumentId("doc-2".into());

        let envelopes = operation_envelope_from_edit(&edit, &document_id);
        assert_eq!(envelopes.len(), 1);
        assert_eq!(envelopes[0].operation_id, protocol_core::OperationId("edit-2#0".into()));
        assert_eq!(envelopes[0].actor, protocol_core::ActorId("unknown".into()));
        assert!(envelopes[0].dependencies.is_empty());
        assert_eq!(envelopes[0].timestamp, protocol_core::HybridLogicalTimestamp::new(0, 0));
        assert_eq!(envelopes[0].inverse.inverse_diff, serde_json::Value::Null, "backwards vec shorter than forwards -> Null inverse");
    }
    //#endregion 🔖Bridge
}
//#endregion 🧪Tests
