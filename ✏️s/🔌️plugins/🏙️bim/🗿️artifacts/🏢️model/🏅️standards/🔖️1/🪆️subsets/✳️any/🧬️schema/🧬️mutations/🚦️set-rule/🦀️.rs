//! 🚦️ `set-rule` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🧲️tightens-the-limit/🦀️.rs"]
mod tests_tightens_the_limit;
#[cfg(test)]
#[path = "🧪️tests/🧵️changes-the-check/🦀️.rs"]
mod tests_changes_the_check;
#[cfg(test)]
#[path = "🧪️tests/🧴️softens-the-severity/🦀️.rs"]
mod tests_softens_the_severity;
#[cfg(test)]
#[path = "🧪️tests/🧳️narrows-the-scope/🦀️.rs"]
mod tests_narrows_the_scope;
#[cfg(test)]
#[path = "🧪️tests/🧱️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧰️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧯️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧮️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧭️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🧩️zero-limit/🦀️.rs"]
mod tests_zero_limit;
#[cfg(test)]
#[path = "🧪️tests/🧨️slope-too-steep/🦀️.rs"]
mod tests_slope_too_steep;
#[cfg(test)]
#[path = "🧪️tests/🧧️storey-missing/🦀️.rs"]
mod tests_storey_missing;
//#endregion 🔖️Leaves
