//! 🧬️ schema of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::standards::v1::subsets::any::schema::artifact_items::{FLOW_DOCUMENT_SCHEMA};
pub use crate::standards::v1::subsets::any::schema::artifact_items_3::{FLOW_CONTENT_ID_DOMAIN, FlowContentChild, FlowContentHashWriter, FlowWorkingScene, cache_flow_content, flow_content_child_from_digest, flow_content_child_handle, flow_content_child_handle_and_cache, flow_content_child_handle_bounded, flow_content_digest, flow_content_node_from_working, flow_content_snapshot_from_working, flow_working_scene_for_handle, widget_from_node, widget_params, working_from_flow_content_snapshot};
pub use crate::standards::v1::subsets::any::schema::artifact_items_4::{flow_composed_snapshot, flow_genesis_content_pack, flow_working_scene};

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
                    pub mod retirement {
                        #[path="../../../../../../../♻️retirement/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
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
