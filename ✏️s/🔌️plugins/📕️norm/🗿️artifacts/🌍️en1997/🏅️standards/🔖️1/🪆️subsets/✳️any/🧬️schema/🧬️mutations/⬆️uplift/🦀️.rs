//! ⬆️ EN 1997 uplift case mutation leaves: permanent stabilizing, permanent destabilizing, variable destabilizing and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../⬆️change-uplift-perm-stabilizing/🦀️.rs"]
pub mod change_uplift_perm_stabilizing;
#[path = "../⬆️change-uplift-perm-destabilizing/🦀️.rs"]
pub mod change_uplift_perm_destabilizing;
#[path = "../⬆️change-uplift-var-destabilizing/🦀️.rs"]
pub mod change_uplift_var_destabilizing;
#[path = "../⬆️change-uplift-pore-pressure/🦀️.rs"]
pub mod change_uplift_pore_pressure;
#[path = "../⬆️change-uplift-total-stress/🦀️.rs"]
pub mod change_uplift_total_stress;
//#endregion 🪢️Mounts
