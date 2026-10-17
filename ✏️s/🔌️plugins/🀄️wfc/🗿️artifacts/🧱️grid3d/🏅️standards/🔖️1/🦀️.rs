//! 🏅️ Standard root for `s.wfc.grid3d@1` — `standard() -> StandardDeclaration`, mounting the one
//! subset this artifact has (`✳️any`). `extensions` is the real `Grid3dSnapshot::EXTENSION`;
//! `mimes` follows the documented `application/vnd.semio.<kind>+json` synthesis every sibling
//! artifact carries, since no external MIME registration exists for this dialect.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfcgrid3d+json"], extensions: &["wfcgrid3d"] }, subsets: vec![subsets::any::subset::<PA>()] }
}

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
//#region 🔖️Declaration
/// 🧾️ Defines `s.wfc.grid3d`'s immutable runtime capability leaves — standard/profile/schema/
/// inference/composer/codec/localization, the same row set `s.puzzle.puzzle2d` declares.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.wfc.grid3d.standard.v1", "standard", "1", &[], None),
        ("s.wfc.grid3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.wfc.grid3d.schema.artifact", "schema", "s.wfc.grid3d", &[("schema", "s.wfc.grid3d")], None),
        ("s.wfc.grid3d.inference.artifact", "inference", "s.wfc.grid3d.solve", &[("schema", "s.wfc.grid3d.solve")], None),
        ("s.wfc.grid3d.composer.native", "composer", "s.wfc.grid3d@1/*", &[("dialect", "s.wfc.grid3d@1/*")], None),
        ("s.wfc.grid3d.codec.document-1", "codec", "s.wfc.grid3d:wfcgrid3d", &[("codec", "s.wfc.grid3d"), ("codec-extension", "13:s.wfc.grid3d:wfcgrid3d")], None),
        ("s.wfc.grid3d.localization.en", "localization", "3D Grid", &[], Some(("en", "3D Grid"))),
        ("s.wfc.grid3d.localization.de", "localization", "3D-Raster", &[], Some(("de", "3D-Raster"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.wfc.grid3d")?);
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

/// 🎭️ The closed app roster this artifact's declaration tree binds — its own editor and viewer, the
/// exact bound `semio_s_artifact_puzzle_2d::artifact` carries.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::grid3d::Grid3dEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::grid3d::Grid3dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::grid3d::Grid3dEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::grid3d::Grid3dViewer>>>
{
}

/// 🌳️ This artifact's declaration tree root — the ONLY registration channel for schema/io/viewer/
/// editor rows, mirroring `semio_s_artifact_puzzle_2d::artifact`.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.grid3d").expect("canonical wfc grid3d kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`.
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "wfc.grid3d",
                    extension: Some("wfcgrid3d"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc.grid3d"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc.grid3d.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc.grid3d.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc.grid3d.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc.grid3d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc.grid3d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc.grid3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems4
