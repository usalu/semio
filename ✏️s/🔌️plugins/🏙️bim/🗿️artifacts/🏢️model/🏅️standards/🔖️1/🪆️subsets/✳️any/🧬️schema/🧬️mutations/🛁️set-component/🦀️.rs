//! 🛁️ `set-component` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves/🦀️.rs"]
mod tests_moves;
#[cfg(test)]
#[path = "🧪️tests/➕️raises/🦀️.rs"]
mod tests_raises;
#[cfg(test)]
#[path = "🧪️tests/✨️turns/🦀️.rs"]
mod tests_turns;
#[cfg(test)]
#[path = "🧪️tests/👍️mirrors/🦀️.rs"]
mod tests_mirrors;
#[cfg(test)]
#[path = "🧪️tests/🧲️renames/🦀️.rs"]
mod tests_renames;
#[cfg(test)]
#[path = "🧪️tests/💪️mounts-on-a-wall/🦀️.rs"]
mod tests_mounts_on_a_wall;
#[cfg(test)]
#[path = "🧪️tests/🎉️unmounts/🦀️.rs"]
mod tests_unmounts;
#[cfg(test)]
#[path = "🧪️tests/🔥️becomes-a-terminal/🦀️.rs"]
mod tests_becomes_a_terminal;
#[cfg(test)]
#[path = "🧪️tests/🚀️clears-the-system/🦀️.rs"]
mod tests_clears_the_system;
#[cfg(test)]
#[path = "🧪️tests/🌟️swaps-the-family/🦀️.rs"]
mod tests_swaps_the_family;
#[cfg(test)]
#[path = "🧪️tests/🌈️moves-to-another-storey/🦀️.rs"]
mod tests_moves_to_another_storey;
#[cfg(test)]
#[path = "🧪️tests/🍀️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️mounted-keeps-its-storey/🦀️.rs"]
mod tests_mounted_keeps_its_storey;
#[cfg(test)]
#[path = "🧪️tests/🛑️host-missing/🦀️.rs"]
mod tests_host_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️host-is-no-wall/🦀️.rs"]
mod tests_host_is_no_wall;
#[cfg(test)]
#[path = "🧪️tests/🙅️host-on-another-storey/🦀️.rs"]
mod tests_host_on_another_storey;
#[cfg(test)]
#[path = "🧪️tests/📛️family-missing/🦀️.rs"]
mod tests_family_missing;
#[cfg(test)]
#[path = "🧪️tests/🚧️profile-family/🦀️.rs"]
mod tests_profile_family;
#[cfg(test)]
#[path = "🧪️tests/🧯️override-needs-its-parameter/🦀️.rs"]
mod tests_override_needs_its_parameter;
#[cfg(test)]
#[path = "🧪️tests/❗️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/📣️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
