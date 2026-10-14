//! ↩️ `change-vent-oda-class` inverse.
use crate::change_vent_oda_class::ChangeVentOdaClass;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentOdaClass, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_oda_class::ChangeVentOdaClass> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentOdaClass { vent_id: payload.vent_id.clone(), new_oda_class: v.oda_class.clone() })]

    })())
}
