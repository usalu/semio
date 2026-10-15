//! ♨️ `set-type-thermal-data` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️states-a-window/🦀️.rs"]
mod tests_states_a_window;
#[cfg(test)]
#[path = "🧪️tests/🧲️states-a-door/🦀️.rs"]
mod tests_states_a_door;
#[cfg(test)]
#[path = "🧪️tests/🧵️improves-the-glazing/🦀️.rs"]
mod tests_improves_the_glazing;
#[cfg(test)]
#[path = "🧪️tests/🧴️clears-the-g-value/🦀️.rs"]
mod tests_clears_the_g_value;
#[cfg(test)]
#[path = "🧪️tests/🧳️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧱️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧰️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧯️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🧮️u-value-zero/🦀️.rs"]
mod tests_u_value_zero;
#[cfg(test)]
#[path = "🧪️tests/🧭️g-value-above-one/🦀️.rs"]
mod tests_g_value_above_one;
#[cfg(test)]
#[path = "🧪️tests/🧩️frame-fraction-one/🦀️.rs"]
mod tests_frame_fraction_one;
#[cfg(test)]
#[path = "🧪️tests/🧨️door-has-no-g-value/🦀️.rs"]
mod tests_door_has_no_g_value;
#[cfg(test)]
#[path = "🧪️tests/🧧️door-has-no-frame-fraction/🦀️.rs"]
mod tests_door_has_no_frame_fraction;
//#endregion 🔖️Leaves
