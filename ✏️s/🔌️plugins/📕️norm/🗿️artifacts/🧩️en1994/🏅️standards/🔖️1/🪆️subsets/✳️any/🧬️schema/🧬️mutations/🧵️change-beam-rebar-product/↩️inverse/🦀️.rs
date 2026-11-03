//! ↩️ Inverse for `change-beam-rebar-product`.
use crate::change_beam_rebar_product::ChangeBeamRebarProduct;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamRebarProduct, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_rebar_product::ChangeBeamRebarProduct> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamRebarProduct { id: payload.id.clone(), new_reinforcement_product: beam.reinforcement_product })])
}
