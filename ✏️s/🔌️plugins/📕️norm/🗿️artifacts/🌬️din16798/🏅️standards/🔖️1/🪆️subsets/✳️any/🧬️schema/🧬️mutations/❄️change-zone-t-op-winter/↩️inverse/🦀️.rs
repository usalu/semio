//! ↩️ `change-zone-t-op-winter` inverse.
use crate::change_zone_t_op_winter::ChangeZoneTOpWinter;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneTOpWinter, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_t_op_winter::ChangeZoneTOpWinter> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneTOpWinter { zone_id: payload.zone_id.clone(), new_t_op_winter_c: z.t_op_winter_c })]

    })())
}
