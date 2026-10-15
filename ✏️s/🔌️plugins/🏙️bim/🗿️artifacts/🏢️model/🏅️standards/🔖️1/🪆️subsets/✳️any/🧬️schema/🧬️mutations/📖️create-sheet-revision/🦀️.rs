//! 📖️ `create-sheet-revision` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-revision/🦀️.rs"]
mod tests_adds_a_revision;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-the-next-mark/🦀️.rs"]
mod tests_adds_the_next_mark;
#[cfg(test)]
#[path = "🧪️tests/🧵️adds-an-undated-revision/🦀️.rs"]
mod tests_adds_an_undated_revision;
#[cfg(test)]
#[path = "🧪️tests/🧴️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/🧳️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧱️sheet-missing/🦀️.rs"]
mod tests_sheet_missing;
#[cfg(test)]
#[path = "🧪️tests/🧰️mark-taken/🦀️.rs"]
mod tests_mark_taken;
#[cfg(test)]
#[path = "🧪️tests/🧯️blank-mark/🦀️.rs"]
mod tests_blank_mark;
#[cfg(test)]
#[path = "🧪️tests/🧮️date-unreadable/🦀️.rs"]
mod tests_date_unreadable;
#[cfg(test)]
#[path = "🧪️tests/🧭️blank-description/🦀️.rs"]
mod tests_blank_description;
//#endregion 🔖️Leaves
