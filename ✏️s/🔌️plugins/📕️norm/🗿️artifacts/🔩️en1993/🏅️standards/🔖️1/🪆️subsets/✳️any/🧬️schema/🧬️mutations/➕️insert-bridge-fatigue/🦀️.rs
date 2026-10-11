//! ➕️ insert-bridge-fatigue
use crate::{BridgeFatigue, En1993Mutation, En1993Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertBridgeFatigue { pub index: Option<usize>, pub bridge_fatigue_item: BridgeFatigue }
impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for InsertBridgeFatigue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "bridge-fatigue", kind: "insert-bridge-fatigue", record: "InsertedBridgeFatigue" };
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert bridge fatigue verification at position #{}", self.index.map_or_else(|| "end".to_string(), |index| index.to_string())), &format!("Ermüdungsnachweis der Brücke an Position #{} einfügen", self.index.map_or_else(|| "end".to_string(), |index| index.to_string()))) }
    fn target(&self) -> Vec<String> { vec![self.index.map_or_else(|| "end".to_string(), |index| index.to_string())] }
}
