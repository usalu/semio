//! 🪵️ Process3d stock mutations: move, relabel and replace the stock solid.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 🧩️Modules
#[path = "."]
pub mod move_stock {
    #[path = "../📍move-stock/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📍move-stock/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📍move-stock/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod change_stock_label {
    #[path = "../🔤change-stock-label/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔤change-stock-label/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔤change-stock-label/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod replace_stock_solid {
    #[path = "../🧊replace-stock-solid/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🧊replace-stock-solid/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧊replace-stock-solid/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
//#endregion 🧩️Modules
