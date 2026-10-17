//! 🪜️ `s.fem.fem2d` load mutations: load cases, loads and combinations (part 2).

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
                        pub mod add_load {
                            #[path = "../➕️add-load/🦀️.rs"]
                            mod component;
                            #[path = "../➕️add-load/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../➕️add-load/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_load {
                            #[path = "../➖️remove-load/🦀️.rs"]
                            mod component;
                            #[path = "../➖️remove-load/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../➖️remove-load/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_load_case_self_weight {
                            #[path = "../⚖️change-load-case-self-weight/🦀️.rs"]
                            mod component;
                            #[path = "../⚖️change-load-case-self-weight/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../⚖️change-load-case-self-weight/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_load {
                            #[path = "../🔁️replace-load/🦀️.rs"]
                            mod component;
                            #[path = "../🔁️replace-load/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🔁️replace-load/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_combination {
                            #[path = "../🔁️replace-combination/🦀️.rs"]
                            mod component;
                            #[path = "../🔁️replace-combination/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🔁️replace-combination/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
