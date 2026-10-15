//! 📐️ `set-beam` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️retypes-and-lowers/🦀️.rs"]
mod tests_retypes_and_lowers;
#[cfg(test)]
#[path = "🧪️tests/➕️inclines-the-beam/🦀️.rs"]
mod tests_inclines_the_beam;
#[cfg(test)]
#[path = "🧪️tests/✨️levels-the-beam/🦀️.rs"]
mod tests_levels_the_beam;
#[cfg(test)]
#[path = "🧪️tests/👍️renames/🦀️.rs"]
mod tests_renames;
#[cfg(test)]
#[path = "🧪️tests/🧲️keeps-equal-fields-out-of-the-diff/🦀️.rs"]
mod tests_keeps_equal_fields_out_of_the_diff;
#[cfg(test)]
#[path = "🧪️tests/🚫️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️already-level/🦀️.rs"]
mod tests_already_level;
#[cfg(test)]
#[path = "🧪️tests/🛑️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
