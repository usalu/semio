//! 🔺️ Equation artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::equation::schema::EquationArtifact;
use crate::artifacts::equation::{equation_children_from_state, EquationGeometry, EquationGraph, EquationSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::equation::schema::diff::*;

//#region 🔖️Apply
impl EquationDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &EquationArtifact) -> protocol::MutationApplyResult<EquationArtifact> {
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

impl MutationDiff<EquationSnapshot> for EquationDiff {
    async fn apply(&self, snapshot: &EquationSnapshot) -> protocol::MutationApplyResult<EquationSnapshot> {
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
    async fn absorb(&mut self, other: Self) {
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
/// 🌉 Builds a whole-triple-replace `EquationDiff` from a literal `(graph, geometry)` pair —
/// mints and caches all three composed children in one call ([`equation_children_from_state`]),
/// then wraps them as the diff's `notation`/`results`/`computed` slots. Every one of this plugin's
/// 14 mutation `diff` functions funnels its final result through this helper, since a graph/
/// geometry-scoped mutation always regenerates all three co-derived children together (text/table/
/// value are three projections of the SAME `(graph, geometry)` state, not independently-editable
/// slots).
pub async fn diff_from_state(graph: EquationGraph, geometry: EquationGeometry) -> EquationDiff {
    let (notation, results, computed) = equation_children_from_state(&graph, &geometry);
    EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn diff_from_state_round_trips_through_apply() {
        // 🔎️ `notation`/`results`/`computed` are three co-derived projections of the SAME
        // `(graph, geometry)` pair — a graph-scoped change regenerates all three handles, unlike the
        // old per-slot ("graph slot only") isolation this test named before the migration.
        let base = EquationSnapshot::default();
        let mut graph = crate::artifacts::equation::equation_graph(&base);
        graph.algorithm = "components".into();
        let geometry = crate::artifacts::equation::equation_geometry(&base);
        let diff = diff_from_state(graph, geometry.clone());
        let applied = diff.apply(&base).expect("valid mutation diff");
        assert_eq!(crate::artifacts::equation::equation_graph(&applied).algorithm, "components");
        assert_eq!(crate::artifacts::equation::equation_geometry(&applied), geometry);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_prefers_the_incoming_slots_when_present() {
        let (notation_a, _, _) = equation_children_from_state(&EquationGraph::default(), &EquationGeometry::default());
        let mut first = EquationDiff { notation: Some(notation_a), ..Default::default() };
        let (_, results_b, _) = equation_children_from_state(&EquationGraph::default(), &EquationGeometry { points: Vec::new() });
        let second = EquationDiff { results: Some(results_b.clone()), ..Default::default() };
        first.absorb(second);
        assert!(first.notation.is_some());
        assert_eq!(first.results, Some(results_b));
    }
}
//#endregion 🧪️Tests
