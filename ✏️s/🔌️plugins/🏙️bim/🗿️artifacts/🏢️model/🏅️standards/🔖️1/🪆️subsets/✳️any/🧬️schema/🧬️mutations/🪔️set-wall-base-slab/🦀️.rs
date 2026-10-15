//! 🪔️ `set-wall-base-slab` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️attaches/🦀️.rs"]
mod tests_attaches;
#[cfg(test)]
#[path = "🧪️tests/➕️frees-the-base/🦀️.rs"]
mod tests_frees_the_base;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️already-free/🦀️.rs"]
mod tests_already_free;
#[cfg(test)]
#[path = "🧪️tests/❌️slab-missing/🦀️.rs"]
mod tests_slab_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️slab-in-another-building/🦀️.rs"]
mod tests_slab_in_another_building;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
