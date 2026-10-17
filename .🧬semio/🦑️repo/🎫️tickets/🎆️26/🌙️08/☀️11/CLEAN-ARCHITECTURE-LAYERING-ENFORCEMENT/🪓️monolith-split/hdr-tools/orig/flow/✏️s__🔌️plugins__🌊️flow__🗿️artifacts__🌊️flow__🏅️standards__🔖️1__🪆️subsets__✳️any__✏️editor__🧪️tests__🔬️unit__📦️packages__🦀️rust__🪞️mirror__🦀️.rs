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
pub use semio_s_artifact_flow_flow_tests::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        pub use semio_s_artifact_flow_flow_editor::editor::flow::*;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::base;
        pub use semio_s_artifact_flow_flow_mutations::editor::flow::host;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_flow_flow_inferences::editor::flow::presence::*;
        }
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::edit_rules;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod add_widget {
                pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::add_widget::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/➕️add-widget/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod connect_media_ports {
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::connect_media_ports::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🔗️connect-media-ports/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::context_menu_at;
            #[path = "."]
            pub mod delete_selection {
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::delete_selection::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod disconnect {
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::disconnect::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/✂️disconnect/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::duplicate_widget;
            #[path = "."]
            pub mod evaluate {
                pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::evaluate::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🧮️evaluate/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::flow_eval_resolve;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::flow_eval_tick;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::focus_selection;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::move_media_node;
            #[path = "."]
            pub mod node_graph_edit {
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::node_graph_edit::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod node_graph_viewport {
                pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::node_graph_viewport::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🔭️node-graph-viewport/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod unit;
            }
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::open_spotlight;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::patch_flow_widgets;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::remove_widget;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::rename_flow_widget;
            #[path = "."]
            pub mod reorganize {
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::reorganize::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🗺️reorganize/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::replace_image;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::run_extension_action;
            #[path = "."]
            pub mod set_active_example {
                pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_active_example::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod set_catalogue_sections {
                pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_catalogue_sections::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🛍️set-catalogue-sections/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod unit;
            }
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_factor;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_snap_enabled;
            #[path = "."]
            pub mod set_grid_visible {
                pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_grid_visible::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/👁️set-grid-visible/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod unit;
            }
            #[path = "."]
            pub mod set_lod_mode {
                pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_lod_mode::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🔬️set-lod-mode/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod unit;
            }
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::set_preview_off;
            pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::set_proximity_distance;
            pub use semio_s_artifact_flow_flow_viewer::editor::flow::commands::spotlight_commit;
            #[path = "."]
            pub mod toggle_extension {
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::commands::toggle_extension::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🔌️toggle-extension/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod unit;
            }
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
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🌊️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        pub use semio_s_artifact_flow_flow_config::editor::flow::modes::edit::windows::main::config;
                        pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::edit::windows::main::transient;
                    }
                    #[path = "."]
                    pub mod compiled {
                        pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::edit::windows::compiled::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🗣️compiled/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
            #[path = "."]
            pub mod generate {
                pub use semio_s_artifact_flow_flow_viewer::editor::flow::modes::generate::*;
                #[path = "."]
                pub mod commands {
                    #[path = "."]
                    pub mod add_generation {
                        pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::add_generation::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/🧬️generate/🎮️commands/➕️add-generation/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod unit;
                    }
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::remove_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::rename_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::select_generation;
                    pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::commands::update_generation_values;
                }
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod form {
                        pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::modes::generate::windows::form::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod generations {
                        pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::generate::windows::generations::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/🧬️generate/🪟️windows/🗂️generations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod preview {
                        pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::generate::windows::preview::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/🧬️generate/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod catalogue {
                pub use semio_s_artifact_flow_flow_viewer::editor::flow::panels::catalogue::*;
                #[cfg(test)]
                #[path = "../../../../../📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod document {
                pub use semio_s_artifact_flow_flow_editor_leaves::editor::flow::panels::document::*;
                #[cfg(test)]
                #[path = "../../../../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_flow_flow_tests::editor::flow::panels::inspection::*;
                #[cfg(test)]
                #[path = "../../../../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        #[cfg(test)]
        #[path = "../../../🦀️.rs"]
        pub mod unit_tests;
        #[cfg(test)]
        #[path = "../../../../🔬️interactive-job/🦀️.rs"]
        pub mod interactive_job_tests;
        #[cfg(test)]
        #[path = "../../../../⚖️declared-verbs/🦀️.rs"]
        pub mod declared_verb_laws;
    }
}
