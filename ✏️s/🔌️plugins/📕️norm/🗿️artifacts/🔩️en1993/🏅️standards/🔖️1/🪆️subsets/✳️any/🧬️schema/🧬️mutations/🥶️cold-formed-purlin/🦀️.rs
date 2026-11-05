//! 🥶️ EN 1993 cold-formed member mutation leaves: purlin span, purlin load, purlin lateral ratio.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🥶️change-cf-purlin-span/🦀️.rs"]
pub mod change_cf_purlin_span;
#[path = "../🥶️change-cf-purlin-load/🦀️.rs"]
pub mod change_cf_purlin_load;
#[path = "../🥶️change-cf-purlin-lateral-ratio/🦀️.rs"]
pub mod change_cf_purlin_lateral_ratio;
//#endregion 🪢️Mounts
