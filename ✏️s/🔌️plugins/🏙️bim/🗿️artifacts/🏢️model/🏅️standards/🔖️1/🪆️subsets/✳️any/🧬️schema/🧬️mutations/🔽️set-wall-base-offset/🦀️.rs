//! 🔽️ `set-wall-base-offset` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️raises-the-base/🦀️.rs"]
mod tests_raises_the_base;
#[cfg(test)]
#[path = "🧪️tests/🌟️sinks-below-the-floor/🦀️.rs"]
mod tests_sinks_below_the_floor;
#[cfg(test)]
#[path = "🧪️tests/🧲️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🕳️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🔗️follows-by-inference/🦀️.rs"]
mod tests_follows_by_inference;
//#endregion 🔖️Leaves
