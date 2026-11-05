//! 🧱️ `change-plated-lat-pressure-coef` mutation leaf: sets `lateral_pressure_coefficient` of one plated panel.

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangePlatedLatPressureCoef {
    pub id: String,
    pub new_lateral_pressure_coefficient: f64,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for ChangePlatedLatPressureCoef where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_plated_lat_pressure_coef::ChangePlatedLatPressureCoef> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "plated-lateral-pressure-coefficient",
        kind: "change-plated-lat-pressure-coef",
        record: "ChangedPlatedLatPressureCoef",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Plate bending coefficient k of sigma = k p b²/t² for the panel aspect ratio and edge conditions (Timoshenko or Roark table, entered; table values not verified) of the plated panel", "Plattenfeld: Plattenbeiwert k von sigma = k p b²/t² für Seitenverhältnis und Lagerung (Tabelle nach Timoshenko oder Roark, eingegeben; Tabellenwerte ungeprüft) ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for ChangePlatedLatPressureCoef where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::change_plated_lat_pressure_coef::ChangePlatedLatPressureCoef> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
        crate::change_plated_lat_pressure_coef::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_plated_lat_pressure_coef::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
