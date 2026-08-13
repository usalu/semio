//! 🪵️ EN 1995 snapshot schema — persistent fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1995 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1995", layout = "lines")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Snapshot {
    #[state(artifact)]
    pub annex: crate::document::AnnexChoice,
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub w_mm3: f64,
    #[state(artifact)]
    pub a_mm2: f64,
    #[state(artifact)]
    pub b_mm: f64,
    #[state(artifact)]
    pub h_mm: f64,
    #[state(artifact)]
    pub f_m_k: f64,
    #[state(artifact)]
    pub f_c_0_k: f64,
    #[state(artifact)]
    pub service_class: String,
    #[state(artifact)]
    pub load_duration: String,
    #[state(artifact)]
    pub m_crit_knm: f64,
    #[state(artifact)]
    pub f_ed_kn: f64,
    #[state(artifact)]
    pub a_ef_mm2: f64,
    #[state(artifact)]
    pub f_v_k: f64,
    #[state(artifact)]
    pub fire_duration_min: f64,
    #[state(artifact)]
    pub section_depth_mm: f64,
    #[state(artifact)]
    pub a_vert_m_s2: f64,
    #[state(artifact)]
    pub n_cycles_bridge: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1995Snapshot, extension = "en1995", envelope_id = "norm.en1995");
//#endregion 🔖️HandcraftedArtifactCodecs


impl Default for En1995Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            m_ed_knm: 25.0,
            n_ed_kn: 50.0,
            v_ed_kn: 15.0,
            w_mm3: 1_000_000.0,
            a_mm2: 20_000.0,
            b_mm: 200.0,
            h_mm: 300.0,
            f_m_k: 24.0,
            f_c_0_k: 21.0,
            service_class: "sc1".into(),
            load_duration: "medium".into(),
            m_crit_knm: 80.0,
            f_ed_kn: 18.0,
            a_ef_mm2: 12_000.0,
            f_v_k: 4.0,
            fire_duration_min: 30.0,
            section_depth_mm: 300.0,
            a_vert_m_s2: 0.3,
            n_cycles_bridge: 500_000.0,
        }
    }
}
