//! 🌡️ `set-space-conditions` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️creates-the-record/🦀️.rs"]
mod tests_creates_the_record;
#[cfg(test)]
#[path = "🧪️tests/🧲️creates-a-full-record/🦀️.rs"]
mod tests_creates_a_full_record;
#[cfg(test)]
#[path = "🧪️tests/🧵️creates-an-empty-record/🦀️.rs"]
mod tests_creates_an_empty_record;
#[cfg(test)]
#[path = "🧪️tests/🧴️raises-the-set-point/🦀️.rs"]
mod tests_raises_the_set_point;
#[cfg(test)]
#[path = "🧪️tests/🧳️adds-ventilation-and-loads/🦀️.rs"]
mod tests_adds_ventilation_and_loads;
#[cfg(test)]
#[path = "🧪️tests/🧱️names-occupancy-and-schedule/🦀️.rs"]
mod tests_names_occupancy_and_schedule;
#[cfg(test)]
#[path = "🧪️tests/🧰️clears-a-field/🦀️.rs"]
mod tests_clears_a_field;
#[cfg(test)]
#[path = "🧪️tests/🧯️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧮️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧭️empty-patch-on-a-record/🦀️.rs"]
mod tests_empty_patch_on_a_record;
#[cfg(test)]
#[path = "🧪️tests/🧩️space-missing/🦀️.rs"]
mod tests_space_missing;
#[cfg(test)]
#[path = "🧪️tests/🧨️cooling-below-heating/🦀️.rs"]
mod tests_cooling_below_heating;
#[cfg(test)]
#[path = "🧪️tests/🧧️set-point-out-of-range/🦀️.rs"]
mod tests_set_point_out_of_range;
#[cfg(test)]
#[path = "🧪️tests/🧦️negative-density/🦀️.rs"]
mod tests_negative_density;
#[cfg(test)]
#[path = "🧪️tests/🧥️negative-ventilation/🦀️.rs"]
mod tests_negative_ventilation;
#[cfg(test)]
#[path = "🧪️tests/🧤️negative-lighting/🦀️.rs"]
mod tests_negative_lighting;
#[cfg(test)]
#[path = "🧪️tests/🧣️blank-schedule/🦀️.rs"]
mod tests_blank_schedule;
//#endregion 🔖️Leaves
