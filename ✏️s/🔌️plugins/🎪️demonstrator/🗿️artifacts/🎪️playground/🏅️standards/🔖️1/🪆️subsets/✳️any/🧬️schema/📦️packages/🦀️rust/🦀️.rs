//! 🧬️ schema of `semio-s-artifact-demonstrator-playground` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use crate::standards::v1::subsets::any::schema::artifact_items::{PLAYGROUND_DOCUMENT_SCHEMA};
pub use crate::standards::v1::subsets::any::schema::artifact_items_4::{PlaygroundCapabilityRow};

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
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod change_schema {
                            #[path = "../../🧬️mutations/✒️change-schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧬️mutations/✒️change-schema/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧬️mutations/✒️change-schema/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/✒️change-schema/🦀️.rs"]
                            pub mod change_schema;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../🚪️io/📝️text/🧬️mutations/✒️change-schema/🦀️.rs"]
                            pub mod change_schema;
                        }
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
