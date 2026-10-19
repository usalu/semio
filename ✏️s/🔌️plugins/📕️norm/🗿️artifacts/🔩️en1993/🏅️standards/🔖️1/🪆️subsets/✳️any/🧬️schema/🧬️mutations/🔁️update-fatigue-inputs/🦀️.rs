//! 🧬️ `upsert-fatigue-detail` — upsert a `FatigueDetail` by id into `fatigue_details`.

use semio_s_artifact_norm_en1993_model::{FatigueDetail, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateFatigueInputs {
    pub fatigue_detail: FatigueDetail,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for UpdateFatigueInputs where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::update_fatigue_inputs::UpdateFatigueInputs> + From<crate::remove_fatigue_detail::RemoveFatigueDetail> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "fatigueDetail", kind: "update-fatigue-inputs", record: "UpdatedFatigueDetail" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<En1993Snapshot>>::Diff> {
        crate::update_fatigue_inputs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::update_fatigue_inputs::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update fatigue detail {}", self.fatigue_detail.id),
            &format!("Kerbdetail {} aktualisieren", self.fatigue_detail.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.fatigue_detail.id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
