//! 🧾️ `set-element-property` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-the-first-property/🦀️.rs"]
mod tests_adds_the_first_property;
#[cfg(test)]
#[path = "🧪️tests/🚛️adds-to-an-existing-set/🦀️.rs"]
mod tests_adds_to_an_existing_set;
#[cfg(test)]
#[path = "🧪️tests/🌀️replaces-a-value/🦀️.rs"]
mod tests_replaces_a_value;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-element/🦀️.rs"]
mod tests_unknown_element;
#[cfg(test)]
#[path = "🧪️tests/🛑️same-value/🦀️.rs"]
mod tests_same_value;
#[cfg(test)]
#[path = "🧪️tests/🧭️blank-property/🦀️.rs"]
mod tests_blank_property;
#[cfg(test)]
#[path = "🧪️tests/🗂️sets-a-type-property/🦀️.rs"]
mod tests_sets_a_type_property;
#[cfg(test)]
#[path = "🧪️tests/🏷️type-property-replaced/🦀️.rs"]
mod tests_type_property_replaced;
//#endregion 🔖️Leaves
