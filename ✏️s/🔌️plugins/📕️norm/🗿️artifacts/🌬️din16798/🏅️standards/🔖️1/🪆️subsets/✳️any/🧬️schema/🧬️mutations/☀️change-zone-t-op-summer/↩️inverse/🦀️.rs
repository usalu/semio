//! ↩️ `change-zone-t-op-summer` inverse.
use crate::change_zone_t_op_summer::ChangeZoneTOpSummer;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneTOpSummer, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_t_op_summer::ChangeZoneTOpSummer> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneTOpSummer { zone_id: payload.zone_id.clone(), new_t_op_summer_c: z.t_op_summer_c })]

    })())
}
