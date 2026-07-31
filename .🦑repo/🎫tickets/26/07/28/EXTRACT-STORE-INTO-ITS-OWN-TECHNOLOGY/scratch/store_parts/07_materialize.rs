//#region 🔖Materialize
pub fn create_document_envelope<P, Operation>(
    schema: &str,
    id: &str,
    initial_projection: P,
    backbone: Option<DocumentBackboneRef>,
) -> DocumentEnvelope<P, Operation>
where
    P: Clone,
{
    DocumentEnvelope {
        schema: schema.into(),
        id: id.into(),
        vcs: DocumentVcs {
            initial_projection,
            edits: Vec::new(),
            changes: Vec::new(),
            checkpoints: Vec::new(),
            alternatives: Vec::new(),
        },
        backbone,
        active_alternative_id: None,
    }
}

pub fn edit_ids_for_changes<P, Operation>(envelope: &DocumentEnvelope<P, Operation>, change_ids: &[String]) -> Vec<String>
where
    Operation: Clone,
    P: Clone,
{
    let mut edit_ids = Vec::new();
    for change_id in change_ids {
        if let Some(change) = envelope.vcs.changes.iter().find(|entry| entry.id == *change_id) {
            edit_ids.extend(change.edit_ids.iter().cloned());
        }
    }
    edit_ids
}

pub fn materialize_document_projection<P, Operation>(
    envelope: &DocumentEnvelope<P, Operation>,
    applied_edit_ids: &[String],
) -> Result<P, VcsError>
where
    P: Clone,
    Operation: crate::Operation<P>,
{
    materialize_document_projection_with_conflicts(envelope, applied_edit_ids).map(|(projection, _conflicts)| projection)
}

/// @emoji 🤝 Adapts `protocol_command::Operation::reconcile`'s new instance-based signature (`&self`,
/// was a per-TYPE associated fn taking no instance at all) to the once-per-materialization call this
/// crate's replay/store paths always performed: runs the LAST applied operation's `reconcile` hook
/// against `projection`, or passes `projection` through unchanged (matching the trait's own no-op
/// default) if no operation has ever been applied yet. Every real `Operation` impl in this crate
/// (`StudioHistoryOperation`/`DemoOperation`/`TimestampedOperation`) inherits the default no-op
/// `reconcile`, which ignores `self` entirely and only inspects `projection` — so which specific
/// operation instance triggers the call is immaterial for every one of them; a technology that
/// overrides `reconcile` to do real cross-document/graph validation (see
/// `framework/product/os/core`'s `OsOperation`) is documented as inspecting the resulting
/// `projection`, not `self`, for the same reason. Maps `protocol::ReconcileReport` to this crate's
/// own `StudioConflict` at this edge — `protocol_command` deliberately doesn't know about studio
/// types (see its `Operation::reconcile` doc comment).
fn reconcile_with_last<P, Op: Operation<P>>(last_operation: Option<&Op>, projection: P) -> (P, Vec<StudioConflict>) {
    match last_operation {
        Some(operation) => {
            let (projection, reports) = operation.reconcile(projection);
            (projection, reports.into_iter().map(StudioConflict::from).collect())
        }
        None => (projection, Vec::new()),
    }
}

/// @emoji 🤝 Same replay as {@link materialize_document_projection}, additionally surfacing whatever
/// {@link Operation::reconcile} reports for the resulting projection. Kept as a twin function (rather
/// than changing `materialize_document_projection`'s signature) so every existing caller across the
/// workspace is unaffected; call sites that care about conflicts (e.g. `DocumentStore`) opt into
/// this one instead.
pub fn materialize_document_projection_with_conflicts<P, Operation>(
    envelope: &DocumentEnvelope<P, Operation>,
    applied_edit_ids: &[String],
) -> Result<(P, Vec<StudioConflict>), VcsError>
where
    P: Clone,
    Operation: crate::Operation<P>,
{
    let mut projection = envelope.vcs.initial_projection.clone();
    let mut last_operation: Option<&Operation> = None;
    for edit_id in applied_edit_ids {
        let edit = envelope
            .vcs
            .edits
            .iter()
            .find(|entry| entry.id == *edit_id)
            .ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for operation in &edit.forwards {
            projection = apply_operation(&projection, operation);
            last_operation = Some(operation);
        }
    }
    Ok(reconcile_with_last(last_operation, projection))
}

fn now_iso() -> String {
    format!("{}", now_ms())
}

fn now_ms() -> u64 {
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as u64)
            .unwrap_or(0)
    }
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now() as u64
    }
}

fn uncommitted_edit_ids<P, Operation>(envelope: &DocumentEnvelope<P, Operation>, applied_edit_ids: &[String]) -> Vec<String>
where
    Operation: Clone,
    P: Clone,
{
    let committed: HashSet<String> = envelope
        .vcs
        .changes
        .iter()
        .flat_map(|change| change.edit_ids.iter().cloned())
        .collect();
    applied_edit_ids
        .iter()
        .filter(|id| !committed.contains(*id))
        .cloned()
        .collect()
}

//#endregion 🔖Materialize
