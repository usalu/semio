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
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1996::schema::En1996Artifact>>,
    #[state(persistent)] pub m_ed_knm: Option<f64>,
    #[state(persistent)] pub n_ed_kn: Option<f64>,
    #[state(persistent)] pub v_ed_kn: Option<f64>,
    #[state(persistent)] pub h_ed_kn: Option<f64>,
    #[state(persistent)] pub z_mm3: Option<f64>,
    #[state(persistent)] pub area_mm2: Option<f64>,
    #[state(persistent)] pub shear_area_mm2: Option<f64>,
    #[state(persistent)] pub f_k_mpa: Option<f64>,
    #[state(persistent)] pub f_vk_mpa: Option<f64>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub masonry_class: Option<crate::document::MasonryClass>,
    #[state(persistent)] pub design_situation: Option<crate::document::DesignSituation>,
    #[state(persistent)] pub mu: Option<f64>,
    #[state(persistent)] pub wall_thickness_mm: Option<f64>,
    #[state(persistent)] pub fire_resistance_min: Option<u32>,
    #[state(persistent)] pub unit: Option<String>,
    #[state(persistent)] pub exposure: Option<crate::artifacts::en1996::part_2::ExposureClass>,
    #[state(persistent)] pub mortar: Option<crate::artifacts::en1996::part_2::MortarClass>,
    #[state(persistent)] pub bed_joint_thickness_mm: Option<f64>,
    #[state(persistent)] pub storeys: Option<u32>,
    #[state(persistent)] pub h_ef_mm: Option<f64>,
    #[state(persistent)] pub t_ef_mm: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
