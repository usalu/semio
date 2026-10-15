//! ⚖️ `create-rule` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-riser-limit/🦀️.rs"]
mod tests_adds_a_riser_limit;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-a-door-width/🦀️.rs"]
mod tests_adds_a_door_width;
#[cfg(test)]
#[path = "🧪️tests/🧵️adds-a-ramp-slope/🦀️.rs"]
mod tests_adds_a_ramp_slope;
#[cfg(test)]
#[path = "🧪️tests/🧴️adds-a-corridor-width/🦀️.rs"]
mod tests_adds_a_corridor_width;
#[cfg(test)]
#[path = "🧪️tests/🧳️adds-a-compartment-area/🦀️.rs"]
mod tests_adds_a_compartment_area;
#[cfg(test)]
#[path = "🧪️tests/🧱️adds-a-note-for-one-element/🦀️.rs"]
mod tests_adds_a_note_for_one_element;
#[cfg(test)]
#[path = "🧪️tests/🧰️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/🧯️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧮️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🧭️zero-limit/🦀️.rs"]
mod tests_zero_limit;
#[cfg(test)]
#[path = "🧪️tests/🧩️slope-too-steep/🦀️.rs"]
mod tests_slope_too_steep;
#[cfg(test)]
#[path = "🧪️tests/🧨️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧧️element-missing/🦀️.rs"]
mod tests_element_missing;
//#endregion 🔖️Leaves
