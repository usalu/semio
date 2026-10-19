//! 🥣️ Companion test crate of `semio-s-artifact-layout-layout-editor-commands`: the unit laws of the layout editor commands, modes, windows and canvas. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod add_frame {
    #[path = "../../🖼️add-frame/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod canvas {
    #[path = "../../../🖼️canvas/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod canvas_drop {
    #[path = "../../📥️canvas-drop/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod canvas_pointer_down {
    #[path = "../../👇️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod delete_selection {
    #[path = "../../🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod export_pdf {
    #[path = "../../📕️export-pdf/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod gumball {
    #[path = "../../🧭️gumball/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod modes {
    #[path = "."]
    pub mod edit {
        #[path = "../../../🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod windows {
            #[path = "."]
            pub mod blueprint {
                #[path = "."]
                pub mod config {
                    #[path = "../../../🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🧪️tests/🔬️window/🦀️.rs"]
                    pub mod tests;
                }
                #[path = "../../../🎭️modes/✏️edit/🪟️windows/📐️blueprint/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
                #[path = "."]
                pub mod transform {
                    #[path = "../../../🎭️modes/✏️edit/🪟️windows/📐️blueprint/🪛️utilities/🔄️transform/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
            #[path = "."]
            pub mod preview {
                #[path = "../../../🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
#[path = "."]
pub mod patch_document {
    #[path = "../../📄️patch-document/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
