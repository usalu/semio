//! 🌋️ EN 1998 artifact root — snapshot re-export and facet modules.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_norm_contract::{app_surface, document, impl_norm_artifact_record, norm_owned_tool_job_factory, norm_results_window_config_owner, results_window_config};

/// 📜 Language-neutral package declaration owned by this artifact.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📦 Validates this artifact's independently compiled package identity.
pub fn package_descriptor() -> Result<semio_s_artifact_norm_contract::NormArtifactPackage, semio_s_artifact_norm_contract::PackageSchemaError> {
    semio_s_artifact_norm_contract::package_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1998", "EN 1998")
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1998_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1998", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const EN1998_DOCUMENT_SCHEMA: &str = "semio.norm.en1998/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.en1998" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.en1998.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.norm.en1998@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.en1998/v1" }, Q { namespace: "codec-extension", value: "20:semio.norm.en1998/v1:en1998" }];
    const EN: &[L] = &[L { locale: "en", text: "EN 1998 design of structures for earthquake resistance" }];
    const DE: &[L] = &[L { locale: "de", text: "EN 1998 Auslegung von Bauwerken gegen Erdbeben" }];
    const ROWS: &[C] = &[
        C { identity: "s.norm.en1998.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.norm.en1998.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.norm.en1998.schema.artifact", kind: "schema", descriptor: "s.norm.en1998", claims: S, localizations: &[] },
        C { identity: "s.norm.en1998.inference.outline", kind: "inference", descriptor: "s.norm.en1998.inference", claims: I, localizations: &[] },
        C { identity: "s.norm.en1998.composer.any", kind: "composer", descriptor: "s.norm.en1998@1/*", claims: M, localizations: &[] },
        C { identity: "s.norm.en1998.grammar.document", kind: "grammar", descriptor: "en1998.document", claims: &[Q { namespace: "grammar", value: "en1998.document" }], localizations: &[] },
        C { identity: "s.norm.en1998.grammar.op", kind: "grammar", descriptor: "en1998.op", claims: &[Q { namespace: "grammar", value: "en1998.op" }], localizations: &[] },
        C { identity: "s.norm.en1998.grammar.diff", kind: "grammar", descriptor: "en1998.diff", claims: &[Q { namespace: "grammar", value: "en1998.diff" }], localizations: &[] },
        C { identity: "s.norm.en1998.grammar.pack", kind: "grammar", descriptor: "en1998.pack", claims: &[Q { namespace: "grammar", value: "en1998.pack" }], localizations: &[] },
        C { identity: "s.norm.en1998.grammar.spr", kind: "grammar", descriptor: "en1998.spr", claims: &[Q { namespace: "grammar", value: "en1998.spr" }], localizations: &[] },
        C { identity: "s.norm.en1998.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1998/v1:en1998", claims: K, localizations: &[] },
        C { identity: "s.norm.en1998.localization.en", kind: "localization", descriptor: "EN 1998 design of structures for earthquake resistance", claims: &[], localizations: EN },
        C { identity: "s.norm.en1998.localization.de", kind: "localization", descriptor: "EN 1998 Auslegung von Bauwerken gegen Erdbeben", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.en1998", ROWS)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(artifact_schema::en1998_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::en1998_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<editor::en1998::En1998PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "en1998.document",
                    extension: Some("en1998"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1998.document"),
                },
                dsl::LanguageSpec {
                    id: "en1998.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1998.op"),
                },
                dsl::LanguageSpec {
                    id: "en1998.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1998.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1998.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1998.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1998.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1998.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration


// #region 🔖️Types

/// 🌩 DE seismic zone per DIN EN 1998-1/NA (zones 0–3 only; schema-restricted).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum DeSeismicZone {
    #[dsl(key = "zone0")]
    Zone0,
    #[dsl(key = "zone1")]
    Zone1,
    #[dsl(key = "zone2")]
    Zone2,
    #[dsl(key = "zone3")]
    Zone3,
}

impl DeSeismicZone {
    /// 📐️ Reference PGA a_gR [m/s²] per DIN EN 1998-1/NA Table NA.1 (2011).
    pub fn a_gr(self) -> f64 {
        match self {
            Self::Zone0 => 0.0,
            Self::Zone1 => 0.4,
            Self::Zone2 => 0.6,
            Self::Zone3 => 0.8,
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            Self::Zone0 => 0,
            Self::Zone1 => 1,
            Self::Zone2 => 2,
            Self::Zone3 => 3,
        }
    }
}

/// 🪨 DE-NA ground/subsoil combination (DIN EN 1998-1/NA Table NA.4) — sole DE ground subject field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum DeGroundCombo {
    #[dsl(key = "A-R")]
    #[value(rename = "A-R")]
    #[cfg_attr(test, serde(rename = "A-R"))]
    AR,
    #[dsl(key = "B-R")]
    #[value(rename = "B-R")]
    #[cfg_attr(test, serde(rename = "B-R"))]
    BR,
    #[dsl(key = "C-R")]
    #[value(rename = "C-R")]
    #[cfg_attr(test, serde(rename = "C-R"))]
    CR,
    #[dsl(key = "B-T")]
    #[value(rename = "B-T")]
    #[cfg_attr(test, serde(rename = "B-T"))]
    BT,
    #[dsl(key = "C-T")]
    #[value(rename = "C-T")]
    #[cfg_attr(test, serde(rename = "C-T"))]
    CT,
    #[dsl(key = "C-S")]
    #[value(rename = "C-S")]
    #[cfg_attr(test, serde(rename = "C-S"))]
    CS,
}

