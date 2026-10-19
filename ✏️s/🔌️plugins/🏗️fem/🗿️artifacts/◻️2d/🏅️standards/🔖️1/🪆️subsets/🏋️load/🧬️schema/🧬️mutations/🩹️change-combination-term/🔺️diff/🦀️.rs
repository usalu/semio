//! 🔺️ Keyed term intent validates the current combination before authoring its sparse child delta.
use super::ChangeCombinationTerm;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
pub fn diff(payload:&ChangeCombinationTerm,base:&Fem2dSnapshotValue)->protocol::MutationOutcome<Fem2dDiff>{
    let Some(row)=base.combinations.iter().find(|row|row.id==payload.id)else{return protocol::MutationOutcome::error("mutation.target-missing","Combination does not exist.",[payload.id.clone()]);};
    if payload.factor.is_some_and(|value|!value.is_finite()){return protocol::MutationOutcome::fatal("mutation.invariant","Combination term factor must be finite.",[payload.id.clone(),payload.case_id.clone()]);}
    let position=row.terms.iter().position(|term|term.case_id==payload.case_id);
    let mut candidate=row.clone();
    let terms=match (payload.factor,position){
        (Some(factor),Some(index))=>{candidate.terms[index].factor=factor;Fem2dCombinationTermsDelta{modified:vec![Fem2dCombinationTermsModification{id:payload.case_id.clone(),patch:Fem2dCombinationTermPatch{factor:Some(factor)}}],..Default::default()}},
        (Some(factor),None)=>{let index=payload.index.unwrap_or(row.terms.len());if index>row.terms.len(){return protocol::MutationOutcome::error("mutation.target-missing","Term insertion index is outside the current combination.",[payload.id.clone(),payload.case_id.clone()]);}let term=semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm{case_id:payload.case_id.clone(),factor};candidate.terms.insert(index,term.clone());Fem2dCombinationTermsDelta{inserted:vec![Fem2dCombinationTermInsertion{index,row:term}],..Default::default()}},
        (None,Some(index))=>{candidate.terms.remove(index);Fem2dCombinationTermsDelta::removal_by_id(payload.case_id.clone(),index)},
        (None,None)=>return protocol::MutationOutcome::empty().warning("mutation.no-op","Combination has no such term."),
    };
    if let Some(rejection)=semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards::combination_term_references(base,&payload.id,&candidate){return rejection;}
    if let Some(rejection)=semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards::combination_factors(&candidate){return rejection;}
    if candidate==*row{return protocol::MutationOutcome::empty().warning("mutation.no-op","Combination term already has that factor.");}
    protocol::MutationOutcome::new(Fem2dDiff{combinations:Some(Fem2dCombinationsDelta{modified:vec![Fem2dCombinationsModification{id:payload.id.clone(),patch:Fem2dCombinationPatch{name:None,terms:Some(terms)}}],..Default::default()}),..Default::default()})
}