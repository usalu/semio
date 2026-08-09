//! 🧬️ Mathematical artifact — document mutation dispatch.

use crate::artifacts::mathematical::diff::{diff_set_geometry, diff_set_graph, diff_set_snapshot};
use crate::artifacts::mathematical::diff::MathematicalDiff;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalGraph, MathematicalSnapshot};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum MathematicalMutation {
    SetGraph { graph: MathematicalGraph },
    SetGeometry { geometry: MathematicalGeometry },
    SetSnapshot { snapshot: MathematicalSnapshot },
}

pub fn apply_mathematical_mutation(snapshot: &mut MathematicalSnapshot, mutation: &MathematicalMutation) {
    match mutation {
        MathematicalMutation::SetGraph { graph } => super::set_graph::mutation::apply(snapshot, graph),
        MathematicalMutation::SetGeometry { geometry } => super::set_geometry::mutation::apply(snapshot, geometry),
        MathematicalMutation::SetSnapshot { snapshot: replacement } => super::set_snapshot::mutation::apply(snapshot, replacement),
    }
}

pub fn inverse_mathematical_mutation(snapshot: &MathematicalSnapshot, mutation: &MathematicalMutation) -> Vec<MathematicalMutation> {
    match mutation {
        MathematicalMutation::SetGraph { .. } => super::set_graph::inverse::inverse(snapshot),
        MathematicalMutation::SetGeometry { .. } => super::set_geometry::inverse::inverse(snapshot),
        MathematicalMutation::SetSnapshot { .. } => super::set_snapshot::inverse::inverse(snapshot),
    }
}

impl Mutation<MathematicalSnapshot> for MathematicalMutation {
    type Diff = MathematicalDiff;

    fn diff(&self, _snapshot: &MathematicalSnapshot) -> Self::Diff {
        match self {
            MathematicalMutation::SetGraph { graph } => diff_set_graph(graph.clone()),
            MathematicalMutation::SetGeometry { geometry } => diff_set_geometry(geometry.clone()),
            MathematicalMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &MathematicalSnapshot) -> Vec<Self> {
        inverse_mathematical_mutation(snapshot, self)
    }
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_graph_diff_carries_whole_graph() {
        let graph = MathematicalGraph {
            algorithm: "bfs".into(),
            ..MathematicalGraph::default()
        };
        let diff = Mutation::diff(&MathematicalMutation::SetGraph { graph }, &MathematicalSnapshot::default());
        assert!(diff.graph.is_some());
        assert!(diff.geometry.is_none());
    }
}
//#endregion 🧪️Tests
