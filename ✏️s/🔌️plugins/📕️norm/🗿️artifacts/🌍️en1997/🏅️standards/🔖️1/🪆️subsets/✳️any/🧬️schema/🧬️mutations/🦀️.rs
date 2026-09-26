//! 🧬️ En1997 closed semantic mutation vocabulary for the geotechnical project subject.

use crate::diff::En1997Diff;
use crate::En1997Snapshot;

//#region 🔖️Leaves
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
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
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

//#region 🔖️FromSnapshot

impl En1997Mutation {
    pub fn from_snapshot(base: &En1997Snapshot, target: &En1997Snapshot) -> Vec<En1997Mutation> {
        let mut mutations = Vec::new();
        if base.annex != target.annex {
            mutations.push(En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        }
        if base.geotechnical_category != target.geotechnical_category {
            mutations.push(En1997Mutation::ChangeGeotechnicalCategory(change_geotechnical_category::ChangeGeotechnicalCategory {
                new_geotechnical_category: target.geotechnical_category,
            }));
        }
        if base.design_situation != target.design_situation {
            mutations.push(En1997Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation {
                new_design_situation: target.design_situation.clone(),
            }));
        }
        if base.design_approach != target.design_approach {
            mutations.push(En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach {
                new_design_approach: target.design_approach.clone(),
            }));
        }
        if base.groundwater_level.to_bits() != target.groundwater_level.to_bits() {
            mutations.push(En1997Mutation::ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel {
                new_groundwater_level: target.groundwater_level,
            }));
        }
        if base.investigation_depth.to_bits() != target.investigation_depth.to_bits() {
            mutations.push(En1997Mutation::ChangeInvestigationDepth(change_investigation_depth::ChangeInvestigationDepth {
                new_investigation_depth: target.investigation_depth,
            }));
        }

        let same_layer_ids = base.layers.len() == target.layers.len() && base.layers.iter().zip(&target.layers).all(|(a, b)| a.id == b.id);
        if same_layer_ids {
            for (base_layer, layer) in base.layers.iter().zip(&target.layers) {
                if base_layer.phi_prime_deg.to_bits() != layer.phi_prime_deg.to_bits() {
                    mutations.push(En1997Mutation::ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime {
                        id: layer.id.clone(),
                        new_phi_prime_deg: layer.phi_prime_deg,
                    }));
                }
                if base_layer.oedometric_modulus.to_bits() != layer.oedometric_modulus.to_bits() {
                    mutations.push(En1997Mutation::ChangeLayerOedometricModulus(change_layer_oedometric_modulus::ChangeLayerOedometricModulus {
                        id: layer.id.clone(),
                        new_oedometric_modulus: layer.oedometric_modulus,
                    }));
                }
            }
        } else if base.layers != target.layers {
            for index in (0..base.layers.len()).rev() {
                mutations.push(En1997Mutation::RemoveLayer(remove_layer::RemoveLayer { index }));
            }
            for (index, layer) in target.layers.iter().enumerate() {
                mutations.push(En1997Mutation::InsertLayer(insert_layer::InsertLayer { index, layer: layer.clone() }));
            }
        }

        let same_footing_ids = base.footings.len() == target.footings.len() && base.footings.iter().zip(&target.footings).all(|(a, b)| a.id == b.id);
        if same_footing_ids {
            for (base_footing, footing) in base.footings.iter().zip(&target.footings) {
                if base_footing.width.to_bits() != footing.width.to_bits() {
                    mutations.push(En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth {
                        id: footing.id.clone(),
                        new_width: footing.width,
                    }));
                }
                if base_footing.embedment.to_bits() != footing.embedment.to_bits() {
                    mutations.push(En1997Mutation::ChangeFootingEmbedment(change_footing_embedment::ChangeFootingEmbedment {
                        id: footing.id.clone(),
                        new_embedment: footing.embedment,
                    }));
                }
            }
        } else if base.footings != target.footings {
            for index in (0..base.footings.len()).rev() {
                mutations.push(En1997Mutation::RemoveFooting(remove_footing::RemoveFooting { index }));
            }
            for (index, footing) in target.footings.iter().enumerate() {
                mutations.push(En1997Mutation::InsertFooting(insert_footing::InsertFooting { index, footing: footing.clone() }));
            }
        }

        let same_pile_ids = base.piles.len() == target.piles.len() && base.piles.iter().zip(&target.piles).all(|(a, b)| a.id == b.id);
        if same_pile_ids {
            for (base_pile, pile) in base.piles.iter().zip(&target.piles) {
                if base_pile.length.to_bits() != pile.length.to_bits() {
                    mutations.push(En1997Mutation::ChangePileLength(change_pile_length::ChangePileLength {
                        id: pile.id.clone(),
                        new_length: pile.length,
                    }));
                }
                if base_pile.count != pile.count {
                    mutations.push(En1997Mutation::ChangePileCount(change_pile_count::ChangePileCount {
                        id: pile.id.clone(),
                        new_count: pile.count,
                    }));
                }
            }
        } else if base.piles != target.piles {
            for index in (0..base.piles.len()).rev() {
                mutations.push(En1997Mutation::RemovePile(remove_pile::RemovePile { index }));
            }
            for (index, pile) in target.piles.iter().enumerate() {
                mutations.push(En1997Mutation::InsertPile(insert_pile::InsertPile { index, pile: pile.clone() }));
            }
        }

        for (base_wall, wall) in base.retaining_walls.iter().zip(&target.retaining_walls) {
            if base_wall.id == wall.id && base_wall.base_width.to_bits() != wall.base_width.to_bits() {
                mutations.push(En1997Mutation::ChangeWallBaseWidth(change_wall_base_width::ChangeWallBaseWidth {
                    id: wall.id.clone(),
                    new_base_width: wall.base_width,
                }));
            }
        }

        for (base_slope, slope) in base.slopes.iter().zip(&target.slopes) {
            if base_slope.id == slope.id && base_slope.angle_deg.to_bits() != slope.angle_deg.to_bits() {
                mutations.push(En1997Mutation::ChangeSlopeAngle(change_slope_angle::ChangeSlopeAngle {
                    id: slope.id.clone(),
                    new_angle_deg: slope.angle_deg,
                }));
            }
        }

        mutations
    }
}

//#endregion 🔖️FromSnapshot
