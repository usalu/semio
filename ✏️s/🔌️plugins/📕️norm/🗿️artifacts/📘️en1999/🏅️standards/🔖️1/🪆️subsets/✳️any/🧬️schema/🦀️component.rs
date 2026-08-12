//! ✨️ EN 1999 artifact schema — every field with its state class.

use crate::document::AnnexChoice;
use crate::artifacts::en1999::En1999Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full EN 1999 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Artifact {
    #[state(persistent)] pub n_ed_kn: f64,
    #[state(persistent)] pub m_ed_knm: f64,
    #[state(persistent)] pub a_mm2: f64,
    #[state(persistent)] pub w_el_mm3: f64,
    #[state(persistent)] pub alloy: String,
    #[state(persistent)] pub chi: f64,
    #[state(persistent)] pub i_t_mm4: f64,
    #[state(persistent)] pub l_cr_mm: f64,
    #[state(persistent)] pub theta_c: f64,
    #[state(persistent)] pub delta_sigma_ed: f64,
    #[state(persistent)] pub delta_sigma_c: f64,
    #[state(persistent)] pub fatigue_m: f64,
    #[state(persistent)] pub n_cycles: f64,
    #[state(persistent)] pub v_weld_ed_kn: f64,
    #[state(persistent)] pub weld_throat_mm: f64,
    #[state(persistent)] pub weld_length_mm: f64,
    #[state(persistent)] pub beta_w: f64,
    #[state(persistent)] pub sheet_b_mm: f64,
    #[state(persistent)] pub sheet_t_mm: f64,
    #[state(persistent)] pub sheet_k_sigma: f64,
    #[state(persistent)] pub sheet_w_el_mm3: f64,
    #[state(persistent)] pub sheet_m_ed_knm: f64,
    #[state(persistent)] pub shell_t_mm: f64,
    #[state(persistent)] pub shell_r_mm: f64,
    #[state(persistent)] pub sigma_ed_shell_mpa: f64,
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1999Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1999Snapshot::default())
    }
}

