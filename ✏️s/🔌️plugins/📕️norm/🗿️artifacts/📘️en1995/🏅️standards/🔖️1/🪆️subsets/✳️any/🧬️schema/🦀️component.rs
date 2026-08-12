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
    use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct En1995BuilderConstruction {
        snapshot: En1995Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1995BuilderConstruction {
        type Snapshot = En1995Snapshot;
        type Mutation = En1995Mutation;
        type Diff = En1995Diff;
        fn empty() -> Self { Self { snapshot: En1995Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1995Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1995Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <En1995Diff as protocol::MutationDiff<En1995Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <En1995Diff as protocol::MutationDiff<En1995Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::en1995::En1995Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct En1995Parts {
        pub snapshot: Option<En1995Snapshot>,
    }

    pub struct En1995AnalyzerAnalysis;

    impl ArtifactAnalysis for En1995AnalyzerAnalysis {
        type Parts = En1995Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1995", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1995Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1995Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1995Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1995BuilderFacets {
        construction: derived_construction::En1995BuilderConstruction,
        analysis: derived_analysis::En1995AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1995ComposerComposition,
    }
    builder: En1995Builder,
    analyzer: En1995Analyzer,
    composer: En1995Composer,
);
//#endregion 🧬️DerivedArtifactFacets
