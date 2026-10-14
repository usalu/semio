//! 🔷️ Lowpoly artifact aggregator — the `s.lowpoly.lowpoly` definition and its declaration (schema descriptor
//! assembled from the facet-owned leaves, inferences, composers, languages and the editor document codec). Every domain
//! facet lives in its own family crate (`semio-s-artifact-lowpoly-lowpoly-*`).

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🔖️Declaration
/// 🪪️ The artifact definition: identity, standard/profile, schema/inference/composer/grammar/codec capabilities and
/// localizations.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.lowpoly.lowpoly.standard.v1", "standard", "1", &[], None),
        ("s.lowpoly.lowpoly.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.lowpoly.lowpoly.schema.artifact", "schema", "s.lowpoly.lowpoly", &[("schema", "s.lowpoly.lowpoly")], None),
        ("s.lowpoly.lowpoly.inference.artifact", "inference", "s.lowpoly.lowpoly.inference", &[("schema", "s.lowpoly.lowpoly.inference")], None),
        ("s.lowpoly.lowpoly.composer.native", "composer", "s.lowpoly.lowpoly@1/*", &[("dialect", "s.lowpoly.lowpoly@1/*")], None),
        ("s.lowpoly.lowpoly.composer.format-1", "composer", "s.stdio.las@1.0/*", &[("dialect", "s.stdio.las@1.0/*")], None),
        ("s.lowpoly.lowpoly.composer.format-2", "composer", "s.stdio.ply@1.0/*", &[("dialect", "s.stdio.ply@1.0/*")], None),
        ("s.lowpoly.lowpoly.composer.format-3", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.lowpoly.lowpoly.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.lowpoly.lowpoly.composer.format-5", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.lowpoly.lowpoly.composer.format-6", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.lowpoly.lowpoly.composer.format-7", "composer", "s.stdio.gltf@2.0/*", &[("dialect", "s.stdio.gltf@2.0/*")], None),
        ("s.lowpoly.lowpoly.composer.format-8", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.lowpoly.lowpoly.composer.format-9", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.lowpoly.lowpoly.grammar.1", "grammar", "lowpoly.document", &[("grammar", "lowpoly.document")], None),
        ("s.lowpoly.lowpoly.grammar.2", "grammar", "lowpoly.op", &[("grammar", "lowpoly.op")], None),
        ("s.lowpoly.lowpoly.grammar.3", "grammar", "lowpoly.diff", &[("grammar", "lowpoly.diff")], None),
        ("s.lowpoly.lowpoly.grammar.4", "grammar", "lowpoly.pack", &[("grammar", "lowpoly.pack")], None),
        ("s.lowpoly.lowpoly.grammar.5", "grammar", "lowpoly.spr", &[("grammar", "lowpoly.spr")], None),
        ("s.lowpoly.lowpoly.codec.document-1", "codec", "lowpoly.document:lowpoly", &[("codec", "lowpoly.document"), ("codec-extension", "16:lowpoly.document:lowpoly")], None),
        ("s.lowpoly.lowpoly.localization.en", "localization", "Lowpoly", &[], Some(("en", "Lowpoly"))),
        ("s.lowpoly.lowpoly.localization.de", "localization", "Niedrigpolygon", &[], Some(("de", "Niedrigpolygon"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.lowpoly.lowpoly")?);
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

/// 📜️ This artifact's declaration — mesh/DWG/stdio format handling flows entirely through `.composers(...)`.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::lowpoly_artifact_schema_descriptor())
        .inferences([semio_s_artifact_lowpoly_lowpoly_inferences::lowpoly_artifact_inference_descriptor()])
        .composers(semio_s_artifact_lowpoly_lowpoly_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_lowpoly_lowpoly_editor::LowpolyPlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "lowpoly.document",
                    extension: Some("lowpoly"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("lowpoly.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "lowpoly.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_lowpoly_lowpoly_mutations::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_lowpoly_lowpoly_mutations::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_lowpoly_lowpoly_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_lowpoly_lowpoly_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("lowpoly.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "lowpoly.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_lowpoly_lowpoly_model_diff::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_lowpoly_lowpoly_model_diff::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("lowpoly.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "lowpoly.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("lowpoly.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "lowpoly.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_lowpoly_lowpoly_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_lowpoly_lowpoly_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("lowpoly.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

/// 🧬️ Descriptor for `s.lowpoly.lowpoly`, assembled from the facet-owned schema leaves.
pub fn lowpoly_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.lowpoly.lowpoly",
        artifact: semio_s_artifact_lowpoly_lowpoly_model::ARTIFACT_LEAVES,
        snapshot: semio_s_artifact_lowpoly_lowpoly_model::snapshot::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_lowpoly_lowpoly_model_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_lowpoly_lowpoly_mutations::MUTATION_LEAVES,
    }
}
