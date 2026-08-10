//! 🌍️ EN 1997 artifact schema — every field with its state class.

use crate::document::AnnexChoice;
use crate::artifacts::en1997::En1997Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full EN 1997 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Artifact {
    #[state(persistent)] pub v_ed_kn: f64,
    #[state(persistent)] pub h_ed_kn: f64,
    #[state(persistent)] pub footing_area_m2: f64,
    #[state(persistent)] pub phi_deg: f64,
    #[state(persistent)] pub c_kpa: f64,
    #[state(persistent)] pub gamma_kn_m3: f64,
    #[state(persistent)] pub b_m: f64,
    #[state(persistent)] pub d_f_m: f64,
    #[state(persistent)] pub e_s_mpa: f64,
    #[state(persistent)] pub nu: f64,
    #[state(persistent)] pub design_approach: String,
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub settlement_limit_mm: f64,
    #[state(persistent)] pub n_pile_ed_kn: f64,
    #[state(persistent)] pub alpha_s: f64,
    #[state(persistent)] pub pile_d_m: f64,
    #[state(persistent)] pub q_s_kpa: f64,
    #[state(persistent)] pub pile_l_m: f64,
    #[state(persistent)] pub q_b_kpa: f64,
    #[state(persistent)] pub pile_base_area_m2: f64,
    #[state(persistent)] pub pile_n_profiles: u32,
    #[state(persistent)] pub z_investigated_m: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1997Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1997Snapshot::default())
    }
}

impl From<En1997Snapshot> for En1997Artifact {
    fn from(snapshot: crate::artifacts::en1997::En1997Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1997Artifact {
    pub fn to_snapshot(&self) -> crate::artifacts::en1997::En1997Snapshot {
        crate::artifacts::en1997::En1997Snapshot {
            v_ed_kn: self.v_ed_kn.clone(),
            h_ed_kn: self.h_ed_kn.clone(),
            footing_area_m2: self.footing_area_m2.clone(),
            phi_deg: self.phi_deg.clone(),
            c_kpa: self.c_kpa.clone(),
            gamma_kn_m3: self.gamma_kn_m3.clone(),
            b_m: self.b_m.clone(),
            d_f_m: self.d_f_m.clone(),
            e_s_mpa: self.e_s_mpa.clone(),
            nu: self.nu.clone(),
            design_approach: self.design_approach.clone(),
            annex: self.annex.clone(),
            settlement_limit_mm: self.settlement_limit_mm.clone(),
            n_pile_ed_kn: self.n_pile_ed_kn.clone(),
            alpha_s: self.alpha_s.clone(),
            pile_d_m: self.pile_d_m.clone(),
            q_s_kpa: self.q_s_kpa.clone(),
            pile_l_m: self.pile_l_m.clone(),
            q_b_kpa: self.q_b_kpa.clone(),
            pile_base_area_m2: self.pile_base_area_m2.clone(),
            pile_n_profiles: self.pile_n_profiles.clone(),
            z_investigated_m: self.z_investigated_m.clone(),
        }
    }

    pub fn from_snapshot(snapshot: crate::artifacts::en1997::En1997Snapshot) -> Self {
        Self {
            v_ed_kn: snapshot.v_ed_kn,
            h_ed_kn: snapshot.h_ed_kn,
            footing_area_m2: snapshot.footing_area_m2,
            phi_deg: snapshot.phi_deg,
            c_kpa: snapshot.c_kpa,
            gamma_kn_m3: snapshot.gamma_kn_m3,
            b_m: snapshot.b_m,
            d_f_m: snapshot.d_f_m,
            e_s_mpa: snapshot.e_s_mpa,
            nu: snapshot.nu,
            design_approach: snapshot.design_approach,
            annex: snapshot.annex,
            settlement_limit_mm: snapshot.settlement_limit_mm,
            n_pile_ed_kn: snapshot.n_pile_ed_kn,
            alpha_s: snapshot.alpha_s,
            pile_d_m: snapshot.pile_d_m,
            q_s_kpa: snapshot.q_s_kpa,
            pile_l_m: snapshot.pile_l_m,
            q_b_kpa: snapshot.q_b_kpa,
            pile_base_area_m2: snapshot.pile_base_area_m2,
            pile_n_profiles: snapshot.pile_n_profiles,
            z_investigated_m: snapshot.z_investigated_m,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1997::En1997Snapshot) {
        self.v_ed_kn = snapshot.v_ed_kn;
        self.h_ed_kn = snapshot.h_ed_kn;
        self.footing_area_m2 = snapshot.footing_area_m2;
        self.phi_deg = snapshot.phi_deg;
        self.c_kpa = snapshot.c_kpa;
        self.gamma_kn_m3 = snapshot.gamma_kn_m3;
        self.b_m = snapshot.b_m;
        self.d_f_m = snapshot.d_f_m;
        self.e_s_mpa = snapshot.e_s_mpa;
        self.nu = snapshot.nu;
        self.design_approach = snapshot.design_approach;
        self.annex = snapshot.annex;
        self.settlement_limit_mm = snapshot.settlement_limit_mm;
        self.n_pile_ed_kn = snapshot.n_pile_ed_kn;
        self.alpha_s = snapshot.alpha_s;
        self.pile_d_m = snapshot.pile_d_m;
        self.q_s_kpa = snapshot.q_s_kpa;
        self.pile_l_m = snapshot.pile_l_m;
        self.q_b_kpa = snapshot.q_b_kpa;
        self.pile_base_area_m2 = snapshot.pile_base_area_m2;
        self.pile_n_profiles = snapshot.pile_n_profiles;
        self.z_investigated_m = snapshot.z_investigated_m;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1997_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1997",
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
