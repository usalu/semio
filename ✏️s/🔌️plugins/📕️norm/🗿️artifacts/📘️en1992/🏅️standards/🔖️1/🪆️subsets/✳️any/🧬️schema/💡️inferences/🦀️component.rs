//! 💡️ En1992 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1992::En1992Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1992Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1992 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1992.inference")]
pub struct En1992Inference {
    #[derived]
    pub outline: En1992Outline,
}

impl protocol::Inference<En1992Snapshot> for En1992Inference {
    async fn infer(snapshot: &En1992Snapshot) -> Self {
        Self { outline: En1992Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1992Snapshot> for En1992Inference {
    async fn inference_schema_id() -> &'static str {
        "s.norm.en1992.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1992.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1992::standards::v1::subsets::any::schema::En1992Builder {
    type Snapshot = En1992Snapshot;
    type Inference = En1992Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1992.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1992_artifact_schema_descriptor`'s registration.
pub async fn en1992_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1992.inference",
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
    async fn inference_determinism_law() {
        let snapshot = En1992Snapshot::default();
        assert_eq!(En1992Inference::infer(&snapshot), En1992Inference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(En1992Inference::infer(&En1992Snapshot::default()), En1992Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
#[cfg(feature = "cross-fem")]
use crate::artifacts::en1992::standards::v1::subsets::any::schema::check_rc_beam_from_fem;
use crate::artifacts::en1992::standards::v1::subsets::any::schema::{check_full_rc_beam, part_1_2, part_2, part_3, part_4};
/// 📋️ Full EN 1992 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1992Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::CheckReport;

/// 📋️ `En1992Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub async fn evaluate(document: &En1992Snapshot) -> CheckReport {
    let mut report = if document.use_fem {
        #[cfg(feature = "cross-fem")]
        {
            check_rc_beam_from_fem(document.span_m, document.udl_kn_m, document.f_ck, document.b_mm, document.d_mm, document.a_s_mm2, document.f_yk, document.rho_l, document.annex).unwrap_or_else(|_| CheckReport::default())
        }
        #[cfg(not(feature = "cross-fem"))]
        {
            CheckReport::default()
        }
    } else {
        check_full_rc_beam(document.m_ed_knm, document.v_ed_kn, document.f_ck, document.b_mm, document.d_mm, document.a_s_mm2, document.f_yk, document.rho_l, document.n_ed_kn, document.p_kn, document.a_c_mm2, document.annex)
    };

    report.push(part_1_2::check_fire_beam_axis_distance(document.b_mm, document.provided_axis_distance_mm, document.fire_rating));

    report.push(part_2::check_bridge_concrete_stress(document.bridge_sigma_c_mpa, document.f_ck));
    report.push(part_2::check_bridge_fatigue(document.bridge_delta_sigma_s_mpa));

    let w_k_liquid = part_3::crack_width_tightness_mm(document.liquid_sigma_s_mpa, document.liquid_rho_p_eff, document.liquid_f_ct_eff_mpa, document.liquid_e_s_mpa, document.liquid_s_r_max_mm);
    report.push(part_3::check_tightness_crack_width(w_k_liquid, document.tightness_class, document.hd_over_h));

    let anchor_n_ed_n = document.anchor_n_ed_kn * 1000.0;
    report.push(part_4::check_anchor_steel(anchor_n_ed_n, document.anchor_a_s_mm2, document.anchor_f_uk_mpa, document.anchor_f_yk_mpa));
    report.push(part_4::check_anchor_concrete_cone(anchor_n_ed_n, document.f_ck, document.anchor_h_ef_mm, document.anchor_cracked));
    report.push(part_4::check_anchor_edge_shear(document.anchor_v_ed_kn * 1000.0, document.anchor_d_mm, document.anchor_h_ef_mm, document.f_ck, document.anchor_c1_mm));

    report
}
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;
    use crate::document::CheckStatus;

    #[test]
    #[cfg(feature = "cross-fem")]
    async fn evaluate_fem_path() {
        let doc = En1992Snapshot { use_fem: true, ..En1992Snapshot::default() };
        let report = evaluate(&doc);
        assert!(!report.checks.is_empty());
        let m_ed = report.checks[0].computed.value / 1_000_000.0;
        assert!((m_ed - 90.0).abs() < 1.0);
    }

    #[test]
    async fn evaluate_analytical_with_prestress() {
        let doc = En1992Snapshot { p_kn: 800.0, ..En1992Snapshot::default() };
        let report = evaluate(&doc);
        assert_eq!(report.checks.len(), 10);
        assert!(report.checks.iter().all(|c| c.status != CheckStatus::NotApplicable));
    }

    #[test]
    async fn evaluate_covers_all_parts() {
        let report = evaluate(&En1992Snapshot::default());
        assert_eq!(report.checks.len(), 9);
        let families: Vec<&str> = report.checks.iter().map(|c| c.clause.family.as_str()).collect();
        assert!(families.contains(&"EN 1992-1-1"));
        assert!(families.contains(&"EN 1992-1-2"));
        assert!(families.contains(&"EN 1992-2"));
        assert!(families.contains(&"EN 1992-3"));
        assert!(families.contains(&"EN 1992-4"));
    }
}
//#endregion 🧪️ComplianceReportTests
