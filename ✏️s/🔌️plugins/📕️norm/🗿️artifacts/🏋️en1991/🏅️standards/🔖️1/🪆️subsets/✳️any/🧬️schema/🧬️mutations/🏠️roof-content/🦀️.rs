//! 🏠 EN 1991 roof content mutation leaves: roof c e, roof c t, roof has parapet and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏠️change-roof-roof-type/🦀️.rs"]
pub mod change_roof_roof_type;
#[path = "../🏠️change-roof-ce/🦀️.rs"]
pub mod change_roof_c_e;
#[path = "../🏠️change-roof-ct/🦀️.rs"]
pub mod change_roof_c_t;
#[path = "../🏠️change-roof-has-parapet/🦀️.rs"]
pub mod change_roof_has_parapet;
#[path = "../🏠️change-roof-multi-span/🦀️.rs"]
pub mod change_roof_multi_span;
//#endregion 🪢️Mounts
