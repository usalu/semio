//! ↩️ Inverse for `DeleteSupport` — recreates the captured support from `base`.
use super::mutation::DeleteSupport;
use crate::artifacts::fem2d::mutations::{create_support, Fem2dMutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteSupport, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    base.supports.iter().find(|item| item.id == payload.id).map(|item| vec![Fem2dMutation::CreateSupport(create_support::mutation::CreateSupport { support: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
