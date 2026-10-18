//! 🏢️ BIM model artifact: building information as a parametric, event-sourced document.
//!
//! `ModelSnapshot` stores authored parameters only; every derived value (elevations, wall heights, quantities, solids) is a field of
//! `ModelInference`. Mutations are sparse typed diffs with concrete inverses (`ModelMutation`, `ModelDiff`).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_bim_model_v1::{BimApplication, artifact, definition};

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_bim_model_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_bim_model_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::*;

                    pub mod snapshot {
    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::*;
    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::anchors_values;
    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::phases_values;
    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::schedule_kit_values;
    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::views_values;
}
                    pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;

                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::option_scope;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::finishes;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::bodies;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::analytical_members;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;

                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls;
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::fillers;
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::beams;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::stairs;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::railings;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::ramps;
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::element_solids::components;
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::element_solids::mep;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
pub use semio_s_artifact_bim_model_cascade_restore::standards::v1::subsets::any::schema::mutations::cascade_restore;

                        pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry;
                        pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify;
                    }
                }
                pub mod io {
    pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::*;
    pub mod export {
        pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::*;
        pub use semio_s_artifact_bim_model_utilities_tests::standards::v1::subsets::any::io::export::solver;
    }
}
            }
        }
    }
}

#[path = "."]
pub mod examples {

    pub use semio_s_artifact_bim_model_examples::examples::house;
    pub use semio_s_artifact_bim_model_formula::examples::office;
    pub use semio_s_artifact_bim_model_formula::examples::demo;
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        pub use semio_s_artifact_bim_model_bim::editor::bim::*;

        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context;
        pub use semio_s_artifact_bim_model_storey_levels::editor::bim::marks;
        pub use semio_s_artifact_bim_model_kit::editor::bim::kit;
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        pub mod entities {
    pub use semio_s_artifact_bim_model_entities::editor::bim::entities::*;
    pub mod components {
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities::components::*;
        #[cfg(any(test, feature = "testkit"))]
        pub use semio_s_artifact_bim_model_v1::editor::bim::entities::components::tests;
    }
}
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        pub use semio_s_artifact_bim_model_kit::editor::bim::utilities;
        pub use semio_s_artifact_bim_model_gestures::editor::bim::gestures;
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome;
        pub use semio_s_artifact_bim_model_tests::editor::bim::presence;
        pub use semio_s_artifact_bim_model_tests::editor::bim::config;
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::transient;
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_bim_model_tests::editor::bim::modes::*;

            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::*;

                #[path = "."]
                pub mod windows {
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
            pub use semio_s_artifact_bim_model_tests::editor::bim::panels::*;

            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::options;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::properties;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::library;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::families;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::classification;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::diagnostics;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::panels::coordination;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_bim_model_tests::editor::bim::commands::*;

            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_entity;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::create_view;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::delete_selection;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::rename_entity;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_field;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::set_view;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_family;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::edit_schedule;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_schedule_csv;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::analyse_model;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::export_sheets;
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

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bim {
        pub use semio_s_artifact_bim_model_viewer::viewer::bim::*;

        pub use semio_s_artifact_bim_model_family_tests::viewer::bim::modes;
    }
}

pub use semio_s_artifact_bim_model_tests_5::render;
