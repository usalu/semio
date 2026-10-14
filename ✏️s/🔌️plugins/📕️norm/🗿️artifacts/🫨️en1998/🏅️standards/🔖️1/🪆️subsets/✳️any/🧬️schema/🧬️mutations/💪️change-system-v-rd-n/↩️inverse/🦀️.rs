//! Inverse for `change-system-v-rd-n`.
use crate::change_system_v_rd_n::ChangeSystemVRdN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemVRdN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_v_rd_n::ChangeSystemVRdN> {
    Ok((|| {
    match base.buildings.get(payload.building_index).and_then(|b| b.systems.get(payload.system_index)) {
        Some(sys) => vec![Op::from(ChangeSystemVRdN { building_index: payload.building_index, system_index: payload.system_index, new_base_shear_resistance_n: sys.base_shear_resistance_n })],
        None => Vec::new(),
    }

    })())
}
