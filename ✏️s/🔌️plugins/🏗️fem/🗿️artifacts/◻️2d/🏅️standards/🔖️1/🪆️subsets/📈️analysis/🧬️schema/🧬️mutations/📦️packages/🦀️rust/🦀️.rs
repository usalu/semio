//! 📈️ `s.fem.fem2d` analysis-settings mutation.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

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
                        #[path = "."]
                        pub mod update_analysis_settings {
                            #[path = "../../🎛️update-analysis-settings/🦀️.rs"]
                            mod component;
                            #[path = "../../🎛️update-analysis-settings/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🎛️update-analysis-settings/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
