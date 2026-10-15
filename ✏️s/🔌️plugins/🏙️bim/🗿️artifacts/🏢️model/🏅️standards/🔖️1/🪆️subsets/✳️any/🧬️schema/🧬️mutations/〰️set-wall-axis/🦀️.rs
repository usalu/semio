//! 〰️ `set-wall-axis` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️stretches-under-an-opening/🦀️.rs"]
mod tests_stretches_under_an_opening;
#[cfg(test)]
#[path = "🧪️tests/🌀️curves-the-wall/🦀️.rs"]
mod tests_curves_the_wall;
#[cfg(test)]
#[path = "🧪️tests/🧵️straightens-the-arc/🦀️.rs"]
mod tests_straightens_the_arc;
#[cfg(test)]
#[path = "🧪️tests/🧲️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🚫️zero-length/🦀️.rs"]
mod tests_zero_length;
#[cfg(test)]
#[path = "🧪️tests/⛔️flat-arc/🦀️.rs"]
mod tests_flat_arc;
#[cfg(test)]
#[path = "🧪️tests/🕳️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🔗️follows-by-inference/🦀️.rs"]
mod tests_follows_by_inference;
//#endregion 🔖️Leaves
