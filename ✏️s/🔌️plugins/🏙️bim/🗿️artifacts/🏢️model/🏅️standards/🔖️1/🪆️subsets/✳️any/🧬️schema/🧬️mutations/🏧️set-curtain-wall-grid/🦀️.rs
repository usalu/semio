//! 🏧️ `set-curtain-wall-grid` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️sets-explicit-lines/🦀️.rs"]
mod tests_sets_explicit_lines;
#[cfg(test)]
#[path = "🧪️tests/➕️sets-a-uniform-spacing/🦀️.rs"]
mod tests_sets_a_uniform_spacing;
#[cfg(test)]
#[path = "🧪️tests/✨️sets-both-directions/🦀️.rs"]
mod tests_sets_both_directions;
#[cfg(test)]
#[path = "🧪️tests/👍️adds-a-line/🦀️.rs"]
mod tests_adds_a_line;
#[cfg(test)]
#[path = "🧪️tests/🧲️removes-a-line/🦀️.rs"]
mod tests_removes_a_line;
#[cfg(test)]
#[path = "🧪️tests/🌟️clears-the-override/🦀️.rs"]
mod tests_clears_the_override;
#[cfg(test)]
#[path = "🧪️tests/💪️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️already-following-the-type/🦀️.rs"]
mod tests_already_following_the_type;
#[cfg(test)]
#[path = "🧪️tests/🛑️lines-not-ascending/🦀️.rs"]
mod tests_lines_not_ascending;
#[cfg(test)]
#[path = "🧪️tests/🚷️line-at-the-start/🦀️.rs"]
mod tests_line_at_the_start;
#[cfg(test)]
#[path = "🧪️tests/🙅️spacing-non-positive/🦀️.rs"]
mod tests_spacing_non_positive;
#[cfg(test)]
#[path = "🧪️tests/📛️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
