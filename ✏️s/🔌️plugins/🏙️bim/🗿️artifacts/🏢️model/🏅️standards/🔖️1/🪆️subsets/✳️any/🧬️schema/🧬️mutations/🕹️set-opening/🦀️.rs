//! 🕹️ `set-opening` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️resizes/🦀️.rs"]
mod tests_resizes;
#[cfg(test)]
#[path = "🧪️tests/➕️clears-the-width-override/🦀️.rs"]
mod tests_clears_the_width_override;
#[cfg(test)]
#[path = "🧪️tests/🟢️retypes-and-flips/🦀️.rs"]
mod tests_retypes_and_flips;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/⛔️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️negative-sill/🦀️.rs"]
mod tests_negative_sill;
#[cfg(test)]
#[path = "🧪️tests/❌️non-positive-height/🦀️.rs"]
mod tests_non_positive_height;
#[cfg(test)]
#[path = "🧪️tests/❗️too-wide-for-the-host/🦀️.rs"]
mod tests_too_wide_for_the_host;
#[cfg(test)]
#[path = "🧪️tests/📛️overlaps-a-neighbour/🦀️.rs"]
mod tests_overlaps_a_neighbour;
#[cfg(test)]
#[path = "🧪️tests/🔴️already-set/🦀️.rs"]
mod tests_already_set;
#[cfg(test)]
#[path = "🧪️tests/🧭️sets-a-sill-override/🦀️.rs"]
mod tests_sets_a_sill_override;
#[cfg(test)]
#[path = "🧪️tests/🧮️restores-the-type-sill/🦀️.rs"]
mod tests_restores_the_type_sill;
#[cfg(test)]
#[path = "🧪️tests/🧯️sets-a-reveal/🦀️.rs"]
mod tests_sets_a_reveal;
#[cfg(test)]
#[path = "🧪️tests/🧰️clears-the-reveal/🦀️.rs"]
mod tests_clears_the_reveal;
#[cfg(test)]
#[path = "🧪️tests/🧲️negative-reveal-depth/🦀️.rs"]
mod tests_negative_reveal_depth;
#[cfg(test)]
#[path = "🧪️tests/🧳️reveal-material-missing/🦀️.rs"]
mod tests_reveal_material_missing;
//#endregion 🔖️Leaves
