//! ↩️ Inverse for `MoveSelection` — the whole-record replacements restoring every BASE node and solid the transform
//! moves (exact, never an inverted transform that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::mutations::{replace_node::ReplaceNode, replace_solid::ReplaceSolid, Fem3dMutation};
use crate::Fem3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &MoveSelection, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
    let outcome = super::diff::diff(payload, base);
    if outcome.messages().iter().any(|message| message.level >= protocol::Severity::Error) {
        return Vec::new();
    }
    let diff = outcome.diff();
    let nodes = diff.nodes.iter().flat_map(|delta| &delta.patched).filter_map(|entry| base.nodes.iter().find(|node| node.id == entry.id)).map(|node| Fem3dMutation::ReplaceNode(ReplaceNode { id: node.id.clone(), new_node: node.clone() }));
    let solids = diff.solids.iter().flat_map(|delta| &delta.patched).filter_map(|entry| base.solids.iter().find(|solid| solid.id == entry.id)).map(|solid| Fem3dMutation::ReplaceSolid(ReplaceSolid { id: solid.id.clone(), new_solid: solid.clone() }));
    nodes.chain(solids).collect()
}
//#endregion 🔖️Inverse
