//! ↩️ Inverse for `ReplaceNode` — recovers the pre-mutation node from `base`.
use super::ReplaceNode;
use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceNode, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.nodes.iter().find(|item| item.id == payload.id).map(|item| vec![Fem3dMutation::ReplaceNode(ReplaceNode { id: payload.id.clone(), new_node: item.clone() })]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
