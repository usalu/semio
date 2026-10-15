//! 🚮️ `delete-storey` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/🧕️cascades-clash-sets-and-rules-scoped-to-it/🦀️.rs"]
mod tests_cascades_clash_sets_and_rules_scoped_to_it;
#[cfg(test)]
#[path = "🧪️tests/✅️cascades-the-walls/🦀️.rs"]
mod tests_cascades_the_walls;
#[cfg(test)]
#[path = "🧪️tests/🧲️empty-storey/🦀️.rs"]
mod tests_empty_storey;
#[cfg(test)]
#[path = "🧪️tests/🚫️constrains-another-wall/🦀️.rs"]
mod tests_constrains_another_wall;
#[cfg(test)]
#[path = "🧪️tests/🌊️cascades-the-opening/🦀️.rs"]
mod tests_cascades_the_opening;
#[cfg(test)]
#[path = "🧪️tests/🏗️cascades-everything-on-it/🦀️.rs"]
mod tests_cascades_everything_on_it;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/📋️cascades-the-schedules/🦀️.rs"]
mod tests_cascades_the_schedules;
//#endregion 🔖️Leaves
