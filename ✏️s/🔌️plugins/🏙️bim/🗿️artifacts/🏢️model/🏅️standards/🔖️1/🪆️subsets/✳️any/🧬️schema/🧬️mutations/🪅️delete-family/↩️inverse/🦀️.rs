//! ↩️ Inverse of `DeleteFamily`: the setters of the properties and classifications of the family, then one concrete create per removed record. The store replays the vector reversed, so it holds them first,
//! then the solids, the parameters against their dependency order and the family last: the family is recreated first, then every parameter after the ones its formula uses, then the solids that use the
//! parameters, then the data.

use crate::standards::v1::subsets::any::schema::mutations::create_family::CreateFamily;
use semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid;
use semio_s_artifact_bim_model_mut_any_109::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter;
use crate::standards::v1::subsets::any::schema::mutations::delete_family::DeleteFamily;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteFamily, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<crate::standards::v1::subsets::any::schema::mutations::create_family::CreateFamily> + From<semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid> + From<semio_s_artifact_bim_model_mut_any_109::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter>,
{
    let Some(family) = base.families.get(&payload.id) else {
        return Vec::new();
    };
    let solids = base.family_solids.iter().filter(|(_, row)| row.family == payload.id).map(|(id, row)| Op::from(CreateFamilySolid { id: id.clone(), solid: row.clone() }));
    let parameters = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::parameters_in_order(base, &payload.id).into_iter().rev().map(|row| Op::from(SetFamilyParameter { family: row.family.clone(), name: row.name.clone(), kind: Some(row.kind), value: Some(row.value.clone()) }));
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::data_rows(base, &payload.id).into_iter().chain(solids).chain(parameters).chain(std::iter::once(Op::from(CreateFamily { id: payload.id.clone(), family: family.clone() }))).collect()
}
