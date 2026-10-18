//! 📄️ sheets of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

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
                        #[path = "../../🦀️.rs"]
                        pub mod sheets;
                        #[path = "."]
                        pub mod svg {
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::style;
                            pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::svg::drawing;
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg::testkit;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                        }
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                    }
                }
            }
        }
    }
}
