//! 🏭️ EN 1998 silos-tanks mutation leaves: silo, tank.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-silo-filling-ratio/🦀️.rs"]
pub mod change_silo_filling_ratio;
#[path = "../✏️change-silo-n-rd-n/🦀️.rs"]
pub mod change_silo_n_rd_n;
#[path = "../✏️change-silo-v-rd-n/🦀️.rs"]
pub mod change_silo_v_rd_n;
#[path = "../✏️change-silo-q-nominal/🦀️.rs"]
pub mod change_silo_q_nominal;
#[path = "../✏️change-tank-filling-ratio/🦀️.rs"]
pub mod change_tank_filling_ratio;
#[path = "../✏️change-tank-v-rd-n/🦀️.rs"]
pub mod change_tank_v_rd_n;
//#endregion 🪢️Mounts
