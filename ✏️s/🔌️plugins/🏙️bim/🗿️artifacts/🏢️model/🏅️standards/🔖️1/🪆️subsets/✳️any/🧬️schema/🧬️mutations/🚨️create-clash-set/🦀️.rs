//! 🚨️ `create-clash-set` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-beams-against-walls/🦀️.rs"]
mod tests_adds_beams_against_walls;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-a-storey-restricted-set/🦀️.rs"]
mod tests_adds_a_storey_restricted_set;
#[cfg(test)]
#[path = "🧪️tests/🧵️adds-a-clearance-set/🦀️.rs"]
mod tests_adds_a_clearance_set;
#[cfg(test)]
#[path = "🧪️tests/🧴️adds-an-element-selected-set/🦀️.rs"]
mod tests_adds_an_element_selected_set;
#[cfg(test)]
#[path = "🧪️tests/🧳️adds-a-phase-restricted-set/🦀️.rs"]
mod tests_adds_a_phase_restricted_set;
#[cfg(test)]
#[path = "🧪️tests/🧱️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/🧰️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧯️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🧮️negative-tolerance/🦀️.rs"]
mod tests_negative_tolerance;
#[cfg(test)]
#[path = "🧪️tests/🧭️clearance-too-large/🦀️.rs"]
mod tests_clearance_too_large;
#[cfg(test)]
#[path = "🧪️tests/🧩️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧨️element-missing/🦀️.rs"]
mod tests_element_missing;
#[cfg(test)]
#[path = "🧪️tests/🧧️class-twice/🦀️.rs"]
mod tests_class_twice;
//#endregion 🔖️Leaves
