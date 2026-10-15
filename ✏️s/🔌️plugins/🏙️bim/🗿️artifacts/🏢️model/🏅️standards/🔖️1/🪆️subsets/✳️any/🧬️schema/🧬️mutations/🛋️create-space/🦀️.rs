//! 🛋️ `create-space` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-bounded-space/🦀️.rs"]
mod tests_adds_a_bounded_space;
#[cfg(test)]
#[path = "🧪️tests/📐️adds-an-explicit-space/🦀️.rs"]
mod tests_adds_an_explicit_space;
#[cfg(test)]
#[path = "🧪️tests/🔁️reuses-a-number-on-another-storey/🦀️.rs"]
mod tests_reuses_a_number_on_another_storey;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🔢️number-taken/🦀️.rs"]
mod tests_number_taken;
#[cfg(test)]
#[path = "🧪️tests/💥️outline-degenerate/🦀️.rs"]
mod tests_outline_degenerate;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🎨️adds-a-finished-space/🦀️.rs"]
mod tests_adds_a_finished_space;
#[cfg(test)]
#[path = "🧪️tests/🏘️zone-missing/🦀️.rs"]
mod tests_zone_missing;
#[cfg(test)]
#[path = "🧪️tests/🧱️finish-material-missing/🦀️.rs"]
mod tests_finish_material_missing;
//#endregion 🔖️Leaves
