//! 🏔️ `set-roof-shape` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️hips-the-roof/🦀️.rs"]
mod tests_hips_the_roof;
#[cfg(test)]
#[path = "🧪️tests/➕️overhangs-and-lifts/🦀️.rs"]
mod tests_overhangs_and_lifts;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️pitch-too-flat/🦀️.rs"]
mod tests_pitch_too_flat;
#[cfg(test)]
#[path = "🧪️tests/🛑️negative-overhang/🦀️.rs"]
mod tests_negative_overhang;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
