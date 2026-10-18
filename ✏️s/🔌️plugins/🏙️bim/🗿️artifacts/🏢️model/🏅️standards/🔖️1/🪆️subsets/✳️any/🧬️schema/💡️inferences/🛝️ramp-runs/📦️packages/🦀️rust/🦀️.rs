//! 🛝️ ramp-runs of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
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
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/🛝️ramp-runs/🦀️.rs"]
                            pub mod ramp_runs;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            #[path = "../../../../../🚪️io/📤️export/🏗️ifc/🧊️brep/🦀️.rs"]
                            pub mod brep;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::sizes;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::storeys;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs;
                        #[path = "."]
                        pub mod ramp_runs {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod families {
                            #[path = "../../../🧬️families/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod clash_sets {
                            #[path = "../../../🧨️clash-sets/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit;
                            #[path = "."]
                            pub mod slabs {
                                #[path = "../../../🧊️element-solids/⬜️slabs/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod ceilings {
                                #[path = "../../../🧊️element-solids/🔲️ceilings/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod stairs {
                                #[path = "../../../🧊️element-solids/🪜️stairs/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../🧬️mutations/🧵️elements/🦀️.rs"]
                        pub mod elements;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules;
                        #[path = "../../../../🧬️mutations/🔌️component-rules/🦀️.rs"]
                        pub mod component_rules;
                        #[path = "../../../../🧬️mutations/📍️placement/🦀️.rs"]
                        pub mod placement;
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
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod schedule {
                        #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧮️schedule/✏️edit/🦀️.rs"]
                        pub mod edit;
                    }
                }
            }
        }
    }
}
