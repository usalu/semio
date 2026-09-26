//! 🌍️ EN 1997 artifact schema — geotechnical project subject + clause helpers.

use crate::document::AnnexChoice;
use crate::{
    En1997Snapshot, FoundationLoadCase, Pile, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ EN 1997 document artifact state (mirrors snapshot).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Artifact {
    #[state(artifact)]
    pub structure_id: String,
    #[state(artifact)]
    pub geotechnical_category: u8,
    #[state(artifact)]
    pub design_situation: String,
    #[state(artifact)]
    pub design_approach: String,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub groundwater_level: f64,
    #[state(artifact)]
    pub investigation_depth: f64,
    #[state(artifact)]
    pub layers: Vec<SoilLayer>,
    #[state(artifact)]
    pub footings: Vec<SpreadFoundation>,
    #[state(artifact)]
    pub piles: Vec<Pile>,
    #[state(artifact)]
    pub retaining_walls: Vec<RetainingWall>,
    #[state(artifact)]
    pub slopes: Vec<Slope>,
    #[state(artifact)]
    pub uplift_cases: Vec<UpliftCase>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1997Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1997Snapshot::default())
    }
}

impl From<En1997Snapshot> for En1997Artifact {
    fn from(snapshot: En1997Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1997Artifact {
    pub fn to_snapshot(&self) -> En1997Snapshot {
        En1997Snapshot {
            structure_id: self.structure_id.clone(),
            geotechnical_category: self.geotechnical_category,
            design_situation: self.design_situation.clone(),
            design_approach: self.design_approach.clone(),
            annex: self.annex,
            groundwater_level: self.groundwater_level,
            investigation_depth: self.investigation_depth,
            layers: self.layers.clone(),
            footings: self.footings.clone(),
            piles: self.piles.clone(),
            retaining_walls: self.retaining_walls.clone(),
            slopes: self.slopes.clone(),
            uplift_cases: self.uplift_cases.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1997Snapshot) -> Self {
        Self {
            structure_id: snapshot.structure_id,
            geotechnical_category: snapshot.geotechnical_category,
            design_situation: snapshot.design_situation,
            design_approach: snapshot.design_approach,
            annex: snapshot.annex,
            groundwater_level: snapshot.groundwater_level,
            investigation_depth: snapshot.investigation_depth,
            layers: snapshot.layers,
            footings: snapshot.footings,
            piles: snapshot.piles,
            retaining_walls: snapshot.retaining_walls,
            slopes: snapshot.slopes,
            uplift_cases: snapshot.uplift_cases,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: En1997Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}
//#endregion 🔖️Conversions


//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{En1997Diff, En1997Mutation, En1997Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1997BuilderConstruction {
        snapshot: En1997Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1997BuilderConstruction {
        type Snapshot = En1997Snapshot;
        type Mutation = En1997Mutation;
        type Diff = En1997Diff;
        fn empty() -> Self {
            Self { snapshot: En1997Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1997Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1997Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1997Diff as protocol::MutationDiff<En1997Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::En1997Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1997Parts {
        pub snapshot: Option<En1997Snapshot>,
    }

    pub struct En1997AnalyzerAnalysis;

    impl ArtifactAnalysis for En1997AnalyzerAnalysis {
        type Parts = En1997Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1997", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1997Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1997Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1997Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 🔖️Descriptor
pub fn en1997_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1997",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

semio_framework_plugin::derive_artifact_facets!(
    pub spec En1997BuilderFacets {
        construction: En1997BuilderConstruction,
        analysis: En1997AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1997ComposerComposition,
    }
    builder: En1997Builder,
    analyzer: En1997Analyzer,
    composer: En1997Composer,
);



//#region 🔖️ComplianceHelpers
use crate::document::{
    CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef,
};

pub mod na_de {
    pub fn gamma_c() -> f64 { 1.4 }
    pub fn gamma_phi() -> f64 { 1.25 }
    pub fn gamma_gamma() -> f64 { 1.0 }
}

/// ⚖️ Design approach / GEO combination per EN 1997-1 §2.4.7 and DIN 1054.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignApproach {
    Da1Str,
    Da1Geo,
    Da2,
    Da3,
    Geo2,
    Geo3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub gamma_g: f64,
    pub gamma_q: f64,
    pub gamma_r_v: f64,
    pub gamma_r_h: f64,
    pub gamma_c: f64,
    pub gamma_phi: f64,
    pub gamma_gamma: f64,
    pub gamma_b: f64,
    pub gamma_s: f64,
    pub gamma_st: f64,
    pub gamma_r_e: f64,
}

impl DesignApproach {
    pub fn label(self, annex: AnnexChoice) -> &'static str {
        match (self, annex) {
            (Self::Da2 | Self::Geo2, AnnexChoice::De) => "DA2*/GEO-2",
            (Self::Da2 | Self::Geo2, AnnexChoice::En) => "DA2",
            (Self::Geo3, _) => "GEO-3",
            (Self::Da1Str, _) => "DA1-C1",
            (Self::Da1Geo, _) => "DA1-C2",
            (Self::Da3, _) => "DA3",
        }
    }

    pub fn annex_params(self, annex: AnnexChoice) -> AnnexParams {
        match (self, annex) {
            (Self::Da2 | Self::Geo2, AnnexChoice::De) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.4, gamma_r_h: 1.1,
                gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Da2 | Self::Geo2, AnnexChoice::En) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Geo3, AnnexChoice::De) => AnnexParams {
                gamma_g: 1.0, gamma_q: 1.3, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.25, gamma_phi: 1.25, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Geo3, AnnexChoice::En) => AnnexParams {
                gamma_g: 1.0, gamma_q: 1.3, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.25, gamma_phi: 1.25, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Da1Str, AnnexChoice::En) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.25, gamma_phi: 1.25, gamma_gamma: 1.0, gamma_b: 1.0, gamma_s: 1.0, gamma_st: 1.0, gamma_r_e: 1.0,
            },
            (Self::Da1Str, _) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: na_de::gamma_c(), gamma_phi: na_de::gamma_phi(), gamma_gamma: na_de::gamma_gamma(),
                gamma_b: 1.0, gamma_s: 1.0, gamma_st: 1.0, gamma_r_e: 1.0,
            },
            (Self::Da1Geo, _) => AnnexParams {
                gamma_g: 1.0, gamma_q: 1.3, gamma_r_v: 1.4, gamma_r_h: 1.1,
                gamma_c: 1.0, gamma_phi: 1.0, gamma_gamma: 1.0, gamma_b: 1.1, gamma_s: 1.1, gamma_st: 1.1, gamma_r_e: 1.1,
            },
            (Self::Da3, AnnexChoice::En) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: 1.25, gamma_phi: 1.25, gamma_gamma: 1.0, gamma_b: 1.15, gamma_s: 1.15, gamma_st: 1.15, gamma_r_e: 1.1,
            },
            (Self::Da3, _) => AnnexParams {
                gamma_g: 1.35, gamma_q: 1.5, gamma_r_v: 1.0, gamma_r_h: 1.0,
                gamma_c: na_de::gamma_c(), gamma_phi: na_de::gamma_phi(), gamma_gamma: na_de::gamma_gamma(),
                gamma_b: 1.15, gamma_s: 1.15, gamma_st: 1.15, gamma_r_e: 1.1,
            },
        }
    }
}


/// ⚖️ DIN 1054 design situation (BS-P / BS-T / BS-A).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignSituation {
    Persistent,
    Transient,
    Accidental,
}

pub fn parse_design_situation(value: &str) -> DesignSituation {
    match value.trim().to_ascii_lowercase().replace('-', "").replace('_', "").as_str() {
        "bst" | "transient" | "vorübergehend" | "voruebergehend" => DesignSituation::Transient,
        "bsa" | "accidental" | "aussergewöhnlich" | "außergewöhnlich" | "aussergewoehnlich" => DesignSituation::Accidental,
        _ => DesignSituation::Persistent,
    }
}

/// ⚖️ Resolve partial factors for approach + annex + DIN 1054 design situation.
///
/// DA2* (DE + DA2/GEO-2): characteristic soil (γ_φ′=γ_c′=1), factors on actions (γ_G, γ_Q)
/// and resistances (γ_R,·) — single coupled verification, not DA1 Combination 1+2.
pub fn resolve_params(approach: DesignApproach, annex: AnnexChoice, situation: &str) -> AnnexParams {
    let mut p = approach.annex_params(annex);
    let sit = parse_design_situation(situation);
    let da2_star = annex == AnnexChoice::De && matches!(approach, DesignApproach::Da2 | DesignApproach::Geo2);
    if da2_star {
        p.gamma_c = 1.0;
        p.gamma_phi = 1.0;
        p.gamma_gamma = 1.0;
    }
    match sit {
        DesignSituation::Persistent => {
            if annex == AnnexChoice::De {
                match approach {
                    DesignApproach::Da2 | DesignApproach::Geo2 => {
                        p.gamma_g = 1.35;
                        p.gamma_q = 1.50;
                        p.gamma_r_v = 1.40;
                        p.gamma_r_h = 1.10;
                    }
                    DesignApproach::Geo3 | DesignApproach::Da1Geo => {
                        p.gamma_g = 1.00;
                        p.gamma_q = 1.30;
                    }
                    _ => {}
                }
            }
        }
        DesignSituation::Transient => {
            if annex == AnnexChoice::De {
                p.gamma_g = 1.20;
                p.gamma_q = 1.30;
                if matches!(approach, DesignApproach::Da2 | DesignApproach::Geo2) {
                    p.gamma_r_v = 1.30;
                    p.gamma_r_h = 1.10;
                    p.gamma_c = 1.0;
                    p.gamma_phi = 1.0;
                }
                if matches!(approach, DesignApproach::Geo3 | DesignApproach::Da3) {
                    p.gamma_phi = (p.gamma_phi * 0.96).max(1.0);
                    p.gamma_c = (p.gamma_c * 0.96).max(1.0);
                }
            } else {
                p.gamma_g = 1.20;
                p.gamma_q = 1.30;
            }
        }
        DesignSituation::Accidental => {
            p.gamma_g = 1.00;
            p.gamma_q = 1.00;
            if annex == AnnexChoice::De && matches!(approach, DesignApproach::Da2 | DesignApproach::Geo2) {
                p.gamma_r_v = 1.20;
                p.gamma_r_h = 1.10;
                p.gamma_c = 1.0;
                p.gamma_phi = 1.0;
            }
        }
    }
    p
}

/// ⚖️ UPL/HYD partial factors (DIN 1054 Table A.2.1 / EN 1997-1 §10).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UplParams {
    pub gamma_g_stb: f64,
    pub gamma_g_dst: f64,
    pub gamma_q_dst: f64,
    pub gamma_hyd: f64,
}

pub fn resolve_upl_params(annex: AnnexChoice, situation: &str) -> UplParams {
    let sit = parse_design_situation(situation);
    match (annex, sit) {
        (AnnexChoice::De, DesignSituation::Transient) => UplParams {
            gamma_g_stb: 0.95, gamma_g_dst: 1.05, gamma_q_dst: 1.30, gamma_hyd: 1.30,
        },
        (AnnexChoice::De, DesignSituation::Accidental) => UplParams {
            gamma_g_stb: 1.00, gamma_g_dst: 1.00, gamma_q_dst: 1.00, gamma_hyd: 1.00,
        },
        (AnnexChoice::De, _) => UplParams {
            gamma_g_stb: 0.90, gamma_g_dst: 1.10, gamma_q_dst: 1.50, gamma_hyd: 1.50,
        },
        (_, DesignSituation::Transient) => UplParams {
            gamma_g_stb: 0.95, gamma_g_dst: 1.05, gamma_q_dst: 1.30, gamma_hyd: 1.30,
        },
        (_, DesignSituation::Accidental) => UplParams {
            gamma_g_stb: 1.00, gamma_g_dst: 1.00, gamma_q_dst: 1.00, gamma_hyd: 1.00,
        },
        _ => UplParams {
            gamma_g_stb: 0.90, gamma_g_dst: 1.10, gamma_q_dst: 1.50, gamma_hyd: 1.50,
        },
    }
}


pub fn parse_design_approach(value: &str) -> DesignApproach {
    match value.to_ascii_lowercase().as_str() {
        "da1geo" => DesignApproach::Da1Geo,
        "da2" | "geo2" => DesignApproach::Da2,
        "da3" => DesignApproach::Da3,
        "geo3" => DesignApproach::Geo3,
        _ => DesignApproach::Da1Str,
    }
}

fn loc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

fn subject_entity(id: &str, path: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(id, path, loc(en, de))
}

pub mod part_1 {
    use super::*;

    /// 📐️ Annex D bearing factor N_q = e^(π·tanφ)·tan²(45+φ/2).
    pub fn bearing_factor_n_q(phi_deg: f64) -> f64 {
        let phi = phi_deg.to_radians();
        (std::f64::consts::PI * phi.tan()).exp() * (std::f64::consts::FRAC_PI_4 + phi / 2.0).tan().powi(2)
    }

    /// 📐️ Annex D bearing factor N_c = (N_q − 1)·cot φ.
    pub fn bearing_factor_n_c(phi_deg: f64) -> f64 {
        if phi_deg.abs() < 1e-9 { return 5.14; }
        let phi = phi_deg.to_radians();
        (bearing_factor_n_q(phi_deg) - 1.0) / phi.tan()
    }

    /// 📐️ Annex D bearing factor N_γ = 2·(N_q − 1)·tan φ.
    pub fn bearing_factor_n_gamma(phi_deg: f64) -> f64 {
        2.0 * (bearing_factor_n_q(phi_deg) - 1.0) * phi_deg.to_radians().tan()
    }

    /// 📐️ Shape/depth/inclination factors and effective widths (Annex D).
    pub fn effective_width(b: f64, eccentricity: f64) -> f64 {
        (b - 2.0 * eccentricity.abs()).max(1e-6)
    }

    pub fn shape_factors(phi_deg: f64, b_prime: f64, l_prime: f64) -> (f64, f64, f64) {
        let n_q = bearing_factor_n_q(phi_deg);
        let n_c = bearing_factor_n_c(phi_deg);
        let ratio = (b_prime / l_prime.max(b_prime)).clamp(0.0, 1.0);
        let s_q = 1.0 + ratio * phi_deg.to_radians().sin();
        let s_gamma = 1.0 - 0.3 * ratio;
        let s_c = if n_c.abs() < 1e-9 { 1.0 } else { (s_q * n_q - 1.0) / (n_c.max(1e-9)) };
        (s_c, s_q, s_gamma.max(0.7))
    }

    pub fn depth_factors(phi_deg: f64, embedment: f64, b_prime: f64) -> (f64, f64, f64) {
        let phi = phi_deg.to_radians();
        let d_b = embedment / b_prime.max(1e-6);
        let d_q = 1.0 + 2.0 * phi.tan() * (1.0 - phi.sin()).powi(2) * d_b.atan();
        let d_c = if phi_deg.abs() < 1e-9 { 1.0 + 0.4 * d_b.atan() } else { d_q - (1.0 - d_q) / (bearing_factor_n_c(phi_deg) * phi.tan()).max(1e-9) };
        let d_gamma = 1.0;
        (d_c, d_q, d_gamma)
    }

    pub fn inclination_factors(phi_deg: f64, h: f64, v: f64, c: f64, b_prime: f64, l_prime: f64) -> (f64, f64, f64) {
        let a = b_prime * l_prime;
        let m = (2.0 + (b_prime / l_prime.max(b_prime))) / (1.0 + (b_prime / l_prime.max(b_prime)));
        let n_c = bearing_factor_n_c(phi_deg);
        let denom = (v + a * c / n_c.max(1e-6)).max(1e-6);
        let ratio = (1.0 - h.abs() / denom).clamp(0.0, 1.0);
        let i_q = ratio.powf(m);
        let i_c = if phi_deg.abs() < 1e-9 { 1.0 - m * h.abs() / (a * c * n_c + 1e-6).max(1e-6) } else {
            i_q - (1.0 - i_q) / (n_c * phi_deg.to_radians().tan()).max(1e-9)
        };
        let i_gamma = ratio.powf(m + 1.0);
        (i_c.max(0.0), i_q.max(0.0), i_gamma.max(0.0))
    }

