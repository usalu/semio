//! 🪝️ `set-beam-axis` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️curves-the-beam/🦀️.rs"]
mod tests_curves_the_beam;
#[cfg(test)]
#[path = "🧪️tests/➕️straightens-the-arc/🦀️.rs"]
mod tests_straightens_the_arc;
#[cfg(test)]
#[path = "🧪️tests/✨️moves-the-beam/🦀️.rs"]
mod tests_moves_the_beam;
#[cfg(test)]
#[path = "🧪️tests/👍️keeps-the-inclination/🦀️.rs"]
mod tests_keeps_the_inclination;
#[cfg(test)]
#[path = "🧪️tests/🚫️zero-length/🦀️.rs"]
mod tests_zero_length;
#[cfg(test)]
#[path = "🧪️tests/⛔️flat-arc/🦀️.rs"]
mod tests_flat_arc;
#[cfg(test)]
#[path = "🧪️tests/❌️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🛑️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
