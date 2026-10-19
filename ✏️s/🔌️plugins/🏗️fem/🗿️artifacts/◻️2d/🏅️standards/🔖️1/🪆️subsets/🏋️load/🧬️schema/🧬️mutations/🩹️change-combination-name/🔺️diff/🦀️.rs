//! 🔺️ A combination rename authors no term fields.
use super::ChangeCombinationName;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
pub fn diff(payload:&ChangeCombinationName,base:&Fem2dSnapshotValue)->protocol::MutationOutcome<Fem2dDiff>{
    let Some(row)=base.combinations.iter().find(|row|row.id==payload.id)else{return protocol::MutationOutcome::error("mutation.target-missing","Combination does not exist.",[payload.id.clone()]);};
    if row.name==payload.name{return protocol::MutationOutcome::empty().warning("mutation.no-op","Combination already has that name.");}
    protocol::MutationOutcome::new(Fem2dDiff{combinations:Some(Fem2dCombinationsDelta{modified:vec![Fem2dCombinationsModification{id:payload.id.clone(),patch:Fem2dCombinationPatch{name:Some(payload.name.clone()),..Default::default()}}],..Default::default()}),..Default::default()})
}