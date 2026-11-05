//! 🔷️ EN 1999 outline vertex mutation leaves: section id, ring, element id and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔷️change-vertex-section-id/🦀️.rs"]
pub mod change_vertex_section_id;
#[path = "../🔷️change-vertex-ring/🦀️.rs"]
pub mod change_vertex_ring;
#[path = "../🔷️change-vertex-element-id/🦀️.rs"]
pub mod change_vertex_element_id;
#[path = "../🔷️change-vertex-y/🦀️.rs"]
pub mod change_vertex_y;
#[path = "../🔷️change-vertex-z/🦀️.rs"]
pub mod change_vertex_z;
//#endregion 🪢️Mounts
