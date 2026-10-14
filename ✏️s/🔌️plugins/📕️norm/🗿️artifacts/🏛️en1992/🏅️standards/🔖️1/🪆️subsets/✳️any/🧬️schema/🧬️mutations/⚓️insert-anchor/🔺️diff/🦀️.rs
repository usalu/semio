use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992AnchorsRows};
use crate::insert_anchor::InsertAnchor;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertAnchor, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if base.anchors.iter().any(|existing| existing.id == payload.anchor.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Anchor id {} already exists.", payload.anchor.id), [payload.anchor.id.clone()]);
    }
    if payload.index > base.anchors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.anchors.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows::insertion(payload.index, payload.anchor.clone())), ..Default::default() })
}
