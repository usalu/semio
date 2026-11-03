//! more mutation leaves

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-conn-bolts-material/🦀️.rs"]
pub mod change_conn_bolts_material;
#[path = "../🔗️change-conn-bolts-diameter/🦀️.rs"]
pub mod change_conn_bolts_diameter;
#[path = "../🔗️change-conn-bolts-edge-distance/🦀️.rs"]
pub mod change_conn_bolts_edge_distance;
#[path = "../🔗️change-conn-bolts-pitch/🦀️.rs"]
pub mod change_conn_bolts_pitch;
//#endregion 🪢️Mounts
