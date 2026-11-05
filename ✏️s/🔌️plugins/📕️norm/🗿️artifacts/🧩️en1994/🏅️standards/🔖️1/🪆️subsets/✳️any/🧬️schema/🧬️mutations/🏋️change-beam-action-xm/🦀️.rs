//! 🏋️ `change-beam-action-xm` mutation leaf: sets `x_m` of one beam action.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBeamActionXM {
    pub beam_id: String,
    pub id: String,
    pub new_x_m: f64,
}

impl<Op> protocol::MutationKind<En1994Snapshot, Op> for ChangeBeamActionXM where Op: protocol::Mutation<En1994Snapshot, Diff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_beam_action_x_m::ChangeBeamActionXM> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "beam-action-x-m",
        kind: "change-beam-action-xm",
        record: "ChangedBeamActionXM",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change beam position of the point force along the span of the beam action", "Trägereinwirkung: Trägereinwirkung: Position der Einzellast entlang der Spannweite ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.beam_id.clone(), self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1994Snapshot, Op> for ChangeBeamActionXM where Op: protocol::ColdMutation<En1994Snapshot, ReferenceDiff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::change_beam_action_x_m::ChangeBeamActionXM> {
    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1994_diff::En1994Diff> {
        crate::change_beam_action_x_m::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_beam_action_x_m::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
