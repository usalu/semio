//! 🔍️ Property and schedule value types of `s.bim.model@1`: property definitions and values, phases and schedule keys.

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
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        pub mod values;
                        #[path = "../../../🧮️phases-values/🦀️.rs"]
                        pub mod phases_values;
                        #[path = "../../../🔩️schedule-kit-values/🦀️.rs"]
                        pub mod schedule_kit_values;
                        #[path = "../../../🪛️property-kit-values/🦀️.rs"]
                        pub mod property_kit_values;
                    }
                }
            }
        }
    }
}