    /// 📐️ Annex D base-inclination factors b_c, b_q, b_γ for footing base tilt α.
    pub fn base_inclination_factors(phi_deg: f64, alpha_deg: f64) -> (f64, f64, f64) {
        let alpha = alpha_deg.to_radians().abs();
        let phi = phi_deg.to_radians();
        if alpha < 1e-12 {
            return (1.0, 1.0, 1.0);
        }
        let b_q = (1.0 - alpha.tan() * phi.tan()).max(0.0).powi(2);
        let b_gamma = b_q;
        let n_c = bearing_factor_n_c(phi_deg);
        let b_c = if phi_deg.abs() < 1e-9 {
            1.0 - 2.0 * alpha / std::f64::consts::PI
        } else {
            b_q - (1.0 - b_q) / (n_c * phi.tan()).max(1e-9)
        };
        (b_c.max(0.0), b_q.max(0.0), b_gamma.max(0.0))
    }

    /// 📐️ Effective unit weight [N/m³] below foundation base with GWL interpolation over width B (Annex D γ′).
    pub fn effective_gamma_below_pa_m(gamma: f64, gamma_prime: f64, embedment: f64, gwl: f64, width: f64) -> f64 {
        let base = embedment;
        let zone = width.max(0.1);
        if gwl >= base + zone {
            gamma
        } else if gwl <= base {
            gamma_prime.max(1.0)
        } else {
            let wet = ((base + zone) - gwl) / zone;
            gamma * (1.0 - wet) + gamma_prime.max(1.0) * wet
        }
    }

    /// 📐️ Ultimate bearing pressure q_u [Pa] per EN 1997-1 Annex D (incl. base inclination).
    pub fn ultimate_bearing_pa(
        phi_deg: f64, c_pa: f64, gamma_overburden_pa_m: f64, gamma_below_pa_m: f64,
        b: f64, l: f64, embedment: f64, eccentricity_b: f64, eccentricity_l: f64, h: f64, v: f64,
        base_inclination_deg: f64,
    ) -> f64 {
        let b_p = effective_width(b, eccentricity_b);
        let l_p = effective_width(l, eccentricity_l);
        let (s_c, s_q, s_g) = shape_factors(phi_deg, b_p, l_p);
        let (d_c, d_q, d_g) = depth_factors(phi_deg, embedment, b_p);
        let (i_c, i_q, i_g) = inclination_factors(phi_deg, h, v, c_pa, b_p, l_p);
        let (b_c, b_q, b_g) = base_inclination_factors(phi_deg, base_inclination_deg);
        let n_c = bearing_factor_n_c(phi_deg);
        let n_q = bearing_factor_n_q(phi_deg);
        let n_g = bearing_factor_n_gamma(phi_deg);
        let q = gamma_overburden_pa_m * embedment;
        c_pa * n_c * s_c * d_c * i_c * b_c
            + q * n_q * s_q * d_q * i_q * b_q
            + 0.5 * gamma_below_pa_m * b_p * n_g * s_g * d_g * i_g * b_g
    }

    pub fn design_bearing_resistance_n(
        phi_deg: f64, c_pa: f64, gamma_pa_m: f64, b: f64, l: f64, embedment: f64,
        e_b: f64, e_l: f64, h: f64, v: f64, approach: DesignApproach, annex: AnnexChoice,
    ) -> f64 {
        design_bearing_resistance_with_params(phi_deg, c_pa, gamma_pa_m, gamma_pa_m, b, l, embedment, e_b, e_l, h, v, 0.0, &approach.annex_params(annex))
    }

    pub fn design_bearing_resistance_with_params(
        phi_deg: f64, c_pa: f64, gamma_over_pa_m: f64, gamma_below_pa_m: f64, b: f64, l: f64, embedment: f64,
        e_b: f64, e_l: f64, h: f64, v: f64, base_inclination_deg: f64, p: &AnnexParams,
    ) -> f64 {
        let phi_d = (phi_deg.to_radians() / p.gamma_phi).atan().to_degrees();
        let c_d = c_pa / p.gamma_c;
        let gamma_o = gamma_over_pa_m / p.gamma_gamma;
        let gamma_b = gamma_below_pa_m / p.gamma_gamma;
        let b_p = effective_width(b, e_b);
        let l_p = effective_width(l, e_l);
        let q_u = ultimate_bearing_pa(phi_d, c_d, gamma_o, gamma_b, b, l, embedment, e_b, e_l, h, v, base_inclination_deg);
        q_u * b_p * l_p / p.gamma_r_v
    }

    /// 📐️ Passive earth-pressure force E_p,k [N] on the front face (Coulomb K_p).
    pub fn passive_earth_force_n(phi_deg: f64, gamma_pa_m: f64, embedment: f64, length: f64, wall_friction_deg: f64) -> f64 {
        if embedment <= 0.0 || length <= 0.0 {
            return 0.0;
        }
        let kp = kp_coulomb(phi_deg, wall_friction_deg);
        0.5 * gamma_pa_m * embedment * embedment * kp * length
    }

    pub fn sliding_resistance_n(phi_deg: f64, c_pa: f64, v_n: f64, a: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        sliding_resistance_with_params(phi_deg, c_pa, v_n, a, &approach.annex_params(annex), 0.0, 0.0, 18_000.0, false)
    }

    pub fn sliding_resistance_with_params(
        phi_deg: f64, c_pa: f64, v_n: f64, a: f64, p: &AnnexParams,
        embedment: f64, length: f64, gamma_pa_m: f64, permanent_embedment: bool,
    ) -> f64 {
        let phi_d = (phi_deg.to_radians() / p.gamma_phi).atan();
        let c_d = c_pa / p.gamma_c;
        let base = c_d * a + v_n * phi_d.tan();
        let passive = if permanent_embedment && embedment > 0.0 {
            0.5 * passive_earth_force_n(phi_deg, gamma_pa_m, embedment, length, 0.0)
        } else {
            0.0
        };
        (base + passive) / p.gamma_r_h
    }

    /// 📐️ Vertical stress increase Δσ [Pa] at depth z below foundation base (2:1).
    pub fn stress_increase_2to1_pa(q_pa: f64, width: f64, length: f64, z: f64) -> f64 {
        let b = width.max(1e-6);
        let l = length.max(b);
        let zz = z.max(0.0);
        q_pa * (b * l) / ((b + zz) * (l + zz))
    }

    /// 📐️ Layer-wise oedometric settlement [m] (EN 1997-1 §6.6); returns (s, governing layer id).
    pub fn settlement_oedometric_m(layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_sls_pa: f64) -> (f64, String) {
        settlement_oedometric_with_gwl(layers, width, length, embedment, q_sls_pa, f64::INFINITY)
    }

    pub fn settlement_oedometric_with_gwl(
        layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_sls_pa: f64, gwl: f64,
    ) -> (f64, String) {
        let mut s = 0.0;
        let mut best_id = String::new();
        let mut best_contrib = 0.0;
        let gamma_w = 9_810.0;
        for layer in layers {
            let z_top = (layer.depth_top - embedment).max(0.0);
            let z_bot = (layer.depth_bottom - embedment).max(0.0);
            if z_bot <= z_top + 1e-12 {
                continue;
            }
            let dz = z_bot - z_top;
            let z_mid = 0.5 * (z_top + z_bot);
            let mut dsig = stress_increase_2to1_pa(q_sls_pa, width, length, z_mid);
            // When GWL is above the mid-layer depth, reduce to effective stress increment (buoyancy on overburden not double-counted on net q).
            let depth_abs = embedment + z_mid;
            if gwl < depth_abs {
                let gamma_eff = layer.gamma_prime.min((layer.gamma - gamma_w).max(0.0)).max(1.0);
                dsig *= (gamma_eff / layer.gamma.max(1.0)).clamp(0.4, 1.0);
            }
            let e = layer.oedometric_modulus.max(1.0);
            let contrib = dsig / e * dz;
            s += contrib;
            if contrib >= best_contrib {
                best_contrib = contrib;
                best_id = layer.id.clone();
            }
        }
        if best_id.is_empty() {
            if let Some(first) = layers.first() {
                best_id = first.id.clone();
                let e = first.oedometric_modulus.max(1.0);
                s = q_sls_pa / e * width.max(0.1) * 0.5;
            }
        }
        (s, best_id)
    }

    pub fn settlement_m(layers: &[SoilLayer], width: f64, q_pa: f64, _poisson: f64) -> f64 {
        settlement_oedometric_m(layers, width, width, 0.0, q_pa).0
    }

    pub fn shaft_resistance_n(alpha_s: f64, d: f64, q_s_pa: f64, l: f64) -> f64 {
        alpha_s * std::f64::consts::PI * d * q_s_pa * l
    }

    pub fn base_resistance_n(q_b_pa: f64, a_base: f64) -> f64 {
        q_b_pa * a_base
    }

    pub fn pile_correlation_factors(n_profiles: u32) -> (f64, f64) {
        match n_profiles {
            0 | 1 => (1.40, 1.40),
            2 => (1.35, 1.27),
            3 | 4 => {
                let t = (n_profiles - 2) as f64 / 3.0;
                (1.35 + t * (1.30 - 1.35), 1.27 + t * (1.15 - 1.27))
            }
            _ => (1.30, 1.15),
        }
    }

    pub fn pile_characteristic_resistance_n(mean: f64, min: f64, n: u32) -> f64 {
        let (xi3, xi4) = pile_correlation_factors(n);
        (mean / xi3).min(min / xi4)
    }

    pub fn pile_design_resistance_n(r_b_k: f64, r_s_k: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        pile_design_resistance_with_params(r_b_k, r_s_k, &approach.annex_params(annex))
    }

    pub fn pile_design_resistance_with_params(r_b_k: f64, r_s_k: f64, p: &AnnexParams) -> f64 {
        r_b_k / p.gamma_b + r_s_k / p.gamma_s
    }

    /// 📐️ Undrained Annex D.3 bearing pressure q_u = (π+2)·c_u·s_c·i_c + q [Pa].
    pub fn undrained_bearing_pa(c_u: f64, gamma: f64, b: f64, l: f64, embedment: f64, e_b: f64, h: f64, _v: f64) -> f64 {
        let b_p = effective_width(b, e_b);
        let l_p = effective_width(l, 0.0);
        let ratio = (b_p / l_p.max(b_p)).clamp(0.0, 1.0);
        let s_c = 1.0 + 0.2 * ratio;
        let a = b_p * l_p;
        let i_c = (1.0 - h.abs() / (a * c_u * (std::f64::consts::PI + 2.0) + 1e-6).max(1e-6)).clamp(0.0, 1.0);
        let q = gamma * embedment;
        (std::f64::consts::PI + 2.0) * c_u * s_c * i_c + q
    }

    pub fn undrained_bearing_resistance_n(c_u: f64, gamma: f64, b: f64, l: f64, embedment: f64, e_b: f64, h: f64, v: f64, p: &AnnexParams) -> f64 {
        let b_p = effective_width(b, e_b);
        let l_p = effective_width(l, 0.0);
        let q_u = undrained_bearing_pa(c_u / p.gamma_c.max(1.0), gamma / p.gamma_gamma, b, l, embedment, e_b, h, v);
        q_u * b_p * l_p / p.gamma_r_v
    }

    /// 📐️ Short-term undrained sliding R_d = A·c_u / γ_R,h [N].
    pub fn undrained_sliding_resistance_n(c_u: f64, a: f64, p: &AnnexParams) -> f64 {
        (c_u / p.gamma_c.max(1.0)) * a / p.gamma_r_h
    }

    /// 📐️ Immediate elastic settlement (EN 1997-1 Annex F.2 / DIN 4019): s = p·B·(1−ν²)·I / E.
    pub fn elastic_settlement_m(q_pa: f64, width: f64, length: f64, nu: f64, e_pa: f64, influence: f64) -> f64 {
        let e = e_pa.max(1.0);
        let b = width.max(0.1);
        let l = length.max(b);
        let i_f = influence.clamp(0.5, 1.5);
        let e_young = if nu > 0.0 && nu < 0.49 {
            e * (1.0 + nu) * (1.0 - 2.0 * nu) / (1.0 - nu).max(0.05)
        } else {
            e
        };
        q_pa * b * (1.0 - nu * nu) * i_f / e_young.max(1.0) * (b / l).sqrt().clamp(0.7, 1.0)
    }

    /// 📐️ Layered immediate settlement — each layer contributes with its own ν and E_oed.
    pub fn elastic_settlement_layered(layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_pa: f64) -> (f64, String) {
        let mut s = 0.0;
        let mut best_id = String::new();
        let mut best = 0.0;
        let b = width.max(0.1);
        let z_limit = layers.iter().map(|l| (l.depth_bottom - embedment).max(0.0)).fold(2.0 * b, f64::max);
        for layer in layers {
            let z_top = (layer.depth_top - embedment).max(0.0);
            if z_top > z_limit { continue; }
            let z_bot = (layer.depth_bottom - embedment).max(z_top).min(z_limit);
            let dz = (z_bot - z_top).max(0.0);
            if dz < 1e-9 { continue; }
            let z_mid = z_top + 0.5 * dz;
            let i_f = (1.0 / (1.0 + z_mid / b)).clamp(0.2, 1.0);
            let stress = q_pa * (b / (b + z_mid)).powi(2).clamp(0.05, 1.0);
            let ds = elastic_settlement_m(stress, width, length, layer.poisson_ratio, layer.oedometric_modulus, i_f) * (dz / (2.0 * b)).clamp(0.02, 1.0);
            if ds >= best {
                best = ds;
                best_id = layer.id.clone();
            }
            s += ds;
        }
        if best_id.is_empty() {
            if let Some(l) = layers.first() { best_id = l.id.clone(); }
        }
        (s, best_id)
    }

    /// 📐️ DIN 1054 / EA-Pfähle pile-type factors (γ_b, γ_s, ξ3, ξ4) for compression.
    pub fn pile_type_factors(pile_type: &str, n_profiles: u32, base: &AnnexParams) -> (f64, f64, f64, f64) {
        let (xi3, xi4) = pile_correlation_factors(n_profiles);
        let key = pile_type.trim().to_ascii_lowercase();
        match key.as_str() {
            "driven" | "ramm" | "rammpfahl" => {
                let (xi3, xi4) = match n_profiles {
                    0 | 1 => (1.35, 1.35),
                    2 => (1.30, 1.25),
                    _ => (1.25, 1.15),
                };
                (base.gamma_b.max(1.10), base.gamma_s.max(1.15), xi3, xi4)
            }
            "cfa" | "sob" | "schnecke" => (base.gamma_b.max(1.15), base.gamma_s.max(1.15), xi3.max(1.40), xi4.max(1.40)),
            _ => (base.gamma_b.max(1.10), base.gamma_s.max(1.10), xi3, xi4), // bored
        }
    }

    pub fn pile_design_resistance_typed(r_b_k: f64, r_s_k: f64, gamma_b: f64, gamma_s: f64) -> f64 {
        r_b_k / gamma_b + r_s_k / gamma_s
    }

    pub fn is_cohesive_layer(layer: &SoilLayer) -> bool {
        let t = layer.soil_type.to_ascii_lowercase();
        layer.cohesion_undrained > 1.0
            || t.contains("clay")
            || t.contains("ton")
            || t.contains("cohesive")
    }

    /// 📐️ Rankine active earth-pressure coefficient K_a = tan²(45°−φ′/2).
    pub fn ka_rankine(phi_deg: f64) -> f64 {
        let alpha = std::f64::consts::FRAC_PI_4 - phi_deg.to_radians() / 2.0;
        alpha.tan().powi(2).max(0.05)
    }

    /// 📐️ Rankine passive K_p = tan²(45°+φ′/2).
    pub fn kp_rankine(phi_deg: f64) -> f64 {
        let alpha = std::f64::consts::FRAC_PI_4 + phi_deg.to_radians() / 2.0;
        alpha.tan().powi(2).min(20.0)
    }

    /// 📐️ Jáky at-rest coefficient K₀ = (1−sin φ′)·OCR^sin φ′.
    pub fn k0_jaky(phi_deg: f64, ocr: f64) -> f64 {
        let phi = phi_deg.to_radians();
        let k0_nc = (1.0 - phi.sin()).max(0.15);
        k0_nc * ocr.max(1.0).powf(phi.sin())
    }

    /// 📐️ Select earth-pressure coefficient by DIN 4085 wall-movement regime.
    pub fn earth_pressure_coefficient(phi_deg: f64, mode: &str, ocr: f64, delta_deg: f64) -> (f64, &'static str) {
        let key = mode.trim().to_ascii_lowercase().replace(['-', '_', ' '], "");
        match key.as_str() {
            "atrest" | "k0" | "ruhe" => (k0_jaky(phi_deg, ocr), "K0"),
            "increasedactive" | "erhohteaktiv" | "erhöhtaktiv" | "increased" => {
                let ka = ka_rankine(phi_deg);
                let k0 = k0_jaky(phi_deg, ocr);
                (0.5 * (ka + k0), "Ka*")
            }
            _ => {
                // Prefer Rankine for smooth; blend Coulomb wall friction when δ provided.
                if delta_deg.abs() > 1e-6 {
                    (ka_coulomb(phi_deg, delta_deg), "Ka-Coulomb")
                } else {
                    (ka_rankine(phi_deg), "Ka-Rankine")
                }
            }
        }
    }

