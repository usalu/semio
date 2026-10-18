//! 🧭️ mut-any-116 of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
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
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::authored::option_rules;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;
                        #[path = "../../🦀️.rs"]
                        pub mod create_design_option;
                        #[path = "../../../🧭️delete-design-option/🦀️.rs"]
                        pub mod delete_design_option;
                        pub use semio_s_artifact_bim_model_mut_any_50::standards::v1::subsets::any::schema::mutations::set_element_option;
                    }
                }
            }
        }
    }
}
