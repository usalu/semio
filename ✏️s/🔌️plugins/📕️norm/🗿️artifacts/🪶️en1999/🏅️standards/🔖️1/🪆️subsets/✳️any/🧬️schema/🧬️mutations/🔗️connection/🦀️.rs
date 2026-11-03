//! 🔗️ EN 1999 connection mutation leaves: bolts bolts per row.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-conn-bolts-bolts-per-row/🦀️.rs"]
pub mod change_conn_bolts_bolts_per_row;
#[path = "../🔗️change-conn-member-id/🦀️.rs"]
pub mod change_conn_member_id;
#[path = "../🔗️change-conn-material-id/🦀️.rs"]
pub mod change_conn_material_id;
#[path = "../🔗️change-conn-kind/🦀️.rs"]
pub mod change_conn_kind;
#[path = "../🔗️change-conn-force-method/🦀️.rs"]
pub mod change_conn_force_method;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1999_mut_connection_more::{change_conn_bolts_material, change_conn_bolts_diameter, change_conn_bolts_edge_distance, change_conn_bolts_pitch};
