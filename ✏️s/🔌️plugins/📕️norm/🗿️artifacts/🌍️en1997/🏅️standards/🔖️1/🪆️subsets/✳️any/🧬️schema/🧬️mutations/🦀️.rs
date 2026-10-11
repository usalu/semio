//! 🧬️ En1997 closed semantic mutation vocabulary for the geotechnical project subject.

use crate::diff::En1997Diff;
use crate::En1997Snapshot;

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::{resolve_edit, EDIT_RULES};

use super::change_annex;
use super::change_geotechnical_category;
use super::change_design_situation;
use super::change_design_approach;
use super::change_groundwater_level;
use super::change_investigation_depth;
use super::change_footing_width;
use super::change_footing_embedment;
use super::change_pile_length;
use super::change_pile_count;
use super::change_wall_base_width;
use super::change_slope_angle;
use super::change_layer_phi_prime;
use super::change_layer_oedometric_modulus;
use super::insert_layer;
use super::remove_layer;
use super::insert_footing;
use super::remove_footing;
use super::insert_pile;
use super::remove_pile;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1997Snapshot, diff = En1997Diff, schema = "norm.en1997")]
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

//#region 🧫️Vectors
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧫️Vectors

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
