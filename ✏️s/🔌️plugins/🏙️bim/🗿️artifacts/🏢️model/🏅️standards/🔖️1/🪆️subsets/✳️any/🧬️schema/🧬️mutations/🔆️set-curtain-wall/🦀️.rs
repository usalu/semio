//! 🔆️ `set-curtain-wall` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves-the-facade/🦀️.rs"]
mod tests_moves_the_facade;
#[cfg(test)]
#[path = "🧪️tests/➕️curves-the-facade/🦀️.rs"]
mod tests_curves_the_facade;
#[cfg(test)]
#[path = "🧪️tests/✨️constrains-the-top/🦀️.rs"]
mod tests_constrains_the_top;
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
#[path = "🧪️tests/❌️zero-length/🦀️.rs"]
mod tests_zero_length;
#[cfg(test)]
#[path = "🧪️tests/🛑️top-storey-missing/🦀️.rs"]
mod tests_top_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
