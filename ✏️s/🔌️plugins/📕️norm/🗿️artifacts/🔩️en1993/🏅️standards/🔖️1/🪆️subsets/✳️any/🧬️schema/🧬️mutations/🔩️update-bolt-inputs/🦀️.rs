//! 🧬️ `upsert-joint` — upsert a `SteelJoint` by id into `joints`.

use semio_s_artifact_norm_en1993_model::{SteelJoint, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateBoltInputs {
    pub joint: SteelJoint,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for UpdateBoltInputs where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_bolt_inputs::UpdateBoltInputs> + From<crate::remove_joint::RemoveJoint> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "joint", kind: "update-bolt-inputs", record: "UpdatedJoint" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update joint {}", self.joint.id),
            &format!("Anschluss {} aktualisieren", self.joint.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.joint.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for UpdateBoltInputs where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_bolt_inputs::UpdateBoltInputs> + From<crate::remove_joint::RemoveJoint> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> {
        crate::update_bolt_inputs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_bolt_inputs::inverse::inverse(self, base)?
    
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
