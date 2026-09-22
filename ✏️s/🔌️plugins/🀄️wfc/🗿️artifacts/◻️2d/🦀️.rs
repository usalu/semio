//! ◻️ WFC 2D artifact — an ARBITRARY slot graph in the plane: slots are free rectangles, adjacency is
//! an explicit edge list with named relation classes, and tiles are flat 2D media. It is the
//! non-rectangular sibling of `grid2d` and the 2D twin of `wfc3d`, and it descends directly from the
//! procedural `assembly` artifact (slots/edges/rules/seed), with kit-module handles replaced by
//! inline tile media and a single implicit relation replaced by named relation classes.
//!
//! The SOLVE is never persisted: it is an inference (`…/🧬️schema/💡️inferences/🦀️.rs`) over this
//! document, driven by the shared `semio-s-plugin-wfc-engine` graph route.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::{WFC_2D_DEFAULT_RELATION, WFC_2D_DOCUMENT_SCHEMA};

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ The one `Dialect` coordinate every surface of the `✳️any` subset binds — `s.wfc.wfc2d@1/*`.
/// It lives at the ARTIFACT level (not under `editor`/`viewer`) so a viewer file can read it without
/// importing through the sibling editor module, which `policyViewerPurityBreaches` refuses.
pub const WFC_2D_DIALECT: Dialect = Dialect { artifact_kind: WFC_2D_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗿️ The `2d.wfc2d` artifact kind — a 2D design surface: the document is a plane of rectangles with
/// vector/bitmap media, so `dimension`/`media_type` say `2d`/`Design` rather than assembly's headless
/// `data`/`Value`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.wfc2d".into(),
        name: "2D".into(),
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

//#region 🔖️Declaration
/// 🧾️ The app bounds every declaration function in this crate carries — one editor app and one
/// viewer app, exactly like `semio_s_artifact_puzzle_2d::ArtifactApps`.
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::wfc2d::Wfc2dEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::wfc2d::Wfc2dViewer>>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::wfc2d::Wfc2dEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::wfc2d::Wfc2dViewer>>>
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
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.wfc2d").expect("canonical wfc2d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

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
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                        pub mod create_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🧪️tests/🧩️inserts-slot-d-in-canonical-order/🦀️.rs"]
                            mod tests_inserts_slot_d_in_canonical_order;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/🧪️tests/🚫️removes-slot-a-and-cascades-edge-ab/🦀️.rs"]
                            mod tests_removes_slot_a_and_cascades_edge_ab;
                        }
                        #[path = "."]
                        pub mod move_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-slot/🧪️tests/↔️drags-slot-b-down/🦀️.rs"]
                            mod tests_drags_slot_b_down;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-slot/🧪️tests/📐️widens-slot-b/🦀️.rs"]
                            mod tests_widens_slot_b;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/🧪️tests/🔗️joins-slot-a-to-slot-c/🦀️.rs"]
                            mod tests_joins_slot_a_to_slot_c;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🧪️tests/✂️severs-edge-ab/🦀️.rs"]
                            mod tests_severs_edge_ab;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/🧪️tests/📌️pins-slot-a-to-the-roof-tile/🦀️.rs"]
                            mod tests_pins_slot_a_to_the_roof_tile;
                        }
                        #[path = "."]
                        pub mod unpin_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unpin-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unpin-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unpin-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unpin-slot/🧪️tests/🔓️releases-the-slot-c-pin/🦀️.rs"]
                            mod tests_releases_the_slot_c_pin;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🀄️create-tile/🧪️tests/🀄️adds-the-window-tile/🦀️.rs"]
                            mod tests_adds_the_window_tile;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🧪️tests/🚫️removes-the-wall-tile-and-cascades-rules-and-pins/🦀️.rs"]
                            mod tests_removes_the_wall_tile_and_cascades_rules_and_pins;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-tile-media/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-tile-media/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-tile-media/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-tile-media/🧪️tests/🎨️repaints-the-roof-tile-as-a-raster/🦀️.rs"]
                            mod tests_repaints_the_roof_tile_as_a_raster;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🧪️tests/⛔️forbids-roof-over-roof/🦀️.rs"]
                            mod tests_forbids_roof_over_roof;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/🧪️tests/🚫️removes-the-wall-wall-rule/🦀️.rs"]
                            mod tests_removes_the_wall_wall_rule;
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
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
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
    pub mod wfc2d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🦀️.rs"]
                    pub mod fill;
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wfc2d {
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
/// 📚️ The bundled WFC problems this subset ships — a forced path, a cyclic two-relation lattice, a
/// non-rectangular vector ring, and the same ring over RASTER tiles.
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🚪️two-room-corridor/🦀️.rs"]
    pub mod two_room_corridor;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️wall-roof-facade-strip/🦀️.rs"]
    pub mod wall_roof_facade_strip;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔷️hex-ring/🦀️.rs"]
    pub mod hex_ring;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🗺️terrain-ring/🦀️.rs"]
    pub mod terrain_ring;

    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
    mod tests;
}
//#endregion 📚️Examples

/// 🪞️ Flat access from the artifact root, mirroring the sibling wfc artifacts: the subset's schema
/// tree reached as `crate::schema`, with the shims below doing the same for diff, mutations and
/// inferences.
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
pub use crate::standards::v1::subsets::any::schema::diff::Wfc2dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Wfc2dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Wfc2dSnapshot;

#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "./🧪️tests/🔬️store-fixture/🦀️.rs"]
mod store_fixture;

#[cfg(test)]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;
