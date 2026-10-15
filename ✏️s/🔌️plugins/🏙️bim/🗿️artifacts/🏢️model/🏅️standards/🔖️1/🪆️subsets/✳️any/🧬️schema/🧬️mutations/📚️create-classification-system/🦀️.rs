//! 📚️ `create-classification-system` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds/🦀️.rs"]
mod tests_adds;
#[cfg(test)]
#[path = "🧪️tests/🚫️adds-the-first-system/🦀️.rs"]
mod tests_adds_the_first_system;
#[cfg(test)]
#[path = "🧪️tests/⛔️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/🛑️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧭️reserved-id/🦀️.rs"]
mod tests_reserved_id;
#[cfg(test)]
#[path = "🧪️tests/💤️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/📝️blank-code/🦀️.rs"]
mod tests_blank_code;
#[cfg(test)]
#[path = "🧪️tests/🎯️code-used-twice/🦀️.rs"]
mod tests_code_used_twice;
#[cfg(test)]
#[path = "🧪️tests/📉️parent-missing/🦀️.rs"]
mod tests_parent_missing;
#[cfg(test)]
#[path = "🧪️tests/🗂️parents-form-a-cycle/🦀️.rs"]
mod tests_parents_form_a_cycle;
//#endregion 🔖️Leaves
