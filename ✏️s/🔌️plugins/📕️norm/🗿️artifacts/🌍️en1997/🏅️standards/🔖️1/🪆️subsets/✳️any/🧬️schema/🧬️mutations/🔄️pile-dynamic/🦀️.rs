//! 🔄️ EN 1997 pile mutation leaves: elastic modulus, settlement, negative friction and dynamic tests.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔄️change-pile-elastic-modulus/🦀️.rs"]
pub mod change_pile_elastic_modulus;
#[path = "../🔄️change-pile-settling-thickness/🦀️.rs"]
pub mod change_pile_settling_thickness;
#[path = "../🔄️change-pile-neg-friction-beta/🦀️.rs"]
pub mod change_pile_neg_friction_beta;
#[path = "../🔄️change-pile-dynamic-test-count/🦀️.rs"]
pub mod change_pile_dynamic_test_count;
#[path = "../🔄️change-pile-dynamic-res-mean/🦀️.rs"]
pub mod change_pile_dynamic_res_mean;
#[path = "../🔄️change-pile-dynamic-res-min/🦀️.rs"]
pub mod change_pile_dynamic_res_min;
#[path = "../🔄️change-pile-hammer-weight/🦀️.rs"]
pub mod change_pile_hammer_weight;
//#endregion 🪢️Mounts
