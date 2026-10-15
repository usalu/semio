//! 🔷️ `set-slab-boundary` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️reshapes/🦀️.rs"]
mod tests_reshapes;
#[cfg(test)]
#[path = "🧪️tests/➕️drops-the-hole/🦀️.rs"]
mod tests_drops_the_hole;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️hole-outside/🦀️.rs"]
mod tests_hole_outside;
#[cfg(test)]
#[path = "🧪️tests/❌️overlapping-holes/🦀️.rs"]
mod tests_overlapping_holes;
#[cfg(test)]
#[path = "🧪️tests/🛑️self-intersecting/🦀️.rs"]
mod tests_self_intersecting;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
