//! 🔺️ Lowpoly artifact — the diff type: an ordered list of operations, replayed to materialize a
//! result. Extracted out of the old `op` crate's `OperationDiff` impl per the taxonomy split.

use crate::artifacts::lowpoly::op::LowpolyOperation;
use crate::artifacts::lowpoly::LowpolyProjection;
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Diff
/// @emoji 📦️ A lowpoly diff is just the ordered list of operations it applies — replaying them over a
/// cloned projection materializes the result and `absorb` concatenates, so a coalesced gesture stays
/// one edit.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LowpolyDiff {
    pub operations: Vec<LowpolyOperation>,
}

impl OperationDiff<LowpolyProjection> for LowpolyDiff {
    fn apply(&self, projection: &LowpolyProjection) -> LowpolyProjection {
        let mut next = projection.clone();
        for operation in &self.operations {
            super::op::apply_lowpoly_operation(&mut next, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.operations.extend(other.operations);
    }
}
//#endregion 🔖️Diff
