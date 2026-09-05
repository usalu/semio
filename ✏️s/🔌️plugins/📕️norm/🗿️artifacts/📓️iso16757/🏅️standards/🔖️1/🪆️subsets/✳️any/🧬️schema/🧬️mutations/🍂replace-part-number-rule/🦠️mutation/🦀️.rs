//! 🔁️ `replace-part-number-rule` — whole-value swap of the part-number derivation rule
//! (`Literal`/`Table`/`Script` variants differ structurally, so this is a `replace`, not a `change`).

use crate::artifacts::iso16757::{part_5::PartNumberRule, Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePartNumberRule {
    pub new_rule: PartNumberRule,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for ReplacePartNumberRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "part-number-rule", kind: "replace-part-number-rule", record: "ReplacedPartNumberRule" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Replace part-number rule".to_string()
    }
}
//#endregion 🔖️Payload
