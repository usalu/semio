use crate::change_layer_oedometric_modulus::ChangeLayerOedometricModulus;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangeLayerOedometricModulus, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_layer_oedometric_modulus::ChangeLayerOedometricModulus> {
    Ok((|| {
    let v = base.layers.iter().find(|f| f.id == payload.id).map(|f| f.oedometric_modulus).unwrap_or(payload.new_oedometric_modulus);
    vec![Op::from(ChangeLayerOedometricModulus { id: payload.id.clone(), new_oedometric_modulus: v })]

    })())
}
