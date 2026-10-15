//! 🧮️ `set-stair` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves-and-widens/🦀️.rs"]
mod tests_moves_and_widens;
#[cfg(test)]
#[path = "🧪️tests/➕️turns-into-an-l-run/🦀️.rs"]
mod tests_turns_into_an_l_run;
#[cfg(test)]
#[path = "🧪️tests/🟢️renames-and-flattens/🦀️.rs"]
mod tests_renames_and_flattens;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/⛔️non-positive-width/🦀️.rs"]
mod tests_non_positive_width;
#[cfg(test)]
#[path = "🧪️tests/🛑️invalid-riser/🦀️.rs"]
mod tests_invalid_riser;
#[cfg(test)]
#[path = "🧪️tests/❌️top-storey-missing/🦀️.rs"]
mod tests_top_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/❗️already-set/🦀️.rs"]
mod tests_already_set;
#[cfg(test)]
#[path = "🧪️tests/🧭️builds-open-risers-and-a-mono-stringer/🦀️.rs"]
mod tests_builds_open_risers_and_a_mono_stringer;
#[cfg(test)]
#[path = "🧪️tests/🧮️nosing-longer-than-the-tread/🦀️.rs"]
mod tests_nosing_longer_than_the_tread;
#[cfg(test)]
#[path = "🧪️tests/🧯️stringer-without-size/🦀️.rs"]
mod tests_stringer_without_size;
//#endregion 🔖️Leaves
