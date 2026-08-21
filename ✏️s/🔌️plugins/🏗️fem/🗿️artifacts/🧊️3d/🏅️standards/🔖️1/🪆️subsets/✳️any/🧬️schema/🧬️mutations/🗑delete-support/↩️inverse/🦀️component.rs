//! ↩️ Inverse for `DeleteSupport` — recreates the captured support from `base`.
use super::mutation::DeleteSupport;
use crate::artifacts::fem3d::mutations::{create_support, Fem3dMutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteSupport, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    base.supports.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::CreateSupport(create_support::mutation::CreateSupport { support: item.clone() })]).unwrap_or_default()
}
//#endregion 🔖️Inverse
