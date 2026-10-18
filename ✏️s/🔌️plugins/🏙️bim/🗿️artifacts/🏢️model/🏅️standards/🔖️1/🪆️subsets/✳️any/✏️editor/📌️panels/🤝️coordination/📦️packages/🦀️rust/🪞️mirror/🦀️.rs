//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod render {
    pub use semio_s_artifact_bim_model_modify::render::envelope;
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
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;
                        pub use semio_s_artifact_bim_model_mut_any_173::standards::v1::subsets::any::schema::mutations::create_wall;
                        pub use semio_s_artifact_bim_model_mut_any_127::standards::v1::subsets::any::schema::mutations::create_railing;
                        pub use semio_s_artifact_bim_model_mut_any_128::standards::v1::subsets::any::schema::mutations::create_space;
                        pub use semio_s_artifact_bim_model_mut_any_159::standards::v1::subsets::any::schema::mutations::create_column;
                        pub use semio_s_artifact_bim_model_mut_any_160::standards::v1::subsets::any::schema::mutations::create_beam;
                        pub use semio_s_artifact_bim_model_mut_any_175::standards::v1::subsets::any::schema::mutations::create_opening;
                        pub use semio_s_artifact_bim_model_mut_any_131::standards::v1::subsets::any::schema::mutations::create_stair;
                        pub use semio_s_artifact_bim_model_mut_any_132::standards::v1::subsets::any::schema::mutations::create_grid_line;
                        pub use semio_s_artifact_bim_model_mut_any_162::standards::v1::subsets::any::schema::mutations::flip_wall;
                        pub use semio_s_artifact_bim_model_mut_any_133::standards::v1::subsets::any::schema::mutations::move_elements;
                        pub use semio_s_artifact_bim_model_mut_any_134::standards::v1::subsets::any::schema::mutations::rotate_elements;
                        pub use semio_s_artifact_bim_model_mut_any_136::standards::v1::subsets::any::schema::mutations::create_ramp;
                        pub use semio_s_artifact_bim_model_mut_any_137::standards::v1::subsets::any::schema::mutations::create_view;
                        pub use semio_s_artifact_bim_model_mut_any_94::standards::v1::subsets::any::schema::mutations::create_annotation_style;
                        pub use semio_s_artifact_bim_model_mut_any_138::standards::v1::subsets::any::schema::mutations::create_dimension;
                        pub use semio_s_artifact_bim_model_mut_any_139::standards::v1::subsets::any::schema::mutations::create_tag;
                        pub use semio_s_artifact_bim_model_mut_any_140::standards::v1::subsets::any::schema::mutations::create_text_note;
                        pub use semio_s_artifact_bim_model_mut_any_141::standards::v1::subsets::any::schema::mutations::create_leader;
                        pub use semio_s_artifact_bim_model_mut_any_146::standards::v1::subsets::any::schema::mutations::create_viewport;
                        pub use semio_s_artifact_bim_model_mut_any_37::standards::v1::subsets::any::schema::mutations::set_viewport;
                        pub use semio_s_artifact_bim_model_mut_any_172::standards::v1::subsets::any::schema::mutations::create_curtain_wall;
                        pub use semio_s_artifact_bim_model_mut_any_186::standards::v1::subsets::any::schema::mutations::create_wall_sweep;
                        pub use semio_s_artifact_bim_model_mut_any_112::standards::v1::subsets::any::schema::mutations::create_support;
                        pub use semio_s_artifact_bim_model_mut_any_113::standards::v1::subsets::any::schema::mutations::create_load_case;
                        pub use semio_s_artifact_bim_model_mut_any_114::standards::v1::subsets::any::schema::mutations::create_load;
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
        #[path = "."]
        pub mod gestures {
            #[path = "../../../../../🧵️gestures/📏️annotate/🦀️.rs"]
            pub mod annotate;
            #[path = "../../../../../🧵️gestures/🧱️chain/🦀️.rs"]
            pub mod chain;
            #[path = "../../../../../🧵️gestures/🦴️structure/🦀️.rs"]
            pub mod structure;
            #[path = "../../../../../🧵️gestures/🪟️opening/🦀️.rs"]
            pub mod opening;
            pub use semio_s_artifact_bim_model_storey_levels::editor::bim::gestures::plane;
            #[path = "../../../../../🧵️gestures/📍️point/🦀️.rs"]
            pub mod point;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::snap;
            #[path = "../../../../../🧵️gestures/🧷️sweep/🦀️.rs"]
            pub mod sweep;
            #[path = "../../../../../🧵️gestures/🚚️transform/🦀️.rs"]
            pub mod transform;
            #[path = "../../../../../🧵️gestures/⌨️typed/🦀️.rs"]
            pub mod typed;
            #[path = "../../../../../🧵️gestures/🖼️viewports/🦀️.rs"]
            pub mod viewports;
        }
        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context;
        pub use semio_s_artifact_bim_model_storey_levels::editor::bim::marks;
        pub use semio_s_artifact_bim_model_kit::editor::bim::kit;
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities;
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod family {
                        pub use semio_s_artifact_bim_model_viewer::editor::bim::modes::edit::windows::family::edit;
                    }
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section;
                    #[path = "."]
                    pub mod schedule {
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::*;
                        pub use semio_s_artifact_bim_model_ramp_runs::editor::bim::modes::edit::windows::schedule::edit;
                    }
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod outliner {
                pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner::*;
                #[path = "."]
                pub mod views {
                    pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::outliner::views::*;
                    #[cfg(test)]
                    #[path = "../../../../🌳️outliner/🖼️views/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
            #[path = "."]
            pub mod diagnostics {
                pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::diagnostics::*;
                pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::panels::diagnostics::*;
                #[cfg(test)]
                #[path = "../../../../🚨️diagnostics/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod coordination {
                #[path = "../../../🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod create_entity {
                #[path = "../../../../../🎮️commands/🏗️create-entity/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod create_view {
                #[path = "../../../../../🎮️commands/🔭️create-view/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod rename_entity {
                #[path = "../../../../../🎮️commands/🏷️rename-entity/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_field {
                #[path = "../../../../../🎮️commands/🩹️set-field/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_view {
                #[path = "../../../../../🎮️commands/🪟️set-view/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_family {
                #[path = "../../../../../🎮️commands/🧬️edit-family/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_schedule {
                #[path = "../../../../../🎮️commands/📋️edit-schedule/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_camera {
                #[path = "../../../../../🎮️commands/🎥️set-camera/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod arm_utility {
                #[path = "../../../../../🎮️commands/🛠️arm-utility/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod flip_walls {
                #[path = "../../../../../🎮️commands/🔃️flip-walls/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod attach_walls {
                #[path = "../../../../../🎮️commands/🔗️attach-walls/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
