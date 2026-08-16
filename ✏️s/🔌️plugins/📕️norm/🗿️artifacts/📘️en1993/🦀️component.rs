//! En1993 — document entities (constitutional: general).

pub use crate::artifacts::en1993::schema::diff::En1993Diff;
pub use crate::artifacts::en1993::schema::mutations::En1993Mutation;
pub use crate::artifacts::en1993::schema::snapshot::En1993Snapshot;

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Types

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1993", "EN 1993")
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::en1993::standards::v1::subsets::any::io::io_registry as v1;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("En1993Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1993" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1993.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.en1993@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.en1993/v1" }, ClaimSpec { namespace: "extension", value: "en1993" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "EN 1993 design of steel structures" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "EN 1993 Bemessung und Konstruktion von Stahlbauten" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.en1993.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1993.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1993.schema.artifact", kind: "schema", descriptor: "s.norm.en1993", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.en1993.inference.outline", kind: "inference", descriptor: "s.norm.en1993.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.en1993.composer.any", kind: "composer", descriptor: "s.en1993@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.en1993.grammar.document", kind: "grammar", descriptor: "en1993.document", claims: &[ClaimSpec { namespace: "grammar", value: "en1993.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1993.grammar.op", kind: "grammar", descriptor: "en1993.op", claims: &[ClaimSpec { namespace: "grammar", value: "en1993.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1993.grammar.diff", kind: "grammar", descriptor: "en1993.diff", claims: &[ClaimSpec { namespace: "grammar", value: "en1993.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1993.grammar.pack", kind: "grammar", descriptor: "en1993.pack", claims: &[ClaimSpec { namespace: "grammar", value: "en1993.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1993.grammar.spr", kind: "grammar", descriptor: "en1993.spr", claims: &[ClaimSpec { namespace: "grammar", value: "en1993.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1993.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1993/v1:en1993", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.en1993.localization.en", kind: "localization", descriptor: "EN 1993 design of steel structures", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.en1993.localization.de", kind: "localization", descriptor: "EN 1993 Bemessung und Konstruktion von Stahlbauten", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.en1993", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::en1993::schema::en1993_artifact_schema_descriptor())
        .inferences([crate::artifacts::en1993::standards::v1::subsets::any::schema::inferences::en1993_artifact_inference_descriptor()])
        .composers(crate::artifacts::en1993::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::en1993::En1993PlayApp>()
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
                    id: "en1993.document",
                    extension: Some("en1993"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::en1992::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1992::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1992::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1992::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1993.document"),
                },
                dsl::LanguageSpec {
                    id: "en1993.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::en1992::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1992::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1992::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1992::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1993.op"),
                },
                dsl::LanguageSpec {
                    id: "en1993.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::en1992::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1992::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1993.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1993.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1992::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1992::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1993.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1993.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1992::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1992::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1993.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