    pub fn ka_coulomb(phi_deg: f64, delta_deg: f64) -> f64 {
        let phi = phi_deg.to_radians();
        let delta = delta_deg.to_radians();
        let num = (phi.cos()).powi(2);
        let den = ((phi.cos() + (phi.sin() * (phi - delta).sin().max(0.0) / (delta.cos()).max(1e-6)).sqrt()).max(1e-6)).powi(2);
        (num / den / phi.cos().max(1e-6)).max(0.1)
    }

    pub fn kp_coulomb(phi_deg: f64, delta_deg: f64) -> f64 {
        let ka = ka_coulomb(phi_deg, delta_deg);
        (1.0 / ka).min(20.0)
    }

    fn layer_props_at(layers: &[SoilLayer], depth: f64) -> (f64, f64, f64, f64) {
        let layer = layers.iter().find(|l| depth >= l.depth_top && depth <= l.depth_bottom).or_else(|| layers.first());
        match layer {
            Some(l) => (l.phi_prime_deg, l.cohesion_effective, l.gamma, l.gamma_prime),
            None => (30.0, 0.0, 18_000.0, 10_000.0),
        }
    }

    /// 📐️ Simplified Bishop FoS with slice equilibrium + circular slip search (EN 1997-1 §11).
    /// Overall stability partial factors are GEO-3 (EN 1997-1 §2.4.7.3.4 and DIN 1054 National Annex), independent of the project design approach.
    /// 📐️ Returns None when `governing_layer_id` is empty or not found among layers.
    pub fn bishop_fos_slices(
        layers: &[SoilLayer], governing_layer_id: &str, height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64,
    ) -> Option<f64> {
        if governing_layer_id.trim().is_empty() || !layers.iter().any(|l| l.id == governing_layer_id) {
            return None;
        }
        Some(bishop_fos_slices_inner(layers, height, angle_deg, length, gwl, gamma_phi, gamma_c))
    }

    fn bishop_fos_slices_inner(
        layers: &[SoilLayer], height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64,
    ) -> f64 {
        let beta = angle_deg.to_radians().clamp(0.05, 1.45);
        let horiz = if length > 0.5 {
            length.max(height / beta.tan().max(0.05))
        } else {
            height / beta.tan().max(0.05)
        };
        let n_slices = 16usize;
        let mut best = f64::INFINITY;
        let toe_x = 0.0;
        let crest_x = horiz;
        for &xc_frac in &[0.25, 0.4, 0.55, 0.7, 0.85] {
            for &yc_frac in &[0.8, 1.1, 1.4, 1.8, 2.2] {
                let xc = toe_x + xc_frac * (crest_x - toe_x);
                let yc = height * yc_frac;
                let r_toe = ((xc - toe_x).hypot(yc - 0.0)).max(height * 0.5);
                let r_crest = ((xc - crest_x).hypot(yc - height)).max(height * 0.5);
                for &r in &[r_toe, r_crest, 0.5 * (r_toe + r_crest), r_toe * 1.15, r_crest * 1.15] {
                    let fos = bishop_one_circle(layers, height, beta, horiz, xc, yc, r, gwl, gamma_phi, gamma_c, n_slices);
                    if fos.is_finite() && fos > 0.05 && fos < 50.0 {
                        best = best.min(fos);
                    }
                }
            }
        }
        if best.is_finite() {
            best
        } else {
            let (phi, c, gamma, _) = layer_props_at(layers, height * 0.5);
            bishop_fos(phi, c, gamma, height, angle_deg, gamma_phi, gamma_c)
        }
    }

    fn bishop_one_circle(
        layers: &[SoilLayer], height: f64, beta: f64, horiz: f64, xc: f64, yc: f64, r: f64, gwl: f64,
        gamma_phi: f64, gamma_c: f64, n_slices: usize,
    ) -> f64 {
        let gamma_w = 9_810.0;
        let dx = horiz / n_slices as f64;
        let mut f = 1.5_f64;
        for _ in 0..40 {
            let mut num = 0.0;
            let mut den = 0.0;
            let mut used = 0usize;
            for i in 0..n_slices {
                let x = (i as f64 + 0.5) * dx;
                // Slope face from crest (0,height) to toe (horiz,0) at angle beta.
                let y_surface = (height - x * beta.tan()).max(0.0);
                if x > horiz {
                    continue;
                }
                let under = r * r - (x - xc).powi(2);
                if under <= 0.0 {
                    continue;
                }
                let y_slip = yc - under.sqrt();
                if y_slip >= y_surface - 1e-9 {
                    continue;
                }
                let h_slice = (y_surface - y_slip).max(0.0);
                if h_slice < 1e-6 {
                    continue;
                }
                let mid_depth = (height - 0.5 * (y_surface + y_slip)).max(0.0);
                let (phi, c, gamma, gamma_p) = layer_props_at(layers, mid_depth);
                let phi_d = (phi.to_radians().tan() / gamma_phi).atan();
                let c_d = c / gamma_c;
                let slip_depth_from_crest = height - y_slip;
                let submerged = gwl < slip_depth_from_crest;
                let gamma_eff = if submerged { gamma_p.max(1.0) } else { gamma };
                let b = dx;
                let w = gamma_eff * h_slice * b;
                let u = if submerged {
                    gamma_w * (gwl - (height - 0.5 * (y_surface + y_slip))).max(0.0)
                } else {
                    0.0
                };
                let alpha = ((x - xc) / r).asin().clamp(-1.2, 1.2);
                let m_alpha = (alpha.cos() + alpha.sin() * phi_d.tan() / f.max(0.1)).max(0.05);
                num += (c_d * b + (w - u * b).max(0.0) * phi_d.tan()) / m_alpha;
                den += w * alpha.sin();
                used += 1;
            }
            if used < n_slices / 3 || den.abs() < 1e-6 {
                return f64::NAN;
            }
            let f_new = num / den;
            if !f_new.is_finite() || f_new <= 0.0 {
                return f64::NAN;
            }
            if (f_new - f).abs() < 1e-4 {
                return f_new.clamp(0.05, 50.0);
            }
            f = 0.5 * (f + f_new);
        }
        f.clamp(0.05, 50.0)
    }

    /// 📐️ Legacy closed-form Bishop (fallback only).
    pub fn bishop_fos(phi_deg: f64, c_pa: f64, gamma_pa_m: f64, height: f64, angle_deg: f64, gamma_phi: f64, gamma_c: f64) -> f64 {
        let phi_d = (phi_deg.to_radians().tan() / gamma_phi).atan();
        let c_d = c_pa / gamma_c;
        let beta = angle_deg.to_radians();
        let weight = gamma_pa_m * height * height / (2.0 * beta.tan().max(0.2));
        let resisting = c_d * height / beta.sin().max(0.2) + weight * phi_d.tan() * beta.cos() / beta.sin().max(0.2);
        let driving = weight * beta.sin().max(0.05);
        resisting / driving.max(1.0)
    }

    /// 📐️ DIN 1054 Table A.6.10-style presumptive design bearing pressure [Pa] for GK1.
    pub fn presumptive_bearing_pa(soil_type: &str, embedment: f64) -> f64 {
        let base = match soil_type.to_ascii_lowercase().as_str() {
            "gravel" | "kies" => 400_000.0,
            "sand" | "sandig" => 200_000.0,
            "silt" | "schluff" => 150_000.0,
            "clay" | "ton" => 120_000.0,
            _ => 180_000.0,
        };
        base * (1.0 + 0.1 * embedment.min(3.0))
    }
}

pub mod part_2 {
    use super::*;
    /// 📐️ φ′ from CPT tip resistance (EN 1997-2 / Robertson-type correlation, cautious).
    pub fn phi_from_cpt_deg(q_c_pa: f64, sigma_v0_pa: f64) -> f64 {
        let ratio = (q_c_pa / sigma_v0_pa.max(1.0)).max(1.0);
        (27.0 + 10.0 * (ratio.log10() - 0.5)).clamp(28.0, 42.0)
    }
    /// 📐️ φ′ from SPT blow count N (EN 1997-2 empirical correlation, cautious).
    pub fn phi_from_spt_deg(n_spt: f64) -> f64 {
        (27.1 + 0.3 * n_spt - 0.00054 * n_spt * n_spt).clamp(25.0, 42.0)
    }
    pub fn min_investigation_depth_m(b: f64) -> f64 { 3.0 * b }
    pub fn min_investigation_depth_for_gk(b: f64, gk: u8) -> f64 {
        match gk {
            1 => 2.0 * b,
            3 => 3.5 * b,
            _ => 3.0 * b,
        }
    }
    /// 📐️ Characteristic φ′ from investigation: min(stated, CPT/SPT-derived when data present).
    /// 📐️ Investigation-only φ′ from CPT/SPT correlations (EN 1997-2); ignores stated φ′.
    pub fn investigation_phi_deg(layer: &SoilLayer, overburden_pa: f64) -> Option<(f64, &'static str)> {
        let mut best: Option<(f64, &'static str)> = None;
        if layer.cpt_qc > 0.0 {
            let derived = phi_from_cpt_deg(layer.cpt_qc, overburden_pa.max(50_000.0));
            best = Some((derived, "cpt"));
        }
        if layer.spt_n > 0.0 {
            let derived = phi_from_spt_deg(layer.spt_n);
            best = match best {
                Some((phi, _)) if derived + 1e-9 < phi => Some((derived, "spt")),
                Some(other) => Some(other),
                None => Some((derived, "spt")),
            };
        }
        best
    }

    /// 📐️ Cautious characteristic φ′ = min(stated, investigation) per EN 1997-1 §2.4.5.2.
    pub fn characteristic_phi_deg(layer: &SoilLayer, overburden_pa: f64) -> (f64, &'static str) {
        let mut phi = layer.phi_prime_deg;
        let mut src = "stated";
        if let Some((derived, dsrc)) = investigation_phi_deg(layer, overburden_pa) {
            if derived + 1e-9 < phi {
                phi = derived;
                src = dsrc;
            }
        }
        (phi, src)
    }
}


fn layer_at(layers: &[SoilLayer], depth: f64) -> Option<&SoilLayer> {
    layers.iter().find(|l| depth >= l.depth_top && depth <= l.depth_bottom).or_else(|| layers.first())
}

fn design_actions(p: &AnnexParams, g: f64, q: f64) -> f64 { p.gamma_g * g + p.gamma_q * q }

fn find_required_width(
    footing: &SpreadFoundation, layer: &SoilLayer, lc: &FoundationLoadCase,
    approach: DesignApproach, annex: AnnexChoice, situation: &str, gwl: f64, start: f64,
) -> f64 {
    let p = resolve_params(approach, annex, situation);
    let mut b = start.max(0.3);
    for _ in 0..40 {
        let v = design_actions(&p, lc.vertical_permanent, lc.vertical_variable);
        let h = design_actions(&p, lc.horizontal_permanent, lc.horizontal_variable);
        let m = design_actions(&p, lc.moment_permanent, lc.moment_variable);
        let e = m / v.max(1.0);
        let gamma_b = part_1::effective_gamma_below_pa_m(layer.gamma, layer.gamma_prime, footing.embedment, gwl, b);
        let r = part_1::design_bearing_resistance_with_params(
            layer.phi_prime_deg, layer.cohesion_effective, layer.gamma, gamma_b, b, footing.length.max(b), footing.embedment,
            e, 0.0, h, v, footing.base_inclination_deg, &p,
        );
        if v <= r { return (b * 1000.0).ceil() / 1000.0; }
        b *= 1.08;
        if b > 30.0 { break; }
    }
    b
}


fn situation_short(situation: &str) -> &'static str {
    match parse_design_situation(situation) {
        DesignSituation::Transient => "BS-T",
        DesignSituation::Accidental => "BS-A",
        DesignSituation::Persistent => "BS-P",
    }
}

fn situation_for_load_case(doc: &En1997Snapshot, lc: &FoundationLoadCase) -> String {
    if !lc.design_situation.trim().is_empty() {
        lc.design_situation.clone()
    } else {
        doc.design_situation.clone()
    }
}

/// 🔗 Fail when an id-bearing collection contains duplicate entity ids.
fn push_duplicate_ids(
    report: &mut CheckReport,
    annex: AnnexChoice,
    table: &str,
    table_en: &str,
    table_de: &str,
    ids: &[String],
    path_for: &dyn Fn(&str) -> String,
) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = subject_entity(
            &id,
            &path,
            &format!("Duplicate {table_en} id"),
            &format!("Doppelte {table_de}-Id"),
        );
        let free = format!("{id}-2");
        report.push(
            CheckResult::assess(
                format!("en1997.integrity.duplicate.{table}.{id}"),
                "EN 1997 integrity",
                ClauseId::new("EN 1997-1", "1", "1.5"),
                subject.clone(),
                loc(
                    &format!("Unique {table_en} id"),
                    &format!("Eindeutige {table_de}-Id"),
                ),
            )
            .annex(annex)
            .explanation(loc(
                &format!("Duplicate {table_en} id '{id}' appears {count} times; each entity id must be unique."),
                &format!("Doppelte {table_de}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
            ))
            .status(CheckStatus::Fail)
            .remedy(Remedy::one_of(
                subject,
                vec![free.clone(), format!("{id}-unique")],
                loc(
                    &format!("Rename the duplicated '{id}' entry to a free id such as '{free}'."),
                    &format!("Den doppelten '{id}'-Eintrag auf eine freie Id wie '{free}' umbenennen."),
                ),
            ))
            .build(),
        );
    }
}

fn push_all_duplicate_ids(report: &mut CheckReport, doc: &En1997Snapshot) {
    let annex = doc.annex;
    push_duplicate_ids(
        report,
        annex,
        "layers",
        "layer",
        "Schicht",
        &doc.layers.iter().map(|l| l.id.clone()).collect::<Vec<_>>(),
        &|id| format!("layers[id={id}].id"),
    );
    push_duplicate_ids(
        report,
        annex,
        "footings",
        "footing",
        "Fundament",
        &doc.footings.iter().map(|f| f.id.clone()).collect::<Vec<_>>(),
        &|id| format!("footings[id={id}].id"),
    );
    for footing in &doc.footings {
        push_duplicate_ids(
            report,
            annex,
            "loadCases",
            "load case",
            "Lastfall",
            &footing.load_cases.iter().map(|lc| lc.id.clone()).collect::<Vec<_>>(),
            &|id| format!("footings[id={}].loadCases[id={id}].id", footing.id),
        );
    }
    push_duplicate_ids(
        report,
        annex,
        "piles",
        "pile",
        "Pfahl",
        &doc.piles.iter().map(|p| p.id.clone()).collect::<Vec<_>>(),
        &|id| format!("piles[id={id}].id"),
    );
    for pile in &doc.piles {
        push_duplicate_ids(
            report,
            annex,
            "testProfiles",
            "test profile",
            "Prüfprofil",
            &pile.test_profiles.iter().map(|t| t.id.clone()).collect::<Vec<_>>(),
            &|id| format!("piles[id={}].testProfiles[id={id}].id", pile.id),
        );
    }
    push_duplicate_ids(
        report,
        annex,
        "retainingWalls",
        "retaining wall",
        "Stützwand",
        &doc.retaining_walls.iter().map(|w| w.id.clone()).collect::<Vec<_>>(),
        &|id| format!("retainingWalls[id={id}].id"),
    );
    push_duplicate_ids(
        report,
        annex,
        "slopes",
        "slope",
        "Böschung",
        &doc.slopes.iter().map(|s| s.id.clone()).collect::<Vec<_>>(),
        &|id| format!("slopes[id={id}].id"),
    );
    push_duplicate_ids(
        report,
        annex,
        "upliftCases",
        "uplift case",
        "Auftriebslastfall",
        &doc.uplift_cases.iter().map(|u| u.id.clone()).collect::<Vec<_>>(),
        &|id| format!("upliftCases[id={id}].id"),
    );
}

