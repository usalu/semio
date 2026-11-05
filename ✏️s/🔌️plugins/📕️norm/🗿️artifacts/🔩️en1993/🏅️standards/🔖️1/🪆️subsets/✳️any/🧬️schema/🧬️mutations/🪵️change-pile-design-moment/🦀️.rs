//! 🪵️ `change-pile-design-moment` mutation leaf: sets `design_moment` of one pile.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangePileDesignMoment {
    pub id: String,
    pub new_design_moment: f64,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangePileDesignMoment where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_pile_design_moment::ChangePileDesignMoment> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "pile-design-moment",
        kind: "change-pile-design-moment",
        record: "ChangedPileDesignMoment",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Design moment M_Ed of the pile", "Pfahl: Bemessungsmoment M_Ed ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangePileDesignMoment where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_pile_design_moment::ChangePileDesignMoment> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_pile_design_moment::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_pile_design_moment::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
