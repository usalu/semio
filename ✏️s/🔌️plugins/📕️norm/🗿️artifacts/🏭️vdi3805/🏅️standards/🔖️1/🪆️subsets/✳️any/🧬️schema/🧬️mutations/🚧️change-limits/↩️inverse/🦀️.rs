//! ↩️ `change-limits` — undo restores BASE's whole limits facet.

use super::ChangeLimits;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeLimits, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Vdi3805Mutation::ChangeLimits(ChangeLimits { new_limits: base.limits })]

    })())
}
//#endregion 🔖️Inverse
