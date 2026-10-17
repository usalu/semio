//! 🧬️ schema of `semio-s-artifact-writer-writer` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::standards::v1::subsets::any::schema::artifact_items::{WRITER_DIALECT, WRITER_DOCUMENT_SCHEMA};
pub use crate::standards::v1::subsets::any::schema::artifact_items_2::{default_uri};
pub use crate::standards::v1::subsets::any::schema::artifact_items_3::{WriterDocumentChild, document_child_handle, document_snapshot_from_text, text_from_document_snapshot};
pub use crate::standards::v1::subsets::any::schema::artifact_items_4::{WriterWorkingScene, attach_writer_document_text, document_child_handle_with_text, genesis_writer_child_pack, writer_snapshot_with_text, writer_text, writer_text_for_handle, writer_text_owner};

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
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
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
