//! ↩️ `set-node-positions` — ONE `set-node-positions` row with every placed node's BASE position; a placement that moves
//! nothing has no inverse.

use super::{equation_targets_invariant, EquationNodePosition, SetNodePositions};
use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &SetNodePositions, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::set_node_positions::SetNodePositions>,
{
    Ok((|| {
    if equation_targets_invariant(&payload.ids()).is_err() || payload.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
        return Vec::new();
    }
    let graph = &base.graph;
    let placed: Vec<(EquationNodePosition, bool)> =
        payload.positions.iter().filter_map(|position| graph.nodes.iter().find(|node| node.id == position.id).map(|node| (EquationNodePosition { id: node.id.clone(), x: node.x, y: node.y }, (node.x, node.y) != (position.x, position.y)))).collect();
    if !placed.iter().any(|(_, moves)| *moves) {
        return Vec::new();
    }
    vec![Op::from(SetNodePositions { positions: placed.into_iter().map(|(position, _)| position).collect() })]

    })())
}
//#endregion 🔖️Inverse
