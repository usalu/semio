//! 🔺️ Diff constructor for `SetAnnotationStyle`: a sparse annotation style patch of exactly the provided fields that differ. The record that results must break none of the create rules: the name is not blank, the text height is positive, mark size, gap and overshoot are lengths of zero or more and at most six decimals are printed.
//! Providing only equal values, or no field, is a no-op.

use super::super::annotating;
use super::SetAnnotationStyle;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetAnnotationStyle, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.annotation_styles.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Annotation style \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = annotating::style_record_fault(&next) {
        let path = fault.path(None);
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Annotation style \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::annotation_styles(payload.id.clone(), Entry::Patched(change)))
}
