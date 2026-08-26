//! 🌋️ EN 1998 artifact root — snapshot re-export and facet modules.

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1998", "EN 1998")
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1998_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1998", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const EN1998_DOCUMENT_SCHEMA: &str = "semio.norm.en1998/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.en1998" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.en1998.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.en1998@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.en1998/v1" }, Q { namespace: "extension", value: "en1998" }];
    const EN: &[L] = &[L { locale: "en", text: "EN 1998 design of structures for earthquake resistance" }];
    const DE: &[L] = &[L { locale: "de", text: "EN 1998 Auslegung von Bauwerken gegen Erdbeben" }];
    const ROWS: &[C] = &[
        C { identity: "s.en1998.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.en1998.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.en1998.schema.artifact", kind: "schema", descriptor: "s.norm.en1998", claims: S, localizations: &[] },
        C { identity: "s.en1998.inference.outline", kind: "inference", descriptor: "s.norm.en1998.inference", claims: I, localizations: &[] },
        C { identity: "s.en1998.composer.any", kind: "composer", descriptor: "s.en1998@1/*", claims: M, localizations: &[] },
        C { identity: "s.en1998.grammar.document", kind: "grammar", descriptor: "en1998.document", claims: &[Q { namespace: "grammar", value: "en1998.document" }], localizations: &[] },
        C { identity: "s.en1998.grammar.op", kind: "grammar", descriptor: "en1998.op", claims: &[Q { namespace: "grammar", value: "en1998.op" }], localizations: &[] },
        C { identity: "s.en1998.grammar.diff", kind: "grammar", descriptor: "en1998.diff", claims: &[Q { namespace: "grammar", value: "en1998.diff" }], localizations: &[] },
        C { identity: "s.en1998.grammar.pack", kind: "grammar", descriptor: "en1998.pack", claims: &[Q { namespace: "grammar", value: "en1998.pack" }], localizations: &[] },
        C { identity: "s.en1998.grammar.spr", kind: "grammar", descriptor: "en1998.spr", claims: &[Q { namespace: "grammar", value: "en1998.spr" }], localizations: &[] },
        C { identity: "s.en1998.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1998/v1:en1998", claims: K, localizations: &[] },
        C { identity: "s.en1998.localization.en", kind: "localization", descriptor: "EN 1998 design of structures for earthquake resistance", claims: &[], localizations: EN },
        C { identity: "s.en1998.localization.de", kind: "localization", descriptor: "EN 1998 Auslegung von Bauwerken gegen Erdbeben", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.en1998", ROWS)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::en1998::schema::en1998_artifact_schema_descriptor())
        .inferences([crate::artifacts::en1998::standards::v1::subsets::any::schema::inferences::en1998_artifact_inference_descriptor()])
        .composers(crate::artifacts::en1998::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::en1998::En1998PlayApp>>()
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
                    id: "en1998.document",
                    extension: Some("en1998"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::en1997::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1997::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1997::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1997::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1998.document"),
                },
                dsl::LanguageSpec {
                    id: "en1998.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::en1997::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1997::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1997::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1997::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1998.op"),
                },
                dsl::LanguageSpec {
                    id: "en1998.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::en1997::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1997::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1998.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1998.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1997::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1997::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1998.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1998.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1997::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1997::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1998.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
