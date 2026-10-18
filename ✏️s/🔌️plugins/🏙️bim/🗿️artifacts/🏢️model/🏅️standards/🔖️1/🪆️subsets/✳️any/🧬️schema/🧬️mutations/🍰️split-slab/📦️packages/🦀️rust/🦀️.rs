//! 🍰️ mut-any-184 of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

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
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;
                        #[path = "."]
                        pub mod modify {
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::cut;
                        }
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::horizontal_rules;
                        pub use semio_s_artifact_bim_model_mut_any_68::standards::v1::subsets::any::schema::mutations::delete_slab;
                        pub use semio_s_artifact_bim_model_mut_any_5::standards::v1::subsets::any::schema::mutations::set_slab_boundary;
                        #[path = "../../🦀️.rs"]
                        pub mod split_slab;
                    }
                }
            }
        }
    }
}
