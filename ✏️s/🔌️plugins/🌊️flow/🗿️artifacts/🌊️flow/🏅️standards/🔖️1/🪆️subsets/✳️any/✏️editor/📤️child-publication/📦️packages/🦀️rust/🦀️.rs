//! 📤️ child publication of `semio-s-artifact-flow-flow` — a path-preserving carve of the editor mounts. See [`child_publication`].

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as dsl;

pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

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
                    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::snapshot;
                }
            }
        }
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        pub use semio_s_artifact_flow_flow_mutations::editor::flow::host;
    }
}

#[path = "../../🦀️.rs"]
mod child_publication;
pub use child_publication::{FlowChildPublicationWork, FlowLeafMode, FlowLeafWork};

#[cfg(test)]
#[global_allocator]
static ORIGINAL_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
