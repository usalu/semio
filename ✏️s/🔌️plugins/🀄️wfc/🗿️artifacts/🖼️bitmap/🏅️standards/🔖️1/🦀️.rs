//! 🏅️ Standard root — `standard() -> StandardDeclaration`, mounting the one subset `✳️any`.
//! `mimes`/`extensions` mirror `definition()`'s own codec capability row: the extension is
//! `BitmapSnapshot::EXTENSION` verbatim, the mime a documented synthesis (no registry entry exists
//! for a semio-native dialect), the same deviation every sibling artifact's standard root carries.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfcbitmap+json"], extensions: &["wfcbitmap"] }, subsets: vec![subsets::any::subset::<PA>()] }
}

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
pub use crate::schema::snapshot::WFC_BITMAP_DOCUMENT_SCHEMA;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`. A bitmap problem is genuinely two-dimensional raster media
/// — its input IS an image and its inferred output is another — so `dimension`/`media_class`/
/// `media_form` say `2d`/`TwoD`/`Raster` rather than the headless `data` shape a pure rule spec
/// would take.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.wfcbitmap".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("WFC Bitmap", "WFC-Bitmap"),
        source_format: WFC_BITMAP_DOCUMENT_SCHEMA.into(),
        component_kind: "wfcbitmap".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        schema: WFC_BITMAP_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.txt".into(), "stdio.json".into()],
        import_stdio_kinds: vec!["stdio.txt".into(), "stdio.json".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
//#region 🔖️Declaration
/// 🧾️ Defines `s.wfc.bitmap`'s immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.wfc.bitmap.standard.v1", "standard", "1", &[], None),
        ("s.wfc.bitmap.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.wfc.bitmap.schema.artifact", "schema", "s.wfc.bitmap", &[("schema", "s.wfc.bitmap")], None),
        ("s.wfc.bitmap.inference.artifact", "inference", "s.wfc.bitmap.solve", &[("schema", "s.wfc.bitmap.solve")], None),
        ("s.wfc.bitmap.composer.native", "composer", "s.wfc.bitmap@1/*", &[("dialect", "s.wfc.bitmap@1/*")], None),
        ("s.wfc.bitmap.composer.format-1", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.wfc.bitmap.composer.format-2", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.wfc.bitmap.grammar.1", "grammar", "wfc.bitmap", &[("grammar", "wfc.bitmap")], None),
        ("s.wfc.bitmap.grammar.2", "grammar", "wfc.bitmap.op", &[("grammar", "wfc.bitmap.op")], None),
        ("s.wfc.bitmap.codec.document-1", "codec", "s.wfc.bitmap:wfcbitmap", &[("codec", "s.wfc.bitmap"), ("codec-extension", "12:s.wfc.bitmap:wfcbitmap")], None),
        ("s.wfc.bitmap.localization.en", "localization", "Bitmap", &[], Some(("en", "Bitmap"))),
        ("s.wfc.bitmap.localization.de", "localization", "Bitmap", &[], Some(("de", "Bitmap"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.wfc.bitmap")?);
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

/// 🎭️ The two app bindings every declaration of this artifact must be able to absorb.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::bitmap::BitmapEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::bitmap::BitmapViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::bitmap::BitmapEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::bitmap::BitmapViewer>>>
{
}

/// 🌳️ This artifact's declaration tree root — the ONLY registration channel for schema/io/viewer/
/// editor rows. `definition()` above is the separate capability-row channel.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.bitmap").expect("canonical wfc bitmap kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. The index order
/// `[document, op, pack, spr]` is the one `io()` reads its `NativeCodecs` slots from.
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "wfc.bitmap",
                    extension: Some("wfcbitmap"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc.bitmap"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc.bitmap.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc.bitmap.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc.bitmap.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc.bitmap.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc.bitmap.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc.bitmap.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems4
