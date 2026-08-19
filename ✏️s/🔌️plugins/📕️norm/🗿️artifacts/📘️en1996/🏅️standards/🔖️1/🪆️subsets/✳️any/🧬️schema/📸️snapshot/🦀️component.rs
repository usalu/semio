//! 🧱️ EN 1996 snapshot schema — artifact-lane fields only.

use crate::artifacts::en1996::{part_2, MasonryClass};
use crate::document::{AnnexChoice, DesignSituation};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1996 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1996", layout = "lines")]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Snapshot {
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub h_ed_kn: f64,
    #[state(artifact)]
    pub z_mm3: f64,
    #[state(artifact)]
    pub area_mm2: f64,
    #[state(artifact)]
    pub shear_area_mm2: f64,
    #[state(artifact)]
    pub f_k_mpa: f64,
    #[state(artifact)]
    pub f_vk_mpa: f64,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub masonry_class: MasonryClass,
    #[state(artifact)]
    pub design_situation: DesignSituation,
    #[state(artifact)]
    pub mu: f64,
    #[state(artifact)]
    pub wall_thickness_mm: f64,
    #[state(artifact)]
    pub fire_resistance_min: u32,
    #[state(artifact)]
    pub unit: String,
    #[state(artifact)]
    pub exposure: part_2::ExposureClass,
    #[state(artifact)]
    pub mortar: part_2::MortarClass,
    #[state(artifact)]
    pub bed_joint_thickness_mm: f64,
    #[state(artifact)]
    pub storeys: u32,
    #[state(artifact)]
    pub h_ef_mm: f64,
    #[state(artifact)]
    pub t_ef_mm: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1996Snapshot, extension = "en1996", envelope_id = "norm.en1996");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1996Snapshot {
    async fn default() -> Self {
        Self {
            m_ed_knm: 8.0,
            n_ed_kn: 200.0,
            v_ed_kn: 35.0,
            h_ed_kn: 20.0,
            z_mm3: 8_000_000.0,
            area_mm2: 500_000.0,
            shear_area_mm2: 300_000.0,
            f_k_mpa: 5.0,
            f_vk_mpa: 0.15,
            annex: AnnexChoice::De,
            masonry_class: MasonryClass::default(),
            design_situation: DesignSituation::Persistent,
            mu: 0.4,
            wall_thickness_mm: 240.0,
            fire_resistance_min: 60,
            unit: "clay".into(),
            exposure: part_2::ExposureClass::Mx1,
            mortar: part_2::MortarClass::M5,
            bed_joint_thickness_mm: 12.0,
            storeys: 2,
            h_ef_mm: 2500.0,
            t_ef_mm: 240.0,
        }
    }
}
