//! En1994 — document entities (constitutional: general).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_norm_contract::{app_surface, document, impl_norm_artifact_record, norm_owned_tool_job_factory, norm_results_window_config_owner, results_window_config};

/// 📜 Language-neutral package declaration owned by this artifact.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📦 Validates this artifact's independently compiled package identity.
pub fn package_descriptor() -> Result<semio_s_artifact_norm_contract::NormArtifactPackage, semio_s_artifact_norm_contract::PackageSchemaError> {
    semio_s_artifact_norm_contract::package_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

//#region 🔖️Types

/// 🧱 Rolled or welded steel section geometry for a composite member (SI).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelSection {
    pub designation: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub width_m: f64,
    #[dsl(unit = "m")]
    pub tw_m: f64,
    #[dsl(unit = "m")]
    pub tf_m: f64,
    #[dsl(unit = "m2")]
    pub a_m2: f64,
    #[dsl(unit = "m3")]
    pub w_pl_y_m3: f64,
    #[dsl(unit = "m4")]
    pub i_y_m4: f64,
    #[dsl(unit = "m2")]
    pub a_v_m2: f64,
}

/// 🧱 Profiled steel sheeting under a composite slab (SI).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ProfiledSheeting {
    pub profile: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub rib_width_m: f64,
    #[dsl(unit = "m")]
    pub thickness_m: f64,
    pub ribs_parallel_to_beam: bool,
}

/// 🧱 Headed stud shear connectors on a composite beam (SI).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct HeadedStuds {
    #[dsl(unit = "m")]
    pub diameter_m: f64,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "Pa")]
    pub f_u_pa: f64,
    pub count_per_rib: u32,
    #[dsl(unit = "m")]
    pub spacing_m: f64,
    pub total_count: u32,
}

/// 📋 Characteristic action / load case under EN 1990 / EN 1991 (SI).
///
/// Area intensity `q_area_pa` [Pa] is the single source of truth for distributed loads
/// (line load = q_area × tributary width). When `q_area_pa ≈ 0`, optional point force `f_k_n`
/// supplies the sole characteristic concentrated load (EN 1990 combination of effects).
/// Fatigue uses Δσ_k / Δτ_k or FLM3 when `kind=fatigue`. Column N/M use [`ColumnAction`].
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CharacteristicAction {
    pub id: String,
    /// permanent | imposed | snow | wind | construction | fatigue
    pub kind: String,
    /// self_steel | wet_concrete | finishes | A..H | flm3 | …
    pub category: String,
    /// construction | composite
    pub stage: String,
    /// Characteristic area load q_k [Pa = N/m²] (× member tributary width → line).
    #[dsl(unit = "Pa")]
    pub q_area_pa: f64,
    /// Sole characteristic concentrated force [N] when `q_area_pa ≈ 0` (never both).
    #[dsl(unit = "N")]
    pub f_k_n: f64,
    #[dsl(unit = "Pa")]
    pub delta_sigma_k_pa: f64,
    #[dsl(unit = "Pa")]
    pub delta_tau_k_pa: f64,
}

/// 🏛️ Column characteristic N/M from structural analysis (EN 1990 action effects, SI).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ColumnAction {
    pub id: String,
    /// permanent | imposed | snow | wind | construction
    pub kind: String,
    pub category: String,
    /// construction | composite
    pub stage: String,
    #[dsl(unit = "N")]
    pub n_k_n: f64,
    /// Characteristic bending moment [N·m].
    pub m_k_nm: f64,
}

/// 🧱 Composite beam subject under EN 1994-1-1 / EN 1994-2 (SI).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeBeam {
    pub id: String,
    #[dsl(unit = "m")]
    pub span_m: f64,
    #[dsl(unit = "m")]
    pub spacing_m: f64,
    /// simply_supported | continuous_2_span
    pub support: String,
    pub construction: String,
    pub steel: SteelSection,
    #[dsl(unit = "m")]
    pub slab_thickness_m: f64,
    #[dsl(unit = "Pa")]
    pub concrete_f_ck_pa: f64,
    #[dsl(unit = "Pa")]
    pub concrete_e_cm_pa: f64,
    pub sheeting: ProfiledSheeting,
    pub studs: HeadedStuds,
    pub transverse_as_m2_per_m: f64,
    #[dsl(unit = "m")]
    pub ltb_length_m: f64,
    pub as_hogging_m2_per_m: f64,
    #[dsl(unit = "m")]
    pub bar_spacing_m: f64,
    #[dsl(unit = "m")]
    pub wk_limit_m: f64,
    pub n_cycles: f64,
    #[dsl(table)]
    pub actions: Vec<CharacteristicAction>,
}

