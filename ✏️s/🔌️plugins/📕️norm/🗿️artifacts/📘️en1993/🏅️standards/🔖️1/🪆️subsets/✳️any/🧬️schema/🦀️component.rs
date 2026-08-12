//! 🧬️ En1993 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1993 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Artifact {
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub n_ed_kn: f64,
    #[state(persistent)] pub m_ed_knm: f64,
    #[state(persistent)] pub v_ed_kn: f64,
    #[state(persistent)] pub a_mm2: f64,
    #[state(persistent)] pub a_v_mm2: f64,
    #[state(persistent)] pub w_pl_mm3: f64,
    #[state(persistent)] pub f_y_mpa: f64,
    #[state(persistent)] pub f_u_mpa: f64,
    #[state(persistent)] pub chi: f64,
    #[state(persistent)] pub a_net_mm2: f64,
    #[state(persistent)] pub tension_n_ed_kn: f64,
    #[state(persistent)] pub fire_thickness_mm: f64,
    #[state(persistent)] pub fire_rating: String,
    #[state(persistent)] pub fire_massivity: f64,
    #[state(persistent)] pub fire_mu_0: f64,
    #[state(persistent)] pub fire_design_temperature_c: f64,
    #[state(persistent)] pub cf_b_bar_mm: f64,
    #[state(persistent)] pub cf_t_mm: f64,
    #[state(persistent)] pub cf_k_sigma: f64,
    #[state(persistent)] pub cf_psi: f64,
    #[state(persistent)] pub cf_n_ed_kn: f64,
    #[state(persistent)] pub cf_gross_resistance_kn: f64,
    #[state(persistent)] pub stainless_m_ed_knm: f64,
    #[state(persistent)] pub stainless_w_pl_mm3: f64,
    #[state(persistent)] pub stainless_f_y_mpa: f64,
    #[state(persistent)] pub plated_lambda_p: f64,
    #[state(persistent)] pub plated_sigma_ed_mpa: f64,
    #[state(persistent)] pub silo_t_mm: f64,
    #[state(persistent)] pub silo_r_mm: f64,
    #[state(persistent)] pub shell_sigma_x_ed_mpa: f64,
    #[state(persistent)] pub silo_k: f64,
    #[state(persistent)] pub silo_gamma_kn_m3: f64,
    #[state(persistent)] pub silo_depth_m: f64,
    #[state(persistent)] pub bolt_f_ed_kn: f64,
    #[state(persistent)] pub bolt_n_bolts: u32,
    #[state(persistent)] pub bolt_a_s_mm2: f64,
    #[state(persistent)] pub bolt_e1_mm: f64,
    #[state(persistent)] pub bolt_e2_mm: f64,
    #[state(persistent)] pub bolt_d0_mm: f64,
    #[state(persistent)] pub bolt_d_mm: f64,
    #[state(persistent)] pub bolt_t_mm: f64,
    #[state(persistent)] pub bolt_f_u_mpa: f64,
    #[state(persistent)] pub bolt_f_ub_mpa: f64,
    #[state(persistent)] pub weld_a_mm: f64,
    #[state(persistent)] pub weld_l_mm: f64,
    #[state(persistent)] pub weld_f_u_mpa: f64,
    #[state(persistent)] pub weld_steel_grade: String,
    #[state(persistent)] pub weld_f_ed_kn: f64,
    #[state(persistent)] pub delta_sigma_mpa: f64,
    #[state(persistent)] pub fatigue_category: u8,
    #[state(persistent)] pub fatigue_method: String,
    #[state(persistent)] pub t10_steel_subgrade: String,
    #[state(persistent)] pub t10_actual_thickness_mm: f64,
    #[state(persistent)] pub t10_t_ed_c: f64,
    #[state(persistent)] pub tension_component_f_uk_kn: f64,
    #[state(persistent)] pub tension_component_f_k_kn: f64,
    #[state(persistent)] pub tension_component_n_ed_kn: f64,
    #[state(persistent)] pub hss_w_el_mm3: f64,
    #[state(persistent)] pub hss_f_y_mpa: f64,
    #[state(persistent)] pub hss_section_class: u8,
    #[state(persistent)] pub hss_m_ed_knm: f64,
    #[state(persistent)] pub bridge_lambda: f64,
    #[state(persistent)] pub bridge_phi_2: f64,
    #[state(persistent)] pub bridge_delta_sigma_p_mpa: f64,
    #[state(persistent)] pub tower_wind_factor: f64,
    #[state(persistent)] pub tower_n_ed_kn: f64,
    #[state(persistent)] pub pile_sigma_mpa: f64,
    #[state(persistent)] pub pile_k_red: f64,
    #[state(persistent)] pub pile_n_ed_kn: f64,
    #[state(persistent)] pub crane_f_z_ed_kn: f64,
    #[state(persistent)] pub crane_wheel_contact_length_mm: f64,
    #[state(persistent)] pub crane_dispersion_mm: f64,
    #[state(persistent)] pub crane_t_w_mm: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1993Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1993::En1993Snapshot {
        crate::artifacts::en1993::En1993Snapshot {
            annex: self.annex,
            n_ed_kn: self.n_ed_kn,
            m_ed_knm: self.m_ed_knm,
            v_ed_kn: self.v_ed_kn,
            a_mm2: self.a_mm2,
            a_v_mm2: self.a_v_mm2,
            w_pl_mm3: self.w_pl_mm3,
            f_y_mpa: self.f_y_mpa,
            f_u_mpa: self.f_u_mpa,
            chi: self.chi,
            a_net_mm2: self.a_net_mm2,
            tension_n_ed_kn: self.tension_n_ed_kn,
            fire_thickness_mm: self.fire_thickness_mm,
            fire_rating: self.fire_rating.clone(),
            fire_massivity: self.fire_massivity,
            fire_mu_0: self.fire_mu_0,
            fire_design_temperature_c: self.fire_design_temperature_c,
            cf_b_bar_mm: self.cf_b_bar_mm,
            cf_t_mm: self.cf_t_mm,
            cf_k_sigma: self.cf_k_sigma,
            cf_psi: self.cf_psi,
            cf_n_ed_kn: self.cf_n_ed_kn,
            cf_gross_resistance_kn: self.cf_gross_resistance_kn,
            stainless_m_ed_knm: self.stainless_m_ed_knm,
            stainless_w_pl_mm3: self.stainless_w_pl_mm3,
            stainless_f_y_mpa: self.stainless_f_y_mpa,
            plated_lambda_p: self.plated_lambda_p,
            plated_sigma_ed_mpa: self.plated_sigma_ed_mpa,
            silo_t_mm: self.silo_t_mm,
            silo_r_mm: self.silo_r_mm,
            shell_sigma_x_ed_mpa: self.shell_sigma_x_ed_mpa,
            silo_k: self.silo_k,
            silo_gamma_kn_m3: self.silo_gamma_kn_m3,
            silo_depth_m: self.silo_depth_m,
            bolt_f_ed_kn: self.bolt_f_ed_kn,
            bolt_n_bolts: self.bolt_n_bolts,
            bolt_a_s_mm2: self.bolt_a_s_mm2,
            bolt_e1_mm: self.bolt_e1_mm,
            bolt_e2_mm: self.bolt_e2_mm,
            bolt_d0_mm: self.bolt_d0_mm,
            bolt_d_mm: self.bolt_d_mm,
            bolt_t_mm: self.bolt_t_mm,
            bolt_f_u_mpa: self.bolt_f_u_mpa,
            bolt_f_ub_mpa: self.bolt_f_ub_mpa,
            weld_a_mm: self.weld_a_mm,
            weld_l_mm: self.weld_l_mm,
            weld_f_u_mpa: self.weld_f_u_mpa,
            weld_steel_grade: self.weld_steel_grade.clone(),
            weld_f_ed_kn: self.weld_f_ed_kn,
            delta_sigma_mpa: self.delta_sigma_mpa,
            fatigue_category: self.fatigue_category,
            fatigue_method: self.fatigue_method.clone(),
            t10_steel_subgrade: self.t10_steel_subgrade.clone(),
            t10_actual_thickness_mm: self.t10_actual_thickness_mm,
            t10_t_ed_c: self.t10_t_ed_c,
            tension_component_f_uk_kn: self.tension_component_f_uk_kn,
            tension_component_f_k_kn: self.tension_component_f_k_kn,
            tension_component_n_ed_kn: self.tension_component_n_ed_kn,
            hss_w_el_mm3: self.hss_w_el_mm3,
            hss_f_y_mpa: self.hss_f_y_mpa,
            hss_section_class: self.hss_section_class,
            hss_m_ed_knm: self.hss_m_ed_knm,
            bridge_lambda: self.bridge_lambda,
            bridge_phi_2: self.bridge_phi_2,
            bridge_delta_sigma_p_mpa: self.bridge_delta_sigma_p_mpa,
            tower_wind_factor: self.tower_wind_factor,
            tower_n_ed_kn: self.tower_n_ed_kn,
            pile_sigma_mpa: self.pile_sigma_mpa,
            pile_k_red: self.pile_k_red,
            pile_n_ed_kn: self.pile_n_ed_kn,
            crane_f_z_ed_kn: self.crane_f_z_ed_kn,
            crane_wheel_contact_length_mm: self.crane_wheel_contact_length_mm,
            crane_dispersion_mm: self.crane_dispersion_mm,
            crane_t_w_mm: self.crane_t_w_mm,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1993::En1993Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            n_ed_kn: snapshot.n_ed_kn,
            m_ed_knm: snapshot.m_ed_knm,
            v_ed_kn: snapshot.v_ed_kn,
            a_mm2: snapshot.a_mm2,
            a_v_mm2: snapshot.a_v_mm2,
            w_pl_mm3: snapshot.w_pl_mm3,
            f_y_mpa: snapshot.f_y_mpa,
            f_u_mpa: snapshot.f_u_mpa,
            chi: snapshot.chi,
            a_net_mm2: snapshot.a_net_mm2,
            tension_n_ed_kn: snapshot.tension_n_ed_kn,
            fire_thickness_mm: snapshot.fire_thickness_mm,
            fire_rating: snapshot.fire_rating.clone(),
            fire_massivity: snapshot.fire_massivity,
            fire_mu_0: snapshot.fire_mu_0,
            fire_design_temperature_c: snapshot.fire_design_temperature_c,
            cf_b_bar_mm: snapshot.cf_b_bar_mm,
            cf_t_mm: snapshot.cf_t_mm,
            cf_k_sigma: snapshot.cf_k_sigma,
            cf_psi: snapshot.cf_psi,
            cf_n_ed_kn: snapshot.cf_n_ed_kn,
            cf_gross_resistance_kn: snapshot.cf_gross_resistance_kn,
            stainless_m_ed_knm: snapshot.stainless_m_ed_knm,
            stainless_w_pl_mm3: snapshot.stainless_w_pl_mm3,
            stainless_f_y_mpa: snapshot.stainless_f_y_mpa,
            plated_lambda_p: snapshot.plated_lambda_p,
            plated_sigma_ed_mpa: snapshot.plated_sigma_ed_mpa,
            silo_t_mm: snapshot.silo_t_mm,
            silo_r_mm: snapshot.silo_r_mm,
            shell_sigma_x_ed_mpa: snapshot.shell_sigma_x_ed_mpa,
            silo_k: snapshot.silo_k,
            silo_gamma_kn_m3: snapshot.silo_gamma_kn_m3,
            silo_depth_m: snapshot.silo_depth_m,
            bolt_f_ed_kn: snapshot.bolt_f_ed_kn,
            bolt_n_bolts: snapshot.bolt_n_bolts,
            bolt_a_s_mm2: snapshot.bolt_a_s_mm2,
            bolt_e1_mm: snapshot.bolt_e1_mm,
            bolt_e2_mm: snapshot.bolt_e2_mm,
            bolt_d0_mm: snapshot.bolt_d0_mm,
            bolt_d_mm: snapshot.bolt_d_mm,
            bolt_t_mm: snapshot.bolt_t_mm,
            bolt_f_u_mpa: snapshot.bolt_f_u_mpa,
            bolt_f_ub_mpa: snapshot.bolt_f_ub_mpa,
            weld_a_mm: snapshot.weld_a_mm,
            weld_l_mm: snapshot.weld_l_mm,
            weld_f_u_mpa: snapshot.weld_f_u_mpa,
            weld_steel_grade: snapshot.weld_steel_grade.clone(),
            weld_f_ed_kn: snapshot.weld_f_ed_kn,
            delta_sigma_mpa: snapshot.delta_sigma_mpa,
            fatigue_category: snapshot.fatigue_category,
            fatigue_method: snapshot.fatigue_method.clone(),
            t10_steel_subgrade: snapshot.t10_steel_subgrade.clone(),
            t10_actual_thickness_mm: snapshot.t10_actual_thickness_mm,
            t10_t_ed_c: snapshot.t10_t_ed_c,
            tension_component_f_uk_kn: snapshot.tension_component_f_uk_kn,
            tension_component_f_k_kn: snapshot.tension_component_f_k_kn,
            tension_component_n_ed_kn: snapshot.tension_component_n_ed_kn,
            hss_w_el_mm3: snapshot.hss_w_el_mm3,
            hss_f_y_mpa: snapshot.hss_f_y_mpa,
            hss_section_class: snapshot.hss_section_class,
            hss_m_ed_knm: snapshot.hss_m_ed_knm,
            bridge_lambda: snapshot.bridge_lambda,
            bridge_phi_2: snapshot.bridge_phi_2,
            bridge_delta_sigma_p_mpa: snapshot.bridge_delta_sigma_p_mpa,
            tower_wind_factor: snapshot.tower_wind_factor,
            tower_n_ed_kn: snapshot.tower_n_ed_kn,
            pile_sigma_mpa: snapshot.pile_sigma_mpa,
            pile_k_red: snapshot.pile_k_red,
            pile_n_ed_kn: snapshot.pile_n_ed_kn,
            crane_f_z_ed_kn: snapshot.crane_f_z_ed_kn,
            crane_wheel_contact_length_mm: snapshot.crane_wheel_contact_length_mm,
            crane_dispersion_mm: snapshot.crane_dispersion_mm,
            crane_t_w_mm: snapshot.crane_t_w_mm,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1993::En1993Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1993` — twenty handcrafted schema leaves.
pub fn en1993_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1993",
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
    use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};

    #[derive(Clone, Debug, Default)]
    pub struct En1993BuilderConstruction {
        snapshot: En1993Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1993BuilderConstruction {
        type Snapshot = En1993Snapshot;
        type Mutation = En1993Mutation;
        type Diff = En1993Diff;
        fn empty() -> Self { Self { snapshot: En1993Snapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1993Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1993Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let d = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(&d, &self.snapshot);
            (self, d)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::en1993::En1993Snapshot;

    #[derive(Clone, Debug, Default)]
    pub struct En1993Parts {
        pub snapshot: Option<En1993Snapshot>,
    }

    pub struct En1993AnalyzerAnalysis;

    impl ArtifactAnalysis for En1993AnalyzerAnalysis {
        type Parts = En1993Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.en1993", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1993Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1993Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1993Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1993BuilderFacets {
        construction: derived_construction::En1993BuilderConstruction,
        analysis: derived_analysis::En1993AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1993ComposerComposition,
    }
    builder: En1993Builder,
    analyzer: En1993Analyzer,
    composer: En1993Composer,
);
//#endregion 🧬️DerivedArtifactFacets
