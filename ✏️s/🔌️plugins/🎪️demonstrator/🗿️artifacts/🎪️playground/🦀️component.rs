//! 🎪️ Playground artifact — demonstrator's owned document entity (minimal schema stub).

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub const PLAYGROUND_DOCUMENT_SCHEMA: &str = "playground.playground";

//#region 🔖️Dialect
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1 — the one `Dialect`
/// coordinate `PlaygroundEditor`/`PlaygroundViewer` (`✏️editor`/`👁️viewer`) both read `DIALECT` off,
/// lives at the ARTIFACT root (not under either surface) so a viewer file can read it without ever
/// importing through the sibling editor module. `artifact_kind` is the 3-part schema id
/// `PlaygroundArtifact`/`PlaygroundSnapshot` are keyed under (`#[artifact_schema(id = "…")]`), not the
/// 2-part `ArtifactIdentity::parse("s.playground")` string `definition()` above uses for a different,
/// older composer/registration purpose.
pub const PLAYGROUND_DIALECT: Dialect = Dialect { artifact_kind: "s.demonstrator.playground", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "playground.document".into(),
        name: "Playground Document".into(),
        source_format: PLAYGROUND_DOCUMENT_SCHEMA.into(),
        component_kind: "playground".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: PLAYGROUND_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called `register_composer_entries`/
/// `register_artifact_schema_descriptor`/`register_artifact_inference_descriptor`/
/// `dsl::register_language` (×5) directly from a plugin `.setup()` callback. Playground owns no
/// `ArtifactApp` (no pane's document schema is `PLAYGROUND_DOCUMENT_SCHEMA`), so there is no
/// `.document_codec()` call and no app-scope `register_app_schema_descriptor` escape hatch to keep —
/// every one of playground's §6 registrars fits this declaration with nothing left over. Lives at the
/// artifact root, not `⚙️engine`, per that ticket's taxonomy pass — `declaration()` describes the
/// artifact (kind/schema/io/ownership), it is not engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.playground.standard.v1", "standard", "1", &[], None),
        ("s.playground.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.playground.schema.artifact", "schema", "s.demonstrator.playground", &[("schema", "s.demonstrator.playground")], None),
        ("s.playground.inference.artifact", "inference", "s.demonstrator.playground.inference", &[("schema", "s.demonstrator.playground.inference")], None),
        ("s.playground.composer.native", "composer", "s.playground@1/*", &[("dialect", "s.playground@1/*")], None),
        ("s.playground.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.playground.composer.format-2", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.playground.composer.format-3", "composer", "s.stdio.xlsx@ecma-376/*", &[("dialect", "s.stdio.xlsx@ecma-376/*")], None),
        ("s.playground.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.playground.grammar.1", "grammar", "playground.document", &[("grammar", "playground.document")], None),
        ("s.playground.grammar.2", "grammar", "playground.op", &[("grammar", "playground.op")], None),
        ("s.playground.grammar.3", "grammar", "playground.diff", &[("grammar", "playground.diff")], None),
        ("s.playground.grammar.4", "grammar", "playground.pack", &[("grammar", "playground.pack")], None),
        ("s.playground.grammar.5", "grammar", "playground.spr", &[("grammar", "playground.spr")], None),
        ("s.playground.localization.en", "localization", "Playground", &[], Some(("en", "Playground"))),
        ("s.playground.localization.de", "localization", "Spielplatz", &[], Some(("de", "Spielplatz"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.playground")?);
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
        .schema(crate::artifacts::playground::standards::v1::subsets::any::schema::playground_artifact_schema_descriptor())
        .inferences([crate::artifacts::playground::standards::v1::subsets::any::schema::inferences::playground_artifact_inference_descriptor()])
        .composers(crate::artifacts::playground::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Private:
/// `declaration()` above is its only caller (moved here with it from `⚙️engine`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g7 revision — kept unexported, not widened).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES.get_or_init(build_pilot_languages).as_slice()
}

fn build_pilot_languages() -> Vec<dsl::LanguageSpec> {
    vec![playground_document_language(), playground_op_language(), playground_diff_language(), playground_pack_language(), playground_spr_language()]
}

fn playground_document_language() -> dsl::LanguageSpec {
    dsl::LanguageSpec {
        id: "playground.document",
        extension: Some("playground"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playground.document"),
    }
}

fn playground_op_language() -> dsl::LanguageSpec {
    dsl::LanguageSpec {
        id: "playground.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playground.op"),
    }
}

fn playground_diff_language() -> dsl::LanguageSpec {
    dsl::LanguageSpec {
        id: "playground.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("playground.diff"),
    }
}

fn playground_pack_language() -> dsl::LanguageSpec {
    dsl::LanguageSpec {
        id: "playground.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playground.pack"),
    }
}

fn playground_spr_language() -> dsl::LanguageSpec {
    dsl::LanguageSpec {
        id: "playground.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playground.spr"),
    }
}
//#endregion 🔖️Register
