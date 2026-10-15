//! 🧩️ `create-family` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️creates-a-table-family/🦀️.rs"]
mod tests_creates_a_table_family;
#[cfg(test)]
#[path = "🧪️tests/➕️creates-a-profile-family/🦀️.rs"]
mod tests_creates_a_profile_family;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️blank-name/🦀️.rs"]
mod tests_blank_name;
//#endregion 🔖️Leaves
