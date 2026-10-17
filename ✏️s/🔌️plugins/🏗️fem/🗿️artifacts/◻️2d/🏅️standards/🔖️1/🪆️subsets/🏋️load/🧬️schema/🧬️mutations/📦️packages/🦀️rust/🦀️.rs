//! 🏋️ `s.fem.fem2d` load mutations: load cases, loads and combinations.

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
                        pub mod create_load_case {
                            #[path = "../../📋️create-load-case/🦀️.rs"]
                            mod component;
                            #[path = "../../📋️create-load-case/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📋️create-load-case/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_load_case {
                            #[path = "../../🗑️delete-load-case/🦀️.rs"]
                            mod component;
                            #[path = "../../🗑️delete-load-case/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗑️delete-load-case/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_combination {
                            #[path = "../../🔗️create-combination/🦀️.rs"]
                            mod component;
                            #[path = "../../🔗️create-combination/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔗️create-combination/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_combination {
                            #[path = "../../✂️delete-combination/🦀️.rs"]
                            mod component;
                            #[path = "../../✂️delete-combination/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../✂️delete-combination/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_load_case_name {
                            #[path = "../../🏷️change-load-case-name/🦀️.rs"]
                            mod component;
                            #[path = "../../🏷️change-load-case-name/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏷️change-load-case-name/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        pub use semio_s_artifact_fem_2d_load_2::standards::v1::subsets::any::schema::mutations::{add_load, remove_load, change_load_case_self_weight, replace_load, replace_combination};
                    }
                }
            }
        }
    }
}
