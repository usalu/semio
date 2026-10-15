//! 🖊️ `set-annotation-style` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️scales-the-text/🦀️.rs"]
mod tests_scales_the_text;
#[cfg(test)]
#[path = "🧪️tests/➕️prints-centimetres/🦀️.rs"]
mod tests_prints_centimetres;
#[cfg(test)]
#[path = "🧪️tests/✨️renames/🦀️.rs"]
mod tests_renames;
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
#[path = "🧪️tests/❌️text-height-not-positive/🦀️.rs"]
mod tests_text_height_not_positive;
#[cfg(test)]
#[path = "🧪️tests/🛑️too-many-decimals/🦀️.rs"]
mod tests_too_many_decimals;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
