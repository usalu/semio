//! 🧪️ EN 1997 laboratory test mutation leaves: layer id, kind, void ratio and more.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🧪️change-lab-test-layer-id/🦀️.rs"]
pub mod change_lab_test_layer_id;
#[path = "../🧪️change-lab-test-kind/🦀️.rs"]
pub mod change_lab_test_kind;
#[path = "../🧪️change-lab-test-void-ratio/🦀️.rs"]
pub mod change_lab_test_void_ratio;
#[path = "../🧪️change-lab-test-preload-stress/🦀️.rs"]
pub mod change_lab_test_preload_stress;
#[path = "../🧪️change-lab-test-drainage-height/🦀️.rs"]
pub mod change_lab_test_drainage_height;
#[path = "../🧪️change-lab-test-consol-time/🦀️.rs"]
pub mod change_lab_test_consol_time;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1997_mut_lab_test_consolidation::{change_lab_test_compression_index, change_lab_test_swelling_index, change_lab_test_stress_increment, change_lab_test_sec_compr_index, change_lab_test_design_life};
