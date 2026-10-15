//! 🧗️ `create-stair` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-straight-flight/🦀️.rs"]
mod tests_adds_a_straight_flight;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-a-u-run-to-the-first-storey/🦀️.rs"]
mod tests_adds_a_u_run_to_the_first_storey;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️non-positive-width/🦀️.rs"]
mod tests_non_positive_width;
#[cfg(test)]
#[path = "🧪️tests/❌️invalid-riser/🦀️.rs"]
mod tests_invalid_riser;
#[cfg(test)]
#[path = "🧪️tests/❗️invalid-tread/🦀️.rs"]
mod tests_invalid_tread;
#[cfg(test)]
#[path = "🧪️tests/📛️broken-flight/🦀️.rs"]
mod tests_broken_flight;
#[cfg(test)]
#[path = "🧪️tests/🔴️top-storey-missing/🦀️.rs"]
mod tests_top_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧮️adds-an-open-stair-with-cut-stringers/🦀️.rs"]
mod tests_adds_an_open_stair_with_cut_stringers;
#[cfg(test)]
#[path = "🧪️tests/🧯️stringer-without-size/🦀️.rs"]
mod tests_stringer_without_size;
#[cfg(test)]
#[path = "🧪️tests/🧰️nosing-longer-than-the-tread/🦀️.rs"]
mod tests_nosing_longer_than_the_tread;
#[cfg(test)]
#[path = "🧪️tests/🧲️non-positive-tread-thickness/🦀️.rs"]
mod tests_non_positive_tread_thickness;
#[cfg(test)]
#[path = "🧪️tests/🧳️non-positive-landing-depth/🦀️.rs"]
mod tests_non_positive_landing_depth;
//#endregion 🔖️Leaves
