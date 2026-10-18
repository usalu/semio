//! 🪑️ components of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

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
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/⚠️diagnostics/🦀️.rs"]
                            pub mod diagnostics;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod holders {
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::holders::*;
                            #[cfg(test)]
                            #[path = "../../../../../🚪️io/📤️export/🧱️holders/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod gbxml {
                            #[path = "."]
                            pub mod geometry {
                                pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::gbxml::geometry::*;
                                #[cfg(test)]
                                #[path = "../../../../../🚪️io/📤️export/🌿️gbxml/📐️geometry/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "../../../../../🚪️io/📤️export/🌿️gbxml/🔗️pairing/🦀️.rs"]
                            pub mod pairing;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces;
                        #[path = "."]
                        pub mod families {
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues;
                        }
                        #[path = "."]
                        pub mod components {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod mep {
                            #[path = "../../../🌀️mep/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::bodies;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        #[path = "."]
                        pub mod diagnostics {
                            #[path = "../../../⚠️diagnostics/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls;
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::roofs;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::stairs;
                            #[path = "."]
                            pub mod components {
                                #[path = "../../../🧊️element-solids/🪑️components/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mep {
                                #[path = "../../../🧊️element-solids/🌀️mep/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts;
                        }
                    }
                }
            }
        }
    }
}
