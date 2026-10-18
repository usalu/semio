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
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            #[path = "."]
                            pub mod brep {
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::export::ifc::brep::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧊️brep/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::profile;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::sizes;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod view_linework {
                            #[path = "../../../../🖼️view-linework/✂️clip/🦀️.rs"]
                            pub mod clip;
                            #[path = "../../../../🖼️view-linework/🪓️cut/🦀️.rs"]
                            pub mod cut;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::view_linework::frame;
                        }
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        #[path = "."]
                        pub mod wall_layout {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout;
                        #[path = "."]
                        pub mod opening_frames {
                            #[path = "../../../../🪟️opening-frames/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs;
                        #[path = "."]
                        pub mod families {
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues;
                            #[path = "."]
                            pub mod metrics {
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::metrics::*;
                                #[cfg(test)]
                                #[path = "../../../../🧬️families/📊️metrics/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod findings {
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::findings::*;
                                #[cfg(test)]
                                #[path = "../../../../🧬️families/🩺️findings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod parameters {
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::parameters::*;
                                #[cfg(test)]
                                #[path = "../../../../🧬️families/🧮️parameters/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod solids {
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::solids::*;
                                #[cfg(test)]
                                #[path = "../../../../🧬️families/🧊️solids/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod clash_sets {
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::*;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::*;
                            #[cfg(test)]
                            #[path = "../../../../🧨️clash-sets/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            #[path = "."]
                            pub mod columns {
                                #[path = "../../../../🧊️element-solids/🏛️columns/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs;
                            #[path = "."]
                            pub mod ramps {
                                #[path = "../../../../🧊️element-solids/🛝️ramps/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_35::standards::v1::subsets::any::schema::mutations::set_schedule;
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
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod family {
                        #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️family/🏷️vocabulary/🦀️.rs"]
                        pub mod vocabulary;
                    }
                    #[path = "."]
                    pub mod schedule {
                        #[path = "."]
                        pub mod edit {
                            pub use semio_s_artifact_bim_model_ramp_runs::editor::bim::modes::edit::windows::schedule::edit::*;
                            #[cfg(test)]
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧮️schedule/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
