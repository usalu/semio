//! 🏭️ EN 1998 silos tanks mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🫙insert-silo/🦀️.rs"]
pub mod insert_silo;
#[path = "../➖️remove-silo/🦀️.rs"]
pub mod remove_silo;
#[path = "../✏️change-silo-height-m/🦀️.rs"]
pub mod change_silo_height_m;
#[path = "../✏️change-silo-radius-m/🦀️.rs"]
pub mod change_silo_radius_m;
#[path = "../✏️change-silo-permanent-gk-n/🦀️.rs"]
pub mod change_silo_permanent_gk_n;
#[path = "../✏️change-silo-content-qk-n/🦀️.rs"]
pub mod change_silo_content_qk_n;
#[path = "../✏️change-silo-content-category/🦀️.rs"]
pub mod change_silo_content_category;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1998_mut_silos_tanks_silo_tank::{change_silo_filling_ratio, change_silo_n_rd_n, change_silo_q_nominal, change_silo_v_rd_n, change_tank_filling_ratio, change_tank_v_rd_n};

pub use semio_s_artifact_norm_en1998_mut_silos_tanks_tank::{change_tank_content_category, change_tank_content_qk_n, change_tank_height_m, change_tank_permanent_gk_n, change_tank_radius_m, insert_tank, remove_tank};

pub use semio_s_artifact_norm_en1998_mut_silos_tanks_shell::{change_tank_shell_t_m, change_tank_shell_str_mpa};
