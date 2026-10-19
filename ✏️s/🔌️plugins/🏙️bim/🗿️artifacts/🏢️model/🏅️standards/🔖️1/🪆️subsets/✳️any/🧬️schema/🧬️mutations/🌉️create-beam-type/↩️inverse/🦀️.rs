//! ↩️ Inverse of `CreateBeamType`: the concrete `DeleteBeamType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_beam_type::DeleteBeamType;
use crate::standards::v1::subsets::any::schema::mutations::create_beam_type::CreateBeamType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateBeamType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_beam_type::DeleteBeamType>,
{
    if base.beam_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteBeamType { id: payload.id.clone() })]
}
