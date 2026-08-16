//! 🔺️ Diff fragment yielded by `ChangeUri`.
use super::mutation::ChangeUri;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse `uri`-only delta, built directly from the payload — real handcrafted construction,
/// never apply-then-capture, never a snapshot clone.
pub fn diff(payload: &ChangeUri, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
    if base.uri == payload.new_uri {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Document URI is already \"{}\".", payload.new_uri));
    }
    protocol::MutationOutcome::new(WriterDiff { uri: Some(payload.new_uri.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
