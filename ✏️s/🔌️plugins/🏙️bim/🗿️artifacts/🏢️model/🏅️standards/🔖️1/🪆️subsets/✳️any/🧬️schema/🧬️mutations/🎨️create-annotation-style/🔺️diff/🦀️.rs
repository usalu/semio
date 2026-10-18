//! 🔺️ Diff constructor for `CreateAnnotationStyle`: one created annotation style entry. The id is free in every collection. The name is not blank, the text height is positive, mark size, gap and overshoot are lengths of zero or more and at most six decimals are printed.
//! Everything the annotation style shows is inferred from the current geometry of what it names, never stored.

use crate::standards::v1::subsets::any::schema::mutations::annotating;
use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::CreateAnnotationStyle;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateAnnotationStyle, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = annotating::style_record_fault(&payload.annotation_style) {
        let path = fault.path(Some("annotation_style"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::annotation_styles(payload.id.clone(), Entry::Created(payload.annotation_style.clone())))
}
