//! 📐️ FEM 2D artifact — document entities (constitutional: general).

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;





// #region 🔖️Register
/// 🧩️ Application wrappers supported by this artifact declaration.
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_fem_2d_viewer::viewer::fem2d::Fem2dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_fem_2d_viewer::viewer::fem2d::Fem2dViewer>>>
{
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.fem.fem2d.standard.v1", "standard", "1", &[], None),
        ("s.fem.fem2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.fem.fem2d.schema.artifact", "schema", "s.fem.fem2d", &[("schema", "s.fem.fem2d")], None),
        ("s.fem.fem2d.inference.artifact", "inference", "s.fem.fem2d.inference", &[("schema", "s.fem.fem2d.inference")], None),
        // 🚪️ One composer claim per dialect `🚪️io/🦀️.rs::io()` registers on the io mechanism: the
        // native `s.fem.fem2d@1/*` plus the six stdio carriers (txt/json/csv/md both ways, stl/obj
        // export), the same shape `🗒️note` and `🧱️block` claim.
        ("s.fem.fem2d.composer.fem2d", "composer", "s.fem.fem2d@1/*", &[("dialect", "s.fem.fem2d@1/*")], None),
        ("s.fem.fem2d.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.fem.fem2d.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.fem.fem2d.composer.txt", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.fem.fem2d.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.fem.fem2d.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.fem.fem2d.grammar.document", "grammar", "fem.fem2d", &[("grammar", "fem.fem2d")], None),
        ("s.fem.fem2d.grammar.op", "grammar", "fem.fem2d.op", &[("grammar", "fem.fem2d.op")], None),
        ("s.fem.fem2d.grammar.diff", "grammar", "fem.fem2d.diff", &[("grammar", "fem.fem2d.diff")], None),
        ("s.fem.fem2d.grammar.pack", "grammar", "fem2d.pack", &[("grammar", "fem2d.pack")], None),
        ("s.fem.fem2d.grammar.spr", "grammar", "fem2d.spr", &[("grammar", "fem2d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Fem2dPlayApp>>()` derives its
        // codec claim from `Fem2dPlayApp::DOCUMENT_SCHEMA` (= `FEM_2D_SCHEMA`, `🦀️.rs`),
        // which is `"fem.2d"`, not `"fem.fem2d"`.
        ("s.fem.fem2d.codec.document.v1", "codec", "fem.2d:fem2d", &[("codec", "fem.2d"), ("codec-extension", "6:fem.2d:fem2d")], None),
        ("s.fem.fem2d.localization.en", "localization", "Finite element model 2D", &[], Some(("en", "Finite element model 2D"))),
        ("s.fem.fem2d.localization.de", "localization", "Finite-Elemente-Modell 2D", &[], Some(("de", "Finite-Elemente-Modell 2D"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.fem.fem2d")?);
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
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.fem.fem2d").expect("canonical fem2d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

// #endregion 🔖️Register

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
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
                            mod tests;
                        }
                    }
                }
            }
        }
    }
}





