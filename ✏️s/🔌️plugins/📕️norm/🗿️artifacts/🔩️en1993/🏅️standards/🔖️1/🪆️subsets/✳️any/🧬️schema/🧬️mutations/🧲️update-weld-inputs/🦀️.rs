//! 🧬️ `upsert-member-action` — upsert a `MemberAction` by id into `member_actions`.

use semio_s_artifact_norm_en1993_model::{MemberAction, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateWeldInputs {
    pub member_action: MemberAction,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for UpdateWeldInputs where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_weld_inputs::UpdateWeldInputs> + From<crate::remove_member_action::RemoveMemberAction> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "memberAction", kind: "update-weld-inputs", record: "UpdatedMemberAction" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update member action {}", self.member_action.id),
            &format!("Bauteilbeanspruchung {} aktualisieren", self.member_action.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.member_action.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for UpdateWeldInputs where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_weld_inputs::UpdateWeldInputs> + From<crate::remove_member_action::RemoveMemberAction> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> {
        crate::update_weld_inputs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_weld_inputs::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
