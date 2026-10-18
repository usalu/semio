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
                        #[path = "."]
                        pub mod inferences {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::io::text::inferences::element_solids;
                            #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🪑️components/🦀️.rs"]
                            pub mod components;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "../../../../../../🚪️io/📤️export/🧾️json/🦀️.rs"]
                        pub mod json;
                        #[path = "."]
                        pub mod sheets {
                            pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::sheets::ink;
                            #[path = "../../../../../../🚪️io/📤️export/📄️sheets/🎨️svg/🦀️.rs"]
                            pub mod svg;
                        }
                        #[path = "."]
                        pub mod svg {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::svg::codec;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::path;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::style;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::sheet;
                            pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::svg::drawing;
                            #[path = "../../../../../../🚪️io/📤️export/🎨️svg/📏️projection/🦀️.rs"]
                            pub mod projection;
                        }
                        #[path = "."]
                        pub mod ifc {
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧾️context/🦀️.rs"]
                            pub mod context;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏛️spatial/🦀️.rs"]
                            pub mod spatial;
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
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls;
                            #[path = "."]
                            pub mod wall_sweeps {
                                pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps::*;
                                pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod plan_kit {
                                pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::*;
                                pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/📐️plan-kit/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod columns {
                                pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns::*;
                                pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🏛️columns/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod beams {
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::beams::*;
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::beams::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/➖️beams/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod slabs {
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs::*;
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/⬜️slabs/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ceilings {
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings::*;
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod roofs {
                                pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs::*;
                                pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🏠️roofs/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod stairs {
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::stairs::*;
                                pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::stairs::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🪜️stairs/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod railings {
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::railings::*;
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::railings::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🛤️railings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod components {
                                pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::element_solids::components::*;
                                pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::element_solids::components::*;
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../🧲️wall-depth/🦀️.rs"]
                        pub mod wall_depth;
                        #[path = "."]
                        pub mod wall_geometry {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
