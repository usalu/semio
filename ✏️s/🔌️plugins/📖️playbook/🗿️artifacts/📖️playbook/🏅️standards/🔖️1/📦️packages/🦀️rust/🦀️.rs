//! 🔖️ v1 of `semio-s-artifact-playbook-playbook` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework as semio_framework;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::mutations::PlaybookMutation;
pub use crate::schema::snapshot::PlaybookSnapshot;
pub use crate::playbook::PLAYBOOK_DOCUMENT_SCHEMA;
use semio_framework_artifact_playbook_playbook as playbook;
pub use crate::playbook::PlaybookBlock;
pub use crate::playbook::PlaybookBlockOption;
pub use crate::playbook::PlaybookExpr;
pub use crate::playbook::PlaybookSpec;
pub use crate::playbook::PlaybookStep;
pub use crate::playbook::PlaybookVectorField;
pub use crate::playbook::PLAYBOOK_BUILTIN_KINDS;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_playbook_playbook_snapshot::{PLAYBOOK_DIALECT, empty_playbook_snapshot, steps_from_flow_content};
pub use crate::standards::v1::artifact_items_5::{PlaybookApplication, artifact, definition, pilot_languages};

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
                    pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_playbook_playbook_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::central_apply;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::io::binary::mutations;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_playbook_playbook_examples::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod playbook {
        pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::*;
        pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::engine;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::config::*;
        }
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::*;
            #[path = "."]
            pub mod add_block {
                pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::add_block::*;
                #[cfg(test)]
                #[path = "../../🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️add-block/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod add_step {
                pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::add_step::*;
                #[cfg(test)]
                #[path = "../../🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️add-step/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
            }
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::move_block;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::move_step;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::remove_block;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::remove_step;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::set_active_example;
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::commands::set_contributions;
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::commands::update_playbook;
        }
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::modes::*;
            #[path = "."]
            pub mod builder {
                pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::modes::builder::*;
                #[cfg(test)]
                #[path = "../../🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::modes::builder::windows::*;
                    #[path = "."]
                    pub mod builder {
                        pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::modes::builder::windows::builder::*;
                        #[cfg(test)]
                        #[path = "../../🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::changes;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::steps;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::activity;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::source;
                    pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::modes::builder::windows::files;
                }
            }
        }
        #[cfg(test)]
        #[path = "../../🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod unit_tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playbook {
        pub use semio_s_artifact_playbook_playbook_inferences::viewer::playbook::*;
        #[cfg(test)]
        #[path = "../../🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
}
