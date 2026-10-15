//! 🪞️ `flip-wall` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️reverses-a-line/🦀️.rs"]
mod tests_reverses_a_line;
#[cfg(test)]
#[path = "🧪️tests/🌀️reverses-an-arc/🦀️.rs"]
mod tests_reverses_an_arc;
#[cfg(test)]
#[path = "🧪️tests/🧲️pointlike-wall/🦀️.rs"]
mod tests_pointlike_wall;
#[cfg(test)]
#[path = "🧪️tests/🕳️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🔗️follows-by-inference/🦀️.rs"]
mod tests_follows_by_inference;
//#endregion 🔖️Leaves
