//! ✏️ `change-storey-centre-of-stiffness-ym` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeStoreyCentreOfStiffnessYM {
    pub building_id: String,
    pub id: String,
    pub new_centre_of_stiffness_y_m: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeStoreyCentreOfStiffnessYM where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_storey_centre_of_stiffness_y_m::ChangeStoreyCentreOfStiffnessYM> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "storey-centre-of-stiffness-y-m",
        kind: "change-storey-centre-of-stiffness-ym",
        record: "ChangeStoreyCentreOfStiffnessYM",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Centre of stiffness Y", "Steifigkeitsschwerpunkt Y ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-storey-centre-of-stiffness-ym".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeStoreyCentreOfStiffnessYM where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_storey_centre_of_stiffness_y_m::ChangeStoreyCentreOfStiffnessYM> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_storey_centre_of_stiffness_y_m::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_storey_centre_of_stiffness_y_m::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
