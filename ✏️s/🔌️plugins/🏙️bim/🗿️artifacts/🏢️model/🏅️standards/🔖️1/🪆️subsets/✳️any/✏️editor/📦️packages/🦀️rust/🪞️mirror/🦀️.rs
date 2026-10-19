//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.



#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod gestures {
            #[path = "."]
            pub mod split {
                #[cfg(test)]
                #[path = "../../../🧵️gestures/🔪️split/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod properties {
                #[path = "."]
                pub mod data {
                    #[cfg(test)]
                    #[path = "../../../📌️panels/🔍️properties/🏷️data/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod export_schedule_csv {
                #[path = "../../../🎮️commands/📊️export-schedule-csv/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod analyse_model {
                #[path = "../../../🎮️commands/🔎️analyse-model/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod export_model {
                #[path = "../../../🎮️commands/📤️export-model/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod export_sheets {
                #[path = "../../../🎮️commands/📄️export-sheets/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_pointer_down {
                #[path = "../../../🎮️commands/🖱️canvas-pointer-down/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_pointer_move {
                #[path = "../../../🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_pointer_up {
                #[path = "../../../🎮️commands/⬆️canvas-pointer-up/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_double_click {
                #[path = "../../../🎮️commands/👆️canvas-double-click/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
