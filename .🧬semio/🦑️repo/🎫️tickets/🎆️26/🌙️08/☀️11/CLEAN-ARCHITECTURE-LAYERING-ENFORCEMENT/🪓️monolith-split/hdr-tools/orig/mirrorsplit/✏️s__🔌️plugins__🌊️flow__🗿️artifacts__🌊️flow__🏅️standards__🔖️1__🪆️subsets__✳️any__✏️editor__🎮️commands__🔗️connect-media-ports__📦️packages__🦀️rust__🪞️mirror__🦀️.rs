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
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::base;
        pub use semio_s_artifact_flow_flow_mutations::editor::flow::host;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::edit_rules;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../🦀️.rs"]
            pub mod connect_media_ports;
            #[path = "../../../../🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../../../✂️disconnect/🦀️.rs"]
            pub mod disconnect;
            #[path = "../../../../⏱️flow-eval-tick/🦀️.rs"]
            pub mod flow_eval_tick;
            #[path = "../../../../🚚️move-media-node/🦀️.rs"]
            pub mod move_media_node;
            #[path = "../../../../✏️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            pub use semio_s_artifact_flow_flow_mutations::editor::flow::commands::patch_flow_widgets;
            #[path = "../../../../➖️remove-widget/🦀️.rs"]
            pub mod remove_widget;
            #[path = "../../../../🏷️rename-flow-widget/🦀️.rs"]
            pub mod rename_flow_widget;
            #[path = "../../../../🗺️reorganize/🦀️.rs"]
            pub mod reorganize;
            #[path = "../../../../↔️set-proximity-distance/🦀️.rs"]
            pub mod set_proximity_distance;
            #[path = "../../../../🔌️toggle-extension/🦀️.rs"]
            pub mod toggle_extension;
            #[path = "../../../../🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod tools {
                    #[path = "../../../../../🎭️modes/✏️edit/🛠️tools/✋️drag/🦀️.rs"]
                    pub mod drag;
                }
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        pub use semio_s_artifact_flow_flow_config::editor::flow::modes::edit::windows::main::config;
                        pub use semio_s_artifact_flow_flow_tests::editor::flow::modes::edit::windows::main::transient;
                        #[path = "."]
                        pub mod options {
                            #[path = "."]
                            pub mod grid {
                                pub use semio_s_artifact_flow_flow_mutations::editor::flow::modes::edit::windows::main::options::grid::*;
                                #[cfg(test)]
                                #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🌐️grid/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod lod {
                                pub use semio_s_artifact_flow_flow_mutations::editor::flow::modes::edit::windows::main::options::lod::*;
                                #[cfg(test)]
                                #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🔭️lod/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod proximity {
                                pub use semio_s_artifact_flow_flow_mutations::editor::flow::modes::edit::windows::main::options::proximity::*;
                                #[cfg(test)]
                                #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/📏️proximity/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️.rs"]
                    pub mod compiled;
                }
            }
            #[path = "."]
            pub mod generate {
                #[path = "."]
                pub mod commands {
                    #[path = "../../../../../🎭️modes/🧬️generate/🎮️commands/➕️add-generation/🦀️.rs"]
                    pub mod add_generation;
                    #[path = "../../../../../🎭️modes/🧬️generate/🎮️commands/🗑️remove-generation/🦀️.rs"]
                    pub mod remove_generation;
                    #[path = "../../../../../🎭️modes/🧬️generate/🎮️commands/🏷️rename-generation/🦀️.rs"]
                    pub mod rename_generation;
                    #[path = "../../../../../🎭️modes/🧬️generate/🎮️commands/🎯️select-generation/🦀️.rs"]
                    pub mod select_generation;
                    #[path = "../../../../../🎭️modes/🧬️generate/🎮️commands/🎚️update-generation-values/🦀️.rs"]
                    pub mod update_generation_values;
                }
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs"]
                    pub mod form;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod flow {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🌊️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
