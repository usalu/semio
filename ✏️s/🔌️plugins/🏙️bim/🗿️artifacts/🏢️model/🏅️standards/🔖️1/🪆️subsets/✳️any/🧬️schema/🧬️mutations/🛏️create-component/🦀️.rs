//! 🛏️ `create-component` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️places-a-table/🦀️.rs"]
mod tests_places_a_table;
#[cfg(test)]
#[path = "🧪️tests/➕️mounts-a-basin/🦀️.rs"]
mod tests_mounts_a_basin;
#[cfg(test)]
#[path = "🧪️tests/✨️places-a-terminal/🦀️.rs"]
mod tests_places_a_terminal;
#[cfg(test)]
#[path = "🧪️tests/👍️places-on-the-first-storey/🦀️.rs"]
mod tests_places_on_the_first_storey;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️family-missing/🦀️.rs"]
mod tests_family_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️profile-family/🦀️.rs"]
mod tests_profile_family;
#[cfg(test)]
#[path = "🧪️tests/🙅️host-missing/🦀️.rs"]
mod tests_host_missing;
#[cfg(test)]
#[path = "🧪️tests/📛️host-is-no-wall/🦀️.rs"]
mod tests_host_is_no_wall;
#[cfg(test)]
#[path = "🧪️tests/🚧️host-on-another-storey/🦀️.rs"]
mod tests_host_on_another_storey;
//#endregion 🔖️Leaves
