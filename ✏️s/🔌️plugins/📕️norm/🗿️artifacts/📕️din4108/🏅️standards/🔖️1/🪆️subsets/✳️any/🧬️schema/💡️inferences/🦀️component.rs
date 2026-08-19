//! 💡️ Din4108 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::din4108::Din4108Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Din4108Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a din4108 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din4108.inference")]
pub struct Din4108Inference {
    #[derived]
    pub outline: Din4108Outline,
}

impl protocol::Inference<Din4108Snapshot> for Din4108Inference {
    async fn infer(snapshot: &Din4108Snapshot) -> Self {
        Self { outline: Din4108Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Din4108Snapshot> for Din4108Inference {
    async fn inference_schema_id() -> &'static str {
        "s.norm.din4108.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.din4108.inference.outline", reads: &["layers"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::din4108::standards::v1::subsets::any::schema::Din4108Builder {
    type Snapshot = Din4108Snapshot;
    type Inference = Din4108Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.din4108.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `din4108_artifact_schema_descriptor`'s registration.
pub async fn din4108_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.din4108.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = Din4108Snapshot::default();
        assert_eq!(Din4108Inference::infer(&snapshot), Din4108Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(Din4108Inference::infer(&Din4108Snapshot::default()), Din4108Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::artifacts::din4108::standards::v1::subsets::any::schema::{bb_2, part_1, part_10, part_2, part_3, part_4, part_5, part_6, part_7, part_8, R_SE_WALL_M2K_W, R_SI_WALL_M2K_W};
/// 📋️ Full DIN 4108 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `Din4108Snapshot -> CheckReport` projection; everything it composes
/// (`part_N`/`bb_2`) is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, ClimateZoneDe, NormError, Quantity};

/// 📋️ Run all applicable DIN 4108 checks for a typical opaque wall.
pub async fn check_opaque_wall(category: part_2::BuildingCategory, layers: &[part_2::Layer], climate: ClimateZoneDe, airtightness_n50: f64) -> Result<CheckReport, NormError> {
    check_opaque_wall_with_bridges(category, layers, climate, airtightness_n50, 0.02)
}

async fn moisture_layers_from_wall(layers: &[part_2::Layer], mu_exterior: f64, mu_interior: f64) -> Vec<part_3::MoistureLayer> {
    layers
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let mu = if i == 0 { mu_exterior } else { mu_interior };
            part_3::MoistureLayer { thickness_m: l.thickness_m, lambda_w_mk: l.lambda_w_mk, mu }
        })
        .collect()
}

async fn parse_airtightness_class(class: &str) -> part_7::AirtightnessClass {
    match class.to_ascii_lowercase().as_str() {
        "class1" | "1" => part_7::AirtightnessClass::Class1,
        "class3" | "3" => part_7::AirtightnessClass::Class3,
        _ => part_7::AirtightnessClass::Class2,
    }
}

async fn parse_application_type(value: &str) -> part_10::ApplicationType {
    match value.to_ascii_uppercase().as_str() {
        "DAA" => part_10::ApplicationType::Daa,
        "DUK" => part_10::ApplicationType::Duk,
        "DZ" => part_10::ApplicationType::Dz,
        "DI" => part_10::ApplicationType::Di,
        "DEO" => part_10::ApplicationType::Deo,
        _ => part_10::ApplicationType::Dad,
    }
}

async fn parse_application_class(value: &str) -> part_10::ApplicationClass {
    match value.to_ascii_lowercase().as_str() {
        "dk" => part_10::ApplicationClass::Dk,
        "dg" => part_10::ApplicationClass::Dg,
        _ => part_10::ApplicationClass::Dm,
    }
}

/// 📋️ Opaque wall checks including thermal bridge correction ψ·l [W/(m²K)].
pub async fn check_opaque_wall_with_bridges(category: part_2::BuildingCategory, layers: &[part_2::Layer], climate: ClimateZoneDe, airtightness_n50: f64, psi_times_l_sum: f64) -> Result<CheckReport, NormError> {
    check_full_envelope(category, layers, climate, airtightness_n50, psi_times_l_sum, 0.5, 20.0, 0.6, 600.0, 15.0, 1.3, "mineral_wool", "AW-01", "class2", 100.0, true, "DEO", "dk")
}

/// 📋️ Full DIN 4108 parts 1, 2–8, 10, and Beiblatt 2 envelope compliance check.
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub async fn check_full_envelope(
    category: part_2::BuildingCategory,
    layers: &[part_2::Layer],
    climate: ClimateZoneDe,
    airtightness_n50: f64,
    psi_times_l_sum: f64,
    rh_int: f64,
    t_int_c: f64,
    solar_absorptance: f64,
    irradiance_w_m2: f64,
    moisture_mu_exterior: f64,
    moisture_mu_interior: f64,
    material_id: &str,
    catalog_id: &str,
    airtightness_class: &str,
    envelope_area_m2: f64,
    bb2_details_conform: bool,
    application_type: &str,
    declared_application_class: &str,
) -> Result<CheckReport, NormError> {
    let mut report = CheckReport::default();
    for part in [part_1::NormPart::Part1, part_1::NormPart::Part2, part_1::NormPart::Part3, part_1::NormPart::Part4, part_1::NormPart::Part5, part_1::NormPart::Part6, part_1::NormPart::Part7, part_1::NormPart::Part8] {
        report.push(part_1::scope_check(part, part_1::BuildingElement::OpaqueWall));
    }
    let u = part_2::u_value_from_resistance(part_2::total_resistance(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
    report.push(part_1::check_input_plausibility(layers, u));
    let limit = part_2::climate_adjusted_u_limit(category, climate);
    report.push(part_2::check_minimum_thermal_protection(category, layers, climate)?);
    let moisture_layers = moisture_layers_from_wall(layers, moisture_mu_exterior, moisture_mu_interior);
    let t_ext = climate.design_external_temperature_c();
    report.push(part_3::check_surface_temperature(&moisture_layers, t_int_c, t_ext, rh_int)?);
    report.push(part_3::check_glaser_moisture(&moisture_layers, t_int_c, t_ext, rh_int)?);
    let lambda_design = part_4::design_lambda_for_material(material_id)?;
    if let Some(insulation) = layers.last() {
        report.push(part_4::check_design_lambda(material_id, insulation.lambda_w_mk)?);
    } else {
        report.push(part_4::check_design_lambda(material_id, lambda_design)?);
    }
    report.push(part_5::check_summer_heat_protection(layers, climate, t_int_c, solar_absorptance, irradiance_w_m2)?);
    report.push(part_6::check_u_value_with_bridges(layers, psi_times_l_sum, limit)?);
    report.push(part_7::check_airtightness(airtightness_n50, parse_airtightness_class(airtightness_class)));
    report.push(part_8::check_against_catalog(catalog_id, u)?);
    report.push(part_10::check_application_class(parse_application_type(application_type), parse_application_class(declared_application_class)));
    report.push(bb_2::check_beiblatt_2_equivalence(psi_times_l_sum, envelope_area_m2, bb2_details_conform)?);
    Ok(report)
}

async fn parse_category(category: &str) -> part_2::BuildingCategory {
    match category.to_ascii_lowercase().as_str() {
        "office" => part_2::BuildingCategory::Office,
        "school" => part_2::BuildingCategory::School,
        "industrial" => part_2::BuildingCategory::Industrial,
        _ => part_2::BuildingCategory::Residential,
    }
}

/// 📋️ `Din4108Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub async fn evaluate(document: &Din4108Snapshot) -> CheckReport {
    let layers: Vec<part_2::Layer> = document.layers.iter().map(|layer| part_2::Layer { thickness_m: layer.thickness_m, lambda_w_mk: layer.lambda_w_mk }).collect();
    check_full_envelope(
        parse_category(&document.category),
        &layers,
        document.climate,
        document.airtightness_n50,
        document.psi_times_l_sum,
        document.rh_int,
        document.t_int_c,
        document.solar_absorptance,
        document.irradiance_w_m2,
        document.moisture_mu_exterior,
        document.moisture_mu_interior,
        &document.material_id,
        &document.catalog_id,
        &document.airtightness_class,
        document.envelope_area_m2,
        document.bb2_details_conform,
        &document.application_type,
        &document.declared_application_class,
    )
    .unwrap_or_else(|err| {
        let mut report = CheckReport::default();
        report.push(CheckResult::from_utilization(
            ClauseId::new("DIN 4108", "input", "1"),
            Quantity::new(crate::document::QuantityKind::Dimensionless, 2.0),
            Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            err.to_string(),
            AnnexChoice::De,
        ));
        report
    })
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    async fn sample_wall() -> Vec<part_2::Layer> {
        vec![part_2::Layer { thickness_m: 0.24, lambda_w_mk: 0.81 }, part_2::Layer { thickness_m: 0.14, lambda_w_mk: 0.035 }]
    }

    async fn sample_moisture_wall() -> Vec<part_3::MoistureLayer> {
        vec![part_3::MoistureLayer { thickness_m: 0.24, lambda_w_mk: 0.81, mu: 15.0 }, part_3::MoistureLayer { thickness_m: 0.14, lambda_w_mk: 0.035, mu: 1.3 }]
    }

    #[semio_framework_async_macros::async_test]
    async fn opaque_wall_passes_din_4108_suite() {
        let report = check_opaque_wall(part_2::BuildingCategory::Residential, &sample_wall(), ClimateZoneDe::Zone2, 2.5).expect("inputs complete");
        assert!(report.all_pass(), "checks: {:?}", report.checks);
    }

    #[semio_framework_async_macros::async_test]
    async fn full_envelope_evaluate_covers_all_eight_parts() {
        let document = Din4108Snapshot::default();
        let report = evaluate(&document);
        assert!(report.checks.len() >= 15, "expected parts 1–8 checks, got {}", report.checks.len());
        assert!(report.all_pass(), "checks: {:?}", report.checks);
        let f_dry = part_3::interior_surface_temperature_factor(&sample_moisture_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.5);
        let f_humid = part_3::interior_surface_temperature_factor(&sample_moisture_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.8);
        assert!(f_humid < f_dry, "humidity correction must reduce f_Rsi");
    }
}
//#endregion 🧪️ComplianceReportTests
