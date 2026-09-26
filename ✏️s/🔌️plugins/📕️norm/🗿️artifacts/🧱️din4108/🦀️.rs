//! 🌡️ DIN 4108 app — document entities (constitutional: general).

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

// #region 🔖️Types
/// 🧩 Parallel fraction of an inhomogeneous ISO 6946 §6.7 layer (area share of one material).
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayerSegment {
    pub id: String,
    pub material_id: String,
    pub fraction: f64,
    pub lambda: f64,
    pub mu: f64,
    pub density: f64,
}

/// 🧱 One layer in an envelope build-up (interior → exterior), SI base units.
/// Empty `segments` = homogeneous; non-empty = ISO 6946 §6.7 upper/lower-bound method.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayerDocument {
    pub id: String,
    pub material_id: String,
    #[dsl(unit = "m")]
    pub thickness_m: f64,
    pub lambda: f64,
    pub mu: f64,
    pub density: f64,
    /// DIN 4108-10 Table 1 application-type code (DAD, WAB, …).
    #[cfg_attr(test, serde(default))]
    pub application_type: String,
    /// DIN 4108-10 compressive / mechanical property class (dh/ds/dm/dk/dx).
    #[cfg_attr(test, serde(default))]
    pub compressive_class: String,
    /// DIN 4108-10 water-absorption class (wk/wf/wd).
    #[cfg_attr(test, serde(default))]
    pub water_class: String,
    /// DIN 4108-10 tensile class (tk/tf).
    #[cfg_attr(test, serde(default))]
    pub tensile_class: String,
    /// DIN 4108-10 acoustic class (sh/sm/sg).
    #[cfg_attr(test, serde(default))]
    pub acoustic_class: String,
    #[dsl(table)]
    #[cfg_attr(test, serde(default))]
    pub segments: Vec<LayerSegment>,
}

/// 🪟 Transparent opening contribution for DIN 4108-2 §8 summer heat protection.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ZoneWindow {
    pub id: String,
    pub orientation: String,
    #[dsl(unit = "deg")]
    pub inclination_deg: f64,
    #[dsl(unit = "m2")]
    pub area_m2: f64,
    pub g_value: f64,
    pub shading_fc: f64,
}

/// 🚪 Thermal zone / room used for summer heat protection aggregation.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ThermalZone {
    pub id: String,
    #[dsl(unit = "m2")]
    pub floor_area_m2: f64,
    pub heaviness: String,
    pub night_ventilation: String,
    #[dsl(table)]
    pub windows: Vec<ZoneWindow>,
}

/// 🏛️ Opaque or transparent envelope element with ISO 6946 layer stack (incl. ΔU_g/ΔU_f/ΔU_r corrections).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct EnvelopeElement {
    pub id: String,
    pub kind: String,
    pub zone_id: String,
    pub orientation_deg: f64,
    pub inclination_deg: f64,
    pub adjacent: String,
    #[dsl(unit = "m2")]
    pub area_m2: f64,
    pub delta_u_g: f64,
    pub delta_u_f: f64,
    pub delta_u_r: f64,
    #[dsl(table)]
    pub layers: Vec<LayerDocument>,
}

/// 🌉️ Linear thermal bridge (DIN 4108 Beiblatt 2).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ThermalBridge {
    pub id: String,
    pub psi: f64,
    #[dsl(unit = "m")]
    pub length_m: f64,
    pub bb2_type: String,
}

