//! 🔩 EN 1992 reinforcement grades mutation leaves: reinforcement ductility, reinforcement e s, reinforcement eps uk and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩️change-reinforcement-name/🦀️.rs"]
pub mod change_reinforcement_name;
#[path = "../🔩️change-reinforcement-es/🦀️.rs"]
pub mod change_reinforcement_e_s;
#[path = "../🔩️change-reinforcement-ductility/🦀️.rs"]
pub mod change_reinforcement_ductility;
#[path = "../🔩️change-reinforcement-k/🦀️.rs"]
pub mod change_reinforcement_k;
#[path = "../🔩️change-reinforcement-eps-uk/🦀️.rs"]
pub mod change_reinforcement_eps_uk;
//#endregion 🪢️Mounts
