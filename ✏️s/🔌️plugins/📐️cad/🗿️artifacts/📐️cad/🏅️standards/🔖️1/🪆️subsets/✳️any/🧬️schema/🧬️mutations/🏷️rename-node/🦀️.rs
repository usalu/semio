//! 🏷️ CAD mutation — `RenameNode` payload + `MutationKind` impl.

use crate::mutations::CadMutation;
use crate::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
/// 🏷️ Renames an existing [`crate::CadNode`]'s `label`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-node")]
pub struct RenameNode {
    pub node_id: String,
    pub new_label: String,
}

impl MutationKind<CadSnapshot, CadMutation> for RenameNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "node", kind: "rename-node", record: "RenamedNode" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename node to \"{}\"", self.new_label), &format!("Knoten in \"{}\" umbenennen", self.new_label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
