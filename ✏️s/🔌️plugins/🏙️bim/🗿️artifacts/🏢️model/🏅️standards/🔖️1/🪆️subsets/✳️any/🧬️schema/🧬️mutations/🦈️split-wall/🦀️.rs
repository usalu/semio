//! 🦈️ `split-wall` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️splits-a-straight-wall/🦀️.rs"]
mod tests_splits_a_straight_wall;
#[cfg(test)]
#[path = "🧪️tests/🌀️splits-an-arc/🦀️.rs"]
mod tests_splits_an_arc;
#[cfg(test)]
#[path = "🧪️tests/🪝️hands-openings-over/🦀️.rs"]
mod tests_hands_openings_over;
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
#[path = "🧪️tests/🔗️follows-by-inference/🦀️.rs"]
mod tests_follows_by_inference;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧮️repeats-the-sweeps-on-the-new-wall/🦀️.rs"]
mod tests_repeats_the_sweeps_on_the_new_wall;
#[cfg(test)]
#[path = "🧪️tests/🧯️sweep-copy-id-taken/🦀️.rs"]
mod tests_sweep_copy_id_taken;
//#endregion 🔖️Leaves
