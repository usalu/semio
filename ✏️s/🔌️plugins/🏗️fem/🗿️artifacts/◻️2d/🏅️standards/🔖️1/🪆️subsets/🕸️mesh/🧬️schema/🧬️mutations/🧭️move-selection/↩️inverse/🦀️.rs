//! ↩️ Inverse for `MoveSelection` — the whole-record replacements restoring every BASE node and region the
//! transform moves (exact, never an inverted transform that would accumulate float error). Nothing moved ⇒
//! `Vec::new()`.
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::mutations::{replace_node::ReplaceNode, replace_region::ReplaceRegion, Fem2dMutation};
use crate::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &MoveSelection, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
    let outcome = super::diff::diff(payload, base);
    if outcome.messages().iter().any(|message| message.level >= protocol::Severity::Error) {
        return Vec::new();
    }
    let diff = outcome.diff();
    let nodes = diff.nodes.iter().flat_map(|delta| &delta.patched).filter_map(|entry| base.nodes.iter().find(|node| node.id == entry.id)).map(|node| Fem2dMutation::ReplaceNode(ReplaceNode { id: node.id.clone(), new_node: node.clone() }));
    let regions = diff.regions.iter().flat_map(|delta| &delta.patched).filter_map(|entry| base.regions.iter().find(|region| region.id == entry.id)).map(|region| Fem2dMutation::ReplaceRegion(ReplaceRegion { id: region.id.clone(), new_region: region.clone() }));
    nodes.chain(regions).collect()
}
//#endregion 🔖️Inverse
