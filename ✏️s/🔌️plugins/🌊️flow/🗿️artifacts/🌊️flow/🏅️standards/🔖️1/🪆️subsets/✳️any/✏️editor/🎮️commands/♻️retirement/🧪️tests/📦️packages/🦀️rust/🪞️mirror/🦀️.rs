//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        pub use semio_s_artifact_flow_flow_editor::editor::flow::*;
        #[path = "."]
        pub mod retained {
            pub use semio_s_artifact_flow_flow_editor::editor::flow::retained::*;
            #[cfg(test)]
            #[path = "../../../../../../🧵️retained/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[cfg(test)]
        #[path = "../../../🦀️.rs"]
        pub mod original_command_retirement_tests;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::editor_vocabulary;
        pub use semio_s_artifact_flow_flow_mutations::editor::flow::host;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_flow_flow_inferences::editor::flow::presence::*;
            #[path = "."]
            pub mod retirement {
                pub use semio_s_artifact_flow_flow_inferences::editor::flow::presence::retirement::*;
                #[cfg(test)]
                #[path="../../../../../../👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
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
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::windows::main::*;
                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_flow_flow_config::editor::flow::modes::edit::windows::main::config::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs"]
                            pub mod tests;
                        }
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
        #[cfg(test)]
        #[path = "../../../../../../🧵️retained/🔎️wire/🧪️tests/🔎️wire/🦀️.rs"]
        pub mod scalar_host_wire_tests;
        #[cfg(test)]
        #[path="../../../../../../🧪️tests/📨️child-preparation/🦀️.rs"]
        pub mod child_preparation_owner_tests;
        #[cfg(test)]
        #[path="../../../../../../🧪️tests/🎟️contributions/🦀️.rs"]
        pub mod contributions_receiving_tests;
    }
}
