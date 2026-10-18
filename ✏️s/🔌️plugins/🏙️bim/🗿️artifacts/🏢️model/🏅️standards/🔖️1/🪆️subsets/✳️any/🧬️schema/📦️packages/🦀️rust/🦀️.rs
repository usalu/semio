//! 🧬️ schema of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
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
                        #[path = "../../📸️snapshot/💠️values/🦀️.rs"]
                        pub mod values;
                        #[path = "../../📸️snapshot/🧮️phases-values/🦀️.rs"]
                        pub mod phases_values;
                        #[path = "../../📸️snapshot/🔩️schedule-kit-values/🦀️.rs"]
                        pub mod schedule_kit_values;
                        #[path = "../../📸️snapshot/🧲️anchors-values/🦀️.rs"]
                        pub mod anchors_values;
                        #[path = "../../📸️snapshot/🪛️property-kit-values/🦀️.rs"]
                        pub mod property_kit_values;
                        #[path = "../../📸️snapshot/🧷️views-values/🦀️.rs"]
                        pub mod views_values;
                        #[path = "../../📸️snapshot/🔧️sheets-values/🦀️.rs"]
                        pub mod sheets_values;
                        #[path = "../../📸️snapshot/⚙️coordination-values/🦀️.rs"]
                        pub mod coordination_values;
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
