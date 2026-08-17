//! 🔺️ Mathematical artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::mathematical::schema::diff::MathematicalDiff;
use crate::artifacts::mathematical::schema::MathematicalArtifact;
use crate::artifacts::mathematical::{mathematical_children_from_state, MathematicalGeometry, MathematicalGraph, MathematicalSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::mathematical::schema::diff::*;

//#region 🔖️Apply
impl MathematicalDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &MathematicalArtifact) -> protocol::MutationApplyResult<MathematicalArtifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(notation) = &self.notation {
                next.notation = notation.clone();
            }
            if let Some(results) = &self.results {
                next.results = results.clone();
            }
            if let Some(computed) = &self.computed {
                next.computed = computed.clone();
            }
            if let Some(equation) = &self.equation {
                next.equation = equation.clone();
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
        })
    }
}

impl MutationDiff<MathematicalSnapshot> for MathematicalDiff {
    fn apply(&self, snapshot: &MathematicalSnapshot) -> protocol::MutationApplyResult<MathematicalSnapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(notation) = &self.notation {
                next.notation = notation.clone();
            }
            if let Some(results) = &self.results {
                next.results = results.clone();
            }
            if let Some(computed) = &self.computed {
                next.computed = computed.clone();
            }
            if let Some(equation) = &self.equation {
                next.equation = equation.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.notation.is_some() {
            self.notation = other.notation;
        }
        if other.results.is_some() {
            self.results = other.results;
        }
        if other.computed.is_some() {
            self.computed = other.computed;
        }
        if other.equation.is_some() {
            self.equation = other.equation;
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
/// 🌉 Builds a whole-triple-replace `MathematicalDiff` from a literal `(graph, geometry)` pair —
/// mints and caches all three composed children in one call ([`mathematical_children_from_state`]),
/// then wraps them as the diff's `notation`/`results`/`computed` slots. Every one of this plugin's
/// 14 mutation `diff` functions funnels its final result through this helper, since a graph/
/// geometry-scoped mutation always regenerates all three co-derived children together (text/table/
/// value are three projections of the SAME `(graph, geometry)` state, not independently-editable
/// slots).
pub fn diff_from_state(graph: MathematicalGraph, geometry: MathematicalGeometry) -> MathematicalDiff {
    let (notation, results, computed) = mathematical_children_from_state(&graph, &geometry);
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_from_state_round_trips_through_apply() {
        // 🔎️ `notation`/`results`/`computed` are three co-derived projections of the SAME
        // `(graph, geometry)` pair — a graph-scoped change regenerates all three handles, unlike the
        // old per-slot ("graph slot only") isolation this test named before the migration.
        let base = MathematicalSnapshot::default();
        let mut graph = crate::artifacts::mathematical::mathematical_graph(&base);
        graph.algorithm = "components".into();
        let geometry = crate::artifacts::mathematical::mathematical_geometry(&base);
        let diff = diff_from_state(graph, geometry.clone());
        let applied = diff.apply(&base).expect("valid mutation diff");
        assert_eq!(crate::artifacts::mathematical::mathematical_graph(&applied).algorithm, "components");
        assert_eq!(crate::artifacts::mathematical::mathematical_geometry(&applied), geometry);
    }

    #[test]
    fn absorb_prefers_the_incoming_slots_when_present() {
        let (notation_a, _, _) = mathematical_children_from_state(&MathematicalGraph::default(), &MathematicalGeometry::default());
        let mut first = MathematicalDiff { notation: Some(notation_a), ..Default::default() };
        let (_, results_b, _) = mathematical_children_from_state(&MathematicalGraph::default(), &MathematicalGeometry { points: Vec::new() });
        let second = MathematicalDiff { results: Some(results_b.clone()), ..Default::default() };
        first.absorb(second);
        assert!(first.notation.is_some());
        assert_eq!(first.results, Some(results_b));
    }
}
//#endregion 🧪️Tests
