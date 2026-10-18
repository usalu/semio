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
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::profile;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
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
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep;
                        #[path = "."]
                        pub mod plan_linework {
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::*;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod view_linework {
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework::*;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework::*;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::view_linework::frame;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::view_linework::hidden_lines;
                            #[cfg(test)]
                            #[path = "../../../../../../🖼️view-linework/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod energy_envelope {
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::*;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::energy_envelope::thermal;
                            #[cfg(test)]
                            #[path = "../../../../../../🌡️energy-envelope/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod annotation_layout {
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::*;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🪧️annotation-layout/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod diagnostics {
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::*;
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::*;
                            #[path = "."]
                            pub mod index {
                                pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::index::*;
                                #[cfg(test)]
                                #[path = "../../../../../../⚠️diagnostics/🗂️index/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod wall_depth {
                                pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::wall_depth::*;
                                #[cfg(test)]
                                #[path = "../../../../../../⚠️diagnostics/🧗️wall-depth/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../⚠️diagnostics/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            #[path = "."]
                            pub mod walls {
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls::*;
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🧊️element-solids/🧱️walls/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                                #[cfg(test)]
                                #[path = "../../../../../../🧊️element-solids/🧱️walls/🧪️tests/🏔️attached/🦀️.rs"]
                                pub mod attached;
                            }
                            #[path = "."]
                            pub mod curtain_walls {
                                pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls::*;
                                pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🧊️element-solids/🪟️curtain-walls/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../🧊️element-solids/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod fillers {
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::fillers::*;
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::fillers::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🧊️element-solids/🚪️fillers/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::stairs;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::*;
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::data;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::walls;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::wall_sweeps;
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader;
                            #[path = "."]
                            pub mod data {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::data::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📥️import/🏗️ifc/🧬️data/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
