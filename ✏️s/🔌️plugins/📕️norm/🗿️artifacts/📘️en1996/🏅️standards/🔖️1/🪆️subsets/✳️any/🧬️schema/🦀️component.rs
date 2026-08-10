//! 🧱️ EN 1996 artifact schema — every field with its state class.

use crate::artifacts::en1996::{MasonryClass, part_2};
use crate::document::{AnnexChoice, DesignSituation};
use crate::artifacts::en1996::En1996Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full EN 1996 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Artifact {
    #[state(persistent)] pub m_ed_knm: f64,
    #[state(persistent)] pub n_ed_kn: f64,
    #[state(persistent)] pub v_ed_kn: f64,
    #[state(persistent)] pub h_ed_kn: f64,
    #[state(persistent)] pub z_mm3: f64,
    #[state(persistent)] pub area_mm2: f64,
    #[state(persistent)] pub shear_area_mm2: f64,
    #[state(persistent)] pub f_k_mpa: f64,
    #[state(persistent)] pub f_vk_mpa: f64,
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub masonry_class: crate::artifacts::en1996::MasonryClass,
    #[state(persistent)] pub design_situation: crate::document::DesignSituation,
    #[state(persistent)] pub mu: f64,
    #[state(persistent)] pub wall_thickness_mm: f64,
    #[state(persistent)] pub fire_resistance_min: u32,
    #[state(persistent)] pub unit: String,
    #[state(persistent)] pub exposure: crate::artifacts::en1996::part_2::ExposureClass,
    #[state(persistent)] pub mortar: crate::artifacts::en1996::part_2::MortarClass,
    #[state(persistent)] pub bed_joint_thickness_mm: f64,
    #[state(persistent)] pub storeys: u32,
    #[state(persistent)] pub h_ef_mm: f64,
    #[state(persistent)] pub t_ef_mm: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1996Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1996Snapshot::default())
    }
}

impl From<En1996Snapshot> for En1996Artifact {
    fn from(snapshot: crate::artifacts::en1996::En1996Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1996Artifact {
    pub fn to_snapshot(&self) -> crate::artifacts::en1996::En1996Snapshot {
        crate::artifacts::en1996::En1996Snapshot {
            m_ed_knm: self.m_ed_knm.clone(),
            n_ed_kn: self.n_ed_kn.clone(),
            v_ed_kn: self.v_ed_kn.clone(),
            h_ed_kn: self.h_ed_kn.clone(),
            z_mm3: self.z_mm3.clone(),
            area_mm2: self.area_mm2.clone(),
            shear_area_mm2: self.shear_area_mm2.clone(),
            f_k_mpa: self.f_k_mpa.clone(),
            f_vk_mpa: self.f_vk_mpa.clone(),
            annex: self.annex.clone(),
            masonry_class: self.masonry_class.clone(),
            design_situation: self.design_situation.clone(),
            mu: self.mu.clone(),
            wall_thickness_mm: self.wall_thickness_mm.clone(),
            fire_resistance_min: self.fire_resistance_min.clone(),
            unit: self.unit.clone(),
            exposure: self.exposure.clone(),
            mortar: self.mortar.clone(),
            bed_joint_thickness_mm: self.bed_joint_thickness_mm.clone(),
            storeys: self.storeys.clone(),
            h_ef_mm: self.h_ef_mm.clone(),
            t_ef_mm: self.t_ef_mm.clone(),
        }
    }

    pub fn from_snapshot(snapshot: crate::artifacts::en1996::En1996Snapshot) -> Self {
        Self {
            m_ed_knm: snapshot.m_ed_knm,
            n_ed_kn: snapshot.n_ed_kn,
            v_ed_kn: snapshot.v_ed_kn,
            h_ed_kn: snapshot.h_ed_kn,
            z_mm3: snapshot.z_mm3,
            area_mm2: snapshot.area_mm2,
            shear_area_mm2: snapshot.shear_area_mm2,
            f_k_mpa: snapshot.f_k_mpa,
            f_vk_mpa: snapshot.f_vk_mpa,
            annex: snapshot.annex,
            masonry_class: snapshot.masonry_class,
            design_situation: snapshot.design_situation,
            mu: snapshot.mu,
            wall_thickness_mm: snapshot.wall_thickness_mm,
            fire_resistance_min: snapshot.fire_resistance_min,
            unit: snapshot.unit,
            exposure: snapshot.exposure,
            mortar: snapshot.mortar,
            bed_joint_thickness_mm: snapshot.bed_joint_thickness_mm,
            storeys: snapshot.storeys,
            h_ef_mm: snapshot.h_ef_mm,
            t_ef_mm: snapshot.t_ef_mm,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1996::En1996Snapshot) {
        self.m_ed_knm = snapshot.m_ed_knm;
        self.n_ed_kn = snapshot.n_ed_kn;
        self.v_ed_kn = snapshot.v_ed_kn;
        self.h_ed_kn = snapshot.h_ed_kn;
        self.z_mm3 = snapshot.z_mm3;
        self.area_mm2 = snapshot.area_mm2;
        self.shear_area_mm2 = snapshot.shear_area_mm2;
        self.f_k_mpa = snapshot.f_k_mpa;
        self.f_vk_mpa = snapshot.f_vk_mpa;
        self.annex = snapshot.annex;
        self.masonry_class = snapshot.masonry_class;
        self.design_situation = snapshot.design_situation;
        self.mu = snapshot.mu;
        self.wall_thickness_mm = snapshot.wall_thickness_mm;
        self.fire_resistance_min = snapshot.fire_resistance_min;
        self.unit = snapshot.unit;
        self.exposure = snapshot.exposure;
        self.mortar = snapshot.mortar;
        self.bed_joint_thickness_mm = snapshot.bed_joint_thickness_mm;
        self.storeys = snapshot.storeys;
        self.h_ef_mm = snapshot.h_ef_mm;
        self.t_ef_mm = snapshot.t_ef_mm;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1996_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1996",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
