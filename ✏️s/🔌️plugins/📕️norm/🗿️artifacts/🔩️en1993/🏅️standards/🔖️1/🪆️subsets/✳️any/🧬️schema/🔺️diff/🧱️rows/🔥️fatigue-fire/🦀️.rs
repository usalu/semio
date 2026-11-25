//! 🧱️ EN 1993 diff rows (fatigue-fire): the sparse field patch, key extractor and keyed row delta of 4 document row lists.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FatigueDetail`.
    pub En1993FatigueDetailPatch of semio_s_artifact_norm_en1993_model::FatigueDetail { set { member_id: String, category: u8, method: semio_s_artifact_norm_en1993_model::FatigueMethod, spectrum: Vec<semio_s_artifact_norm_en1993_model::FatigueBand> , stress_history: String, consequence: semio_s_artifact_norm_en1993_model::FatigueConsequence} }
}

/// 🔑️ Key extractor of the foreign `FatigueDetail` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FatigueDetailDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FatigueDetail> for En1993FatigueDetailDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FatigueDetail) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FatigueDetail` list.
    pub En1993FatigueDetailDelta { removal: En1993FatigueDetailRemoval, insertion: En1993FatigueDetailInsertion, relocation: En1993FatigueDetailRelocation, modification: En1993FatigueDetailModification, row: semio_s_artifact_norm_en1993_model::FatigueDetail, patch: En1993FatigueDetailPatch, list: Vec<semio_s_artifact_norm_en1993_model::FatigueDetail>, key: String = by En1993FatigueDetailDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FireExposure`.
    pub En1993FireExposurePatch of semio_s_artifact_norm_en1993_model::FireExposure { set { member_id: String, rating: semio_s_artifact_norm_en1993_model::FireRating, protection_thickness: f64, section_factor: f64, mu0: f64, design_temperature: f64, protection_conductivity: f64, protection_density: f64, protection_specific_heat: f64 } }
}

/// 🔑️ Key extractor of the foreign `FireExposure` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FireExposureDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FireExposure> for En1993FireExposureDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FireExposure) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FireExposure` list.
    pub En1993FireExposureDelta { removal: En1993FireExposureRemoval, insertion: En1993FireExposureInsertion, relocation: En1993FireExposureRelocation, modification: En1993FireExposureModification, row: semio_s_artifact_norm_en1993_model::FireExposure, patch: En1993FireExposurePatch, list: Vec<semio_s_artifact_norm_en1993_model::FireExposure>, key: String = by En1993FireExposureDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `BridgeFatigue`.
    pub En1993BridgeFatiguePatch of semio_s_artifact_norm_en1993_model::BridgeFatigue { set { member_id: String, lambda: f64, phi2: f64, delta_sigma_p: f64, category: u8, method: semio_s_artifact_norm_en1993_model::FatigueMethod , heavy_vehicle_weight: f64, reference_weight: f64, observed_traffic: f64, design_life: f64, second_lane_traffic: f64, second_lane_weight: f64, lambda_1: f64, lambda_max: f64, detail_id: String, plate_thickness: f64, deck_thickness: f64, trough_spacing: f64, trough_width: f64, trough_thickness: f64, crossbeam_spacing: f64, surfacing_thickness: f64, wheel_load: f64, wheel_patch: f64, consequence: semio_s_artifact_norm_en1993_model::FatigueConsequence} }
}

/// 🔑️ Key extractor of the foreign `BridgeFatigue` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993BridgeFatigueDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::BridgeFatigue> for En1993BridgeFatigueDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::BridgeFatigue) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `BridgeFatigue` list.
    pub En1993BridgeFatigueDelta { removal: En1993BridgeFatigueRemoval, insertion: En1993BridgeFatigueInsertion, relocation: En1993BridgeFatigueRelocation, modification: En1993BridgeFatigueModification, row: semio_s_artifact_norm_en1993_model::BridgeFatigue, patch: En1993BridgeFatiguePatch, list: Vec<semio_s_artifact_norm_en1993_model::BridgeFatigue>, key: String = by En1993BridgeFatigueDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FatigueStress`.
    pub En1993FatigueStressPatch of semio_s_artifact_norm_en1993_model::FatigueStress { set { detail_id: String, max_case_id: String, min_case_id: String, fibre_z: f64, fibre_y: f64, relief: semio_s_artifact_norm_en1993_model::StressRelief, concentration: f64, thickness: f64, cycles: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FatigueStressDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FatigueStressDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FatigueStress> for En1993FatigueStressDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FatigueStress) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FatigueStress` list.
    pub En1993FatigueStressDelta { removal: En1993FatigueStressRemoval, insertion: En1993FatigueStressInsertion, relocation: En1993FatigueStressRelocation, modification: En1993FatigueStressModification, row: semio_s_artifact_norm_en1993_model::FatigueStress, patch: En1993FatigueStressPatch, list: Vec<semio_s_artifact_norm_en1993_model::FatigueStress>, key: String = by En1993FatigueStressDeltaKey }
}
