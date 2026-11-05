//! 🏗️ EN 1997 pile group mutation leaves: pile id, rows, columns and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-pile-group-pile-id/🦀️.rs"]
pub mod change_pile_group_pile_id;
#[path = "../🏗️change-pile-group-rows/🦀️.rs"]
pub mod change_pile_group_rows;
#[path = "../🏗️change-pile-group-columns/🦀️.rs"]
pub mod change_pile_group_columns;
#[path = "../🏗️change-pile-group-spacing/🦀️.rs"]
pub mod change_pile_group_spacing;
#[path = "../🏗️change-pile-group-cap-vertical/🦀️.rs"]
pub mod change_pile_group_cap_vertical;
#[path = "../🏗️change-pile-group-cap-moment-x/🦀️.rs"]
pub mod change_pile_group_cap_moment_x;
#[path = "../🏗️change-pile-group-cap-moment-y/🦀️.rs"]
pub mod change_pile_group_cap_moment_y;
//#endregion 🪢️Mounts
