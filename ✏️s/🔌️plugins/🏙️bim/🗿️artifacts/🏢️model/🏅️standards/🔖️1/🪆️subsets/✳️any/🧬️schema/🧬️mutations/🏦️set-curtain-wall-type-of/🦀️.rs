//! 🏦️ `set-curtain-wall-type-of` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️retypes-the-facade/🦀️.rs"]
mod tests_retypes_the_facade;
#[cfg(test)]
#[path = "🧪️tests/➕️keeps-the-overrides/🦀️.rs"]
mod tests_keeps_the_overrides;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/❌️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
