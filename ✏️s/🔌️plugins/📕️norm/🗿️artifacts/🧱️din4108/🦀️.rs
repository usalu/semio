//! 🌡️ DIN 4108 app — document entities (constitutional: general).

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

// #region 🔖️Types

//#endregion 🔖️Types

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
        .schema(semio_s_artifact_norm_din4108_schema::din4108_artifact_schema_descriptor())
        .inferences([semio_s_artifact_norm_din4108_inferences::din4108_artifact_inference_descriptor()])
        .composers(semio_s_artifact_norm_din4108_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_din4108_editor::Din4108PlayApp>>()
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
                    id: "din4108.document",
                    extension: Some("din4108"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_norm_din4108_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_din4108_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_din4108_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_din4108_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("din4108.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "din4108.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_norm_din4108_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_din4108_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_din4108_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_din4108_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("din4108.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "din4108.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_norm_din4108_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_din4108_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("din4108.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "din4108.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_din4108_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_din4108_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("din4108.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "din4108.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_din4108_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_din4108_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("din4108.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

// ---- Shims: keep pre-migration module paths resolving for external callers ----

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs"]
mod checks_compliance;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
//#endregion 🪢️Mounts
