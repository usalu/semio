//! 🧬️ En1990 snapshot — complete basis-of-design subject (project → actions → members).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
pub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 🏗 Kind of structure: building or bridge type, selecting the action sets and the ψ factors (EN 1990 Annex A1/A2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum StructureKind {
    Building,
    RoadBridge,
    Footbridge,
    RailBridge,
}

/// 🧐 Design supervision level DSL1 to DSL3 (EN 1990 Annex B Table B4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SupervisionLevel {
    Dsl1,
    Dsl2,
    Dsl3,
}

/// 🔍 Inspection level IL1 to IL3 during execution (EN 1990 Annex B Table B5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum InspectionLevel {
    Il1,
    Il2,
    Il3,
}

/// 📸️ Persisted EN 1990 basis-of-design subject. Forces in N, lengths in m, frequencies in Hz.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(extension = "en1990")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub project_id: String,
    #[state(artifact)]
    pub structure_kind: StructureKind,
    #[state(artifact)]
    pub altitude_m: f64,
    #[state(artifact)]
    pub consequence_class: u8,
    #[state(artifact)]
    pub reliability_class: ReliabilityClass,
    #[state(artifact)]
    pub design_working_life_category: u8,
    #[state(artifact)]
    pub design_working_life_years: f64,
    #[state(artifact)]
    pub reference_period_years: f64,
    #[state(artifact)]
    pub supervision_level: SupervisionLevel,
    #[state(artifact)]
    pub inspection_level: InspectionLevel,
    #[state(artifact)]
    pub beta_computed: f64,
    #[dsl(table)]
    #[state(artifact)]
    pub permanents: Vec<PermanentAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub variables: Vec<VariableAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub accidentals: Vec<AccidentalAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub seismics: Vec<SeismicAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub members: Vec<Member>,
    #[dsl(table)]
    #[state(artifact)]
    pub bridge_sls: Vec<BridgeSls>,
    #[dsl(table)]
    #[state(artifact)]
    pub effects: Vec<MemberEffect>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_spectra: Vec<FatigueSpectrum>,
    #[dsl(table)]
    #[state(artifact)]
    pub reliability_analyses: Vec<ReliabilityAnalysis>,
    #[dsl(table)]
    #[state(artifact)]
    pub test_series: Vec<TestSeries>,
    #[dsl(table)]
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[dsl(table)]
    #[state(artifact)]
    pub cases: Vec<CaseRow>,
    #[dsl(table)]
    #[state(artifact)]
    pub factor_overrides: Vec<FactorOverride>,
    #[dsl(table)]
    #[state(artifact)]
    pub traffic_groups: Vec<TrafficGroup>,
}

