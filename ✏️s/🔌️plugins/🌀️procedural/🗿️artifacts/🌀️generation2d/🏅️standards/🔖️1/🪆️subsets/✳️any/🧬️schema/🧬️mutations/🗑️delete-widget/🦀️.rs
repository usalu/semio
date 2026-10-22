//! 🗑️ Generation2d mutation — `DeleteWidget`: removes an id-keyed widget (captures the removed
//! widget for its inverse). Wired module name (`remove_widget`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DeleteWidget
/// 🗑️ `delete-widget` payload — removes the widget with `id`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteWidget {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_widget<Op: From<DeleteWidget>>(id: String) -> Op {
    Op::from(DeleteWidget { id })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for DeleteWidget
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::create_widget::CreateWidget>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "widget", kind: "delete-widget", record: "DeletedWidget" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete widget \"{}\"", self.id), &format!("Widget \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for DeleteWidget
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<crate::create_widget::CreateWidget>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::delete_widget::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::delete_widget::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️DeleteWidget
