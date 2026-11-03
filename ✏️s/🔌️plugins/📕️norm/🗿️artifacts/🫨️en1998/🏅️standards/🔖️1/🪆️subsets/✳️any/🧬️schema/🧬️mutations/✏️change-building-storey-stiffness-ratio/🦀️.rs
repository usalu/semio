//! ✏️ `change-building-storey-stiffness-ratio` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeBuildingStoreyStiffnessRatio {
    pub id: String,
    pub new_storey_stiffness_ratio: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeBuildingStoreyStiffnessRatio where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_building_storey_stiffness_ratio::ChangeBuildingStoreyStiffnessRatio> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "building-storey-stiffness-ratio",
        kind: "change-building-storey-stiffness-ratio",
        record: "ChangeBuildingStoreyStiffnessRatio",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Effective share of the entered gross storey stiffnesses (EN 1998-1 4.3.1(6)): 0 = simplified method with the cracked-stiffness factor of the registry, 0 < ratio <= 1 = effective stiffness declared from a more accurate analysis (1 = entered values are already effective)", "Wirksamer Anteil der eingegebenen Brutto-Geschosssteifigkeiten (EN 1998-1 4.3.1(6)): 0 = vereinfachtes Verfahren mit dem Faktor der gerissenen Steifigkeit aus dem Register, 0 < Anteil <= 1 = aus genauerer Berechnung angegebene wirksame Steifigkeit (1 = eingegebene Werte sind bereits wirksam) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-building-storey-stiffness-ratio".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeBuildingStoreyStiffnessRatio where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_building_storey_stiffness_ratio::ChangeBuildingStoreyStiffnessRatio> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_building_storey_stiffness_ratio::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_building_storey_stiffness_ratio::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
