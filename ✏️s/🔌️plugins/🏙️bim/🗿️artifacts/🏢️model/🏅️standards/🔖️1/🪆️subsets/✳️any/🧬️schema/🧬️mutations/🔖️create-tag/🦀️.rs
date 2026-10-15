//! 🔖️ `create-tag` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️tags-a-wall/🦀️.rs"]
mod tests_tags_a_wall;
#[cfg(test)]
#[path = "🧪️tests/➕️tags-a-window-size/🦀️.rs"]
mod tests_tags_a_window_size;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️element-missing/🦀️.rs"]
mod tests_element_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️style-missing/🦀️.rs"]
mod tests_style_missing;
//#endregion 🔖️Leaves
