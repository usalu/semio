//! `term:<caseId>` factor, `addTerm` with the case id as value, `removeTerm`) → keyed name/term events.
//! 🩹️ Fem3d play app command — `patch-combination`: one-field edit of a combination (`name`, a

use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_combination_name::ChangeCombinationName;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_combination_term::ChangeCombinationTerm;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem3dSnapshot = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

/// ⚖️ The field prefix a term factor row binds: `term:<caseId>`.
pub const TERM_FIELD_PREFIX: &str = "term:";
/// ➕️ The field the add-term select binds; the chosen case id rides the control's own value.
pub const ADD_TERM_FIELD: &str = "addTerm";
/// ➖️ The field a remove-term button binds; the case id rides the value.
pub const REMOVE_TERM_FIELD: &str = "removeTerm";
/// ⚖️ The factor a freshly added term starts at — the characteristic (1.0) weighting.
pub const ADDED_TERM_FACTOR: f64 = 1.0;

//#region 🔖️PatchCombination
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-combination")]
pub struct PatchCombination {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchCombination, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let combination=doc.snapshot.combinations.iter().find(|row|row.id==payload.id).ok_or_else(||Fault::from("fem3d.patch.combination-missing"))?;
    if payload.field=="name" {
        if combination.name==payload.value{return Ok(Emit::default());}
        return Ok(Emit::mutations(vec![Fem3dMutation::ChangeCombinationName(ChangeCombinationName{id:payload.id.clone(),name:payload.value.clone()})]));
    }
    let value=payload.value.trim();
    let (case_id,factor)=if let Some(case_id)=payload.field.strip_prefix(TERM_FIELD_PREFIX){
        let factor:f64=value.parse().map_err(|_|Fault::from("fem3d.patch.combination-value"))?;
        (case_id,if factor==0.0{None}else{Some(factor)})
    }else if payload.field==ADD_TERM_FIELD{
        if combination.terms.contains_key(value){return Ok(Emit::default());}
        (value,Some(ADDED_TERM_FACTOR))
    }else if payload.field=="removeTerm"{(value,None)}else{return Err(Fault::from("fem3d.patch.combination-field"));};
    if case_id.is_empty(){return Err(Fault::from("fem3d.patch.combination-value"));}
    if combination.terms.get(case_id).copied()==factor{return Ok(Emit::default());}
    Ok(Emit::mutations(vec![Fem3dMutation::ChangeCombinationTerm(ChangeCombinationTerm{id:payload.id.clone(),case_id:case_id.into(),factor})]))
}
//#endregion 🔖️PatchCombination

//#region 🧪️Tests
//#endregion 🧪️Tests
