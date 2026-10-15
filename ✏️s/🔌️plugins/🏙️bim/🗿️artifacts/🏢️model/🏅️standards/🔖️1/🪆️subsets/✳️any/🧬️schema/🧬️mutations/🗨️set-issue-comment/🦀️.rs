//! 🗨️ `set-issue-comment` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️edits-the-text/🦀️.rs"]
mod tests_edits_the_text;
#[cfg(test)]
#[path = "🧪️tests/🧲️re-dates/🦀️.rs"]
mod tests_re_dates;
#[cfg(test)]
#[path = "🧪️tests/🧵️re-signs/🦀️.rs"]
mod tests_re_signs;
#[cfg(test)]
#[path = "🧪️tests/🧴️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧳️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧱️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧰️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧯️blank-text/🦀️.rs"]
mod tests_blank_text;
#[cfg(test)]
#[path = "🧪️tests/🧮️blank-author/🦀️.rs"]
mod tests_blank_author;
#[cfg(test)]
#[path = "🧪️tests/🧭️unreadable-date/🦀️.rs"]
mod tests_unreadable_date;
//#endregion 🔖️Leaves
