//! ➕️ `insert-connection` — inserts one timber connection at an index (clamped to the list end).
use semio_s_artifact_norm_en1995_model::{En1995Snapshot, TimberConnection};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertConnection { pub index: usize, pub connection: TimberConnection }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for InsertConnection where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_connection::RemoveConnection> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "connection", kind: "insert-connection", record: "InsertedConnection" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert connection {} at #{}", self.connection.id, self.index), &format!("Verbindung {} an #{} einfügen", self.connection.id, self.index)) }
    fn target(&self) -> Vec<String> { vec![self.connection.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for InsertConnection where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::remove_connection::RemoveConnection> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::insert_connection::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_connection::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
