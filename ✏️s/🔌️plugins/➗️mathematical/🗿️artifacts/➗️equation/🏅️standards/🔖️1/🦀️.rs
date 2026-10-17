//! 🏅️ Standard root for `s.mathematical.equation` standard `1` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM). Exports `standard() -> StandardDeclaration`,
//! mounting subset `any` (the only subset this standard has). No real mime/extension registration
//! exists for this artifact anywhere in the codebase (unlike stdio's `📜️artifact-definition.json`)
//! — the old `ArtifactCapability` channel (`crate::definition`) only ever
//! claimed a codec id (`semio.equation/v1`) and an extension (`equation`), never a mime
//! type, so `mimes` here is a documented synthesis, not a literal carry-over (see the W4 report's
//! `## openQuestions`, mirrors `🎬️sequence`'s identical documented deviation).

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<A: crate::EquationApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.equation+json"], extensions: &["equation"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
//#region 🔖️Declaration
/// 📌️ ⚠️ DEAD (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): was the sole grammar/
/// protocol source for the OLD `declaration()` (`.languages(pilot_languages())`), deleted below in
/// the atomic cutover to `artifact()`/`.declare_artifact(...)`. Kept only as a documented historical
/// marker of why `NativeCodecs.{snapshot,diff,mutations,inferences}: LanguagePair { text: None,
/// binary: None }` in `🚪️io/🦀️.rs::io()` is a deliberate scope-narrowing, not an
/// oversight — the five roles below are exactly the five `dsl::LanguageSpec`s that field still
/// needs wiring one day. Handcrafted facet grammars (text) and protocols (binary) for in-process
/// execution — built once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't
/// `const fn`, mirroring note's `pilot_languages()` convention.
#[allow(dead_code)]
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "equation.document",
                    extension: Some("equation"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("equation.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "equation.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("equation.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "equation.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("equation.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "equation.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("equation.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "equation.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("equation.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from a
/// plugin `.setup()` callback (see `🗒️note`'s exemplar conversion, same shape).
/// `crate::editor::equation::config::schema::register_app_schema()` is the one exception, still called
/// from this file's own `.setup()`: it registers the `EquationPlayApp` CONFIG/PRESENCE schema, an
/// app-scope concern `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in the §6 artifact-scoped set.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.mathematical.equation.standard.v1", "standard", "1", &[], None),
        ("s.mathematical.equation.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.mathematical.equation.schema.artifact", "schema", "s.mathematical.equation", &[("schema", "s.mathematical.equation")], None),
        ("s.mathematical.equation.inference.artifact", "inference", "s.mathematical.equation.inference", &[("schema", "s.mathematical.equation.inference")], None),
        ("s.mathematical.equation.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.mathematical.equation.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.mathematical.equation.grammar.document", "grammar", "equation.document", &[("grammar", "equation.document")], None),
        ("s.mathematical.equation.grammar.op", "grammar", "equation.op", &[("grammar", "equation.op")], None),
        ("s.mathematical.equation.grammar.diff", "grammar", "equation.diff", &[("grammar", "equation.diff")], None),
        ("s.mathematical.equation.grammar.pack", "grammar", "equation.pack", &[("grammar", "equation.pack")], None),
        ("s.mathematical.equation.grammar.spr", "grammar", "equation.spr", &[("grammar", "equation.spr")], None),
        ("s.mathematical.equation.codec.document.v1", "codec", "semio.equation/v1:equation", &[("codec", "semio.equation/v1"), ("codec-extension", "17:semio.equation/v1:equation")], None),
        ("s.mathematical.equation.localization.en", "localization", "Equation", &[], Some(("en", "Equation"))),
        ("s.mathematical.equation.localization.de", "localization", "Gleichung", &[], Some(("de", "Gleichung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.mathematical.equation")?);
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

/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): the whole
/// `s.mathematical.equation` artifact through the declaration tree — one standard (`1`), one
/// subset (`any`). Replaces the OLD `declaration()`/`ArtifactDeclaration::builder(...)` channel
/// outright (atomic cutover; both channels never coexist — the OLD channel's `.composers(...)`
/// registered a native composer entry (`EquationComposerComposition`, writing the
/// non-canonical, under-qualified `Dialect{artifact_kind:"s.mathematical",...}` coordinate) with
/// NO matching `composer` capability row in `definition()` above — `definition()` only ever
/// declared composer capabilities for the two EXPORT directions (`s.stdio.md`/`s.stdio.json`), never
/// for this artifact's own native composer — which is exactly what shipped this plugin's WASM
/// manifest as `assembly-failed` (`try_build()`'s `runtime_capability_requirements`/capability-row
/// mismatch faulted every `try_build()` call). `declare_artifact`/`artifact()` do not run that same
/// composer-capability preflight at all — the new tree's io hops are typed `Serializer`/
/// `Deserializer` entries validated by `io_register`, not `ComposerEntry` capability rows — so this
/// cutover fixes the manifest as a side effect of deleting the broken channel, not a separate fix.
/// `localization: &[]` is a documented shortfall, not an oversight — the real en/de localized
/// descriptors still live on `definition()`'s `ArtifactCapability` rows above (kept per debt D1,
/// deleted repo-wide only in W6); wiring them into this field too is real follow-up work, not
/// required for the tree to register or for any law to hold (mirrors `🎬️sequence`'s and the stdio
/// pilot's own documented deviation).
pub fn artifact<A: EquationApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.mathematical.equation").expect("canonical mathematical.equation kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait EquationApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::equation::EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::equation::EquationViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> EquationApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::equation::EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::equation::EquationViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems6
