//! 🪵️ EN 1995 artifact root — snapshot re-export and facet modules.

pub use crate::artifacts::en1995::schema::diff::En1995Diff;
pub use crate::artifacts::en1995::schema::mutations::En1995Mutation;
pub use crate::artifacts::en1995::schema::snapshot::En1995Snapshot;

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1995", "EN 1995")
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1995_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect {
    artifact_kind: "s.norm.en1995",
    standard: semio_framework_plugin::app::StandardId("1"),
    subset: semio_framework_plugin::app::SubsetId::ANY,
};
pub const EN1995_DOCUMENT_SCHEMA: &str = "semio.norm.en1995/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1995" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1995.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.en1995@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.en1995/v1" }, ClaimSpec { namespace: "extension", value: "en1995" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "EN 1995 design of timber structures" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "EN 1995 Bemessung und Konstruktion von Holzbauten" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.en1995.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1995.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1995.schema.artifact", kind: "schema", descriptor: "s.norm.en1995", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.en1995.inference.outline", kind: "inference", descriptor: "s.norm.en1995.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.en1995.composer.any", kind: "composer", descriptor: "s.en1995@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.en1995.grammar.document", kind: "grammar", descriptor: "en1995.document", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1995.grammar.op", kind: "grammar", descriptor: "en1995.op", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1995.grammar.diff", kind: "grammar", descriptor: "en1995.diff", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1995.grammar.pack", kind: "grammar", descriptor: "en1995.pack", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1995.grammar.spr", kind: "grammar", descriptor: "en1995.spr", claims: &[ClaimSpec { namespace: "grammar", value: "en1995.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1995.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1995/v1:en1995", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.en1995.localization.en", kind: "localization", descriptor: "EN 1995 design of timber structures", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.en1995.localization.de", kind: "localization", descriptor: "EN 1995 Bemessung und Konstruktion von Holzbauten", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.en1995", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::en1995::schema::en1995_artifact_schema_descriptor())
        .inferences([crate::artifacts::en1995::standards::v1::subsets::any::schema::inferences::en1995_artifact_inference_descriptor()])
        .composers(crate::artifacts::en1995::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::en1995::En1995PlayApp>>()
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
                    id: "en1995.document",
                    extension: Some("en1995"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::en1994::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1994::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1994::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1994::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1995.document"),
                },
                dsl::LanguageSpec {
                    id: "en1995.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::en1994::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1994::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1994::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1994::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1995.op"),
                },
                dsl::LanguageSpec {
                    id: "en1995.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::en1994::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1994::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1995.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1995.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1994::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1994::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1995.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1995.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1994::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1994::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1995.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
