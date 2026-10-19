//! ↩️ Concrete set-workset inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::set_workset::SetWorkset;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Patch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
pub fn inverse<Op>(payload: &SetWorkset, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_workset::SetWorkset>,
{
    let Some(record) = base.worksets.get(&payload.id) else { return Vec::new(); };
    let patch = payload.patch().minimal(record).restoring(record);
    if patch.is_empty() { return Vec::new(); }
    vec![Op::from(SetWorkset::from_patch(payload.id.clone(), patch))]
}
