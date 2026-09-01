//! 🏷️ `rename-step` payload — changes an id-keyed [`ProcessStep`]'s `label` (its identity/display
//! field — the step's `id` itself is immutable).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::rename_step::RenameStep;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RenameStep
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenameStep {
    pub id: String,
    pub new_label: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for RenameStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "step", kind: "rename-step", record: "RenamedStep" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename step to \"{}\"", self.new_label)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️RenameStep
