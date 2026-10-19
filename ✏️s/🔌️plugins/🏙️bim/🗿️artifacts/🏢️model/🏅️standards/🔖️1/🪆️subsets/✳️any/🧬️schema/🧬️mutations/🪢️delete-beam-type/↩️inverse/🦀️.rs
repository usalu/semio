//! ↩️ Inverse of `DeleteBeamType`: the setters of the properties and classifications of the type, then the concrete `CreateBeamType` carrying the full removed record, none when the beam type was absent.

use crate::standards::v1::subsets::any::schema::mutations::create_beam_type::CreateBeamType;
use crate::standards::v1::subsets::any::schema::mutations::delete_beam_type::DeleteBeamType;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteBeamType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_beam_type::CreateBeamType>,
{
    match base.beam_types.get(&payload.id) {
        Some(record) => semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::data_rows(base, &payload.id).into_iter().chain(std::iter::once(Op::from(CreateBeamType { id: payload.id.clone(), beam_type: record.clone() }))).collect(),
        None => Vec::new(),
    }
}
