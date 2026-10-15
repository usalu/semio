//! 📑️ `set-sheet` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames/🦀️.rs"]
mod tests_renames;
#[cfg(test)]
#[path = "🧪️tests/🧲️renumbers/🦀️.rs"]
mod tests_renumbers;
#[cfg(test)]
#[path = "🧪️tests/🧵️resizes-the-paper/🦀️.rs"]
mod tests_resizes_the_paper;
#[cfg(test)]
#[path = "🧪️tests/🧴️turns-the-sheet/🦀️.rs"]
mod tests_turns_the_sheet;
#[cfg(test)]
#[path = "🧪️tests/🧳️cuts-a-custom-paper/🦀️.rs"]
mod tests_cuts_a_custom_paper;
#[cfg(test)]
#[path = "🧪️tests/🧱️fills-the-title-block/🦀️.rs"]
mod tests_fills_the_title_block;
#[cfg(test)]
#[path = "🧪️tests/🧰️marks-the-revision-and-scale/🦀️.rs"]
mod tests_marks_the_revision_and_scale;
#[cfg(test)]
#[path = "🧪️tests/🧯️clears-the-title-block/🦀️.rs"]
mod tests_clears_the_title_block;
#[cfg(test)]
#[path = "🧪️tests/🧮️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧭️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧩️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧨️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧧️number-taken/🦀️.rs"]
mod tests_number_taken;
#[cfg(test)]
#[path = "🧪️tests/🧦️blank-number/🦀️.rs"]
mod tests_blank_number;
#[cfg(test)]
#[path = "🧪️tests/🧥️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🧤️custom-paper-too-small/🦀️.rs"]
mod tests_custom_paper_too_small;
#[cfg(test)]
#[path = "🧪️tests/🧣️date-unreadable/🦀️.rs"]
mod tests_date_unreadable;
//#endregion 🔖️Leaves
