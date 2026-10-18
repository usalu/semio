//! 🏗️ ifc-2 of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

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
                    pub mod import {
                        #[path = "../../🦀️.rs"]
                        pub mod ifc;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::io::export::ifc::codec;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::data;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::walls;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::wall_sweeps;
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::ceilings;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::components;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::mep;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::authored::option_rules;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;
                    }
                }
            }
        }
    }
}
