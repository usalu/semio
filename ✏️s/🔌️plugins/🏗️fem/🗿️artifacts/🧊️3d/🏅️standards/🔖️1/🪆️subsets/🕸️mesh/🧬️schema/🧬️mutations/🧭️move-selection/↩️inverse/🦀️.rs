//! ↩️ Inverse for `MoveSelection` — the whole-record replacements restoring every BASE node and solid the transform
//! moves (exact, never an inverted transform that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::mutations::{replace_node::ReplaceNode,replace_solid::ReplaceSolid,Fem3dMutation};

use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &MoveSelection, base: &Fem3dSnapshot) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    if payload.breach().is_some() {
        return Ok(Vec::new());
    }
    let nodes = base.nodes.iter().filter(|node| payload.node_ids.contains(&node.id) && payload.moved_node(node).is_some()).map(|node| Fem3dMutation::ReplaceNode(ReplaceNode { id: node.id.clone(), new_node: node.clone() }));
    let solids = base.solids.iter().filter(|solid| payload.solid_ids.contains(&solid.id) && payload.moved_solid(solid).is_some()).map(|solid| Fem3dMutation::ReplaceSolid(ReplaceSolid { id: solid.id.clone(), new_solid: solid.clone() }));
    Ok(nodes.chain(solids).collect())
}
//#endregion 🔖️Inverse