fn push_governing_situation_summary(
    report: &mut CheckReport,
    id: String,
    part: &str,
    annex: AnnexChoice,
    subject: SubjectRef,
    sit: &str,
    approach: &str,
    util: f64,
    governing_check: &str,
) {
    let sit_lbl = situation_short(sit);
    report.push(
        CheckResult::assess(
            id,
            part,
            ClauseId::new("EN 1997-1", "2", "2.4.7"),
            subject,
            loc("Governing design situation", "Maßgebende Bemessungssituation"),
        )
        .utilization(
            Quantity::new(QuantityKind::Dimensionless, util),
            Quantity::new(QuantityKind::Dimensionless, util.max(1.0)),
        )
        .annex(annex)
        .explanation(loc(
            &format!("Governing: {sit_lbl} ({sit}) + approach {approach}; check {governing_check}; u={util:.3}."),
            &format!("Maßgebend: {sit_lbl} ({sit}) + Ansatz {approach}; Nachweis {governing_check}; u={util:.3}."),
        ))
        .build(),
    );
}

fn id_path(collection: &str, id: &str, field: &str) -> String {
    format!("{collection}[id={id}].{field}")
}

pub fn check_project(doc: &En1997Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    push_all_duplicate_ids(&mut report, doc);
    let approach = parse_design_approach(&doc.design_approach);
    let annex = doc.annex;
    let p_project = resolve_params(approach, annex, &doc.design_situation);
    let upl_p = resolve_upl_params(annex, &doc.design_situation);
    let part1 = "DIN EN 1997-1";
    let part2 = "DIN EN 1997-2";
    let part1054 = "DIN 1054";
    let gwl = doc.groundwater_level;
    let gk = doc.geotechnical_category;
    let mut design_situations: Vec<String> = vec![doc.design_situation.clone()];
    for footing in doc.footings.iter() {
        for lc in footing.load_cases.iter() {
            let sit = situation_for_load_case(doc, lc);
            if !design_situations.iter().any(|s| s == &sit) {
                design_situations.push(sit);
            }
        }
    }
    if design_situations.is_empty() {
        design_situations.push("bsP".into());
    }


    // Geotechnical category applicability (DIN 1054 / DIN 4020).
    {
        let gk_ok = (1..=3).contains(&gk);
        let mut gk_check = CheckResult::assess(
            "en1997.gk.category",
            part1054,
            ClauseId::new("DIN 1054", "A", "A.2"),
            subject_entity("", "geotechnicalCategory", "Geotechnical category", "Geotechnische Kategorie"),
            loc("Geotechnical category GK", "Geotechnische Kategorie GK"),
        )
        .annex(annex)
        .explanation(loc(
            &format!("GK={gk} (1=simple, 2=standard, 3=complex)."),
            &format!("GK={gk} (1=einfach, 2=üblich, 3=schwierig)."),
        ));
        if !gk_ok {
            gk_check = gk_check.utilization(Quantity::new(QuantityKind::Dimensionless, gk as f64), Quantity::new(QuantityKind::Dimensionless, 2.0))
                .remedy(Remedy::one_of(
                    subject_entity("", "geotechnicalCategory", "Geotechnical category", "Geotechnische Kategorie"),
                    vec!["1".into(), "2".into(), "3".into()],
                    loc("Set geotechnical category to 1, 2, or 3.", "Geotechnische Kategorie auf 1, 2 oder 3 setzen."),
                ));
        } else {
            gk_check = gk_check.utilization(Quantity::new(QuantityKind::Dimensionless, 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0));
        }
        report.push(gk_check.build());
    }

    for layer in doc.layers.iter() {
        let path_phi = format!("layers[id={}].phiPrimeDeg", layer.id);
        let path_cu = format!("layers[id={}].cohesionUndrained", layer.id);
        let path_c = format!("layers[id={}].cohesionEffective", layer.id);
        let thick = (layer.depth_bottom - layer.depth_top).max(0.1);
        let overburden = layer.gamma * ((layer.depth_top + layer.depth_bottom) * 0.5).max(0.5);
        let cohesive = part_1::is_cohesive_layer(layer);
        let (computed, limit, path, title) = if cohesive {
            let cu_req = 0.15 * overburden;
            (layer.cohesion_undrained.max(0.0), cu_req.max(1.0), path_cu.clone(), loc("Layer undrained strength", "Schicht undränierte Festigkeit"))
        } else {
            let idx = layer.phi_prime_deg + layer.cohesion_effective / 1000.0;
            (idx, 20.0_f64, path_c.clone(), loc("Layer drained strength φ′/c′", "Schicht dränierte Festigkeit φ′/c′"))
        };
        let mut chk = CheckResult::assess(
            format!("en1997.layer.strength.{}", layer.id),
            part1,
            ClauseId::new("EN 1997-1", "3", "3.3"),
            subject_entity(&layer.id, &path, &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
            title,
        )
        .minimum(Quantity::new(QuantityKind::Dimensionless, computed), Quantity::new(QuantityKind::Dimensionless, limit))
        .annex(annex)
        .explanation(loc(
            &format!("Layer {} ({}) thickness={:.1} m, φ′={:.1}°, c_u={:.0} kPa, γ={:.0}.", layer.id, layer.soil_type, thick, layer.phi_prime_deg, layer.cohesion_undrained/1000.0, layer.gamma),
            &format!("Schicht {} ({}) d={:.1} m, φ′={:.1}°, c_u={:.0} kPa, γ={:.0}.", layer.id, layer.soil_type, thick, layer.phi_prime_deg, layer.cohesion_undrained/1000.0, layer.gamma),
        ));
        if computed < limit {
            if cohesive {
                chk = chk.remedy(Remedy::at_least(
                    subject_entity(&layer.id, &path_cu, &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
                    Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained),
                    Quantity::new(QuantityKind::Pressure, limit),
                    loc("Increase undrained cohesion for this cohesive layer.", "Undränierte Kohäsion dieser bindigen Schicht erhöhen."),
                ));
            } else {
                let c_need = ((limit - layer.phi_prime_deg).max(0.0) * 1000.0).max(layer.cohesion_effective);
                chk = chk.remedy(Remedy::at_least(
                    subject_entity(&layer.id, &path_c, &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
                    Quantity::new(QuantityKind::Pressure, layer.cohesion_effective),
                    Quantity::new(QuantityKind::Pressure, c_need),
                    loc("Increase effective cohesion for this drained layer.", "Effektive Kohäsion dieser dränierten Schicht erhöhen."),
                )).remedy(Remedy::at_least(
                    subject_entity(&layer.id, &path_phi, &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
                    Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                    Quantity::new(QuantityKind::Dimensionless, limit),
                    loc("Increase φ′ for this drained layer.", "φ′ dieser dränierten Schicht erhöhen."),
                ));
            }
        }
        report.push(chk.build());
        if layer.cpt_qc > 0.0 || layer.spt_n > 0.0 {
            let overburden = layer.gamma * ((layer.depth_top + layer.depth_bottom) * 0.5).max(0.5);
            let path_phi = format!("layers[id={}].phiPrimeDeg", layer.id);
            let path_cpt = format!("layers[id={}].cptQc", layer.id);
            let path_spt = format!("layers[id={}].sptN", layer.id);
            let label_l = loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id));
            if layer.cpt_qc > 0.0 {
                let phi_cpt = part_2::phi_from_cpt_deg(layer.cpt_qc, overburden.max(50_000.0));
                let mut cpt_chk = CheckResult::assess(
                    format!("en1997.2.phi.cpt.{}", layer.id),
                    part2,
                    ClauseId::new("EN 1997-2", "2", "2.4.3"),
                    subject_entity(&layer.id, &path_cpt, &label_l.en, &label_l.de),
                    loc("φ′ vs CPT correlation (EN 1997-2)", "φ′ vs CPT-Korrelation (EN 1997-2)"),
                )
                .utilization(
                    Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                    Quantity::new(QuantityKind::Dimensionless, phi_cpt.max(1e-6)),
                )
                .annex(annex)
                .explanation(loc(
                    &format!("φ′_stated={:.1}° vs φ′_CPT={:.1}° (qc={:.2} MPa, σ′v0={:.0} kPa).", layer.phi_prime_deg, phi_cpt, layer.cpt_qc/1e6, overburden/1000.0),
                    &format!("φ′_angegeben={:.1}° gegen φ′_CPT={:.1}° (qc={:.2} MPa, σ′v0={:.0} kPa).", layer.phi_prime_deg, phi_cpt, layer.cpt_qc/1e6, overburden/1000.0),
                ));
                if layer.phi_prime_deg > phi_cpt + 1e-9 {
                    cpt_chk = cpt_chk.status(CheckStatus::Fail).remedy(Remedy::at_most(
                        subject_entity(&layer.id, &path_phi, &label_l.en, &label_l.de),
                        Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                        Quantity::new(QuantityKind::Dimensionless, phi_cpt),
                        loc(
                            &format!("Reduce stated φ′ to ≤ {:.1}° (CPT correlation, EN 1997-1 §2.4.5.2).", phi_cpt),
                            &format!("Angegebenen φ′ auf ≤ {:.1}° absenken (CPT-Korrelation, EN 1997-1 §2.4.5.2).", phi_cpt),
                        ),
                    ));
                }
                report.push(cpt_chk.build());
            }
            if layer.spt_n > 0.0 {
                let phi_spt = part_2::phi_from_spt_deg(layer.spt_n);
                let mut spt_chk = CheckResult::assess(
                    format!("en1997.2.phi.spt.{}", layer.id),
                    part2,
                    ClauseId::new("EN 1997-2", "2", "2.4.3"),
                    subject_entity(&layer.id, &path_spt, &label_l.en, &label_l.de),
                    loc("φ′ vs SPT correlation (EN 1997-2)", "φ′ vs SPT-Korrelation (EN 1997-2)"),
                )
                .utilization(
                    Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                    Quantity::new(QuantityKind::Dimensionless, phi_spt.max(1e-6)),
                )
                .annex(annex)
                .explanation(loc(
                    &format!("φ′_stated={:.1}° vs φ′_SPT={:.1}° (N={:.0}).", layer.phi_prime_deg, phi_spt, layer.spt_n),
                    &format!("φ′_angegeben={:.1}° gegen φ′_SPT={:.1}° (N={:.0}).", layer.phi_prime_deg, phi_spt, layer.spt_n),
                ));
                if layer.phi_prime_deg > phi_spt + 1e-9 {
                    spt_chk = spt_chk.status(CheckStatus::Fail).remedy(Remedy::at_most(
                        subject_entity(&layer.id, &path_phi, &label_l.en, &label_l.de),
                        Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                        Quantity::new(QuantityKind::Dimensionless, phi_spt),
                        loc(
                            &format!("Reduce stated φ′ to ≤ {:.1}° (SPT correlation, EN 1997-1 §2.4.5.2).", phi_spt),
                            &format!("Angegebenen φ′ auf ≤ {:.1}° absenken (SPT-Korrelation, EN 1997-1 §2.4.5.2).", phi_spt),
                        ),
                    ));
                }
                report.push(spt_chk.build());
            }
            let (phi_char, phi_src) = part_2::investigation_phi_deg(layer, overburden)
                .unwrap_or((layer.phi_prime_deg, "stated"));
            let mut phi_chk = CheckResult::assess(
                format!("en1997.2.phi.derived.{}", layer.id),
                part2,
                ClauseId::new("EN 1997-2", "2", "2.4.3"),
                subject_entity(&layer.id, &path_phi, &label_l.en, &label_l.de),
                loc("Characteristic φ′ vs investigation (EN 1997-1 §2.4.5.2)", "Charakteristischer φ′ vs Erkundung (EN 1997-1 §2.4.5.2)"),
            )
            .utilization(
                Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                Quantity::new(QuantityKind::Dimensionless, phi_char.max(1e-6)),
            )
            .annex(annex)
            .explanation(loc(
                &format!("φ′_stated={:.1}° vs φ′_char={:.1}° from {phi_src} (EN 1997-2 CPT/SPT; qc={:.0} MPa, N={:.0}).", layer.phi_prime_deg, phi_char, layer.cpt_qc/1e6, layer.spt_n),
                &format!("φ′_angegeben={:.1}° gegen φ′_char={:.1}° aus {phi_src} (EN 1997-2 CPT/SPT; qc={:.0} MPa, N={:.0}).", layer.phi_prime_deg, phi_char, layer.cpt_qc/1e6, layer.spt_n),
            ));
            if layer.phi_prime_deg > phi_char + 1e-9 {
                phi_chk = phi_chk.status(CheckStatus::Fail).remedy(Remedy::at_most(
                    subject_entity(&layer.id, &path_phi, &label_l.en, &label_l.de),
                    Quantity::new(QuantityKind::Dimensionless, layer.phi_prime_deg),
                    Quantity::new(QuantityKind::Dimensionless, phi_char),
                    loc(
                        &format!("Reduce stated φ′ to ≤ {:.1}° (cautious estimate from {phi_src}, EN 1997-1 §2.4.5.2).", phi_char),
                        &format!("Angegebenen φ′ auf ≤ {:.1}° absenken (vorsichtiger Schätzwert aus {phi_src}, EN 1997-1 §2.4.5.2).", phi_char),
                    ),
                ));
            }
            report.push(phi_chk.build());
        }
    }

    if doc.footings.is_empty() {
        report.push(
            CheckResult::assess("en1997.footing.na", part1, ClauseId::new("EN 1997-1", "6", "6.5"), SubjectRef::whole(loc("Project", "Projekt")), loc("Spread foundation checks", "Flachgründungsnachweise"))
                .not_applicable(loc("No spread foundations in the subject.", "Keine Flachgründungen im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }

    for footing in doc.footings.iter() {
        let layer = layer_at(&doc.layers, footing.embedment).cloned().unwrap_or_else(|| SoilLayer {
            id: "default".into(), soil_type: "sand".into(), depth_top: 0.0, depth_bottom: 10.0,
            gamma: 18_000.0, gamma_prime: 10_000.0, phi_prime_deg: 30.0, cohesion_effective: 0.0, cohesion_undrained: 0.0,
            oedometric_modulus: 30e6, poisson_ratio: 0.3, cpt_qc: 0.0, spt_n: 0.0,
        });
        let overburden = layer.gamma * footing.embedment.max(0.5);
        let (phi_char, _phi_src) = part_2::characteristic_phi_deg(&layer, overburden);
        let path_w = id_path("footings", &footing.id, "width");
        let label = loc(&format!("Footing {}", footing.id), &format!("Fundament {}", footing.id));

        if footing.load_cases.is_empty() {
            report.push(
                CheckResult::assess(format!("en1997.footing.{}.loads.na", footing.id), part1, ClauseId::new("EN 1997-1", "6", "6.5"), subject_entity(&footing.id, &path_w, &label.en, &label.de), loc("Foundation actions", "Einwirkungen auf das Fundament"))
                    .not_applicable(loc("No load cases on this footing.", "Keine Lastfälle für dieses Fundament."))
                    .annex(annex)
                    .build(),
            );
        }

        let gamma_below = part_1::effective_gamma_below_pa_m(layer.gamma, layer.gamma_prime, footing.embedment, gwl, footing.width);
        let phi_use = phi_char;
        let mut foot_gov_u = -1.0f64;
        let mut foot_gov_sit = doc.design_situation.clone();
        let mut foot_gov_check = String::new();

        for lc in footing.load_cases.iter() {
            let sit = situation_for_load_case(doc, lc);
            let p = resolve_params(approach, annex, &sit);
            let v_d = design_actions(&p, lc.vertical_permanent, lc.vertical_variable);
            let h_d = design_actions(&p, lc.horizontal_permanent, lc.horizontal_variable);
            let m_d = design_actions(&p, lc.moment_permanent, lc.moment_variable);
            let e_b = m_d / v_d.max(1.0);
            let a = footing.width * footing.length;

            let sit_kind = parse_design_situation(&sit);
            let cohesive = part_1::is_cohesive_layer(&layer);
            let short_term = matches!(sit_kind, DesignSituation::Transient | DesignSituation::Accidental);
            let use_undrained = cohesive && short_term && layer.cohesion_undrained > 0.0;
            let r_v = if gk == 1 {
                let q_all = part_1::presumptive_bearing_pa(&layer.soil_type, footing.embedment);
                q_all * a / p.gamma_r_v
            } else if use_undrained {
                part_1::undrained_bearing_resistance_n(
                    layer.cohesion_undrained, gamma_below, footing.width, footing.length, footing.embedment, e_b, h_d, v_d, &p,
                )
            } else {
                part_1::design_bearing_resistance_with_params(
                    phi_use, layer.cohesion_effective, layer.gamma, gamma_below, footing.width, footing.length, footing.embedment,
                    e_b, 0.0, h_d, v_d, footing.base_inclination_deg, &p,
                )
            };
            let b_req = find_required_width(footing, &layer, lc, approach, annex, &sit, gwl, footing.width);
            let mut bearing = CheckResult::assess(
                format!("en1997.6.5.bearing.{}.{}", footing.id, lc.id),
                if gk == 1 { part1054 } else { part1 },
                ClauseId::new(if gk == 1 { "DIN 1054" } else { "EN 1997-1" }, "6", if gk == 1 { "A.6.10" } else { "6.5.2" }),
                subject_entity(&footing.id, &path_w, &label.en, &label.de),
                loc("Bearing resistance", "Tragfähigkeit"),
            )
            .utilization(Quantity::new(QuantityKind::Force, v_d), Quantity::new(QuantityKind::Force, r_v))
            .annex(annex)
            .explanation(loc(
                &format!("V_d={:.0} kN vs R_d={:.0} kN (GK{gk}, α={:.1}°, γ′_eff={:.0}, GWL={:.1} m{}).", v_d / 1000.0, r_v / 1000.0, footing.base_inclination_deg, gamma_below, gwl, if use_undrained { ", undrained c_u Annex D.3" } else { "" }),
                &format!("V_d={:.0} kN gegen R_d={:.0} kN (GK{gk}, α={:.1}°, γ′_eff={:.0}, GW={:.1} m{}).", v_d / 1000.0, r_v / 1000.0, footing.base_inclination_deg, gamma_below, gwl, if use_undrained { ", undräniert c_u Anhang D.3" } else { "" }),
            ));
            if v_d > r_v {
                bearing = bearing.remedy(Remedy::at_least(
                    subject_entity(&footing.id, &path_w, &label.en, &label.de),
                    Quantity::length_m(footing.width),
                    Quantity::length_m(b_req),
                    loc(&format!("Increase footing width from {:.2} m to at least {:.2} m.", footing.width, b_req), &format!("Fundamentbreite von {:.2} m auf mindestens {:.2} m erhöhen.", footing.width, b_req)),
                ));
            }
            let u_bearing = if r_v > 1e-9 { v_d / r_v } else { 0.0 };
            if u_bearing > foot_gov_u {
                foot_gov_u = u_bearing;
                foot_gov_sit = sit.clone();
                foot_gov_check = format!("en1997.6.5.bearing.{}.{}", footing.id, lc.id);
            }
            report.push(bearing.build());

            let gamma_passive = part_1::effective_gamma_below_pa_m(layer.gamma, layer.gamma_prime, 0.0, gwl, footing.embedment.max(0.1));
            let r_h = if use_undrained {
                part_1::undrained_sliding_resistance_n(layer.cohesion_undrained, a, &p)
            } else {
                part_1::sliding_resistance_with_params(
                    phi_use, layer.cohesion_effective, v_d, a, &p,
                    footing.embedment, footing.length, gamma_passive, footing.embedment > 0.0,
                )
            };
            let path_hv = format!("footings[id={}].loadCases[id={}].horizontalVariable", footing.id, lc.id);
            let mut sliding = CheckResult::assess(
                format!("en1997.6.5.3.sliding.{}.{}", footing.id, lc.id),
                part1,
                ClauseId::new("EN 1997-1", "6", "6.5.3"),
                subject_entity(&footing.id, &path_hv, &label.en, &label.de),
                loc("Sliding resistance", "Gleitwiderstand"),
            )
            .utilization(Quantity::new(QuantityKind::Force, h_d), Quantity::new(QuantityKind::Force, r_h))
            .annex(annex)
            .explanation(loc(
                &format!("H_d={:.0} kN vs R_h,d={:.0} kN (passive ≤0.5·E_p,k, γ_eff={:.0}).", h_d / 1000.0, r_h / 1000.0, gamma_passive),
                &format!("H_d={:.0} kN gegen R_h,d={:.0} kN (Passiv ≤0.5·E_p,k, γ_eff={:.0}).", h_d / 1000.0, r_h / 1000.0, gamma_passive),
            ));
            if h_d > r_h {
                sliding = sliding.remedy(Remedy::at_most(
                    subject_entity(&footing.id, &path_hv, &label.en, &label.de),
                    Quantity::new(QuantityKind::Force, lc.horizontal_variable),
                    Quantity::new(QuantityKind::Force, (r_h / p.gamma_q.max(1.0)).max(0.0)),
                    loc("Reduce horizontal variable action or increase embedment/width.", "Horizontale veränderliche Einwirkung reduzieren oder Einbindung/Breite erhöhen."),
                )).remedy(Remedy::at_least(
                    subject_entity(&footing.id, &path_w, &label.en, &label.de),
                    Quantity::length_m(footing.width),
                    Quantity::length_m(b_req.max(footing.width * 1.2)),
                    loc("Increase footing width to raise sliding resistance.", "Fundamentbreite erhöhen, um den Gleitwiderstand zu steigern."),
                ));
            }
            let u_sliding = if r_h > 1e-9 { h_d / r_h } else { 0.0 };
            if u_sliding > foot_gov_u {
                foot_gov_u = u_sliding;
                foot_gov_sit = sit.clone();
                foot_gov_check = format!("en1997.6.5.3.sliding.{}.{}", footing.id, lc.id);
            }
            report.push(sliding.build());
            // Companion undrained (Annex D.3 / short-term sliding) whenever c_u is defined — always readable.
            if layer.cohesion_undrained > 0.0 || cohesive {
                let path_cu = format!("layers[id={}].cohesionUndrained", layer.id);
                let r_vu = part_1::undrained_bearing_resistance_n(
                    layer.cohesion_undrained.max(1.0), gamma_below, footing.width, footing.length, footing.embedment, e_b, h_d, v_d, &p,
                );
                let mut ub = CheckResult::assess(
                    format!("en1997.6.5.bearing.undrained.{}.{}", footing.id, lc.id),
                    part1,
                    ClauseId::new("EN 1997-1", "6", "D.3"),
                    subject_entity(&footing.id, &path_cu, &label.en, &label.de),
                    loc("Undrained bearing resistance (Annex D.3)", "Undränierte Tragfähigkeit (Anhang D.3)"),
                )
                .utilization(Quantity::new(QuantityKind::Force, v_d), Quantity::new(QuantityKind::Force, r_vu))
                .annex(annex)
                .explanation(loc(
                    &format!("Total-stress R/A′=(π+2)·c_u·b_c·s_c·i_c+q; c_u={:.0} kPa, R_d={:.0} kN (BS-T short-term).", layer.cohesion_undrained/1000.0, r_vu/1000.0),
                    &format!("Totalspannung R/A′=(π+2)·c_u·b_c·s_c·i_c+q; c_u={:.0} kPa, R_d={:.0} kN (BS-T kurzzeitig).", layer.cohesion_undrained/1000.0, r_vu/1000.0),
                ));
                if v_d > r_vu {
                    ub = ub.remedy(Remedy::at_least(
                        subject_entity(&layer.id, &path_cu, &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
                        Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained),
                        Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained * v_d / r_vu.max(1.0)),
                        loc("Increase undrained strength or footing size for short-term bearing.", "Undränierte Festigkeit oder Fundamentgröße für Kurzzeit erhöhen."),
                    )).remedy(Remedy::at_least(
                        subject_entity(&footing.id, &path_w, &label.en, &label.de),
                        Quantity::length_m(footing.width),
                        Quantity::length_m(footing.width * (v_d / r_vu.max(1.0)).sqrt()),
                        loc("Widen footing for undrained bearing.", "Fundament für undränierte Tragfähigkeit verbreitern."),
                    ));
                }
                report.push(ub.build());
                let r_hu = part_1::undrained_sliding_resistance_n(layer.cohesion_undrained.max(1.0), a, &p);
                let mut us = CheckResult::assess(
                    format!("en1997.6.5.3.sliding.undrained.{}.{}", footing.id, lc.id),
                    part1,
                    ClauseId::new("EN 1997-1", "6", "6.5.3"),
                    subject_entity(&footing.id, &path_cu, &label.en, &label.de),
                    loc("Undrained sliding resistance (short-term)", "Undränierter Gleitwiderstand (kurzzeitig)"),
                )
                .utilization(Quantity::new(QuantityKind::Force, h_d), Quantity::new(QuantityKind::Force, r_hu))
                .annex(annex)
                .explanation(loc(
                    &format!("R_d=A·c_u/γ_R,h; c_u={:.0} kPa, R_h,d={:.0} kN.", layer.cohesion_undrained/1000.0, r_hu/1000.0),
                    &format!("R_d=A·c_u/γ_R,h; c_u={:.0} kPa, R_h,d={:.0} kN. (DE-NA)", layer.cohesion_undrained/1000.0, r_hu/1000.0),
                ));
                if h_d > r_hu {
                    us = us.remedy(Remedy::at_least(
                        subject_entity(&layer.id, &path_cu, &format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
                        Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained),
                        Quantity::new(QuantityKind::Pressure, layer.cohesion_undrained * h_d / r_hu.max(1.0)),
                        loc("Increase c_u or reduce horizontal action for short-term sliding.", "c_u erhöhen oder Horizontallast für Kurzzeitgleiten reduzieren."),
                    ));
                }
                report.push(us.build());
            }

            let e_perm = lc.moment_permanent / lc.vertical_permanent.max(1.0);
            let e_tot = (lc.moment_permanent + lc.moment_variable) / (lc.vertical_permanent + lc.vertical_variable).max(1.0);
            let limit_perm = footing.width / 6.0;
            let limit_tot = footing.width / 3.0;
            let ecc_ok = e_perm <= limit_perm + 1e-9 && e_tot <= limit_tot + 1e-9;
            let ecc_u = (e_perm / limit_perm.max(1e-9)).max(e_tot / limit_tot.max(1e-9));
            let mut ecc = CheckResult::assess(
                format!("en1997.6.5.4.eccentricity.{}.{}", footing.id, lc.id),
                part1054,
                ClauseId::new("DIN 1054", "7", "7.5"),
                subject_entity(&footing.id, &path_w, &label.en, &label.de),
                loc("Eccentricity limits", "Ausmittenbegrenzung"),
            )
            .utilization(Quantity::length_m(ecc_u), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(loc(
                &format!("e_perm={:.3} m (≤ B/6={:.3}), e_tot={:.3} m (≤ B/3={:.3}).", e_perm, limit_perm, e_tot, limit_tot),
                &format!("e_perm={:.3} m (≤ B/6={:.3}), e_tot={:.3} m (≤ B/3={:.3}). (DE-NA)", e_perm, limit_perm, e_tot, limit_tot),
            ));
            if !ecc_ok {
                let b_need = (6.0 * e_perm).max(3.0 * e_tot);
                ecc = ecc.remedy(Remedy::at_least(
                    subject_entity(&footing.id, &path_w, &label.en, &label.de),
                    Quantity::length_m(footing.width),
                    Quantity::length_m(b_need),
                    loc(&format!("Widen footing to at least {:.2} m so eccentricity stays within B/6 and B/3.", b_need), &format!("Fundament auf mindestens {:.2} m verbreitern.", b_need)),
                ));
            }
            report.push(ecc.build());
        }

        // Settlement SLS once per footing
        {
            let mut q_sls_max: f64 = 0.0;
            let area = (footing.width * footing.length).max(1e-9_f64);
            for lc in &footing.load_cases {
                q_sls_max = q_sls_max.max((lc.vertical_permanent + lc.vertical_variable) / area);
            }
            let (s, gov_id) = part_1::settlement_oedometric_with_gwl(
                &doc.layers, footing.width, footing.length, footing.embedment, q_sls_max, gwl,
            );
            let gov_layer = doc.layers.iter().find(|l| l.id == gov_id).unwrap_or(&layer);
            let gov_path = format!("layers[id={}].oedometricModulus", gov_layer.id);
            let path_limit = id_path("footings", &footing.id, "settlementLimit");
            let mut sett = CheckResult::assess(
                format!("en1997.6.6.settlement.{}", footing.id),
                part1,
                ClauseId::new("EN 1997-1", "6", "6.6"),
                subject_entity(&footing.id, &path_limit, &label.en, &label.de),
                loc("Serviceability settlement", "Setzung (GZ 2)"),
            )
            .utilization(Quantity::length_m(s), Quantity::length_m(footing.settlement_limit))
            .annex(annex)
            .explanation(loc(
                &format!("s={:.1} mm vs limit {:.1} mm (oedometric, GWL={:.1} m, layer {}).", s * 1000.0, footing.settlement_limit * 1000.0, gwl, gov_layer.id),
                &format!("s={:.1} mm gegen Grenze {:.1} mm (ödometrisch, GW={:.1} m, Schicht {}).", s * 1000.0, footing.settlement_limit * 1000.0, gwl, gov_layer.id),
            ));
            if s > footing.settlement_limit {
                let e_req = gov_layer.oedometric_modulus * s / footing.settlement_limit.max(1e-9);
                sett = sett.remedy(Remedy::at_least(
                    subject_entity(&gov_layer.id, &gov_path, &format!("Layer {}", gov_layer.id), &format!("Schicht {}", gov_layer.id)),
                    Quantity::new(QuantityKind::Stress, gov_layer.oedometric_modulus),
                    Quantity::new(QuantityKind::Stress, e_req),
                    loc(&format!("Increase oedometric modulus of layer {} to at least {:.0} MPa.", gov_layer.id, e_req / 1e6), &format!("Steifemodul der Schicht {} auf mindestens {:.0} MPa erhöhen.", gov_layer.id, e_req / 1e6)),
                )).remedy(Remedy::at_least(
                    subject_entity(&footing.id, &path_w, &label.en, &label.de),
                    Quantity::length_m(footing.width),
                    Quantity::length_m(footing.width * (s / footing.settlement_limit).sqrt()),
                    loc("Widen footing to reduce contact stress and settlement.", "Fundament verbreitern, um Sohlspannung und Setzung zu reduzieren."),
                ));
            }
            report.push(sett.build());
            let (s_el, el_gov_id) = part_1::elastic_settlement_layered(&doc.layers, footing.width, footing.length, footing.embedment, q_sls_max);
            let el_gov = doc.layers.iter().find(|l| l.id == el_gov_id).unwrap_or(gov_layer);
            let nu = el_gov.poisson_ratio.clamp(0.0, 0.49);
            let e_mod = el_gov.oedometric_modulus.max(1.0);
            let path_nu = format!("layers[id={}].poissonRatio", el_gov.id);
            let mut el = CheckResult::assess(
                format!("en1997.6.6.elastic.{}", footing.id),
                part1,
                ClauseId::new("EN 1997-1", "6", "F.2"),
                subject_entity(&footing.id, &path_nu, &label.en, &label.de),
                loc("Immediate elastic settlement (Annex F.2)", "Sofortsetzung elastisch (Anhang F.2)"),
            )
            .utilization(Quantity::length_m(s_el), Quantity::length_m(footing.settlement_limit))
            .annex(annex)
            .explanation(loc(
                &format!("s_el=p·B·(1−ν²)·I/E = {:.1} mm (ν={:.2}, E_oed={:.0} MPa, layer {}).", s_el*1000.0, nu, e_mod/1e6, el_gov.id),
                &format!("s_el=p·B·(1−ν²)·I/E = {:.1} mm (ν={:.2}, E_oed={:.0} MPa, Schicht {}).", s_el*1000.0, nu, e_mod/1e6, el_gov.id),
            ));
            if s_el > footing.settlement_limit {
                el = el.remedy(Remedy::at_most(
                    subject_entity(&el_gov.id, &path_nu, &format!("Layer {}", el_gov.id), &format!("Schicht {}", el_gov.id)),
                    Quantity::new(QuantityKind::Dimensionless, nu),
                    Quantity::new(QuantityKind::Dimensionless, (nu * 0.85).max(0.1)),
                    loc("Reduce Poisson ratio or raise stiffness / widen footing for elastic settlement.", "Poissonzahl senken oder Steifigkeit erhöhen / Fundament verbreitern."),
                )).remedy(Remedy::at_least(
                    subject_entity(&el_gov.id, &format!("layers[id={}].oedometricModulus", el_gov.id), &format!("Layer {}", el_gov.id), &format!("Schicht {}", el_gov.id)),
                    Quantity::new(QuantityKind::Pressure, e_mod),
                    Quantity::new(QuantityKind::Pressure, e_mod * s_el / footing.settlement_limit.max(1e-6)),
                    loc("Increase oedometric modulus to reduce elastic settlement.", "Ödometermodul erhöhen, um Sofortsetzung zu reduzieren."),
                ));
            }
            report.push(el.build());
        }

        let z_min = part_2::min_investigation_depth_for_gk(footing.width.max(footing.length), gk);
        let mut inv = CheckResult::assess(
            format!("en1997.2.4.2.investigation.{}", footing.id),
            part2,
            ClauseId::new("EN 1997-2", "2", "2.4.2"),
            subject_entity("", "investigationDepth", "Ground investigation", "Baugrunderkundung"),
            loc("Minimum investigation depth", "Mindestaufschlusstiefe"),
        )
        .minimum(Quantity::length_m(doc.investigation_depth), Quantity::length_m(z_min))
        .annex(annex)
        .explanation(loc(
            &format!("z={:.2} m; GK{gk} requires ≥ {:.2} m.", doc.investigation_depth, z_min),
            &format!("z={:.2} m; GK{gk} fordert ≥ {:.2} m.", doc.investigation_depth, z_min),
        ));
        if doc.investigation_depth < z_min {
            inv = inv.remedy(Remedy::at_least(
                subject_entity("", "investigationDepth", "Ground investigation", "Baugrunderkundung"),
                Quantity::length_m(doc.investigation_depth),
                Quantity::length_m(z_min),
                loc(&format!("Deepen investigation from {:.2} m to at least {:.2} m.", doc.investigation_depth, z_min), &format!("Aufschlusstiefe von {:.2} m auf mindestens {:.2} m erhöhen.", doc.investigation_depth, z_min)),
            ));
        }
        report.push(inv.build());
        if foot_gov_u >= 0.0 {
            push_governing_situation_summary(
                &mut report,
                format!("en1997.governing.situation.footing.{}", footing.id),
                part1,
                annex,
                subject_entity(&footing.id, &path_w, &label.en, &label.de),
                &foot_gov_sit,
                &doc.design_approach,
                foot_gov_u,
                &foot_gov_check,
            );
        }
    }

    if doc.piles.is_empty() {
        report.push(
            CheckResult::assess("en1997.pile.na", part1, ClauseId::new("EN 1997-1", "7", "7.6"), SubjectRef::whole(loc("Project", "Projekt")), loc("Pile checks", "Pfahlnachweise"))
                .not_applicable(loc("No piles in the subject.", "Keine Pfähle im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }

    for pile in doc.piles.iter() {
        let label = loc(&format!("Pile {}", pile.id), &format!("Pfahl {}", pile.id));
        let a_base = std::f64::consts::PI * (pile.diameter * 0.5).powi(2);
        let (r_s_mean, r_s_min, r_b_mean, r_b_min, n) = if pile.test_profiles.is_empty() {
            let rs = part_1::shaft_resistance_n(pile.alpha_s, pile.diameter, pile.unit_shaft_resistance, pile.length);
            let rb = part_1::base_resistance_n(pile.unit_base_resistance, a_base);
            (rs, rs, rb, rb, 1u32)
        } else {
            let shafts: Vec<f64> = pile.test_profiles.iter().map(|t| t.shaft_resistance).collect();
            let bases: Vec<f64> = pile.test_profiles.iter().map(|t| t.base_resistance).collect();
            let n = shafts.len() as u32;
            (shafts.iter().sum::<f64>() / n as f64, shafts.iter().cloned().fold(f64::INFINITY, f64::min),
             bases.iter().sum::<f64>() / n as f64, bases.iter().cloned().fold(f64::INFINITY, f64::min), n)
        };
        let path_l = id_path("piles", &pile.id, "length");
        let mut best_comp: Option<(f64, String, CheckResult)> = None;
        let mut best_ten: Option<(f64, CheckResult)> = None;
        for sit in design_situations.iter() {
            let p = resolve_params(approach, annex, sit);
            let (gamma_b_t, gamma_s_t, xi3_t, xi4_t) = part_1::pile_type_factors(&pile.pile_type, n, &p);
            let r_b_k_t = (r_b_mean / xi3_t).min(r_b_min / xi4_t);
            let r_s_k_t = (r_s_mean / xi3_t).min(r_s_min / xi4_t);
            let r_s_geom = part_1::shaft_resistance_n(pile.alpha_s, pile.diameter, pile.unit_shaft_resistance, pile.length);
            let r_b_geom = part_1::base_resistance_n(pile.unit_base_resistance, a_base);
            let r_c_d = part_1::pile_design_resistance_typed(
                r_b_k_t.min(r_b_geom / xi3_t),
                r_s_k_t.min(r_s_geom / xi3_t),
                gamma_b_t,
                gamma_s_t,
            ) * pile.count as f64;
            let n_ed = design_actions(&p, pile.compression_permanent, pile.compression_variable);
            let u = if r_c_d > 1e-9 { n_ed / r_c_d } else { 0.0 };
            let sit_lbl = situation_short(sit);
            let mut comp = CheckResult::assess(
                format!("en1997.7.6.2.compression.{}", pile.id),
                part1,
                ClauseId::new("EN 1997-1", "7", "7.6.2"),
                subject_entity(&pile.id, &path_l, &label.en, &label.de),
                loc("Pile axial compression", "Pfahldruckwiderstand"),
            )
            .utilization(Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, r_c_d))
            .annex(annex)
            .explanation(loc(
                &format!("Governing sit={sit_lbl}; N_ed={:.0} kN vs R_c,d={:.0} kN (type={}, γ_b={:.2}, γ_s={:.2}, ξ3={:.2}).", n_ed / 1000.0, r_c_d / 1000.0, pile.pile_type, gamma_b_t, gamma_s_t, xi3_t),
                &format!("Maßgebend sit={sit_lbl}; N_ed={:.0} kN gegen R_c,d={:.0} kN (Typ={}, γ_b={:.2}, γ_s={:.2}, ξ3={:.2}).", n_ed / 1000.0, r_c_d / 1000.0, pile.pile_type, gamma_b_t, gamma_s_t, xi3_t),
            ));
            if n_ed > r_c_d {
                let n_req = ((n_ed / (r_c_d / pile.count.max(1) as f64)).ceil() as u32).max(pile.count);
                comp = comp.remedy(Remedy::at_least(
                    subject_entity(&pile.id, &id_path("piles", &pile.id, "count"), &label.en, &label.de),
                    Quantity::new(QuantityKind::Dimensionless, pile.count as f64),
                    Quantity::new(QuantityKind::Dimensionless, n_req as f64),
                    loc("Increase pile count or length for compression.", "Pfahlan zahl oder -länge für Druck erhöhen."),
                )).remedy(Remedy::at_least(
                    subject_entity(&pile.id, &path_l, &label.en, &label.de),
                    Quantity::length_m(pile.length),
                    Quantity::length_m(pile.length * n_ed / r_c_d.max(1.0)),
                    loc("Increase pile length to raise compression resistance.", "Pfahllänge erhöhen, um den Druckwiderstand zu steigern."),
                ));
            }
            let built = comp.build();
            if best_comp.as_ref().map(|(bu, _, _)| u > *bu).unwrap_or(true) {
                best_comp = Some((u, sit.clone(), built));
            }
            let t_ed = design_actions(&p, pile.tension_permanent, pile.tension_variable);
            let r_t_d = r_s_k_t.min(r_s_geom / xi3_t) / gamma_s_t * pile.count as f64;
            if t_ed > 1.0 || pile.tension_permanent > 0.0 || pile.tension_variable > 0.0 {
                let u_t = if r_t_d > 1e-9 { t_ed / r_t_d } else { 0.0 };
                let mut ten = CheckResult::assess(
                    format!("en1997.7.6.3.tension.{}", pile.id),
                    part1,
                    ClauseId::new("EN 1997-1", "7", "7.6.3"),
                    subject_entity(&pile.id, &path_l, &label.en, &label.de),
                    loc("Pile tension resistance", "Pfahlzugwiderstand"),
                )
                .utilization(Quantity::new(QuantityKind::Force, t_ed), Quantity::new(QuantityKind::Force, r_t_d))
                .annex(annex)
                .explanation(loc(
                    &format!("Sit={sit_lbl}; T_ed={:.0} kN vs R_t,d={:.0} kN.", t_ed / 1000.0, r_t_d / 1000.0),
                    &format!("Sit={sit_lbl}; T_ed={:.0} kN gegen R_t,d={:.0} kN.", t_ed / 1000.0, r_t_d / 1000.0),
                ));
                if t_ed > r_t_d {
                    ten = ten.remedy(Remedy::at_least(
                        subject_entity(&pile.id, &path_l, &label.en, &label.de),
                        Quantity::length_m(pile.length),
                        Quantity::length_m(pile.length * t_ed / r_t_d.max(1.0)),
                        loc("Increase pile length to raise shaft tension resistance.", "Pfahllänge erhöhen, um den Zugwiderstand zu steigern."),
                    ));
                }
                let tb = ten.build();
                if best_ten.as_ref().map(|(bu, _)| u_t > *bu).unwrap_or(true) {
                    best_ten = Some((u_t, tb));
                }
            }
        }
        if let Some((u, sit, c)) = best_comp {
            let chk = c.id.clone();
            report.push(c);
            push_governing_situation_summary(
                &mut report,
                format!("en1997.governing.situation.pile.{}", pile.id),
                part1,
                annex,
                subject_entity(&pile.id, &path_l, &label.en, &label.de),
                &sit,
                &doc.design_approach,
                u,
                &chk,
            );
        }
        if let Some((_, c)) = best_ten { report.push(c); }
    }

        if doc.retaining_walls.is_empty() {
        report.push(
            CheckResult::assess("en1997.wall.na", part1, ClauseId::new("EN 1997-1", "9", "9.7"), SubjectRef::whole(loc("Project", "Projekt")), loc("Retaining wall checks", "Stützkonstruktionsnachweise"))
                .not_applicable(loc("No retaining walls in the subject.", "Keine Stützbauwerke im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }

    for wall in doc.retaining_walls.iter() {
        let label = loc(&format!("Wall {}", wall.id), &format!("Wand {}", wall.id));
        let (k_ep, k_label) = part_1::earth_pressure_coefficient(wall.backfill_phi_deg, &wall.earth_pressure_mode, wall.ocr, wall.wall_friction_deg);
        let e_a = 0.5 * k_ep * wall.backfill_gamma * wall.height.powi(2) + k_ep * wall.surcharge * wall.height;
        // Select governing design situation (max sliding utilization) among project situations.
        let mut wall_sit = doc.design_situation.clone();
        let mut wall_u = -1.0f64;
        for sit in design_situations.iter() {
            let p = resolve_params(approach, annex, sit);
            let h_try = design_actions(&p, wall.horizontal_permanent + e_a, 0.0);
            let gamma_w_try = 9_810.0;
            let gamma_c_try = wall.concrete_gamma.max(1_000.0);
            let stem_vol = wall.base_width * wall.stem_thickness;
            let self_w = stem_vol * gamma_c_try;
            let v_try = design_actions(&p, wall.vertical_permanent + self_w, 0.0);
            let a_try = wall.base_width.max(0.1);
            let gamma_p = if gwl < wall.embedment { (wall.backfill_gamma - gamma_w_try).max(1_000.0) } else { wall.backfill_gamma };
            let e_p_k = part_1::passive_earth_force_n(wall.backfill_phi_deg, gamma_p, wall.embedment, 1.0, wall.wall_friction_deg);
            let r_slide = part_1::sliding_resistance_with_params(wall.backfill_phi_deg, 0.0, v_try, a_try, &p, 0.0, 0.0, wall.backfill_gamma, false) + 0.5 * e_p_k / p.gamma_r_e;
            let u = if r_slide > 1e-9 { h_try / r_slide } else { 0.0 };
            if u > wall_u {
                wall_u = u;
                wall_sit = sit.clone();
            }
        }
        let p_project = resolve_params(approach, annex, &wall_sit);
        let h_d = design_actions(&p_project, wall.horizontal_permanent + e_a, 0.0);
        let gamma_w = 9_810.0;
        let stem_below_gwl = (wall.embedment - gwl).max(0.0).min(wall.height + wall.embedment);
        let gamma_c = wall.concrete_gamma.max(1_000.0);
        let gamma_c_prime = (gamma_c - gamma_w).max(1_000.0);
        // Vertical stem/base self-weight: γ_c above GWL, γ′_c = γ_c − γ_w below GWL.
        let stem_vol = wall.base_width * wall.stem_thickness; // m² × 1 m run
        let self_weight = if gwl >= wall.height + wall.embedment {
            stem_vol * gamma_c
        } else if gwl <= 0.0 {
            stem_vol * gamma_c_prime
        } else {
            let frac_sub = (stem_below_gwl / (wall.height + wall.embedment).max(0.1)).clamp(0.0, 1.0);
            stem_vol * (gamma_c * (1.0 - frac_sub) + gamma_c_prime * frac_sub)
        };
        let v_d = design_actions(&p_project, wall.vertical_permanent + self_weight, 0.0);
        let a = wall.base_width * 1.0;
        let gamma_p = if gwl < wall.embedment { (wall.backfill_gamma - gamma_w).max(1_000.0) } else { wall.backfill_gamma };
        // Passive earth resistance with γ_R,e and wall embedment
        let e_p_k = part_1::passive_earth_force_n(wall.backfill_phi_deg, gamma_p, wall.embedment, 1.0, wall.wall_friction_deg);
        let r_passive = e_p_k / p_project.gamma_r_e;
        let r_base = part_1::sliding_resistance_with_params(wall.backfill_phi_deg, 0.0, v_d, a, &p_project, 0.0, 0.0, wall.backfill_gamma, false);
        let r_slide = r_base + 0.5 * r_passive;
        let path_b = id_path("retainingWalls", &wall.id, "baseWidth");
        let mut slide = CheckResult::assess(
            format!("en1997.9.sliding.{}", wall.id),
            part1,
            ClauseId::new("EN 1997-1", "9", "9.7.3"),
            subject_entity(&wall.id, &path_b, &label.en, &label.de),
            loc("Retaining wall sliding", "Gleiten der Stützwand"),
        )
        .utilization(Quantity::new(QuantityKind::Force, h_d), Quantity::new(QuantityKind::Force, r_slide))
        .annex(annex)
        .explanation(loc(
            &format!("Sit={}; H_d={:.0} kN, R_slide={:.0} kN, {k_label}={:.3}, embed={:.2} m, γ_R,e={:.2}.", situation_short(&wall_sit), h_d/1000.0, r_slide/1000.0, k_ep, wall.embedment, p_project.gamma_r_e),
            &format!("Sit={}; H_d={:.0} kN, R_Gleiten={:.0} kN, {k_label}={:.3}, Einbindung={:.2} m, γ_R,e={:.2}.", situation_short(&wall_sit), h_d/1000.0, r_slide/1000.0, k_ep, wall.embedment, p_project.gamma_r_e),
        ));
        if h_d > r_slide {
            let path_h = id_path("retainingWalls", &wall.id, "height");
            slide = slide.remedy(Remedy::at_least(
                subject_entity(&wall.id, &path_b, &label.en, &label.de),
                Quantity::length_m(wall.base_width),
                Quantity::length_m(wall.base_width * h_d / r_slide.max(1.0)),
                loc("Widen wall base or increase embedment.", "Sohlbreite oder Einbindung erhöhen."),
            )).remedy(Remedy::at_least(
                subject_entity(&wall.id, &id_path("retainingWalls", &wall.id, "embedment"), &label.en, &label.de),
                Quantity::length_m(wall.embedment),
                Quantity::length_m((wall.embedment * 1.4).max(1.0)),
                loc("Increase wall embedment to raise passive resistance.", "Wand-Einbindung erhöhen, um den Passivwiderstand zu steigern."),
            )).remedy(Remedy::at_most(
                subject_entity(&wall.id, &path_h, &label.en, &label.de),
                Quantity::length_m(wall.height),
                Quantity::length_m((wall.height * 0.9).max(1.0)),
                loc("Reduce retained height to lower active earth pressure.", "Stützhöhe reduzieren, um den aktiven Erddruck zu verringern."),
            ));
        }
        report.push(slide.build());

        // Earth-pressure regime check (Rankine / K0 / increased active)
        let path_mode = id_path("retainingWalls", &wall.id, "earthPressureMode");
        let path_move = id_path("retainingWalls", &wall.id, "wallMovement");
        let ka_r = part_1::ka_rankine(wall.backfill_phi_deg);
        let k0 = part_1::k0_jaky(wall.backfill_phi_deg, wall.ocr);
        let mode_key = wall.earth_pressure_mode.to_ascii_lowercase().replace(['-','_',' '], "");
        let move_key = wall.wall_movement.to_ascii_lowercase().replace(['-','_',' '], "");
        let mode_ok = matches!(mode_key.as_str(), "active" | "atrest" | "k0" | "increasedactive" | "ruhe");
        let move_ok = matches!(move_key.as_str(), "free" | "rigid" | "propped" | "gestuetzt" | "gestützt" | "steif" | "");
        // DIN 4085: free cantilever needs active (sufficient displacement); rigid/propped → at-rest or increased active.
        let consistent = match move_key.as_str() {
            "free" | "" => matches!(mode_key.as_str(), "active"),
            "rigid" | "steif" | "propped" | "gestuetzt" | "gestützt" => matches!(mode_key.as_str(), "atrest" | "k0" | "increasedactive" | "ruhe"),
            _ => false,
        };
        let regime_fail = !mode_ok || !move_ok || !consistent;
        let (computed_k, limit_k) = if regime_fail {
            (2.0, 1.0)
        } else {
            // OCR-sensitive K₀ index (DIN 4085 Jáky) — always ≤1 so Pass when regime is valid.
            (k0.clamp(1e-9, 0.999), 1.0)
        };
        let mut ep = CheckResult::assess(
            format!("en1997.9.earthPressure.{}", wall.id),
            part1054,
            ClauseId::new("DIN 4085", "6", "6.1"),
            subject_entity(&wall.id, &path_mode, &label.en, &label.de),
            loc("Earth-pressure regime vs wall movement (DIN 4085)", "Erddruckansatz vs Wandbewegung (DIN 4085)"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, computed_k), Quantity::new(QuantityKind::Dimensionless, limit_k))
        .annex(annex)
        .explanation(loc(
            &format!("Mode={}, movement={}, {k_label}={:.3} (Ka,R={:.3}, K0={:.3}); consistent={}.", wall.earth_pressure_mode, wall.wall_movement, k_ep, ka_r, k0, consistent && mode_ok && move_ok),
            &format!("Ansatz={}, Bewegung={}, {k_label}={:.3} (Ka,R={:.3}, K0={:.3}); konsistent={}.", wall.earth_pressure_mode, wall.wall_movement, k_ep, ka_r, k0, consistent && mode_ok && move_ok),
        ));
        if !mode_ok || !move_ok {
            ep = ep.status(CheckStatus::Fail).remedy(Remedy::one_of(
                subject_entity(&wall.id, &path_mode, &label.en, &label.de),
                vec!["active".into(), "atRest".into(), "increasedActive".into()],
                loc("Select active, atRest, or increasedActive per DIN 4085.", "active, atRest oder increasedActive nach DIN 4085 wählen."),
            )).remedy(Remedy::one_of(
                subject_entity(&wall.id, &path_move, &label.en, &label.de),
                vec!["free".into(), "rigid".into(), "propped".into()],
                loc("Select free, rigid, or propped wall movement class.", "Wandbewegungsklasse free, rigid oder propped wählen."),
            ));
        } else if !consistent {
            ep = ep.status(CheckStatus::Fail).remedy(Remedy::one_of(
                subject_entity(&wall.id, &path_mode, &label.en, &label.de),
                if matches!(move_key.as_str(), "free" | "") {
                    vec!["active".into()]
                } else {
                    vec!["atRest".into(), "increasedActive".into()]
                },
                loc(
                    "Align earth-pressure mode with wall movement (DIN 4085 Table): free→active; rigid/propped→atRest or increasedActive.",
                    "Erddruckansatz an Wandbewegung anpassen (DIN 4085 Tabelle): frei→active; starr/gestützt→atRest oder increasedActive.",
                ),
            )).remedy(Remedy::one_of(
                subject_entity(&wall.id, &path_move, &label.en, &label.de),
                vec!["free".into(), "rigid".into(), "propped".into()],
                loc("Or change wall movement class to match the selected earth-pressure mode.", "Oder Wandbewegungsklasse dem gewählten Erddruckansatz anpassen."),
            ));
        }
        report.push(ep.build());

        let overturning_arm = wall.height / 3.0;
        let resisting_arm = wall.base_width / 2.0;
        let m_dst = h_d * overturning_arm;
        let m_stb = v_d * resisting_arm;
        let path_e = id_path("retainingWalls", &wall.id, "embedment");
        let mut ot = CheckResult::assess(
            format!("en1997.9.overturning.{}", wall.id),
            part1,
            ClauseId::new("EN 1997-1", "9", "9.7.2"),
            subject_entity(&wall.id, &path_e, &label.en, &label.de),
            loc("Retaining wall overturning", "Kippen der Stützwand"),
        )
        .minimum(Quantity::new(QuantityKind::Moment, m_stb), Quantity::new(QuantityKind::Moment, m_dst))
        .annex(annex)
        .explanation(loc(&format!("M_stb={:.0} kNm vs M_dst={:.0} kNm.", m_stb/1000.0, m_dst/1000.0), &format!("M_stb={:.0} kNm gegen M_dst={:.0} kNm.", m_stb/1000.0, m_dst/1000.0)));
        if m_stb < m_dst {
            ot = ot.remedy(Remedy::at_least(
                subject_entity(&wall.id, &path_b, &label.en, &label.de),
                Quantity::length_m(wall.base_width),
                Quantity::length_m(wall.base_width * m_dst / m_stb.max(1.0)),
                loc("Widen base to increase restoring moment.", "Sohlbreite erhöhen, um das rückstellende Moment zu steigern."),
            ));
        }
        report.push(ot.build());
        if wall_u >= 0.0 {
            push_governing_situation_summary(
                &mut report,
                format!("en1997.governing.situation.wall.{}", wall.id),
                part1,
                annex,
                subject_entity(&wall.id, &id_path("retainingWalls", &wall.id, "baseWidth"), &label.en, &label.de),
                &wall_sit,
                &doc.design_approach,
                wall_u,
                &format!("en1997.9.sliding.{}", wall.id),
            );
        }
    }

    if doc.slopes.is_empty() {
        report.push(
            CheckResult::assess("en1997.slope.na", part1, ClauseId::new("EN 1997-1", "11", "11.5"), SubjectRef::whole(loc("Project", "Projekt")), loc("Slope stability", "Böschungsbruch"))
                .not_applicable(loc("No slopes in the subject.", "Keine Böschungen im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }

    // Overall stability uses GEO-3 (EN 1997-1 §2.4.7.3.4 + DIN 1054 NA) regardless of project DA.
    for slope in doc.slopes.iter() {
        let geo3 = DesignApproach::Geo3.annex_params(annex);
        let label = loc(&format!("Slope {}", slope.id), &format!("Böschung {}", slope.id));
        let path = id_path("slopes", &slope.id, "angleDeg");
        let path_gov = id_path("slopes", &slope.id, "governingLayerId");
        let fos_opt = part_1::bishop_fos_slices(
            &doc.layers, &slope.governing_layer_id, slope.height, slope.angle_deg, slope.length, gwl, geo3.gamma_phi, geo3.gamma_c,
        );
        if let Some(fos) = fos_opt {
            let mut st = CheckResult::assess(
                format!("en1997.11.bishop.{}", slope.id),
                part1,
                ClauseId::new("EN 1997-1", "11", "11.5"),
                subject_entity(&slope.id, &path, &label.en, &label.de),
                loc("Overall stability (Bishop slices / GEO-3)", "Gesamtstandsicherheit (Bishop-Lamellen / GEO-3)"),
            )
            .minimum(Quantity::new(QuantityKind::Dimensionless, fos), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(loc(
                &format!("FoS={:.2} (GEO-3 per EN 1997-1 §2.4.7.3.4 / DIN 1054 NA; slices by depth, governing layer {}, L={:.1} m, GWL={:.1} m, γ_φ={:.2}).", fos, slope.governing_layer_id, slope.length, gwl, geo3.gamma_phi),
                &format!("η={:.2} (GEO-3 nach EN 1997-1 §2.4.7.3.4 / DIN 1054 NA; Lamellen nach Tiefe, maßgebende Schicht {}, L={:.1} m, GW={:.1} m, γ_φ={:.2}).", fos, slope.governing_layer_id, slope.length, gwl, geo3.gamma_phi),
            ));
            if fos < 1.0 {
                let path_h = id_path("slopes", &slope.id, "height");
                st = st.remedy(Remedy::at_most(
                    subject_entity(&slope.id, &path, &label.en, &label.de),
                    Quantity::new(QuantityKind::Dimensionless, slope.angle_deg),
                    Quantity::new(QuantityKind::Dimensionless, (slope.angle_deg * 0.85).max(10.0)),
                    loc("Flatten the slope angle to raise Bishop FoS.", "Böschungswinkel abflachen, um die Bishop-Sicherheit zu erhöhen."),
                )).remedy(Remedy::at_most(
                    subject_entity(&slope.id, &path_h, &label.en, &label.de),
                    Quantity::length_m(slope.height),
                    Quantity::length_m((slope.height * 0.9).max(1.0)),
                    loc("Reduce slope height to raise Bishop FoS.", "Böschungshöhe verringern, um die Bishop-Sicherheit zu erhöhen."),
                ));
            }
            report.push(st.build());
        } else {
            let choices: Vec<String> = doc.layers.iter().map(|l| l.id.clone()).collect();
            let st = CheckResult::assess(
                format!("en1997.11.bishop.{}", slope.id),
                part1,
                ClauseId::new("EN 1997-1", "11", "11.5"),
                subject_entity(&slope.id, &path_gov, &label.en, &label.de),
                loc("Overall stability (Bishop slices / GEO-3)", "Gesamtstandsicherheit (Bishop-Lamellen / GEO-3)"),
            )
            .utilization(Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .status(CheckStatus::Fail)
            .annex(annex)
            .explanation(loc(
                &format!("Governing layer id '{}' is missing or unknown — Bishop slices cannot run.", slope.governing_layer_id),
                &format!("Maßgebende Schicht-Id '{}' fehlt oder unbekannt — Bishop-Lamellen nicht möglich.", slope.governing_layer_id),
            ))
            .remedy(if choices.is_empty() {
                Remedy::at_least(
                    subject_entity(&slope.id, &path_gov, &label.en, &label.de),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    loc("Add a soil layer and set slopes[].governingLayerId to its id.", "Bodenschicht anlegen und slopes[].governingLayerId setzen."),
                )
            } else {
                Remedy::one_of(
                    subject_entity(&slope.id, &path_gov, &label.en, &label.de),
                    choices,
                    loc("Set governingLayerId to an existing layer id.", "governingLayerId auf eine bestehende Schicht-Id setzen."),
                )
            });
            report.push(st.build());
        }
    }

    if doc.uplift_cases.is_empty() {
        report.push(
            CheckResult::assess("en1997.upl.na", part1, ClauseId::new("EN 1997-1", "10", "10.2"), SubjectRef::whole(loc("Project", "Projekt")), loc("Uplift / heave", "Aufschwimmen / hydraulischer Grundbruch"))
                .not_applicable(loc("No uplift/heave cases in the subject.", "Keine Aufschwimm-/HYD-Fälle im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }

    for u in doc.uplift_cases.iter() {
        let label = loc(&format!("Uplift {}", u.id), &format!("Aufschwimmen {}", u.id));
        let pore = if u.pore_pressure > 1.0 { u.pore_pressure } else { 9_810.0 * gwl.max(0.0) };
        let mut upl_sit = doc.design_situation.clone();
        let mut upl_u = -1.0f64;
        let mut upl_p = resolve_upl_params(annex, &doc.design_situation);
        for sit in design_situations.iter() {
            let p = resolve_upl_params(annex, sit);
            let stab_try = u.permanent_stabilizing * p.gamma_g_stb;
            let dest_try = p.gamma_g_dst * u.permanent_destabilizing + p.gamma_q_dst * u.variable_destabilizing;
            let u_try = if stab_try > 1e-9 { dest_try / stab_try } else { 0.0 };
            if u_try > upl_u {
                upl_u = u_try;
                upl_sit = sit.clone();
                upl_p = p;
            }
        }
        let stab = u.permanent_stabilizing * upl_p.gamma_g_stb;
        let dest = upl_p.gamma_g_dst * u.permanent_destabilizing + upl_p.gamma_q_dst * u.variable_destabilizing;
        let path_s = id_path("upliftCases", &u.id, "permanentStabilizing");
        let mut upl = CheckResult::assess(
            format!("en1997.10.upl.{}", u.id),
            part1,
            ClauseId::new("EN 1997-1", "10", "10.2"),
            subject_entity(&u.id, &path_s, &label.en, &label.de),
            loc("Uplift equilibrium (UPL)", "Aufschwimmen (UPL)"),
        )
        .minimum(Quantity::new(QuantityKind::Force, stab), Quantity::new(QuantityKind::Force, dest))
        .annex(annex)
        .explanation(loc(
            &format!("G_stb,d={:.0} kN vs V_dst,d={:.0} kN (γ_stb={:.2}, γ_dst={:.2}/γ_Q={:.2}).", stab/1000.0, dest/1000.0, upl_p.gamma_g_stb, upl_p.gamma_g_dst, upl_p.gamma_q_dst),
            &format!("G_stb,d={:.0} kN gegen V_dst,d={:.0} kN (γ_stb={:.2}, γ_dst={:.2}/γ_Q={:.2}).", stab/1000.0, dest/1000.0, upl_p.gamma_g_stb, upl_p.gamma_g_dst, upl_p.gamma_q_dst),
        ));
        if stab < dest {
            let g_req = dest / upl_p.gamma_g_stb.max(0.5);
            upl = upl.remedy(Remedy::at_least(
                subject_entity(&u.id, &path_s, &label.en, &label.de),
                Quantity::new(QuantityKind::Force, u.permanent_stabilizing),
                Quantity::new(QuantityKind::Force, g_req),
                loc(&format!("Increase permanent stabilizing force to at least {:.0} kN.", g_req/1000.0), &format!("Ständige stabilisierende Einwirkung auf mindestens {:.0} kN erhöhen.", g_req/1000.0)),
            ));
        }
        report.push(upl.build());

        let sigma_d = u.total_stress;
        let u_d = pore * upl_p.gamma_hyd;
        let path_t = id_path("upliftCases", &u.id, "totalStress");
        let mut hyd = CheckResult::assess(
            format!("en1997.10.hyd.{}", u.id),
            part1,
            ClauseId::new("EN 1997-1", "10", "10.3"),
            subject_entity(&u.id, &path_t, &label.en, &label.de),
            loc("Hydraulic heave (HYD)", "Hydraulischer Grundbruch (HYD)"),
        )
        .minimum(Quantity::new(QuantityKind::Pressure, sigma_d), Quantity::new(QuantityKind::Pressure, u_d))
        .annex(annex)
        .explanation(loc(
            &format!("σ={:.0} kPa vs u_d={:.0} kPa (γ_HYD={:.2}, GWL={:.1} m).", sigma_d/1000.0, u_d/1000.0, upl_p.gamma_hyd, gwl),
            &format!("σ={:.0} kPa gegen u_d={:.0} kPa (γ_HYD={:.2}, GW={:.1} m).", sigma_d/1000.0, u_d/1000.0, upl_p.gamma_hyd, gwl),
        ));
        if sigma_d < u_d {
            hyd = hyd.remedy(Remedy::at_least(
                subject_entity(&u.id, &path_t, &label.en, &label.de),
                Quantity::new(QuantityKind::Pressure, u.total_stress),
                Quantity::new(QuantityKind::Pressure, u_d),
                loc("Increase total stress or lower groundwater / pore pressure.", "Totalspannung erhöhen oder Grundwasser / Porenwasserdruck absenken."),
            )).remedy(Remedy::at_most(
                subject_entity("", "groundwaterLevel", "Groundwater level", "Grundwasserstand"),
                Quantity::length_m(gwl),
                Quantity::length_m((gwl * 0.7).max(0.0)),
                loc("Lower groundwater level to reduce pore pressure.", "Grundwasserstand absenken, um den Porenwasserdruck zu reduzieren."),
            ));
        }
        report.push(hyd.build());
        if upl_u >= 0.0 {
            push_governing_situation_summary(
                &mut report,
                format!("en1997.governing.situation.uplift.{}", u.id),
                part1,
                annex,
                subject_entity(&u.id, &path_s, &label.en, &label.de),
                &upl_sit,
                &doc.design_approach,
                upl_u,
                &format!("en1997.10.upl.{}", u.id),
            );
        }
    }

    // Project governing design situation + approach (max utilization across applicable checks).
    let approach_lbl = doc.design_approach.clone();
    let governing_info = report
        .checks
        .iter()
        .filter(|c| c.status != CheckStatus::NotApplicable && !c.id.starts_with("en1997.governing.situation"))
        .max_by(|a, b| a.utilization.partial_cmp(&b.utilization).unwrap_or(std::cmp::Ordering::Equal))
        .map(|c| (c.utilization, c.id.clone(), c.explanation.en.clone()));
    if let Some((gov_u, gov_id, gov_expl)) = governing_info {
        let sit_guess = design_situations
            .iter()
            .find(|s| gov_expl.contains(situation_short(s)) || gov_expl.contains(s.as_str()))
            .cloned()
            .unwrap_or_else(|| doc.design_situation.clone());
        push_governing_situation_summary(
            &mut report,
            "en1997.governing.situation".into(),
            part1,
            annex,
            SubjectRef::whole(loc("Project", "Projekt")),
            &sit_guess,
            &approach_lbl,
            gov_u,
            &gov_id,
        );
    }

    report
}


//#region 🏷️FieldMetadata
/// 🏷️ EN 1997 NormFieldMeta lookup — SI units + en/de labels (index wildcards via `[]`).
pub fn lookup_field_meta(path: &str) -> Option<crate::app_surface::NormFieldMeta> {
    use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

    const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
        NormFieldChoice { value, label_en, label_de }
    }

    const ANNEX: &[NormFieldChoice] = &[
        choice("de", "Germany (DIN)", "Deutschland (DIN)"),
        choice("en", "EN (CEN)", "EN (CEN)"),
    ];
    const SITUATION: &[NormFieldChoice] = &[
        choice("bsP", "Persistent (BS-P)", "Ständig (BS-P)"),
        choice("bsT", "Transient (BS-T)", "Vorübergehend (BS-T)"),
        choice("bsA", "Accidental (BS-A)", "Außergewöhnlich (BS-A)"),
    ];
    const APPROACH: &[NormFieldChoice] = &[
        choice("da1str", "DA1 Combination 1 (STR)", "DA1 Kombination 1 (STR)"),
        choice("da1geo", "DA1 Combination 2 (GEO)", "DA1 Kombination 2 (GEO)"),
        choice("da2", "DA2 / DA2* (GEO-2)", "DA2 / DA2* (GEO-2)"),
        choice("geo2", "GEO-2", "GEO-2"),
        choice("da3", "DA3", "DA3"),
        choice("geo3", "GEO-3", "GEO-3"),
    ];
    const PILE_TYPE: &[NormFieldChoice] = &[
        choice("bored", "Bored pile", "Bohrpfahl"),
        choice("driven", "Driven pile", "Rammpfahl"),
        choice("cfa", "CFA pile", "Schneckenortbetonpfahl (CFA)"),
    ];
    const EP_MODE: &[NormFieldChoice] = &[
        choice("active", "Active (Rankine/Coulomb)", "Aktiv (Rankine/Coulomb)"),
        choice("atRest", "At rest (Jáky K₀)", "Ruhe (Jáky K₀)"),
        choice("increasedActive", "Increased active (DIN 4085)", "Erhöht aktiv (DIN 4085)"),
    ];
    const WALL_MOVE: &[NormFieldChoice] = &[
        choice("free", "Free (cantilever, sufficient displacement)", "Frei (Kragarm, ausreichende Verschiebung)"),
        choice("rigid", "Rigid (little displacement)", "Starr (geringe Verschiebung)"),
        choice("propped", "Propped / supported", "Gestützt"),
    ];
    const SOIL: &[NormFieldChoice] = &[
        choice("sand", "Sand", "Sand"),
        choice("gravel", "Gravel", "Kies"),
        choice("clay", "Clay", "Ton"),
        choice("silt", "Silt", "Schluff"),
        choice("fill", "Fill", "Auffüllung"),
    ];

    const TABLE: &[(&str, NormFieldMeta)] = &[
        ("structureId", NormFieldMeta { label_en: "Structure ID", label_de: "Bauwerks-ID", unit: None, choices: None }),
        ("geotechnicalCategory", NormFieldMeta { label_en: "Geotechnical category", label_de: "Geotechnische Kategorie", unit: None, choices: None }),
        ("designSituation", NormFieldMeta { label_en: "Design situation (DIN 1054)", label_de: "Bemessungssituation (DIN 1054)", unit: None, choices: Some(SITUATION) }),
        ("designApproach", NormFieldMeta { label_en: "Design approach", label_de: "Nachweisverfahren", unit: None, choices: Some(APPROACH) }),
        ("annex", NormFieldMeta { label_en: "National annex", label_de: "Nationaler Anhang", unit: None, choices: Some(ANNEX) }),
        ("groundwaterLevel", NormFieldMeta { label_en: "Groundwater level", label_de: "Grundwasserstand", unit: Some("m"), choices: None }),
        ("investigationDepth", NormFieldMeta { label_en: "Investigation depth", label_de: "Aufschlusstiefe", unit: Some("m"), choices: None }),
        ("layers", NormFieldMeta { label_en: "Soil layers", label_de: "Bodenschichten", unit: None, choices: None }),
        ("layers[].id", NormFieldMeta { label_en: "Layer ID", label_de: "Schicht-ID", unit: None, choices: None }),
        ("layers[].soilType", NormFieldMeta { label_en: "Soil type", label_de: "Bodenart", unit: None, choices: Some(SOIL) }),
        ("layers[].depthTop", NormFieldMeta { label_en: "Depth top", label_de: "Tiefe oben", unit: Some("m"), choices: None }),
        ("layers[].depthBottom", NormFieldMeta { label_en: "Depth bottom", label_de: "Tiefe unten", unit: Some("m"), choices: None }),
        ("layers[].gamma", NormFieldMeta { label_en: "Unit weight γ", label_de: "Wichte γ", unit: Some("N/m³"), choices: None }),
        ("layers[].gammaPrime", NormFieldMeta { label_en: "Submerged unit weight γ′", label_de: "Wichte unter Auftrieb γ′", unit: Some("N/m³"), choices: None }),
        ("layers[].phiPrimeDeg", NormFieldMeta { label_en: "Friction angle φ′", label_de: "Reibungswinkel φ′", unit: Some("°"), choices: None }),
        ("layers[].cohesionEffective", NormFieldMeta { label_en: "Effective cohesion c′", label_de: "Effektive Kohäsion c′", unit: Some("Pa"), choices: None }),
        ("layers[].cohesionUndrained", NormFieldMeta { label_en: "Undrained cohesion c_u", label_de: "Undränierte Kohäsion c_u", unit: Some("Pa"), choices: None }),
        ("layers[].oedometricModulus", NormFieldMeta { label_en: "Oedometric modulus E_oed", label_de: "Steifemodul E_s / E_oed", unit: Some("Pa"), choices: None }),
        ("layers[].poissonRatio", NormFieldMeta { label_en: "Poisson ratio ν", label_de: "Querdehnzahl ν", unit: None, choices: None }),
        ("layers[].cptQc", NormFieldMeta { label_en: "CPT tip resistance q_c", label_de: "CPT-Spitzenwiderstand q_c", unit: Some("Pa"), choices: None }),
        ("layers[].sptN", NormFieldMeta { label_en: "SPT blow count N", label_de: "SPT-Schlagzahl N", unit: None, choices: None }),
        ("footings", NormFieldMeta { label_en: "Spread foundations", label_de: "Flachgründungen", unit: None, choices: None }),
        ("footings[].id", NormFieldMeta { label_en: "Footing ID", label_de: "Fundament-ID", unit: None, choices: None }),
        ("footings[].width", NormFieldMeta { label_en: "Width B", label_de: "Breite B", unit: Some("m"), choices: None }),
        ("footings[].length", NormFieldMeta { label_en: "Length L", label_de: "Länge L", unit: Some("m"), choices: None }),
        ("footings[].embedment", NormFieldMeta { label_en: "Embedment D_f", label_de: "Einbindetiefe D_f", unit: Some("m"), choices: None }),
        ("footings[].baseInclinationDeg", NormFieldMeta { label_en: "Base inclination", label_de: "Sohlneigung", unit: Some("°"), choices: None }),
        ("footings[].settlementLimit", NormFieldMeta { label_en: "Settlement limit", label_de: "Setzungsgrenze", unit: Some("m"), choices: None }),
        ("footings[].loadCases", NormFieldMeta { label_en: "Load cases", label_de: "Lastfälle", unit: None, choices: None }),
        ("footings[].loadCases[].id", NormFieldMeta { label_en: "Load case ID", label_de: "Lastfall-ID", unit: None, choices: None }),
        ("footings[].loadCases[].designSituation", NormFieldMeta { label_en: "Load-case design situation", label_de: "Lastfall-Bemessungssituation", unit: None, choices: Some(SITUATION) }),
        ("footings[].loadCases[].verticalPermanent", NormFieldMeta { label_en: "Vertical permanent V_G", label_de: "Vertikal ständig V_G", unit: Some("N"), choices: None }),
        ("footings[].loadCases[].verticalVariable", NormFieldMeta { label_en: "Vertical variable V_Q", label_de: "Vertikal veränderlich V_Q", unit: Some("N"), choices: None }),
        ("footings[].loadCases[].horizontalPermanent", NormFieldMeta { label_en: "Horizontal permanent H_G", label_de: "Horizontal ständig H_G", unit: Some("N"), choices: None }),
        ("footings[].loadCases[].horizontalVariable", NormFieldMeta { label_en: "Horizontal variable H_Q", label_de: "Horizontal veränderlich H_Q", unit: Some("N"), choices: None }),
        ("footings[].loadCases[].momentPermanent", NormFieldMeta { label_en: "Moment permanent M_G", label_de: "Moment ständig M_G", unit: Some("N·m"), choices: None }),
        ("footings[].loadCases[].momentVariable", NormFieldMeta { label_en: "Moment variable M_Q", label_de: "Moment veränderlich M_Q", unit: Some("N·m"), choices: None }),
        ("piles", NormFieldMeta { label_en: "Piles", label_de: "Pfähle", unit: None, choices: None }),
        ("piles[].id", NormFieldMeta { label_en: "Pile ID", label_de: "Pfahl-ID", unit: None, choices: None }),
        ("piles[].pileType", NormFieldMeta { label_en: "Pile type", label_de: "Pfahlart", unit: None, choices: Some(PILE_TYPE) }),
        ("piles[].diameter", NormFieldMeta { label_en: "Diameter D", label_de: "Durchmesser D", unit: Some("m"), choices: None }),
        ("piles[].length", NormFieldMeta { label_en: "Length L", label_de: "Länge L", unit: Some("m"), choices: None }),
        ("piles[].count", NormFieldMeta { label_en: "Pile count", label_de: "Pfahlanzahl", unit: None, choices: None }),
        ("piles[].alphaS", NormFieldMeta { label_en: "Shaft factor α_s", label_de: "Mantelfaktor α_s", unit: None, choices: None }),
        ("piles[].unitShaftResistance", NormFieldMeta { label_en: "Unit shaft resistance q_s", label_de: "Mantelwiderstand q_s", unit: Some("Pa"), choices: None }),
        ("piles[].unitBaseResistance", NormFieldMeta { label_en: "Unit base resistance q_b", label_de: "Spitzendruck q_b", unit: Some("Pa"), choices: None }),
        ("piles[].compressionPermanent", NormFieldMeta { label_en: "Compression permanent", label_de: "Druck ständig", unit: Some("N"), choices: None }),
        ("piles[].compressionVariable", NormFieldMeta { label_en: "Compression variable", label_de: "Druck veränderlich", unit: Some("N"), choices: None }),
        ("piles[].tensionPermanent", NormFieldMeta { label_en: "Tension permanent", label_de: "Zug ständig", unit: Some("N"), choices: None }),
        ("piles[].tensionVariable", NormFieldMeta { label_en: "Tension variable", label_de: "Zug veränderlich", unit: Some("N"), choices: None }),
        ("piles[].testProfiles", NormFieldMeta { label_en: "Pile test profiles", label_de: "Pfahl-Prüfprofile", unit: None, choices: None }),
        ("piles[].testProfiles[].id", NormFieldMeta { label_en: "Test profile ID", label_de: "Prüfprofil-ID", unit: None, choices: None }),
        ("piles[].testProfiles[].shaftResistance", NormFieldMeta { label_en: "Measured shaft resistance", label_de: "Gemessener Mantelwiderstand", unit: Some("N"), choices: None }),
        ("piles[].testProfiles[].baseResistance", NormFieldMeta { label_en: "Measured base resistance", label_de: "Gemessener Spitzenwiderstand", unit: Some("N"), choices: None }),
        ("retainingWalls", NormFieldMeta { label_en: "Retaining walls", label_de: "Stützbauwerke", unit: None, choices: None }),
        ("retainingWalls[].id", NormFieldMeta { label_en: "Wall ID", label_de: "Wand-ID", unit: None, choices: None }),
        ("retainingWalls[].height", NormFieldMeta { label_en: "Wall height", label_de: "Wandhöhe", unit: Some("m"), choices: None }),
        ("retainingWalls[].embedment", NormFieldMeta { label_en: "Embedment", label_de: "Einbindung", unit: Some("m"), choices: None }),
        ("retainingWalls[].baseWidth", NormFieldMeta { label_en: "Base width", label_de: "Sohlbreite", unit: Some("m"), choices: None }),
        ("retainingWalls[].stemThickness", NormFieldMeta { label_en: "Stem thickness", label_de: "Schaftdicke", unit: Some("m"), choices: None }),
        ("retainingWalls[].backfillPhiDeg", NormFieldMeta { label_en: "Backfill φ′", label_de: "Hinterfüllung φ′", unit: Some("°"), choices: None }),
        ("retainingWalls[].backfillGamma", NormFieldMeta { label_en: "Backfill γ", label_de: "Hinterfüllung γ", unit: Some("N/m³"), choices: None }),
        ("retainingWalls[].wallFrictionDeg", NormFieldMeta { label_en: "Wall friction δ", label_de: "Wandreibung δ", unit: Some("°"), choices: None }),
        ("retainingWalls[].earthPressureMode", NormFieldMeta { label_en: "Earth-pressure mode", label_de: "Erddruckansatz", unit: None, choices: Some(EP_MODE) }),
        ("retainingWalls[].wallMovement", NormFieldMeta { label_en: "Wall movement class", label_de: "Wandbewegungsklasse", unit: None, choices: Some(WALL_MOVE) }),
        ("retainingWalls[].concreteGamma", NormFieldMeta { label_en: "Concrete unit weight γ_c", label_de: "Beton-Wichte γ_c", unit: Some("N/m³"), choices: None }),
        ("retainingWalls[].ocr", NormFieldMeta { label_en: "OCR", label_de: "OCR", unit: None, choices: None }),
        ("retainingWalls[].surcharge", NormFieldMeta { label_en: "Surcharge", label_de: "Auflast", unit: Some("Pa"), choices: None }),
        ("retainingWalls[].verticalPermanent", NormFieldMeta { label_en: "Vertical permanent", label_de: "Vertikal ständig", unit: Some("N"), choices: None }),
        ("retainingWalls[].horizontalPermanent", NormFieldMeta { label_en: "Horizontal permanent", label_de: "Horizontal ständig", unit: Some("N"), choices: None }),
        ("slopes", NormFieldMeta { label_en: "Slopes", label_de: "Böschungen", unit: None, choices: None }),
        ("slopes[].id", NormFieldMeta { label_en: "Slope ID", label_de: "Böschungs-ID", unit: None, choices: None }),
        ("slopes[].angleDeg", NormFieldMeta { label_en: "Slope angle", label_de: "Böschungswinkel", unit: Some("°"), choices: None }),
        ("slopes[].height", NormFieldMeta { label_en: "Slope height", label_de: "Böschungshöhe", unit: Some("m"), choices: None }),
        ("slopes[].length", NormFieldMeta { label_en: "Slope length", label_de: "Böschungslänge", unit: Some("m"), choices: None }),
        ("slopes[].governingLayerId", NormFieldMeta { label_en: "Governing layer id", label_de: "Maßgebende Schicht-Id", unit: None, choices: None }),
        ("upliftCases", NormFieldMeta { label_en: "Uplift / heave cases", label_de: "Aufschwimmen / HYD", unit: None, choices: None }),
        ("upliftCases[].id", NormFieldMeta { label_en: "Uplift case ID", label_de: "Aufschwimmfall-ID", unit: None, choices: None }),
        ("upliftCases[].permanentStabilizing", NormFieldMeta { label_en: "Permanent stabilizing", label_de: "Ständig stabilisierend", unit: Some("N"), choices: None }),
        ("upliftCases[].permanentDestabilizing", NormFieldMeta { label_en: "Permanent destabilizing", label_de: "Ständig destabilisierend", unit: Some("N"), choices: None }),
        ("upliftCases[].variableDestabilizing", NormFieldMeta { label_en: "Variable destabilizing", label_de: "Veränderlich destabilisierend", unit: Some("N"), choices: None }),
        ("upliftCases[].porePressure", NormFieldMeta { label_en: "Pore pressure u", label_de: "Porenwasserdruck u", unit: Some("Pa"), choices: None }),
        ("upliftCases[].totalStress", NormFieldMeta { label_en: "Total stress σ", label_de: "Totalspannung σ", unit: Some("Pa"), choices: None }),
    ];
    lookup_norm_field_meta(TABLE, path)
}



//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
