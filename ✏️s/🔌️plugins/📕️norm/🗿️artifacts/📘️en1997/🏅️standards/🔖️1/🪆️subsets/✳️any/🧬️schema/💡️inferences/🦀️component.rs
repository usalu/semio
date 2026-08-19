//! 💡️ En1997 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1997::En1997Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1997Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1997 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1997.inference")]
pub struct En1997Inference {
    #[derived]
    pub outline: En1997Outline,
}

impl protocol::Inference<En1997Snapshot> for En1997Inference {
    async fn infer(snapshot: &En1997Snapshot) -> Self {
        Self { outline: En1997Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1997Snapshot> for En1997Inference {
    async fn inference_schema_id() -> &'static str {
        "s.norm.en1997.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1997.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1997::standards::v1::subsets::any::schema::En1997Builder {
    type Snapshot = En1997Snapshot;
    type Inference = En1997Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1997.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1997_artifact_schema_descriptor`'s registration.
pub async fn en1997_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1997.inference",
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
        let snapshot = En1997Snapshot::default();
        assert_eq!(En1997Inference::infer(&snapshot), En1997Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(En1997Inference::infer(&En1997Snapshot::default()), En1997Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::artifacts::en1997::standards::v1::subsets::any::schema::{check_shallow_foundation, part_1, part_2, DesignApproach};
/// 📋️ Full EN 1997 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1997Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport};

/// 📋️ Full EN 1997 check across bearing, sliding, settlement, pile axial (part 1), and ground investigation adequacy (part 2).
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub async fn check_full_geotechnical(
    v_ed_kn: f64,
    h_ed_kn: f64,
    footing_area_m2: f64,
    phi_deg: f64,
    c_kpa: f64,
    gamma_kn_m3: f64,
    b_m: f64,
    d_f_m: f64,
    e_s_mpa: f64,
    nu: f64,
    approach: DesignApproach,
    annex: AnnexChoice,
    settlement_limit_mm: f64,
    n_pile_ed_kn: f64,
    alpha_s: f64,
    pile_d_m: f64,
    q_s_kpa: f64,
    pile_l_m: f64,
    q_b_kpa: f64,
    pile_base_area_m2: f64,
    pile_n_profiles: u32,
    z_investigated_m: f64,
) -> CheckReport {
    let mut report = check_shallow_foundation(v_ed_kn, h_ed_kn, footing_area_m2, phi_deg, c_kpa, gamma_kn_m3, b_m, d_f_m, e_s_mpa, nu, approach, annex, settlement_limit_mm);
    let r_s_cal = part_1::shaft_resistance_kn(alpha_s, pile_d_m, q_s_kpa, pile_l_m);
    let r_b_cal = part_1::base_resistance_kn(q_b_kpa, pile_base_area_m2);
    let r_s_k = part_1::pile_characteristic_resistance_kn(r_s_cal, r_s_cal, pile_n_profiles);
    let r_b_k = part_1::pile_characteristic_resistance_kn(r_b_cal, r_b_cal, pile_n_profiles);
    let r_c_d = part_1::pile_design_resistance_kn(r_b_k, r_s_k, approach, annex);
    report.push(part_1::check_pile_axial(n_pile_ed_kn, r_c_d, annex));
    report.push(part_2::check_investigation_depth(z_investigated_m, b_m, annex));
    report
}

async fn parse_design_approach(value: &str) -> DesignApproach {
    match value.to_ascii_lowercase().as_str() {
        "da1geo" => DesignApproach::Da1Geo,
        "da2" => DesignApproach::Da2,
        "da3" => DesignApproach::Da3,
        _ => DesignApproach::Da1Str,
    }
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1997Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub async fn evaluate(document: &En1997Snapshot) -> CheckReport {
    check_full_geotechnical(
        document.v_ed_kn,
        document.h_ed_kn,
        document.footing_area_m2,
        document.phi_deg,
        document.c_kpa,
        document.gamma_kn_m3,
        document.b_m,
        document.d_f_m,
        document.e_s_mpa,
        document.nu,
        parse_design_approach(&document.design_approach),
        document.annex,
        document.settlement_limit_mm,
        document.n_pile_ed_kn,
        document.alpha_s,
        document.pile_d_m,
        document.q_s_kpa,
        document.pile_l_m,
        document.q_b_kpa,
        document.pile_base_area_m2,
        document.pile_n_profiles,
        document.z_investigated_m,
    )
}

//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn full_geotechnical_worked_example() {
        let report = check_full_geotechnical(500.0, 80.0, 2.0, 30.0, 0.0, 18.0, 2.0, 1.5, 30_000.0, 0.3, DesignApproach::Da1Str, AnnexChoice::De, 25.0, 800.0, 0.7, 0.6, 80.0, 12.0, 2500.0, 0.28, 1, 8.0);
        assert_eq!(report.checks.len(), 5);
        assert!(report.checks[3].utilization < 1.0);
        assert_eq!(report.checks[4].status, CheckStatus::Pass);
    }

    #[semio_framework_async_macros::async_test]
    async fn evaluate_runs_all_parts() {
        let report = evaluate(&En1997Snapshot::default());
        assert_eq!(report.checks.len(), 5);
    }
}
//#endregion 🧪️ComplianceReportTests
