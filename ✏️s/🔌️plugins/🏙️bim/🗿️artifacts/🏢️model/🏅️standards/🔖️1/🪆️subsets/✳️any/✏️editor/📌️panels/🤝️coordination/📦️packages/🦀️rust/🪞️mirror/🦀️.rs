//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.



#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod gestures {
            #[path = "../../../../../🧵️gestures/📏️annotate/🦀️.rs"]
            pub mod annotate;
            #[path = "../../../../../🧵️gestures/🧱️chain/🦀️.rs"]
            pub mod chain;
            #[path = "../../../../../🧵️gestures/🦴️structure/🦀️.rs"]
            pub mod structure;
            #[path = "../../../../../🧵️gestures/🪟️opening/🦀️.rs"]
            pub mod opening;
            #[path = "../../../../../🧵️gestures/📍️point/🦀️.rs"]
            pub mod point;
            #[path = "../../../../../🧵️gestures/🧷️sweep/🦀️.rs"]
            pub mod sweep;
            #[path = "../../../../../🧵️gestures/🚚️transform/🦀️.rs"]
            pub mod transform;
            #[path = "../../../../../🧵️gestures/⌨️typed/🦀️.rs"]
            pub mod typed;
            #[path = "../../../../../🧵️gestures/🖼️viewports/🦀️.rs"]
            pub mod viewports;
        }
        
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod outliner {
                #[path = "."]
                pub mod views {
                    #[cfg(test)]
                    #[path = "../../../../🌳️outliner/🖼️views/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
            #[path = "."]
            pub mod diagnostics {
                #[cfg(test)]
                #[path = "../../../../🚨️diagnostics/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod coordination {
                #[path = "../../../🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod create_entity {
                #[path = "../../../../../🎮️commands/🏗️create-entity/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod create_view {
                #[path = "../../../../../🎮️commands/🔭️create-view/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod rename_entity {
                #[path = "../../../../../🎮️commands/🏷️rename-entity/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_field {
                #[path = "../../../../../🎮️commands/🩹️set-field/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_view {
                #[path = "../../../../../🎮️commands/🪟️set-view/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_family {
                #[path = "../../../../../🎮️commands/🧬️edit-family/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_schedule {
                #[path = "../../../../../🎮️commands/📋️edit-schedule/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_camera {
                #[path = "../../../../../🎮️commands/🎥️set-camera/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod arm_utility {
                #[path = "../../../../../🎮️commands/🛠️arm-utility/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod flip_walls {
                #[path = "../../../../../🎮️commands/🔃️flip-walls/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod attach_walls {
                #[path = "../../../../../🎮️commands/🔗️attach-walls/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
