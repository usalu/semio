//! 🔁️ `change-fatigue-stress-history` mutation leaf: sets `stress_history` of one fatigue detail.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFatigueStressHistory {
    pub id: String,
    pub new_stress_history: String,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangeFatigueStressHistory where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_fatigue_stress_history::ChangeFatigueStressHistory> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "fatigue-detail-stress-history",
        kind: "change-fatigue-stress-history",
        record: "ChangedFatigueStressHistory",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Stress history Δσ in N/mm² as numbers separated by spaces; rainflow counted into the blocks of the damage sum when the spectrum is empty of the fatigue detail", "Ermüdungsdetail: Spannungsgeschichte σ in N/mm² als durch Leerzeichen getrennte Zahlen; wird bei leerem Kollektiv mit Rainflow in die Blöcke der Schadenssumme gezählt ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangeFatigueStressHistory where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_fatigue_stress_history::ChangeFatigueStressHistory> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_fatigue_stress_history::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_fatigue_stress_history::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
