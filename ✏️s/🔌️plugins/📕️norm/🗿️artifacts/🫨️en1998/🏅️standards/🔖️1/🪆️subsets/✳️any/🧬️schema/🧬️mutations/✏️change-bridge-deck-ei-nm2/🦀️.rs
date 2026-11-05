//! ✏️ `change-bridge-deck-ei-nm2` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBridgeDeckEiNm2 {
    pub id: String,
    pub new_deck_ei_nm2: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeBridgeDeckEiNm2 where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_bridge_deck_ei_nm2::ChangeBridgeDeckEiNm2> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "bridge-deck-ei-nm2",
        kind: "change-bridge-deck-ei-nm2",
        record: "ChangeBridgeDeckEiNm2",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Transverse bending stiffness EI of the deck in N·m² (0 = rigid deck)", "Biegesteifigkeit EI des Überbaus quer in N·m² (0 = starrer Überbau) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-bridge-deck-ei-nm2".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeBridgeDeckEiNm2 where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_bridge_deck_ei_nm2::ChangeBridgeDeckEiNm2> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_bridge_deck_ei_nm2::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_bridge_deck_ei_nm2::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
