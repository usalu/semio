//! 🔺️ Diff constructor for `DeleteTextNote`: the text note leaves in one sparse diff (see the shared cascade); the elements it names stay untouched.

use crate::standards::v1::subsets::any::schema::mutations::delete_text_note::DeleteTextNote;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteTextNote, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.text_notes.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Text note \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Text note", Some(&payload.id))
}
