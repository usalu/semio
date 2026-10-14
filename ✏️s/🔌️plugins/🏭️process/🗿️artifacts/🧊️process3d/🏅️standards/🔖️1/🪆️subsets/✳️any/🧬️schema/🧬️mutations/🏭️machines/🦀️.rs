//! 🏭️ Process3d workshop machine mutations: create, delete, rename, re-icon and re-capability machines.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[path = "."]
pub mod create_machine {
    #[path = "../🏭create-machine/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🏭create-machine/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏭create-machine/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_machine {
    #[path = "../❌delete-machine/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../❌delete-machine/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../❌delete-machine/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod rename_machine {
    #[path = "../🔖rename-machine/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔖rename-machine/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔖rename-machine/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_machine_icon {
    #[path = "../🎨change-machine-icon/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🎨change-machine-icon/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎨change-machine-icon/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod replace_machine_capabilities {
    #[path = "../🔁replace-machine/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔁replace-machine/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔁replace-machine/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
