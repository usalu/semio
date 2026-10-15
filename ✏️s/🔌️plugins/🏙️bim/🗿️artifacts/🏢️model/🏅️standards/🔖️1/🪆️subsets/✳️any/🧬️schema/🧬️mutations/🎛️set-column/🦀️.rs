//! 🎛️ `set-column` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️retypes-moves-and-renames/🦀️.rs"]
mod tests_retypes_moves_and_renames;
#[cfg(test)]
#[path = "🧪️tests/✨️constrains-the-top/🦀️.rs"]
mod tests_constrains_the_top;
#[cfg(test)]
#[path = "🧪️tests/⚖️keeps-equal-fields-out-of-the-diff/🦀️.rs"]
mod tests_keeps_equal_fields_out_of_the_diff;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧲️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️top-storey-missing/🦀️.rs"]
mod tests_top_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧩️top-storey-other-building/🦀️.rs"]
mod tests_top_storey_other_building;
#[cfg(test)]
#[path = "🧪️tests/🔻️top-below-base/🦀️.rs"]
mod tests_top_below_base;
#[cfg(test)]
#[path = "🧪️tests/🛑️base-above-top/🦀️.rs"]
mod tests_base_above_top;
#[cfg(test)]
#[path = "🧪️tests/⚠️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
//#endregion 🔖️Leaves
