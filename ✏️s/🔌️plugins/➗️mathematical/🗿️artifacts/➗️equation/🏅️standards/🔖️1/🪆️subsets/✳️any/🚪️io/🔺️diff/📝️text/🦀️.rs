//! 🔺️ Equation artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::EquationArtifact;
use crate::EquationSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::schema::diff::*;

//#region 🔖️Apply
impl EquationDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &EquationArtifact) -> protocol::MutationApplyResult<EquationArtifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(graph) = &self.graph {
                next.graph = graph.clone();
            }
            if let Some(geometry) = &self.geometry {
                next.geometry = geometry.clone();
            }
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
}

impl MutationDiff<EquationSnapshot> for EquationDiff {
    fn apply(&self, snapshot: &EquationSnapshot) -> protocol::MutationApplyResult<EquationSnapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(graph) = &self.graph {
                next.graph = graph.clone();
            }
            if let Some(geometry) = &self.geometry {
                next.geometry = geometry.clone();
            }
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
        if other.graph.is_some() {
            self.graph = other.graph;
        }
        if other.geometry.is_some() {
            self.geometry = other.geometry;
        }
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
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
