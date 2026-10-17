//! 🔖️ v1 of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_schema as schema;

#[cfg(test)]
pub use crate::snapshot::schema::PresentationSnapshot;
pub use semio_s_artifact_animate_presentation_inferences::{PRESENTATION_DOCUMENT_SCHEMA, presentation_working_scene};
pub use semio_s_artifact_animate_presentation_tests::{default_presentation_snapshot};
pub use semio_s_artifact_animate_presentation_mutations::{ANIMATE_DIALECT};

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
                    pub use semio_s_artifact_animate_presentation_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::inferences;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_animate_presentation_mutations::standards::v1::subsets::any::io::*;
                }
            }
        }
    }
}
pub use semio_s_artifact_animate_presentation_inferences::snapshot;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_animate_presentation_examples::examples::demo::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        pub use semio_s_artifact_animate_presentation_animate::editor::animate::*;
        pub use semio_s_artifact_animate_presentation_animate::editor::animate::terminology;
        pub use semio_s_artifact_animate_presentation_mutations::editor::animate::support;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                pub use semio_s_artifact_animate_presentation_tests::editor::animate::modes::main::windows;
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_animate_presentation_animate::editor::animate::panels::*;
            pub use semio_s_artifact_animate_presentation_animate::editor::animate::panels::artifact;
            pub use semio_s_artifact_animate_presentation_animate::editor::animate::panels::catalogue;
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_animate_presentation_animate::editor::animate::panels::inspection::*;
                #[cfg(test)]
                #[path = "../../🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs"]
                pub mod semantic_contract;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod animate {
        pub use semio_s_artifact_animate_presentation_mutations::viewer::animate::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_animate_presentation_tests::viewer::animate::modes::view::windows;
            }
        }
    }
}
