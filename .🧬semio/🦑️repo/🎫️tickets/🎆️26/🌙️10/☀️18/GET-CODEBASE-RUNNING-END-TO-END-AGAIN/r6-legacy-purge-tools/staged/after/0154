//! 🌍️ EN 1997 artifact root — snapshot re-export and facet modules.

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

//#endregion 🔖️Types

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.en1997" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.en1997.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.norm.en1997@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.en1997/v1" }, Q { namespace: "codec-extension", value: "20:semio.norm.en1997/v1:en1997" }];
    const EN: &[L] = &[L { locale: "en", text: "EN 1997 geotechnical design" }];
    const DE: &[L] = &[L { locale: "de", text: "EN 1997 Entwurf, Berechnung und Bemessung in der Geotechnik" }];
    const ROWS: &[C] = &[
        C { identity: "s.norm.en1997.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.norm.en1997.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.norm.en1997.schema.artifact", kind: "schema", descriptor: "s.norm.en1997", claims: S, localizations: &[] },
        C { identity: "s.norm.en1997.inference.outline", kind: "inference", descriptor: "s.norm.en1997.inference", claims: I, localizations: &[] },
        C { identity: "s.norm.en1997.composer.any", kind: "composer", descriptor: "s.norm.en1997@1/*", claims: M, localizations: &[] },
        C { identity: "s.norm.en1997.grammar.document", kind: "grammar", descriptor: "en1997.document", claims: &[Q { namespace: "grammar", value: "en1997.document" }], localizations: &[] },
        C { identity: "s.norm.en1997.grammar.op", kind: "grammar", descriptor: "en1997.op", claims: &[Q { namespace: "grammar", value: "en1997.op" }], localizations: &[] },
        C { identity: "s.norm.en1997.grammar.diff", kind: "grammar", descriptor: "en1997.diff", claims: &[Q { namespace: "grammar", value: "en1997.diff" }], localizations: &[] },
        C { identity: "s.norm.en1997.grammar.pack", kind: "grammar", descriptor: "en1997.pack", claims: &[Q { namespace: "grammar", value: "en1997.pack" }], localizations: &[] },
        C { identity: "s.norm.en1997.grammar.spr", kind: "grammar", descriptor: "en1997.spr", claims: &[Q { namespace: "grammar", value: "en1997.spr" }], localizations: &[] },
        C { identity: "s.norm.en1997.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1997/v1:en1997", claims: K, localizations: &[] },
        C { identity: "s.norm.en1997.localization.en", kind: "localization", descriptor: "EN 1997 geotechnical design", claims: &[], localizations: EN },
        C { identity: "s.norm.en1997.localization.de", kind: "localization", descriptor: "EN 1997 Entwurf, Berechnung und Bemessung in der Geotechnik", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.en1997", ROWS)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_norm_en1997_schema::en1997_artifact_schema_descriptor())
        .inferences([semio_s_artifact_norm_en1997_inferences::en1997_artifact_inference_descriptor()])
        .composers(semio_s_artifact_norm_en1997_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1997_editor::En1997PlayApp>>()
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
                    id: "en1997.document",
                    extension: Some("en1997"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_norm_en1997_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1997_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_en1997_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1997_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1997.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1997.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_norm_en1997_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1997_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_en1997_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1997_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1997.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1997.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_norm_en1997_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_en1997_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("en1997.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1997.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_en1997_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1997_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1997.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "en1997.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_en1997_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_en1997_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("en1997.spr"),
                },
            ]
        })
        .as_slice()
}

//#endregion 🪪️Declaration

// ---- Crate-root module paths: the crate's public surface ----

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs"]
mod checks_compliance;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
mod diff_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔬️regen-assets/🦀️.rs"]
mod model_regen_assets;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
//#endregion 🪢️Mounts
