//! 🏠️ DIN 4108 building mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../💨️change-airtightness-n50/🦀️.rs"]
pub mod change_airtightness_n50;
#[path = "../✅️change-bb2-details-conform/🦀️.rs"]
pub mod change_bb2_details_conform;
#[path = "../🌦️change-climate-zone/🦀️.rs"]
pub mod change_climate_zone;
#[path = "../💨change-has-mechanical-ventilation/🦀️.rs"]
pub mod change_has_mechanical_ventilation;
#[path = "../💧️change-rh-int/🦀️.rs"]
pub mod change_rh_int;
#[path = "../🌡️change-t-int-c/🦀️.rs"]
pub mod change_t_int_c;
#[path = "../🗂️change-usage/🦀️.rs"]
pub mod change_usage;
#[path = "../➕️insert-case/🦀️.rs"]
pub mod insert_case;
#[path = "../➖️remove-case/🦀️.rs"]
pub mod remove_case;
#[path = "../🔢️change-case-number/🦀️.rs"]
pub mod change_case_number;
#[path = "../🔤️change-case-text/🦀️.rs"]
pub mod change_case_text;
#[path = "../🏷️change-case-label-en/🦀️.rs"]
pub mod change_case_label_en;
#[path = "../🏷️change-case-label-de/🦀️.rs"]
pub mod change_case_label_de;
#[path = "../📇️change-case-entry-id/🦀️.rs"]
pub mod change_case_entry_id;
//#endregion 🪢️Mounts
