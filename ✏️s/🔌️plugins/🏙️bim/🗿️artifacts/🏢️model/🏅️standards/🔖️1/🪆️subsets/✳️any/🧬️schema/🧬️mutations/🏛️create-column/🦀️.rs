//! 🏛️ `create-column` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-to-the-storey-top/🦀️.rs"]
mod tests_adds_to_the_storey_top;
#[cfg(test)]
#[path = "🧪️tests/✨️spans-into-the-storey-above/🦀️.rs"]
mod tests_spans_into_the_storey_above;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️storey-missing/🦀️.rs"]
mod tests_storey_missing;
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
#[path = "🧪️tests/🛑️zero-height/🦀️.rs"]
mod tests_zero_height;
#[cfg(test)]
#[path = "🧪️tests/📌️authored-top-is-stored/🦀️.rs"]
mod tests_authored_top_is_stored;
#[cfg(test)]
#[path = "🧪️tests/🧮️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/💪️adds-a-leaning-column/🦀️.rs"]
mod tests_adds_a_leaning_column;
#[cfg(test)]
#[path = "🧪️tests/🧱️tilt-too-steep/🦀️.rs"]
mod tests_tilt_too_steep;
//#endregion 🔖️Leaves
