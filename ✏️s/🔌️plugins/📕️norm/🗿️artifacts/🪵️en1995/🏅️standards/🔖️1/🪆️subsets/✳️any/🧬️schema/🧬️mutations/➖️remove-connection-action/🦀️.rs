//! ➖️ `remove-connection-action` — removes the characteristic action at an index from a connection's action table.
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveConnectionAction { pub connection_id: String, pub index: usize }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for RemoveConnectionAction where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::insert_connection_action::InsertConnectionAction> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "connection-action", kind: "remove-connection-action", record: "RemovedConnectionAction" };
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1995Snapshot>>::Diff> { crate::remove_connection_action::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::remove_connection_action::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove action #{} from connection {}", self.index, self.connection_id), &format!("Einwirkung #{} aus Verbindung {} entfernen", self.index, self.connection_id)) }
    fn target(&self) -> Vec<String> { vec![self.connection_id.clone(), self.index.to_string()] }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
