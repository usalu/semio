//! 🎯 `change-reliability-class` — sets the reliability class.

use semio_s_plugin_norm_compliance::ReliabilityClass;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeReliabilityClass {
    pub new_reliability_class: ReliabilityClass,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangeReliabilityClass where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_reliability_class::ChangeReliabilityClass> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "reliability-class", kind: "change-reliability-class", record: "ChangedReliabilityClass" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Change reliability class to {:?}", self.new_reliability_class), &format!("Zuverlässigkeitsklasse auf {:?} ändern", self.new_reliability_class))
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangeReliabilityClass where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_reliability_class::ChangeReliabilityClass> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> {
        crate::change_reliability_class::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
