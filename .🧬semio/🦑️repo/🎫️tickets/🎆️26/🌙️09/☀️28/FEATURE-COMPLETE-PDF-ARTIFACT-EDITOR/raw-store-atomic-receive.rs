//! 🛬️ Prepares one verified remote edit and a private causal candidate for the Store merge.

use super::*;
use semio_os_spr::{CausalPayload, MutationDag, MutationDagAppliedStep, MutationEnvelope, VerifiedAtomicEdit};
use semio_framework_value::paged::ByteReadSource;

pub enum AtomicStoreReceive {
    AwaitingDependencies,
    Finished(semio_os_spr::MergeReport),
}

/// 📥️ Resolves every causal row before offering one author edit to the existing history fold.
pub async fn ingest_verified_atomic_edit<P, M>(
    store: &mut StoreParts<'_, P, M>, incoming: &VerifiedAtomicEdit,
) -> Result<AtomicStoreReceive, VcsError>
where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
    ensure_durable_group_idle(store)?;
    let events = incoming.events();
    if events.is_empty() || events.iter().any(|event| {
        event.document_id.0 != store.log.envelope.id || event.diff.schema.0 != store.log.envelope.schema ||
            event.inverse.schema.0 != store.log.envelope.schema || semio_os_spr::is_history_transition(event) ||
            event.line != events[0].line || event.verb != events[0].verb
    }) {
        return Err(refused("atomic remote edit does not belong to this document, schema, and lane"));
    }

    if let Some(previous) = store.log.envelope.vcs.edits.find_near(|edit| edit.id == incoming.edit_id()) {
        let established = store.log.operation_envelopes(previous)?;
        if established.len() != events.len() {
            return Err(refused("atomic remote edit reuses an established edit with different operations"));
        }
        for (old, next) in established.iter().zip(events) {
            let prepared = normalize_operation::<P, M>(next)?;
            if !same_event(old, prepared.as_ref().unwrap_or(next)) {
                return Err(refused("atomic remote edit reuses an established edit with different operations"));
            }
        }
        return Ok(AtomicStoreReceive::Finished(semio_os_spr::MergeReport {
            policy: store.log.merge_policy, accepted: true, insertion_index: store.log.applied_edit_ids.len() as u32,
            replayed: Vec::new(), worst: None, conflict: None,
        }));
    }

    let mut candidate = (*store.log.dag).clone();
    let proposed: std::collections::HashSet<_> = events.iter().map(|event| event.mutation_id.clone()).collect();
    let mut remaining = proposed.clone();
    for event in events {
        match candidate.insert(event.clone()) {
            Ok(semio_os_spr::InsertResult::AlreadyApplied(displaced)) => {
                retire_uncommitted(store, candidate, [displaced])?;
                return Err(refused("atomic remote edit overlaps an established operation"));
            }
            Ok(_) => {}
            Err(rejected) => {
                let reason = VcsError::Backbone(rejected.error.to_string());
                retire_uncommitted(store, candidate, [rejected.envelope])?;
                return Err(reason);
            }
        }
    }
    let mut ready = Vec::new();
    let mut transitions = Vec::new();
    while let Some(event) = take_ready(&mut candidate) {
        remaining.remove(&event.mutation_id);
        if semio_os_spr::is_history_transition(&event) { transitions.push(event); }
        else { ready.push(event); }
    }
    if !remaining.is_empty() {
        retire_uncommitted(store, candidate, ready.into_iter().chain(transitions))?;
        return Ok(AtomicStoreReceive::AwaitingDependencies);
    }

    let edit = match prepare_edit::<P, M>(incoming).await {
        Ok(edit) => edit,
        Err(error) => {
            retire_uncommitted(store, candidate, ready.into_iter().chain(transitions))?;
            return Err(error);
        }
    };
    let mut batch = vec![edit];
    let mut clock = store.log.clock;
    for event in &ready {
        clock.merge(&event.timestamp);
        if proposed.contains(&event.mutation_id) { continue; }
        let decoded = match edit_from_operation_envelope_cold::<M>(event).await {
            Ok(mut edit) => { stamp_edit_semantics::<P, M>(&mut edit, &store.log.envelope.schema); edit }
            Err(error) => {
                retire_scratch_edits::<P, M>(batch);
                retire_uncommitted(store, candidate, ready.into_iter().chain(transitions))?;
                return Err(error);
            }
        };
        batch.push(decoded);
    }
    retire_redundant(store, ready)?;
    let report = commit_ready_remote_batch(store, candidate, batch, transitions, Some(incoming.edit_id())).await?;
    if report.accepted { store.log.clock = clock; }
    Ok(AtomicStoreReceive::Finished(report))
}

