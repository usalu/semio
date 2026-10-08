//! ↩️ `move-nodes` — ONE absolute `set-node-positions` row putting every moved node back at its BASE position (never a
//! negated offset), so a multi-node drag stays one point-invertible row.

use crate::standards::v1::subsets::graph::schema::mutations::set_node_positions::{equation_targets_invariant, EquationNodePosition, SetNodePositions};
use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveNodes, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if equation_targets_invariant(&payload.ids).is_err() || !payload.dx.is_finite() || !payload.dy.is_finite() || (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    let graph = &base.graph;
    let positions: Vec<EquationNodePosition> = payload.ids.iter().filter_map(|id| graph.nodes.iter().find(|node| &node.id == id)).map(|node| EquationNodePosition { id: node.id.clone(), x: node.x, y: node.y }).collect();
    if positions.is_empty() {
        return Vec::new();
    }
    vec![EquationMutation::SetNodePositions(SetNodePositions { positions })]

    })())
}
//#endregion 🔖️Inverse
