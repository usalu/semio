//! 🧱️ EN 1998 members mutation leaves: assessment.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🏋️change-assessment-rkn/🦀️.rs"]
pub mod change_assessment_rkn;
#[path = "../🔧insert-assessment/🦀️.rs"]
pub mod insert_assessment;
#[path = "../➖️remove-assessment/🦀️.rs"]
pub mod remove_assessment;
#[path = "../✏️change-assessment-knowledge-level/🦀️.rs"]
pub mod change_assessment_knowledge_level;
#[path = "../✏️change-assessment-limit-state/🦀️.rs"]
pub mod change_assessment_limit_state;
#[path = "../✏️change-assessment-supported-building-id/🦀️.rs"]
pub mod change_assessment_supported_building_id;
#[path = "../✏️change-assessment-gamma-el/🦀️.rs"]
pub mod change_assessment_gamma_el;
//#endregion 🪢️Mounts
