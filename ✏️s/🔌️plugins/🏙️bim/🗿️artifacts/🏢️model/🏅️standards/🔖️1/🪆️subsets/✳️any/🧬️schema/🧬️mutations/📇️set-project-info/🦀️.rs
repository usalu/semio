//! 📇️ `set-project-info` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames-the-project/🦀️.rs"]
mod tests_renames_the_project;
#[cfg(test)]
#[path = "🧪️tests/🧲️sets-the-phases/🦀️.rs"]
mod tests_sets_the_phases;
#[cfg(test)]
#[path = "🧪️tests/🚫️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/⛔️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/💥️blank-phase/🦀️.rs"]
mod tests_blank_phase;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
//#endregion 🔖️Leaves
