//! ↩️ Inverse for `SetSnapshot`.
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &LowpolySnapshot, _replacement: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    vec![LowpolyMutation::SetSnapshot { snapshot: base.clone() }]
}
//#endregion 🔖️Inverse
