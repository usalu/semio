//! 🌋️ EN 1998 artifact schema — every field with its state class.

use crate::artifacts::en1998::En1998Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full EN 1998 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Artifact {
    #[state(artifact)]
    pub seismic_zone: u8,
    #[state(artifact)]
    pub ground_type: String,
    #[state(artifact)]
    pub importance_class: String,
    #[state(artifact)]
    pub structural_system: String,
    #[state(artifact)]
    pub t1_s: f64,
    #[state(artifact)]
    pub mass_t: f64,
    #[state(artifact)]
    pub v_rd_kn: f64,
    #[state(artifact)]
    pub drift_mm: f64,
    #[state(artifact)]
    pub height_m: f64,
    #[state(artifact)]
    pub multiple_resisting_systems: bool,
    #[state(artifact)]
    pub annex: String,
    #[state(artifact)]
    pub en_a_gr: f64,
    #[state(artifact)]
    pub en_ground_type: String,
    #[state(artifact)]
    pub en_spectrum_type: String,
    #[state(artifact)]
    pub period_ratio: f64,
    #[state(artifact)]
    pub bridge_v_rd_kn: f64,
    #[state(artifact)]
    pub bearing_d_ed_mm: f64,
    #[state(artifact)]
    pub bearing_d_rd_mm: f64,
    #[state(artifact)]
    pub retrofit_knowledge_level: String,
    #[state(artifact)]
    pub retrofit_limit_state: String,
    #[state(artifact)]
    pub retrofit_e_d_kn: f64,
    #[state(artifact)]
    pub retrofit_r_k_kn: f64,
    #[state(artifact)]
    pub retrofit_gamma_el: f64,
    #[state(artifact)]
    pub silo_height_m: f64,
    #[state(artifact)]
    pub silo_radius_m: f64,
    #[state(artifact)]
    pub silo_n_rd_kn: f64,
    #[state(artifact)]
    pub silo_v_ed_kn: f64,
    #[state(artifact)]
    pub silo_v_rd_kn: f64,
    #[state(artifact)]
    pub silo_q_nominal: f64,
    #[state(artifact)]
    pub tank_height_m: f64,
    #[state(artifact)]
    pub tank_radius_m: f64,
    #[state(artifact)]
    pub tank_mass_t: f64,
    #[state(artifact)]
    pub tank_v_rd_kn: f64,
    #[state(artifact)]
    pub tower_m_ed_knm: f64,
    #[state(artifact)]
    pub tower_m_rd_knm: f64,
    #[state(artifact)]
    pub tower_is_chimney: bool,
    #[state(artifact)]
    pub tower_q_nominal: f64,
    #[state(artifact)]
    pub tower_mass_t: f64,
    #[state(artifact)]
    pub foundation_area_m2: f64,
    #[state(artifact)]
    pub foundation_p_rd_kpa: f64,
    #[state(artifact)]
    pub foundation_h_ed_kn: f64,
    #[state(artifact)]
    pub foundation_h_rd_kn: f64,
    #[state(artifact)]
    pub k_foundation: f64,
    #[state(artifact)]
    pub k_soil: f64,
    #[state(artifact)]
    pub wall_height_m: f64,
    #[state(artifact)]
    pub wall_phi_deg: f64,
    #[state(artifact)]
    pub wall_soil_gamma_kn_m3: f64,
    #[state(artifact)]
    pub wall_r: f64,
    #[state(artifact)]
    pub wall_h_rd_kn: f64,
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1998Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1998Snapshot::default())
    }
}

