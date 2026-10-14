//! `upsert-cold-formed-member` — upsert a `ColdFormedMember` by id into `cold_formed_members`.

use semio_s_artifact_norm_en1993_model::{ColdFormedMember, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateColdFormedInputs {
    pub cold_formed_member: ColdFormedMember,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for UpdateColdFormedInputs where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_cold_formed_inputs::UpdateColdFormedInputs> + From<crate::remove_cold_formed_member::RemoveColdFormedMember> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "coldFormedMember", kind: "update-cold-formed-inputs", record: "UpdatedColdFormedMember" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1993Snapshot>>::Diff> {
        crate::update_cold_formed_inputs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_cold_formed_inputs::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update cold-formed member {}", self.cold_formed_member.id),
            &format!("Kaltprofil-Bauteil {} aktualisieren", self.cold_formed_member.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.cold_formed_member.id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
