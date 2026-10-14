use crate::insert_vent_system::InsertVentSystem;
use crate::remove_vent_system::RemoveVentSystem;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &InsertVentSystem, _base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_vent_system::RemoveVentSystem> {
    Ok((|| {
    vec![Op::from(RemoveVentSystem { vent_id: payload.vent.id.clone() })]

    })())
}
