//! ↩️ Concrete delete-workset inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::delete_workset::DeleteWorkset;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &DeleteWorkset, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_workset::CreateWorkset> + From<semio_s_artifact_bim_model_mut_any_51::standards::v1::subsets::any::schema::mutations::set_element_workset::SetElementWorkset>,
{
    let Some(record) = base.worksets.get(&payload.id) else { return Vec::new(); };
    let mut result = Vec::new();
    for (id, member) in &base.element_worksets { if member.target == payload.id { result.push(Op::from(semio_s_artifact_bim_model_mut_any_51::standards::v1::subsets::any::schema::mutations::set_element_workset::SetElementWorkset { id: id.clone(), workset: Some(member.target.clone()) })); } }
    result.push(Op::from(crate::standards::v1::subsets::any::schema::mutations::create_workset::CreateWorkset { id: payload.id.clone(), workset: record.clone() }));
    result
}
