//! 🏙️ FEM 3D artifact — document entity types (constitutional: general).

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;







// #region 🔖️Register
/// 🧩️ Application wrappers supported by this artifact declaration.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_fem_3d_viewer::viewer::fem3d::Fem3dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_fem_3d_viewer::viewer::fem3d::Fem3dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.fem.fem3d.standard.v1", "standard", "1", &[], None),
        ("s.fem.fem3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.fem.fem3d.schema.artifact", "schema", "s.fem.fem3d", &[("schema", "s.fem.fem3d")], None),
        ("s.fem.fem3d.inference.artifact", "inference", "s.fem.fem3d.inference", &[("schema", "s.fem.fem3d.inference")], None),
        ("s.fem.fem3d.composer.fem3d", "composer", "s.fem.fem3d@1/*", &[("dialect", "s.fem.fem3d@1/*")], None),
        ("s.fem.fem3d.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.fem.fem3d.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.fem.fem3d.composer.txt", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.fem.fem3d.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.fem.fem3d.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.fem.fem3d.grammar.document", "grammar", "fem.fem3d", &[("grammar", "fem.fem3d")], None),
        ("s.fem.fem3d.grammar.op", "grammar", "fem.fem3d.op", &[("grammar", "fem.fem3d.op")], None),
        ("s.fem.fem3d.grammar.diff", "grammar", "fem.fem3d.diff", &[("grammar", "fem.fem3d.diff")], None),
        ("s.fem.fem3d.grammar.pack", "grammar", "fem3d.pack", &[("grammar", "fem3d.pack")], None),
        ("s.fem.fem3d.grammar.spr", "grammar", "fem3d.spr", &[("grammar", "fem3d.spr")], None),
        ("s.fem.fem3d.codec.document.v1", "codec", "fem.3d:fem3d", &[("codec", "fem.3d"), ("codec-extension", "6:fem.3d:fem3d")], None),
        ("s.fem.fem3d.localization.en", "localization", "Finite element model 3D", &[], Some(("en", "Finite element model 3D"))),
        ("s.fem.fem3d.localization.de", "localization", "Finite-Elemente-Modell 3D", &[], Some(("de", "Finite-Elemente-Modell 3D"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.fem.fem3d")?);
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
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.fem.fem3d").expect("canonical fem3d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

// #endregion 🔖️Register

// #region 🧪️Tests
/// 🪶️ The full-stack SQLite snapshot round trip through the declared artifact (editor and viewer included).
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot;
// #endregion 🧪️Tests


#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}