/// 🧱 Composite column subject under EN 1994-1-1 §6.7 (SI).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeColumn {
    pub id: String,
    /// encased | concrete_filled | partially_encased
    pub kind: String,
    #[dsl(unit = "m")]
    pub length_m: f64,
    /// Outer diameter (CHS) or side length (SHS) for local buckling / confinement [m].
    #[dsl(unit = "m")]
    pub outer_size_m: f64,
    #[dsl(unit = "m")]
    pub wall_thickness_m: f64,
    #[dsl(unit = "m2")]
    pub steel_a_m2: f64,
    #[dsl(unit = "Pa")]
    pub steel_f_y_pa: f64,
    #[dsl(unit = "m2")]
    pub concrete_a_m2: f64,
    #[dsl(unit = "Pa")]
    pub concrete_f_ck_pa: f64,
    #[dsl(unit = "m2")]
    pub reinforcement_as_m2: f64,
    #[dsl(unit = "Pa")]
    pub reinforcement_f_yk_pa: f64,
    #[dsl(unit = "m4")]
    pub i_m4: f64,
    pub buckling_curve: String,
    #[dsl(table)]
    pub actions: Vec<ColumnAction>,
}

/// 🧱 Composite slab with profiled sheeting under EN 1994-1-1 §9 (SI).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositeSlab {
    pub id: String,
    #[dsl(unit = "m")]
    pub span_m: f64,
    /// simply_supported | continuous_2_span
    pub support: String,
    pub sheeting: ProfiledSheeting,
    #[dsl(unit = "m")]
    pub concrete_thickness_m: f64,
    #[dsl(unit = "Pa")]
    pub f_ck_pa: f64,
    pub m_factor: f64,
    pub k_factor: f64,
    pub as_m2_per_m: f64,
    #[dsl(table)]
    pub actions: Vec<CharacteristicAction>,
}

impl SteelSection {
    /// 🧱 HEB 300 placeholder for insert mutations.
    pub fn heb300() -> Self {
        Self { designation: "HEB300".into(), height_m: 0.300, width_m: 0.300, tw_m: 0.011, tf_m: 0.019, a_m2: 0.0149, w_pl_y_m3: 1.867e-3, i_y_m4: 2.517e-4, a_v_m2: 0.0045 }
    }

    /// 📚 Normalizes rolled-section designations (`HEB 320` → `HEB320`).
    pub fn normalize_designation(raw: &str) -> String {
        raw.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_uppercase()
    }

    /// 📚 Catalogue geometry for common HEB sections (EN 10365 / ArcelorMittal tables, SI).
    pub fn from_catalogue(designation: &str) -> Option<Self> {
        match Self::normalize_designation(designation).as_str() {
            "HEB300" => Some(Self::heb300()),
            "HEB320" => Some(Self {
                designation: "HEB320".into(), height_m: 0.320, width_m: 0.300, tw_m: 0.0115, tf_m: 0.0205,
                a_m2: 0.0161, w_pl_y_m3: 2.129e-3, i_y_m4: 3.082e-4, a_v_m2: 0.0049,
            }),
            "HEB340" => Some(Self {
                designation: "HEB340".into(), height_m: 0.340, width_m: 0.300, tw_m: 0.0120, tf_m: 0.0215,
                a_m2: 0.0171, w_pl_y_m3: 2.402e-3, i_y_m4: 3.666e-4, a_v_m2: 0.0052,
            }),
            "HEB360" => Some(Self {
                designation: "HEB360".into(), height_m: 0.360, width_m: 0.300, tw_m: 0.0125, tf_m: 0.0225,
                a_m2: 0.0180, w_pl_y_m3: 2.682e-3, i_y_m4: 4.319e-4, a_v_m2: 0.0055,
            }),
            "HEB400" => Some(Self {
                designation: "HEB400".into(), height_m: 0.400, width_m: 0.300, tw_m: 0.0135, tf_m: 0.0240,
                a_m2: 0.0198, w_pl_y_m3: 3.231e-3, i_y_m4: 5.768e-4, a_v_m2: 0.0061,
            }),
            _ => None,
        }
    }

