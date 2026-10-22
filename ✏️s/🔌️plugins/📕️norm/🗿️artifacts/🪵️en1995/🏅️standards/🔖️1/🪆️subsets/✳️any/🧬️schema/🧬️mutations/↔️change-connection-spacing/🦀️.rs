//! ↔️ `change-connection-spacing` — changes `spacingM` (Spacing a₁) on one addressed connection.
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeConnectionSpacing { pub connection_id: String, pub new_value: f64 }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for ChangeConnectionSpacing where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::change_connection_spacing::ChangeConnectionSpacing> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "connection-spacingM", kind: "change-connection-spacing", record: "ChangedConnectionSpacing" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Spacing a₁ of connection {}", self.connection_id), &format!("Abstand a₁ von Verbindung {} ändern", self.connection_id)) }
    fn target(&self) -> Vec<String> { vec![self.connection_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for ChangeConnectionSpacing where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::change_connection_spacing::ChangeConnectionSpacing> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::change_connection_spacing::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_connection_spacing::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
