//! 🧾️ Equation editor command — `edit-equation`: the edited graph and point geometry as the concrete kinds of every changed
//! field (`change-graph-directed`, `update-graph-algorithm`, `disconnect-nodes`, `delete-node`, `create-node`,
//! `change-node-label`, `set-node-positions`, `connect-nodes`, `insert-point`, `remove-point`, `set-point-positions`) — never a
//! whole-graph or whole-geometry replace.

use crate::op::EquationMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::EquationGraphDsl;
use crate::standards::v1::subsets::geometry::schema::mutations::{insert_point::InsertPoint, remove_point::RemovePoint, set_point_positions::{EquationPointPosition, SetPointPositions}};
use crate::standards::v1::subsets::graph::schema::mutations::{
    change_graph_directed::ChangeGraphDirected, change_node_label::ChangeNodeLabel, connect_nodes::ConnectNodes, create_node::CreateNode, delete_node::DeleteNode, disconnect_nodes::DisconnectNodes,
    set_node_positions::{EquationNodePosition, SetNodePositions}, update_graph_algorithm::UpdateGraphAlgorithm,
};
use crate::{EquationGeometry, EquationGraph, EquationPoint, EquationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit-equation")]
pub struct EditEquation {
    #[dsl(block)]
    pub graph: EquationGraphDsl,
    #[dsl(block)]
    pub geometry: EquationGeometry,
}

type GraphEditRule = fn(&EquationGraph, &EquationGraph) -> Vec<EquationMutation>;

/// 📐️ The per-field edit-rules table of the graph (norm's `EDIT_RULES` shape), in emission order: scalars, then edge removals
/// (before the node deletions that would strand them), node deletions, position-exact node creations, node label and position
/// edits, and position-exact edge creations — so every index is an index of the edited list.
const GRAPH_EDIT_RULES: [GraphEditRule; 6] = [
    |base, next| (base.directed != next.directed).then(|| EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: next.directed })).into_iter().collect(),
    |base, next| (base.algorithm != next.algorithm || base.algorithm_seed != next.algorithm_seed).then(|| EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: next.algorithm.clone(), new_algorithm_seed: next.algorithm_seed.clone() })).into_iter().collect(),
    |base, next| {
        let kept: BTreeMap<&str, &crate::EquationEdge> = next.edges.iter().map(|edge| (edge.id.as_str(), edge)).collect();
        let severed = base.edges.iter().filter(|edge| kept.get(edge.id.as_str()).is_none_or(|next| next != edge));
        let removed_nodes: BTreeSet<&str> = base.nodes.iter().filter(|node| next.nodes.iter().all(|kept| kept.id != node.id)).map(|node| node.id.as_str()).collect();
        let stranded = base.edges.iter().filter(|edge| removed_nodes.contains(edge.source.as_str()) || removed_nodes.contains(edge.target.as_str()));
        let mut rows: Vec<EquationMutation> = Vec::new();
        for edge in severed.chain(stranded) {
            if !rows.iter().any(|row| matches!(row, EquationMutation::DisconnectNodes(DisconnectNodes { id }) if *id == edge.id)) {
                rows.push(EquationMutation::DisconnectNodes(DisconnectNodes { id: edge.id.clone() }));
            }
        }
        let leaves = rows.into_iter().chain(base.nodes.iter().filter(|node| removed_nodes.contains(node.id.as_str())).map(|node| EquationMutation::DeleteNode(DeleteNode { id: node.id.clone() })));
        leaves.collect()
    },
    |base, next| {
        let held: BTreeSet<&str> = base.nodes.iter().map(|node| node.id.as_str()).collect();
        let created = next.nodes.iter().enumerate().filter(|(_, node)| !held.contains(node.id.as_str()));
        created.map(|(index, node)| EquationMutation::CreateNode(CreateNode { id: node.id.clone(), label: node.label.clone(), x: node.x, y: node.y, index: Some(index) })).collect()
    },
    |base, next| {
        let held: BTreeMap<&str, &crate::EquationNode> = base.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let labels = next.nodes.iter().filter_map(|node| held.get(node.id.as_str()).filter(|held| held.label != node.label).map(|_| EquationMutation::ChangeNodeLabel(ChangeNodeLabel { id: node.id.clone(), new_label: node.label.clone() })));
        let moved: Vec<EquationNodePosition> = next.nodes.iter().filter(|node| held.get(node.id.as_str()).is_some_and(|held| (held.x, held.y) != (node.x, node.y))).map(|node| EquationNodePosition { id: node.id.clone(), x: node.x, y: node.y }).collect();
        labels.chain((!moved.is_empty()).then(|| EquationMutation::SetNodePositions(SetNodePositions { positions: moved }))).collect()
    },
    |base, next| {
        let held: BTreeMap<&str, &crate::EquationEdge> = base.edges.iter().map(|edge| (edge.id.as_str(), edge)).collect();
        let created = next.edges.iter().enumerate().filter(|(_, edge)| held.get(edge.id.as_str()).is_none_or(|held| held != edge));
        created.map(|(index, edge)| EquationMutation::ConnectNodes(ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), index: Some(index) })).collect()
    },
];

/// 🧮️ The concrete kinds that turn `base` into `next`.
pub(crate) fn equation_graph_edit_leaves(base: &EquationGraph, next: &EquationGraph) -> Vec<EquationMutation> {
    GRAPH_EDIT_RULES.iter().flat_map(|rule| rule(base, next)).collect()
}

/// 📍️ The concrete point kinds that turn `base` into `next`: the shared prefix re-positions in one `set-point-positions` row,
/// a longer list appends `insert-point` rows at their exact index, a shorter one drops the tail last-first with `remove-point`.
pub(crate) fn equation_point_edit_leaves(base: &[EquationPoint], next: &[EquationPoint]) -> Vec<EquationMutation> {
    let moved: Vec<EquationPointPosition> = base.iter().zip(next).enumerate().filter(|(_, (base, next))| base != next).map(|(index, (_, next))| EquationPointPosition { index, x: next.x, y: next.y }).collect();
    let positions = (!moved.is_empty()).then(|| EquationMutation::SetPointPositions(SetPointPositions { positions: moved }));
    let inserted = next.iter().enumerate().skip(base.len()).map(|(index, point)| EquationMutation::InsertPoint(InsertPoint { index, x: point.x, y: point.y }));
    let removed = (next.len()..base.len()).rev().map(|index| EquationMutation::RemovePoint(RemovePoint { index }));
    positions.into_iter().chain(inserted).chain(removed).collect()
}

pub fn handle(payload: &EditEquation, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    let Ok(graph) = crate::standards::v1::subsets::any::io::text::snapshot::math_graph_from_dsl(payload.graph.clone()) else {
        return Ok(Emit::default());
    };
    let leaves = equation_graph_edit_leaves(&doc.snapshot.graph, &graph).into_iter().chain(equation_point_edit_leaves(&doc.snapshot.geometry.points, &payload.geometry.points)).collect();
    Ok(Emit::mutations(leaves))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
