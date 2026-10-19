//! ↩️ Inverse of `CreateAreaScheme`: the concrete `DeleteAreaScheme` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_101::standards::v1::subsets::any::schema::mutations::delete_area_scheme::DeleteAreaScheme;
use crate::standards::v1::subsets::any::schema::mutations::create_area_scheme::CreateAreaScheme;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateAreaScheme, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_101::standards::v1::subsets::any::schema::mutations::delete_area_scheme::DeleteAreaScheme>,
{
    if semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![Op::from(DeleteAreaScheme { id: payload.id.clone() })]
}
