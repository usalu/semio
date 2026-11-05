//! 📐️ `change-section-point-y` mutation leaf: sets `y` of one section point.

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSectionPointY {
    pub member_id: String,
    pub section_point_id: String,
    pub new_y: f64,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for ChangeSectionPointY where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_section_point_y::ChangeSectionPointY> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "section-point-y",
        kind: "change-section-point-y",
        record: "ChangedSectionPointY",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change coordinate y of the section point", "Querschnittspunkt: Koordinate y ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.member_id.clone(), self.section_point_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for ChangeSectionPointY where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::change_section_point_y::ChangeSectionPointY> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1992_diff::En1992Diff> {
        crate::change_section_point_y::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_section_point_y::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
