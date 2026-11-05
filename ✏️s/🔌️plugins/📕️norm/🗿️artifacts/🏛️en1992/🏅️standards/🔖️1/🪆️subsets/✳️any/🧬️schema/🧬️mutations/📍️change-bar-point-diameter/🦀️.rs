//! 📍 `change-bar-point-diameter` mutation leaf: sets `diameter` of one bar point.

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBarPointDiameter {
    pub member_id: String,
    pub bar_id: String,
    pub new_diameter: f64,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for ChangeBarPointDiameter where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_bar_point_diameter::ChangeBarPointDiameter> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "bar-point-diameter",
        kind: "change-bar-point-diameter",
        record: "ChangedBarPointDiameter",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change bar diameter Ø of the bar point", "Bewehrungsstab: Stabdurchmesser Ø ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.member_id.clone(), self.bar_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for ChangeBarPointDiameter where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_bar_point_diameter::ChangeBarPointDiameter> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1992_diff::En1992Diff> {
        crate::change_bar_point_diameter::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_bar_point_diameter::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
