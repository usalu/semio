//! 🏗️ `set-building` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️raises-the-datum/🦀️.rs"]
mod tests_raises_the_datum;
#[cfg(test)]
#[path = "🧪️tests/🧲️places-the-building/🦀️.rs"]
mod tests_places_the_building;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🔁️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🪜️raises-the-absolute-levels/🦀️.rs"]
mod tests_raises_the_absolute_levels;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
//#endregion 🔖️Leaves
