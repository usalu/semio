//! 👁️ viewer of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_flow_flow_schema::{FLOW_DOCUMENT_SCHEMA, flow_composed_snapshot, flow_genesis_content_pack};
pub use crate::viewer::flow::artifact_items_2::{FLOW_DIALECT};

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
                    pub use semio_s_artifact_flow_flow_mutations::standards::v1::subsets::any::schema::mutations;
                    pub use semio_s_artifact_flow_flow_mutations::standards::v1::subsets::any::schema::retirement;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::editor_vocabulary;
        pub use semio_s_artifact_flow_flow_mutations::editor::flow::host;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../✏️editor/🎮️commands/🧮️evaluate/🦀️.rs"]
            pub mod evaluate;
            #[path = "../../../✏️editor/🎮️commands/🏁️flow-eval-resolve/🦀️.rs"]
            pub mod flow_eval_resolve;
            #[path = "."]
            pub mod flow_eval_tick {
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::flow_eval_tick::*;
                #[cfg(test)]
                #[path = "../../../✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::node_graph_edit;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::reorganize;
            #[path = "../../../✏️editor/🎮️commands/▶️run-extension-action/🦀️.rs"]
            pub mod run_extension_action;
            #[path = "../../../✏️editor/🎮️commands/✅️spotlight-commit/🦀️.rs"]
            pub mod spotlight_commit;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::windows::main::*;
                        pub use semio_s_artifact_flow_flow_config::editor::flow::modes::edit::windows::main::config;
                    }
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::windows::compiled;
                }
            }
            #[path = "."]
            pub mod generate {
                #[path = "../../../✏️editor/🎭️modes/🧬️generate/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::windows::form;
                    pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::generate::windows::generations;
                    pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::generate::windows::preview;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod flow {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_flow_flow_editor_leaves::viewer::flow::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_flow_flow_editor_leaves::viewer::flow::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../🎭️modes/👁️view/🪟️windows/🌊️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
