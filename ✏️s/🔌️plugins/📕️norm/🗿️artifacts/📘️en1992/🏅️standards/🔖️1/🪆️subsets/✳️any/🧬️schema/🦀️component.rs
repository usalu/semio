//! 🧬️ En1992 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use crate::artifacts::en1992::part_3::TightnessClass;
use crate::artifacts::en1992::part_1_2::FireRating;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1992 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Artifact {
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub m_ed_knm: f64,
    #[state(persistent)] pub v_ed_kn: f64,
    #[state(persistent)] pub f_ck: f64,
    #[state(persistent)] pub b_mm: f64,
    #[state(persistent)] pub d_mm: f64,
    #[state(persistent)] pub a_s_mm2: f64,
    #[state(persistent)] pub f_yk: f64,
    #[state(persistent)] pub rho_l: f64,
    #[state(persistent)] pub n_ed_kn: f64,
    #[state(persistent)] pub p_kn: f64,
    #[state(persistent)] pub a_c_mm2: f64,
    #[state(persistent)] pub use_fem: bool,
    #[state(persistent)] pub span_m: f64,
    #[state(persistent)] pub udl_kn_m: f64,
    #[state(persistent)] pub fire_rating: crate::artifacts::en1992::part_1_2::FireRating,
    #[state(persistent)] pub provided_axis_distance_mm: f64,
    #[state(persistent)] pub bridge_sigma_c_mpa: f64,
    #[state(persistent)] pub bridge_delta_sigma_s_mpa: f64,
    #[state(persistent)] pub tightness_class: crate::artifacts::en1992::part_3::TightnessClass,
    #[state(persistent)] pub hd_over_h: f64,
    #[state(persistent)] pub liquid_sigma_s_mpa: f64,
    #[state(persistent)] pub liquid_rho_p_eff: f64,
    #[state(persistent)] pub liquid_f_ct_eff_mpa: f64,
    #[state(persistent)] pub liquid_e_s_mpa: f64,
    #[state(persistent)] pub liquid_s_r_max_mm: f64,
    #[state(persistent)] pub anchor_h_ef_mm: f64,
    #[state(persistent)] pub anchor_cracked: bool,
    #[state(persistent)] pub anchor_f_uk_mpa: f64,
    #[state(persistent)] pub anchor_f_yk_mpa: f64,
    #[state(persistent)] pub anchor_a_s_mm2: f64,
    #[state(persistent)] pub anchor_d_mm: f64,
    #[state(persistent)] pub anchor_c1_mm: f64,
    #[state(persistent)] pub anchor_n_ed_kn: f64,
    #[state(persistent)] pub anchor_v_ed_kn: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1992Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1992::En1992Snapshot {
        crate::artifacts::en1992::En1992Snapshot {
            annex: self.annex,
            m_ed_knm: self.m_ed_knm,
            v_ed_kn: self.v_ed_kn,
            f_ck: self.f_ck,
            b_mm: self.b_mm,
            d_mm: self.d_mm,
            a_s_mm2: self.a_s_mm2,
            f_yk: self.f_yk,
            rho_l: self.rho_l,
            n_ed_kn: self.n_ed_kn,
            p_kn: self.p_kn,
            a_c_mm2: self.a_c_mm2,
            use_fem: self.use_fem,
            span_m: self.span_m,
            udl_kn_m: self.udl_kn_m,
            fire_rating: self.fire_rating,
            provided_axis_distance_mm: self.provided_axis_distance_mm,
            bridge_sigma_c_mpa: self.bridge_sigma_c_mpa,
            bridge_delta_sigma_s_mpa: self.bridge_delta_sigma_s_mpa,
            tightness_class: self.tightness_class,
            hd_over_h: self.hd_over_h,
            liquid_sigma_s_mpa: self.liquid_sigma_s_mpa,
            liquid_rho_p_eff: self.liquid_rho_p_eff,
            liquid_f_ct_eff_mpa: self.liquid_f_ct_eff_mpa,
            liquid_e_s_mpa: self.liquid_e_s_mpa,
            liquid_s_r_max_mm: self.liquid_s_r_max_mm,
            anchor_h_ef_mm: self.anchor_h_ef_mm,
            anchor_cracked: self.anchor_cracked,
            anchor_f_uk_mpa: self.anchor_f_uk_mpa,
            anchor_f_yk_mpa: self.anchor_f_yk_mpa,
            anchor_a_s_mm2: self.anchor_a_s_mm2,
            anchor_d_mm: self.anchor_d_mm,
            anchor_c1_mm: self.anchor_c1_mm,
            anchor_n_ed_kn: self.anchor_n_ed_kn,
            anchor_v_ed_kn: self.anchor_v_ed_kn,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1992::En1992Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            m_ed_knm: snapshot.m_ed_knm,
            v_ed_kn: snapshot.v_ed_kn,
            f_ck: snapshot.f_ck,
            b_mm: snapshot.b_mm,
            d_mm: snapshot.d_mm,
            a_s_mm2: snapshot.a_s_mm2,
            f_yk: snapshot.f_yk,
            rho_l: snapshot.rho_l,
            n_ed_kn: snapshot.n_ed_kn,
            p_kn: snapshot.p_kn,
            a_c_mm2: snapshot.a_c_mm2,
            use_fem: snapshot.use_fem,
            span_m: snapshot.span_m,
            udl_kn_m: snapshot.udl_kn_m,
            fire_rating: snapshot.fire_rating,
            provided_axis_distance_mm: snapshot.provided_axis_distance_mm,
            bridge_sigma_c_mpa: snapshot.bridge_sigma_c_mpa,
            bridge_delta_sigma_s_mpa: snapshot.bridge_delta_sigma_s_mpa,
            tightness_class: snapshot.tightness_class,
            hd_over_h: snapshot.hd_over_h,
            liquid_sigma_s_mpa: snapshot.liquid_sigma_s_mpa,
            liquid_rho_p_eff: snapshot.liquid_rho_p_eff,
            liquid_f_ct_eff_mpa: snapshot.liquid_f_ct_eff_mpa,
            liquid_e_s_mpa: snapshot.liquid_e_s_mpa,
            liquid_s_r_max_mm: snapshot.liquid_s_r_max_mm,
            anchor_h_ef_mm: snapshot.anchor_h_ef_mm,
            anchor_cracked: snapshot.anchor_cracked,
            anchor_f_uk_mpa: snapshot.anchor_f_uk_mpa,
            anchor_f_yk_mpa: snapshot.anchor_f_yk_mpa,
            anchor_a_s_mm2: snapshot.anchor_a_s_mm2,
            anchor_d_mm: snapshot.anchor_d_mm,
            anchor_c1_mm: snapshot.anchor_c1_mm,
            anchor_n_ed_kn: snapshot.anchor_n_ed_kn,
            anchor_v_ed_kn: snapshot.anchor_v_ed_kn,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1992::En1992Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1992` — twenty handcrafted schema leaves.
pub fn en1992_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1992",
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
    use crate::artifacts::en1992::{En1992Diff, En1992Mutation, En1992Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct En1992BuilderConstruction {
        snapshot: En1992Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1992BuilderConstruction {
        type Snapshot = En1992Snapshot;
        type Mutation = En1992Mutation;
        type Diff = En1992Diff;
        fn empty() -> Self { Self { snapshot: En1992Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1992Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1992Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::en1992::En1992Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct En1992Parts {
        pub snapshot: Option<En1992Snapshot>,
    }

    pub struct En1992AnalyzerAnalysis;

    impl ArtifactAnalysis for En1992AnalyzerAnalysis {
        type Parts = En1992Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1992", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1992Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1992Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1992Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1992BuilderFacets {
        construction: derived_construction::En1992BuilderConstruction,
        analysis: derived_analysis::En1992AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1992ComposerComposition,
    }
    builder: En1992Builder,
    analyzer: En1992Analyzer,
    composer: En1992Composer,
);
//#endregion 🧬️DerivedArtifactFacets
