//! 🛷️ `move-opening` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️slides-along-the-host/🦀️.rs"]
mod tests_slides_along_the_host;
#[cfg(test)]
#[path = "🧪️tests/➕️rehosts-to-another-wall/🦀️.rs"]
mod tests_rehosts_to_another_wall;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/⛔️host-missing/🦀️.rs"]
mod tests_host_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️beyond-the-host-end/🦀️.rs"]
mod tests_beyond_the_host_end;
#[cfg(test)]
#[path = "🧪️tests/❌️overlaps-a-neighbour/🦀️.rs"]
mod tests_overlaps_a_neighbour;
#[cfg(test)]
#[path = "🧪️tests/❗️already-there/🦀️.rs"]
mod tests_already_there;
//#endregion 🔖️Leaves
