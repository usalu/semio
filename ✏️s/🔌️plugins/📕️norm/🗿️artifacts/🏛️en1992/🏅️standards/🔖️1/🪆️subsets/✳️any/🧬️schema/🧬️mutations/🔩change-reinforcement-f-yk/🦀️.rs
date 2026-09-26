//! 🔧 `change-reinforcement-f-yk` — updates f_ck/f_yk on a material grade by id.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeReinforcementFYk {
    pub grade_id: String,
    pub new_f_yk: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeReinforcementFYk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "grade", kind: "change-reinforcement-f-yk", record: "ChangedReinforcementFYk" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change grade {} strength", self.grade_id), &format!("Festigkeit von Sorte {} ändern", self.grade_id))
    }
    fn target(&self) -> Vec<String> { vec![self.grade_id.clone()] }
}
