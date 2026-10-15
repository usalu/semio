//! 🦚️ `set-wall-location` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves-to-the-exterior-face/🦀️.rs"]
mod tests_moves_to_the_exterior_face;
#[cfg(test)]
#[path = "🧪️tests/🌟️moves-to-the-core-centre/🦀️.rs"]
mod tests_moves_to_the_core_centre;
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
