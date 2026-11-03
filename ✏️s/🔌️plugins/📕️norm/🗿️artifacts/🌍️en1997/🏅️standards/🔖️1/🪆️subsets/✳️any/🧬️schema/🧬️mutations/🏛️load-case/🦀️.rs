//! 🏛️ EN 1997 footing load case mutation leaves: design situation, vertical permanent, vertical variable and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏛️change-load-case-design-situation/🦀️.rs"]
pub mod change_load_case_design_situation;
#[path = "../🏛️change-load-case-vertical-perm/🦀️.rs"]
pub mod change_load_case_vertical_perm;
#[path = "../🏛️change-load-case-vertical-var/🦀️.rs"]
pub mod change_load_case_vertical_var;
#[path = "../🏛️change-load-case-horiz-permanent/🦀️.rs"]
pub mod change_load_case_horiz_permanent;
#[path = "../🏛️change-load-case-horiz-variable/🦀️.rs"]
pub mod change_load_case_horiz_variable;
#[path = "../🏛️change-load-case-moment-permanent/🦀️.rs"]
pub mod change_load_case_moment_permanent;
#[path = "../🏛️change-load-case-moment-variable/🦀️.rs"]
pub mod change_load_case_moment_variable;
//#endregion 🪢️Mounts
