//! ↩️ `change-vent-sfp-class` inverse.
use crate::change_vent_sfp_class::ChangeVentSfpClass;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentSfpClass, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_sfp_class::ChangeVentSfpClass> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentSfpClass { vent_id: payload.vent_id.clone(), new_sfp_required_class: v.sfp_required_class })]

    })())
}
