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
                        #[path = "."]
                        pub mod ramp_runs {
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs::*;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod spaces {
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::*;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🏠️spaces/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod quantities {
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::*;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧮️quantities/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod finishes {
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::finishes::*;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::finishes::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🎨️finishes/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod zones {
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::*;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🏘️zones/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod effective_properties {
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::*;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🏷️effective-properties/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod schedules {
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::*;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📋️schedules/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[cfg(test)]
                            #[path = "../../../../../../📋️schedules/🧪️tests/🗂️rows/🦀️.rs"]
                            pub mod tests_rows;
                            #[cfg(test)]
                            #[path = "../../../../../../📋️schedules/🧪️tests/🧮️table/🦀️.rs"]
                            pub mod tests_table;
                            #[cfg(test)]
                            #[path = "../../../../../../📋️schedules/🧪️tests/🕸️graph/🦀️.rs"]
                            pub mod tests_graph;
                        }
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        #[path = "."]
                        pub mod components {
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::*;
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod bodies {
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::bodies::*;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::bodies::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📦️bodies/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        #[cfg(any(test, feature = "testkit"))]
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::schedule_kit;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "."]
                        pub mod inferences {
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::io::text::inferences::quantities;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::io::text::inferences::spaces;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::text::inferences::ramp_runs;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::io::text::inferences::schedules;
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader;
                            #[path = "."]
                            pub mod zoning {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::zoning::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📥️import/🏗️ifc/🏘️zoning/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod elements {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::elements::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📥️import/🏗️ifc/🏗️elements/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