impl En1990Snapshot {
    /// 📸️ Reference EN 1990 subject under the explicitly chosen national annex; the annex is never defaulted.
    pub fn example(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
        Self {
            annex,
            project_id: "office-cc2".into(),
            structure_kind: StructureKind::Building,
            altitude_m: 1200.0,
            consequence_class: 2,
            reliability_class: ReliabilityClass::RC2,
            design_working_life_category: 4,
            design_working_life_years: 50.0,
            reference_period_years: 50.0,
            supervision_level: SupervisionLevel::Dsl2,
            inspection_level: InspectionLevel::Il2,
            beta_computed: 3.85,
            permanents: vec![
                PermanentAction { id: "G-sup".into(), kind: PermanentKind::GSup, gk: 80_000.0 },
                PermanentAction { id: "G-inf".into(), kind: PermanentKind::GInf, gk: 20_000.0 },
            ],
            variables: vec![
                VariableAction { id: "Q-office".into(), category: "office".into(), qk: 40_000.0, group: String::new() },
                VariableAction { id: "Q-wind".into(), category: "wind".into(), qk: 20_000.0, group: String::new() },
                VariableAction { id: "Q-snow".into(), category: "snow".into(), qk: 15_000.0, group: String::new() },
            ],
            accidentals: vec![],
            seismics: vec![],
            members: vec![Member {
                id: "beam-B1".into(),
                label_en: "Office beam B1".into(),
                label_de: "Büroträger B1".into(),
                rd_str: 250_000.0,
                rd_geo: 250_000.0,
                rd_equ_stab: 200_000.0,
                rd_equ_destab: 180_000.0,
                rd_fat: 0.0,
                span: 6.0,
                precamber: 0.0,
                deflection_w1: 0.006,
                deflection_w2: 0.003,
                deflection_w3: 0.009,
                deflection_limit_ratio: 250.0,
                deflection_limit_ratio_w3: 300.0,
                sls_limit_char: 250_000.0,
                sls_limit_freq: 250_000.0,
                sls_limit_qp: 250_000.0,
                vibration_frequency: 5.5,
                vibration_frequency_min: 3.0,
                frame_height: 0.0,
                frame_drift: 0.0,
                storey_height: 0.0,
                storey_drift: 0.0,
                material_strength_k: 0.0,
                material_conversion_factor: 0.0,
                material_partial_factor: 0.0,
                material_design_strength: 0.0,
                nominal_dimension: 0.0,
                dimension_tolerance: 0.0,
                design_dimension: 0.0,
                member_alpha_cr: 0.0,
                second_order_analysis: 0.0,
            }],
            bridge_sls: vec![],
            effects: vec![
                MemberEffect { member_id: "beam-B1".into(), action_id: "G-sup".into(), influence: 1.0 },
                MemberEffect { member_id: "beam-B1".into(), action_id: "G-inf".into(), influence: -1.0 },
                MemberEffect { member_id: "beam-B1".into(), action_id: "Q-office".into(), influence: 1.0 },
                MemberEffect { member_id: "beam-B1".into(), action_id: "Q-wind".into(), influence: 1.0 },
                MemberEffect { member_id: "beam-B1".into(), action_id: "Q-snow".into(), influence: 1.0 },
            ],
            fatigue_spectra: vec![],
            reliability_analyses: vec![],
            test_series: vec![],
            placements: vec![],
            cases: vec![CaseRow {
                id: "case-durability".into(),
                label_en: "Durability period of the office".into(),
                label_de: "Dauerhaftigkeitszeitraum des Bürogebäudes".into(),
                entry_id: "E90.04".into(),
                numbers: vec![CaseNumber { id: "years".into(), value: 50.0 }],
                texts: vec![],
            }],
            factor_overrides: vec![],
            traffic_groups: vec![],
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

/// ⚖️ Kind of a permanent action: unfavourable, favourable or prestress.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum PermanentKind {
    GSup,
    GInf,
    Prestress,
}

/// ⚓️ Permanent action entry (G_k,sup / G_k,inf / prestress) — forces in N.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PermanentAction {
    pub id: String,
    pub kind: PermanentKind,
    pub gk: f64,
}

/// 🏋️ Variable action — category selects ψ (site altitude is on the snapshot); actions sharing a non-empty `group` are mutually exclusive (EN 1990 §4.1.1, pattern loading per span: one action per loaded span in one group).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct VariableAction {
    pub id: String,
    pub category: String,
    pub qk: f64,
    pub group: String,
}

/// 💥 Accidental design action A_d [N] — EN 1990 §1.5.3.5 / §6.4.3.3 (A_d is already a design value).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AccidentalAction {
    pub id: String,
    pub ad: f64,
}

/// 🛡️ Reliability class RC1 to RC3 of EN 1990 Annex B, shared by every Eurocode artifact (`ActionFactors` reads K_FI of the class from the registry).
pub use semio_s_plugin_norm_compliance::ReliabilityClass;

/// 🚦️ Traffic load group of EN 1991-2 Table 4.4a as used by EN 1990 Annex A2: the groups are mutually exclusive; the per-component coefficients are registry rows `ndp.en1990.a2.group.<group>.<category>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum LoadGroup {
    #[dsl(key = "gr1a")]
    GR1A,
    #[dsl(key = "gr1b")]
    GR1B,
    #[dsl(key = "gr2")]
    GR2,
    #[dsl(key = "gr3")]
    GR3,
    #[dsl(key = "gr4")]
    GR4,
    #[dsl(key = "gr5")]
    GR5,
    #[dsl(key = "gr6")]
    GR6,
}

impl LoadGroup {
    pub const ALL: [Self; 7] = [Self::GR1A, Self::GR1B, Self::GR2, Self::GR3, Self::GR4, Self::GR5, Self::GR6];

    /// 🏷️ The lower-case group id (`gr1a`) that keys the registry rows and the generator's exclusive groups.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GR1A => "gr1a",
            Self::GR1B => "gr1b",
            Self::GR2 => "gr2",
            Self::GR3 => "gr3",
            Self::GR4 => "gr4",
            Self::GR5 => "gr5",
            Self::GR6 => "gr6",
        }
    }

    /// 🔎️ The group of a lower-case id, `None` outside gr1a to gr6.
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|key| key.as_str() == id)
    }
}

/// 🏛 Seismic importance class — EN 1998-1 Table 4.3 / DIN EN 1998-1/NA Table NA.5 (γ_I).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ImportanceClass {
    #[dsl(key = "i")]
    I,
    #[dsl(key = "ii")]
    II,
    #[dsl(key = "iii")]
    III,
    #[dsl(key = "iv")]
    IV,
}

