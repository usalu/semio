//! 🔶️ `create-family-solid` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️extrudes-a-profile/🦀️.rs"]
mod tests_extrudes_a_profile;
#[cfg(test)]
#[path = "🧪️tests/➕️places-a-cuboid/🦀️.rs"]
mod tests_places_a_cuboid;
#[cfg(test)]
#[path = "🧪️tests/✨️sweeps-a-rail/🦀️.rs"]
mod tests_sweeps_a_rail;
#[cfg(test)]
#[path = "🧪️tests/👍️revolves-a-foot/🦀️.rs"]
mod tests_revolves_a_foot;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️family-missing/🦀️.rs"]
mod tests_family_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🚷️formula-does-not-parse/🦀️.rs"]
mod tests_formula_does_not_parse;
#[cfg(test)]
#[path = "🧪️tests/🙅️unknown-parameter/🦀️.rs"]
mod tests_unknown_parameter;
#[cfg(test)]
#[path = "🧪️tests/📛️polygon-too-small/🦀️.rs"]
mod tests_polygon_too_small;
//#endregion 🔖️Leaves
