//! ↩️ Inverse of `CreateClassificationSystem`: the concrete `DeleteClassificationSystem` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::DeleteClassificationSystem;
use crate::standards::v1::subsets::any::schema::mutations::create_classification_system::CreateClassificationSystem;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateClassificationSystem, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::DeleteClassificationSystem>,
{
    if semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id).is_some() || payload.id == "entry" {
        return Vec::new();
    }
    vec![Op::from(DeleteClassificationSystem { id: payload.id.clone() })]
}
