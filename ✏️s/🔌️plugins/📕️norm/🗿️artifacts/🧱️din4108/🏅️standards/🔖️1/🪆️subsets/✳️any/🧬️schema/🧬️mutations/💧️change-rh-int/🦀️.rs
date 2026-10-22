//! 💧️ `change-rh-int`.

use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeRhInt {
    pub new_rh_int: f64,
}

impl<Op> protocol::MutationKind<Din4108Snapshot, Op> for ChangeRhInt where Op: protocol::Mutation<Din4108Snapshot, Diff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_rh_int::ChangeRhInt> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "rh-int",
        kind: "change-rh-int",
        record: "ChangedRhInt",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change indoor relative humidity", "Relative Raumluftfeuchte ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Din4108Snapshot, Op> for ChangeRhInt where Op: protocol::ColdMutation<Din4108Snapshot, ReferenceDiff = semio_s_artifact_norm_din4108_diff::Din4108Diff> + From<crate::change_rh_int::ChangeRhInt> {
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Din4108Snapshot>>::ReferenceDiff> {
        crate::change_rh_int::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_rh_int::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
