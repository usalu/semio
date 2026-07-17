//! ⚡ Headless building energy model engine: typed Rust API for BEM simulation.

#![allow(clippy::too_many_arguments)]

#[path = "src/error.rs"]
mod error;
#[path = "src/units.rs"]
mod units;
#[path = "src/num.rs"]
mod num;
#[path = "src/props.rs"]
mod props;
#[path = "src/model.rs"]
mod model;
#[path = "src/schedule.rs"]
mod schedule;
#[path = "src/site.rs"]
mod site;
#[path = "src/curves.rs"]
mod curves;
#[path = "src/hvac_topo.rs"]
mod hvac_topo;
#[path = "src/ideal_hvac.rs"]
mod ideal_hvac;
#[path = "src/zone_hvac.rs"]
mod zone_hvac;
#[path = "src/terminal.rs"]
mod terminal;
#[path = "src/air_system.rs"]
mod air_system;
#[path = "src/fans.rs"]
mod fans;
#[path = "src/coils.rs"]
mod coils;
#[path = "src/evaporative.rs"]
mod evaporative;
#[path = "src/humidity_eq.rs"]
mod humidity_eq;
#[path = "src/heat_recovery.rs"]
mod heat_recovery;
#[path = "src/plant.rs"]
mod plant;
#[path = "src/shw.rs"]
mod shw;
#[path = "src/solar_thermal.rs"]
mod solar_thermal;
#[path = "src/refrigeration.rs"]
mod refrigeration;
#[path = "src/electrical.rs"]
mod electrical;
#[path = "src/water.rs"]
mod water;
#[path = "src/faults.rs"]
mod faults;

pub use error::*;
pub use units::*;
pub use num::*;
pub use props::*;
pub use model::*;
pub use schedule::*;
pub use site::*;
pub use curves::*;
pub use hvac_topo::*;
pub use ideal_hvac::*;
pub use zone_hvac::*;
pub use terminal::*;
pub use air_system::*;
pub use fans::*;
pub use coils::*;
pub use evaporative::*;
pub use humidity_eq::*;
pub use heat_recovery::*;
pub use plant::*;
pub use shw::*;
pub use solar_thermal::*;
pub use refrigeration::*;
pub use electrical::*;
pub use water::*;
pub use faults::*;
