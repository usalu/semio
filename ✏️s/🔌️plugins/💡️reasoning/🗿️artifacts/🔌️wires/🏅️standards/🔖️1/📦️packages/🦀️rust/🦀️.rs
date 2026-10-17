//! 🔖️ v1 of `semio-s-artifact-reasoning-wires` — a path-preserving carve of the artifact root mounts.

pub use semio_s_artifact_reasoning_wires_schema::{WIRES_DIALECT};
pub use crate::standards::v1::artifact_items_5::{WiresApplication, artifact, definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::io::*;
                }
            }
        }
    }
}
pub use semio_s_artifact_reasoning_wires_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wires {
        pub use semio_s_artifact_reasoning_wires_wires::editor::wires::*;
        #[cfg(test)]
        #[path = "../../🪆️subsets/✳️any/✏️editor/🧪️tests/⚖️declared-verbs/🦀️.rs"]
        pub mod declared_verb_laws;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wires {
        pub use semio_s_artifact_reasoning_wires_viewer::viewer::wires::*;
    }
}
