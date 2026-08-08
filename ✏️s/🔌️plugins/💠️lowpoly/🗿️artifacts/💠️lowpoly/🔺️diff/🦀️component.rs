//! 🔺️ Lowpoly artifact — the aggregate diff type: an ordered list of mutations, replayed to
//! materialize a result.

use crate::artifacts::lowpoly::mutations::{apply_lowpoly_mutation, LowpolyMutation};
use crate::artifacts::lowpoly::LowpolyProjection;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Diff
/// @emoji 📦️ A lowpoly diff is just the ordered list of mutations it applies — replaying them over a
/// cloned projection materializes the result and `absorb` concatenates, so a coalesced gesture stays
/// one edit.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LowpolyDiff {
    pub mutations: Vec<LowpolyMutation>,
}

impl MutationDiff<LowpolyProjection> for LowpolyDiff {
    fn apply(&self, projection: &LowpolyProjection) -> LowpolyProjection {
        let mut next = projection.clone();
        for mutation in &self.mutations {
            apply_lowpoly_mutation(&mut next, mutation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.mutations.extend(other.mutations);
    }
}
//#endregion 🔖️Diff
