//! 🏅️ Standard root for `s.wfc.wfc3d@1` — `standard() -> StandardDeclaration`, mounting subset `any`,
//! this artifact's only subset. `mimes` is a documented synthesis (no MIME registration exists for
//! this kind outside the capability rows); `extensions` carries the real `wfc3d` value the codec
//! capability claims.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfc3d+json"], extensions: &["wfc3d"] }, subsets: vec![subsets::any::subset::<PA>()] }
}

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
//#region 🔖️Declaration
/// 🧾️ Defines `s.wfc.wfc3d`'s immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.wfc.wfc3d.standard.v1", "standard", "1", &[], None),
        ("s.wfc.wfc3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.wfc.wfc3d.schema.artifact", "schema", "s.wfc.wfc3d", &[("schema", "s.wfc.wfc3d")], None),
        ("s.wfc.wfc3d.inference.artifact", "inference", "s.wfc.wfc3d.solve", &[("schema", "s.wfc.wfc3d.solve")], None),
        ("s.wfc.wfc3d.composer.native", "composer", "s.wfc.wfc3d@1/*", &[("dialect", "s.wfc.wfc3d@1/*")], None),
        ("s.wfc.wfc3d.grammar.1", "grammar", "wfc3d.snapshot", &[("grammar", "wfc3d.snapshot")], None),
        ("s.wfc.wfc3d.grammar.2", "grammar", "wfc3d.mutations", &[("grammar", "wfc3d.mutations")], None),
        ("s.wfc.wfc3d.codec.document-1", "codec", "s.wfc.wfc3d:wfc3d", &[("codec", "s.wfc.wfc3d"), ("codec-extension", "5:s.wfc.wfc3d:wfc3d")], None),
        ("s.wfc.wfc3d.localization.en", "localization", "3D", &[], Some(("en", "3D"))),
        ("s.wfc.wfc3d.localization.de", "localization", "3D", &[], Some(("de", "3D"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.wfc.wfc3d")?);
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

/// 🎭️ The two apps this artifact contributes to its plugin's closed fleet.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::wfc3d::Wfc3dEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::wfc3d::Wfc3dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<crate::editor::wfc3d::Wfc3dEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<crate::viewer::wfc3d::Wfc3dViewer>>>
{
}

/// 🌳️ This artifact's declaration tree root — the ONLY registration channel for schema/io/viewer/
/// editor rows.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.wfc3d").expect("canonical wfc3d kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) — built once and leaked to a
/// `&'static` slice since `dsl::passthrough_hooks` is not `const fn`. Index order is fixed:
/// 0 = document, 1 = op, 2 = diff, 3 = pack, 4 = spr.
pub fn wfc3d_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "wfc3d.snapshot",
                    extension: Some("wfc3d"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc3d.snapshot"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc3d.mutations",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc3d.mutations"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc3d.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("wfc3d.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc3d.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc3d.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "wfc3d.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("wfc3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems4