impl From<En1998Snapshot> for En1998Artifact {
    fn from(snapshot: En1998Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1998Artifact {
    pub fn to_snapshot(&self) -> En1998Snapshot {
        En1998Snapshot {
            seismic_zone: self.seismic_zone.clone(),
            ground_type: self.ground_type.clone(),
            importance_class: self.importance_class.clone(),
            structural_system: self.structural_system.clone(),
            t1_s: self.t1_s.clone(),
            mass_t: self.mass_t.clone(),
            v_rd_kn: self.v_rd_kn.clone(),
            drift_mm: self.drift_mm.clone(),
            height_m: self.height_m.clone(),
            multiple_resisting_systems: self.multiple_resisting_systems.clone(),
            annex: self.annex.clone(),
            en_a_gr: self.en_a_gr.clone(),
            en_ground_type: self.en_ground_type.clone(),
            en_spectrum_type: self.en_spectrum_type.clone(),
            period_ratio: self.period_ratio.clone(),
            bridge_v_rd_kn: self.bridge_v_rd_kn.clone(),
            bearing_d_ed_mm: self.bearing_d_ed_mm.clone(),
            bearing_d_rd_mm: self.bearing_d_rd_mm.clone(),
            retrofit_knowledge_level: self.retrofit_knowledge_level.clone(),
            retrofit_limit_state: self.retrofit_limit_state.clone(),
            retrofit_e_d_kn: self.retrofit_e_d_kn.clone(),
            retrofit_r_k_kn: self.retrofit_r_k_kn.clone(),
            retrofit_gamma_el: self.retrofit_gamma_el.clone(),
            silo_height_m: self.silo_height_m.clone(),
            silo_radius_m: self.silo_radius_m.clone(),
            silo_n_rd_kn: self.silo_n_rd_kn.clone(),
            silo_v_ed_kn: self.silo_v_ed_kn.clone(),
            silo_v_rd_kn: self.silo_v_rd_kn.clone(),
            silo_q_nominal: self.silo_q_nominal.clone(),
            tank_height_m: self.tank_height_m.clone(),
            tank_radius_m: self.tank_radius_m.clone(),
            tank_mass_t: self.tank_mass_t.clone(),
            tank_v_rd_kn: self.tank_v_rd_kn.clone(),
            tower_m_ed_knm: self.tower_m_ed_knm.clone(),
            tower_m_rd_knm: self.tower_m_rd_knm.clone(),
            tower_is_chimney: self.tower_is_chimney.clone(),
            tower_q_nominal: self.tower_q_nominal.clone(),
            tower_mass_t: self.tower_mass_t.clone(),
            foundation_area_m2: self.foundation_area_m2.clone(),
            foundation_p_rd_kpa: self.foundation_p_rd_kpa.clone(),
            foundation_h_ed_kn: self.foundation_h_ed_kn.clone(),
            foundation_h_rd_kn: self.foundation_h_rd_kn.clone(),
            k_foundation: self.k_foundation.clone(),
            k_soil: self.k_soil.clone(),
            wall_height_m: self.wall_height_m.clone(),
            wall_phi_deg: self.wall_phi_deg.clone(),
            wall_soil_gamma_kn_m3: self.wall_soil_gamma_kn_m3.clone(),
            wall_r: self.wall_r.clone(),
            wall_h_rd_kn: self.wall_h_rd_kn.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1998Snapshot) -> Self {
        Self {
            seismic_zone: snapshot.seismic_zone,
            ground_type: snapshot.ground_type,
            importance_class: snapshot.importance_class,
            structural_system: snapshot.structural_system,
            t1_s: snapshot.t1_s,
            mass_t: snapshot.mass_t,
            v_rd_kn: snapshot.v_rd_kn,
            drift_mm: snapshot.drift_mm,
            height_m: snapshot.height_m,
            multiple_resisting_systems: snapshot.multiple_resisting_systems,
            annex: snapshot.annex,
            en_a_gr: snapshot.en_a_gr,
            en_ground_type: snapshot.en_ground_type,
            en_spectrum_type: snapshot.en_spectrum_type,
            period_ratio: snapshot.period_ratio,
            bridge_v_rd_kn: snapshot.bridge_v_rd_kn,
            bearing_d_ed_mm: snapshot.bearing_d_ed_mm,
            bearing_d_rd_mm: snapshot.bearing_d_rd_mm,
            retrofit_knowledge_level: snapshot.retrofit_knowledge_level,
            retrofit_limit_state: snapshot.retrofit_limit_state,
            retrofit_e_d_kn: snapshot.retrofit_e_d_kn,
            retrofit_r_k_kn: snapshot.retrofit_r_k_kn,
            retrofit_gamma_el: snapshot.retrofit_gamma_el,
            silo_height_m: snapshot.silo_height_m,
            silo_radius_m: snapshot.silo_radius_m,
            silo_n_rd_kn: snapshot.silo_n_rd_kn,
            silo_v_ed_kn: snapshot.silo_v_ed_kn,
            silo_v_rd_kn: snapshot.silo_v_rd_kn,
            silo_q_nominal: snapshot.silo_q_nominal,
            tank_height_m: snapshot.tank_height_m,
            tank_radius_m: snapshot.tank_radius_m,
            tank_mass_t: snapshot.tank_mass_t,
            tank_v_rd_kn: snapshot.tank_v_rd_kn,
            tower_m_ed_knm: snapshot.tower_m_ed_knm,
            tower_m_rd_knm: snapshot.tower_m_rd_knm,
            tower_is_chimney: snapshot.tower_is_chimney,
            tower_q_nominal: snapshot.tower_q_nominal,
            tower_mass_t: snapshot.tower_mass_t,
            foundation_area_m2: snapshot.foundation_area_m2,
            foundation_p_rd_kpa: snapshot.foundation_p_rd_kpa,
            foundation_h_ed_kn: snapshot.foundation_h_ed_kn,
            foundation_h_rd_kn: snapshot.foundation_h_rd_kn,
            k_foundation: snapshot.k_foundation,
            k_soil: snapshot.k_soil,
            wall_height_m: snapshot.wall_height_m,
            wall_phi_deg: snapshot.wall_phi_deg,
            wall_soil_gamma_kn_m3: snapshot.wall_soil_gamma_kn_m3,
            wall_r: snapshot.wall_r,
            wall_h_rd_kn: snapshot.wall_h_rd_kn,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: En1998Snapshot) {
        self.seismic_zone = snapshot.seismic_zone;
        self.ground_type = snapshot.ground_type;
        self.importance_class = snapshot.importance_class;
        self.structural_system = snapshot.structural_system;
        self.t1_s = snapshot.t1_s;
        self.mass_t = snapshot.mass_t;
        self.v_rd_kn = snapshot.v_rd_kn;
        self.drift_mm = snapshot.drift_mm;
        self.height_m = snapshot.height_m;
        self.multiple_resisting_systems = snapshot.multiple_resisting_systems;
        self.annex = snapshot.annex;
        self.en_a_gr = snapshot.en_a_gr;
        self.en_ground_type = snapshot.en_ground_type;
        self.en_spectrum_type = snapshot.en_spectrum_type;
        self.period_ratio = snapshot.period_ratio;
        self.bridge_v_rd_kn = snapshot.bridge_v_rd_kn;
        self.bearing_d_ed_mm = snapshot.bearing_d_ed_mm;
        self.bearing_d_rd_mm = snapshot.bearing_d_rd_mm;
        self.retrofit_knowledge_level = snapshot.retrofit_knowledge_level;
        self.retrofit_limit_state = snapshot.retrofit_limit_state;
        self.retrofit_e_d_kn = snapshot.retrofit_e_d_kn;
        self.retrofit_r_k_kn = snapshot.retrofit_r_k_kn;
        self.retrofit_gamma_el = snapshot.retrofit_gamma_el;
        self.silo_height_m = snapshot.silo_height_m;
        self.silo_radius_m = snapshot.silo_radius_m;
        self.silo_n_rd_kn = snapshot.silo_n_rd_kn;
        self.silo_v_ed_kn = snapshot.silo_v_ed_kn;
        self.silo_v_rd_kn = snapshot.silo_v_rd_kn;
        self.silo_q_nominal = snapshot.silo_q_nominal;
        self.tank_height_m = snapshot.tank_height_m;
        self.tank_radius_m = snapshot.tank_radius_m;
        self.tank_mass_t = snapshot.tank_mass_t;
        self.tank_v_rd_kn = snapshot.tank_v_rd_kn;
        self.tower_m_ed_knm = snapshot.tower_m_ed_knm;
        self.tower_m_rd_knm = snapshot.tower_m_rd_knm;
        self.tower_is_chimney = snapshot.tower_is_chimney;
        self.tower_q_nominal = snapshot.tower_q_nominal;
        self.tower_mass_t = snapshot.tower_mass_t;
        self.foundation_area_m2 = snapshot.foundation_area_m2;
        self.foundation_p_rd_kpa = snapshot.foundation_p_rd_kpa;
        self.foundation_h_ed_kn = snapshot.foundation_h_ed_kn;
        self.foundation_h_rd_kn = snapshot.foundation_h_rd_kn;
        self.k_foundation = snapshot.k_foundation;
        self.k_soil = snapshot.k_soil;
        self.wall_height_m = snapshot.wall_height_m;
        self.wall_phi_deg = snapshot.wall_phi_deg;
        self.wall_soil_gamma_kn_m3 = snapshot.wall_soil_gamma_kn_m3;
        self.wall_r = snapshot.wall_r;
        self.wall_h_rd_kn = snapshot.wall_h_rd_kn;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1998_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1998",
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
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::en1998::{En1998Diff, En1998Mutation, En1998Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1998BuilderConstruction {
        snapshot: En1998Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1998BuilderConstruction {
        type Snapshot = En1998Snapshot;
        type Mutation = En1998Mutation;
        type Diff = En1998Diff;
        fn empty() -> Self {
            Self { snapshot: En1998Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1998Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1998Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::en1998::En1998Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1998Parts {
        pub snapshot: Option<En1998Snapshot>,
    }

    pub struct En1998AnalyzerAnalysis;

    impl ArtifactAnalysis for En1998AnalyzerAnalysis {
        type Parts = En1998Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1998", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1998Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1998Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1998Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec En1998BuilderFacets {
        construction: En1998BuilderConstruction,
        analysis: En1998AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1998ComposerComposition,
    }
    builder: En1998Builder,
    analyzer: En1998Analyzer,
    composer: En1998Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1998 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. `na_de`, `part_1` through `part_6`, `AnnexParams`,
/// `check_building_seismic_with_annex` and `check_building_seismic` are pure function libraries; the
/// snapshot-level composition (`evaluate`, `check_full_seismic`) lives in `💡️inferences`. `na_de`
/// re-exports `crate::artifacts::en1990`'s relocated `NaDe`.
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

// #region 🔖️NaDe
pub mod na_de {
    pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::na_de::NaDe;

    /// 🌋️ German seismic zone per DIN EN 1998-1/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SeismicZone {
        Zone0,
        Zone1,
        Zone2,
        Zone3,
    }

    impl SeismicZone {
        pub fn as_u8(self) -> u8 {
            match self {
                Self::Zone0 => 0,
                Self::Zone1 => 1,
                Self::Zone2 => 2,
                Self::Zone3 => 3,
            }
        }

        pub fn a_g(self) -> f64 {
            match self {
                Self::Zone0 => 0.0,
                Self::Zone1 => 0.08,
                Self::Zone2 => 0.15,
                Self::Zone3 => 0.24,
            }
        }
    }

    /// 🪨️ Ground type per EN 1998-1 Table 3.1 (DE NA).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GroundType {
        A,
        B,
        C,
        D,
        E,
    }

    impl GroundType {
        pub fn spectrum_params(self) -> (f64, f64, f64, f64) {
            match self {
                Self::A => (0.05, 0.25, 0.8, 1.0),
                Self::B => (0.15, 0.4, 2.0, 1.0),
                Self::C => (0.20, 0.6, 2.0, 1.15),
                Self::D => (0.25, 0.8, 2.0, 1.35),
                Self::E => (0.35, 1.2, 2.0, 1.4),
            }
        }
    }

    pub fn peak_ground_acceleration(zone: SeismicZone) -> f64 {
        zone.a_g()
    }
}
// #endregion 🔖️NaDe

// #region 🔖️Part1
pub mod part_1 {
    use super::*;

    /// 🏗️ Structural system behaviour factor q per EN 1998-1 Table 6.1.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum StructuralSystem {
        MomentFrameDch,
        MomentFrameDcm,
        MomentFrameDcl,
        ShearWall,
        BracedFrame,
        InvertedPendulum,
        DualSystem,
    }

    impl StructuralSystem {
        pub fn q(self) -> f64 {
            match self {
                Self::MomentFrameDch => 4.0,
                Self::MomentFrameDcm => 3.3,
                Self::MomentFrameDcl => 2.0,
                Self::ShearWall => 3.0,
                Self::BracedFrame => 2.5,
                Self::InvertedPendulum => 1.5,
                Self::DualSystem => 4.0,
            }
        }
    }

    /// 📊️ Importance factor γ_I per EN 1998-1 Table 4.3.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImportanceClass {
        Cc1,
        Cc2,
        Cc3,
        Cc4,
    }

    impl ImportanceClass {
        pub fn gamma_i(self) -> f64 {
            match self {
                Self::Cc1 => 0.8,
                Self::Cc2 => 1.0,
                Self::Cc3 => 1.2,
                Self::Cc4 => 1.4,
            }
        }
    }

    /// 🌍️ EN 1998-1 elastic response spectrum shape per §3.2.2.2: Type 1 (M_s ≥ 5.5) vs Type 2 (M_s < 5.5).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SpectrumType {
        Type1,
        Type2,
    }

    /// 🪨️ Generic EN 1998-1 ground type per Table 3.1, independent of any national annex table.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EnGroundType {
        A,
        B,
        C,
        D,
        E,
    }

    impl EnGroundType {
        /// 📊️ (S, T_B, T_C, T_D) per EN 1998-1 Table 3.2 (Type 1) / Table 3.3 (Type 2).
        pub fn spectrum_params(self, spectrum: SpectrumType) -> (f64, f64, f64, f64) {
            match spectrum {
                SpectrumType::Type1 => match self {
                    Self::A => (1.0, 0.15, 0.4, 2.0),
                    Self::B => (1.2, 0.15, 0.5, 2.0),
                    Self::C => (1.15, 0.20, 0.6, 2.0),
                    Self::D => (1.35, 0.20, 0.8, 2.0),
                    Self::E => (1.4, 0.15, 0.5, 2.0),
                },
                SpectrumType::Type2 => match self {
                    Self::A => (1.0, 0.05, 0.25, 1.2),
                    Self::B => (1.35, 0.05, 0.25, 1.2),
                    Self::C => (1.5, 0.10, 0.25, 1.2),
                    Self::D => (1.8, 0.10, 0.30, 1.2),
                    Self::E => (1.6, 0.05, 0.25, 1.2),
                },
            }
        }
    }

    /// 📈️ Elastic response spectrum Type 1/2 shape horizontal [g] per EN 1998-1 §3.2.2.2, given resolved (a_g, S, T_B, T_C, T_D).
    pub fn elastic_response_spectrum_type1(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, t: f64) -> f64 {
        let eta = 1.0;
        if t <= tb {
            a_g * s * (1.0 + t / tb * (2.5 * eta - 1.0))
        } else if t <= tc {
            a_g * s * 2.5 * eta
        } else if t <= td {
            a_g * s * 2.5 * eta * tc / t
        } else {
            a_g * s * 2.5 * eta * tc * td / (t * t)
        }
    }

    /// 📉️ Design spectrum Sd(T) = S_e(T) · γ_I / q [g].
    pub fn design_spectrum_sd(s_e: f64, gamma_i: f64, q: f64) -> f64 {
        s_e * gamma_i / q
    }

    /// 🌊️ Base shear V_b = S_e(T1) · m · γ_I / q [kN] with mass in tonnes.
    pub fn base_shear_kn(s_e: f64, mass_t: f64, gamma_i: f64, q: f64) -> f64 {
        s_e * mass_t * 9.81 * gamma_i / q
    }

    /// 🌊️ Base shear from design spectrum S_d(T1) [kN].
    pub fn base_shear_from_design_kn(s_d: f64, mass_t: f64) -> f64 {
        s_d * mass_t * 9.81
    }

    /// 🔁️ Redundancy factor ρ per EN 1998-1 §4.2.5.
    pub fn redundancy_factor(multiple_resisting_systems: bool) -> f64 {
        if multiple_resisting_systems {
            1.0
        } else {
            1.3
        }
    }

    /// 📐️ Interstorey drift limit with ρ per EN 1998-1 §4.3.3.4 [mm].
    pub fn drift_limit_mm(height_m: f64, rho: f64, ductility: DuctilityClass, nu: f64) -> f64 {
        let theta = match ductility {
            DuctilityClass::Dch => 0.01,
            DuctilityClass::Dcm => 0.007,
            DuctilityClass::Dcl => 0.005,
        };
        nu * rho * theta * height_m * 1000.0
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DuctilityClass {
        Dch,
        Dcm,
        Dcl,
    }

    pub fn check_drift(drift_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-1", "§4.3", "4.3.3"), Quantity::length_m(drift_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "interstorey drift SLS", annex)
    }

    pub fn check_base_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-1", "§4.3", "4.3.4"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "seismic base shear ULS", annex)
    }
}
// #endregion 🔖️Part1

// #region 🔖️AnnexParams
/// 🇪️🇺️ Resolved seismic-action model per NDP: DE zone table vs EN Type-1/2 spectrum (EN 1998-1 §3.2 NDP).
#[derive(Clone, Debug, PartialEq)]
pub enum AnnexParams {
    De { zone: na_de::SeismicZone, ground: na_de::GroundType },
    En { a_gr: f64, ground: part_1::EnGroundType, spectrum: part_1::SpectrumType },
}

impl AnnexParams {
    pub fn choice(&self) -> AnnexChoice {
        match self {
            Self::De { .. } => AnnexChoice::De,
            Self::En { .. } => AnnexChoice::En,
        }
    }

    /// 📐️ Resolved (a_g, S, T_B, T_C, T_D) feeding `part_1::elastic_response_spectrum_type1`.
    pub fn ground_params(&self) -> (f64, f64, f64, f64, f64) {
        match self {
            Self::De { zone, ground } => {
                let (tb, tc, td, s) = ground.spectrum_params();
                (zone.a_g(), s, tb, tc, td)
            }
            Self::En { a_gr, ground, spectrum } => {
                let (s, tb, tc, td) = ground.spectrum_params(*spectrum);
                (*a_gr, s, tb, tc, td)
            }
        }
    }

    /// 📈️ Elastic response spectrum S_e(T) [g] resolved for this annex selection.
    pub fn elastic_response_spectrum(&self, t: f64) -> f64 {
        let (a_g, s, tb, tc, td) = self.ground_params();
        part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t)
    }
}
// #endregion 🔖️AnnexParams

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    /// 🌉️ Isolated bridge design spectrum reduction factor q_isol.
    pub fn isolation_reduction_factor(period_ratio: f64) -> f64 {
        (period_ratio * period_ratio).max(1.0)
    }

    /// 🌉️ Design spectrum for isolated bridge deck [g].
    pub fn isolated_spectrum_sd(s_e: f64, gamma_i: f64, q_isol: f64) -> f64 {
        s_e * gamma_i / q_isol
    }

    /// 🌉️ Bearing displacement check limit [mm].
    pub fn bearing_displacement_limit_mm(d_max_mm: f64) -> f64 {
        d_max_mm
    }

    pub fn check_bridge_seismic(v_ed: f64, v_rd: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-2", "§5", "5.3"), Quantity::force_kn(v_ed), Quantity::force_kn(v_rd), "bridge seismic shear", AnnexChoice::En)
    }

    pub fn check_isolation_bearing(d_ed_mm: f64, d_rd_mm: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-2", "§7", "7.5"), Quantity::length_m(d_ed_mm / 1000.0), Quantity::length_m(d_rd_mm / 1000.0), "isolation bearing displacement", AnnexChoice::En)
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    /// 🔍️ Knowledge level per EN 1998-3 §3.4, driving the confidence factor CF.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum KnowledgeLevel {
        Kl1,
        Kl2,
        Kl3,
    }

    impl KnowledgeLevel {
        /// 🎯️ Confidence factor CF per EN 1998-3 Table 3.1.
        pub fn confidence_factor(self) -> f64 {
            match self {
                Self::Kl1 => 1.35,
                Self::Kl2 => 1.20,
                Self::Kl3 => 1.00,
            }
        }
    }

    /// ⚖️ Limit state for existing-building assessment per EN 1998-3 §2.1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RetrofitLimitState {
        DamageLimitation,
        SignificantDamage,
        NearCollapse,
    }

    impl RetrofitLimitState {
        /// 📑️ EN 1998-3 §2.3.4 clause subsection per limit state.
        pub fn clause_section(self) -> &'static str {
            match self {
                Self::DamageLimitation => "2.3.4.1",
                Self::SignificantDamage => "2.3.4.2",
                Self::NearCollapse => "2.3.4.3",
            }
        }
    }

    /// 🏚️ Design capacity R_d = R_k / (CF · γ_el) per EN 1998-3 §2.3.3.
    pub fn design_capacity_kn(r_k_kn: f64, cf: f64, gamma_el: f64) -> f64 {
        r_k_kn / (cf * gamma_el)
    }

    /// 🏚️ Existing-element seismic capacity check E_d ≤ R_k / (CF · γ_el) per EN 1998-3 §2.3.3.
    pub fn check_element_capacity(e_d_kn: f64, r_k_kn: f64, cf: f64, gamma_el: f64, limit_state: RetrofitLimitState, annex: AnnexChoice) -> CheckResult {
        let r_d = design_capacity_kn(r_k_kn, cf, gamma_el);
        CheckResult::from_utilization(ClauseId::new("EN 1998-3", "§2.3", limit_state.clause_section()), Quantity::force_kn(e_d_kn), Quantity::force_kn(r_d), "existing element seismic capacity", annex)
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
pub mod part_4 {
    use super::*;

    /// 🏺️ Impulsive period T_i [s] for circular silo/tank per EN 1998-4 Annex.
    pub fn impulsive_period_s(height_m: f64, radius_m: f64) -> f64 {
        0.1 * (height_m / radius_m).sqrt()
    }

    /// 🏺️ Convective (sloshing) period T_c [s] for circular silo/tank.
    pub fn convective_period_s(radius_m: f64) -> f64 {
        2.0 * (radius_m / 9.81).sqrt()
    }

    /// 🏺️ Impulsive mass ratio μ_i.
    pub fn impulsive_mass_ratio(h_over_r: f64) -> f64 {
        (0.45 * h_over_r / (1.0 + 0.75 * h_over_r)).clamp(0.1, 0.85)
    }

    /// 🏺️ Convective mass ratio μ_c.
    pub fn convective_mass_ratio(h_over_r: f64) -> f64 {
        (0.55 / (1.0 + 0.75 * h_over_r)).clamp(0.05, 0.75)
    }

    /// 🏺️ Combined silo base shear via SRSS of impulsive and convective components [kN].
    pub fn silo_base_shear_kn(v_i_kn: f64, v_c_kn: f64) -> f64 {
        (v_i_kn * v_i_kn + v_c_kn * v_c_kn).sqrt()
    }

    /// 🛢️ Tank base shear V = m_i·S_e(T_i) + m_c·S_e(T_c) [kN] per EN 1998-4 §4 simplified model.
    pub fn tank_base_shear_kn(m_i_t: f64, s_e_i: f64, m_c_t: f64, s_e_c: f64) -> f64 {
        (m_i_t * s_e_i + m_c_t * s_e_c) * 9.81
    }

    /// 🏺️ Behaviour factor q capped at 1.5 for silos per EN 1998-4 Table 2.1.
    pub fn silo_behaviour_factor(q_nominal: f64) -> f64 {
        q_nominal.min(1.5)
    }

    pub fn check_silo_wall(n_ed_kn: f64, n_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-4", "§3", "3.4"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_rd_kn), "silo wall seismic", AnnexChoice::En)
    }

    pub fn check_silo_anchor(v_ed_kn: f64, v_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-4", "§3", "3.5"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "silo anchorage", AnnexChoice::En)
    }

    pub fn check_tank_base_shear(v_ed_kn: f64, v_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-4", "§4", "4.3"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "tank hydrodynamic base shear", AnnexChoice::En)
    }
}
// #endregion 🔖️Part4

// #region 🔖️Part5
pub mod part_5 {
    use super::*;

    /// 🧱️ Foundation stiffness ratio r = K_f / K_s per EN 1998-5 §7.
    pub fn stiffness_ratio(k_foundation: f64, k_soil: f64) -> f64 {
        k_foundation / k_soil
    }

    /// 🧱️ Radiation damping ratio ξ for shallow foundation.
    pub fn radiation_damping(ratio: f64) -> f64 {
        (0.05 + 0.1 * ratio / (1.0 + ratio)).clamp(0.05, 0.20)
    }

    /// 🧱️ Bearing capacity reduction factor under seismic loading per EN 1998-5 §7.
    pub fn bearing_reduction_factor(a_g: f64) -> f64 {
        (1.0 - 1.5 * a_g).max(0.5)
    }

    /// 🧱️ Seismic bearing pressure [kPa].
    pub fn seismic_bearing_pressure_kpa(v_seismic_kn: f64, area_m2: f64) -> f64 {
        v_seismic_kn / area_m2
    }

    pub fn check_foundation_bearing(p_ed_kpa: f64, p_rd_kpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1998-5", "§7", "7.3"),
            Quantity::new(crate::document::QuantityKind::Pressure, p_ed_kpa * 1000.0),
            Quantity::new(crate::document::QuantityKind::Pressure, p_rd_kpa * 1000.0),
            "foundation seismic bearing",
            AnnexChoice::De,
        )
    }

    pub fn check_foundation_sliding(h_ed_kn: f64, h_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-5", "§7", "7.4"), Quantity::force_kn(h_ed_kn), Quantity::force_kn(h_rd_kn), "foundation seismic sliding", AnnexChoice::De)
    }

    /// 🌍️ Horizontal seismic coefficient k_h = α·S/r per EN 1998-5 §7.3.2.2 (r: wall-displacement class).
    pub fn horizontal_seismic_coefficient(alpha: f64, s: f64, r: f64) -> f64 {
        alpha * s / r
    }

    /// 🧱️ Mononobe-Okabe dynamic active earth-pressure coefficient K_AE per EN 1998-5 Annex E (vertical wall, horizontal backfill, no wall friction). Reduces to the classic Rankine K_a at k_h = 0.
    pub fn mononobe_okabe_k_ae(phi_deg: f64, k_h: f64) -> f64 {
        let phi = phi_deg.to_radians();
        let theta = k_h.atan();
        let bracket = 1.0 + ((phi.sin() * (phi - theta).sin()) / theta.cos()).sqrt();
        (phi - theta).cos().powi(2) / (theta.cos() * bracket * bracket)
    }

    /// 🧱️ Dynamic active thrust increment on a retaining wall [kN/m] from K_AE.
    pub fn retaining_wall_thrust_kn_m(gamma_soil_kn_m3: f64, height_m: f64, k_ae: f64) -> f64 {
        0.5 * gamma_soil_kn_m3 * height_m * height_m * k_ae
    }

    pub fn check_retaining_wall_sliding(h_ed_kn_m: f64, h_rd_kn_m: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-5", "§6", "E.2"), Quantity::force_kn(h_ed_kn_m), Quantity::force_kn(h_rd_kn_m), "retaining wall seismic thrust", AnnexChoice::En)
    }
}
// #endregion 🔖️Part5

// #region 🔖️Part6
pub mod part_6 {
    use super::*;

    /// 🗼️ Along-wind base overturning moment [kNm] per EN 1998-6 §4 (wind-induced dynamic response of slender towers).
    pub fn along_wind_overturning_knm(rho_air: f64, v_crit_m_s: f64, height_m: f64, diameter_m: f64, c_d: f64) -> f64 {
        let q_z = 0.5 * rho_air * v_crit_m_s * v_crit_m_s / 1000.0;
        q_z * c_d * diameter_m * height_m * height_m / 2.0
    }

    /// 🗼️ Critical wind speed for vortex shedding [m/s].
    pub fn critical_wind_speed_m_s(strouhal: f64, frequency_hz: f64, diameter_m: f64) -> f64 {
        strouhal * frequency_hz * diameter_m
    }

    /// 🗼️ First-mode natural frequency [Hz] for a cantilever tower.
    pub fn tower_frequency_hz(e_i_pa: f64, i_m4: f64, mass_kg_m: f64, height_m: f64) -> f64 {
        let lambda = 1.875;
        let omega = lambda * lambda * (e_i_pa * i_m4 / (mass_kg_m * height_m.powi(4))).sqrt();
        omega / (2.0 * std::f64::consts::PI)
    }

    /// 🗼️ Behaviour factor q capped per EN 1998-6 Table 4.1: 1.5 for chimneys, 2.0 for other towers/masts.
    pub fn tower_behaviour_factor(q_nominal: f64, is_chimney: bool) -> f64 {
        let cap = if is_chimney { 1.5 } else { 2.0 };
        q_nominal.min(cap)
    }

    /// 🗼️ First-mode participation factor Γ for a uniform cantilever with mode shape φ(x) = 1 − cos(πx/2H) per EN 1998-6 Annex B (simplified modal analysis).
    pub fn cantilever_modal_participation_factor() -> f64 {
        let numerator = 1.0 - 2.0 / std::f64::consts::PI;
        let denominator = 1.5 - 4.0 / std::f64::consts::PI;
        numerator / denominator
    }

    /// 🗼️ Modal base shear V_b1 = Γ · S_d(T1) · m · g [kN] for the cantilever first mode.
    pub fn tower_base_shear_kn(gamma: f64, s_d: f64, mass_t: f64) -> f64 {
        gamma * s_d * mass_t * 9.81
    }

    pub fn check_tower_overturning(m_ed_knm: f64, m_rd_knm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1998-6", "§4", "4.3.2"),
            Quantity::new(crate::document::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(crate::document::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "tower overturning",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖️Part6

/// 📋️ Building seismic check generalized over DE zone-based or EN Type-1/2-spectrum annex selection.
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_building_seismic_with_annex(
    annex: &AnnexParams,
    importance: part_1::ImportanceClass,
    system: part_1::StructuralSystem,
    t1_s: f64,
    mass_t: f64,
    v_rd_kn: f64,
    drift_mm: f64,
    height_m: f64,
    multiple_resisting_systems: bool,
) -> CheckReport {
    let choice = annex.choice();
    let gamma_i = importance.gamma_i();
    let q = system.q();
    let s_e = annex.elastic_response_spectrum(t1_s);
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
    let v_b = part_1::base_shear_from_design_kn(s_d, mass_t);
    let rho = part_1::redundancy_factor(multiple_resisting_systems);
    let drift_limit = part_1::drift_limit_mm(height_m, rho, part_1::DuctilityClass::Dcm, 1.0);
    let mut report = CheckReport::default();
    report.push(part_1::check_base_shear(v_b, v_rd_kn, choice));
    report.push(part_1::check_drift(drift_mm, drift_limit, choice));
    report
}

/// 📋️ Building seismic check (DE NA zone parameters).
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_building_seismic(
    zone: na_de::SeismicZone,
    ground: na_de::GroundType,
    importance: part_1::ImportanceClass,
    system: part_1::StructuralSystem,
    t1_s: f64,
    mass_t: f64,
    v_rd_kn: f64,
    drift_mm: f64,
    height_m: f64,
    multiple_resisting_systems: bool,
) -> CheckReport {
    check_building_seismic_with_annex(&AnnexParams::De { zone, ground }, importance, system, t1_s, mass_t, v_rd_kn, drift_mm, height_m, multiple_resisting_systems)
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn zone2_spectrum_sd_at_t1() {
        let a_g = na_de::SeismicZone::Zone2.a_g();
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
        assert!((s_e - 0.375).abs() < 1e-9);
        let sd = part_1::design_spectrum_sd(s_e, 1.0, 1.0);
        assert!((sd - 0.375).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn zone2_spectrum_sd_at_half_second() {
        let a_g = na_de::SeismicZone::Zone2.a_g();
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.5);
        assert!((s_e - 0.3).abs() < 1e-9);
        let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
        let q = part_1::StructuralSystem::MomentFrameDch.q();
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
        assert!((s_d - 0.075).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn base_shear_uses_design_spectrum() {
        let a_g = 0.15;
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
        let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
        let q = part_1::StructuralSystem::MomentFrameDch.q();
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
        let v_b = part_1::base_shear_kn(s_e, 100.0, gamma_i, q);
        assert!(v_b > 0.0);
        let expected = s_e * 100.0 * 9.81 * gamma_i / q;
        assert!((v_b - expected).abs() < 1e-6);
        assert!((part_1::base_shear_from_design_kn(s_d, 100.0) - v_b).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn drift_rho_limit() {
        let rho = part_1::redundancy_factor(false);
        assert!((rho - 1.3).abs() < 1e-9);
        let limit = part_1::drift_limit_mm(12.0, rho, part_1::DuctilityClass::Dcm, 1.0);
        assert!((limit - 109.2).abs() < 0.1);
    }

    #[semio_framework_async_macros::async_test]
    fn building_seismic_base_shear_uses_sd() {
        let a_g = na_de::SeismicZone::Zone2.a_g();
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
        let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
        let q = part_1::StructuralSystem::MomentFrameDch.q();
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
        let expected_v_b = part_1::base_shear_from_design_kn(s_d, 500.0);

        let report = check_building_seismic(na_de::SeismicZone::Zone2, na_de::GroundType::B, part_1::ImportanceClass::Cc2, part_1::StructuralSystem::MomentFrameDch, 0.3, 500.0, 800.0, 20.0, 12.0, true);
        assert_eq!(report.checks.len(), 2);
        assert!((report.checks[0].computed.value - expected_v_b * 1000.0).abs() < 1e-3);
    }

    #[semio_framework_async_macros::async_test]
    fn en_type1_vs_de_zone_divergence_same_nominal_ag() {
        let a_g = 0.15;
        let annex_de = AnnexParams::De { zone: na_de::SeismicZone::Zone2, ground: na_de::GroundType::B };
        let annex_en = AnnexParams::En { a_gr: a_g, ground: part_1::EnGroundType::B, spectrum: part_1::SpectrumType::Type1 };
        let s_e_de = annex_de.elastic_response_spectrum(0.3);
        let s_e_en = annex_en.elastic_response_spectrum(0.3);
        assert!((s_e_de - 0.375).abs() < 1e-9);
        assert!((s_e_en - 0.45).abs() < 1e-9);
        assert!((s_e_en - s_e_de).abs() > 0.05);
    }

    #[semio_framework_async_macros::async_test]
    fn building_seismic_e2e() {
        let report = check_building_seismic(na_de::SeismicZone::Zone2, na_de::GroundType::B, part_1::ImportanceClass::Cc2, part_1::StructuralSystem::MomentFrameDch, 0.3, 500.0, 800.0, 20.0, 12.0, true);
        assert_eq!(report.checks.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    fn silo_impulsive_convective() {
        let h = 10.0;
        let r = 5.0;
        let t_i = part_4::impulsive_period_s(h, r);
        let t_c = part_4::convective_period_s(r);
        assert!(t_i < t_c);
        let v = part_4::silo_base_shear_kn(200.0, 150.0);
        assert!((v - 250.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn silo_behaviour_factor_capped() {
        assert!((part_4::silo_behaviour_factor(2.0) - 1.5).abs() < 1e-9);
        assert!((part_4::silo_behaviour_factor(1.0) - 1.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn tank_base_shear_combines_impulsive_and_convective() {
        let a_g = 0.15;
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let t_i = part_4::impulsive_period_s(8.0, 4.0);
        let t_c = part_4::convective_period_s(4.0);
        let s_e_i = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_i);
        let s_e_c = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_c);
        let v_tank = part_4::tank_base_shear_kn(100.0, s_e_i, 50.0, s_e_c);
        assert!((v_tank - 412.8624420576831).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn bridge_isolation_distinct() {
        let q_isol = part_2::isolation_reduction_factor(2.0);
        assert!((q_isol - 4.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn retrofit_confidence_factor_scales_capacity_exactly() {
        let r_k = 400.0;
        let r_d_kl3 = part_3::design_capacity_kn(r_k, part_3::KnowledgeLevel::Kl3.confidence_factor(), 1.0);
        let r_d_kl1 = part_3::design_capacity_kn(r_k, part_3::KnowledgeLevel::Kl1.confidence_factor(), 1.0);
        assert!((r_d_kl3 / r_d_kl1 - 1.35).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn mononobe_okabe_k_ae_matches_hand_calc_and_reduces_to_rankine() {
        let k_ae = part_5::mononobe_okabe_k_ae(30.0, 0.2);
        assert!((k_ae - 0.46407409106465564).abs() < 1e-9);
        let k_a_static = part_5::mononobe_okabe_k_ae(30.0, 0.0);
        let rankine_ka = (1.0 - 30.0_f64.to_radians().sin()) / (1.0 + 30.0_f64.to_radians().sin());
        assert!((k_a_static - rankine_ka).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn retaining_wall_thrust_from_k_ae() {
        let k_h = part_5::horizontal_seismic_coefficient(0.15, 1.0, 1.5);
        assert!((k_h - 0.1).abs() < 1e-9);
        let k_ae = part_5::mononobe_okabe_k_ae(30.0, 0.2);
        let thrust = part_5::retaining_wall_thrust_kn_m(18.0, 4.0, k_ae);
        assert!((thrust - 66.82666911331042).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn cantilever_modal_participation_factor_matches_closed_form() {
        let gamma = part_6::cantilever_modal_participation_factor();
        assert!((gamma - 1.602484997695127).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn tower_behaviour_factor_capped_by_type() {
        assert!((part_6::tower_behaviour_factor(3.0, true) - 1.5).abs() < 1e-9);
        assert!((part_6::tower_behaviour_factor(3.0, false) - 2.0).abs() < 1e-9);
    }
}
//#endregion 🧪️ComplianceHelpersTests