impl From<En1999Snapshot> for En1999Artifact {
    fn from(snapshot: crate::artifacts::en1999::En1999Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1999Artifact {
    pub fn to_snapshot(&self) -> crate::artifacts::en1999::En1999Snapshot {
        crate::artifacts::en1999::En1999Snapshot {
            n_ed_kn: self.n_ed_kn.clone(),
            m_ed_knm: self.m_ed_knm.clone(),
            a_mm2: self.a_mm2.clone(),
            w_el_mm3: self.w_el_mm3.clone(),
            alloy: self.alloy.clone(),
            chi: self.chi.clone(),
            i_t_mm4: self.i_t_mm4.clone(),
            l_cr_mm: self.l_cr_mm.clone(),
            theta_c: self.theta_c.clone(),
            delta_sigma_ed: self.delta_sigma_ed.clone(),
            delta_sigma_c: self.delta_sigma_c.clone(),
            fatigue_m: self.fatigue_m.clone(),
            n_cycles: self.n_cycles.clone(),
            v_weld_ed_kn: self.v_weld_ed_kn.clone(),
            weld_throat_mm: self.weld_throat_mm.clone(),
            weld_length_mm: self.weld_length_mm.clone(),
            beta_w: self.beta_w.clone(),
            sheet_b_mm: self.sheet_b_mm.clone(),
            sheet_t_mm: self.sheet_t_mm.clone(),
            sheet_k_sigma: self.sheet_k_sigma.clone(),
            sheet_w_el_mm3: self.sheet_w_el_mm3.clone(),
            sheet_m_ed_knm: self.sheet_m_ed_knm.clone(),
            shell_t_mm: self.shell_t_mm.clone(),
            shell_r_mm: self.shell_r_mm.clone(),
            sigma_ed_shell_mpa: self.sigma_ed_shell_mpa.clone(),
            annex: self.annex.clone(),
        }
    }

    pub fn from_snapshot(snapshot: crate::artifacts::en1999::En1999Snapshot) -> Self {
        Self {
            n_ed_kn: snapshot.n_ed_kn,
            m_ed_knm: snapshot.m_ed_knm,
            a_mm2: snapshot.a_mm2,
            w_el_mm3: snapshot.w_el_mm3,
            alloy: snapshot.alloy,
            chi: snapshot.chi,
            i_t_mm4: snapshot.i_t_mm4,
            l_cr_mm: snapshot.l_cr_mm,
            theta_c: snapshot.theta_c,
            delta_sigma_ed: snapshot.delta_sigma_ed,
            delta_sigma_c: snapshot.delta_sigma_c,
            fatigue_m: snapshot.fatigue_m,
            n_cycles: snapshot.n_cycles,
            v_weld_ed_kn: snapshot.v_weld_ed_kn,
            weld_throat_mm: snapshot.weld_throat_mm,
            weld_length_mm: snapshot.weld_length_mm,
            beta_w: snapshot.beta_w,
            sheet_b_mm: snapshot.sheet_b_mm,
            sheet_t_mm: snapshot.sheet_t_mm,
            sheet_k_sigma: snapshot.sheet_k_sigma,
            sheet_w_el_mm3: snapshot.sheet_w_el_mm3,
            sheet_m_ed_knm: snapshot.sheet_m_ed_knm,
            shell_t_mm: snapshot.shell_t_mm,
            shell_r_mm: snapshot.shell_r_mm,
            sigma_ed_shell_mpa: snapshot.sigma_ed_shell_mpa,
            annex: snapshot.annex,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1999::En1999Snapshot) {
        self.n_ed_kn = snapshot.n_ed_kn;
        self.m_ed_knm = snapshot.m_ed_knm;
        self.a_mm2 = snapshot.a_mm2;
        self.w_el_mm3 = snapshot.w_el_mm3;
        self.alloy = snapshot.alloy;
        self.chi = snapshot.chi;
        self.i_t_mm4 = snapshot.i_t_mm4;
        self.l_cr_mm = snapshot.l_cr_mm;
        self.theta_c = snapshot.theta_c;
        self.delta_sigma_ed = snapshot.delta_sigma_ed;
        self.delta_sigma_c = snapshot.delta_sigma_c;
        self.fatigue_m = snapshot.fatigue_m;
        self.n_cycles = snapshot.n_cycles;
        self.v_weld_ed_kn = snapshot.v_weld_ed_kn;
        self.weld_throat_mm = snapshot.weld_throat_mm;
        self.weld_length_mm = snapshot.weld_length_mm;
        self.beta_w = snapshot.beta_w;
        self.sheet_b_mm = snapshot.sheet_b_mm;
        self.sheet_t_mm = snapshot.sheet_t_mm;
        self.sheet_k_sigma = snapshot.sheet_k_sigma;
        self.sheet_w_el_mm3 = snapshot.sheet_w_el_mm3;
        self.sheet_m_ed_knm = snapshot.sheet_m_ed_knm;
        self.shell_t_mm = snapshot.shell_t_mm;
        self.shell_r_mm = snapshot.shell_r_mm;
        self.sigma_ed_shell_mpa = snapshot.sigma_ed_shell_mpa;
        self.annex = snapshot.annex;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1999_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1999",
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
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::en1999::{En1999Diff, En1999Mutation, En1999Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct En1999BuilderConstruction {
        snapshot: En1999Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1999BuilderConstruction {
        type Snapshot = En1999Snapshot;
        type Mutation = En1999Mutation;
        type Diff = En1999Diff;
        fn empty() -> Self { Self { snapshot: En1999Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1999Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1999Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <En1999Diff as protocol::MutationDiff<En1999Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <En1999Diff as protocol::MutationDiff<En1999Snapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::en1999::En1999Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct En1999Parts {
        pub snapshot: Option<En1999Snapshot>,
    }

    pub struct En1999AnalyzerAnalysis;

    impl ArtifactAnalysis for En1999AnalyzerAnalysis {
        type Parts = En1999Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1999", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1999Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1999Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1999Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1999BuilderFacets {
        construction: derived_construction::En1999BuilderConstruction,
        analysis: derived_analysis::En1999AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1999ComposerComposition,
    }
    builder: En1999Builder,
    analyzer: En1999Analyzer,
    composer: En1999Composer,
);
//#endregion 🧬️DerivedArtifactFacets
