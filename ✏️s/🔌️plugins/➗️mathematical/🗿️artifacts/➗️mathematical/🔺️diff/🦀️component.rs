//! 🔺️ Mathematical artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::mathematical::diff::schema::MathematicalDiff;
use crate::artifacts::mathematical::schema::MathematicalArtifact;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalGraph, MathematicalSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

//#region 🔖️Apply
impl MathematicalDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &MathematicalArtifact) -> MathematicalArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(graph) = &self.graph {
            next.graph = graph.clone();
        }
        if let Some(geometry) = &self.geometry {
            next.geometry = geometry.clone();
        }
        if let Some(value) = self.camera_x {
            next.camera_x = value;
        }
        if let Some(value) = self.camera_y {
            next.camera_y = value;
        }
        if let Some(value) = self.camera_zoom {
            next.camera_zoom = value;
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

impl MutationDiff<MathematicalSnapshot> for MathematicalDiff {
    fn apply(&self, snapshot: &MathematicalSnapshot) -> MathematicalSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(graph) = &self.graph {
            next.graph = graph.clone();
        }
        if let Some(geometry) = &self.geometry {
            next.geometry = geometry.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            self.artifact = other.artifact;
            self.graph = None;
            self.geometry = None;
            self.camera_x = None;
            self.camera_y = None;
            self.camera_zoom = None;
            self.locale = None;
            return;
        }
        if other.graph.is_some() {
            self.graph = other.graph;
        }
        if other.geometry.is_some() {
            self.geometry = other.geometry;
        }
        if other.camera_x.is_some() {
            self.camera_x = other.camera_x;
        }
        if other.camera_y.is_some() {
            self.camera_y = other.camera_y;
        }
        if other.camera_zoom.is_some() {
            self.camera_zoom = other.camera_zoom;
        }
        if other.locale.is_some() {
            self.locale = other.locale;
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
/// 🖼️ Whole-artifact replacement from a snapshot (UI fields defaulted).
pub fn diff_set_snapshot(snapshot: &MathematicalSnapshot) -> MathematicalDiff {
    MathematicalDiff {
        artifact: Some(Box::new(MathematicalArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

pub fn diff_set_graph(graph: MathematicalGraph) -> MathematicalDiff {
    MathematicalDiff {
        graph: Some(graph),
        ..Default::default()
    }
}

pub fn diff_set_geometry(geometry: MathematicalGeometry) -> MathematicalDiff {
    MathematicalDiff {
        geometry: Some(geometry),
        ..Default::default()
    }
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_only_diff_touches_only_the_graph_slot() {
        let base = MathematicalSnapshot::default();
        let diff = MathematicalDiff {
            graph: Some(MathematicalGraph {
                algorithm: "components".into(),
                ..MathematicalGraph::default()
            }),
            ..Default::default()
        };
        let applied = diff.apply(&base);
        assert_eq!(applied.graph.algorithm, "components");
        assert_eq!(applied.geometry, base.geometry);
    }

    #[test]
    fn absorb_prefers_the_incoming_slots_when_present() {
        let mut first = MathematicalDiff {
            graph: Some(MathematicalGraph::default()),
            ..Default::default()
        };
        let second = MathematicalDiff {
            geometry: Some(MathematicalGeometry { points: Vec::new() }),
            ..Default::default()
        };
        first.absorb(second);
        assert!(first.graph.is_some());
        assert_eq!(first.geometry, Some(MathematicalGeometry { points: Vec::new() }));
    }
}
//#endregion 🧪️Tests
