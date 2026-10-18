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
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                        }
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        #[path = "."]
                        pub mod families {
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues;
                        }
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::placement;
                        pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify;
                        pub use semio_s_artifact_bim_model_mut_any_2::standards::v1::subsets::any::schema::mutations::set_storey_height;
                        pub use semio_s_artifact_bim_model_mut_any_175::standards::v1::subsets::any::schema::mutations::create_opening;
                        pub use semio_s_artifact_bim_model_mut_any_186::standards::v1::subsets::any::schema::mutations::create_wall_sweep;
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
        pub use semio_s_artifact_bim_model_bim::editor::bim::*;
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
            pub use semio_s_artifact_bim_model_gestures::editor::bim::gestures::*;
            #[path = "."]
            pub mod annotate {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::annotate::*;
                #[cfg(test)]
                #[path = "../../../../../📏️annotate/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod area {
                pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::area::*;
                #[cfg(test)]
                #[path = "../../../../../🧩️area/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod chain {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::chain::*;
                #[cfg(test)]
                #[path = "../../../../../🧱️chain/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod frame {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::frame::*;
                #[cfg(test)]
                #[path = "../../../../../🏗️frame/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod opening {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::opening::*;
                #[cfg(test)]
                #[path = "../../../../../🪟️opening/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::overlay;
            pub use semio_s_artifact_bim_model_storey_levels::editor::bim::gestures::plane;
            #[path = "."]
            pub mod point {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::point::*;
                #[cfg(test)]
                #[path = "../../../../../📍️point/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod select {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::select::*;
                #[cfg(test)]
                #[path = "../../../../../🎯️select/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod session {
                pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::*;
                #[cfg(test)]
                #[path = "../../../../../🧭️session/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod snap {
                pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::snap::*;
                #[cfg(test)]
                #[path = "../../../../../🧲️snap/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod sweep {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::sweep::*;
                #[cfg(test)]
                #[path = "../../../../../🧷️sweep/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod transform {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::transform::*;
                #[cfg(test)]
                #[path = "../../../../../🚚️transform/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod duplicate {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::duplicate::*;
                #[cfg(test)]
                #[path = "../../../../../👯️duplicate/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod reshape {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::reshape::*;
                #[cfg(test)]
                #[path = "../../../../../✂️reshape/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod typed {
                pub use semio_s_artifact_bim_model_coordination::editor::bim::gestures::typed::*;
                #[cfg(test)]
                #[path = "../../../../../⌨️typed/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod place {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::place::*;
                #[cfg(test)]
                #[path = "../../../../../🪑️place/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod route {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::gestures::route::*;
                #[cfg(test)]
                #[path = "../../../../../🌀️route/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[cfg(test)]
            #[path = "../../../🦀️.rs"]
            pub mod tests;
            #[cfg(test)]
            #[path = "../../../../🧷️app/🦀️.rs"]
            pub mod app_tests;
        }
        #[path = "."]
        pub mod chrome {
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome::*;
            pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::chrome::*;
            #[cfg(test)]
            #[path = "../../../../../../🎛️chrome/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[cfg(any(test, feature = "testkit"))]
        pub use semio_s_artifact_bim_model_bim::editor::bim::fixture;
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
                    #[path = "."]
                    pub mod family {
                        pub use semio_s_artifact_bim_model_dispatch_context::editor::bim::modes::edit::windows::family::*;
                        pub use semio_s_artifact_bim_model_wall_layout::editor::bim::modes::edit::windows::family::vocabulary;
                    }
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod properties {
                pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::properties::*;
                #[path = "."]
                pub mod overrides {
                    pub use semio_s_artifact_bim_model_gestures::editor::bim::panels::properties::overrides::*;
                    #[cfg(test)]
                    #[path = "../../../../../../📌️panels/🔍️properties/🎚️overrides/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::*;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_pointer_down;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_pointer_move;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_pointer_up;
            pub use semio_s_artifact_bim_model_bim::editor::bim::commands::canvas_double_click;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::canvas_escape;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_down;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::world_pointer_move;
            pub use semio_s_artifact_bim_model_coordination::editor::bim::commands::arm_utility;
            pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::set_property;
            #[path = "."]
            pub mod remove_property {
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::remove_property::*;
                pub use semio_s_artifact_bim_model_move_storey::editor::bim::commands::remove_property::*;
                #[cfg(test)]
                #[path = "../../../../../../🎮️commands/🗃️remove-property/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::cursor_keys;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_input;
            pub use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_submit;
        }
    }
}