impl DeGroundCombo {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AR => "A-R",
            Self::BR => "B-R",
            Self::CR => "C-R",
            Self::BT => "B-T",
            Self::CT => "C-T",
            Self::CS => "C-S",
        }
    }
}


/// 🌚️ Seismic site parameters (DE zones 0–3 + DE-NA ground/subsoil, or EN A–E spectrum).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Site {
    pub seismic_zone: DeSeismicZone,
    /// EN annex reference PGA [m/s²]. For DE annex, evaluate derives a_gR from `seismic_zone` (Table NA.1).
    #[dsl(unit = "m/s2")]
    pub a_gr: f64,
    pub de_ground_combo: DeGroundCombo,
    /// EN annex ground type A–E (EN 1998-1 Table 3.1). Empty under DE annex.
    #[cfg_attr(test, serde(default, skip_serializing_if = "String::is_empty"))]
    pub en_ground_type: String,
    /// EN annex spectrum type (`type1` | `type2`). Empty under DE annex.
    #[cfg_attr(test, serde(default, skip_serializing_if = "String::is_empty"))]
    pub en_spectrum_type: String,
    pub importance_class: String,
}

/// 🏗️ Lateral-force-resisting system for one horizontal direction.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998System {
    pub id: String,
    pub direction: String,
    pub system_type: String,
    pub material: String,
    pub ductility_class: String,
    pub q0: f64,
    pub alpha_u_over_alpha_1: f64,
    pub k_w: f64,
    #[dsl(unit = "N")]
    pub base_shear_resistance_n: f64,
}

/// 🏋️ Variable action Q_k,i — category selects ψ₂ (EN 1990 DE NA); φ from EN 1998-1 Table 4.2.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998VariableAction {
    pub id: String,
    /// Imposed-load category (A–H / snow / wind) for ψ₂.
    pub category: String,
    #[dsl(unit = "N")]
    pub qk_n: f64,
}

/// 🏬 Storey — seismic mass from ΣG_k + Σψ_E,i·Q_k,i (EN 1998-1 §3.2.4 / EN 1990 6.12b).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Storey {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    /// Characteristic permanent action ΣG_k [N] on the storey.
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    /// EN 1998-1 Table 4.2 correlated occupancies (φ = 0.8 storeys / 1.0 roof).
    pub correlated_occupancy: bool,
    #[dsl(table)]
    pub variables: Vec<En1998VariableAction>,
    pub stiffness_x: f64,
    pub stiffness_y: f64,
    #[dsl(unit = "m")]
    pub centre_of_mass_x_m: f64,
    #[dsl(unit = "m")]
    pub centre_of_mass_y_m: f64,
    #[dsl(unit = "m")]
    pub centre_of_stiffness_x_m: f64,
    #[dsl(unit = "m")]
    pub centre_of_stiffness_y_m: f64,
    #[dsl(unit = "m")]
    pub drift_x_m: f64,
    #[dsl(unit = "m")]
    pub drift_y_m: f64,
    /// Per-storey shear resistance V_Rd,i [N] for storey-force verification.
    #[dsl(unit = "N")]
    pub shear_resistance_n: f64,
}

