//! 👯️ `copy-elements` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️copies-a-wall-with-its-openings/🦀️.rs"]
mod tests_copies_a_wall_with_its_openings;
#[cfg(test)]
#[path = "🧪️tests/🚛️copies-every-placed-kind/🦀️.rs"]
mod tests_copies_every_placed_kind;
#[cfg(test)]
#[path = "🧪️tests/🧲️copies-in-place/🦀️.rs"]
mod tests_copies_in_place;
#[cfg(test)]
#[path = "🧪️tests/⛔️empty-selection/🦀️.rs"]
mod tests_empty_selection;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
#[cfg(test)]
#[path = "🧪️tests/🚧️opening-alone/🦀️.rs"]
mod tests_opening_alone;
#[cfg(test)]
#[path = "🧪️tests/🪝️storey-has-no-placement/🦀️.rs"]
mod tests_storey_has_no_placement;
#[cfg(test)]
#[path = "🧪️tests/🛑️blank-prefix/🦀️.rs"]
mod tests_blank_prefix;
#[cfg(test)]
#[path = "🧪️tests/🧩️minted-id-taken/🦀️.rs"]
mod tests_minted_id_taken;
#[cfg(test)]
#[path = "🧪️tests/🧭️copies-a-wall-with-its-sweeps/🦀️.rs"]
mod tests_copies_a_wall_with_its_sweeps;
//#endregion 🔖️Leaves
