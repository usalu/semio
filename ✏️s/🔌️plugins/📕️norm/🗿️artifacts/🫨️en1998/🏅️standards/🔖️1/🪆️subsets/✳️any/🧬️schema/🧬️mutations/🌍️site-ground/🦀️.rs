//! 🌍️ EN 1998 site ground classification mutation leaves: N_SPT, c_u, soft clay, liquefiable and alluvium indicators.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../✏️change-site-n-spt/🦀️.rs"]
pub mod change_site_n_spt;
#[path = "../✏️change-site-cu-kpa/🦀️.rs"]
pub mod change_site_c_u_kpa;
#[path = "../✏️change-site-soft-clay-thickness-m/🦀️.rs"]
pub mod change_site_soft_clay_thickness_m;
#[path = "../✏️change-site-soft-clay-pi/🦀️.rs"]
pub mod change_site_soft_clay_pi;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_site_ground_more::{change_site_liquefiable_layer, change_site_alluvium_thickness_m, change_site_alluvium_v_s_m_s, change_site_below_alluvium_v_s_m_s};
