//! 💡️ En1996 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1996::En1996Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1996Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1996 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1996.inference")]
pub struct En1996Inference {
    #[state(inferred)]
    pub outline: En1996Outline,
}

impl protocol::Inference<En1996Snapshot> for En1996Inference {
    fn infer(snapshot: &En1996Snapshot) -> Self {
        Self { outline: En1996Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1996Snapshot> for En1996Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1996.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1996.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1996::standards::v1::subsets::any::schema::En1996Builder {
    type Snapshot = En1996Snapshot;
    type Inference = En1996Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1996.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1996_artifact_schema_descriptor`'s registration.
pub fn en1996_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1996.inference",
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

    #[test]
    fn inference_determinism_law() {
        let snapshot = En1996Snapshot::default();
        assert_eq!(En1996Inference::infer(&snapshot), En1996Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(En1996Inference::infer(&En1996Snapshot::default()), En1996Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
/// 📋️ Full EN 1996 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1996Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::{CheckReport, DesignSituation};
use crate::artifacts::en1996::standards::v1::subsets::any::schema::{AnnexParams, MasonryUnit, part_1_1, part_1_2, part_2, part_3};

fn parse_masonry_unit(value: &str) -> MasonryUnit {
    match value.to_ascii_lowercase().as_str() {
        "calcium_silicate" | "calcium silicate" => MasonryUnit::CalciumSilicate,
        "aac" => MasonryUnit::Aac,
        _ => MasonryUnit::Clay,
    }
}

/// ⚖️ Derive the resolved γ_M annex parameters from a document's annex/class/situation inputs. Moved
/// here (from an inherent `En1996Snapshot::annex_params()` method in the pre-split monolith) because it
/// constructs the compute-layer `AnnexParams`, which cannot be an inherent impl on the foreign
/// `crate::artifacts::en1996::En1996Snapshot` type across the crate boundary (Rust's orphan rule).
pub fn annex_params(document: &En1996Snapshot) -> AnnexParams {
    AnnexParams { annex: document.annex, masonry_class: document.masonry_class, accidental: document.design_situation == DesignSituation::Accidental }
}

/// 📋️ Full EN 1996 check across flexure, compression, shear, sliding (part 1-1), fire wall (part 1-2), exposure/bed-joint (part 2), and the simplified method (part 3).
pub fn check_full_masonry(document: &En1996Snapshot) -> CheckReport {
    let g_m = annex_params(document).gamma_m();
    let f_d = part_1_1::design_strength_mpa(document.f_k_mpa, g_m);
    let f_vd = part_1_1::shear_design_strength_mpa(document.f_vk_mpa, g_m);
    let sigma = document.n_ed_kn * 1000.0 / document.area_mm2;
    let m_rd_flex = part_1_1::flexural_resistance_knm(document.z_mm3, f_d);
    let v_rd = part_1_1::shear_resistance_kn(document.shear_area_mm2, f_vd);
    let h_rd = part_1_1::sliding_resistance_kn(document.mu, document.n_ed_kn, f_vd, document.shear_area_mm2);
    let unit = parse_masonry_unit(&document.unit);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_flexure(document.m_ed_knm, m_rd_flex, document.annex));
    report.push(part_1_1::check_compression(sigma, f_d, document.annex));
    report.push(part_1_1::check_shear(document.v_ed_kn, v_rd, document.annex));
    report.push(part_1_1::check_sliding(document.h_ed_kn, h_rd, document.annex));
    let required_fire = part_1_2::required_wall_thickness_mm(document.fire_resistance_min, unit);
    report.push(part_1_2::check_fire_wall(document.wall_thickness_mm, required_fire));
    report.push(part_2::check_exposure_mortar(document.exposure, unit, document.mortar));
    report.push(part_2::check_bed_joint_thickness(document.bed_joint_thickness_mm));
    let phi_s = part_3::phi_s(document.h_ef_mm, document.t_ef_mm);
    report.push(part_3::check_simplified_compression(document.n_ed_kn, phi_s, f_d, document.area_mm2, document.storeys, document.h_ef_mm, document.t_ef_mm, document.annex));
    report
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1996Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub fn evaluate(document: &En1996Snapshot) -> CheckReport {
    check_full_masonry(document)
}

//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;

    #[test]
    fn full_masonry_worked_example() {
        let report = check_full_masonry(&En1996Snapshot::default());
        assert_eq!(report.checks.len(), 8);
    }

    #[test]
    fn evaluate_runs_all_parts() {
        let report = evaluate(&En1996Snapshot::default());
        assert_eq!(report.checks.len(), 8);
    }
}
//#endregion 🧪️ComplianceReportTests

