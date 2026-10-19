//! 🔺️ Diff constructor for `CreateTextNote`: one created text note entry. The id is free in every collection. The storey and the style must exist, the position and rotation are finite and the text is not blank.
//! Everything the text note shows is inferred from the current geometry of what it names, never stored.

use crate::standards::v1::subsets::any::schema::mutations::create_text_note::CreateTextNote;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateTextNote, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::annotating::note_fault(base, &payload.text_note) {
        let path = fault.path(Some("text_note"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::text_notes(payload.id.clone(), Entry::Created(payload.text_note.clone())))
}
