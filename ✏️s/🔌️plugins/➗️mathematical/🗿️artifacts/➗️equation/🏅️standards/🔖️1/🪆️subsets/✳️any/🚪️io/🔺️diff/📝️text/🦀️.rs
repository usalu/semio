//! 🔺️ Equation artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::EquationArtifact;
use crate::{equation_children_from_state, EquationGeometry, EquationGraph, EquationSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::schema::diff::*;

//#region 🔖️Apply
impl EquationDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &EquationArtifact) -> protocol::MutationApplyResult<EquationArtifact> {
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
    fn apply(&self, snapshot: &EquationSnapshot) -> protocol::MutationApplyResult<EquationSnapshot> {
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
/// 🌉 Builds a whole-triple-replace `EquationDiff` from a literal `(graph, geometry)` pair —
/// mints and caches all three composed children in one call ([`equation_children_from_state`]),
/// then wraps them as the diff's `notation`/`results`/`computed` slots. Every one of this plugin's
/// 14 mutation `diff` functions funnels its final result through this helper, since a graph/
/// geometry-scoped mutation always regenerates all three co-derived children together (text/table/
/// value are three projections of the SAME `(graph, geometry)` state, not independently-editable
/// slots).
pub fn diff_from_state(graph: &EquationGraph, geometry: &EquationGeometry) -> EquationDiff {
    let (notation, results, computed) = equation_children_from_state(graph, geometry);
    EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
