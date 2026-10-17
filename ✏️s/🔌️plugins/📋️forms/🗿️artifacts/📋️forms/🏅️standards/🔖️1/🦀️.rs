//! 🏅️ Forms standard `1` root (design.md §1) — `pub fn standard() -> StandardDeclaration`, mounting
//! its one subset (`✳️any`). New file: this level did not exist before ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM.

pub fn standard<A: crate::FormsApplication>() -> semio_framework_plugin::app::declarations::StandardDeclaration<A> {
    use crate::standards::v1::subsets;
    use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
    use {semio_framework_artifact_reference::StandardId};

    StandardDeclaration {
        id: StandardId("1"),
        // 🪪️ `extensions: ["forms"]` is real, carried over from the native text codec's own
        // extension constant (`🚪️io/📝️text/📸️snapshot/🦀️.rs`) and the old `definition()`'s own
        // `"s.forms.codec.document.v1"` capability row (`extension` claim = `"forms"`). No real MIME
        // registration exists anywhere in the pre-migration code for this artifact (only that codec
        // id + extension claim) — `application/vnd.semio.forms+json` is a documented synthesis,
        // matching `🎬️sequence`'s identical documented deviation (`📓️w4-sequence-report.md`
        // openQuestion 1). Flag for whoever eventually wires a real media-type registry.
        media: MediaDeclaration { mimes: &["application/vnd.semio.forms+json"], extensions: &["forms"] },
        subsets: vec![subsets::any::subset()],
    }
}

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{FORMS_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::forms::create_forms_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "form.dictionary".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Form Dictionary", "Formularwörterbuch"),
        source_format: "form.dictionary".into(),
        component_kind: "forms".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: FORMS_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
//#region 🔖️Declaration
/// 🔖️ This artifact's OLD-channel definition (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1)
/// — kept per debt D1 (`📌️important.md`): not deleted repo-wide until W6. Ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM's `artifact()` below is the sole LIVE
/// registration channel (`.declare_artifact(...)`, plugin root); `definition()` has zero callers
/// left this pass (harmless — it still carries the real en/de localized names the new
/// `ArtifactDeclaration.localization` field does not yet populate, see that field's own doc and
/// `## openQuestions`). The old side-effecting `pilot_languages()`/`declaration()` pair (which
/// wired this artifact's hand-authored grammars into `dsl::register_languages` via the now-deleted
/// `.artifact(...)`/`.languages(...)` chain) is deleted outright, not kept: `NativeCodecs`'s
/// `LanguagePair` fields stay `{ text: None, binary: None }` (`🚪️io/🦀️.rs`), the same
/// documented deferral every other subset on this ticket carries — the underlying grammar/protocol
/// `.semio` assets themselves are untouched and still compiled into their own facet files.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    type CapabilityRow<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], Option<(&'a str, &'a str)>);
    let rows: &[CapabilityRow<'_>] = &[
        ("s.forms.forms.standard.v1", "standard", "1", &[], None),
        ("s.forms.forms.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.forms.forms.schema.artifact", "schema", "s.forms.forms", &[("schema", "s.forms.forms")], None),
        ("s.forms.forms.inference.artifact", "inference", "s.forms.forms.inference", &[("schema", "s.forms.forms.inference")], None),
        ("s.forms.forms.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.forms.forms.grammar.document", "grammar", "forms.forms", &[("grammar", "forms.forms")], None),
        ("s.forms.forms.grammar.op", "grammar", "forms.forms.op", &[("grammar", "forms.forms.op")], None),
        ("s.forms.forms.grammar.diff", "grammar", "forms.forms.diff", &[("grammar", "forms.forms.diff")], None),
        ("s.forms.forms.grammar.pack", "grammar", "forms.pack", &[("grammar", "forms.pack")], None),
        ("s.forms.forms.grammar.spr", "grammar", "forms.spr", &[("grammar", "forms.spr")], None),
        ("s.forms.forms.codec.document.v1", "codec", "forms.form:forms", &[("codec", "forms.form"), ("codec-extension", "10:forms.form:forms")], None),
        ("s.forms.forms.localization.en", "localization", "Forms", &[], Some(("en", "Forms"))),
        ("s.forms.forms.localization.de", "localization", "Formulare", &[], Some(("de", "Formulare"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.forms.forms")?);
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

/// 🌳️ This artifact's declaration tree root (design.md §1/§2) — ONE standard (`1`), ONE subset
/// (`any`). Sole registration channel (plugin root `.declare_artifact(artifact())`); the old
/// `.artifact(declaration())`/`.editor::<>()`/`.viewer::<>()` triad is deleted, not kept alongside
/// it (a second parallel registration channel is the compatibility layer CLAUDE.md forbids).
pub fn artifact<A: FormsApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};

    ArtifactDeclaration { kind: ArtifactKindId::parse("s.forms.forms").expect("canonical forms kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 📋️ Application variants required to assemble the Forms artifact.
pub trait FormsApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::forms::FormsPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::forms::FormsViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> FormsApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::forms::FormsPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::forms::FormsViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems4
