//! 🏗️ EN 1997 pile group mutation leaves: single resistance, shaft cohesion sum, base cohesion.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-pile-group-single-res/🦀️.rs"]
pub mod change_pile_group_single_res;
#[path = "../🏗️change-pile-group-shaft-coh-sum/🦀️.rs"]
pub mod change_pile_group_shaft_coh_sum;
#[path = "../🏗️change-pile-group-base-cohesion/🦀️.rs"]
pub mod change_pile_group_base_cohesion;
//#endregion 🪢️Mounts
