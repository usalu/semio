//! 🌡 `change-thermal-section-surfacing` mutation leaf: sets `surfacing` of one thermal section.

use semio_s_artifact_norm_en1991_model::En1991Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeThermalSectionSurfacing {
    pub id: String,
    pub new_surfacing: u8,
}

impl<Op> protocol::MutationKind<En1991Snapshot, Op> for ChangeThermalSectionSurfacing where Op: protocol::Mutation<En1991Snapshot, Diff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::change_thermal_section_surfacing::ChangeThermalSectionSurfacing> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "thermal-section-surfacing",
        kind: "change-thermal-section-surfacing",
        record: "ChangedThermalSectionSurfacing",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change surfacing (0 none, 1 waterproofed, 2 50 mm, 3 100 mm, 4 150 mm) of the thermal section", "Temperaturquerschnitt: Belag (0 keiner, 1 Abdichtung, 2 50 mm, 3 100 mm, 4 150 mm) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1991Snapshot, Op> for ChangeThermalSectionSurfacing where Op: protocol::ColdMutation<En1991Snapshot, ReferenceDiff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::change_thermal_section_surfacing::ChangeThermalSectionSurfacing> {
    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1991_diff::En1991Diff> {
        crate::change_thermal_section_surfacing::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_thermal_section_surfacing::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
