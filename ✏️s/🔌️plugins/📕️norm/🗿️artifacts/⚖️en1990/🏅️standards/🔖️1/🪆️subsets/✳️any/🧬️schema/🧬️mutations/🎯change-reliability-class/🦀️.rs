//! 🧬️ `change-reliability-class` mutation for EN 1990.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeReliabilityClass {
    pub new_reliability_class: u8,
}

impl<Op> protocol::MutationKind<En1990Snapshot, Op> for ChangeReliabilityClass where Op: protocol::Mutation<En1990Snapshot, Diff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_reliability_class::ChangeReliabilityClass> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "reliability-class",
        kind: "change-reliability-class",
        record: "ChangedReliabilityClass",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change reliability class", "Zuverlässigkeitsklasse ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1990Snapshot, Op> for ChangeReliabilityClass where Op: protocol::ColdMutation<En1990Snapshot, ReferenceDiff = semio_s_artifact_norm_en1990_diff::En1990Diff> + From<crate::change_reliability_class::ChangeReliabilityClass> {
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1990Snapshot>>::ReferenceDiff> {
        crate::change_reliability_class::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_reliability_class::inverse::inverse(self, base)?
    
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
