pub mod artifacts { pub mod epw {
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)] pub struct EpwLocation { pub city:String, pub latitude:String, pub longitude:String, pub time_zone:String, pub elevation:String }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)] pub struct EpwRecord { pub year:String, pub month:String, pub day:String, pub hour:String, pub minute:String, pub dry_bulb_temp:String, pub dew_point_temp:String, pub relative_humidity:String, pub atmospheric_pressure:String, pub wind_speed:String, pub wind_direction:String, pub direct_normal_radiation:String, pub diffuse_horizontal_radiation:String, pub horizontal_infrared_radiation:String, pub liquid_precip_depth:String, pub snow_depth:String }
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)] pub struct EpwSnapshot { pub location:EpwLocation, pub records:Vec<EpwRecord> }
pub mod standards { pub mod energyplus { pub mod subsets { pub mod any { pub mod schema { pub mod snapshot { pub use crate::artifacts::epw::EpwRecord; } } pub mod io { pub fn decode_epw(_: &str) -> Result<crate::artifacts::epw::EpwSnapshot,String> { Err("focused harness excludes EPW codec".into()) } } } } } }
} }

