//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::*;
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::finishes;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules;
                        pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify;
                        pub use semio_s_artifact_bim_model_mut_any_179::standards::v1::subsets::any::schema::mutations::copy_elements;
                        pub use semio_s_artifact_bim_model_mut_any_180::standards::v1::subsets::any::schema::mutations::array_elements;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[cfg(test)]
    #[path = "../../../🪆️subsets/✳️any/📚️examples/🧰️checks/🦀️.rs"]
    pub mod checks;
    #[path = "."]
    pub mod house {
        pub use semio_s_artifact_bim_model_examples::examples::house::*;
        pub use semio_s_artifact_bim_model_examples::examples::house::*;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/📚️examples/🏡️house/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod office {
        pub use semio_s_artifact_bim_model_formula::examples::office::*;
        pub use semio_s_artifact_bim_model_formula::examples::office::*;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/📚️examples/🏢️office/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_bim_model_formula::examples::demo::*;
        pub use semio_s_artifact_bim_model_formula::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        pub use semio_s_artifact_bim_model_bim::editor::bim::*;
        #[path = "."]
        pub mod gestures {
            pub use semio_s_artifact_bim_model_storey_levels::editor::bim::gestures::plane;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session;
        }
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context;
        pub use semio_s_artifact_bim_model_storey_levels::editor::bim::marks;
        pub use semio_s_artifact_bim_model_kit::editor::bim::kit;
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        #[path = "."]
        pub mod entities {
            pub use semio_s_artifact_bim_model_entities::editor::bim::entities::*;
            #[path = "."]
            pub mod energy {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::energy::*;
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🧩️entities/🌡️energy/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod mep {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::mep::*;
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🧩️entities/🌀️mep/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod components {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::components::*;
                #[cfg(any(test, feature = "testkit"))]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🧩️entities/🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        pub use semio_s_artifact_bim_model_kit::editor::bim::utilities;
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod unit_tests;
        #[cfg(any(test, feature = "testkit"))]
        pub use semio_s_artifact_bim_model_bim::editor::bim::fixture;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧪️tests/⌨️completeness/🦀️.rs"]
        pub mod completeness_tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule;
                    pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::properties;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::library;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::diagnostics;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::*;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_entity;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_view;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::delete_selection;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::rename_entity;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_field;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_view;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::analyse_model;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_camera;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_pointer_down;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_pointer_move;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_pointer_up;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_double_click;
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
            #[path = "."]
            pub mod select_findings {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::select_findings::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::select_findings::*;
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️select-findings/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
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
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_submit;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bim {
        pub use semio_s_artifact_bim_model_viewer::viewer::bim::*;
    }
}
