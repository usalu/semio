//! 🔡️ `set-family-parameter` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-parameter/🦀️.rs"]
mod tests_adds_a_parameter;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-a-dependent-parameter/🦀️.rs"]
mod tests_adds_a_dependent_parameter;
#[cfg(test)]
#[path = "🧪️tests/✨️changes-a-formula/🦀️.rs"]
mod tests_changes_a_formula;
#[cfg(test)]
#[path = "🧪️tests/👍️changes-the-kind/🦀️.rs"]
mod tests_changes_the_kind;
#[cfg(test)]
#[path = "🧪️tests/🧲️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️family-missing/🦀️.rs"]
mod tests_family_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️bad-name/🦀️.rs"]
mod tests_bad_name;
#[cfg(test)]
#[path = "🧪️tests/🚷️formula-does-not-parse/🦀️.rs"]
mod tests_formula_does_not_parse;
#[cfg(test)]
#[path = "🧪️tests/🙅️unknown-parameter/🦀️.rs"]
mod tests_unknown_parameter;
#[cfg(test)]
#[path = "🧪️tests/📛️closes-a-circle/🦀️.rs"]
mod tests_closes_a_circle;
#[cfg(test)]
#[path = "🧪️tests/🚧️new-needs-a-kind/🦀️.rs"]
mod tests_new_needs_a_kind;
#[cfg(test)]
#[path = "🧪️tests/🧯️new-needs-a-formula/🦀️.rs"]
mod tests_new_needs_a_formula;
#[cfg(test)]
#[path = "🧪️tests/❗️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
//#endregion 🔖️Leaves
