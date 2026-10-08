//! ➕️ CAD mutation — `CreateNode` payload + `MutationKind` impl.

use crate::mutations::CadMutation;
use crate::{CadNode, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
/// ➕️ Brings a new [`CadNode`] into existence in the scene graph tree.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-node")]
pub struct CreateNode {
    #[dsl(block)]
    pub node: CadNode,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
}

impl MutationKind<CadSnapshot, CadMutation> for CreateNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create node \"{}\"", self.node.label), &format!("Knoten \"{}\" erstellen", self.node.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.node.id.clone()]
    }
, index: None }
//#endregion 🔖️Mutation
