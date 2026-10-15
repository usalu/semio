//! 🚿️ `set-component-override` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-an-override/🦀️.rs"]
mod tests_adds_an_override;
#[cfg(test)]
#[path = "🧪️tests/➕️replaces-an-override/🦀️.rs"]
mod tests_replaces_an_override;
#[cfg(test)]
#[path = "🧪️tests/✨️uses-another-parameter/🦀️.rs"]
mod tests_uses_another_parameter;
#[cfg(test)]
#[path = "🧪️tests/👍️breaks-a-family-circle-by-override/🦀️.rs"]
mod tests_breaks_a_family_circle_by_override;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️component-missing/🦀️.rs"]
mod tests_component_missing;
#[cfg(test)]
#[path = "🧪️tests/❌️parameter-missing/🦀️.rs"]
mod tests_parameter_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️formula-does-not-parse/🦀️.rs"]
mod tests_formula_does_not_parse;
#[cfg(test)]
#[path = "🧪️tests/🚷️unknown-parameter/🦀️.rs"]
mod tests_unknown_parameter;
#[cfg(test)]
#[path = "🧪️tests/🙅️closes-a-circle/🦀️.rs"]
mod tests_closes_a_circle;
#[cfg(test)]
#[path = "🧪️tests/📛️closes-a-circle-by-override/🦀️.rs"]
mod tests_closes_a_circle_by_override;
#[cfg(test)]
#[path = "🧪️tests/🚧️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
//#endregion 🔖️Leaves
