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
                    pub mod bcf {
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::bcf::*;
                        #[cfg(test)]
                        #[path = "../../../../../../🚪️io/💬️bcf/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "."]
                        pub mod inferences {
                            #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🪜️stair-runs/🦀️.rs"]
                            pub mod stair_runs;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            #[path = "."]
                            pub mod frames {
                                pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::ifc::frames::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧭️frames/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod gbxml {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gbxml::codec;
                        }
                        #[path = "."]
                        pub mod gltf {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::codec;
                            #[path = "."]
                            pub mod document {
                                pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::document::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🧊️gltf/🧬️document/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::gltf::testkit;
                        }
                        #[path = "."]
                        pub mod energy {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::energy::target;
                            #[path = "."]
                            pub mod library {
                                pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::energy::library::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🔋️energy/🧱️library/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod diff {
                        #[path = "."]
                        pub mod patches {
                            #[path = "."]
                            pub mod building_elements {
                                #[path = "../../../../../🔺️diff/🩹️patches/🧱️building-elements/🪜️stairs-railings-ramps/🦀️.rs"]
                                pub mod stairs_railings_ramps;
                            }
                            #[path = "../../../../../🔺️diff/🩹️patches/🖋️hand-written/🦀️.rs"]
                            pub mod hand_written;
                        }
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra;
                    }
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::storeys;
                    }
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod storey_levels {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod curtain_layout {
                            #[path = "../../../../🪞️curtain-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stair_runs {
                            #[path = "../../../../🪜️stair-runs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../🧬️mutations/🔩️family-rules/🦀️.rs"]
                        pub mod family_rules;
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
            #[path = "../../../../../../✏️editor/🧵️gestures/📐️plane/🦀️.rs"]
            pub mod plane;
        }
        #[path = "."]
        pub mod marks {
            #[path = "../../../../../../✏️editor/🖍️marks/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
