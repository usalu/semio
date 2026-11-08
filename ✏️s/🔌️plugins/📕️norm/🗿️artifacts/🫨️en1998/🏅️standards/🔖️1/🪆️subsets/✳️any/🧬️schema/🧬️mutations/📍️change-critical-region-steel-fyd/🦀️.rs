//! 📍️ `change-critical-region-steel-fyd` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCriticalRegionSteelDesignYieldMpa {
    pub id: String,
    pub new_steel_design_yield_mpa: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeCriticalRegionSteelDesignYieldMpa where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_critical_region_steel_fyd::ChangeCriticalRegionSteelDesignYieldMpa> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "critical-region-steel-design-yield-mpa",
        kind: "change-critical-region-steel-fyd",
        record: "ChangeCriticalRegionSteelDesignYieldMpa",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change critical region: design yield strength of the longitudinal steel", "Kritischer Bereich: Bemessungs-Streckgrenze der Längsbewehrung ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeCriticalRegionSteelDesignYieldMpa where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_critical_region_steel_fyd::ChangeCriticalRegionSteelDesignYieldMpa> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_critical_region_steel_fyd::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_critical_region_steel_fyd::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
