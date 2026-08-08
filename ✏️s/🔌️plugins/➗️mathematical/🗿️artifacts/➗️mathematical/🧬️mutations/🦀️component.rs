//! 🧬️ Mathematical artifact — document mutation dispatch.

use crate::artifacts::mathematical::diff::MathDiff;
use crate::artifacts::mathematical::{MathGeometry, MathGraph, MathProjection};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum MathMutation {
    SetGraph { graph: MathGraph },
    SetGeometry { geometry: MathGeometry },
}

pub fn apply_math_mutation(projection: &mut MathProjection, mutation: &MathMutation) {
    match mutation {
        MathMutation::SetGraph { graph } => super::set_graph::mutation::apply(projection, graph),
        MathMutation::SetGeometry { geometry } => super::set_geometry::mutation::apply(projection, geometry),
    }
}

pub fn inverse_math_mutation(projection: &MathProjection, mutation: &MathMutation) -> Vec<MathMutation> {
    match mutation {
        MathMutation::SetGraph { .. } => super::set_graph::inverse::inverse(projection),
        MathMutation::SetGeometry { .. } => super::set_geometry::inverse::inverse(projection),
    }
}

impl Mutation<MathProjection> for MathMutation {
    type Diff = MathDiff;

    fn diff(&self, _projection: &MathProjection) -> Self::Diff {
        match self {
            MathMutation::SetGraph { graph } => MathDiff { graph: Some(graph.clone()), geometry: None },
            MathMutation::SetGeometry { geometry } => MathDiff { graph: None, geometry: Some(geometry.clone()) },
        }
    }

    fn inverse(&self, projection: &MathProjection) -> Vec<Self> {
        inverse_math_mutation(projection, self)
    }
}
//#endregion 🔖️Mutations
