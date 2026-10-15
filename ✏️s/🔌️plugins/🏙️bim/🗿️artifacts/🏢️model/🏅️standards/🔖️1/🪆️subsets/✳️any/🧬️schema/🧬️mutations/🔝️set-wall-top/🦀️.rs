//! 🔝️ `set-wall-top` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️constrains-to-the-first-storey/🦀️.rs"]
mod tests_constrains_to_the_first_storey;
#[cfg(test)]
#[path = "🧪️tests/🧲️frees-the-height/🦀️.rs"]
mod tests_frees_the_height;
#[cfg(test)]
#[path = "🧪️tests/🚫️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️attaches-to-a-roof/🦀️.rs"]
mod tests_attaches_to_a_roof;
#[cfg(test)]
#[path = "🧪️tests/🧮️attaches-to-a-slab/🦀️.rs"]
mod tests_attaches_to_a_slab;
#[cfg(test)]
#[path = "🧪️tests/🧯️roof-missing/🦀️.rs"]
mod tests_roof_missing;
#[cfg(test)]
#[path = "🧪️tests/🧰️ceiling-missing/🦀️.rs"]
mod tests_ceiling_missing;
#[cfg(test)]
#[path = "🧪️tests/🧳️frees-an-attached-top/🦀️.rs"]
mod tests_frees_an_attached_top;
#[cfg(test)]
#[path = "🧪️tests/🧴️roof-in-another-building/🦀️.rs"]
mod tests_roof_in_another_building;
//#endregion 🔖️Leaves
