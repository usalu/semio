//! 🏗️ EN 1990 members deflection mutation leaves: member defl limit ratio, member defl limit ratio w3, member deflection w1 and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-deflection-w1/🦀️.rs"]
pub mod change_member_deflection_w1;
#[path = "../🏗️change-member-deflection-w2/🦀️.rs"]
pub mod change_member_deflection_w2;
#[path = "../🏗️change-member-deflection-w3/🦀️.rs"]
pub mod change_member_deflection_w3;
#[path = "../🏗️change-member-defl-limit-ratio/🦀️.rs"]
pub mod change_member_defl_limit_ratio;
#[path = "../🏗️change-member-defl-limit-ratio-w3/🦀️.rs"]
pub mod change_member_defl_limit_ratio_w3;
//#endregion 🪢️Mounts
