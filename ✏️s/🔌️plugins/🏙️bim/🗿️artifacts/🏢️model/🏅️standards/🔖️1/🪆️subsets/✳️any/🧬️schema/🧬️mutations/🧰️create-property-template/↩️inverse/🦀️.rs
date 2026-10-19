//! ↩️ Inverse of `CreatePropertyTemplate`: the concrete `DeletePropertyTemplate` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_property_template::DeletePropertyTemplate;
use crate::standards::v1::subsets::any::schema::mutations::create_property_template::CreatePropertyTemplate;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreatePropertyTemplate, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_property_template::DeletePropertyTemplate>,
{
    if semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![Op::from(DeletePropertyTemplate { id: payload.id.clone() })]
}
