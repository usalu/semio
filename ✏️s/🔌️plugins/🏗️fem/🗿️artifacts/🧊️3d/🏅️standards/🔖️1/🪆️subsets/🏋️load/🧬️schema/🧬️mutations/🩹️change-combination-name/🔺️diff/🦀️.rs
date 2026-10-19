//! 🔺️ A combination rename authors no term fields.
use super::ChangeCombinationName;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
pub fn diff(payload:&ChangeCombinationName,base:&Fem3dSnapshotValue)->protocol::MutationOutcome<Fem3dDiff>{
    let Some(row)=base.combinations.iter().find(|row|row.id==payload.id)else{return protocol::MutationOutcome::error("mutation.target-missing","Combination does not exist.",[payload.id.clone()]);};
    if row.name==payload.name{return protocol::MutationOutcome::empty().warning("mutation.no-op","Combination already has that name.");}
    protocol::MutationOutcome::new(Fem3dDiff{combinations:Some(Fem3dCombinationsDelta{modified:vec![Fem3dCombinationsModification{id:payload.id.clone(),patch:Fem3dCombinationPatch{name:Some(payload.name.clone()),..Default::default()}}],..Default::default()}),..Default::default()})
}