/// 🧱 Member summary where material ductility rules apply.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Member {
    pub id: String,
    pub material: String,
    pub role: String,
    pub detailing_compatible_with_q: bool,
    /// RC column min cross-section dimension b [m] (§5.4.3 / §5.5.3).
    #[dsl(unit = "m")]
    pub min_dimension_m: f64,
    /// RC longitudinal reinforcement ratio ρ.
    pub rho: f64,
    /// RC compression reinforcement ratio ρ′ (beams).
    pub rho_prime: f64,
    /// RC confinement mechanical volumetric ratio ω_wd.
    pub omega_wd: f64,
    /// Steel cross-section class (1–4) for q-compatibility (§6.5 / §6.6).
    pub steel_section_class: u8,
}

/// 🏢 Building subject for EN 1998-1.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Building {
    pub id: String,
    pub name: String,
    /// Plan dimension Lx [m] for accidental eccentricity (§4.3.2).
    #[dsl(unit = "m")]
    pub plan_width_m: f64,
    /// Plan dimension Ly [m] for accidental eccentricity (§4.3.2).
    #[dsl(unit = "m")]
    pub plan_length_m: f64,
    #[dsl(table)]
    pub systems: Vec<En1998System>,
    #[dsl(table)]
    pub storeys: Vec<En1998Storey>,
    #[dsl(table)]
    pub members: Vec<En1998Member>,
    pub plan_regular: bool,
    pub elevation_regular: bool,
    pub t1_method: String,
    pub t1_given_s: f64,
    pub ct: f64,
    pub drift_limit_class: String,
    pub nu: f64,
    pub multiple_resisting_systems: bool,
    pub claims_simple_masonry: bool,
    pub masonry_wall_area_ratio: f64,
    pub accidental_eccentricity_ratio: f64,
}

/// 🌉 Bridge subject for EN 1998-2.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Bridge {
    pub id: String,
    /// Isolation period ratio T_isol/T_fixed (EN 1998-2).
    pub period_ratio: f64,
    /// Fundamental period T [s] for displacement / shear (EN 1998-2 §4.2).
    #[dsl(unit = "s")]
    pub fundamental_period_s: f64,
    #[dsl(unit = "N")]
    pub v_rd_n: f64,
    #[dsl(unit = "m")]
    pub bearing_d_rd_m: f64,
    /// Characteristic permanent action ΣG_k [N] (deck + self-weight).
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    /// EN 1998-1 Table 4.2 correlated occupancy flag for φ on variable actions.
    pub correlated_occupancy: bool,
    #[dsl(table)]
    pub variables: Vec<En1998VariableAction>,
}

/// 🔧 Retrofit assessment for EN 1998-3.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Assessment {
    pub id: String,
    pub knowledge_level: String,
    /// EN 1998-3 limit state: nc | sd | dl (Table 2.1 + DE NA).
    pub limit_state: String,
    /// Building id providing seismic demand (spectrum × mass model).
    pub supported_building_id: String,
    #[dsl(unit = "N")]
    pub r_k_n: f64,
    pub gamma_el: f64,
}

/// 🫙 Silo for EN 1998-4.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Silo {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub radius_m: f64,
    /// Structure permanent action ΣG_k [N].
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    /// Characteristic content action Q_k [N] at full fill.
    #[dsl(unit = "N")]
    pub content_qk_n: f64,
    /// Content category for ψ₂ (typically E storage).
    pub content_category: String,
    /// Design filling ratio 0…1 (EN 1998-4).
    pub filling_ratio: f64,
    #[dsl(unit = "N")]
    pub n_rd_n: f64,
    #[dsl(unit = "N")]
    pub v_rd_n: f64,
    pub q_nominal: f64,
}

/// 🛢️ Tank for EN 1998-4.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Tank {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub radius_m: f64,
    /// Structure permanent action ΣG_k [N].
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    /// Characteristic content action Q_k [N] at full fill.
    #[dsl(unit = "N")]
    pub content_qk_n: f64,
    /// Content category for ψ₂ (typically E storage / liquid).
    pub content_category: String,
    /// Design filling ratio 0…1 (EN 1998-4).
    pub filling_ratio: f64,
    #[dsl(unit = "N")]
    pub v_rd_n: f64,
}

/// 🪨 Foundation for EN 1998-5.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Foundation {
    pub id: String,
    /// Building id whose seismic base shear drives foundation demand (EN 1998-5 §5.3).
    pub supported_building_id: String,
    #[dsl(unit = "m2")]
    pub area_m2: f64,
    #[dsl(unit = "Pa")]
    pub p_rd_pa: f64,
    #[dsl(unit = "N")]
    pub h_rd_n: f64,
    pub k_foundation: f64,
    pub k_soil: f64,
}

