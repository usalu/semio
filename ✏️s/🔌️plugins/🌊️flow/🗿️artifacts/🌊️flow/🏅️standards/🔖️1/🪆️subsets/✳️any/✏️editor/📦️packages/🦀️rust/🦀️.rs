//! ✏️ editor of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
use semio_framework_artifact_playbook_playbook as playbook;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_flow_flow_schema::{FLOW_DOCUMENT_SCHEMA, FlowWorkingScene, flow_composed_snapshot, flow_genesis_content_pack};
pub use semio_s_artifact_flow_flow_viewer::{FLOW_DIALECT};
pub use crate::editor::flow::artifact_items_5::{artifact_kind};

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
pub use semio_s_artifact_flow_flow_tests::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::editor_vocabulary;
        pub use semio_s_artifact_flow_flow_mutations::editor::flow::host;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_flow_flow_inferences::editor::flow::presence::*;
        }
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::terminology;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::add_widget;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::connect_media_ports;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::context_menu_at;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::delete_selection;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::disconnect;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::duplicate_widget;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::evaluate;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::flow_eval_resolve;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::flow_eval_tick;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::focus_selection;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::move_media_node;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::node_graph_edit;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::node_graph_viewport;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::open_spotlight;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::patch_flow_widgets;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::remove_widget;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::rename_flow_widget;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::reorganize;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::replace_image;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::run_extension_action;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_active_example;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_catalogue_sections;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_factor;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_snap_enabled;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_visible;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_lod_mode;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_preview_off;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::set_proximity_distance;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::spotlight_commit;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::toggle_extension;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::set_contributions;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_flow_flow_viewer::editor::flow::modes::edit::*;
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::tools;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::windows::main::*;
                        pub use semio_s_artifact_flow_flow_config::editor::flow::modes::edit::windows::main::config;
                        pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::edit::windows::main::transient;
                    }
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::windows::compiled;
                }
            }
            #[path = "."]
            pub mod generate {
                pub use semio_s_artifact_flow_flow_viewer::editor::flow::modes::generate::*;
                #[path = "."]
                pub mod commands {
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::add_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::remove_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::rename_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::select_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::update_generation_values;
                }
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
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::panels::catalogue;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::panels::document;
            pub use semio_s_artifact_flow_flow_tests::editor::flow::panels::inspection;
        }
    }
}
