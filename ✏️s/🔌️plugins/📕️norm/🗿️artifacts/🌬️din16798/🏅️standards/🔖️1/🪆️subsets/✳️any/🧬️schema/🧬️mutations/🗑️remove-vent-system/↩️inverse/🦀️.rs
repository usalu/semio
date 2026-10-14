use crate::remove_vent_system::RemoveVentSystem;
use crate::insert_vent_system::InsertVentSystem;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &RemoveVentSystem, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_vent_system::InsertVentSystem> {
    Ok((|| {
    let Some((index, vent)) = base.vent_systems.iter().enumerate().find(|(_, v)| v.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(InsertVentSystem { index: Some(index), vent: vent.clone() })]

    })())
}
