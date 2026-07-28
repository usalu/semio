/// @emoji 🌳 Walks `checkpoint_id`'s ancestor chain via `parent_id` back to the root, nearest-first
/// (`checkpoint_id` itself is the first entry). Cycle-guarded (a malformed/adversarial parent chain
/// stops instead of looping forever) — every well-formed chain built by `reconcile_alternative`/
/// `CommitCheckpoint` is already acyclic, this is defense in depth, not a documented invariant break.
fn checkpoint_ancestors<P, Operation>(envelope: &DocumentVcsEnvelope<P, Operation>, checkpoint_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    let mut current = Some(checkpoint_id.to_string());
    while let Some(id) = current {
        if !seen.insert(id.clone()) {
            break;
        }
        let parent = envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == id).and_then(|checkpoint| checkpoint.parent_id.clone());
        chain.push(id);
        current = parent;
    }
    chain
}

/// @emoji 🌳 The merge-base of checkpoints `a` and `b`: the nearest checkpoint common to both
/// ancestor chains (via `parent_id`), or `None` if their histories share no common ancestor.
/// Supports branch-merge tooling that needs to know "everything since the fork point" on either
/// side. `b`'s chain is walked nearest-to-farthest so the FIRST hit in `a`'s ancestor set is the
/// nearest (not merely *a*) common ancestor.
pub fn merge_base<P, Operation>(envelope: &DocumentVcsEnvelope<P, Operation>, a: &str, b: &str) -> Option<String> {
    let ancestors_a: HashSet<String> = checkpoint_ancestors(envelope, a).into_iter().collect();
    checkpoint_ancestors(envelope, b).into_iter().find(|id| ancestors_a.contains(id))
}

pub fn reconcile_alternative<P, Operation>(
    envelope: &mut DocumentVcsEnvelope<P, Operation>,
    alternative_name: &str,
    checkpoint_message: Option<String>,
    authors: Vec<Author>,
) -> Result<String, VcsError>
where
    P: Clone + Serialize + DeserializeOwned,
    Operation: Clone + Serialize + DeserializeOwned,
{
    if envelope.vcs.checkpoints.is_empty() {
        return Err(VcsError::NoCheckpoint);
    }
    let checkpoint_id = envelope
        .vcs
        .checkpoints
        .last()
        .map(|checkpoint| checkpoint.id.clone())
        .ok_or(VcsError::NoCheckpoint)?;
    let alternative_id = create_document_vcs_id("alternative");
    envelope.vcs.alternatives.push(Alternative {
        id: alternative_id.clone(),
        name: alternative_name.to_string(),
        checkpoint_ids: vec![checkpoint_id],
    });
    if let Some(message) = checkpoint_message {
        let change = Change {
            id: create_document_vcs_id("change"),
            edit_ids: Vec::new(),
            description: Some(message),
            saved_at: now_iso(),
        };
        let parent = envelope.vcs.checkpoints.last();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        change_ids.push(change.id.clone());
        envelope.vcs.changes.push(change);
        let timestamp = now_iso();
        let checkpoint_message = Some("reconciled".to_string());
        let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &envelope.vcs.changes, checkpoint_message.as_deref(), &authors, &timestamp);
        envelope.vcs.checkpoints.push(Checkpoint {
            id,
            change_ids,
            parent_id,
            authors,
            message: checkpoint_message,
            timestamp,
        });
    }
    Ok(alternative_id)
}
