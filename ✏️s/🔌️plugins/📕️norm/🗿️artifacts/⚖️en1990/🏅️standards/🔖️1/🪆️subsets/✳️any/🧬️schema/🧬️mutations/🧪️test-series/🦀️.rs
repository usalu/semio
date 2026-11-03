//! 🧪 EN 1990 test series mutation leaves: test conversion factor, test distribution, test mean and more.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧪️change-test-quantity/🦀️.rs"]
pub mod change_test_quantity;
#[path = "../🧪️change-test-pairs/🦀️.rs"]
pub mod change_test_pairs;
#[path = "../🧪️change-test-distribution/🦀️.rs"]
pub mod change_test_distribution;
#[path = "../🧪️change-test-sample-size/🦀️.rs"]
pub mod change_test_sample_size;
#[path = "../🧪️change-test-mean/🦀️.rs"]
pub mod change_test_mean;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1990_mut_test_series_more::{change_test_std_dev, change_test_variation_known, change_test_conversion_factor, change_test_required, change_test_theory_variation};
