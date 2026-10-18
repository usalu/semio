//! 🏠️ spaces of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

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
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/🏠️spaces/🦀️.rs"]
                            pub mod spaces;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::profile;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod view_linework {
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::view_linework::clip;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::view_linework::cut;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::view_linework::frame;
                            #[path = "../../../🖼️view-linework/👁️hidden-lines/🦀️.rs"]
                            pub mod hidden_lines;
                        }
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs;
                        #[path = "."]
                        pub mod spaces {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        #[path = "."]
                        pub mod bodies {
                            #[path = "../../../📦️bodies/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod analytical_members {
                            #[path = "../../../🦴️analytical-members/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod energy_envelope {
                            #[path = "../../../🌡️energy-envelope/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            #[path = "."]
                            pub mod walls {
                                #[path = "../../../🧊️element-solids/🧱️walls/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod fillers {
                                #[path = "../../../🧊️element-solids/🚪️fillers/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::columns;
                            #[path = "."]
                            pub mod beams {
                                #[path = "../../../🧊️element-solids/➖️beams/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::slabs;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::element_solids::ceilings;
                            #[path = "."]
                            pub mod railings {
                                #[path = "../../../🧊️element-solids/🛤️railings/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod ramps {
                                pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::ramps::*;
                                pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::ramps::*;
                                #[cfg(test)]
                                #[path = "../../../🧊️element-solids/🛝️ramps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod rail_hosts {
                                #[path = "../../../🧊️element-solids/🪝️rail-hosts/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                }
            }
        }
    }
}
