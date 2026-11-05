//! 🏗️ EN 1990 members vibration mutation leaves: member sls limit char, member sls limit freq, member sls limit qp and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏗️change-member-sls-limit-char/🦀️.rs"]
pub mod change_member_sls_limit_char;
#[path = "../🏗️change-member-sls-limit-freq/🦀️.rs"]
pub mod change_member_sls_limit_freq;
#[path = "../🏗️change-member-sls-limit-qp/🦀️.rs"]
pub mod change_member_sls_limit_qp;
#[path = "../🏗️change-member-vibration-frequency/🦀️.rs"]
pub mod change_member_vibration_frequency;
#[path = "../🏗️change-member-vib-freq-min/🦀️.rs"]
pub mod change_member_vib_freq_min;
//#endregion 🪢️Mounts
