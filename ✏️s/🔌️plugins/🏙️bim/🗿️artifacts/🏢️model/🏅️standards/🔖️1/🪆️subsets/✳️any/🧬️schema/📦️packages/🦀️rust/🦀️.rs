//! 🧬️ schema of `semio-s-artifact-bim-model` — the artifact schema descriptor; the snapshot value types are carved into the `*-values` crates and re-exported here.

extern crate semio_os_kernel as dsl;
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
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            #[path = "../../../🚪️io/📤️export/🏗️ifc/🧱️codec/🦀️.rs"]
                            pub mod codec;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_bim_model_geometry_values::standards::v1::subsets::any::schema::snapshot::anchors_values;
                        pub use semio_s_artifact_bim_model_properties_values::standards::v1::subsets::any::schema::snapshot::{phases_values, property_kit_values, schedule_kit_values};
                        pub use semio_s_artifact_bim_model_issues_values::standards::v1::subsets::any::schema::snapshot::coordination_values;
                        pub use semio_s_artifact_bim_model_values::standards::v1::subsets::any::schema::snapshot::{sheets_values, values, views_values};
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod families {
                            #[path = "../../💡️inferences/🧬️families/⚠️issues/🦀️.rs"]
                            pub mod issues;
                        }
                        #[path = "."]
                        pub mod energy_envelope {
                            #[path = "../../💡️inferences/🌡️energy-envelope/🧱️thermal/🦀️.rs"]
                            pub mod thermal;
                        }
                    }
                }
            }
        }
    }
}
