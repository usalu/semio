//! ✏️ `change-foundation-liq-unit-wt-nm3` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFoundationLiqUnitWtNM3 {
    pub id: String,
    pub new_liq_unit_weight_n_m3: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeFoundationLiqUnitWtNM3 where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_foundation_liq_unit_wt_n_m3::ChangeFoundationLiqUnitWtNM3> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "foundation-liq-unit-weight-n-m3",
        kind: "change-foundation-liq-unit-wt-nm3",
        record: "ChangeFoundationLiqUnitWtNM3",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Total unit weight of the soil above the layer", "Wichte des Bodens über der Schicht ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-foundation-liq-unit-wt-nm3".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeFoundationLiqUnitWtNM3 where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_foundation_liq_unit_wt_n_m3::ChangeFoundationLiqUnitWtNM3> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_foundation_liq_unit_wt_n_m3::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_foundation_liq_unit_wt_n_m3::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
