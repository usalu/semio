//! 🪄️ `set-zone` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames-and-recategorises/🦀️.rs"]
mod tests_renames_and_recategorises;
#[cfg(test)]
#[path = "🧪️tests/👥️sets-the-density/🦀️.rs"]
mod tests_sets_the_density;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/💤️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/📉️negative-density/🦀️.rs"]
mod tests_negative_density;
//#endregion 🔖️Leaves
