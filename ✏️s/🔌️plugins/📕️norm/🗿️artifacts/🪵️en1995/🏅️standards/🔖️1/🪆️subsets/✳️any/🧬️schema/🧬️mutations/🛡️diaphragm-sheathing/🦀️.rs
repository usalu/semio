//! 🛡️ EN 1995 diaphragm-sheathing mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🛡️change-diaphragm-kind/🦀️.rs"]
pub mod change_diaphragm_kind;
#[path = "../🛡️change-diaphragm-method/🦀️.rs"]
pub mod change_diaphragm_method;
#[path = "../🛡️change-diaphragm-panel-width/🦀️.rs"]
pub mod change_diaphragm_panel_width;
#[path = "../🛡️change-diaphragm-wall-height/🦀️.rs"]
pub mod change_diaphragm_wall_height;
#[path = "../🛡️change-diaphragm-fastener-spacing/🦀️.rs"]
pub mod change_diaphragm_fastener_spacing;
#[path = "../🛡️change-diaphragm-panels/🦀️.rs"]
pub mod change_diaphragm_panels;
//#endregion 🪢️Mounts
