//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod render {
    pub use semio_s_artifact_bim_model_plan_linework::render::plan;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc;
                        #[path = "."]
                        pub mod gltf {
                            pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::io::export::gltf::*;
                            pub use semio_s_artifact_bim_model_family_tests::standards::v1::subsets::any::io::export::gltf::scene;
                        }
                        pub use semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg;
                        #[path = "."]
                        pub mod csv {
                            pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::csv::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::csv::codec;
                        }
                        pub use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                        }
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context;
        pub use semio_s_artifact_bim_model_storey_levels::editor::bim::marks;
        pub use semio_s_artifact_bim_model_kit::editor::bim::kit;
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities;
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        pub use semio_s_artifact_bim_model_kit::editor::bim::utilities;
        #[path = "."]
        pub mod gestures {
            pub use semio_s_artifact_bim_model_gestures::editor::bim::gestures::*;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::overlay;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session;
            #[path = "."]
            pub mod split {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::split::*;
                #[cfg(test)]
                #[path = "../../../🧵️gestures/🔪️split/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome;
        pub use semio_s_artifact_bim_model_tests::editor::bim::presence;
        pub use semio_s_artifact_bim_model_tests::editor::bim::config;
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::transient;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::*;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule;
                    pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::options;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner;
            #[path = "."]
            pub mod properties {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::properties::*;
                #[path = "."]
                pub mod data {
                    pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::properties::data::*;
                    #[cfg(test)]
                    #[path = "../../../📌️panels/🔍️properties/🏷️data/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::library;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::families;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::classification;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::diagnostics;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_entity;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_view;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::delete_selection;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::rename_entity;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_field;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_view;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_family;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_schedule;
            #[path = "."]
            pub mod export_schedule_csv {
                #[path = "../../../🎮️commands/📊️export-schedule-csv/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod analyse_model {
                #[path = "../../../🎮️commands/🔎️analyse-model/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod export_model {
                #[path = "../../../🎮️commands/📤️export-model/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod export_sheets {
                #[path = "../../../🎮️commands/📄️export-sheets/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_camera;
            #[path = "."]
            pub mod canvas_pointer_down {
                #[path = "../../../🎮️commands/🖱️canvas-pointer-down/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_pointer_move {
                #[path = "../../../🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_pointer_up {
                #[path = "../../../🎮️commands/⬆️canvas-pointer-up/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_double_click {
                #[path = "../../../🎮️commands/👆️canvas-double-click/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::canvas_commit_draft;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::canvas_escape;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_down;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_move;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::arm_utility;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::flip_walls;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::attach_walls;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::move_storey;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::split_wall;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_property;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::remove_property;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::select_findings;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::coordinate;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_classification;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::remove_classification;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::apply_template;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::conditions;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::edit_template;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::edit_classification;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::search_classification;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::cursor_keys;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::place_elements;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::place_grid_columns;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_input;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::browse_families;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_override;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::gesture_keys;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_submit;
        }
    }
}
