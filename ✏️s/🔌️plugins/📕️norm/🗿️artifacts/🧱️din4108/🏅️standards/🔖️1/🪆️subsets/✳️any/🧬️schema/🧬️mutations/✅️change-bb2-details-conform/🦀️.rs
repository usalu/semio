//! ✅️ `change-bb2-details-conform`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBb2DetailsConform {
    pub new_bb2_details_conform: bool,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeBb2DetailsConform where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_bb2_details_conform::ChangeBb2DetailsConform> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "bb2-details-conform",
        kind: "change-bb2-details-conform",
        record: "ChangedBb2DetailsConform",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Supplement 2 detail conformity", "Konformität der Anschlussdetails nach Beiblatt 2 ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeBb2DetailsConform where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_bb2_details_conform::ChangeBb2DetailsConform> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_bb2_details_conform::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_bb2_details_conform::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
pub mod diff {
    //! 🔺️ `change-bb2-details-conform` diff.

    use crate::change_bb2_details_conform::ChangeBb2DetailsConform;
    use semio_s_artifact_norm_din4108_diff::Din4108Diff;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn diff(payload: &ChangeBb2DetailsConform, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
        if base.bb2_details_conform == payload.new_bb2_details_conform {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "bb2_details_conform already has this value.");
        }
        protocol::MutationOutcome::new(Din4108Diff { bb2_details_conform: Some(payload.new_bb2_details_conform), ..Default::default() })
    }
}
pub mod inverse {
    //! ↩️ `change-bb2-details-conform` inverse.

    use crate::change_bb2_details_conform::ChangeBb2DetailsConform;
    use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

    pub fn inverse<Op>(_payload: &ChangeBb2DetailsConform, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bb2_details_conform::ChangeBb2DetailsConform> {
        Ok((|| {
        vec![Op::from(ChangeBb2DetailsConform { new_bb2_details_conform: base.bb2_details_conform })]

        })())
    }
}
//#endregion 🪢️Mounts
