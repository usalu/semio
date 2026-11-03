//! 📍️ `change-tie-beam-relative-displacement` mutation leaf: sets `relative-displacement` of one tie_beam.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeTieBeamRelativeDisplacement {
    pub id: String,
    pub new_relative_displacement: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeTieBeamRelativeDisplacement where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_tie_beam_relative_displacement::ChangeTieBeamRelativeDisplacement> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "tie-beam-relative-displacement",
        kind: "change-tie-beam-relative-displacement",
        record: "ChangedTieBeamYaw",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change tie beam: Relative ground displacement", "Fundamentbalken: Relative Bodenverschiebung ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeTieBeamRelativeDisplacement where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_tie_beam_relative_displacement::ChangeTieBeamRelativeDisplacement> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1998_diff::En1998Diff> {
        crate::change_tie_beam_relative_displacement::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_tie_beam_relative_displacement::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
