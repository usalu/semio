//! 💬️ `create-issue-comment` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-comment/🦀️.rs"]
mod tests_adds_a_comment;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-a-reply/🦀️.rs"]
mod tests_adds_a_reply;
#[cfg(test)]
#[path = "🧪️tests/🧵️adds-a-dated-comment/🦀️.rs"]
mod tests_adds_a_dated_comment;
#[cfg(test)]
#[path = "🧪️tests/🧴️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/🧳️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧱️issue-missing/🦀️.rs"]
mod tests_issue_missing;
#[cfg(test)]
#[path = "🧪️tests/🧰️blank-author/🦀️.rs"]
mod tests_blank_author;
#[cfg(test)]
#[path = "🧪️tests/🧯️blank-text/🦀️.rs"]
mod tests_blank_text;
#[cfg(test)]
#[path = "🧪️tests/🧮️unreadable-date/🦀️.rs"]
mod tests_unreadable_date;
//#endregion 🔖️Leaves
