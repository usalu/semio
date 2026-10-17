//! 🔖️ v1 of `semio-s-artifact-vcs-vcs` — a path-preserving carve of the artifact root mounts.

pub use semio_s_artifact_vcs_vcs_tests::{VcsApplication};
pub use crate::standards::v1::artifact_items_6::{artifact};

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
                pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::*;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod vcs {
        pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::*;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod vcs {
        pub use semio_s_artifact_vcs_vcs_tests::viewer::vcs::*;
    }
}
