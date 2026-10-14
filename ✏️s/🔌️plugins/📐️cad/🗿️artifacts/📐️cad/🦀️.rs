//! 📐️ CAD artifact aggregator — the `s.cad.cad` definition, its declaration (schema, inferences, composers, languages and
//! the editor document codec) and the full-stack activation tests. Every domain facet lives in its own family crate
//! (`semio-s-artifact-cad-cad-*`): model, mutations, inferences, io, play, editor, viewer and their parts.

#[cfg(test)]
#[path = "🧪️tests/🚀️activation/🦀️.rs"]
mod activation_tests;

//#region 🔖️ArtifactKind


/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// `🗒️note`'s own `pilot_languages()` convention (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// M1/W1b). Relocated from `⚙️engine/🦀️.rs` alongside `declaration()` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — `declaration()`'s only caller, kept private.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "cad.document",
                    extension: Some("cad"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_cad_cad_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_cad_cad_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_cad_cad_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_cad_cad_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("cad.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "cad.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_cad_cad_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_cad_cad_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_cad_cad_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_cad_cad_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("cad.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "cad.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_cad_cad_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_cad_cad_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("cad.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "cad.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_cad_cad_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_cad_cad_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("cad.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "cad.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_cad_cad_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_cad_cad_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("cad.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration freezes its schema, inference, codec, composer, and language
/// contributions before plugin assembly.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.cad.cad.standard.v1", "standard", "1", &[], None),
        ("s.cad.cad.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.cad.cad.schema.artifact", "schema", "s.cad.cad", &[("schema", "s.cad.cad")], None),
        ("s.cad.cad.inference.artifact", "inference", "s.cad.cad.inference", &[("schema", "s.cad.cad.inference")], None),
        ("s.cad.cad.composer.native", "composer", "s.cad.cad@1/*", &[("dialect", "s.cad.cad@1/*")], None),
        ("s.cad.cad.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.cad.cad.grammar.document", "grammar", "cad.document", &[("grammar", "cad.document")], None),
        ("s.cad.cad.grammar.op", "grammar", "cad.op", &[("grammar", "cad.op")], None),
        ("s.cad.cad.grammar.diff", "grammar", "cad.diff", &[("grammar", "cad.diff")], None),
        ("s.cad.cad.grammar.pack", "grammar", "cad.pack", &[("grammar", "cad.pack")], None),
        ("s.cad.cad.grammar.spr", "grammar", "cad.spr", &[("grammar", "cad.spr")], None),
        ("s.cad.cad.codec.document.v1", "codec", "cad.scene:cad", &[("codec", "cad.scene"), ("codec-extension", "9:cad.scene:cad")], None),
        ("s.cad.cad.localization.en", "localization", "CAD", &[], Some(("en", "CAD"))),
        ("s.cad.cad.localization.de", "localization", "CAD-Modellierung", &[], Some(("de", "CAD-Modellierung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.cad.cad")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(semio_s_artifact_cad_cad_model::cad_artifact_schema_descriptor())
        .inferences([semio_s_artifact_cad_cad_inferences::cad_artifact_inference_descriptor()])
        .composers(semio_s_artifact_cad_cad_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::app::EditorApp<semio_s_artifact_cad_cad_editor::CadPlayApp>>()
        .try_build()
}
//#endregion 🔖️ArtifactKind
