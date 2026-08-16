//! 🌍️ EN 1997 artifact root — snapshot re-export and facet modules.

pub use crate::artifacts::en1997::schema::diff::En1997Diff;
pub use crate::artifacts::en1997::schema::mutations::En1997Mutation;
pub use crate::artifacts::en1997::schema::snapshot::En1997Snapshot;

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1997", "EN 1997")
}
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::en1997::standards::v1::subsets::any::io::io_registry as v1;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("En1997Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
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
    use crate::artifacts::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.en1997" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.en1997.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.en1997@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.en1997/v1" }, Q { namespace: "extension", value: "en1997" }];
    const EN: &[L] = &[L { locale: "en", text: "EN 1997 geotechnical design" }];
    const DE: &[L] = &[L { locale: "de", text: "EN 1997 Entwurf, Berechnung und Bemessung in der Geotechnik" }];
    const ROWS: &[C] = &[
        C { identity: "s.en1997.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.en1997.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.en1997.schema.artifact", kind: "schema", descriptor: "s.norm.en1997", claims: S, localizations: &[] },
        C { identity: "s.en1997.inference.outline", kind: "inference", descriptor: "s.norm.en1997.inference", claims: I, localizations: &[] },
        C { identity: "s.en1997.composer.any", kind: "composer", descriptor: "s.en1997@1/*", claims: M, localizations: &[] },
        C { identity: "s.en1997.grammar.document", kind: "grammar", descriptor: "en1997.document", claims: &[Q { namespace: "grammar", value: "en1997.document" }], localizations: &[] },
        C { identity: "s.en1997.grammar.op", kind: "grammar", descriptor: "en1997.op", claims: &[Q { namespace: "grammar", value: "en1997.op" }], localizations: &[] },
        C { identity: "s.en1997.grammar.diff", kind: "grammar", descriptor: "en1997.diff", claims: &[Q { namespace: "grammar", value: "en1997.diff" }], localizations: &[] },
        C { identity: "s.en1997.grammar.pack", kind: "grammar", descriptor: "en1997.pack", claims: &[Q { namespace: "grammar", value: "en1997.pack" }], localizations: &[] },
        C { identity: "s.en1997.grammar.spr", kind: "grammar", descriptor: "en1997.spr", claims: &[Q { namespace: "grammar", value: "en1997.spr" }], localizations: &[] },
        C { identity: "s.en1997.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1997/v1:en1997", claims: K, localizations: &[] },
        C { identity: "s.en1997.localization.en", kind: "localization", descriptor: "EN 1997 geotechnical design", claims: &[], localizations: EN },
        C { identity: "s.en1997.localization.de", kind: "localization", descriptor: "EN 1997 Entwurf, Berechnung und Bemessung in der Geotechnik", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.en1997", ROWS)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::en1997::schema::en1997_artifact_schema_descriptor())
        .inferences([crate::artifacts::en1997::standards::v1::subsets::any::schema::inferences::en1997_artifact_inference_descriptor()])
        .composers(crate::artifacts::en1997::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::en1997::En1997PlayApp>()
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
                    id: "en1997.document",
                    extension: Some("en1997"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::en1996::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1996::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1996::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1996::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1997.document"),
                },
                dsl::LanguageSpec {
                    id: "en1997.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::en1996::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1996::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1996::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1996::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1997.op"),
                },
                dsl::LanguageSpec {
                    id: "en1997.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::en1996::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1996::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1997.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1997.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1996::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1996::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1997.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1997.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1996::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1996::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1997.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
