//! 🧬️ mutations of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
pub use crate::standards::v1::subsets::any::schema::diff::FlowDiff;
pub use semio_s_artifact_flow_flow_schema::{FLOW_DOCUMENT_SCHEMA, FlowWorkingScene, flow_content_node_from_working, flow_content_snapshot_from_working};

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
                    pub use semio_s_artifact_flow_flow_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "../../../../../../../../♻️retirement/🦀️.rs"]
                    pub mod retirement;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::text::*;
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_flow_flow_tests::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
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
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::editor_vocabulary;
        #[path = "../../../../✏️editor/🖥️host/🦀️.rs"]
        pub mod host;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::edit_rules;
        pub use semio_s_artifact_flow_flow_inferences::editor::flow::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../✏️editor/🎮️commands/➕️add-widget/🦀️.rs"]
            pub mod add_widget;
            #[path = "../../../../✏️editor/🎮️commands/🖱️context-menu-at/🦀️.rs"]
            pub mod context_menu_at;
            #[path = "../../../../✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs"]
            pub mod duplicate_widget;
            #[path = "../../../../✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs"]
            pub mod focus_selection;
            #[path = "../../../../✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "../../../../✏️editor/🎮️commands/🔦️open-spotlight/🦀️.rs"]
            pub mod open_spotlight;
            #[path = "../../../../✏️editor/🎮️commands/🩹️patch-flow-widgets/🦀️.rs"]
            pub mod patch_flow_widgets;
            #[path = "../../../../✏️editor/🎮️commands/🖼️replace-image/🦀️.rs"]
            pub mod replace_image;
            #[path = "../../../../✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../../../✏️editor/🎮️commands/🛍️set-catalogue-sections/🦀️.rs"]
            pub mod set_catalogue_sections;
            #[path = "../../../../✏️editor/🎮️commands/📏️set-grid-factor/🦀️.rs"]
            pub mod set_grid_factor;
            #[path = "../../../../✏️editor/🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "../../../../✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "../../../../✏️editor/🎮️commands/🔬️set-lod-mode/🦀️.rs"]
            pub mod set_lod_mode;
            #[path = "../../../../✏️editor/🎮️commands/🙈️set-preview-off/🦀️.rs"]
            pub mod set_preview_off;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_flow_flow_config::editor::flow::modes::edit::windows::main::config;
                        #[path = "."]
                        pub mod options {
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🌐️grid/🦀️.rs"]
                            pub mod grid;
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🔭️lod/🦀️.rs"]
                            pub mod lod;
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/📏️proximity/🦀️.rs"]
                            pub mod proximity;
                        }
                    }
                }
            }
        }
    }
}
