//! 🔺️ Diff constructor for `CreatePropertyTemplate`: one created template entry. The id must be free in every collection, the template needs a name no other template has, kinds it lists at most once and sound,
//! uniquely named property definitions (a default and every allowed value of the kind of the property and within its own range). Which elements the template applies to is not checked here: it is inferred.

use crate::standards::v1::subsets::any::schema::mutations::create_property_template::CreatePropertyTemplate;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{template_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreatePropertyTemplate, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let template = &payload.template;
    if let Some((field, message)) = template_problem(&template.name, &template.applies_to, &template.properties) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["template", field]);
    }
    if base.property_templates.values().any(|row| row.name == template.name) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("A property set template named \"{}\" already exists.", template.name), ["template", "name"]);
    }
    MutationOutcome::new(ModelDiff::property_templates(payload.id.clone(), Entry::Created(template.clone())))
}
