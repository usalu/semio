//! ↩️ `change-zone-label-de` inverse.

use crate::change_zone_label_de::ChangeZoneLabelDe;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneLabelDe, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_label_de::ChangeZoneLabelDe> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneLabelDe { zone_id: payload.zone_id.clone(), new_label_de: row.label_de.clone() })]).unwrap_or_default())
}
