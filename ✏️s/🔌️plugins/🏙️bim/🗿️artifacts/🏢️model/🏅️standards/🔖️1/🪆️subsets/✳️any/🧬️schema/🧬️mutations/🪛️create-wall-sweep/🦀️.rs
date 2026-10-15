//! 🪛️ `create-wall-sweep` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-baseboard/🦀️.rs"]
mod tests_adds_a_baseboard;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-an-embedded-rail/🦀️.rs"]
mod tests_adds_an_embedded_rail;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️host-missing/🦀️.rs"]
mod tests_host_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️profile-degenerate/🦀️.rs"]
mod tests_profile_degenerate;
#[cfg(test)]
#[path = "🧪️tests/🙅️negative-height/🦀️.rs"]
mod tests_negative_height;
#[cfg(test)]
#[path = "🧪️tests/📛️inset-swallows-the-profile/🦀️.rs"]
mod tests_inset_swallows_the_profile;
//#endregion 🔖️Leaves
