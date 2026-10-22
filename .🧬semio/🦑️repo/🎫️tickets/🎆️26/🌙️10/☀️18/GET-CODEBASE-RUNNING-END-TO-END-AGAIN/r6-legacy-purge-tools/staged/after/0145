//! 🎞️ Animate presentation artifact — document entities + `ArtifactKindSpec` (constitutional: general).

// 🌀️ R7 — `fn` in public traits (`Sobject`, `Animation`) is deliberate under O1's universal-async
// ruling; callers cannot assume `Send` from the lint's suggested fix, and R3 answers that structurally
// (every dyn seam here is a `dyn_enum_close!`-generated enum, so `Send` falls out of the concrete
// variant types). Never resolved by `+ Send` on the trait method or by making the method sync.
#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;

pub use crate::standards::v1::subsets::any::schema::diff::PresentationDiff;

pub use semio_s_artifact_animate_presentation_inferences::{PRESENTATION_DOCUMENT_SCHEMA};
pub use crate::snapshot::schema::{default_snapshot, demo_snapshot, PresentationSnapshot};

pub use semio_s_artifact_animate_presentation_mutations::{ANIMATE_DIALECT};

pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraft, FigureTileFrame, FigureTileSource};

pub use semio_s_artifact_animate_presentation_inferences::{DEMO_FIGURE_SRC};

pub use semio_s_artifact_animate_presentation_inferences::{DEMO_FIGURE_PIXEL_WIDTH};
pub use semio_s_artifact_animate_presentation_inferences::{DEMO_FIGURE_PIXEL_HEIGHT};

pub use semio_s_artifact_animate_presentation_inferences::{default_figure_tile_source};

pub use semio_s_artifact_animate_presentation_tests::{default_presentation_snapshot};

pub use semio_s_artifact_animate_presentation_host::{demo_presentation_snapshot};

pub use semio_s_artifact_animate_presentation_inferences::{AnimationChild, PresentationChild, animation_child_handle, presentation_child_handle, presentation_snapshot_from_source_tiles, source_tiles_from_presentation_snapshot};

pub use semio_s_artifact_animate_presentation_inferences::{PRESENTATION_CANVAS_FIT_EXTENT, PRESENTATION_CANVAS_SCALE, genesis_presentation_child_pack, presentation_canvas_camera, presentation_snapshot_with_tiles, presentation_working_scene};

pub use semio_s_artifact_animate_presentation_animate::{artifact_kind};

pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraftPatch};

//#region 🧪️Tests
#[cfg(test)]
#[allow(clippy::items_after_test_module)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.animate.presentation")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.animate.presentation")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.animate.presentation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.animate.presentation.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.animate.presentation.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.animate.presentation@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.animate.presentation@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.pdf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pdf@1.4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pdf@1.4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"animate.presentation:presentation")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "animate.presentation")?)?
                .claim(ArtifactIdentityClaim::codec_extension("animate.presentation", "presentation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Animate Presentation")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Animate Presentation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Animate Presentation")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Animate Presentation")?)?,
        )
}

/// 🗿️ New declaration-tree root (design.md §1/§2 recipe step 6) — replaces the OLD `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).document_codec(...)`
/// chain) outright, no dual channel (mirrors `🎬️sequence`'s identical atomic cutover). `kind` matches
/// `ANIMATE_DIALECT.artifact_kind` / `PresentationSnapshot`'s own `#[artifact_schema(id = ...)]`
/// (`"s.animate.presentation"`), NOT `definition()`'s legacy `ArtifactIdentity` root (`"s.presentation"`,
/// kept unread by the new tree per debt D1). `localization: &[]` is a documented shortfall — the
/// real en/de localized names still live on `definition()`'s kept capability rows.
pub fn artifact<PA>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA>
where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::animate::AnimatePresentationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::animate::AnimatePresentationViewer>, semio_s_artifact_stdio_semio::SemioMembers>>,
{
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.animate.presentation").expect("canonical animate.presentation kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}
//#endregion 🔖️Declaration

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_animate_presentation_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_animate_presentation_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_animate_presentation_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_animate_presentation_tile_grid::standards::v1::subsets::any::schema::tile_grid;
                    pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_animate_presentation_mutations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod resize_source_frame {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_source_frame::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_source_frame::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_source_frame::inverse;

                        }
                        #[path = "."]
                        pub mod replace_source {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::replace_source::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::replace_source::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::replace_source::inverse;

                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::inverse;

                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;

                        }
                        #[path = "."]
                        pub mod delete_tiles {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tiles::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tiles::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tiles::inverse;

                        }
                        #[path = "."]
                        pub mod rename_tile {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::rename_tile::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::rename_tile::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::rename_tile::inverse;

                        }
                        #[path = "."]
                        pub mod resize_tile_crop {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_tile_crop::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_tile_crop::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_tile_crop::inverse;

                        }
                        #[path = "."]
                        pub mod reorder_tiles {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::reorder_tiles::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::reorder_tiles::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::reorder_tiles::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_animate_presentation_mutations::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_animate_presentation_tile_grid::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_animate_presentation_examples::standards::v1::subsets::any::io::binary::inferences;
    pub use semio_s_artifact_animate_presentation_mutations::standards::v1::subsets::any::io::binary::mutations;
    pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::io::binary::snapshot;
}
pub mod sqlite {
    pub use semio_s_artifact_animate_presentation_examples::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf;
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::export::serializers::artifacts::pdf;
                                pub use semio_s_artifact_animate_presentation_tests::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----

pub use semio_s_artifact_animate_presentation_inferences::diff;
pub use semio_s_artifact_animate_presentation_mutations::mutations;
pub use semio_s_artifact_animate_presentation_inferences::snapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_animate_presentation_examples::examples::demo::*;

