//! 🔺️ `s.fem.fem3d` diff: the cold row-delta algebra over the model snapshot and its text and binary representations.

#![allow(clippy::result_large_err)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as vcs;

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
                    #[path = "../../🦀️.rs"]
                    pub mod diff;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
