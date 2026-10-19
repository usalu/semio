//! 🔺️ Diff constructor for `CreateAnnotationStyle`: one created annotation style entry. The id is free in every collection. The name is not blank, the text height is positive, mark size, gap and overshoot are lengths of zero or more and at most six decimals are printed.
//! Everything the annotation style shows is inferred from the current geometry of what it names, never stored.

use crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::CreateAnnotationStyle;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateAnnotationStyle, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::annotating::style_record_fault(&payload.annotation_style) {
        let path = fault.path(Some("annotation_style"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::annotation_styles(payload.id.clone(), Entry::Created(payload.annotation_style.clone())))
}
