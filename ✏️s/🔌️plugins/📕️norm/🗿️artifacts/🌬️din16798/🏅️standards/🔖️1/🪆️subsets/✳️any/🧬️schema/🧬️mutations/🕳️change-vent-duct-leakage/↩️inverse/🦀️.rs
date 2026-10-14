//! ↩️ `change-vent-duct-leakage` inverse.
use crate::change_vent_duct_leakage::ChangeVentDuctLeakage;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentDuctLeakage, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_duct_leakage::ChangeVentDuctLeakage> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentDuctLeakage { vent_id: payload.vent_id.clone(), new_duct_leakage_m3_s_m2: v.duct_leakage_m3_s_m2 })]

    })())
}
