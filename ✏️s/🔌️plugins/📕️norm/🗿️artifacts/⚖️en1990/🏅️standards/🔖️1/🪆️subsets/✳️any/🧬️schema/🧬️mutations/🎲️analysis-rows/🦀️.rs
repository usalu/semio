//! 🎲 EN 1990 analysis rows mutation leaves: placement, reliability analysis, test series and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🎲️insert-reliability-analysis/🦀️.rs"]
pub mod insert_reliability_analysis;
#[path = "../🎲️remove-reliability-analysis/🦀️.rs"]
pub mod remove_reliability_analysis;
#[path = "../🧪️insert-test-series/🦀️.rs"]
pub mod insert_test_series;
#[path = "../🧪️remove-test-series/🦀️.rs"]
pub mod remove_test_series;
#[path = "../📍️insert-placement/🦀️.rs"]
pub mod insert_placement;
#[path = "../📍️remove-placement/🦀️.rs"]
pub mod remove_placement;
//#endregion 🪢️Mounts
