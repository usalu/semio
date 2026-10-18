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
                        #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod svg {
                            #[path = "."]
                            pub mod codec {
                                pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::svg::codec::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🎨️svg/🧱️codec/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::path;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::style;
                            #[path = "."]
                            pub mod sheet {
                                pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::sheet::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🎨️svg/📐️sheet/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod drawing {
                                pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::svg::drawing::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🎨️svg/🖍️drawing/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg::testkit;
                        }
                        #[path = "."]
                        pub mod energy {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::energy::target;
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::energy::library;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::energy::pairs;
                            #[path = "../../../../../../🚪️io/📤️export/🔋️energy/🏗️build/🦀️.rs"]
                            pub mod build;
                            #[path = "../../../../../../🚪️io/📤️export/🔋️energy/📊️table/🦀️.rs"]
                            pub mod table;
                        }
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::ifc::frames;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::export::ifc::brep;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧬️data/🦀️.rs"]
                            pub mod data;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏗️frame/🦀️.rs"]
                            pub mod frame;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/⬜️horizontal/🦀️.rs"]
                            pub mod horizontal;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🔲️ceilings/🦀️.rs"]
                            pub mod ceilings;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪜️circulation/🦀️.rs"]
                            pub mod circulation;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🛝️ramps/🦀️.rs"]
                            pub mod ramps;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🦴️structure/🦀️.rs"]
                            pub mod structure;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone;
                        }
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        #[path = "."]
                        pub mod energy_envelope {
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::energy_envelope::thermal;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;
                        pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::placement;
                        pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry;
                        #[path = "."]
                        pub mod modify {
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
#[path = "."]
pub mod render {
    #[path = "."]
    pub mod plan {
        pub use semio_s_artifact_bim_model_plan_linework::render::plan::*;
        #[cfg(test)]
        #[path = "../../../../../../🖌️render/🗺️plan/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "../../../../../../🖌️render/🌡️envelope/🦀️.rs"]
    pub mod envelope;
    #[path = "../../../../../../🖌️render/🦴️structure/🦀️.rs"]
    pub mod structure;
}
