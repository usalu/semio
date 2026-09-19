//! 🧱️ WFC 3D grid artifact — `s.wfc.grid3d`: a regular box grid with NON-UNIFORM per-axis cell
//! sizes, six fixed face neighbours, mesh tiles and a closed adjacency allow-list. The document
//! persists only the PROBLEM (extent, cell sizes, periodicity, tiles, rules, pins, masks); the
//! SOLVE is an inference (`…/🧬️schema/💡️inferences/🦀️.rs`) driving the shared
//! `semio_s_plugin_wfc_engine` `WfcJob` over a `Grid3dTopology`, republished into the editor's
//! preview window and never written back into the snapshot.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::WFC_GRID3D_DOCUMENT_SCHEMA;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ The canonical surface-id coordinate for this artifact's ONE subset (`✳️any`) —
/// `s.wfc.grid3d@1/*`. Lives at the ARTIFACT root (not under `✏️editor`/`👁️viewer`) so a viewer file
/// reads it without importing through the sibling editor module (`policyViewerPurityBreaches`).
/// NOT [`artifact_kind`]'s OS-level `"3d.wfcgrid3d"` kind id, which is a different namespace.
pub const WFC_GRID3D_DIALECT: Dialect = Dialect { artifact_kind: WFC_GRID3D_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — a 3D document: its tiles are meshes and its preview is a
/// real instanced `World3d` scene, so `dimension`/`media_class`/`media_form` follow the
/// `puzzle.3d`/`remodeling` mesh precedent rather than assembly's headless `data` one.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.wfcgrid3d".into(),
        name: "3D Grid".into(),
        source_format: WFC_GRID3D_DOCUMENT_SCHEMA.into(),
        component_kind: "wfcgrid3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: WFC_GRID3D_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.txt".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧊️MeshChild
/// 🧊️ Addresses one tile mesh as this document's child in the `s.stdio.semio@v1/mesh` store — the
/// one place the dialect is spelled, so every author (examples, editor affordances, importers) mints
/// the handle here rather than restating it. `child_id` IS the mesh artifact id.
pub fn mesh_child_handle(mesh_id: &str) -> store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot> {
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    store::ArtifactChild::new(mesh_id.to_string(), store::os_io::ArtifactRef { artifact_id: mesh_id.to_string(), dialect })
}
//#endregion 🧊️MeshChild

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
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::grid3d::Grid3dEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::grid3d::Grid3dViewer>>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::grid3d::Grid3dEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::grid3d::Grid3dViewer>>>
{
}

/// 🌳️ This artifact's declaration tree root — the ONLY registration channel for schema/io/viewer/
/// editor rows, mirroring `semio_s_artifact_puzzle_2d::artifact`.
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.grid3d").expect("canonical wfc grid3d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`.
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "wfc.grid3d",
                    extension: Some("wfcgrid3d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.grid3d"),
                },
                dsl::LanguageSpec {
                    id: "wfc.grid3d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.grid3d.op"),
                },
                dsl::LanguageSpec {
                    id: "wfc.grid3d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.grid3d.pack"),
                },
                dsl::LanguageSpec {
                    id: "wfc.grid3d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.grid3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[cfg(feature = "component-app-assembly")]
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        #[cfg(feature = "component-app-assembly")]
        pub use component::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[cfg(feature = "component-app-assembly")]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                #[cfg(feature = "component-app-assembly")]
                pub use component::*;

                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                    pub mod diff;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌐️scene-internals/🦀️.rs"]
                    pub mod scene_internals;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod change_seed {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds-the-solve-from-7-to-99/🦀️.rs"]
                            mod tests_reseeds_the_solve_from_7_to_99;
                        }
                        #[path = "."]
                        pub mod resize_grid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-grid/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-grid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-grid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-grid/🧪️tests/📐️grows-the-grid-to-3x2x2/🦀️.rs"]
                            mod tests_grows_the_grid_to_3x2x2;
                        }
                        #[path = "."]
                        pub mod change_cell_sizes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-cell-sizes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-cell-sizes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-cell-sizes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-cell-sizes/🧪️tests/📏️stretches-the-x-axis-columns/🦀️.rs"]
                            mod tests_stretches_the_x_axis_columns;
                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-periodicity/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-periodicity/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-periodicity/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️change-periodicity/🧪️tests/🔁️wraps-the-x-axis/🦀️.rs"]
                            mod tests_wraps_the_x_axis;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-tile/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-tile/🧪️tests/🧱️adds-the-roof-tile/🦀️.rs"]
                            mod tests_adds_the_roof_tile;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-tile/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-tile/🧪️tests/🕳️removes-the-air-tile-and-cascades/🦀️.rs"]
                            mod tests_removes_the_air_tile_and_cascades;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/🧪️tests/⚖️raises-the-wall-tile-bias/🦀️.rs"]
                            mod tests_raises_the_wall_tile_bias;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-tile-media/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-tile-media/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-tile-media/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-tile-media/🧪️tests/🖼️replaces-the-wall-tile-mesh/🦀️.rs"]
                            mod tests_replaces_the_wall_tile_mesh;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🧪️tests/🚦️allows-air-above-air/🦀️.rs"]
                            mod tests_allows_air_above_air;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-rule/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-rule/🧪️tests/❌️removes-the-floor-wall-rule/🦀️.rs"]
                            mod tests_removes_the_floor_wall_rule;
                        }
                        #[path = "."]
                        pub mod pin_cell {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-cell/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-cell/🧪️tests/📌️pins-the-far-cell-to-wall/🦀️.rs"]
                            mod tests_pins_the_far_cell_to_wall;
                        }
                        #[path = "."]
                        pub mod unpin_cell {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-cell/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-cell/🧪️tests/📍️releases-the-origin-cell/🦀️.rs"]
                            mod tests_releases_the_origin_cell;
                        }
                        #[path = "."]
                        pub mod mask_cell {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️mask-cell/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️mask-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️mask-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️mask-cell/🧪️tests/🚫️carves-out-the-far-edge-cell/🦀️.rs"]
                            mod tests_carves_out_the_far_edge_cell;
                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unmask-cell/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unmask-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unmask-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unmask-cell/🧪️tests/🔓️restores-the-masked-corner/🦀️.rs"]
                            mod tests_restores_the_masked_corner;
                        }
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }

                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod grid3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"]
        pub mod window;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}

//#region 📚️Examples
/// 📚️ The bundled tile sets this subset ships — one architectural block set on a non-uniform
/// 4×3×4 grid, one pipe network. Each slug owns its own directory under
/// `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`, and the Rust builder there is the authority its
/// `🗣️.dsl.semio` asset is printed from.
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️blocks/🦀️.rs"]
    pub mod blocks;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪠️pipes-3d/🦀️.rs"]
    pub mod pipes_3d;

    /// 📇️ Every bundled example, in the order the editor's example picker offers them. Append-only:
    /// a new example is added at the END so a stored picker index never re-points.
    pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![blocks::source(), pipes_3d::source()]
    }

    /// 📇️ The same roster as a leaked `&'static` slice, for the subset declaration.
    pub fn example_source_slice() -> &'static [semio_framework_plugin::ExampleSource] {
        static EXAMPLES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
        EXAMPLES.get_or_init(sources).as_slice()
    }

    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
    mod tests;
}
//#endregion 📚️Examples

/// 🪞️ Flat access from the artifact root, mirroring generation3d: the subset's schema tree reached as
/// `crate::schema`, with the sibling shims below doing the same for diff, mutations and inferences.
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
}
pub use crate::standards::v1::subsets::any::schema::diff::Grid3dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Grid3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Grid3dSnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;

#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "🧪️tests/🔬️store-fixture/🦀️.rs"]
mod store_fixture;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-wfc-grid3d-1/🦀️.rs"]
mod mutate_wfc_grid3d_1;
//#endregion 🧪️Tests
