//! 🧬️ schema of `semio-s-artifact-sequence-sequence` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::mutations::SequenceMutation;
pub use crate::snapshot::schema::SequenceSnapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::default_host_snapshot;
pub use crate::snapshot::schema::SequenceHostSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::standards::v1::subsets::any::schema::artifact_items::{SEQUENCE_DOCUMENT_SCHEMA};
pub use crate::standards::v1::subsets::any::schema::artifact_items_3::{SequenceCamera, SequenceContentChild, SequenceEdge, SequenceStep, SequenceWorkingScene, SlotRef, StepParams, genesis_sequence_child_pack, require_sequence_working_scene, sequence_content_child_handle, sequence_content_child_with_owner, sequence_content_snapshot_from_working, sequence_derivable_scene, sequence_step_from_node, sequence_step_params, sequence_working_scene_for_handle, working_from_sequence_content_snapshot};
pub use crate::standards::v1::subsets::any::schema::artifact_items_5::{definition};

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
                    #[path = "../../⚙️operations/🦀️.rs"]
                    pub mod operations;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🧬️mutations/🦀️.rs"]
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
                        #[path = "../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
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
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }

}
