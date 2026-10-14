//! ↩️ `change-vent-filter-sup` inverse.
use crate::change_vent_filter_sup::ChangeVentFilterSup;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentFilterSup, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_filter_sup::ChangeVentFilterSup> {
    Ok((|| {
    let Some(v) = base.vent_systems.iter().find(|v| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentFilterSup { vent_id: payload.vent_id.clone(), new_filter_sup_class: v.filter_sup_class.clone() })]

    })())
}
