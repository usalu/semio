//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.


#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod gestures {
            #[path = "../../../🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod overlay {
                #[cfg(test)]
                #[path = "../../../🫧️overlay/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod properties {
                #[path = "../../../../📌️panels/🔍️properties/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod families {
                #[path = "../../../../📌️panels/🪑️families/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod classification {
                #[path = "../../../../📌️panels/🗂️classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod delete_selection {
                #[path = "../../../../🎮️commands/🗑️delete-selection/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_commit_draft {
                #[path = "../../../../🎮️commands/✅️canvas-commit-draft/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_escape {
                #[path = "../../../../🎮️commands/🚪️canvas-escape/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod world_pointer_down {
                #[path = "../../../../🎮️commands/🌍️world-pointer-down/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod world_pointer_move {
                #[path = "../../../../🎮️commands/🌐️world-pointer-move/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cursor_keys {
                #[path = "../../../../🎮️commands/🧭️cursor-keys/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod engagement_input {
                #[path = "../../../../🎮️commands/⌨️engagement-input/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod gesture_keys {
                #[path = "../../../../🎮️commands/🔑️gesture-keys/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod engagement_submit {
                #[path = "../../../../🎮️commands/📨️engagement-submit/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