impl ImportanceClass {
    /// 🏛 Importance factor γ_I per EN 1998-1 Table 4.3 (DE NA Table NA.5 identical).
    pub fn gamma_i(self) -> f64 {
        match self {
            Self::I => 0.8,
            Self::II => 1.0,
            Self::III => 1.2,
            Self::IV => 1.4,
        }
    }
}

/// 🌋️ Characteristic seismic action A_Ek [N] with importance class; A_Ed = γ_I · A_Ek (EN 1990 §6.4.3.4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SeismicAction {
    pub id: String,
    pub a_ek: f64,
    pub importance_class: ImportanceClass,
}

impl SeismicAction {
    /// 🌕️ Design seismic action A_Ed = γ_I · A_Ek (EN 1990 §6.4.3.4).
    pub fn a_ed(&self) -> f64 {
        self.importance_class.gamma_i() * self.a_ek
    }
}

/// 🏗️ Member / verification point with design resistances and SLS criteria (SI: N, m, Hz): `precamber` is w_c, `deflection_w1..3` the Fig. A1.1 components, `sls_limit_*` the design criterion C_d of Eq. 6.13 per combination family.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Member {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub rd_str: f64,
    pub rd_geo: f64,
    pub rd_equ_stab: f64,
    pub rd_equ_destab: f64,
    pub rd_fat: f64,
    pub span: f64,
    pub precamber: f64,
    pub deflection_w1: f64,
    pub deflection_w2: f64,
    pub deflection_w3: f64,
    pub deflection_limit_ratio: f64,
    pub deflection_limit_ratio_w3: f64,
    pub sls_limit_char: f64,
    pub sls_limit_freq: f64,
    pub sls_limit_qp: f64,
    pub vibration_frequency: f64,
    pub vibration_frequency_min: f64,
    pub frame_height: f64,
    pub frame_drift: f64,
    pub storey_height: f64,
    pub storey_drift: f64,
    pub material_strength_k: f64,
    pub material_conversion_factor: f64,
    pub material_partial_factor: f64,
    pub material_design_strength: f64,
    pub nominal_dimension: f64,
    pub dimension_tolerance: f64,
    pub design_dimension: f64,
    pub member_alpha_cr: f64,
    pub second_order_analysis: f64,
}

/// 🌉 Annex A2.4 SLS criteria for a bridge member (only used when structureKind is a bridge).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BridgeSls {
    pub id: String,
    pub member_id: String,
    pub deck_acceleration: f64,
    pub deck_acceleration_limit: f64,
    pub deck_twist: f64,
    pub deck_twist_limit: f64,
    pub bridge_deflection: f64,
    pub bridge_deflection_limit: f64,
    pub track_gauge: f64,
    pub line_speed: f64,
    pub ballast_depth: f64,
    pub deck_mass: f64,
    pub natural_frequency: f64,
    pub end_rotation: f64,
    pub basic_wind_speed: f64,
    pub damping_ratio: f64,
}

/// 🔗 Influence coefficient mapping a catalogue action magnitude onto a member effect (dimensionless).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MemberEffect {
    pub member_id: String,
    pub action_id: String,
    pub influence: f64,
}

/// 🔩 Palmgren-Miner spectrum bin of a member — EN 1990 §6.4.5 with the EN 1993-1-9 S-N curve; stresses in Pa.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueSpectrum {
    pub id: String,
    pub member_id: String,
    pub detail_category: f64,
    pub stress_range: f64,
    pub cycles: f64,
}

/// 📈 Probability distribution family of a basic variable (EN 1990 Annex C).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum DistributionKind {
    Normal,
    Lognormal,
    Gumbel,
    Weibull,
}

/// 🎯 FORM run for g = R − E — EN 1990 Annex C.5 to C.7; basic variables by distribution, mean and standard deviation.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ReliabilityAnalysis {
    pub id: String,
    pub resistance_distribution: DistributionKind,
    pub resistance_mean: f64,
    pub resistance_sd: f64,
    pub action_distribution: DistributionKind,
    pub action_mean: f64,
    pub action_sd: f64,
    /// 📐 Distribution of the second action in g = R − (E₁ + E₂). An empty analysis uses normal.
    pub second_action_distribution: DistributionKind,
    /// ➕ Mean μ_E2 of the second action, in newtons.
    pub second_action_mean: f64,
    /// 📏 Standard deviation σ_E2 of the second action, in newtons.
    pub second_action_sd: f64,
    /// 🏗️ Id of the member whose reliability this analysis represents: the member shows a 3D reliability badge with the FORM index; empty = the analysis belongs to the project.
    pub member_id: String,
}

/// 🧪 Distribution assumed for the design value derived from tests (EN 1990 D.7.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum TestDistribution {
    Normal,
    Lognormal,
}

