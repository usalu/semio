//! 🧱️ EN 1993 model rows (fatigue-fire): 8 row and vocabulary types of the snapshot with their display and key helpers.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1993_model_rows_materials::*;

/// 📊 Fatigue stress-range band (Δσ_i, n_i) for Palmgren-Miner.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueBand {
    pub id: String,
    pub delta_sigma: f64,
    pub cycles: f64,
}

/// ♻ Fatigue assessment method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FatigueMethod {
    DamageTolerant,
    SafeLife,
}

/// 🔄 Fatigue detail category assessment.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueDetail {
    pub id: String,
    pub member_id: String,
    pub category: u8,
    pub method: FatigueMethod,
    #[dsl(table)]
    pub spectrum: Vec<FatigueBand>,
    pub stress_history: String,
    /// 🔄 Consequence class of a fatigue failure for the partial factor γ_Mf of EN 1993-1-9 Table 3.1: with the assessment method (damage tolerant or safe life) it selects one of the four registry rows ndp.en1993.1-9.gamma-mf.<method>.<consequence>.
    pub consequence: FatigueConsequence,
}

/// ⏱️ Standard fire resistance class of a member (EN 1993-1-2 4.2.5): R30, R60, R90 or R120.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FireRating {
    R30,
    R60,
    R90,
    R120,
}

/// 🔥 Fire exposure on a member.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FireExposure {
    pub id: String,
    pub member_id: String,
    pub rating: FireRating,
    pub protection_thickness: f64,
    pub section_factor: f64,
    pub mu0: f64,
    pub design_temperature: f64,
    pub protection_conductivity: f64,
    pub protection_density: f64,
    pub protection_specific_heat: f64,
}

/// 🌉 Bridge fatigue parameters (EN 1993-2).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BridgeFatigue {
    pub id: String,
    pub member_id: String,
    pub lambda: f64,
    pub phi2: f64,
    pub delta_sigma_p: f64,
    pub category: u8,
    pub method: FatigueMethod,
    pub heavy_vehicle_weight: f64,
    pub reference_weight: f64,
    pub observed_traffic: f64,
    pub design_life: f64,
    pub second_lane_traffic: f64,
    pub second_lane_weight: f64,
    pub lambda_1: f64,
    pub lambda_max: f64,
    pub detail_id: String,
    pub plate_thickness: f64,
    pub deck_thickness: f64,
    pub trough_spacing: f64,
    pub trough_width: f64,
    pub trough_thickness: f64,
    pub crossbeam_spacing: f64,
    pub surfacing_thickness: f64,
    pub wheel_load: f64,
    pub wheel_patch: f64,
    /// 🌉 Consequence class of a fatigue failure of the bridge detail for the partial factor γ_Mf of EN 1993-1-9 Table 3.1: with the assessment method it selects one of the four registry rows ndp.en1993.1-9.gamma-mf.<method>.<consequence>.
    pub consequence: FatigueConsequence,
}

/// ⚠ Consequence class of a fatigue failure (EN 1993-1-9 Table 3.1): low consequence or high consequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FatigueConsequence {
    Low,
    High,
}

impl std::fmt::Display for FatigueConsequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FatigueConsequence {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::High => "High",
        }
    }
}

/// ⏳ ⏳ Stress range of one fatigue detail from the fatigue load model (EN 1993-1-9 5.1 to 5.4): the stresses at the detail fibre from the member actions of the highest and the lowest load case, the reduction of the compressive part for non-welded or stress-relieved details, the geometric stress concentration, the plate thickness for the size effect and the number of cycles of the range.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueStress {
    pub id: String,
    /// 🔗 Id of the fatigue detail (category, method) the range belongs to.
    pub detail_id: String,
    /// ⬆ Id of the load case whose member actions give the largest stress at the detail fibre (fatigue load model in its unfavourable position).
    pub max_case_id: String,
    /// ⬇ Id of the load case whose member actions give the smallest stress at the detail fibre.
    pub min_case_id: String,
    /// 📍 Distance z of the detail fibre from the y axis, positive in the direction of positive bending moment M_y compression: sigma = N/A + M_y z/I_y - M_z y/I_z.
    pub fibre_z: f64,
    /// 📍 Distance y of the detail fibre from the z axis.
    pub fibre_y: f64,
    /// 🧯 As-welded details take the full range; stress-relieved welded and non-welded details take the compressive part of the cycle reduced (EN 1993-1-9 5.4).
    pub relief: StressRelief,
    /// 📐 Geometric stress concentration factor k_f of the nominal stress at the detail (EN 1993-1-9 5.2); 0 = not given, the nominal stress is used unchanged.
    pub concentration: f64,
    /// 📏 Thickness t of the cracked plate for the size effect k_s = (25/t)^n of the detail strength (EN 1993-1-9 7.1); 0 = not given, no size effect is applied.
    pub thickness: f64,
    /// 🔁 Number of cycles of this stress range in the design life.
    pub cycles: f64,
}

impl FireRating {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::R30 => "r30",
            Self::R60 => "r60",
            Self::R90 => "r90",
            Self::R120 => "r120",
        }
    }
}

impl std::fmt::Display for FireRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FatigueMethod {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DamageTolerant => "damage_tolerant",
            Self::SafeLife => "safe_life",
        }
    }
}

impl std::fmt::Display for FatigueMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
