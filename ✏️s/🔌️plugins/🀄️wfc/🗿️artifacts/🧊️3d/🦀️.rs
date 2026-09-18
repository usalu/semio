//! 🧊️ WFC 3d artifact — an ARBITRARY slot graph in space (`s.wfc.wfc3d`): slots are boxes placed
//! freely at `x`/`y`/`z`, adjacency is an explicit edge list carrying a named `relation`, and the
//! placeable catalogue is mesh tiles. No grid assumption anywhere; the regular-box sibling is
//! `🧱️grid3d`. The SOLVE is an inference over this spec (`🧬️schema/💡️inferences/🦀️.rs`), never
//! persisted state — the editor's preview window renders the inferred assignment and the document
//! keeps only the problem.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::WFC3D_DOCUMENT_SCHEMA;

use crate::schema::snapshot::{Color, TileMedia3d};
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ The canonical surface-id coordinate for this artifact's ONE subset (`✳️any`) —
/// `s.wfc.wfc3d@1/*`. Lives at the ARTIFACT root (not under `✏️editor`/`👁️viewer`) so a viewer file
/// can read it without importing through the sibling editor module. `artifact_kind` matches
/// `Wfc3dSnapshot`/`Wfc3dDiff`'s own `#[artifact_schema(id = "s.wfc.wfc3d")]` attribute and
/// `WFC3D_DOCUMENT_SCHEMA`'s literal value; it is NOT `artifact_kind()`'s OS-level `"3d.wfc3d"`
/// kind id, which is a different, unrelated namespace.
pub const WFC3D_DIALECT: Dialect = Dialect { artifact_kind: WFC3D_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`. `dimension`/`media_*` follow the 3d/mesh family: the
/// editor's preview window is a `World3d` surface placing one mesh instance per solved slot.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.wfc3d".into(),
        name: "3D".into(),
        source_format: WFC3D_DOCUMENT_SCHEMA.into(),
        component_kind: "wfc3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: WFC3D_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.txt".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🥽️MeshChild
/// 🥽️ Addresses one tile's geometry as this document's child in the `s.stdio.semio@v1/mesh` store —
/// the `TileMedia3d::MeshChild` variant's handle. A tile catalogue small enough to author inline
/// uses `TileMedia3d::Mesh` instead and mints no handle at all.
pub fn mesh_child_handle(tile_id: &str) -> store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot> {
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    store::ArtifactChild::new(tile_id.to_string(), store::os_io::ArtifactRef { artifact_id: tile_id.to_string(), dialect })
}
/// 📦️ An axis-aligned unit box in tile space (`0..1` on every axis), as inline media. Twelve
/// triangles over eight corners — the honest default body for a tile that fills its slot's box, and
/// what the preview falls back to when a tile's media resolves to nothing.
pub fn unit_box_media(color: Option<Color>) -> TileMedia3d {
    TileMedia3d::Mesh {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0],
        indices: vec![0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5],
        color,
    }
}

/// 🔺️ A unit wedge in tile space — the box with its `+y`/`+z` edge collapsed, so a roof/ramp tile
/// reads differently from a solid bay at a glance.
pub fn unit_wedge_media(color: Option<Color>) -> TileMedia3d {
    TileMedia3d::Mesh {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0],
        indices: vec![0, 2, 1, 0, 3, 2, 0, 1, 5, 0, 5, 4, 3, 0, 4, 1, 2, 5, 2, 3, 4, 2, 4, 5],
        color,
    }
}
//#endregion 🥽️MeshChild

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
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::wfc3d::Wfc3dEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::wfc3d::Wfc3dViewer>>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::wfc3d::Wfc3dEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::wfc3d::Wfc3dViewer>>>
{
}

/// 🌳️ This artifact's declaration tree root — the ONLY registration channel for schema/io/viewer/
/// editor rows.
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.wfc3d").expect("canonical wfc3d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) — built once and leaked to a
/// `&'static` slice since `dsl::passthrough_hooks` is not `const fn`. Index order is fixed:
/// 0 = document, 1 = op, 2 = diff, 3 = pack, 4 = spr.
pub fn wfc3d_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "wfc3d.snapshot",
                    extension: Some("wfc3d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc3d.snapshot"),
                },
                dsl::LanguageSpec {
                    id: "wfc3d.mutations",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc3d.mutations"),
                },
                dsl::LanguageSpec {
                    id: "wfc3d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("wfc3d.diff"),
                },
                dsl::LanguageSpec {
                    id: "wfc3d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc3d.pack"),
                },
                dsl::LanguageSpec {
                    id: "wfc3d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc3d.spr"),
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
                        pub mod create_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🧪️tests/🧩️inserts-room-c-at-the-sorted-position/🦀️.rs"]
                            mod tests_inserts_room_c_at_the_sorted_position;
                        }
                        #[path = "."]
                        pub mod delete_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/🧪️tests/🕳️removes-room-a-and-cascades-its-edge/🦀️.rs"]
                            mod tests_removes_room_a_and_cascades_its_edge;
                        }
                        #[path = "."]
                        pub mod move_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-slot/🧪️tests/🚚️lifts-room-b-one-storey/🦀️.rs"]
                            mod tests_lifts_room_b_one_storey;
                        }
                        #[path = "."]
                        pub mod resize_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-slot/🧪️tests/📐️widens-room-a-to-a-double-bay/🦀️.rs"]
                            mod tests_widens_room_a_to_a_double_bay;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/🧪️tests/🔗️joins-room-a-to-room-b-beside/🦀️.rs"]
                            mod tests_joins_room_a_to_room_b_beside;
                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🧪️tests/✂️severs-the-corridor-b-edge/🦀️.rs"]
                            mod tests_severs_the_corridor_b_edge;
                        }
                        #[path = "."]
                        pub mod pin_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/🧪️tests/📌️pins-room-a-to-the-room-tile/🦀️.rs"]
                            mod tests_pins_room_a_to_the_room_tile;
                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-slot/🧪️tests/📍️releases-the-pin-on-room-a/🦀️.rs"]
                            mod tests_releases_the_pin_on_room_a;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🀄️create-tile/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🀄️create-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🀄️create-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🀄️create-tile/🧪️tests/🀄️adds-a-stair-tile-to-the-catalogue/🦀️.rs"]
                            mod tests_adds_a_stair_tile_to_the_catalogue;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🧪️tests/🗑️drops-the-corridor-tile-and-its-rule/🦀️.rs"]
                            mod tests_drops_the_corridor_tile_and_its_rule;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/🧪️tests/⚖️raises-the-room-tile-selection-bias/🦀️.rs"]
                            mod tests_raises_the_room_tile_selection_bias;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-tile-media/🧪️tests/🖼️swaps-the-corridor-box-for-a-wedge/🦀️.rs"]
                            mod tests_swaps_the_corridor_box_for_a_wedge;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🧪️tests/🚦️forbids-two-rooms-side-by-side/🦀️.rs"]
                            mod tests_forbids_two_rooms_side_by_side;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-rule/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫️delete-rule/🧪️tests/🚫️drops-the-room-corridor-pairing/🦀️.rs"]
                            mod tests_drops_the_room_corridor_pairing;
                        }
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
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
        pub mod config;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs"]
        pub mod transient;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs"]
                    pub mod graph;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️preview/🦀️.rs"]
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
    pub mod wfc3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}

