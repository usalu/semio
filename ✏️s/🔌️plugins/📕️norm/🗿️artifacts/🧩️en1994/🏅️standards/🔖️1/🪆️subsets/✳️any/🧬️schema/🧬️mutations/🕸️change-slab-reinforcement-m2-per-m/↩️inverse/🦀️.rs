//! ↩️ Inverse for `change-slab-reinforcement-m2-per-m`.
use crate::change_slab_reinforcement_m2_per_m::ChangeSlabReinforcementM2PerM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeSlabReinforcementM2PerM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slab_reinforcement_m2_per_m::ChangeSlabReinforcementM2PerM> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(slab) = base.slabs.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeSlabReinforcementM2PerM { id: payload.id.clone(), new_as_m2_per_m: slab.as_m2_per_m })])
}
