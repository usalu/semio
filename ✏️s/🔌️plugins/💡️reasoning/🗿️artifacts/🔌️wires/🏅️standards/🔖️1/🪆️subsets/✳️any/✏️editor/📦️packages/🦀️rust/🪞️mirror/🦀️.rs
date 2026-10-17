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
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_reasoning_wires_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_reasoning_wires_schema::standards::v1::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_reasoning_wires_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_reasoning_wires_snapshot_2::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_reasoning_wires_snapshot_2::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_reasoning_wires_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wires {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod window_transient {
            pub use semio_s_artifact_reasoning_wires_window_transient::editor::wires::window_transient::*;
            #[cfg(test)]
            #[path = "../../../🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        pub use semio_s_artifact_reasoning_wires_inferences::editor::wires::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../🎮️commands/🔵️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "../../../🎮️commands/🔗️add-relationship/🦀️.rs"]
            pub mod add_relationship;
            #[path = "../../../🎮️commands/👇️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "../../../🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
            pub mod canvas_pointer_move;
            #[path = "../../../🎮️commands/👆️canvas-pointer-up/🦀️.rs"]
            pub mod canvas_pointer_up;
            #[path = "../../../🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../../🎮️commands/🎥️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "../../../🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;

        #[cfg(test)]
        #[path = "../../../🎮️commands/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_reasoning_wires_viewer::editor::wires::modes::edit::*;
                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_reasoning_wires_viewer::editor::wires::modes::edit::tools::*;
                    #[path = "."]
                    pub mod reorganize {
                        pub use semio_s_artifact_reasoning_wires_viewer::editor::wires::modes::edit::tools::reorganize::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_reasoning_wires_windows::editor::wires::modes::edit::windows::*;
                    #[path = "."]
                    pub mod canvas {
                        pub use semio_s_artifact_reasoning_wires_windows::editor::wires::modes::edit::windows::canvas::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🕸️canvas/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;

        #[cfg(test)]
        #[path = "../../../📌️panels/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod wires {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_reasoning_wires_viewer::viewer::wires::modes::view::windows;
            }
        }
    }
}
