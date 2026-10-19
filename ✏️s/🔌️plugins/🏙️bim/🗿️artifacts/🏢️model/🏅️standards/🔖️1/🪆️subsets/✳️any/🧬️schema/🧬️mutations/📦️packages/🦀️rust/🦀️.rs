//! 🧬️ mut-any-56 of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

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
                    pub mod mutations {
                        #[path = "../../🌡️set-space-conditions/🦀️.rs"]
                        pub mod set_space_conditions;
                        #[path = "../../🥶️remove-space-conditions/🦀️.rs"]
                        pub mod remove_space_conditions;
                    }
                }
            }
        }
    }
}
