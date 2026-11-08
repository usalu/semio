//! 🦴️ `change-fire-insulation-specific-heat` mutation leaf: sets `specific_heat` of one connection.

use semio_s_artifact_norm_en1999_diff::En1999Diff;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeFireInsulationSpecificHeat {
    pub fire_insulation_id: String,
    pub new_specific_heat: f64,
}

impl<Op> protocol::MutationKind<En1999Snapshot, Op> for ChangeFireInsulationSpecificHeat where Op: protocol::Mutation<En1999Snapshot, Diff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::change_fire_insulation_specific_heat::ChangeFireInsulationSpecificHeat> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "fire-insulation-specific-heat",
        kind: "change-fire-insulation-specific-heat",
        record: "ChangedFireInsulationSpecificHeat",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change fire insulation: Specific heat c_p", "Brandschutzbekleidung: Spezifische Wärmekapazität c_p ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1999Snapshot, Op> for ChangeFireInsulationSpecificHeat where Op: protocol::ColdMutation<En1999Snapshot, ReferenceDiff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::change_fire_insulation_specific_heat::ChangeFireInsulationSpecificHeat> {
    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::change_fire_insulation_specific_heat::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_fire_insulation_specific_heat::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
