//! 🏛️ EN 1992 member shear mutation leaves: member punch col position, member punch column depth, member punch column width and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏛️change-member-stirrups-diameter/🦀️.rs"]
pub mod change_member_stirrups_diameter;
#[path = "../🏛️change-member-stirrups-legs/🦀️.rs"]
pub mod change_member_stirrups_legs;
#[path = "../🏛️change-member-punch-column-width/🦀️.rs"]
pub mod change_member_punch_column_width;
#[path = "../🏛️change-member-punch-column-depth/🦀️.rs"]
pub mod change_member_punch_column_depth;
#[path = "../🏛️change-member-punch-col-position/🦀️.rs"]
pub mod change_member_punch_col_position;
#[path = "../🏛️change-member-punching-asw/🦀️.rs"]
pub mod change_member_punching_asw;
#[path = "../🏛️change-member-punching-beta-method/🦀️.rs"]
pub mod change_member_punching_beta_method;
#[path = "../🏛️change-member-punching-moment-c1/🦀️.rs"]
pub mod change_member_punching_moment_c1;
#[path = "../🏛️change-member-punching-moment-c2/🦀️.rs"]
pub mod change_member_punching_moment_c2;
//#endregion 🪢️Mounts
