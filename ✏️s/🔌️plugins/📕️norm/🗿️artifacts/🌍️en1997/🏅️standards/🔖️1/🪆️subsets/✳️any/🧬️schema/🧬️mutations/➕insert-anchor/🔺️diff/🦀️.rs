use crate::insert_anchor::InsertAnchor;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997AnchorsRows};

pub fn diff(payload: &InsertAnchor, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.anchors.iter().any(|existing| existing.id == payload.anchor.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Anchor id {} already exists.", payload.anchor.id), [payload.anchor.id.clone()]);
    }
    if payload.index > base.anchors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.anchors.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::insertion(payload.index, payload.anchor.clone())), ..Default::default() })
}
