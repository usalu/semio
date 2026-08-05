//! 🔺️ Mathematical artifact — the operation diff (constitutional: diff).

use crate::artifacts::mathematical::{MathGeometry, MathGraph, MathProjection};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 📤️ Coarse-grained diff: each field replaces one top-level projection slice.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MathDiff {
    #[serde(default)]
    pub graph: Option<MathGraph>,
    #[serde(default)]
    pub geometry: Option<MathGeometry>,
}

impl OperationDiff<MathProjection> for MathDiff {
    fn apply(&self, projection: &MathProjection) -> MathProjection {
        let mut next = projection.clone();
        if let Some(graph) = &self.graph {
            next.graph = graph.clone();
        }
        if let Some(geometry) = &self.geometry {
            next.geometry = geometry.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.graph.is_some() {
            self.graph = other.graph;
        }
        if other.geometry.is_some() {
            self.geometry = other.geometry;
        }
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_only_diff_touches_only_the_graph_slot() {
        let base = MathProjection::default();
        let diff = MathDiff { graph: Some(MathGraph { algorithm: "components".into(), ..MathGraph::default() }), geometry: None };
        let applied = diff.apply(&base);
        assert_eq!(applied.graph.algorithm, "components");
        assert_eq!(applied.geometry, base.geometry);
    }

    #[test]
    fn absorb_prefers_the_incoming_slots_when_present() {
        let mut first = MathDiff { graph: Some(MathGraph::default()), geometry: None };
        let second = MathDiff { graph: None, geometry: Some(MathGeometry { points: Vec::new() }) };
        first.absorb(second);
        assert!(first.graph.is_some());
        assert_eq!(first.geometry, Some(MathGeometry { points: Vec::new() }));
    }
}
//#endregion 🧪️Tests