//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("din4108", "DIN 4108")
}
//#endregion 🔖️ArtifactKind

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const DIN4108_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.din4108", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const DIN4108_DOCUMENT_SCHEMA: &str = "semio.norm.din4108/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din4108" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din4108.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.norm.din4108@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.din4108/v1" }, ClaimSpec { namespace: "codec-extension", value: "21:semio.norm.din4108/v1:din4108" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "DIN 4108 thermal insulation and energy economy in buildings" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "DIN 4108 Wärme- und Feuchteschutz im Hochbau" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.norm.din4108.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.schema.artifact", kind: "schema", descriptor: "s.norm.din4108", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.inference.outline", kind: "inference", descriptor: "s.norm.din4108.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.composer.any", kind: "composer", descriptor: "s.norm.din4108@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.grammar.document", kind: "grammar", descriptor: "din4108.document", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.grammar.op", kind: "grammar", descriptor: "din4108.op", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.grammar.diff", kind: "grammar", descriptor: "din4108.diff", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.grammar.pack", kind: "grammar", descriptor: "din4108.pack", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.grammar.spr", kind: "grammar", descriptor: "din4108.spr", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.codec.document.v1", kind: "codec", descriptor: "semio.norm.din4108/v1:din4108", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din4108.localization.en", kind: "localization", descriptor: "DIN 4108 thermal insulation and energy economy in buildings", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.norm.din4108.localization.de", kind: "localization", descriptor: "DIN 4108 Wärme- und Feuchteschutz im Hochbau", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.din4108", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(artifact_schema::din4108_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::din4108_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<editor::din4108::Din4108PlayApp>>()
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
                    id: "din4108.document",
                    extension: Some("din4108"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("din4108.document"),
                },
                dsl::LanguageSpec {
                    id: "din4108.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("din4108.op"),
                },
                dsl::LanguageSpec {
                    id: "din4108.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("din4108.diff"),
                },
                dsl::LanguageSpec {
                    id: "din4108.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("din4108.pack"),
                },
                dsl::LanguageSpec {
                    id: "din4108.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("din4108.spr"),
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
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;

                        #[path = "."]
                        pub mod change_element_adjacent {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-element-adjacent/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-element-adjacent/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-element-adjacent/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_thermal_bridge_length {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-thermal-bridge-length/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-thermal-bridge-length/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-thermal-bridge-length/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_zone_window_g_value {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-zone-window-g-value/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-zone-window-g-value/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️change-zone-window-g-value/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_zone_window_shading_fc {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️change-zone-window-shading-fc/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️change-zone-window-shading-fc/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️change-zone-window-shading-fc/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_bb2_details_conform {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️change-bb2-details-conform/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️change-bb2-details-conform/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️change-bb2-details-conform/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod insert_layer {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-layer/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-layer/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-layer/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod insert_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_layer {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-layer/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-layer/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-layer/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod insert_thermal_bridge {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️insert-thermal-bridge/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️insert-thermal-bridge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️insert-thermal-bridge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_zone_night_ventilation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌙change-zone-night-ventilation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌙change-zone-night-ventilation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌙change-zone-night-ventilation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_layer_lambda {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡change-layer-lambda/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡change-layer-lambda/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡change-layer-lambda/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_t_int_c {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-t-int-c/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-t-int-c/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-t-int-c/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_climate_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️change-climate-zone/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️change-climate-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️change-climate-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_has_mechanical_ventilation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-has-mechanical-ventilation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-has-mechanical-ventilation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-has-mechanical-ventilation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod insert_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️insert-element/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️insert-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️insert-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_element_kind {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-element-kind/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-element-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-element-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_layer_mu {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-layer-mu/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-layer-mu/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-layer-mu/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_rh_int {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️change-rh-int/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️change-rh-int/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️change-rh-int/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_airtightness_n50 {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️change-airtightness-n50/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️change-airtightness-n50/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️change-airtightness-n50/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_zone_window_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-zone-window-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-zone-window-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-zone-window-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_layer_thickness {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-layer-thickness/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-layer-thickness/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-layer-thickness/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_element_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-element-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-element-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-element-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_zone_floor_area {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-zone-floor-area/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod reorder_layers {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-layers/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-layers/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-layers/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_thermal_bridge_psi {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘change-thermal-bridge-psi/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘change-thermal-bridge-psi/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘change-thermal-bridge-psi/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        
                        #[path = "."]
                        pub mod change_element_orientation_deg {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-element-orientation-deg/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-element-orientation-deg/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-element-orientation-deg/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_element_inclination_deg {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-element-inclination-deg/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-element-inclination-deg/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-element-inclination-deg/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_element_delta_u_g {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-ug/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-ug/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-ug/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_element_delta_u_f {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-uf/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-uf/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-uf/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_element_delta_u_r {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-ur/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-ur/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/Δchange-element-delta-ur/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_thermal_bridge_bb2_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷change-thermal-bridge-bb2-type/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷change-thermal-bridge-bb2-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷change-thermal-bridge-bb2-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_zone_window_orientation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-zone-window-orientation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-zone-window-orientation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-zone-window-orientation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_zone_window_inclination_deg {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-zone-window-inclination-deg/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-zone-window-inclination-deg/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-zone-window-inclination-deg/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_layer_application_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-layer-application-type/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-layer-application-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-layer-application-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }

                        #[path = "."]
                        pub mod change_layer_compressive_class {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-layer-compressive-class/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-layer-compressive-class/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-layer-compressive-class/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_usage {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️change-usage/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️change-usage/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️change-usage/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️remove-element/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️remove-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️remove-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_zone_window {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️remove-zone-window/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️remove-zone-window/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️remove-zone-window/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_thermal_bridge {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊remove-thermal-bridge/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊remove-thermal-bridge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊remove-thermal-bridge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_zone_heaviness {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-zone-heaviness/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-zone-heaviness/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-zone-heaviness/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_layer_material_id {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️change-layer-material-id/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️change-layer-material-id/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️change-layer-material-id/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod insert_zone_window {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟insert-zone-window/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟insert-zone-window/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟insert-zone-window/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
pub use crate::standards::v1::subsets::any::schema::diff::Din4108Diff;
pub use crate::standards::v1::subsets::any::schema::mutations::Din4108Mutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Din4108Snapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod failing_thin_insulation {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️failing-thin-insulation/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏷️field-meta/🦀️.rs"]
pub mod field_meta;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod din4108 {
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
    pub mod din4108 {
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
