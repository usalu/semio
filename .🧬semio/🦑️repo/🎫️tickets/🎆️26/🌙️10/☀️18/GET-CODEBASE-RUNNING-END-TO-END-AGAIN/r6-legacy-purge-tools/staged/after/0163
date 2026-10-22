//! 🪵️ EN 1995 artifact root — snapshot re-export and facet modules.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

// #region 🔖️Types

//#endregion 🔖️Types

/// 📜 Language-neutral package declaration owned by this artifact.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📦 Validates this artifact's independently compiled package identity.
pub fn package_descriptor() -> Result<semio_s_artifact_norm_contract::NormArtifactPackage, semio_s_artifact_norm_contract::PackageSchemaError> {
    semio_s_artifact_norm_contract::package_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1995" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1995.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.norm.en1995@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.en1995/v1" }, ClaimSpec { namespace: "codec-extension", value: "20:semio.norm.en1995/v1:en1995" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "EN 1995 design of timber structures" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "EN 1995 Bemessung und Konstruktion von Holzbauten" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.norm.en1995.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.schema.artifact", kind: "schema", descriptor: "s.norm.en1995", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.inference.outline", kind: "inference", descriptor: "s.norm.en1995.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.composer.any", kind: "composer", descriptor: "s.norm.en1995@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.grammar.document", kind: "grammar", descriptor: "en1995.document", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.grammar.op", kind: "grammar", descriptor: "en1995.op", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.grammar.diff", kind: "grammar", descriptor: "en1995.diff", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.grammar.pack", kind: "grammar", descriptor: "en1995.pack", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.grammar.spr", kind: "grammar", descriptor: "en1995.spr", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1995/v1:en1995", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1995.localization.en", kind: "localization", descriptor: "EN 1995 design of timber structures", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.norm.en1995.localization.de", kind: "localization", descriptor: "EN 1995 Bemessung und Konstruktion von Holzbauten", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.en1995", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_norm_en1995_schema::en1995_artifact_schema_descriptor())
        .inferences([semio_s_artifact_norm_en1995_inferences::en1995_artifact_inference_descriptor()])
        .composers(semio_s_artifact_norm_en1995_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1995_editor::En1995PlayApp>>()
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
                    id: "en1995.document",
                    extension: Some("en1995"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_norm_en1995_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1995_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_en1995_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1995_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1995.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1995.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_norm_en1995_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1995_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_en1995_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1995_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1995.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1995.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_norm_en1995_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1995_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("en1995.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1995.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_en1995_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1995_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1995.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1995.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_en1995_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1995_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1995.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

// ---- Crate-root module paths: the crate's public surface ----

//#region 🪢️TaxonomyMounts

//#endregion 🔢️TaxonomyMounts

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
mod diff_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod mutations_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
//#endregion 🪢️Mounts
