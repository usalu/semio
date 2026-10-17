//! 🏅️ Standard root — `standard() -> StandardDeclaration`, mounting the single `✳️any` subset.
//! `extensions` carries this artifact's real `ArtifactDsl::EXTENSION` (`wfcgrid2d`, the same string
//! `definition()`'s `s.wfc.grid2d.codec.document-1` row claims); `mimes` is the documented
//! `application/vnd.semio.<artifact>+json` synthesis every sibling artifact uses, since no real
//! MIME registration exists for a semio-native dialect.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfcgrid2d+json"], extensions: &["wfcgrid2d"] }, subsets: vec![subsets::any::subset::<PA>()] }
}

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
pub use crate::standards::v1::subsets::any::schema::snapshot::WFC_GRID2D_DOCUMENT_SCHEMA;

//#region 🔖️ArtifactKind
/// 🗿️ The `2d.wfcgrid2d` artifact kind — the artifact, not the app, owns its identity. The document
/// renders as a 2D design (tile media drawn into cell rects), so `dimension`/`media_class` follow
/// `2d.puzzle`'s precedent rather than assembly's headless `data` shape.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.wfcgrid2d".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("2D Grid", "2D-Gitter"),
        source_format: WFC_GRID2D_DOCUMENT_SCHEMA.into(),
        component_kind: "wfcgrid2d".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
        schema: WFC_GRID2D_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
//#region 🔖️Declaration
/// 🧾️ Defines `s.wfc.grid2d`'s immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.wfc.grid2d.standard.v1", "standard", "1", &[], None),
        ("s.wfc.grid2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.wfc.grid2d.schema.artifact", "schema", "s.wfc.grid2d", &[("schema", "s.wfc.grid2d")], None),
        ("s.wfc.grid2d.inference.artifact", "inference", "s.wfc.grid2d.solve", &[("schema", "s.wfc.grid2d.solve")], None),
        ("s.wfc.grid2d.composer.native", "composer", "s.wfc.grid2d@1/*", &[("dialect", "s.wfc.grid2d@1/*")], None),
        ("s.wfc.grid2d.composer.format-1", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.wfc.grid2d.composer.format-2", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.wfc.grid2d.grammar.1", "grammar", "wfc.grid2d", &[("grammar", "wfc.grid2d")], None),
        ("s.wfc.grid2d.grammar.2", "grammar", "wfc.grid2d.op", &[("grammar", "wfc.grid2d.op")], None),
        ("s.wfc.grid2d.grammar.3", "grammar", "wfc.grid2d.diff", &[("grammar", "wfc.grid2d.diff")], None),
        ("s.wfc.grid2d.grammar.4", "grammar", "wfc.grid2d.pack", &[("grammar", "wfc.grid2d.pack")], None),
        ("s.wfc.grid2d.grammar.5", "grammar", "wfc.grid2d.spr", &[("grammar", "wfc.grid2d.spr")], None),
        ("s.wfc.grid2d.codec.document-1", "codec", "s.wfc.grid2d:wfcgrid2d", &[("codec", "s.wfc.grid2d"), ("codec-extension", "13:s.wfc.grid2d:wfcgrid2d")], None),
        ("s.wfc.grid2d.localization.en", "localization", "2D Grid", &[], Some(("en", "2D Grid"))),
        ("s.wfc.grid2d.localization.de", "localization", "2D-Raster", &[], Some(("de", "2D-Raster"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.wfc.grid2d")?);
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

/// 🎭️ The two app bounds this artifact's declaration tree threads through `standard()`/`subset()`.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::grid2d::Grid2dEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::grid2d::Grid2dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::grid2d::Grid2dEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::grid2d::Grid2dViewer>>>
{
}

/// 🌳️ This artifact's declaration tree root — the ONLY registration channel for
/// schema/io/viewer/editor rows. `definition()` above is the separate capability-row channel.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.grid2d").expect("canonical wfc grid2d kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems3
