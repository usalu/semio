//! ↩️ Inverse of `CreateBeam`: the concrete `DeleteBeam` of the id it created, none when the id was already taken.

use semio_s_artifact_bim_model_mut_any_67::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam;
use crate::standards::v1::subsets::any::schema::mutations::create_beam::CreateBeam;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateBeam, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_67::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam>,
{
    if base.beams.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteBeam { id: payload.id.clone() })]
}