    /// 📚 Resolves catalogue properties when the designation is known; otherwise keeps stored geometry.
    pub fn resolve(&self) -> Self {
        Self::from_catalogue(&self.designation).unwrap_or_else(|| self.clone())
    }

    /// 🪛 Non-catalogue welded plate girder (geometry leaves drive checks).
    pub fn custom_plate() -> Self {
        Self {
            designation: "CUSTOM-PLATE".into(),
            height_m: 0.450, width_m: 0.220, tw_m: 0.010, tf_m: 0.018,
            a_m2: 0.0124, w_pl_y_m3: 1.95e-3, i_y_m4: 3.8e-4, a_v_m2: 0.0040,
        }
    }

    /// 📚 Heavier HEB options for OneOf remedies (wire values = designation leaf).
    pub fn heavier_heb_options(current: &str) -> Vec<String> {
        let order = ["HEB300", "HEB320", "HEB340", "HEB360", "HEB400"];
        let cur = Self::normalize_designation(current);
        let idx = order.iter().position(|d| *d == cur).unwrap_or(0);
        order[idx.saturating_add(1)..].iter().map(|s| (*s).to_string()).collect()
    }

    /// 📐️ Web clear height h_w [m].
    pub fn h_w_m(&self) -> f64 {
        (self.height_m - 2.0 * self.tf_m).max(0.0)
    }

    /// 📐️ Shear area from plate geometry (EN 1993-1-1 §6.2.6 for rolled I).
    pub fn shear_area_from_plates_m2(&self) -> f64 {
        (self.height_m * self.tw_m).max(self.a_v_m2)
    }
}
impl ProfiledSheeting {
    pub fn trapezoidal() -> Self {
        Self { profile: "trapezoidal".into(), height_m: 0.055, rib_width_m: 0.035, thickness_m: 0.0009, ribs_parallel_to_beam: false }
    }

    /// 📐️ Steel area of sheeting per metre width A_p [m²/m] from thickness and developed profile factor ~1.2.
    pub fn a_p_m2_per_m(&self) -> f64 {
        self.thickness_m * 1.2
    }
}
impl HeadedStuds {
    pub fn typical() -> Self {
        Self { diameter_m: 0.019, height_m: 0.075, f_u_pa: 450e6, count_per_rib: 1, spacing_m: 0.150, total_count: 72 }
    }
}

impl CharacteristicAction {
    /// 📐️ Permanent area load (line = q_area × tributary width in `action_internals`).
    pub fn permanent_area(id: &str, stage: &str, category: &str, q_area_pa: f64) -> Self {
        Self {
            id: id.into(), kind: "permanent".into(), category: category.into(), stage: stage.into(),
            q_area_pa, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        }
    }
    pub fn imposed_area(id: &str, stage: &str, category: &str, q_area_pa: f64) -> Self {
        Self {
            id: id.into(), kind: "imposed".into(), category: category.into(), stage: stage.into(),
            q_area_pa, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        }
    }
    /// 📍 Sole characteristic concentrated force when area load is absent.
    pub fn point_force(id: &str, stage: &str, kind: &str, category: &str, f_k_n: f64) -> Self {
        Self {
            id: id.into(), kind: kind.into(), category: category.into(), stage: stage.into(),
            q_area_pa: 0.0, f_k_n, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        }
    }
    pub fn fatigue_flm3(id: &str) -> Self {
        Self {
            id: id.into(), kind: "fatigue".into(), category: "flm3".into(), stage: "composite".into(),
            q_area_pa: 0.0, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        }
    }
}

impl ColumnAction {
    /// 🏛️ Column characteristic N_k / M_k from structural analysis.
    pub fn forces(id: &str, stage: &str, kind: &str, category: &str, n_k_n: f64, m_k_nm: f64) -> Self {
        Self {
            id: id.into(), kind: kind.into(), category: category.into(), stage: stage.into(),
            n_k_n, m_k_nm,
        }
    }
}

