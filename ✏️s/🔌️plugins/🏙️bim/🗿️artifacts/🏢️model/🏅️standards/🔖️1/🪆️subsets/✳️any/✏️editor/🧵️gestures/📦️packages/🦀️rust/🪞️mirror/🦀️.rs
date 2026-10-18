//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                        }
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties;
                        #[path = "."]
                        pub mod families {
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;
                        pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules;
                        pub use semio_s_artifact_bim_model_mut_any_174::standards::v1::subsets::any::schema::mutations::set_wall_top;
                        pub use semio_s_artifact_bim_model_mut_any_85::standards::v1::subsets::any::schema::mutations::delete_elements;
                        pub use semio_s_artifact_bim_model_mut_any_157::standards::v1::subsets::any::schema::mutations::create_mep_element;
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
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context;
        pub use semio_s_artifact_bim_model_storey_levels::editor::bim::marks;
        pub use semio_s_artifact_bim_model_kit::editor::bim::kit;
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities;
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        pub use semio_s_artifact_bim_model_kit::editor::bim::utilities;
        #[path = "."]
        pub mod gestures {
            #[path = "../../../🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod overlay {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::overlay::*;
                #[cfg(test)]
                #[path = "../../../🫧️overlay/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::transient;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod family {
                        pub use semio_s_artifact_bim_model_wall_layout::editor::bim::modes::edit::windows::family::vocabulary;
                    }
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod properties {
                #[path = "../../../../📌️panels/🔍️properties/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod families {
                #[path = "../../../../📌️panels/🪑️families/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod classification {
                #[path = "../../../../📌️panels/🗂️classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod delete_selection {
                #[path = "../../../../🎮️commands/🗑️delete-selection/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_commit_draft {
                #[path = "../../../../🎮️commands/✅️canvas-commit-draft/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_escape {
                #[path = "../../../../🎮️commands/🚪️canvas-escape/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod world_pointer_down {
                #[path = "../../../../🎮️commands/🌍️world-pointer-down/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod world_pointer_move {
                #[path = "../../../../🎮️commands/🌐️world-pointer-move/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::attach_walls;
            #[path = "."]
            pub mod cursor_keys {
                #[path = "../../../../🎮️commands/🧭️cursor-keys/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::place_elements;
            #[path = "."]
            pub mod engagement_input {
                #[path = "../../../../🎮️commands/⌨️engagement-input/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod gesture_keys {
                #[path = "../../../../🎮️commands/🔑️gesture-keys/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod engagement_submit {
                #[path = "../../../../🎮️commands/📨️engagement-submit/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
