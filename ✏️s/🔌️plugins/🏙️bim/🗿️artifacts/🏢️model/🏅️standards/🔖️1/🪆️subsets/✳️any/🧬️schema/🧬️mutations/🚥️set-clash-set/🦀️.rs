//! 🚥️ `set-clash-set` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🧲️retargets-side-a/🦀️.rs"]
mod tests_retargets_side_a;
#[cfg(test)]
#[path = "🧪️tests/🧵️retargets-side-b/🦀️.rs"]
mod tests_retargets_side_b;
#[cfg(test)]
#[path = "🧪️tests/🧴️loosens-the-tolerance/🦀️.rs"]
mod tests_loosens_the_tolerance;
#[cfg(test)]
#[path = "🧪️tests/🧳️asks-for-a-clearance/🦀️.rs"]
mod tests_asks_for_a_clearance;
#[cfg(test)]
#[path = "🧪️tests/🧱️drops-the-clearance/🦀️.rs"]
mod tests_drops_the_clearance;
#[cfg(test)]
#[path = "🧪️tests/🧰️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧯️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧮️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧭️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧩️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🧨️tolerance-too-large/🦀️.rs"]
mod tests_tolerance_too_large;
#[cfg(test)]
#[path = "🧪️tests/🧧️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧦️element-missing/🦀️.rs"]
mod tests_element_missing;
//#endregion 🔖️Leaves
