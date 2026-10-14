//! 🧱️ `s.fem.fem3d` material mutations.

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
                        pub mod create_material {
                            #[path = "../../🌱️create-material/🦀️.rs"]
                            mod component;
                            #[path = "../../🌱️create-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🌱️create-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_material {
                            #[path = "../../🗑️delete-material/🦀️.rs"]
                            mod component;
                            #[path = "../../🗑️delete-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗑️delete-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_material {
                            #[path = "../../🔁️replace-material/🦀️.rs"]
                            mod component;
                            #[path = "../../🔁️replace-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔁️replace-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
