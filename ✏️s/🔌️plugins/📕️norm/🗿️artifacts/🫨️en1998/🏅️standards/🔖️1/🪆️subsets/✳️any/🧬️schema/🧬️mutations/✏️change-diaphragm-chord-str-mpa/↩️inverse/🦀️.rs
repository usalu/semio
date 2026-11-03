//! ↩️ `change-diaphragm-chord-str-mpa` inverse: restores the previous `chord_strength_mpa`.

use crate::change_diaphragm_chord_str_mpa::ChangeDiaphragmChordStrMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeDiaphragmChordStrMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_chord_str_mpa::ChangeDiaphragmChordStrMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDiaphragmChordStrMpa, base: &En1998Snapshot) -> Option<ChangeDiaphragmChordStrMpa> {
    let r0 = base.diaphragms.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDiaphragmChordStrMpa { id: payload.id.clone(), new_chord_strength_mpa: r0.chord_strength_mpa })
}
