//! 📚️ EN 1990 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "🏢️accidental-seismic-compliant/🦀️.rs"]
pub mod accidental_seismic_compliant;
#[cfg(test)]
#[path = "🏢️accidental-seismic-compliant/🧪️tests/🧩️example/🦀️.rs"]
mod example_accidental_seismic_compliant;
#[path = "🏢️accidental-seismic-failing/🦀️.rs"]
pub mod accidental_seismic_failing;
#[cfg(test)]
#[path = "🏢️accidental-seismic-failing/🧪️tests/🧩️example/🦀️.rs"]
mod example_accidental_seismic_failing;
#[path = "🏢️fatigue-compliant/🦀️.rs"]
pub mod fatigue_compliant;
#[cfg(test)]
#[path = "🏢️fatigue-compliant/🧪️tests/🧩️example/🦀️.rs"]
mod example_fatigue_compliant;
#[path = "🏢️fatigue-failing/🦀️.rs"]
pub mod fatigue_failing;
#[cfg(test)]
#[path = "🏢️fatigue-failing/🧪️tests/🧩️example/🦀️.rs"]
mod example_fatigue_failing;
#[path = "🏢️high-consequence-office/🦀️.rs"]
pub mod high_consequence_office;
#[cfg(test)]
#[path = "🏢️high-consequence-office/🧪️tests/🧩️example/🦀️.rs"]
mod example_high_consequence_office;
#[path = "🏢️road-bridge-compliant/🦀️.rs"]
pub mod road_bridge_compliant;
#[cfg(test)]
#[path = "🏢️road-bridge-compliant/🧪️tests/🧩️example/🦀️.rs"]
mod example_road_bridge_compliant;
#[path = "🏢️road-bridge-failing/🦀️.rs"]
pub mod road_bridge_failing;
#[cfg(test)]
#[path = "🏢️road-bridge-failing/🧪️tests/🧩️example/🦀️.rs"]
mod example_road_bridge_failing;
#[cfg(test)]
#[path = "🧪️tests/🗜️pack-twins/🦀️.rs"]
mod pack_twins;
//#endregion 🪢️Mounts
