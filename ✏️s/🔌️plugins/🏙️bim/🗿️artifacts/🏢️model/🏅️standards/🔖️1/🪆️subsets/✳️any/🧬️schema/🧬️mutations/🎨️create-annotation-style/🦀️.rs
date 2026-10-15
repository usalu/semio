//! 🎨️ `create-annotation-style` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/➕️adds-a-millimetre-style/🦀️.rs"]
mod tests_adds_a_millimetre_style;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🛑️text-height-not-positive/🦀️.rs"]
mod tests_text_height_not_positive;
#[cfg(test)]
#[path = "🧪️tests/🚷️too-many-decimals/🦀️.rs"]
mod tests_too_many_decimals;
//#endregion 🔖️Leaves
