//! 🪵️ EN 1995 artifact schema — every field with its state class.

use crate::artifacts::en1995::En1995Snapshot;
use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full EN 1995 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Artifact {
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub m_ed_knm: f64,
    #[state(persistent)] pub n_ed_kn: f64,
    #[state(persistent)] pub v_ed_kn: f64,
    #[state(persistent)] pub w_mm3: f64,
    #[state(persistent)] pub a_mm2: f64,
    #[state(persistent)] pub b_mm: f64,
    #[state(persistent)] pub h_mm: f64,
    #[state(persistent)] pub f_m_k: f64,
    #[state(persistent)] pub f_c_0_k: f64,
    #[state(persistent)] pub service_class: String,
    #[state(persistent)] pub load_duration: String,
    #[state(persistent)] pub m_crit_knm: f64,
    #[state(persistent)] pub f_ed_kn: f64,
    #[state(persistent)] pub a_ef_mm2: f64,
    #[state(persistent)] pub f_v_k: f64,
    #[state(persistent)] pub fire_duration_min: f64,
    #[state(persistent)] pub section_depth_mm: f64,
    #[state(persistent)] pub a_vert_m_s2: f64,
    #[state(persistent)] pub n_cycles_bridge: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1995Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1995Snapshot::default())
    }
}

impl From<En1995Snapshot> for En1995Artifact {
    fn from(snapshot: crate::artifacts::en1995::En1995Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1995Artifact {
    pub fn to_snapshot(&self) -> crate::artifacts::en1995::En1995Snapshot {
        crate::artifacts::en1995::En1995Snapshot {
            annex: self.annex.clone(),
            m_ed_knm: self.m_ed_knm.clone(),
            n_ed_kn: self.n_ed_kn.clone(),
            v_ed_kn: self.v_ed_kn.clone(),
            w_mm3: self.w_mm3.clone(),
            a_mm2: self.a_mm2.clone(),
            b_mm: self.b_mm.clone(),
            h_mm: self.h_mm.clone(),
            f_m_k: self.f_m_k.clone(),
            f_c_0_k: self.f_c_0_k.clone(),
            service_class: self.service_class.clone(),
            load_duration: self.load_duration.clone(),
            m_crit_knm: self.m_crit_knm.clone(),
            f_ed_kn: self.f_ed_kn.clone(),
            a_ef_mm2: self.a_ef_mm2.clone(),
            f_v_k: self.f_v_k.clone(),
            fire_duration_min: self.fire_duration_min.clone(),
            section_depth_mm: self.section_depth_mm.clone(),
            a_vert_m_s2: self.a_vert_m_s2.clone(),
            n_cycles_bridge: self.n_cycles_bridge.clone(),
        }
    }

    pub fn from_snapshot(snapshot: crate::artifacts::en1995::En1995Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            m_ed_knm: snapshot.m_ed_knm,
            n_ed_kn: snapshot.n_ed_kn,
            v_ed_kn: snapshot.v_ed_kn,
            w_mm3: snapshot.w_mm3,
            a_mm2: snapshot.a_mm2,
            b_mm: snapshot.b_mm,
            h_mm: snapshot.h_mm,
            f_m_k: snapshot.f_m_k,
            f_c_0_k: snapshot.f_c_0_k,
            service_class: snapshot.service_class,
            load_duration: snapshot.load_duration,
            m_crit_knm: snapshot.m_crit_knm,
            f_ed_kn: snapshot.f_ed_kn,
            a_ef_mm2: snapshot.a_ef_mm2,
            f_v_k: snapshot.f_v_k,
            fire_duration_min: snapshot.fire_duration_min,
            section_depth_mm: snapshot.section_depth_mm,
            a_vert_m_s2: snapshot.a_vert_m_s2,
            n_cycles_bridge: snapshot.n_cycles_bridge,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1995::En1995Snapshot) {
        self.annex = snapshot.annex;
        self.m_ed_knm = snapshot.m_ed_knm;
        self.n_ed_kn = snapshot.n_ed_kn;
        self.v_ed_kn = snapshot.v_ed_kn;
        self.w_mm3 = snapshot.w_mm3;
        self.a_mm2 = snapshot.a_mm2;
        self.b_mm = snapshot.b_mm;
        self.h_mm = snapshot.h_mm;
        self.f_m_k = snapshot.f_m_k;
        self.f_c_0_k = snapshot.f_c_0_k;
        self.service_class = snapshot.service_class;
        self.load_duration = snapshot.load_duration;
        self.m_crit_knm = snapshot.m_crit_knm;
        self.f_ed_kn = snapshot.f_ed_kn;
        self.a_ef_mm2 = snapshot.a_ef_mm2;
        self.f_v_k = snapshot.f_v_k;
        self.fire_duration_min = snapshot.fire_duration_min;
        self.section_depth_mm = snapshot.section_depth_mm;
        self.a_vert_m_s2 = snapshot.a_vert_m_s2;
        self.n_cycles_bridge = snapshot.n_cycles_bridge;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1995_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1995",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
