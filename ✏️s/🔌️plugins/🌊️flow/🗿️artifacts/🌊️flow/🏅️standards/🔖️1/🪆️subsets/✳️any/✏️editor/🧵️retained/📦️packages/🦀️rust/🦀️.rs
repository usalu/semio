//! 🧵️ retained command frontiers of `semio-s-artifact-flow-flow` — a path-preserving carve of the editor mounts. See [`retained`].

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as dsl;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_flow_flow_schema::{FLOW_DOCUMENT_SCHEMA, FlowWorkingScene};
pub use semio_s_artifact_flow_flow_inferences::editor::flow::editor_vocabulary::{FLOW_STORE_MAX_SCENE_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
pub use semio_s_artifact_flow_flow_editor_publication as child_publication;
pub use editor::flow::commands::{add_widget, connect_media_ports, disconnect, evaluate, flow_eval_resolve, flow_eval_tick, node_graph_edit, remove_widget, reorganize};

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
            pub mod generate {
                #[path = "."]
                pub mod commands {
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::add_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::remove_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::rename_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::select_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::update_generation_values;
                }
            }
        }
    }
}

/// 🧩️ The editor application shape every Flow retained command is generic over: the aggregated command, snapshot and mutation of the Flow document.
pub trait FlowApp: semio_plugin::ArtifactEditor<Snapshot = FlowSnapshot, Mutation = FlowMutation, Command = FlowCommand, ConfigMutation = semio_plugin::NoConfigMutation, DraftMutation = semio_plugin::NoConfigMutation> {}
impl<A> FlowApp for A where A: semio_plugin::ArtifactEditor<Snapshot = FlowSnapshot, Mutation = FlowMutation, Command = FlowCommand, ConfigMutation = semio_plugin::NoConfigMutation, DraftMutation = semio_plugin::NoConfigMutation> {}

#[path = "../../../🕹️flow-command/🦀️.rs"]
mod flow_command;
pub use flow_command::FlowCommand;

#[path = "../../🏠️instance-owner/🦀️.rs"]
mod instance_owner;
pub use instance_owner::FlowInstanceOperationOwner;

#[path = "../../🦀️.rs"]
pub mod retained;

#[cfg(test)]
#[global_allocator]
static ORIGINAL_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