async fn prepare_edit<P, M>(incoming: &VerifiedAtomicEdit) -> Result<Edit<M>, VcsError>
where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
    let mut events = incoming.events().iter();
    let first = events.next().ok_or_else(|| refused("atomic empty edit"))?;
    let first_prepared = normalize_operation::<P, M>(first)?;
    let mut original = edit_from_operation_envelope_cold(first_prepared.as_ref().unwrap_or(first)).await?;
    original.id = incoming.edit_id().into();
    for event in events {
        let prepared = match normalize_operation::<P, M>(event) {
            Ok(value) => value,
            Err(error) => { retire_scratch_edits::<P, M>([original]); return Err(error); }
        };
        let mut item = match edit_from_operation_envelope_cold::<M>(prepared.as_ref().unwrap_or(event)).await {
            Ok(value) => value,
            Err(error) => { retire_scratch_edits::<P, M>([original]); return Err(error); }
        };
        original.forwards.append(&mut item.forwards);
        original.mutation_meta.append(&mut item.mutation_meta);
        original.inverse.extend(std::mem::take(&mut *item.inverse));
        retire_scratch_edits::<P, M>([item]);
    }
    stamp_edit_semantics::<P, M>(&mut original, &first.diff.schema.0);
    Ok(original)
}

fn normalize_operation<P, M>(source: &MutationEnvelope) -> Result<Option<MutationEnvelope>, VcsError>
where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
    let forward = native_operation::<P, M>(&source.diff.payload)?;
    let inverse = native_operation::<P, M>(&source.inverse.payload)?;
    if forward.is_none() && inverse.is_none() { return Ok(None); }
    let mut normalized = source.clone();
    if let Some((payload, target)) = forward { normalized.diff.payload = payload; normalized.target = target; }
    if let Some((payload, _)) = inverse { normalized.inverse.payload = payload; }
    Ok(Some(normalized))
}

fn native_operation<P, M>(payload: &CausalPayload) -> Result<Option<(CausalPayload, Vec<String>)>, VcsError>
where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
    let mut tag = [0; 2];
    if payload.read_bytes(0, &mut tag) != 2 || tag != *b"op" { return Ok(None); }
    let bytes = payload.to_vec_owner();
    let json = std::str::from_utf8(&bytes[2..]).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let operation: M = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)
        .map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let body = operation.encode_op().map_err(|error| VcsError::Serialize(error.to_string()));
    let target = operation.conflict_target();
    retire_scratch_operations::<P, M>([operation]);
    let encoded = CausalPayload::try_from_slice(&body?).map_err(VcsError::InverseRefused)?;
    Ok(Some((encoded, target)))
}

fn same_event(old: &MutationEnvelope, next: &MutationEnvelope) -> bool {
    old.mutation_id == next.mutation_id && old.document_id == next.document_id && old.actor == next.actor &&
        old.dependencies == next.dependencies && old.target == next.target && old.diff == next.diff && old.inverse == next.inverse &&
        old.timestamp == next.timestamp && old.transaction == next.transaction && old.verb == next.verb && old.line == next.line
}

fn take_ready(candidate: &mut MutationDag) -> Option<MutationEnvelope> {
    loop {
        match candidate.take_next_applied() {
            MutationDagAppliedStep::Envelope(event) => return Some(event),
            MutationDagAppliedStep::SeededIdentity => continue,
            MutationDagAppliedStep::Complete => return None,
        }
    }
}

fn retire_uncommitted<P, M>(
    store: &mut StoreParts<'_, P, M>, dag: MutationDag, events: impl IntoIterator<Item = MutationEnvelope>,
) -> Result<(), VcsError>
where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
    store.owners.displaced_retirements.reserve(1)?;
    store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(dag)));
    retire_redundant(store, events)
}

fn retire_redundant<P, M>(
    store: &mut StoreParts<'_, P, M>, events: impl IntoIterator<Item = MutationEnvelope>,
) -> Result<(), VcsError>
where P: semio_os_spr::StoreState, M: semio_os_spr::StoreMutationFor<P> {
    for event in events {
        store.owners.displaced_retirements.reserve(1)?;
        store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::causal_envelope(event)));
    }
    Ok(())
}

fn refused(reason: &'static str) -> VcsError { VcsError::ValidationFailed(reason.into()) }
