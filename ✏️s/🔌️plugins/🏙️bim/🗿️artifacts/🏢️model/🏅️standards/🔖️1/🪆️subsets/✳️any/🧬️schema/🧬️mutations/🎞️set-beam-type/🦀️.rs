//! 🎞️ `set-beam-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames/🦀️.rs"]
mod tests_renames;
#[cfg(test)]
#[path = "🧪️tests/🧲️swaps-the-profile/🦀️.rs"]
mod tests_swaps_the_profile;
#[cfg(test)]
#[path = "🧪️tests/✨️changes-every-field/🦀️.rs"]
mod tests_changes_every_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/⛔️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️invalid-profile/🦀️.rs"]
mod tests_invalid_profile;
#[cfg(test)]
#[path = "🧪️tests/❌️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
//#endregion 🔖️Leaves
