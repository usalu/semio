//! 🔖️ v1 of `semio-s-artifact-sequence-sequence` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::snapshot::schema::SequenceSnapshot;
pub use crate::snapshot::schema::default_host_snapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::SequenceHostSnapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_sequence_sequence_schema::{SEQUENCE_DOCUMENT_SCHEMA, SequenceStep, SequenceWorkingScene, SlotRef, StepParams, genesis_sequence_child_pack};
pub use semio_s_artifact_sequence_sequence_tests::{SEQUENCE_DIALECT};
pub use crate::standards::v1::artifact_items_4::{artifact_kind};
pub use crate::standards::v1::artifact_items_6::{SequenceApplication, artifact};

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
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_sequence_sequence_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_sequence_sequence_schema::snapshot;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_sequence_sequence_inferences::examples::demo::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sequence {
        pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::edit_rules;
        pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::terminology;
        pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::base;
        pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::host;
        #[path = "../../🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs"]
        pub mod retained;
        #[path = "../../🪆️subsets/✳️any/✏️editor/🔁️persistent/🦀️.rs"]
        pub mod persistent;
        #[path = "../../🪆️subsets/✳️any/✏️editor/🎞️import/🦀️.rs"]
        pub mod import;
        #[path = "../../🪆️subsets/✳️any/✏️editor/🎹️app/🦀️.rs"]
        pub mod app;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::commands::connection;
            #[path = "."]
            pub mod example {
                pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::commands::example::*;
                #[cfg(test)]
                #[path = "../../🪆️subsets/✳️any/✏️editor/🎮️commands/📚️example/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::commands::layout;
            pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::commands::node_graph;
            pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::commands::playback;
            pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::commands::step;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::modes::edit::windows::compiled;
                    pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::modes::edit::windows::main;
                    pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::modes::edit::windows::script;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::panels::catalogue;
            pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::panels::document;
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::panels::inspection::*;
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
    pub mod sequence {
        pub use semio_s_artifact_sequence_sequence_tests::viewer::sequence::*;
    }
}
