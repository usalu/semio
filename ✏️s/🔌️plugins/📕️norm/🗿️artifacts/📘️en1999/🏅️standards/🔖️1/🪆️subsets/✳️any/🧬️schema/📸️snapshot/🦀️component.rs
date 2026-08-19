//! ✨️ EN 1999 snapshot schema — artifact-lane fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1999 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1999", layout = "lines")]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Snapshot {
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[state(artifact)]
    pub a_mm2: f64,
    #[state(artifact)]
    pub w_el_mm3: f64,
    #[state(artifact)]
    pub alloy: String,
    #[state(artifact)]
    pub chi: f64,
    #[state(artifact)]
    pub i_t_mm4: f64,
    #[state(artifact)]
    pub l_cr_mm: f64,
    #[state(artifact)]
    pub theta_c: f64,
    #[state(artifact)]
    pub delta_sigma_ed: f64,
    #[state(artifact)]
    pub delta_sigma_c: f64,
    #[state(artifact)]
    pub fatigue_m: f64,
    #[state(artifact)]
    pub n_cycles: f64,
    #[state(artifact)]
    pub v_weld_ed_kn: f64,
    #[state(artifact)]
    pub weld_throat_mm: f64,
    #[state(artifact)]
    pub weld_length_mm: f64,
    #[state(artifact)]
    pub beta_w: f64,
    #[state(artifact)]
    pub sheet_b_mm: f64,
    #[state(artifact)]
    pub sheet_t_mm: f64,
    #[state(artifact)]
    pub sheet_k_sigma: f64,
    #[state(artifact)]
    pub sheet_w_el_mm3: f64,
    #[state(artifact)]
    pub sheet_m_ed_knm: f64,
    #[state(artifact)]
    pub shell_t_mm: f64,
    #[state(artifact)]
    pub shell_r_mm: f64,
    #[state(artifact)]
    pub sigma_ed_shell_mpa: f64,
    #[state(artifact)]
    pub annex: AnnexChoice,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1999Snapshot, extension = "en1999", envelope_id = "norm.en1999");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1999Snapshot {
    fn default() -> Self {
        Self {
            n_ed_kn: 80.0,
            m_ed_knm: 4.0,
            a_mm2: 1200.0,
            w_el_mm3: 24_000.0,
            alloy: "aw6060t6".into(),
            chi: 0.85,
            i_t_mm4: 5000.0,
            l_cr_mm: 3000.0,
            theta_c: 200.0,
            delta_sigma_ed: 45.0,
            delta_sigma_c: 71.0,
            fatigue_m: 8.0,
            n_cycles: 500_000.0,
            v_weld_ed_kn: 25.0,
            weld_throat_mm: 4.0,
            weld_length_mm: 120.0,
            beta_w: 0.63,
            sheet_b_mm: 200.0,
            sheet_t_mm: 2.0,
            sheet_k_sigma: 4.0,
            sheet_w_el_mm3: 8000.0,
            sheet_m_ed_knm: 0.5,
            shell_t_mm: 4.0,
            shell_r_mm: 500.0,
            sigma_ed_shell_mpa: 150.0,
            annex: AnnexChoice::De,
        }
    }
}
