//! 🧱 EN 1992 grades rows mutation leaves: concrete grade, prestress steel, reinforcement grade and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧱️insert-concrete-grade/🦀️.rs"]
pub mod insert_concrete_grade;
#[path = "../🧱️remove-concrete-grade/🦀️.rs"]
pub mod remove_concrete_grade;
#[path = "../🔩️insert-reinforcement-grade/🦀️.rs"]
pub mod insert_reinforcement_grade;
#[path = "../🔩️remove-reinforcement-grade/🦀️.rs"]
pub mod remove_reinforcement_grade;
#[path = "../🧵️insert-prestress-steel/🦀️.rs"]
pub mod insert_prestress_steel;
#[path = "../🧵️remove-prestress-steel/🦀️.rs"]
pub mod remove_prestress_steel;
//#endregion 🪢️Mounts
