//! ⚡️ DIN V 18599 app — document entities (constitutional: general).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

/// 📜 Language-neutral package declaration owned by this artifact.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📦 Validates this artifact's independently compiled package identity.
pub fn package_descriptor() -> Result<semio_s_artifact_norm_contract::NormArtifactPackage, semio_s_artifact_norm_contract::PackageSchemaError> {
    semio_s_artifact_norm_contract::package_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

// #region 🔖️Types
pub use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

/// 📋️ Annual energy balancing inputs stored in the persisted snapshot.
pub type BalancingInputs = Din18599Snapshot;
//#endregion 🔖️Types

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din18599" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din18599.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.norm.din18599@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.din18599/v1" }, ClaimSpec { namespace: "codec-extension", value: "22:semio.norm.din18599/v1:din18599" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "DIN V 18599 energy performance of buildings" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "DIN V 18599 Energetische Bewertung von Gebäuden" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.norm.din18599.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.schema.artifact", kind: "schema", descriptor: "s.norm.din18599", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.inference.outline", kind: "inference", descriptor: "s.norm.din18599.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.composer.any", kind: "composer", descriptor: "s.norm.din18599@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.document", kind: "grammar", descriptor: "din18599.document", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.op", kind: "grammar", descriptor: "din18599.op", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.diff", kind: "grammar", descriptor: "din18599.diff", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.pack", kind: "grammar", descriptor: "din18599.pack", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.spr", kind: "grammar", descriptor: "din18599.spr", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.codec.document.v1", kind: "codec", descriptor: "semio.norm.din18599/v1:din18599", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.localization.en", kind: "localization", descriptor: "DIN V 18599 energy performance of buildings", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.norm.din18599.localization.de", kind: "localization", descriptor: "DIN V 18599 Energetische Bewertung von Gebäuden", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.din18599", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_norm_din18599_schema::din18599_artifact_schema_descriptor())
        .inferences([semio_s_artifact_norm_din18599_inferences::din18599_artifact_inference_descriptor()])
        .composers(semio_s_artifact_norm_din18599_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_din18599_editor::Din18599PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "din18599.document",
                    extension: Some("din18599"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_norm_din18599_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_din18599_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_din18599_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_din18599_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("din18599.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "din18599.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_norm_din18599_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_din18599_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_din18599_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_din18599_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("din18599.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "din18599.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_norm_din18599_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_din18599_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("din18599.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "din18599.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_din18599_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_din18599_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("din18599.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "din18599.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_din18599_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_din18599_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("din18599.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

// ---- Crate-root module paths: the crate's public surface ----

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
mod diff_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🪢️Mounts
