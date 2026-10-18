//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod render {
    pub use semio_s_artifact_bim_model_tests_5::render::world;
    pub use semio_s_artifact_bim_model_plan_linework::render::plan;
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
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod sheets {
                            pub use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::*;
                            pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::sheets::ink;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
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
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
                        #[path = "."]
                        pub mod families {
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues;
                        }
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules;
                        pub use semio_s_artifact_bim_model_mut_any_129::standards::v1::subsets::any::schema::mutations::create_slab;
                        pub use semio_s_artifact_bim_model_mut_any_130::standards::v1::subsets::any::schema::mutations::create_roof;
                        pub use semio_s_artifact_bim_model_mut_any_135::standards::v1::subsets::any::schema::mutations::create_ceiling;
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
            #[path = "../../../../🧵️gestures/🧩️area/🦀️.rs"]
            pub mod area;
            pub use semio_s_artifact_bim_model_storey_levels::editor::bim::gestures::plane;
            #[path = "../../../../🧵️gestures/🧭️session/🦀️.rs"]
            pub mod session;
            #[path = "../../../../🧵️gestures/🧲️snap/🦀️.rs"]
            pub mod snap;
        }
        #[path = "."]
        pub mod dispatch_context {
            #[path = "../../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
        pub use semio_s_artifact_bim_model_storey_levels::editor::bim::marks;
        pub use semio_s_artifact_bim_model_kit::editor::bim::kit;
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        pub use semio_s_artifact_bim_model_entities::editor::bim::entities;
        pub use semio_s_artifact_bim_model_interaction::editor::bim::interaction;
        pub use semio_s_artifact_bim_model_kit::editor::bim::utilities;
        #[path = "."]
        pub mod chrome {
            #[path = "../../../../🎛️chrome/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        pub use semio_s_artifact_bim_model_tests::editor::bim::presence;
        #[path = "."]
        pub mod transient {
            #[path = "../../../../🫧️transient/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod plan {
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::*;
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::*;
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🗺️plan/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod world {
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::*;
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::*;
                        #[path = "."]
                        pub mod envelope {
                            pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::envelope::*;
                            #[cfg(test)]
                            #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧊️world/🌡️envelope/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧊️world/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod section {
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::*;
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::*;
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/📐️section/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod schedule {
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::*;
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::*;
                        pub use semio_s_artifact_bim_model_ramp_runs::editor::bim::modes::edit::windows::schedule::edit;
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧮️schedule/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod family {
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/🧬️family/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod sheet {
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::*;
                        pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::*;
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🪟️windows/📄️sheet/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod options {
                #[path = "../../../../📌️panels/🧭️options/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod outliner {
                #[path = "../../../../📌️panels/🌳️outliner/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod library {
                #[path = "../../../../📌️panels/🛍️library/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod diagnostics {
                #[path = "../../../../📌️panels/🚨️diagnostics/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
