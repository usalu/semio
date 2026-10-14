//! ✏️ `s.fem.fem3d` play app editor: the app, its manifest, view commands, options and the edit mode.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod fem3d {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "../../📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }
        #[path = "../../🌉️wasm/🦀️.rs"]
        pub mod wasm;
        #[path = "."]
        pub mod options {
            #[path = "../../🛠️options/🔄️transform/🦀️.rs"]
            pub mod transform;
        }
        #[path = "../../🛠️window-measures/🦀️.rs"]
        pub mod window_measures;
        #[path = "."]
        pub mod commands {
            #[path = "../../🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../🎮️commands/👁️set-result-display/🦀️.rs"]
            pub mod set_result_display;
            #[path = "../../🎮️commands/⏯️set-result-animation/🦀️.rs"]
            pub mod set_result_animation;
            #[path = "../../🎮️commands/⏱️result-animation-tick/🦀️.rs"]
            pub mod result_animation_tick;
            #[path = "../../🎮️commands/🎯️focus-entity/🦀️.rs"]
            pub mod focus_entity;
            #[path = "../../🎮️commands/🧭️gumball/🦀️.rs"]
            pub mod gumball;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