//#region 📚️Examples
/// 📚️ The bundled problem specs this subset ships — a forced corridor path, a cyclic facade strip,
/// and a non-boxed tower. Each slug owns its own directory under `📚️examples/`, and the Rust builder
/// there is the authority its `🗣️.dsl.semio` asset is printed from.
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🚪️two-room-corridor/🦀️.rs"]
    pub mod two_room_corridor;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️wall-roof-facade-strip/🦀️.rs"]
    pub mod wall_roof_facade_strip;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🗼️tower-stack/🦀️.rs"]
    pub mod tower_stack;

    /// 📇️ Every bundled example, in the order the editor's example picker offers them.
    pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![two_room_corridor::source(), wall_roof_facade_strip::source(), tower_stack::source()]
    }

    /// 📇️ The same roster as a `&'static` slice, for the subset declaration.
    pub fn example_source_slice() -> &'static [semio_framework_plugin::ExampleSource] {
        static EXAMPLES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
        EXAMPLES.get_or_init(sources).as_slice()
    }

    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
    mod tests;
}
//#endregion 📚️Examples

//#region 🔗️FlatShims
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
pub mod io {
    pub use crate::standards::v1::subsets::any::io::*;
}
pub use crate::standards::v1::subsets::any::schema::diff::Wfc3dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Wfc3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Wfc3dSnapshot;

//#endregion 🔗️FlatShims

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;

#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🗄️store-fixture/🦀️.rs"]
mod store_fixture;
//#endregion 🧪️Tests
