//! 🏅️ Standard root — `standard() -> StandardDeclaration`, mounting the one subset this artifact
//! has. `mimes`/`extensions` are the artifact's own: `.wfc2d` is the extension the native codec
//! claims (`ArtifactDsl::EXTENSION`), and the vendor MIME follows the same synthesis every sibling
//! artifact documents, since no MIME registration exists for a semio artifact kind.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfc2d+json"], extensions: &["wfc2d"] }, subsets: vec![subsets::any::subset::<PA>()] }
}

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
pub use crate::schema::snapshot::WFC_2D_DOCUMENT_SCHEMA;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗿️ The `2d.wfc2d` artifact kind — a 2D design surface: the document is a plane of rectangles with
/// vector/bitmap media, so `dimension`/`media_type` say `2d`/`Design` rather than assembly's headless
/// `data`/`Value`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.wfc2d".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("2D", "2D"),
        source_format: WFC_2D_DOCUMENT_SCHEMA.into(),
        component_kind: "wfc2d".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
        schema: WFC_2D_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
//#region 🔖️Declaration
/// 🧾️ The app bounds every declaration function in this crate carries — one editor app and one
/// viewer app, exactly like `semio_s_artifact_puzzle_2d::ArtifactApps`.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::wfc2d::Wfc2dEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::wfc2d::Wfc2dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::wfc2d::Wfc2dEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::wfc2d::Wfc2dViewer>>>
{
}

/// 🧾️ Defines `s.wfc.wfc2d`'s immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.wfc.wfc2d.standard.v1", "standard", "1", &[], None),
        ("s.wfc.wfc2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.wfc.wfc2d.schema.artifact", "schema", "s.wfc.wfc2d", &[("schema", "s.wfc.wfc2d")], None),
        ("s.wfc.wfc2d.inference.artifact", "inference", "s.wfc.wfc2d.solve", &[("schema", "s.wfc.wfc2d.solve")], None),
        ("s.wfc.wfc2d.composer.native", "composer", "s.wfc.wfc2d@1/*", &[("dialect", "s.wfc.wfc2d@1/*")], None),
        ("s.wfc.wfc2d.grammar.1", "grammar", "wfc.wfc2d", &[("grammar", "wfc.wfc2d")], None),
        ("s.wfc.wfc2d.grammar.2", "grammar", "wfc.wfc2d.op", &[("grammar", "wfc.wfc2d.op")], None),
        ("s.wfc.wfc2d.grammar.3", "grammar", "wfc.wfc2d.diff", &[("grammar", "wfc.wfc2d.diff")], None),
        ("s.wfc.wfc2d.grammar.4", "grammar", "wfc.wfc2d.pack", &[("grammar", "wfc.wfc2d.pack")], None),
        ("s.wfc.wfc2d.grammar.5", "grammar", "wfc.wfc2d.spr", &[("grammar", "wfc.wfc2d.spr")], None),
        ("s.wfc.wfc2d.codec.document-1", "codec", "s.wfc.wfc2d:wfc2d", &[("codec", "s.wfc.wfc2d"), ("codec-extension", "12:s.wfc.wfc2d:wfc2d")], None),
        ("s.wfc.wfc2d.localization.en", "localization", "2D", &[], Some(("en", "2D"))),
        ("s.wfc.wfc2d.localization.de", "localization", "2D", &[], Some(("de", "2D"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.wfc.wfc2d")?);
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

/// 🌳️ This artifact's declaration tree root — the ONLY registration channel for schema/io/viewer/
/// editor rows.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.wfc2d").expect("canonical wfc2d kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems3