/// 🧪 Design value from tests — EN 1990 Annex D.7: X_d = η·(m − k_n·s), logarithmic for a lognormal quantity.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TestSeries {
    pub id: String,
    pub quantity: String,
    pub distribution: TestDistribution,
    pub sample_size: u32,
    pub mean: f64,
    pub std_dev: f64,
    pub variation_known: f64,
    pub conversion_factor: f64,
    pub required: f64,
    /// 🧪 Experimental and theoretical resistance of the same specimen as r_e:r_t pairs separated by semicolons for the standard evaluation of Annex D.7.2; empty = not given, the summary statistics are used.
    pub pairs: String,
    /// 📉 Coefficient of variation V_rt of the basic variables of the resistance function for the standard evaluation of Annex D.7.2; used only with test pairs.
    pub theory_variation: f64,
    /// 🧮 Mean value of the resistance model used with the test pairs of Annex D.7.2; 0 selects the summary statistics.
    pub theory_mean: f64,
}

/// 📍️ Persisted pose of one scene entity (`id` is the entity id; SI: m, rad). Entities without a placement follow the documented default layout.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Placement {
    pub id: String,
    /// ↔️ Origin x [m].
    pub x: f64,
    /// ↕️ Origin y [m].
    pub y: f64,
    /// ⬆️ Origin z [m].
    pub z: f64,
    /// 🧭️ Yaw about +Z [rad].
    pub yaw: f64,
}

/// 🎚️ Project override of one annex factor (γ, ψ₀/ψ₁/ψ₂, ξ or K_FI): applied by the combination generator only where the annex allows it; every row deviates from the National Annex and carries its justification.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FactorOverride {
    pub id: String,
    /// 🏷️ `gamma`, `psi0`, `psi1`, `psi2`, `xi` or `k_fi`.
    pub factor: String,
    /// 🎯️ Action id, exclusive group id or ψ category; empty = every action.
    pub target: String,
    /// 🔢️ The replacement value.
    pub value: f64,
    /// 📝️ Why the project deviates from the National Annex.
    pub justification: String,
}

/// 🚦️ One load component of a traffic load group: the variable action is a member of exactly this group, groups never act together; the coefficient of the component in the group is the registry row `ndp.en1990.a2.group.<group>.<category>`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TrafficGroup {
    pub id: String,
    /// 🚦️ The load group of EN 1991-2 Table 4.4a.
    pub group: LoadGroup,
    /// 🔗️ Id of the variable action that is the component.
    pub variable_id: String,
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1990", "EN 1990", EN1990_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1990_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1990", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1990_DOCUMENT_SCHEMA: &str = "semio.norm.en1990/v1";

#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "../../🚪️io/🪞️records/🦀️.rs"]
mod records;

impl DistributionKind {
    /// 🔤 Registry and kernel key of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Lognormal => "lognormal",
            Self::Gumbel => "gumbel",
            Self::Weibull => "weibull",
        }
    }
}

impl std::fmt::Display for DistributionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TestDistribution {
    /// 🔤 Registry and kernel key of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Lognormal => "lognormal",
        }
    }
}

impl std::fmt::Display for TestDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PermanentKind {
    /// 🔤 Registry and kernel key of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GSup => "g_sup",
            Self::GInf => "g_inf",
            Self::Prestress => "prestress",
        }
    }
}

impl std::fmt::Display for PermanentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl StructureKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Building => "building",
            Self::RoadBridge => "road_bridge",
            Self::Footbridge => "footbridge",
            Self::RailBridge => "rail_bridge",
        }
    }

    /// 🏗️ Annex A1 (buildings) or Annex A2 (bridges) the structure kind selects.
    pub const fn domain(self) -> semio_s_plugin_norm_compliance::action_factors::Domain {
        match self {
            Self::Building => semio_s_plugin_norm_compliance::action_factors::Domain::Building,
            Self::RoadBridge | Self::Footbridge | Self::RailBridge => semio_s_plugin_norm_compliance::action_factors::Domain::Bridge,
        }
    }

    /// 🌉 True for the three bridge kinds.
    pub const fn is_bridge(self) -> bool {
        matches!(self.domain(), semio_s_plugin_norm_compliance::action_factors::Domain::Bridge)
    }
}

impl std::fmt::Display for StructureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SupervisionLevel {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dsl1 => "DSL1",
            Self::Dsl2 => "DSL2",
            Self::Dsl3 => "DSL3",
        }
    }
}

impl std::fmt::Display for SupervisionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl InspectionLevel {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Il1 => "IL1",
            Self::Il2 => "IL2",
            Self::Il3 => "IL3",
        }
    }
}

impl std::fmt::Display for InspectionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
