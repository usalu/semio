//! 💡️ En1991 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1991Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::outline::En1991Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1991 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1991.inference")]
pub struct En1991Inference {
    #[derived]
    pub outline: En1991Outline,
}

impl protocol::Inference<En1991Snapshot> for En1991Inference {
    fn infer(snapshot: &En1991Snapshot) -> Self {
        Self { outline: En1991Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1991Snapshot> for En1991Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1991.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1991.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1991Builder {
    type Snapshot = En1991Snapshot;
    type Inference = En1991Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1991.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1991_artifact_schema_descriptor`'s registration.
pub fn en1991_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1991.inference",
        inference: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use semio_s_artifact_norm_en1990::standards::v1::subsets::any::schema::{NaDe, NaEn, NationalAnnexes};
use crate::standards::v1::subsets::any::schema::{part_1_1, part_1_2, part_1_3, part_1_4, part_1_5, part_1_6, part_1_7, part_2, part_3, part_4};
/// 📋️ Full EN 1991 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1991Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, ImposedCategory, NationalAnnex, Quantity};

/// 📋️ Aggregate action checks for a typical floor bay.
pub fn check_floor_actions(area_m2: f64, category: ImposedCategory, wind_zone_vb: f64, snow_zone: u8, use_de_na: bool) -> CheckReport {
    // 🔀️ O1 de-dyn: runtime-chosen concrete type (was `&dyn NationalAnnex`) — the closed-set enum
    // `NationalAnnexes` (`dyn_enum_close!` in en1990's schema module) replaces the trait object.
    let annex: NationalAnnexes = if use_de_na { NaDe.into() } else { NaEn.into() };
    let mut report = CheckReport::default();
    report.push(part_1_1::check_imposed(area_m2, category, &annex));
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, wind_zone_vb, c_e);
    report.push(part_1_4::check_wind(part_1_4::wind_pressure(q_p, 0.8, 0.2), 1.5, &annex));
    let s = part_1_3::roof_snow_load(part_1_3::ground_snow_load_zone(snow_zone), 0.8);
    report.push(part_1_3::check_snow(s, 1.2, &annex));
    report
}

/// 📋️ Full EN 1991 action checks across parts 1-1 through 1-7 and parts 2–4.
pub fn check_full_actions(document: &En1991Snapshot) -> CheckReport {
    let annex: NationalAnnexes = if document.annex == AnnexChoice::De { NaDe.into() } else { NaEn.into() };
    let mut report = CheckReport::default();
    report.push(part_1_1::check_imposed(document.area_m2, document.category, &annex));
    report.push(part_1_1::check_self_weight(&document.self_weight_material, document.self_weight_thickness_m, document.assumed_g_k_kn_m2, document.annex));
    report.push(part_1_2::check_fire_action(document.fire_curve, document.fire_resistance_min, document.fire_member_capacity_c, document.annex));
    let s_k = part_1_3::design_ground_snow_load(document.annex, document.snow_zone, document.snow_altitude_m, document.en_s_k_kn_m2);
    let s = part_1_3::roof_snow_load(s_k, 0.8);
    report.push(part_1_3::check_snow(s, 1.2, &annex));
    let v_b = part_1_4::design_basic_wind_velocity(document.annex, document.wind_zone, document.en_v_b_m_s);
    let c_e = part_1_4::exposure_factor(10.0, part_1_4::TerrainCategory::II);
    let q_p = part_1_4::peak_velocity_pressure(1.25, v_b, c_e);
    let c_sc_d = part_1_4::structural_factor(document.c_s, document.c_d);
    let w_p = part_1_4::wind_pressure(q_p, 0.8, 0.2) * c_sc_d;
    report.push(part_1_4::check_wind(w_p, 1.5, &annex));
    report.push(part_1_5::check_temperature_action(document.delta_t_k, 50.0));
    let q_const = part_1_6::construction_load_kn_m2(&document.construction_activity);
    report.push(part_1_6::check_construction_load(q_const, 5.0));
    let impact = part_1_7::impact_force_kn(document.accidental_mass_t, document.accidental_speed_km_h);
    report.push(CheckResult::from_utilization(ClauseId::new("EN 1991-1-7", "Annex B", "B.2"), Quantity::force_kn(impact), Quantity::force_kn(500.0), "accidental impact", annex.choice()));
    report.push(part_2::check_lm1_moment(document.annex, document.bridge_span_m, document.bridge_lane, document.bridge_lane_width_m, document.bridge_moment_resistance_knm));
    let wheel = part_3::design_vertical_wheel_load(&document.crane_class, &document.hoist_class, document.hoisting_speed_m_s);
    report.push(part_3::check_crane_load(wheel, wheel * 1.2));
    let silo_p = part_4::janssen_horizontal_pressure_kpa(document.silo_bulk_density_kn_m3, document.silo_hydraulic_radius_m, document.silo_mu, document.silo_k, document.silo_height_m);
    report.push(part_4::check_silo_pressure(silo_p, 100.0));
    report
}

/// 📋️ `En1991Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub fn evaluate(document: &En1991Snapshot) -> CheckReport {
    check_full_actions(document)
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests
