//! 🪟️ curtain-walls of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as schema;

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
                        #[path = "../../../../../../🚪️io/📤️export/🧱️holders/🦀️.rs"]
                        pub mod holders;
                        #[path = "."]
                        pub mod gbxml {
                            #[path = "../../../../../../🚪️io/📤️export/🌿️gbxml/📐️geometry/🦀️.rs"]
                            pub mod geometry;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::profile;
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
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls;
                            #[path = "."]
                            pub mod curtain_walls {
                                #[path = "../../🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::fillers;
                            #[path = "."]
                            pub mod wall_sweeps {
                                #[path = "../../../🧷️wall-sweeps/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::ramps;
                            #[path = "."]
                            pub mod rail_hosts {
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::*;
                                pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::rail_hosts::*;
                                #[cfg(test)]
                                #[path = "../../../🪝️rail-hosts/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
