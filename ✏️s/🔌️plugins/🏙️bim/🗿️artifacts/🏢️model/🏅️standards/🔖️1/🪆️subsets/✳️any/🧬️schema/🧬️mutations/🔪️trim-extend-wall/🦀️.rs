//! 🔪️ `trim-extend-wall` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️trims-to-the-target/🦀️.rs"]
mod tests_trims_to_the_target;
#[cfg(test)]
#[path = "🧪️tests/🌟️extends-to-the-target/🦀️.rs"]
mod tests_extends_to_the_target;
#[cfg(test)]
#[path = "🧪️tests/🪝️keeps-openings-in-place/🦀️.rs"]
mod tests_keeps_openings_in_place;
#[cfg(test)]
#[path = "🧪️tests/🌀️extends-an-arc/🦀️.rs"]
mod tests_extends_an_arc;
#[cfg(test)]
#[path = "🧪️tests/🚧️never-meets/🦀️.rs"]
mod tests_never_meets;
#[cfg(test)]
#[path = "🧪️tests/🛑️behind-the-other-end/🦀️.rs"]
mod tests_behind_the_other_end;
#[cfg(test)]
#[path = "🧪️tests/🧲️already-there/🦀️.rs"]
mod tests_already_there;
#[cfg(test)]
#[path = "🧪️tests/⛔️opening-no-longer-fits/🦀️.rs"]
mod tests_opening_no_longer_fits;
#[cfg(test)]
#[path = "🧪️tests/🧩️its-own-target/🦀️.rs"]
mod tests_its_own_target;
#[cfg(test)]
#[path = "🧪️tests/🚫️target-missing/🦀️.rs"]
mod tests_target_missing;
#[cfg(test)]
#[path = "🧪️tests/🕳️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
