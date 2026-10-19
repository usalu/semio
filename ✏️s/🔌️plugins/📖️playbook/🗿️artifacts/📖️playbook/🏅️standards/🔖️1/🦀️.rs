//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real
//! MIME registration exists for `s.playbook.playbook` outside the old `ArtifactDefinition`'s
//! capability rows (`definition()`, kept — debt D1) — that channel only ever claimed a codec id
//! (`playbook.playbook:playbook`) and an extension (`playbook`), never a mime type.
//! `mimes: ["application/vnd.semio.playbook+json"]` is a documented synthesis (matches `🗒️note`/
//! `🖍️draw`/`🔱️trinity`'s identical documented deviation); `extensions: ["playbook"]` is the real,
//! carried-over value from `definition()`'s `s.playbook.codec.document.v1` row.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<A: crate::PlaybookApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.playbook+json"], extensions: &["playbook"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use semio_framework_artifact_playbook_playbook as playbook;

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. `crate::editor::playbook::config::schema::register_app_schema()` is the
/// one exception, still called from this file's own `.setup()`: it registers the `PlaybookPlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set. Lives at the artifact root, not `⚙️engine` (reloc-g7 revision of that same ticket) —
/// `declaration()` describes the artifact (kind/schema/io/ownership), it is not engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    type CapabilityRow<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], Option<(&'a str, &'a str)>);
    let rows: &[CapabilityRow<'_>] = &[
        ("s.playbook.playbook.standard.v1", "standard", "1", &[], None),
        ("s.playbook.playbook.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.playbook.playbook.schema.artifact", "schema", "s.playbook.playbook", &[("schema", "s.playbook.playbook")], None),
        ("s.playbook.playbook.inference.artifact", "inference", "s.playbook.playbook.inference", &[("schema", "s.playbook.playbook.inference")], None),
        ("s.playbook.playbook.composer.playbook", "composer", "s.playbook.playbook@1/*", &[("dialect", "s.playbook.playbook@1/*")], None),
        ("s.playbook.playbook.composer.txt", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.playbook.playbook.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.playbook.playbook.grammar.document", "grammar", "playbook.playbook", &[("grammar", "playbook.playbook")], None),
        ("s.playbook.playbook.grammar.op", "grammar", "playbook.playbook.op", &[("grammar", "playbook.playbook.op")], None),
        ("s.playbook.playbook.grammar.diff", "grammar", "playbook.playbook.diff", &[("grammar", "playbook.playbook.diff")], None),
        ("s.playbook.playbook.grammar.pack", "grammar", "playbook.pack", &[("grammar", "playbook.pack")], None),
        ("s.playbook.playbook.grammar.spr", "grammar", "playbook.spr", &[("grammar", "playbook.spr")], None),
        ("s.playbook.playbook.codec.document.v1", "codec", "playbook.playbook:playbook", &[("codec", "playbook.playbook"), ("codec-extension", "17:playbook.playbook:playbook")], None),
        ("s.playbook.playbook.localization.en", "localization", "Playbook", &[], Some(("en", "Playbook"))),
        ("s.playbook.playbook.localization.de", "localization", "Playbook", &[], Some(("de", "Playbook"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.playbook.playbook")?);
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

/// 🌳️ This artifact's declaration tree root (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
/// RUNTIME`, `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe) —
/// replaces the old `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...)
/// .composers(...).languages(...).document_codec(...)` chain, deleted outright, no dual channel) as
/// the ONLY registration channel for schema/io/viewer/editor rows. `definition()` (old
/// `ArtifactDefinition`/capability rows, above) is kept per debt D1.
pub fn artifact<A: PlaybookApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.playbook.playbook").expect("canonical playbook kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧰️ Application variants required to assemble the Playbook artifact.
pub trait PlaybookApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::playbook::PlaybookPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::playbook::PlaybookViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> PlaybookApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::playbook::PlaybookPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::playbook::PlaybookViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Private:
/// `declaration()` above is its only caller (moved here with it from `⚙️engine`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g7 — kept unexported, not widened).
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.playbook",
                    extension: Some("playbook"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.playbook"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.playbook.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.playbook.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.playbook.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.playbook.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "playbook.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("playbook.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register
}
//#endregion 🧩️ArtifactItems5