fn default_beam_actions(steel_a_m2: f64, spacing_m: f64, slab_thickness_m: f64, sheeting_h: f64) -> Vec<CharacteristicAction> {
    let g_steel_line = steel_a_m2 * 7850.0 * 9.81;
    let h_conc = (slab_thickness_m - sheeting_h).max(0.04);
    let g_steel_area = g_steel_line / spacing_m.max(1e-6);
    let g_wet_area = 25e3 * h_conc;
    vec![
        CharacteristicAction::permanent_area("G-steel", "construction", "self_steel", g_steel_area),
        CharacteristicAction::permanent_area("G-wet", "construction", "wet_concrete", g_wet_area),
        CharacteristicAction {
            id: "Q-constr".into(), kind: "construction".into(), category: "construction_load".into(), stage: "construction".into(),
            q_area_pa: 1.0e3, f_k_n: 0.0, delta_sigma_k_pa: 0.0, delta_tau_k_pa: 0.0,
        },
        CharacteristicAction::permanent_area("G-steel-comp", "composite", "self_steel", g_steel_area),
        CharacteristicAction::permanent_area("G-slab", "composite", "slab", g_wet_area * 0.9),
        CharacteristicAction::permanent_area("G-finishes", "composite", "finishes", 1.0e3),
        CharacteristicAction::imposed_area("Q-office", "composite", "B", 2.0e3),
    ]
}