/// 🧱 Retaining wall for EN 1998-5.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998RetainingWall {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    pub phi_deg: f64,
    pub soil_gamma: f64,
    pub r: f64,
    pub h_rd_n_per_m: f64,
}

/// 🗼 Tower / mast / chimney for EN 1998-6.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Tower {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    pub m_rd_nm: f64,
    pub is_chimney: bool,
    pub q_nominal: f64,
    /// Characteristic permanent action ΣG_k [N].
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    pub correlated_occupancy: bool,
    #[dsl(table)]
    pub variables: Vec<En1998VariableAction>,
}
// #endregion 🔖️Types


#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🏷️field-meta/🦀️.rs"]
pub mod field_meta;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod outline {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod change_annex {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️change-annex/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️change-annex/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️change-annex/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod update_site {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌚️update-site/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌚️update-site/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌚️update-site/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_building {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-building/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-building/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-building/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_building {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-building/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-building/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-building/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_system_base_shear_resistance_n {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️change-system-base-shear-resistance-n/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️change-system-base-shear-resistance-n/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️change-system-base-shear-resistance-n/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_storey_permanent_gk_n {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-storey-permanent-gk-n/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-storey-permanent-gk-n/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-storey-permanent-gk-n/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_storey_stiffness_x {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-storey-stiffness-x/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-storey-stiffness-x/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-storey-stiffness-x/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_storey_drift_xm {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-storey-drift-xm/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-storey-drift-xm/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-storey-drift-xm/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_building_plan_regular {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-building-plan-regular/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-building-plan-regular/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️change-building-plan-regular/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_building_elevation_regular {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-building-elevation-regular/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-building-elevation-regular/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-building-elevation-regular/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_member_detailing_compatible {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️change-member-detailing-compatible/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️change-member-detailing-compatible/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️change-member-detailing-compatible/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_building_masonry_wall_area_ratio {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-building-masonry-wall-area-ratio/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-building-masonry-wall-area-ratio/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-building-masonry-wall-area-ratio/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_bridge {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉insert-bridge/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉insert-bridge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉insert-bridge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_bridge_v_rd_n {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️change-bridge-v-rd-n/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️change-bridge-v-rd-n/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛑️change-bridge-v-rd-n/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_assessment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧insert-assessment/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧insert-assessment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧insert-assessment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_assessment_rkn {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-assessment-rkn/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-assessment-rkn/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-assessment-rkn/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_silo {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫙insert-silo/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫙insert-silo/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫙insert-silo/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_tank {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢insert-tank/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢insert-tank/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛢insert-tank/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_foundation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨insert-foundation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨insert-foundation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨insert-foundation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_retaining_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️insert-retaining-wall/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️insert-retaining-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️insert-retaining-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_tower {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼insert-tower/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼insert-tower/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼insert-tower/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_tower_m_rd_nm {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️change-tower-m-rd-nm/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️change-tower-m-rd-nm/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️change-tower-m-rd-nm/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod artifact_schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
}
pub mod spr {
    pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::diff::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
    }
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::mutations::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
    }
}
pub mod snapshot {
    pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
}
pub use crate::standards::v1::subsets::any::schema::diff::En1998Diff;
pub use crate::standards::v1::subsets::any::schema::mutations::En1998Mutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::En1998Snapshot;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod en1998 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs"]
            pub mod evaluate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☑️selected-check/🦀️.rs"]
            pub mod selected_check;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️set-snapshot/🦀️.rs"]
            pub mod set_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️set-field/🦀️.rs"]
            pub mod set_field;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕insert-item/🦀️.rs"]
            pub mod insert_item;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖remove-item/🦀️.rs"]
            pub mod remove_item;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹apply-remedy/🦀️.rs"]
            pub mod apply_remedy;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs"]
                    pub mod inputs;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod en1998 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs"]
                    pub mod report;
                }
            }
        }
    }
}

//#region 🪢️TaxonomyMounts
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-rc-frame/🦀️.rs"]
pub mod seismic_rc_frame;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/⚠️seismic-rc-frame-fail/🦀️.rs"]
pub mod seismic_rc_frame_fail;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-multipart/🦀️.rs"]
pub mod seismic_multipart;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/⚠️seismic-multipart-fail/🦀️.rs"]
pub mod seismic_multipart_fail;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-rc-frame/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
