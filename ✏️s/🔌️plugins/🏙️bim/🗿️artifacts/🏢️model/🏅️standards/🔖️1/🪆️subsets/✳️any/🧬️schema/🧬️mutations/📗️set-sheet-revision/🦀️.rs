//! 📗️ `set-sheet-revision` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️redates/🦀️.rs"]
mod tests_redates;
#[cfg(test)]
#[path = "🧪️tests/🧲️describes/🦀️.rs"]
mod tests_describes;
#[cfg(test)]
#[path = "🧪️tests/🧵️re-marks/🦀️.rs"]
mod tests_re_marks;
#[cfg(test)]
#[path = "🧪️tests/🧴️signs/🦀️.rs"]
mod tests_signs;
#[cfg(test)]
#[path = "🧪️tests/🧳️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧱️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧰️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧯️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧮️mark-taken/🦀️.rs"]
mod tests_mark_taken;
#[cfg(test)]
#[path = "🧪️tests/🧭️blank-mark/🦀️.rs"]
mod tests_blank_mark;
#[cfg(test)]
#[path = "🧪️tests/🧩️date-unreadable/🦀️.rs"]
mod tests_date_unreadable;
#[cfg(test)]
#[path = "🧪️tests/🧨️blank-description/🦀️.rs"]
mod tests_blank_description;
//#endregion 🔖️Leaves
