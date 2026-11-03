//! 🧬️ En1997 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;

/// 🔑️ Key extractor of the foreign `SoilLayer` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoilLayerKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::SoilLayer> for SoilLayerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::SoilLayer) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `SpreadFoundation` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpreadFoundationKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::SpreadFoundation> for SpreadFoundationKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::SpreadFoundation) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `Pile` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PileKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::Pile> for PileKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::Pile) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `RetainingWall` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RetainingWallKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::RetainingWall> for RetainingWallKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::RetainingWall) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `Slope` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SlopeKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::Slope> for SlopeKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::Slope) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `UpliftCase` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpliftCaseKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::UpliftCase> for UpliftCaseKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::UpliftCase) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `Anchor` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnchorKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::Anchor> for AnchorKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::Anchor) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `EmbeddedWall` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmbeddedWallKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::EmbeddedWall> for EmbeddedWallKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::EmbeddedWall) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `PileGroup` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PileGroupKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::PileGroup> for PileGroupKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::PileGroup) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `LabTest` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LabTestKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::LabTest> for LabTestKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::LabTest) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `LayerWater` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayerWaterKey;

/// 🔑️ Key extractor of the foreign `CharacteristicBasis` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharacteristicBasisKey;

/// 🔑️ Key extractor of the foreign `Investigation` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvestigationKey;

/// 🔑️ Key extractor of the foreign `Placement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::LayerWater> for LayerWaterKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::LayerWater) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::CharacteristicBasis> for CharacteristicBasisKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::CharacteristicBasis) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::Investigation> for InvestigationKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::Investigation) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::Placement> for PlacementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::Placement) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `layers` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997LayersPatch of semio_s_artifact_norm_en1997_model::SoilLayer { set { soil_type: semio_s_artifact_norm_en1997_model::SoilType, depth_top: f64, depth_bottom: f64, gamma: f64, gamma_prime: f64, phi_prime_deg: f64, cohesion_effective: f64, cohesion_undrained: f64, oedometric_modulus: f64, poisson_ratio: f64, cpt_qc: f64, spt_n: f64, cpt_nkt: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `layers` list (rows keyed by `id`).
    pub En1997LayersRows { removal: En1997LayersRemoved, insertion: En1997LayersInserted, relocation: En1997LayersMoved, modification: En1997LayersModified, row: semio_s_artifact_norm_en1997_model::SoilLayer, patch: En1997LayersPatch, list: Vec<semio_s_artifact_norm_en1997_model::SoilLayer>, key: String = by SoilLayerKey }
}

