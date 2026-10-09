//! 🔺️ Diff constructor for `DeleteAnnotationStyle`: one deleted style entry; refused while a dimension, tag, text note or leader still uses it.

use super::DeleteAnnotationStyle;
use crate::{Entry, ModelDiff, ModelSnapshot};
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
