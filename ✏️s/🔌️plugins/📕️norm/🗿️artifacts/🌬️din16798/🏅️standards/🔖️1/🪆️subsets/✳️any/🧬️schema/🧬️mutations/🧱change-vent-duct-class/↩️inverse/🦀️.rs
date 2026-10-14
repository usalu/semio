//! ↩️ `change-vent-duct-class` inverse.
use crate::change_vent_duct_class::ChangeVentDuctClass;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentDuctClass, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_duct_class::ChangeVentDuctClass> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentDuctClass { vent_id: payload.vent_id.clone(), new_duct_class: v.duct_class.clone() })]

    })())
}
