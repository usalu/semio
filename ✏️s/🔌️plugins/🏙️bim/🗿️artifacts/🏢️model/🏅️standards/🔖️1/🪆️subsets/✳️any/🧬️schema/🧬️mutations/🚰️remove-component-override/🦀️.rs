//! 🚰️ `remove-component-override` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️removes/🦀️.rs"]
mod tests_removes;
#[cfg(test)]
#[path = "🧪️tests/➕️removes-one-of-two/🦀️.rs"]
mod tests_removes_one_of_two;
#[cfg(test)]
#[path = "🧪️tests/🚫️no-such-override/🦀️.rs"]
mod tests_no_such_override;
#[cfg(test)]
#[path = "🧪️tests/⛔️component-missing/🦀️.rs"]
mod tests_component_missing;
//#endregion 🔖️Leaves