impl CompositeBeam {
    pub fn default_placeholder() -> Self {
        let steel = SteelSection::heb300();
        let sheeting = ProfiledSheeting::trapezoidal();
        let spacing = 3.0;
        let slab_t = 0.140;
        let actions = default_beam_actions(steel.a_m2, spacing, slab_t, sheeting.height_m);
        Self {
            id: "beam-new".into(), span_m: 8.0, spacing_m: spacing, support: "simply_supported".into(),
            construction: "propped".into(),
            steel, slab_thickness_m: slab_t, concrete_f_ck_pa: 30e6, concrete_e_cm_pa: 33e9,
            sheeting, studs: HeadedStuds::typical(),
            transverse_as_m2_per_m: 2.5e-3, ltb_length_m: 4.0,
            as_hogging_m2_per_m: 5.0e-4, bar_spacing_m: 0.150, wk_limit_m: 0.0003,
            n_cycles: 2.0e6, actions,
        }
    }
}
impl CompositeColumn {
    pub fn default_placeholder() -> Self {
        Self {
            id: "col-new".into(), kind: "concrete_filled".into(), length_m: 3.5,
            outer_size_m: 0.300, wall_thickness_m: 0.010,
            steel_a_m2: 0.0084, steel_f_y_pa: 355e6, concrete_a_m2: 0.072, concrete_f_ck_pa: 30e6,
            reinforcement_as_m2: 0.0012, reinforcement_f_yk_pa: 500e6, i_m4: 2.1e-4, buckling_curve: "a".into(),
            actions: vec![
                ColumnAction::forces("G-col", "composite", "permanent", "self", 1200e3, 20e3),
                ColumnAction::forces("Q-col", "composite", "imposed", "B", 400e3, 15e3),
            ],
        }
    }
}
impl CompositeSlab {
    pub fn default_placeholder() -> Self {
        Self {
            id: "slab-new".into(), span_m: 3.0, support: "simply_supported".into(),
            sheeting: ProfiledSheeting::trapezoidal(),
            concrete_thickness_m: 0.140, f_ck_pa: 30e6, m_factor: 180.0, k_factor: 0.05, as_m2_per_m: 2.5e-4,
            actions: vec![
                CharacteristicAction::permanent_area("G-slab", "composite", "self", 3.5e3),
                CharacteristicAction::imposed_area("Q-slab", "composite", "B", 2.0e3),
            ],
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1994", "EN 1994")
}
//#endregion 🔖️ArtifactKind

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1994_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1994", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const EN1994_DOCUMENT_SCHEMA: &str = "semio.norm.en1994/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1994" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1994.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.norm.en1994@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.en1994/v1" }, ClaimSpec { namespace: "codec-extension", value: "20:semio.norm.en1994/v1:en1994" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "EN 1994 design of composite steel and concrete structures" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "EN 1994 Bemessung und Konstruktion von Verbundtragwerken aus Stahl und Beton" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.norm.en1994.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.schema.artifact", kind: "schema", descriptor: "s.norm.en1994", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.inference.outline", kind: "inference", descriptor: "s.norm.en1994.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.composer.any", kind: "composer", descriptor: "s.norm.en1994@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.grammar.document", kind: "grammar", descriptor: "en1994.document", claims: &[ClaimSpec { namespace: "grammar", value: "en1994.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.grammar.op", kind: "grammar", descriptor: "en1994.op", claims: &[ClaimSpec { namespace: "grammar", value: "en1994.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.grammar.diff", kind: "grammar", descriptor: "en1994.diff", claims: &[ClaimSpec { namespace: "grammar", value: "en1994.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.grammar.pack", kind: "grammar", descriptor: "en1994.pack", claims: &[ClaimSpec { namespace: "grammar", value: "en1994.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.grammar.spr", kind: "grammar", descriptor: "en1994.spr", claims: &[ClaimSpec { namespace: "grammar", value: "en1994.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1994/v1:en1994", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1994.localization.en", kind: "localization", descriptor: "EN 1994 design of composite steel and concrete structures", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.norm.en1994.localization.de", kind: "localization", descriptor: "EN 1994 Bemessung und Konstruktion von Verbundtragwerken aus Stahl und Beton", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.en1994", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(artifact_schema::en1994_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::en1994_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<editor::en1994::En1994PlayApp>>()
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
                    id: "en1994.document",
                    extension: Some("en1994"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1994.document"),
                },
                dsl::LanguageSpec {
                    id: "en1994.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1994.op"),
                },
                dsl::LanguageSpec {
                    id: "en1994.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1994.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1994.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1994.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1994.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1994.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

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
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod change_annex {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-annex/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-annex/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-annex/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_structure_kind {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-structure-kind/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-structure-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-structure-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_steel_fy_pa {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-steel-fy-pa/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-steel-fy-pa/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏋️change-steel-fy-pa/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_fire_rating {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-fire-rating/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-fire-rating/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️change-fire-rating/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_insulation_thickness_m {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️change-insulation-thickness-m/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️change-insulation-thickness-m/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️change-insulation-thickness-m/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_fatigue_detail {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-fatigue-detail/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-fatigue-detail/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-fatigue-detail/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_beam {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-beam/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-beam/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-beam/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_beam {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-beam/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-beam/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-beam/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_action_q_area_pa {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-beam-action-q-area-pa/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-beam-action-q-area-pa/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️change-beam-action-q-area-pa/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_stud_spacing_m {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-beam-stud-spacing-m/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-beam-stud-spacing-m/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-beam-stud-spacing-m/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_span_m {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-beam-span-m/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-beam-span-m/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-beam-span-m/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_slab_thickness_m {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-beam-slab-thickness-m/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-beam-slab-thickness-m/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-beam-slab-thickness-m/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_stud_diameter_m {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭕️change-beam-stud-diameter-m/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭕️change-beam-stud-diameter-m/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭕️change-beam-stud-diameter-m/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_stud_count {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/#️⃣change-beam-stud-count/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/#️⃣change-beam-stud-count/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/#️⃣change-beam-stud-count/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_stud_fu_pa {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️change-beam-stud-fu-pa/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️change-beam-stud-fu-pa/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️change-beam-stud-fu-pa/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_transverse_as {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-beam-transverse-as/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-beam-transverse-as/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-beam-transverse-as/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_beam_construction {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-beam-construction/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-beam-construction/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-beam-construction/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_column {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➗️insert-column/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➗️insert-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➗️insert-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_column {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛔️remove-column/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛔️remove-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛔️remove-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_column_action_force_n {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️change-column-action-force-n/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️change-column-action-force-n/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬇️change-column-action-force-n/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_column_kind {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️change-column-kind/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️change-column-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️change-column-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_slab {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕insert-slab/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕insert-slab/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕insert-slab/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_slab {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-slab/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-slab/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-slab/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_slab_action_q_area_pa {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-slab-action-q-area-pa/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-slab-action-q-area-pa/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-slab-action-q-area-pa/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_slab_thickness_m {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-slab-thickness-m/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-slab-thickness-m/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-slab-thickness-m/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        pub use component::*;
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
pub use crate::standards::v1::subsets::any::schema::diff::En1994Diff;
pub use crate::standards::v1::subsets::any::schema::mutations::En1994Mutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::{decode_en1994_dsl, encode_en1994_snapshot_json, En1994Snapshot};

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod en1994 {
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
    pub mod en1994 {
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
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️composite-bridge-girder/🦀️.rs"]
pub mod composite_bridge_girder;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏷️field-meta/🦀️.rs"]
pub mod field_meta;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢composite-floor-beam/🦀️.rs"]
pub mod composite_floor_beam;
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢composite-floor-beam-failing/🦀️.rs"]
pub mod composite_floor_beam_failing;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️composite-bridge-girder/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
