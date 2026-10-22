//! 🧬️ En1997 closed semantic mutation vocabulary for the geotechnical project subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES};

use semio_s_artifact_norm_en1997_mut_design::change_annex;
use semio_s_artifact_norm_en1997_mut_design::change_geotechnical_category;
use semio_s_artifact_norm_en1997_mut_design::change_design_situation;
use semio_s_artifact_norm_en1997_mut_design::change_design_approach;
use semio_s_artifact_norm_en1997_mut_design::change_groundwater_level;
use semio_s_artifact_norm_en1997_mut_design::change_investigation_depth;
use semio_s_artifact_norm_en1997_mut_foundations::change_footing_width;
use semio_s_artifact_norm_en1997_mut_foundations::change_footing_embedment;
use semio_s_artifact_norm_en1997_mut_foundations::change_pile_length;
use semio_s_artifact_norm_en1997_mut_foundations::change_pile_count;
use semio_s_artifact_norm_en1997_mut_foundations::change_wall_base_width;
use semio_s_artifact_norm_en1997_mut_ground::change_slope_angle;
use semio_s_artifact_norm_en1997_mut_ground::change_layer_phi_prime;
use semio_s_artifact_norm_en1997_mut_ground::change_layer_oedometric_modulus;
use semio_s_artifact_norm_en1997_mut_ground::insert_layer;
use semio_s_artifact_norm_en1997_mut_ground::remove_layer;
use semio_s_artifact_norm_en1997_mut_foundations::insert_footing;
use semio_s_artifact_norm_en1997_mut_foundations::remove_footing;
use semio_s_artifact_norm_en1997_mut_foundations::insert_pile;
use semio_s_artifact_norm_en1997_mut_foundations::remove_pile;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1997Snapshot, diff = En1997Diff, schema = "norm.en1997", reference = En1997Snapshot, reference_diff = En1997Diff)]
pub enum En1997Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeGeotechnicalCategory(change_geotechnical_category::ChangeGeotechnicalCategory),
    ChangeDesignSituation(change_design_situation::ChangeDesignSituation),
    ChangeDesignApproach(change_design_approach::ChangeDesignApproach),
    ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel),
    ChangeInvestigationDepth(change_investigation_depth::ChangeInvestigationDepth),
    ChangeFootingWidth(change_footing_width::ChangeFootingWidth),
    ChangeFootingEmbedment(change_footing_embedment::ChangeFootingEmbedment),
    ChangePileLength(change_pile_length::ChangePileLength),
    ChangePileCount(change_pile_count::ChangePileCount),
    ChangeWallBaseWidth(change_wall_base_width::ChangeWallBaseWidth),
    ChangeSlopeAngle(change_slope_angle::ChangeSlopeAngle),
    ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime),
    ChangeLayerOedometricModulus(change_layer_oedometric_modulus::ChangeLayerOedometricModulus),
    InsertLayer(insert_layer::InsertLayer),
    RemoveLayer(remove_layer::RemoveLayer),
    InsertFooting(insert_footing::InsertFooting),
    RemoveFooting(remove_footing::RemoveFooting),
    InsertPile(insert_pile::InsertPile),
    RemovePile(remove_pile::RemovePile),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-geotechnical-category",
    "change-design-situation",
    "change-design-approach",
    "change-groundwater-level",
    "change-investigation-depth",
    "change-footing-width",
    "change-footing-embedment",
    "change-pile-length",
    "change-pile-count",
    "change-wall-base-width",
    "change-slope-angle",
    "change-layer-phi-prime",
    "change-layer-oedometric-modulus",
    "insert-layer",
    "remove-layer",
    "insert-footing",
    "remove-footing",
    "insert-pile",
    "remove-pile",
];
//#endregion 🔖️Mutations

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1997Mutation;
    use semio_s_artifact_norm_en1997_model::En1997Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_en1997_mutation(base: &En1997Snapshot, mutation: &En1997Mutation) -> Result<(En1997Snapshot, Vec<String>), String> {
        let raised = <En1997Mutation as protocol::ColdMutation<En1997Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_en1997_mutation(mutation: &En1997Mutation, base: &En1997Snapshot) -> Result<Vec<En1997Mutation>, semio_framework_value::ValueError> {
        <En1997Mutation as protocol::ColdMutation<En1997Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;