        pub use semio_s_artifact_animate_presentation_tests::examples::demo::tests;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        pub use semio_s_artifact_animate_presentation_animate::editor::animate::*;
pub use semio_s_artifact_animate_presentation_examples::editor::animate::demo_session;

        #[path = "."]
        pub mod engine {
            //! ⚙️ This app's own stateful behaviour (compiler/🎞️slide/video-export at the root, plus the
            //! Manim-class animation core and headless video renderer as sibling `<topic>/🦀️.rs`
            //! files) — a non-taxonomy, editor-only facet (ticket
            //! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET packet recipe step 4: only editor-side
            //! files reference it, so it moved wholesale into `✏️editor/⚙️engine/`).
            pub use semio_s_artifact_animate_presentation_mutations::editor::animate::engine::*;

            pub use semio_s_artifact_animate_presentation_animation::editor::animate::engine::animation;
            pub use semio_s_artifact_animate_presentation_inferences::editor::animate::engine::camera;
            pub use semio_s_artifact_animate_presentation_inferences::editor::animate::engine::config;
            pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::geometry;
            pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::rate;
            pub use semio_s_artifact_animate_presentation_mut_family::editor::animate::engine::scene;
            pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::text;
            pub use semio_s_artifact_animate_presentation_video::editor::animate::engine::video;
        }

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::config::*;
pub mod io {
    pub use semio_s_artifact_animate_presentation_examples::editor::animate::config::io::*;
    pub mod binary {
        pub use semio_s_artifact_animate_presentation_viewer::editor::animate::config::io::binary::*;
        pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::config::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_animate_presentation_viewer::editor::animate::config::io::text::*;
        pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::config::io::text::mutations;
    }
}

            pub use semio_s_artifact_animate_presentation_viewer::editor::animate::config::schema;
        }

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_animate_presentation_viewer::editor::animate::presence::*;
pub mod io {
    pub use semio_s_artifact_animate_presentation_examples::editor::animate::presence::io::*;
    pub mod binary {
        pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::presence::io::binary::*;
        pub use semio_s_artifact_animate_presentation_viewer::editor::animate::presence::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::presence::io::text::*;
        pub use semio_s_artifact_animate_presentation_viewer::editor::animate::presence::io::text::mutations;
    }
}

            pub use semio_s_artifact_animate_presentation_viewer::editor::animate::presence::schema;
        }
        pub use semio_s_artifact_animate_presentation_animate::editor::animate::terminology;
        pub use semio_s_artifact_animate_presentation_mutations::editor::animate::support;
        pub use semio_s_artifact_animate_presentation_viewer::editor::animate::wasm;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_animate_presentation_mutations::editor::animate::commands::add_tile;
            pub use semio_s_artifact_animate_presentation_mutations::editor::animate::commands::canvas_pointer_down;
            pub use semio_s_artifact_animate_presentation_mutations::editor::animate::commands::clear_tiles;
            pub use semio_s_artifact_animate_presentation_schema::editor::animate::commands::copy_prompt;
            pub use semio_s_artifact_animate_presentation_schema::editor::animate::commands::delete_selection;
            pub use semio_s_artifact_animate_presentation_schema::editor::animate::commands::delete_tile;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::engagement_input;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::engagement_submit;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::export_video_from_deck;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::no_operation;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::patch_tile_crops;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::rename_tiles;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::reset_grid;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::seed_grid;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::set_active_example;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::set_frame;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::set_source;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                pub use semio_s_artifact_animate_presentation_tests::editor::animate::modes::main::*;

                pub use semio_s_artifact_animate_presentation_tests::editor::animate::modes::main::windows;
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_animate_presentation_animate::editor::animate::panels::artifact;
            pub use semio_s_artifact_animate_presentation_animate::editor::animate::panels::catalogue;
            pub use semio_s_artifact_animate_presentation_animate::editor::animate::panels::inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod animate {
        pub use semio_s_artifact_animate_presentation_mutations::viewer::animate::*;

        pub use semio_s_artifact_animate_presentation_viewer::viewer::animate::presence;
        pub use semio_s_artifact_animate_presentation_viewer::viewer::animate::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_animate_presentation_tests::viewer::animate::modes::view::*;

                pub use semio_s_artifact_animate_presentation_tests::viewer::animate::modes::view::windows;
            }
        }
    }
}

pub use semio_s_artifact_animate_presentation_schema::host;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