/// 🔑️ Key extractor of the foreign `FoundationLoadCase` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FoundationLoadCaseKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::FoundationLoadCase> for FoundationLoadCaseKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::FoundationLoadCase) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `load_cases` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997FootingsLoadCasesPatch of semio_s_artifact_norm_en1997_model::FoundationLoadCase { set { design_situation: semio_s_artifact_norm_en1997_model::SituationChoice, vertical_permanent: f64, vertical_variable: f64, horizontal_permanent: f64, horizontal_variable: f64, moment_permanent: f64, moment_variable: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `load_cases` list (rows keyed by `id`).
    pub En1997FootingsLoadCasesRows { removal: En1997FootingsLoadCasesRemoved, insertion: En1997FootingsLoadCasesInserted, relocation: En1997FootingsLoadCasesMoved, modification: En1997FootingsLoadCasesModified, row: semio_s_artifact_norm_en1997_model::FoundationLoadCase, patch: En1997FootingsLoadCasesPatch, list: Vec<semio_s_artifact_norm_en1997_model::FoundationLoadCase>, key: String = by FoundationLoadCaseKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `footings` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997FootingsPatch of semio_s_artifact_norm_en1997_model::SpreadFoundation { set { width: f64, length: f64, embedment: f64, base_inclination_deg: f64, settlement_limit: f64 } nest { load_cases: En1997FootingsLoadCasesRows } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `footings` list (rows keyed by `id`).
    pub En1997FootingsRows { removal: En1997FootingsRemoved, insertion: En1997FootingsInserted, relocation: En1997FootingsMoved, modification: En1997FootingsModified, row: semio_s_artifact_norm_en1997_model::SpreadFoundation, patch: En1997FootingsPatch, list: Vec<semio_s_artifact_norm_en1997_model::SpreadFoundation>, key: String = by SpreadFoundationKey }
}

/// 🔑️ Key extractor of the foreign `PileTestProfile` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PileTestProfileKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::PileTestProfile> for PileTestProfileKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::PileTestProfile) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `test_profiles` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997PilesTestProfilesPatch of semio_s_artifact_norm_en1997_model::PileTestProfile { set { shaft_resistance: f64, base_resistance: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `test_profiles` list (rows keyed by `id`).
    pub En1997PilesTestProfilesRows { removal: En1997PilesTestProfilesRemoved, insertion: En1997PilesTestProfilesInserted, relocation: En1997PilesTestProfilesMoved, modification: En1997PilesTestProfilesModified, row: semio_s_artifact_norm_en1997_model::PileTestProfile, patch: En1997PilesTestProfilesPatch, list: Vec<semio_s_artifact_norm_en1997_model::PileTestProfile>, key: String = by PileTestProfileKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `piles` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997PilesPatch of semio_s_artifact_norm_en1997_model::Pile { set { pile_type: semio_s_artifact_norm_en1997_model::PileType, diameter: f64, length: f64, count: u32, alpha_s: f64, unit_shaft_resistance: f64, unit_base_resistance: f64, compression_permanent: f64, compression_variable: f64, tension_permanent: f64, tension_variable: f64, bending_stiffness_ei: f64, horizontal_subgrade_modulus: f64, lateral_head_load: f64, elastic_modulus: f64, settling_thickness: f64, negative_friction_beta: f64, dynamic_test_count: f64, dynamic_resistance_mean: f64, dynamic_resistance_min: f64, hammer_weight: f64, drop_height: f64, set_per_blow: f64, elastic_compression: f64, hammer_efficiency: f64, concrete_class: semio_s_artifact_norm_en1997_model::PileConcreteClass } nest { test_profiles: En1997PilesTestProfilesRows } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `piles` list (rows keyed by `id`).
    pub En1997PilesRows { removal: En1997PilesRemoved, insertion: En1997PilesInserted, relocation: En1997PilesMoved, modification: En1997PilesModified, row: semio_s_artifact_norm_en1997_model::Pile, patch: En1997PilesPatch, list: Vec<semio_s_artifact_norm_en1997_model::Pile>, key: String = by PileKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `retaining_walls` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997RetainingWallsPatch of semio_s_artifact_norm_en1997_model::RetainingWall { set { height: f64, embedment: f64, base_width: f64, stem_thickness: f64, backfill_phi_deg: f64, backfill_gamma: f64, wall_friction_deg: f64, earth_pressure_mode: semio_s_artifact_norm_en1997_model::EarthPressureMode, wall_movement: semio_s_artifact_norm_en1997_model::WallMovement, ocr: f64, concrete_gamma: f64, surcharge: f64, vertical_permanent: f64, horizontal_permanent: f64, seismic_kh: f64, seismic_kv: f64, reinforcement_spacing: f64, reinforcement_width: f64, reinforcement_friction: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `retaining_walls` list (rows keyed by `id`).
    pub En1997RetainingWallsRows { removal: En1997RetainingWallsRemoved, insertion: En1997RetainingWallsInserted, relocation: En1997RetainingWallsMoved, modification: En1997RetainingWallsModified, row: semio_s_artifact_norm_en1997_model::RetainingWall, patch: En1997RetainingWallsPatch, list: Vec<semio_s_artifact_norm_en1997_model::RetainingWall>, key: String = by RetainingWallKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `slopes` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997SlopesPatch of semio_s_artifact_norm_en1997_model::Slope { set { angle_deg: f64, height: f64, length: f64, governing_layer_id: String, circle_centre_x: f64, circle_centre_z: f64, circle_radius: f64, width: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `slopes` list (rows keyed by `id`).
    pub En1997SlopesRows { removal: En1997SlopesRemoved, insertion: En1997SlopesInserted, relocation: En1997SlopesMoved, modification: En1997SlopesModified, row: semio_s_artifact_norm_en1997_model::Slope, patch: En1997SlopesPatch, list: Vec<semio_s_artifact_norm_en1997_model::Slope>, key: String = by SlopeKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `uplift_cases` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997UpliftCasesPatch of semio_s_artifact_norm_en1997_model::UpliftCase { set { permanent_stabilizing: f64, permanent_destabilizing: f64, variable_destabilizing: f64, pore_pressure: f64, total_stress: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `uplift_cases` list (rows keyed by `id`).
    pub En1997UpliftCasesRows { removal: En1997UpliftCasesRemoved, insertion: En1997UpliftCasesInserted, relocation: En1997UpliftCasesMoved, modification: En1997UpliftCasesModified, row: semio_s_artifact_norm_en1997_model::UpliftCase, patch: En1997UpliftCasesPatch, list: Vec<semio_s_artifact_norm_en1997_model::UpliftCase>, key: String = by UpliftCaseKey }
}

/// 🔑️ Key extractor of the foreign `AnchorTest` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnchorTestKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::AnchorTest> for AnchorTestKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::AnchorTest) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `tests` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997AnchorsTestsPatch of semio_s_artifact_norm_en1997_model::AnchorTest { set { pull_out_resistance: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `tests` list (rows keyed by `id`).
    pub En1997AnchorsTestsRows { removal: En1997AnchorsTestsRemoved, insertion: En1997AnchorsTestsInserted, relocation: En1997AnchorsTestsMoved, modification: En1997AnchorsTestsModified, row: semio_s_artifact_norm_en1997_model::AnchorTest, patch: En1997AnchorsTestsPatch, list: Vec<semio_s_artifact_norm_en1997_model::AnchorTest>, key: String = by AnchorTestKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `anchors` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997AnchorsPatch of semio_s_artifact_norm_en1997_model::Anchor { set { wall_id: String, tendon: semio_s_artifact_norm_en1997_model::TendonType, head_depth: f64, inclination_deg: f64, free_length: f64, fixed_length: f64, fixed_diameter: f64, bond_stress: f64, design_force: f64, tendon_yield_force: f64, min_free_length: f64, test_factor: f64 } nest { tests: En1997AnchorsTestsRows } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `anchors` list (rows keyed by `id`).
    pub En1997AnchorsRows { removal: En1997AnchorsRemoved, insertion: En1997AnchorsInserted, relocation: En1997AnchorsMoved, modification: En1997AnchorsModified, row: semio_s_artifact_norm_en1997_model::Anchor, patch: En1997AnchorsPatch, list: Vec<semio_s_artifact_norm_en1997_model::Anchor>, key: String = by AnchorKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `embedded_walls` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997EmbeddedWallsPatch of semio_s_artifact_norm_en1997_model::EmbeddedWall { set { kind: semio_s_artifact_norm_en1997_model::EmbeddedWallKind, retained_height: f64, embedment: f64, prop_depth: f64, thickness: f64, backfill_gamma: f64, phi_deg: f64, delta_active_deg: f64, delta_passive_deg: f64, surcharge: f64, undrained_strength: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `embedded_walls` list (rows keyed by `id`).
    pub En1997EmbeddedWallsRows { removal: En1997EmbeddedWallsRemoved, insertion: En1997EmbeddedWallsInserted, relocation: En1997EmbeddedWallsMoved, modification: En1997EmbeddedWallsModified, row: semio_s_artifact_norm_en1997_model::EmbeddedWall, patch: En1997EmbeddedWallsPatch, list: Vec<semio_s_artifact_norm_en1997_model::EmbeddedWall>, key: String = by EmbeddedWallKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `pile_groups` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997PileGroupsPatch of semio_s_artifact_norm_en1997_model::PileGroup { set { pile_id: String, rows: u32, columns: u32, spacing: f64, cap_vertical: f64, cap_moment_x: f64, cap_moment_y: f64, single_resistance: f64, shaft_cohesion_sum: f64, base_cohesion: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `pile_groups` list (rows keyed by `id`).
    pub En1997PileGroupsRows { removal: En1997PileGroupsRemoved, insertion: En1997PileGroupsInserted, relocation: En1997PileGroupsMoved, modification: En1997PileGroupsModified, row: semio_s_artifact_norm_en1997_model::PileGroup, patch: En1997PileGroupsPatch, list: Vec<semio_s_artifact_norm_en1997_model::PileGroup>, key: String = by PileGroupKey }
}

/// 🔑️ Key extractor of the foreign `LabPoint` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LabPointKey;

/// 🔑️ Key extractor of the foreign `InvestigationReading` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvestigationReadingKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::LabPoint> for LabPointKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::LabPoint) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1997_model::InvestigationReading> for InvestigationReadingKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1997_model::InvestigationReading) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `points` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997LabTestsPointsPatch of semio_s_artifact_norm_en1997_model::LabPoint { set { normal_stress: f64, shear_stress: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `points` list (rows keyed by `id`).
    pub En1997LabTestsPointsRows { removal: En1997LabTestsPointsRemoved, insertion: En1997LabTestsPointsInserted, relocation: En1997LabTestsPointsMoved, modification: En1997LabTestsPointsModified, row: semio_s_artifact_norm_en1997_model::LabPoint, patch: En1997LabTestsPointsPatch, list: Vec<semio_s_artifact_norm_en1997_model::LabPoint>, key: String = by LabPointKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `investigations_readings` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997InvestigationsReadingsPatch of semio_s_artifact_norm_en1997_model::InvestigationReading { set { depth: f64, measurement: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `investigations_readings` list (rows keyed by `id`).
    pub En1997InvestigationsReadingsRows { removal: En1997InvestigationsReadingsRemoved, insertion: En1997InvestigationsReadingsInserted, relocation: En1997InvestigationsReadingsMoved, modification: En1997InvestigationsReadingsModified, row: semio_s_artifact_norm_en1997_model::InvestigationReading, patch: En1997InvestigationsReadingsPatch, list: Vec<semio_s_artifact_norm_en1997_model::InvestigationReading>, key: String = by InvestigationReadingKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `lab_tests` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997LabTestsPatch of semio_s_artifact_norm_en1997_model::LabTest { set { layer_id: String, kind: semio_s_artifact_norm_en1997_model::LabTestKind, void_ratio: f64, preload_stress: f64, drainage_height: f64, consolidation_time: f64, compression_index: f64, swelling_index: f64, stress_increment: f64, secondary_compression_index: f64, design_life: f64 } nest { points: En1997LabTestsPointsRows } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `lab_tests` list (rows keyed by `id`).
    pub En1997LabTestsRows { removal: En1997LabTestsRemoved, insertion: En1997LabTestsInserted, relocation: En1997LabTestsMoved, modification: En1997LabTestsModified, row: semio_s_artifact_norm_en1997_model::LabTest, patch: En1997LabTestsPatch, list: Vec<semio_s_artifact_norm_en1997_model::LabTest>, key: String = by LabTestKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `layer_waters` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997LayerWatersPatch of semio_s_artifact_norm_en1997_model::LayerWater { set { layer_id: String, piezometric_depth: f64, permeability: f64, head_difference: f64, flow_length: f64, well_radius: f64, well_water_depth: f64, influence_radius: f64, well_capacity: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `placements` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997PlacementsPatch of semio_s_artifact_norm_en1997_model::Placement { set { x: f64, y: f64, z: f64, yaw: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `layer_waters` list (rows keyed by `id`).
    pub En1997LayerWatersRows { removal: En1997LayerWatersRemoved, insertion: En1997LayerWatersInserted, relocation: En1997LayerWatersMoved, modification: En1997LayerWatersModified, row: semio_s_artifact_norm_en1997_model::LayerWater, patch: En1997LayerWatersPatch, list: Vec<semio_s_artifact_norm_en1997_model::LayerWater>, key: String = by LayerWaterKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `characteristic_bases` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997CharacteristicBasesPatch of semio_s_artifact_norm_en1997_model::CharacteristicBasis { set { layer_id: String, parameter: semio_s_artifact_norm_en1997_model::CharacteristicParameter, basis: semio_s_artifact_norm_en1997_model::ParameterBasis } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `characteristic_bases` list (rows keyed by `id`).
    pub En1997CharacteristicBasesRows { removal: En1997CharacteristicBasesRemoved, insertion: En1997CharacteristicBasesInserted, relocation: En1997CharacteristicBasesMoved, modification: En1997CharacteristicBasesModified, row: semio_s_artifact_norm_en1997_model::CharacteristicBasis, patch: En1997CharacteristicBasesPatch, list: Vec<semio_s_artifact_norm_en1997_model::CharacteristicBasis>, key: String = by CharacteristicBasisKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
    /// 🩹️ Sparse patch of the `investigations` row addressed by its key (typed leaves; nested rows through `nest` lanes).
    pub En1997InvestigationsPatch of semio_s_artifact_norm_en1997_model::Investigation { set { position_x: f64, position_y: f64, ground_level: f64, depth: f64, kind: semio_s_artifact_norm_en1997_model::InvestigationKind } nest { readings: En1997InvestigationsReadingsRows } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `investigations` list (rows keyed by `id`).
    pub En1997InvestigationsRows { removal: En1997InvestigationsRemoved, insertion: En1997InvestigationsInserted, relocation: En1997InvestigationsMoved, modification: En1997InvestigationsModified, row: semio_s_artifact_norm_en1997_model::Investigation, patch: En1997InvestigationsPatch, list: Vec<semio_s_artifact_norm_en1997_model::Investigation>, key: String = by InvestigationKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `placements` list (rows keyed by `id`).
    pub En1997PlacementsRows { removal: En1997PlacementsRemoved, insertion: En1997PlacementsInserted, relocation: En1997PlacementsMoved, modification: En1997PlacementsModified, row: semio_s_artifact_norm_en1997_model::Placement, patch: En1997PlacementsPatch, list: Vec<semio_s_artifact_norm_en1997_model::Placement>, key: String = by PlacementKey }
}

/// 🔺️ Keyed sparse diff of the En1997 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Diff {
    #[state(artifact)]
    pub structure_id: Option<String>,
    #[state(artifact)]
    pub geotechnical_category: Option<u8>,
    #[state(artifact)]
    pub design_situation: Option<semio_s_artifact_norm_en1997_model::SituationChoice>,
    #[state(artifact)]
    pub design_approach: Option<semio_s_artifact_norm_en1997_model::ApproachChoice>,
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub groundwater_level: Option<f64>,
    #[state(artifact)]
    pub investigation_depth: Option<f64>,
    #[state(artifact)]
    pub layers: Option<En1997LayersRows>,
    #[state(artifact)]
    pub footings: Option<En1997FootingsRows>,
    #[state(artifact)]
    pub piles: Option<En1997PilesRows>,
    #[state(artifact)]
    pub retaining_walls: Option<En1997RetainingWallsRows>,
    #[state(artifact)]
    pub slopes: Option<En1997SlopesRows>,
    #[state(artifact)]
    pub uplift_cases: Option<En1997UpliftCasesRows>,
    #[state(artifact)]
    pub anchors: Option<En1997AnchorsRows>,
    #[state(artifact)]
    pub embedded_walls: Option<En1997EmbeddedWallsRows>,
    #[state(artifact)]
    pub pile_groups: Option<En1997PileGroupsRows>,
    #[state(artifact)]
    pub lab_tests: Option<En1997LabTestsRows>,
    #[state(artifact)]
    pub layer_waters: Option<En1997LayerWatersRows>,
    #[state(artifact)]
    pub placements: Option<En1997PlacementsRows>,
    #[state(artifact)]
    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,
    #[state(artifact)]
    pub investigations: Option<En1997InvestigationsRows>,
    #[state(artifact)]
    pub characteristic_bases: Option<En1997CharacteristicBasesRows>,
}

impl protocol::MutationDiff<En1997Snapshot> for En1997Diff {}

impl protocol::ColdMutationDiff<En1997Snapshot> for En1997Diff {
    fn absorb(&mut self, other: Self) {
        if other.structure_id.is_some() {
            self.structure_id = other.structure_id;
        }
        if other.geotechnical_category.is_some() {
            self.geotechnical_category = other.geotechnical_category;
        }
        if other.design_situation.is_some() {
            self.design_situation = other.design_situation;
        }
        if other.design_approach.is_some() {
            self.design_approach = other.design_approach;
        }
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.groundwater_level.is_some() {
            self.groundwater_level = other.groundwater_level;
        }
        if other.investigation_depth.is_some() {
            self.investigation_depth = other.investigation_depth;
        }
        if let Some(theirs) = other.layers {
            match self.layers.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.layers = Some(theirs),
            }
            self.layers = self.layers.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.footings {
            match self.footings.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.footings = Some(theirs),
            }
            self.footings = self.footings.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.piles {
            match self.piles.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.piles = Some(theirs),
            }
            self.piles = self.piles.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.retaining_walls {
            match self.retaining_walls.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.retaining_walls = Some(theirs),
            }
            self.retaining_walls = self.retaining_walls.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.slopes {
            match self.slopes.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.slopes = Some(theirs),
            }
            self.slopes = self.slopes.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.uplift_cases {
            match self.uplift_cases.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.uplift_cases = Some(theirs),
            }
            self.uplift_cases = self.uplift_cases.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.anchors {
            match self.anchors.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.anchors = Some(theirs),
            }
            self.anchors = self.anchors.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.embedded_walls {
            match self.embedded_walls.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.embedded_walls = Some(theirs),
            }
            self.embedded_walls = self.embedded_walls.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.pile_groups {
            match self.pile_groups.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.pile_groups = Some(theirs),
            }
            self.pile_groups = self.pile_groups.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.lab_tests {
            match self.lab_tests.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.lab_tests = Some(theirs),
            }
            self.lab_tests = self.lab_tests.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.layer_waters {
            match self.layer_waters.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.layer_waters = Some(theirs),
            }
            self.layer_waters = self.layer_waters.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.characteristic_bases {
            match self.characteristic_bases.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.characteristic_bases = Some(theirs),
            }
            self.characteristic_bases = self.characteristic_bases.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.investigations {
            match self.investigations.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.investigations = Some(theirs),
            }
            self.investigations = self.investigations.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.placements {
            match self.placements.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.placements = Some(theirs),
            }
            self.placements = self.placements.take().filter(|rows| !rows.is_empty());
        }
        self.cases.absorb(other.cases);
    }

    fn apply(&self, unpublished: &mut En1997Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.structure_id {
            next.structure_id = value.clone();
        }
        if let Some(value) = &self.geotechnical_category {
            next.geotechnical_category = value.clone();
        }
        if let Some(value) = &self.design_situation {
            next.design_situation = value.clone();
        }
        if let Some(value) = &self.design_approach {
            next.design_approach = value.clone();
        }
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.groundwater_level {
            next.groundwater_level = value.clone();
        }
        if let Some(value) = &self.investigation_depth {
            next.investigation_depth = value.clone();
        }
        if let Some(rows) = &self.layers {
            next.layers = rows.commit_onto(&base.layers, capability).map_err(|error| error.under(["layers"]))?;
        }
        if let Some(rows) = &self.footings {
            next.footings = rows.commit_onto(&base.footings, capability).map_err(|error| error.under(["footings"]))?;
        }
        if let Some(rows) = &self.piles {
            next.piles = rows.commit_onto(&base.piles, capability).map_err(|error| error.under(["piles"]))?;
        }
        if let Some(rows) = &self.retaining_walls {
            next.retaining_walls = rows.commit_onto(&base.retaining_walls, capability).map_err(|error| error.under(["retaining_walls"]))?;
        }
        if let Some(rows) = &self.slopes {
            next.slopes = rows.commit_onto(&base.slopes, capability).map_err(|error| error.under(["slopes"]))?;
        }
        if let Some(rows) = &self.uplift_cases {
            next.uplift_cases = rows.commit_onto(&base.uplift_cases, capability).map_err(|error| error.under(["uplift_cases"]))?;
        }
        if let Some(rows) = &self.anchors {
            next.anchors = rows.commit_onto(&base.anchors, capability).map_err(|error| error.under(["anchors"]))?;
        }
        if let Some(rows) = &self.embedded_walls {
            next.embedded_walls = rows.commit_onto(&base.embedded_walls, capability).map_err(|error| error.under(["embedded_walls"]))?;
        }
        if let Some(rows) = &self.pile_groups {
            next.pile_groups = rows.commit_onto(&base.pile_groups, capability).map_err(|error| error.under(["pile_groups"]))?;
        }
        if let Some(rows) = &self.lab_tests {
            next.lab_tests = rows.commit_onto(&base.lab_tests, capability).map_err(|error| error.under(["lab_tests"]))?;
        }
        if let Some(rows) = &self.layer_waters {
            next.layer_waters = rows.commit_onto(&base.layer_waters, capability).map_err(|error| error.under(["layer_waters"]))?;
        }
        if let Some(rows) = &self.characteristic_bases {
            next.characteristic_bases = rows.commit_onto(&base.characteristic_bases, capability).map_err(|error| error.under(["characteristic_bases"]))?;
        }
        if let Some(rows) = &self.investigations {
            next.investigations = rows.commit_onto(&base.investigations, capability).map_err(|error| error.under(["investigations"]))?;
        }
        if let Some(rows) = &self.placements {
            next.placements = rows.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;
        }
        next.cases = self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?;
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<En1997Snapshot> for En1997Diff {
    fn is_empty(&self) -> bool {
        self.structure_id.is_none() && self.geotechnical_category.is_none() && self.design_situation.is_none() && self.design_approach.is_none() && self.annex.is_none() && self.groundwater_level.is_none() && self.investigation_depth.is_none() && self.layers.as_ref().map_or(true, |rows| rows.is_empty()) && self.footings.as_ref().map_or(true, |rows| rows.is_empty()) && self.piles.as_ref().map_or(true, |rows| rows.is_empty()) && self.retaining_walls.as_ref().map_or(true, |rows| rows.is_empty()) && self.slopes.as_ref().map_or(true, |rows| rows.is_empty()) && self.uplift_cases.as_ref().map_or(true, |rows| rows.is_empty()) && self.anchors.as_ref().map_or(true, |rows| rows.is_empty()) && self.embedded_walls.as_ref().map_or(true, |rows| rows.is_empty()) && self.pile_groups.as_ref().map_or(true, |rows| rows.is_empty()) && self.lab_tests.as_ref().map_or(true, |rows| rows.is_empty()) && self.layer_waters.as_ref().map_or(true, |rows| rows.is_empty()) && self.characteristic_bases.as_ref().map_or(true, |rows| rows.is_empty()) && self.investigations.as_ref().map_or(true, |rows| rows.is_empty()) && self.placements.as_ref().map_or(true, |rows| rows.is_empty()) && self.cases.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1997Snapshot> for En1997Diff {
    fn inverse(&self, base: &En1997Snapshot) -> Self {
        Self {
            structure_id: self.structure_id.as_ref().map(|_| base.structure_id.clone()),
            geotechnical_category: self.geotechnical_category.as_ref().map(|_| base.geotechnical_category.clone()),
            design_situation: self.design_situation.as_ref().map(|_| base.design_situation.clone()),
            design_approach: self.design_approach.as_ref().map(|_| base.design_approach.clone()),
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            groundwater_level: self.groundwater_level.as_ref().map(|_| base.groundwater_level.clone()),
            investigation_depth: self.investigation_depth.as_ref().map(|_| base.investigation_depth.clone()),
            layers: self.layers.as_ref().map(|rows| rows.inverse(&base.layers)).filter(|rows| !rows.is_empty()),
            footings: self.footings.as_ref().map(|rows| rows.inverse(&base.footings)).filter(|rows| !rows.is_empty()),
            piles: self.piles.as_ref().map(|rows| rows.inverse(&base.piles)).filter(|rows| !rows.is_empty()),
            retaining_walls: self.retaining_walls.as_ref().map(|rows| rows.inverse(&base.retaining_walls)).filter(|rows| !rows.is_empty()),
            slopes: self.slopes.as_ref().map(|rows| rows.inverse(&base.slopes)).filter(|rows| !rows.is_empty()),
            uplift_cases: self.uplift_cases.as_ref().map(|rows| rows.inverse(&base.uplift_cases)).filter(|rows| !rows.is_empty()),
            anchors: self.anchors.as_ref().map(|rows| rows.inverse(&base.anchors)).filter(|rows| !rows.is_empty()),
            embedded_walls: self.embedded_walls.as_ref().map(|rows| rows.inverse(&base.embedded_walls)).filter(|rows| !rows.is_empty()),
            pile_groups: self.pile_groups.as_ref().map(|rows| rows.inverse(&base.pile_groups)).filter(|rows| !rows.is_empty()),
            lab_tests: self.lab_tests.as_ref().map(|rows| rows.inverse(&base.lab_tests)).filter(|rows| !rows.is_empty()),
            layer_waters: self.layer_waters.as_ref().map(|rows| rows.inverse(&base.layer_waters)).filter(|rows| !rows.is_empty()),
            characteristic_bases: self.characteristic_bases.as_ref().map(|rows| rows.inverse(&base.characteristic_bases)).filter(|rows| !rows.is_empty()),
            investigations: self.investigations.as_ref().map(|rows| rows.inverse(&base.investigations)).filter(|rows| !rows.is_empty()),
            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),
            cases: self.cases.inverse(&base.cases),
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1997_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1997_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1997_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1997_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves
