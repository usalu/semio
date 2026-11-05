//! 🚰 EN 1992 member-field mutation leaves: liquid-retaining and bridge stress fields.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../💦change-member-liquid-sigma-s/🦀️.rs"]
pub mod change_member_liquid_sigma_s;
#[path = "../🫧change-member-liquid-rho-p-eff/🦀️.rs"]
pub mod change_member_liquid_rho_p_eff;
#[path = "../🧊change-member-liquid-f-ct-eff/🦀️.rs"]
pub mod change_member_liquid_f_ct_eff;
#[path = "../🌊change-member-liquid-sr-max/🦀️.rs"]
pub mod change_member_liquid_s_r_max;
#[path = "../🌁change-member-bridge-sigma-c/🦀️.rs"]
pub mod change_member_bridge_sigma_c;
#[path = "../🎢change-member-bridge-delta-sigma-s/🦀️.rs"]
pub mod change_member_bridge_delta_sigma_s;
//#endregion 🪢️Mounts
