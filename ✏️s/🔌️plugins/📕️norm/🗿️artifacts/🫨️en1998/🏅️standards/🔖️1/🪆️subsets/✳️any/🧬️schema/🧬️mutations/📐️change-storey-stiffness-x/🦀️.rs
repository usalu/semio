//! 📐️ `change-storey-stiffness-x` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeStoreyStiffnessX {
    pub building_index: usize,
    pub storey_index: usize,
    pub new_stiffness_x: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeStoreyStiffnessX where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_storey_stiffness_x::ChangeStoreyStiffnessX> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "storey-stiffness-x",
        kind: "change-storey-stiffness-x",
        record: "ChangeStoreyStiffnessX",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change storey stiffness", "Geschosssteifigkeit ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-storey-stiffness-x".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeStoreyStiffnessX where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_storey_stiffness_x::ChangeStoreyStiffnessX> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_storey_stiffness_x::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_storey_stiffness_x::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
