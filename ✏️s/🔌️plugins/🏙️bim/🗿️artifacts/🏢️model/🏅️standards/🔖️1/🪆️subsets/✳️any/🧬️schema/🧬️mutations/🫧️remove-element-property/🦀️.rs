//! 🫧️ `remove-element-property` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️removes-one-of-two/🦀️.rs"]
mod tests_removes_one_of_two;
#[cfg(test)]
#[path = "🧪️tests/🚛️removes-the-last-property/🦀️.rs"]
mod tests_removes_the_last_property;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing-property/🦀️.rs"]
mod tests_missing_property;
#[cfg(test)]
#[path = "🧪️tests/⛔️unknown-element/🦀️.rs"]
mod tests_unknown_element;
#[cfg(test)]
#[path = "🧪️tests/🗂️removes-a-type-property/🦀️.rs"]
mod tests_removes_a_type_property;
//#endregion 🔖️Leaves
