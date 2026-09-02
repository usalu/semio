//! 💡️ En1995 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1995::En1995Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1995Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1995 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1995.inference")]
pub struct En1995Inference {
    #[derived]
    pub outline: En1995Outline,
}

impl protocol::Inference<En1995Snapshot> for En1995Inference {
    fn infer(snapshot: &En1995Snapshot) -> Self {
        Self { outline: En1995Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1995Snapshot> for En1995Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1995.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1995.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1995::standards::v1::subsets::any::schema::En1995Builder {
    type Snapshot = En1995Snapshot;
    type Inference = En1995Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1995.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1995_artifact_schema_descriptor`'s registration.
pub fn en1995_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1995.inference",
        inference: schema::FacetLeaves {
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
mod tests {
    use super::*;
    use protocol::Inference;

    #[semio_framework_async_macros::async_test]
    fn inference_determinism_law() {
        let snapshot = En1995Snapshot::default();
        assert_eq!(En1995Inference::infer(&snapshot), En1995Inference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    fn inference_default_law() {
        assert_eq!(En1995Inference::infer(&En1995Snapshot::default()), En1995Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::artifacts::en1995::standards::v1::subsets::any::schema::{check_glulam_beam, k_crit, k_mod, lambda_rel_m, part_1_1, part_1_2, part_2, ServiceClass};
/// 📋️ Full EN 1995 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1995Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{AnnexChoice, CheckReport, LoadDuration};

/// 📋️ Full EN 1995 check across bending, compression, shear, connections, fire, and bridge parts.
#[allow(clippy::too_many_arguments)]
pub fn check_full_timber(
    m_ed_knm: f64,
    n_ed_kn: f64,
    v_ed_kn: f64,
    w_mm3: f64,
    a_mm2: f64,
    b_mm: f64,
    h_mm: f64,
    f_m_k: f64,
    f_c_0_k: f64,
    f_v_k: f64,
    service: ServiceClass,
    duration: LoadDuration,
    m_crit_knm: f64,
    f_ed_kn: f64,
    a_ef_mm2: f64,
    fire_duration_min: f64,
    section_depth_mm: f64,
    annex: AnnexChoice,
    a_vert_m_s2: f64,
    n_cycles_bridge: f64,
) -> CheckReport {
    let km = k_mod(service, duration);
    let lambda = lambda_rel_m(w_mm3, f_m_k, m_crit_knm);
    let kc = k_crit(lambda);
    let mut report = check_glulam_beam(m_ed_knm, n_ed_kn, v_ed_kn, w_mm3, a_mm2, b_mm, h_mm, f_m_k, f_c_0_k, f_v_k, service, duration, m_crit_knm, annex);
    let f_rd = part_1_1::connection_bearing_resistance_kn(a_ef_mm2, f_v_k, km, annex);
    report.push(part_1_1::check_connection_bearing(f_ed_kn, f_rd, annex));
    let charred = part_1_2::charred_depth_mm(fire_duration_min);
    let remaining = part_1_2::residual_section_mm(section_depth_mm, charred);
    report.push(part_1_2::check_fire(charred, remaining));
    let m_rd_bridge = part_2::bridge_bending_resistance_knm(w_mm3, f_m_k, service, duration, kc, annex);
    report.push(part_2::check_bridge_timber(m_ed_knm, m_rd_bridge));
    report.push(part_2::check_pedestrian_vibration(a_vert_m_s2));
    report.push(part_2::check_bridge_fatigue(m_ed_knm, m_rd_bridge, n_cycles_bridge));
    report
}

fn parse_service_class(value: &str) -> ServiceClass {
    match value.to_ascii_lowercase().as_str() {
        "sc2" => ServiceClass::Sc2,
        "sc3" => ServiceClass::Sc3,
        _ => ServiceClass::Sc1,
    }
}

fn parse_load_duration(value: &str) -> LoadDuration {
    match value.to_ascii_lowercase().as_str() {
        "permanent" => LoadDuration::Permanent,
        "long" => LoadDuration::Long,
        "short" => LoadDuration::Short,
        "instantaneous" => LoadDuration::Instantaneous,
        _ => LoadDuration::Medium,
    }
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1995Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub fn evaluate(document: &En1995Snapshot) -> CheckReport {
    check_full_timber(
        document.m_ed_knm,
        document.n_ed_kn,
        document.v_ed_kn,
        document.w_mm3,
        document.a_mm2,
        document.b_mm,
        document.h_mm,
        document.f_m_k,
        document.f_c_0_k,
        document.f_v_k,
        parse_service_class(&document.service_class),
        parse_load_duration(&document.load_duration),
        document.m_crit_knm,
        document.f_ed_kn,
        document.a_ef_mm2,
        document.fire_duration_min,
        document.section_depth_mm,
        document.annex,
        document.a_vert_m_s2,
        document.n_cycles_bridge,
    )
}

//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn full_timber_worked_example() {
        let report = check_full_timber(25.0, 50.0, 15.0, 1_800_000.0, 20_000.0, 200.0, 300.0, 24.0, 21.0, 4.0, ServiceClass::Sc1, LoadDuration::Medium, 80.0, 18.0, 12_000.0, 30.0, 300.0, AnnexChoice::De, 0.3, 500_000.0);
        assert_eq!(report.checks.len(), 8);
        assert!(report.checks[0].utilization < 1.0, "beam bending check should pass");
        assert!(report.checks[2].utilization < 1.0, "shear check should pass");
        assert!(report.checks[5].utilization < 1.0, "bridge bending check should pass");
        assert!(report.checks[6].utilization < 1.0, "pedestrian vibration check should pass");
        assert!(report.checks[7].utilization < 1.0, "bridge fatigue check should pass");
    }

    #[semio_framework_async_macros::async_test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&En1995Snapshot::default());
        assert_eq!(report.checks.len(), 8);
    }
}
//#endregion 🧪️ComplianceReportTests
