//! 📇️ ISO 16757 catalogue mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "."]
pub mod change_exchange_process {
    #[path = "../🔄️change-exchange-process/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔄️change-exchange-process/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🔄️change-exchange-process/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_script_limits {
    #[path = "../🚦️change-script-limits/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🚦️change-script-limits/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🚦️change-script-limits/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod rename_catalogue {
    #[path = "../📇️rename-catalogue/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📇️rename-catalogue/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../📇️rename-catalogue/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod rename_manufacturer {
    #[path = "../🏭️rename-manufacturer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🏭️rename-manufacturer/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "../🏭️rename-manufacturer/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🪢️Mounts
