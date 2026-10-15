//! 📦️ `set-family-solid` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️hides-the-solid/🦀️.rs"]
mod tests_hides_the_solid;
#[cfg(test)]
#[path = "🧪️tests/➕️reshapes/🦀️.rs"]
mod tests_reshapes;
#[cfg(test)]
#[path = "🧪️tests/✨️moves/🦀️.rs"]
mod tests_moves;
#[cfg(test)]
#[path = "🧪️tests/👍️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️formula-does-not-parse/🦀️.rs"]
mod tests_formula_does_not_parse;
#[cfg(test)]
#[path = "🧪️tests/🛑️unknown-parameter/🦀️.rs"]
mod tests_unknown_parameter;
#[cfg(test)]
#[path = "🧪️tests/🚷️polygon-too-small/🦀️.rs"]
mod tests_polygon_too_small;
#[cfg(test)]
#[path = "🧪️tests/🙅️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/📛️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
