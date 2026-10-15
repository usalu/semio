//! 🪠️ `set-wall-sweep` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️lifts-and-resizes/🦀️.rs"]
mod tests_lifts_and_resizes;
#[cfg(test)]
#[path = "🧪️tests/➕️moves-to-the-other-face/🦀️.rs"]
mod tests_moves_to_the_other_face;
#[cfg(test)]
#[path = "🧪️tests/✨️rehosts/🦀️.rs"]
mod tests_rehosts;
#[cfg(test)]
#[path = "🧪️tests/👍️renames/🦀️.rs"]
mod tests_renames;
#[cfg(test)]
#[path = "🧪️tests/🧲️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
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
#[cfg(test)]
#[path = "🧪️tests/🚧️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
