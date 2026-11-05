//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../➖️remove-joint/🦀️.rs"]
pub mod remove_joint;
#[path = "../➕️insert-tie-beam/🦀️.rs"]
pub mod insert_tie_beam;
#[path = "../➖️remove-tie-beam/🦀️.rs"]
pub mod remove_tie_beam;
#[path = "../➕️insert-seismic-slope/🦀️.rs"]
pub mod insert_seismic_slope;
#[path = "../➖️remove-seismic-slope/🦀️.rs"]
pub mod remove_seismic_slope;
#[path = "../➕️insert-joint/🦀️.rs"]
pub mod insert_joint;
//#endregion 🪢️Mounts
