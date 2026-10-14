//! 🛡️ `s.fem.fem2d` boundary mutations: supports.

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
                        pub mod create_support {
                            #[path = "../../🛡️create-support/🦀️.rs"]
                            mod component;
                            #[path = "../../🛡️create-support/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🛡️create-support/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_support {
                            #[path = "../../🗑️delete-support/🦀️.rs"]
                            mod component;
                            #[path = "../../🗑️delete-support/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗑️delete-support/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_support {
                            #[path = "../../🔁️replace-support/🦀️.rs"]
                            mod component;
                            #[path = "../../🔁️replace-support/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔁️replace-support/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
