//! 💥️ `delete-wall` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🌊️cascades-the-opening/🦀️.rs"]
mod tests_cascades_the_opening;
#[cfg(test)]
#[path = "🧪️tests/🏗️cascades-the-opening-and-data/🦀️.rs"]
mod tests_cascades_the_opening_and_data;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️cascades-its-sweeps/🦀️.rs"]
mod tests_cascades_its_sweeps;
//#endregion 🔖️Leaves
