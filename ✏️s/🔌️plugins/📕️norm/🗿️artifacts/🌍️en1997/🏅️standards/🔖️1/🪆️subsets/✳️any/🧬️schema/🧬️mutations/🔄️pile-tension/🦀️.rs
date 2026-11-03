//! 🔄️ EN 1997 pile mutation leaves: tension permanent, tension variable.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔄️change-pile-tension-permanent/🦀️.rs"]
pub mod change_pile_tension_permanent;
#[path = "../🔄️change-pile-tension-variable/🦀️.rs"]
pub mod change_pile_tension_variable;
#[path = "../🔄️change-pile-drop-height/🦀️.rs"]
pub mod change_pile_drop_height;
#[path = "../🔄️change-pile-set-per-blow/🦀️.rs"]
pub mod change_pile_set_per_blow;
#[path = "../🔄️change-pile-elastic-compression/🦀️.rs"]
pub mod change_pile_elastic_compression;
#[path = "../🔄️change-pile-hammer-efficiency/🦀️.rs"]
pub mod change_pile_hammer_efficiency;
//#endregion 🪢️Mounts
