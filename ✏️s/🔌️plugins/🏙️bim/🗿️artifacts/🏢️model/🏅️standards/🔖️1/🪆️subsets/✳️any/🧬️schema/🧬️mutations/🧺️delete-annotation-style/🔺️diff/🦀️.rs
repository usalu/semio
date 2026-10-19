//! 🔺️ Diff constructor for `DeleteAnnotationStyle`: one deleted style entry; refused while a dimension, tag, text note or leader still uses it.

use crate::standards::v1::subsets::any::schema::mutations::delete_annotation_style::DeleteAnnotationStyle;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteAnnotationStyle, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.annotation_styles.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Annotation style \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let used = base.dimensions.values().any(|row| row.style == payload.id)
        || base.tags.values().any(|row| row.style == payload.id)
        || base.text_notes.values().any(|row| row.style == payload.id)
        || base.leaders.values().any(|row| row.style == payload.id);
    if used {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Annotation style \"{}\" is still used by annotations.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::annotation_styles(payload.id.clone(), Entry::Deleted))
}
