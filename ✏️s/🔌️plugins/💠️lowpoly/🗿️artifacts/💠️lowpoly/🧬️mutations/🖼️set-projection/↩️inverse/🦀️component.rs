//! ↩️ Inverse for `SetProjection`.
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolyProjection;

//#region 🔖️Inverse
pub fn inverse(base: &LowpolyProjection, _replacement: &LowpolyProjection) -> Vec<LowpolyMutation> {
    vec![LowpolyMutation::SetProjection { projection: base.clone() }]
}
//#endregion 🔖️Inverse
