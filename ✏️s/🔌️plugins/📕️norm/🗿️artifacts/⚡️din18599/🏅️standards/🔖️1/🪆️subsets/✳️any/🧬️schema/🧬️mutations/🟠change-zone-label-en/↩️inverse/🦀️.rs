//! ↩️ `change-zone-label-en` inverse.

use crate::change_zone_label_en::ChangeZoneLabelEn;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneLabelEn, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_label_en::ChangeZoneLabelEn> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneLabelEn { zone_id: payload.zone_id.clone(), new_label_en: row.label_en.clone() })]).unwrap_or_default())
}
