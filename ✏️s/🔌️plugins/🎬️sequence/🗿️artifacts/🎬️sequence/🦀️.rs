//! 🎬️ Sequence artifact — the document entity this plugin's app edits (constitutional: general).

extern crate semio_framework_os_infinite as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;

use neural_engine::{Value};
use semio_framework_plugin::{MediaClass, MediaForm, MediaType};

use semio_s_artifact_stdio_semio_flow::schema::snapshot::{SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};

pub use crate::schema::mutations::SequenceMutation;

pub use crate::schema::diff::SequenceDiff;

pub use semio_s_artifact_sequence_sequence_schema::{SEQUENCE_DOCUMENT_SCHEMA};
pub use crate::snapshot::schema::{default_host_snapshot, default_snapshot, SequenceHostSnapshot, SequenceSnapshot};

pub use semio_s_artifact_sequence_sequence_tests::{SEQUENCE_DIALECT};

pub use semio_s_artifact_sequence_sequence_schema::{SequenceCamera, SequenceContentChild, SequenceEdge, SequenceStep, SequenceWorkingScene, SlotRef, StepParams, genesis_sequence_child_pack, require_sequence_working_scene, sequence_content_child_handle, sequence_content_child_with_owner, sequence_content_snapshot_from_working, sequence_derivable_scene, sequence_step_from_node, sequence_step_params, sequence_working_scene_for_handle, working_from_sequence_content_snapshot};

//#region 🔖️Collections
impl protocol::Identified<String> for SequenceStep {
    fn id(&self) -> &String {
        &self.id
    }
}

impl protocol::Identified<String> for SequenceEdge {
    fn id(&self) -> &String {
        &self.id
    }
}

//#endregion 🔖️Collections

pub use semio_s_artifact_sequence_sequence_v1::{artifact_kind};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
/// 🧪️ The mounted-app harness (`context`) and the app-level laws of the sequence editor: they drive the whole app, so they
/// sit above every layer and the leaf tests of commands, windows and panels reach the harness by this path.
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
pub mod editor_unit_tests;
//#endregion 🧪️Tests
pub use semio_s_artifact_sequence_sequence_schema::{definition};

pub use semio_s_artifact_sequence_sequence_v1::{SequenceApplication, artifact};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_sequence_sequence_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_sequence_sequence_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_sequence_sequence_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::operations;
                    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_sequence_sequence_editor::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_sequence_sequence_inferences::standards::v1::subsets::any::io::binary::inferences;
    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::io::binary::mutations;
    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::io::binary::snapshot;
}
pub mod sqlite {
    pub use semio_s_artifact_sequence_sequence_inferences::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_sequence_sequence_editor::standards::v1::subsets::any::io::text::diff;
    pub use semio_s_artifact_sequence_sequence_inferences::standards::v1::subsets::any::io::text::inferences;
    pub use semio_s_artifact_sequence_sequence_schema::standards::v1::subsets::any::io::text::mutations;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::csv;
                                pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::md;
                                pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
                                pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::export::serializers::artifacts::md;
                                pub use semio_s_artifact_sequence_sequence_tests::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub use semio_s_artifact_sequence_sequence_schema::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
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
    /// 🖥️ Sequence play app: the artifact nest mounts the layers `🧱️base` (ids, typed media I/O, faults, camera),
    /// `🖥️host` (the UI-editing engine), the leaf nodes (`🎮️commands/*`, `🎭️modes/✏️edit/🪟️windows/*`, `📌️panels/*`,
    /// `📐️edit-rules`, `🗣️terminology`) and, on top of them, `🧵️retained` (the aggregated command enum),
    /// `🔁️persistent`, `🎞️import` and `🎹️app` (the `ArtifactEditor` impl and the manifest stitch). Every layer names the
    /// layer it uses by its own module path.
    #[path = "."]
    pub mod sequence {
        pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::presence;
        pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::config;

        pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::edit_rules;
        pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::terminology;
        pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::base;
        pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::host;
        pub use semio_s_artifact_sequence_sequence_v1::editor::sequence::retained;
        pub use semio_s_artifact_sequence_sequence_v1::editor::sequence::persistent;
        pub use semio_s_artifact_sequence_sequence_v1::editor::sequence::import;
        pub use semio_s_artifact_sequence_sequence_v1::editor::sequence::app;
        #[cfg(target_arch = "wasm32")]
pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::wasm;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::commands::connection;
            pub use semio_s_artifact_sequence_sequence_tests::editor::sequence::commands::example;
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
            pub use semio_s_artifact_sequence_sequence_editor::editor::sequence::panels::inspection;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                pub use semio_s_artifact_sequence_sequence_inferences::editor::sequence::examples::demo_session::*;

            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sequence {
        pub use semio_s_artifact_sequence_sequence_tests::viewer::sequence::*;

        pub use semio_s_artifact_sequence_sequence_inferences::viewer::sequence::presence;
        pub use semio_s_artifact_sequence_sequence_inferences::viewer::sequence::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_sequence_sequence_inferences::viewer::sequence::modes::view::*;

                pub use semio_s_artifact_sequence_sequence_inferences::viewer::sequence::modes::view::windows;
            }
        }
    }
}

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connection/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️layout/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_2;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_3;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🧪️tests/🧪️history/🦀️.rs"]
mod hoisted_tests_4;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️step/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_5;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️compiled/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_6;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_7;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_8;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_9;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_10;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_11;
