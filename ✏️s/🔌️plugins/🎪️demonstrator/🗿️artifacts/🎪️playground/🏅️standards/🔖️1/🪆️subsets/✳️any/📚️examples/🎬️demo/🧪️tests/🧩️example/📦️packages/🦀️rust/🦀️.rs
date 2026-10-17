//! 🧩️ examples of `semio-s-artifact-demonstrator-playground` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use semio_s_artifact_demonstrator_playground_schema::{PLAYGROUND_DOCUMENT_SCHEMA, PlaygroundCapabilityRow};
pub use semio_s_artifact_demonstrator_playground_editor::{PLAYGROUND_DIALECT};
pub use crate::standards::v1::subsets::any::io::artifact_items_5::{build_pilot_languages, declaration, definition, pilot_languages, playground_diff_language, playground_document_language, playground_op_language, playground_pack_language, playground_spr_language};

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
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_demonstrator_playground_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../🦀️.rs"]
        pub mod tests;
    }
}
