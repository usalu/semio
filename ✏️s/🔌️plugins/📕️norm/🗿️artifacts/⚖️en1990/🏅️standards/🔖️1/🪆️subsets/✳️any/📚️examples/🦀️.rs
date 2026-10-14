//! 📚️ EN 1990 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🏢️accidental-seismic-compliant/🦀️.rs"]
pub mod accidental_seismic_compliant;
#[path = "🏢️accidental-seismic-failing/🦀️.rs"]
pub mod accidental_seismic_failing;
#[cfg(test)]
#[path = "🏢️fatigue-failing/🧪️tests/🧩️example/🦀️.rs"]
mod example;
#[path = "🏢️fatigue-compliant/🦀️.rs"]
pub mod fatigue_compliant;
#[path = "🏢️fatigue-failing/🦀️.rs"]
pub mod fatigue_failing;
#[path = "🏢️high-consequence-office/🦀️.rs"]
pub mod high_consequence_office;
#[path = "🏢️road-bridge-compliant/🦀️.rs"]
pub mod road_bridge_compliant;
#[path = "🏢️road-bridge-failing/🦀️.rs"]
pub mod road_bridge_failing;
//#endregion 🪢️Mounts
