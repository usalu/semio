//! ✏️ `change-retaining-wall-permeability-ms` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeRetainingWallPermeabilityMS {
    pub index: usize,
    pub new_permeability_m_s: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeRetainingWallPermeabilityMS where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_retaining_wall_permeability_m_s::ChangeRetainingWallPermeabilityMS> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "retaining-wall-permeability-m-s",
        kind: "change-retaining-wall-permeability-ms",
        record: "ChangeRetainingWallPermeabilityMS",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change Permeability k of the backfill (0 = low permeability: the water moves with the soil)", "Durchlässigkeit k der Hinterfüllung (0 = geringe Durchlässigkeit: das Wasser bewegt sich mit dem Boden) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-retaining-wall-permeability-ms".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeRetainingWallPermeabilityMS where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_retaining_wall_permeability_m_s::ChangeRetainingWallPermeabilityMS> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_retaining_wall_permeability_m_s::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_retaining_wall_permeability_m_s::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
