//! 🥖️ `split-beam` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️splits-a-beam/🦀️.rs"]
mod tests_splits_a_beam;
#[cfg(test)]
#[path = "🧪️tests/🌟️splits-a-slanted-beam/🦀️.rs"]
mod tests_splits_a_slanted_beam;
#[cfg(test)]
#[path = "🧪️tests/🚫️at-the-start/🦀️.rs"]
mod tests_at_the_start;
#[cfg(test)]
#[path = "🧪️tests/⛔️beyond-the-end/🦀️.rs"]
mod tests_beyond_the_end;
#[cfg(test)]
#[path = "🧪️tests/🧩️new-id-taken/🦀️.rs"]
mod tests_new_id_taken;
#[cfg(test)]
#[path = "🧪️tests/🕳️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/💪️splits-an-arc-beam/🦀️.rs"]
mod tests_splits_an_arc_beam;
#[cfg(test)]
#[path = "🧪️tests/➕️splits-an-inclined-beam/🦀️.rs"]
mod tests_splits_an_inclined_beam;
//#endregion 🔖️Leaves
