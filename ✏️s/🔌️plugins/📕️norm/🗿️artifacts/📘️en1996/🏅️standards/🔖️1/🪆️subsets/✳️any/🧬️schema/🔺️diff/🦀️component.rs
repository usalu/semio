//! 🧬️ EN 1996 diff schema — sparse field delta.

use crate::artifacts::en1996::schema::En1996Artifact as EnArtifact;
use crate::artifacts::en1996::En1996Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Diff {
    #[state(artifact)] pub artifact: Option<Box<crate::artifacts::en1996::schema::En1996Artifact>>,
    #[state(artifact)] pub m_ed_knm: Option<f64>,
    #[state(artifact)] pub n_ed_kn: Option<f64>,
    #[state(artifact)] pub v_ed_kn: Option<f64>,
    #[state(artifact)] pub h_ed_kn: Option<f64>,
    #[state(artifact)] pub z_mm3: Option<f64>,
    #[state(artifact)] pub area_mm2: Option<f64>,
    #[state(artifact)] pub shear_area_mm2: Option<f64>,
    #[state(artifact)] pub f_k_mpa: Option<f64>,
    #[state(artifact)] pub f_vk_mpa: Option<f64>,
    #[state(artifact)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)] pub masonry_class: Option<crate::artifacts::en1996::MasonryClass>,
    #[state(artifact)] pub design_situation: Option<crate::document::DesignSituation>,
    #[state(artifact)] pub mu: Option<f64>,
    #[state(artifact)] pub wall_thickness_mm: Option<f64>,
    #[state(artifact)] pub fire_resistance_min: Option<u32>,
    #[state(artifact)] pub unit: Option<String>,
    #[state(artifact)] pub exposure: Option<crate::artifacts::en1996::part_2::ExposureClass>,
    #[state(artifact)] pub mortar: Option<crate::artifacts::en1996::part_2::MortarClass>,
    #[state(artifact)] pub bed_joint_thickness_mm: Option<f64>,
    #[state(artifact)] pub storeys: Option<u32>,
    #[state(artifact)] pub h_ef_mm: Option<f64>,
    #[state(artifact)] pub t_ef_mm: Option<f64>,
    #[state(presence)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
