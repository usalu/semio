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
                    pub use semio_s_artifact_vcs_vcs_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod add_tag {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::inverse;
                        }
                        #[path = "."]
                        pub mod rename_vcs {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::inverse;
                        }
                        #[path = "."]
                        pub mod change_notes {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::inverse;
                        }
                        #[path = "."]
                        pub mod change_counter {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::inverse;
                        }
                        #[path = "."]
                        pub mod remove_tag {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::inverse;
                        }
                        #[path = "."]
                        pub mod change_status {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_status::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_status::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_status::inverse;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_vcs_vcs_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod vcs {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::config::*;
            pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::config::schema;
        }
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::presence::*;
        }
        pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../🎮️commands/👇️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "../../../🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
            pub mod canvas_pointer_move;
            #[path = "../../../🎮️commands/👆️canvas-pointer-up/🦀️.rs"]
            pub mod canvas_pointer_up;
            #[path = "../../../🎮️commands/🛞️canvas-wheel/🦀️.rs"]
            pub mod canvas_wheel;
            #[path = "../../../🎮️commands/🩹️edit/🦀️.rs"]
            pub mod edit;
            #[path = "../../../🎮️commands/📚️example/🦀️.rs"]
            pub mod example;
            #[path = "../../../🎮️commands/📈️increment-counter/🦀️.rs"]
            pub mod increment_counter;
            #[path = "../../../🎮️commands/⏸️no-operation/🦀️.rs"]
            pub mod no_operation;
            #[path = "../../../🎮️commands/🏷️rename-vcs/🦀️.rs"]
            pub mod rename_vcs;
            #[path = "../../../🎮️commands/🔢️change-counter/🦀️.rs"]
            pub mod change_counter;
            #[path = "../../../🎮️commands/🚦️change-status/🦀️.rs"]
            pub mod change_status;
            #[path = "../../../🎮️commands/🗒️change-notes/🦀️.rs"]
            pub mod change_notes;
            #[path = "../../../🎮️commands/📝️text-edit/🦀️.rs"]
            pub mod text_edit;

        #[cfg(test)]
        #[path = "../../../🎮️commands/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️.rs"]
                    pub mod editor;
                    #[path = "."]
                    pub mod history {
                        pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::modes::edit::windows::history::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/📜️history/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
