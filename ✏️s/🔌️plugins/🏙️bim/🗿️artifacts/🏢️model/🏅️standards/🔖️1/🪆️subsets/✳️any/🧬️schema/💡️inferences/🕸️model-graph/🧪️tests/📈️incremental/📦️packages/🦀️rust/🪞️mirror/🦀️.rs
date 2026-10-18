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
                        pub mod sheets {
                            #[path = "../../../../../../../../🚪️io/📤️export/📄️sheets/✒️ink/🦀️.rs"]
                            pub mod ink;
                            #[path = "../../../../../../../../🚪️io/📤️export/📄️sheets/📊️tables/🦀️.rs"]
                            pub mod tables;
                        }
                        #[path = "."]
                        pub mod svg {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::svg::codec;
                            #[path = "."]
                            pub mod path {
                                pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::path::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📤️export/🎨️svg/✒️path/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod style {
                                pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::style::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📤️export/🎨️svg/🎚️style/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::sheet;
                            #[path = "../../../../../../../../🚪️io/📤️export/🎨️svg/🖍️drawing/🦀️.rs"]
                            pub mod drawing;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::sizes;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::storeys;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod storey_levels {
                            pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::*;
                            pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🏢️storey-levels/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::*;
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::*;
                            #[path = "."]
                            pub mod dirty {
                                pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::dirty::*;
                                #[cfg(test)]
                                #[path = "../../../../../🎯️dirty/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod session {
                                pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::session::*;
                                #[cfg(test)]
                                #[path = "../../../../../📡️session/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod incremental_tests;
                        }
                        #[path = "."]
                        pub mod wall_layout {
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::*;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧱️wall-layout/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod attach {
                                pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::attach::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🧱️wall-layout/🧲️attach/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod curtain_layout {
                            pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout::*;
                            pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🪞️curtain-layout/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod opening_frames {
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::*;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🪟️opening-frames/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        #[path = "."]
                        pub mod sheet_layout {
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::*;
                            #[path = "."]
                            pub mod windows {
                                pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::windows::*;
                                #[cfg(test)]
                                #[path = "../../../../../../📄️sheet-layout/🖼️windows/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod title_block {
                                pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::title_block::*;
                                #[cfg(test)]
                                #[path = "../../../../../../📄️sheet-layout/🧾️title-block/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs;
                        }
                    }
                }
            }
        }
    }
}
