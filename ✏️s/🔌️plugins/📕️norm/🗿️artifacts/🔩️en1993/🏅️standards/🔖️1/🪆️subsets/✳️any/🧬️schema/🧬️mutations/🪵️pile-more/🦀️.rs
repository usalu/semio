//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🪵️change-pile-shaft-perimeter/🦀️.rs"]
pub mod change_pile_shaft_perimeter;
#[path = "../🪵️change-pile-environment/🦀️.rs"]
pub mod change_pile_environment;
#[path = "../🪵️change-pile-design-life/🦀️.rs"]
pub mod change_pile_design_life;
#[path = "../🪵️change-pile-wall-thickness/🦀️.rs"]
pub mod change_pile_wall_thickness;
//#endregion 🪢️Mounts
