#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

// GIS terrain artifact — the document entity the 3d app edits (constitutional: general).

/// 📸️ Persisted GIS terrain snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
pub use crate::schema::snapshot::GisTerrainSnapshot;

pub use semio_s_artifact_gis_gisterrain_inferences::{GISTERRAIN_DIALECT, GIS_3D_TERRAIN_SCHEMA};

//#region 🔹Types
//#endregion 🔹Types

// 🧱️ `mesh_artifact_kind()` (the shared `3d.mesh` interchange kind duplicate) REMOVED — ticket
// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`. `3d.mesh` is a duplicate kind id being deleted
// repo-wide: mesh is now canonically `s.stdio.semio@v1/mesh` (composed via `GisTerrainSnapshot.mesh:
// Option<store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot>>` below, never a standalone `ArtifactKindSpec`).
// lowpoly independently declared the identical id and removed its own copy first (see that
// plugin's `🦀️.rs`, same region); this removes gis's copy, the other place this ticket's
// design doc (`📓️design-full-plan.md` §4: "gis→terrain C:mesh … kills `3d.mesh` dup") flagged it.
// Registration call site removed from `🎛️apps/🧊️3d/🦀️.rs`'s `create_gis3d_app()`
// (`.artifact_kind(mesh_artifact_kind())`).

//#region 🔹ArtifactKind
/// 🏔️ The canonical GIS terrain artifact-kind declaration: one id with its dialect and its native codec receipt
/// (`s.gis.gisterrain`), so the hub's one open-target rule pairs it with the terrain editor and viewer; the payload
/// schema stays `gis.terrain`.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: GISTERRAIN_DIALECT.artifact_kind.into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D Terrain", "3D-Gelände"),
        source_format: GIS_3D_TERRAIN_SCHEMA.into(),
        component_kind: "gisterrain".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        schema: GIS_3D_TERRAIN_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: standards::v1::subsets::any::io::export_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
        import_stdio_kinds: standards::v1::subsets::any::io::import_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
    }
}
//#endregion 🔹ArtifactKind

pub use semio_s_artifact_gis_gisterrain_inferences::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key, gis_terrain_mesh_from_snapshot};

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// the plugin root's `register_gis_exports` fan-out. Relocated from `⚙️engine` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2): `declaration()` describes the artifact
/// (kind, schema, io ports, ownership), which is not engine behaviour.
/// 🧾️ Defines s.gis.gisterrain's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    ArtifactDefinition::new(ArtifactIdentity::parse("s.gis.gisterrain")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.gis.gisterrain")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gisterrain")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.gis.gisterrain.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gisterrain.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.gis.gisterrain@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.gis.gisterrain@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.composer.las")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.las@1.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.las@1.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.composer.ply")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.ply@1.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.ply@1.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.composer.stl")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.stl@ascii/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.stl@ascii/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.composer.gltf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.gltf@2.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.gltf@2.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.composer.obj")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.obj@3.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.obj@3.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"gis.terrain:gisterrain")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "gis.terrain")?)?
                .claim(ArtifactIdentityClaim::codec_extension("gis.terrain", "gisterrain")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"GIS Terrain")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "GIS Terrain")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gisterrain.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"GIS Gel\xc3\xa4nde")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "GIS Gelände")?)?,
        )
}

/// 🔖️ Assembles s.gis.gisterrain's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::gisterrain_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::gisterrain_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec_bare::<GisTerrainSnapshot, GisTerrainMutation>(GIS_3D_TERRAIN_SCHEMA, GISTERRAIN_DIALECT)
        .try_build()
}
//#endregion 🔖️Register

//#region 🔹Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔹Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::operations;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod change_exaggeration {
                            pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::change_exaggeration::*;
                            pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::change_exaggeration::diff;
                            pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::change_exaggeration::inverse;

                        }
                        #[path = "."]
                        pub mod change_imported_features {
                            pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::change_imported_features::*;
                            pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::change_imported_features::diff;
                            pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::change_imported_features::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::standards::v1::subsets::any::io::binary::inferences;
    pub mod mutations {
        pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::io::binary::mutations::*;
        pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::standards::v1::subsets::any::io::binary::mutations::change_exaggeration;
        pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::standards::v1::subsets::any::io::binary::mutations::change_imported_features;
    }
}
pub mod sqlite {
    pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::las;
                                pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::ply;
                                pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                                pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::io::export::serializers::artifacts::stl;
                                pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::standards::v1::subsets::any::io::export::serializers::artifacts::gltf;
                                pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::standards::v1::subsets::any::io::export::serializers::artifacts::obj;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub use semio_s_artifact_gis_gisterrain_inferences::diff;
pub use semio_s_artifact_gis_gisterrain_operations::mutations;
pub use semio_s_artifact_gis_gisterrain_inferences::snapshot;
pub use crate::standards::v1::subsets::any::schema::diff::GisTerrainDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::examples::demo::*;

    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis3d {
        pub use semio_s_artifact_gis_gisterrain_editor::editor::gis3d::*;

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::editor::gis3d::presence::*;

            pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::editor::gis3d::presence::schema;
        }

        #[path = "."]
        pub mod examples {
            pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::editor::gis3d::examples::demo_session;

        }

        pub use semio_s_artifact_gis_gisterrain_editor::editor::gis3d::commands;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::editor::gis3d::modes::view::*;

                pub mod windows {
    pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::editor::gis3d::modes::view::windows::*;
    pub mod terrain {
        pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::editor::gis3d::modes::view::windows::terrain::*;
        pub mod config {
            pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::editor::gis3d::modes::view::windows::terrain::config::*;
            pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::editor::gis3d::modes::view::windows::terrain::config::schema;
        }
    }
}
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gisterrain {
        pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::viewer::gisterrain::*;

        pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::viewer::gisterrain::presence;
        pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::viewer::gisterrain::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::viewer::gisterrain::modes::view::*;

                pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::viewer::gisterrain::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{GisterrainBuilderConstruction, GisTerrainParts, GisTerrainAnalyzerAnalysis, GisterrainBuilderFacets, GisterrainBuilder, GisTerrainAnalyzer, GisTerrainComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
