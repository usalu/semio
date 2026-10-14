//! 🪜️ Process3d step mutations: create, delete, rename, reorder, enable, re-origin and re-measure steps.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[path = "."]
pub mod create_step {
    #[path = "../🌱create-step/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🌱create-step/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌱create-step/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_step {
    #[path = "../🗑️delete-step/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🗑️delete-step/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗑️delete-step/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod rename_step {
    #[path = "../🏷️rename-step/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🏷️rename-step/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏷️rename-step/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_step_enabled {
    #[path = "../🔘change-step-enabled/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔘change-step-enabled/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔘change-step-enabled/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_step_origin {
    #[path = "../🧷change-step-origin/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🧷change-step-origin/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧷change-step-origin/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod replace_step_measure {
    #[path = "../📐replace-step-measure/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📐replace-step-measure/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📐replace-step-measure/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod reorder_steps {
    #[path = "../🔀reorder-steps/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔀reorder-steps/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔀reorder-steps/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
