//! 🛡️ `s.fem.fem3d` guards: the payload validations and referential-integrity lookups every mutation kind shares.

#![allow(clippy::result_large_err)]

extern crate semio_os_kernel as protocol;

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
                    pub mod guards;
                }
            }
        }
    }
}
