//! 🧰️ `create-property-template` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds/🦀️.rs"]
mod tests_adds;
#[cfg(test)]
#[path = "🧪️tests/🚫️adds-the-first-template/🦀️.rs"]
mod tests_adds_the_first_template;
#[cfg(test)]
#[path = "🧪️tests/⛔️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/🛑️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧭️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/💤️name-already-defined/🦀️.rs"]
mod tests_name_already_defined;
#[cfg(test)]
#[path = "🧪️tests/📝️kind-listed-twice/🦀️.rs"]
mod tests_kind_listed_twice;
#[cfg(test)]
#[path = "🧪️tests/🎯️property-defined-twice/🦀️.rs"]
mod tests_property_defined_twice;
#[cfg(test)]
#[path = "🧪️tests/📉️default-breaks-the-range/🦀️.rs"]
mod tests_default_breaks_the_range;
#[cfg(test)]
#[path = "🧪️tests/🗂️default-of-another-kind/🦀️.rs"]
mod tests_default_of_another_kind;
#[cfg(test)]
#[path = "🧪️tests/📌️range-on-text/🦀️.rs"]
mod tests_range_on_text;
//#endregion 🔖️Leaves
