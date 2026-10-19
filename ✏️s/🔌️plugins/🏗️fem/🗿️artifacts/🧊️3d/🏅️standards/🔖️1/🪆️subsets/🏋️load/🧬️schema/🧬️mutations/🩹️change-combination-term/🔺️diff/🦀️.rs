//! 🔺️ Keyed term intent validates the current combination before authoring its sparse child delta.
use super::ChangeCombinationTerm;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
pub fn diff(payload:&ChangeCombinationTerm,base:&Fem3dSnapshotValue)->protocol::MutationOutcome<Fem3dDiff>{
    let Some(row)=base.combinations.iter().find(|row|row.id==payload.id)else{return protocol::MutationOutcome::error("mutation.target-missing","Combination does not exist.",[payload.id.clone()]);};
    if payload.factor.is_some_and(|value|!value.is_finite()){return protocol::MutationOutcome::fatal("mutation.invariant","Combination term factor must be finite.",[payload.id.clone(),payload.case_id.clone()]);}
    let mut candidate=row.clone();
    let terms=if let Some(factor)=payload.factor{candidate.terms.insert(payload.case_id.clone(),factor);protocol::command::MapDelta::set(payload.case_id.clone(),factor)}else if candidate.terms.remove(&payload.case_id).is_some(){protocol::command::MapDelta::remove(payload.case_id.clone())}else{return protocol::MutationOutcome::empty().warning("mutation.no-op","Combination has no such term.");};
    if let Some(rejection)=semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::resolve_combination_terms(base,&candidate){return rejection;}
    if let Some(breach)=semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::combination_breach(&candidate){return semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant(breach,vec![payload.id.clone(),payload.case_id.clone()]);}
    if candidate==*row{return protocol::MutationOutcome::empty().warning("mutation.no-op","Combination term already has that factor.");}
    protocol::MutationOutcome::new(Fem3dDiff{combinations:Some(Fem3dCombinationsDelta{modified:vec![Fem3dCombinationsModification{id:payload.id.clone(),patch:Fem3dCombinationPatch{name:None,terms:Some(terms)}}],..Default::default()}),..Default::default()})
}