//! 📍️ `remove-stud` diff on the positional `studs` lane.

use crate::remove_stud::RemoveStud;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994StudsRows};
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn diff(payload: &RemoveStud, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.studs.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shear stud {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if index >= base.studs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "studs index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { studs: Some(En1994StudsRows::removal(&base.studs, index)), ..Default::default() })
}
