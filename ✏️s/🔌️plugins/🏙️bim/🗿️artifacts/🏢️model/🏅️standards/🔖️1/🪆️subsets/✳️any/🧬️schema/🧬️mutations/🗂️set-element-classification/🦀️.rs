//! 🗂️ `set-element-classification` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️classifies-an-element/🦀️.rs"]
mod tests_classifies_an_element;
#[cfg(test)]
#[path = "🧪️tests/🚛️reclassifies/🦀️.rs"]
mod tests_reclassifies;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-element/🦀️.rs"]
mod tests_unknown_element;
#[cfg(test)]
#[path = "🧪️tests/🚧️empty-code/🦀️.rs"]
mod tests_empty_code;
#[cfg(test)]
#[path = "🧪️tests/🛑️same-classification/🦀️.rs"]
mod tests_same_classification;
#[cfg(test)]
#[path = "🧪️tests/🧩️adds-another-system/🦀️.rs"]
mod tests_adds_another_system;
#[cfg(test)]
#[path = "🧪️tests/🏷️classifies-a-type/🦀️.rs"]
mod tests_classifies_a_type;
#[cfg(test)]
#[path = "🧪️tests/📝️code-outside-the-table/🦀️.rs"]
mod tests_code_outside_the_table;
#[cfg(test)]
#[path = "🧪️tests/⛔️unknown-system/🦀️.rs"]
mod tests_unknown_system;
//#endregion 🔖️Leaves
