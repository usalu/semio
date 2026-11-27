//! 🚨️ Rule and issue value types of `s.bim.model@1`: element classes, rules, selectors, issues and camera viewpoints.

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
                        #[path = "../../../⚙️coordination-values/🦀️.rs"]
                        pub mod coordination_values;
                    }
                }
            }
        }
    }
}
