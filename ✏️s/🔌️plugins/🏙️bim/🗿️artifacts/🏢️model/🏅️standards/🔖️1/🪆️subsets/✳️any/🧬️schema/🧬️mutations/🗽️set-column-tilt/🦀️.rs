//! 🗽️ `set-column-tilt` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️leans-the-column/🦀️.rs"]
mod tests_leans_the_column;
#[cfg(test)]
#[path = "🧪️tests/➕️re-aims-the-lean/🦀️.rs"]
mod tests_re_aims_the_lean;
#[cfg(test)]
#[path = "🧪️tests/✨️straightens-the-column/🦀️.rs"]
mod tests_straightens_the_column;
#[cfg(test)]
#[path = "🧪️tests/🚫️angle-zero/🦀️.rs"]
mod tests_angle_zero;
#[cfg(test)]
#[path = "🧪️tests/⛔️too-steep/🦀️.rs"]
mod tests_too_steep;
#[cfg(test)]
#[path = "🧪️tests/❌️negative-angle/🦀️.rs"]
mod tests_negative_angle;
#[cfg(test)]
#[path = "🧪️tests/🛑️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🚷️already-plumb/🦀️.rs"]
mod tests_already_plumb;
#[cfg(test)]
#[path = "🧪️tests/🙅️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
