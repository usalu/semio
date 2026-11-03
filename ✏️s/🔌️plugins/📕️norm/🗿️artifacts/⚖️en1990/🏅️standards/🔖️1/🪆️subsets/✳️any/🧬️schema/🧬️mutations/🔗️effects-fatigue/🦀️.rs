//! 🔗 EN 1990 effects fatigue mutation leaves: effect action id, effect influence, effect member id and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔗️change-effect-member-id/🦀️.rs"]
pub mod change_effect_member_id;
#[path = "../🔗️change-effect-action-id/🦀️.rs"]
pub mod change_effect_action_id;
#[path = "../🔗️change-effect-influence/🦀️.rs"]
pub mod change_effect_influence;
#[path = "../🔄️change-fatigue-member-id/🦀️.rs"]
pub mod change_fatigue_member_id;
#[path = "../🔄️change-fatigue-detail-category/🦀️.rs"]
pub mod change_fatigue_detail_category;
#[path = "../🔄️change-fatigue-stress-range/🦀️.rs"]
pub mod change_fatigue_stress_range;
#[path = "../🔄️change-fatigue-cycles/🦀️.rs"]
pub mod change_fatigue_cycles;
//#endregion 🪢️Mounts
