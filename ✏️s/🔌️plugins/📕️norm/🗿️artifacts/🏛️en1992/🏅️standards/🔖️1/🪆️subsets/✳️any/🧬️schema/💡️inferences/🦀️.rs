//! 💡️ En1992 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::En1992Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;


//#region 🔖️Inference
/// 💡️ Everything inferable from a en1992 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1992.inference")]
pub struct En1992Inference {
    #[derived]
    pub outline: En1992Outline,
}

impl protocol::Inference<En1992Snapshot> for En1992Inference {
    fn infer(snapshot: &En1992Snapshot) -> Self {
        Self { outline: En1992Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1992Snapshot> for En1992Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1992.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1992.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1992Builder {
    type Snapshot = En1992Snapshot;
    type Inference = En1992Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1992.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1992_artifact_schema_descriptor`'s registration.
pub fn en1992_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1992.inference",
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
#[cfg(feature = "cross-fem")]
use crate::standards::v1::subsets::any::schema::check_rc_beam_from_fem;
use crate::standards::v1::subsets::any::schema::{check_full_rc_beam, part_1_2, part_2, part_3, part_4};
/// 📋️ Full EN 1992 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. `evaluate` is the `En1992Snapshot -> CheckReport` projection; everything it composes
/// is a pure helper living in the parent `🧬️schema`.
use crate::document::CheckReport;

/// 📋️ `En1992Snapshot -> CheckReport` conformance law — the artifact's compliance evaluation.
pub fn evaluate(document: &En1992Snapshot) -> CheckReport {
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
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::En1992Outline;
//#endregion 🔁️Re-exports
