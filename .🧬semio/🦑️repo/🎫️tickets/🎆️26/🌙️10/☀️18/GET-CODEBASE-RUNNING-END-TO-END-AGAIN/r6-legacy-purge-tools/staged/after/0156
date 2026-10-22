//! 📏️ En1991 — document entities (constitutional: general).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

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
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1991" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1991.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.norm.en1991@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.en1991/v1" }, ClaimSpec { namespace: "codec-extension", value: "20:semio.norm.en1991/v1:en1991" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "EN 1991 actions on structures" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "EN 1991 Einwirkungen auf Tragwerke" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.norm.en1991.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.schema.artifact", kind: "schema", descriptor: "s.norm.en1991", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.inference.outline", kind: "inference", descriptor: "s.norm.en1991.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.composer.any", kind: "composer", descriptor: "s.norm.en1991@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.grammar.document", kind: "grammar", descriptor: "en1991.document", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.grammar.op", kind: "grammar", descriptor: "en1991.op", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.grammar.diff", kind: "grammar", descriptor: "en1991.diff", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.grammar.pack", kind: "grammar", descriptor: "en1991.pack", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.grammar.spr", kind: "grammar", descriptor: "en1991.spr", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1991/v1:en1991", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.norm.en1991.localization.en", kind: "localization", descriptor: "EN 1991 actions on structures", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.norm.en1991.localization.de", kind: "localization", descriptor: "EN 1991 Einwirkungen auf Tragwerke", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.en1991", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_norm_en1991_schema::en1991_artifact_schema_descriptor())
        .inferences([semio_s_artifact_norm_en1991_inferences::en1991_artifact_inference_descriptor()])
        .composers(semio_s_artifact_norm_en1991_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1991_editor::En1991PlayApp>>()
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
                    id: "en1991.document",
                    extension: Some("en1991"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_norm_en1991_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1991_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_en1991_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1991_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1991.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1991.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_norm_en1991_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1991_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_en1991_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1991_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1991.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1991.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_norm_en1991_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1991_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("en1991.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1991.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_en1991_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1991_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1991.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1991.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_en1991_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1991_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1991.spr"),
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
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs"]
mod inferences_compliance_report;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
//#endregion 🪢️Mounts
