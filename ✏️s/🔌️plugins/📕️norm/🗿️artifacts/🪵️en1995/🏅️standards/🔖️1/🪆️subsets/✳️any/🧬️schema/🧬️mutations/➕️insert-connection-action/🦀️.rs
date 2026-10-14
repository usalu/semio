//! ➕️ `insert-connection-action` — inserts one characteristic action into a connection's action table.
use semio_s_artifact_norm_en1995_model::{ConnectionAction, En1995Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertConnectionAction { pub connection_id: String, pub index: usize, pub action: ConnectionAction }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for InsertConnectionAction where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_connection_action::RemoveConnectionAction> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "connection-action", kind: "insert-connection-action", record: "InsertedConnectionAction" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1995Snapshot>>::Diff> { crate::insert_connection_action::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_connection_action::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert action {} into connection {} at #{}", self.action.id, self.connection_id, self.index), &format!("Einwirkung {} in Verbindung {} an #{} einfügen", self.action.id, self.connection_id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.connection_id.clone(), self.action.id.clone()] }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
