//! 📄️ `create-sheet` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-sheet/🦀️.rs"]
mod tests_adds_a_sheet;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-a-portrait-a1/🦀️.rs"]
mod tests_adds_a_portrait_a1;
#[cfg(test)]
#[path = "🧪️tests/🧵️adds-a-custom-paper/🦀️.rs"]
mod tests_adds_a_custom_paper;
#[cfg(test)]
#[path = "🧪️tests/🧴️adds-a-filled-title-block/🦀️.rs"]
mod tests_adds_a_filled_title_block;
#[cfg(test)]
#[path = "🧪️tests/🧳️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/🧱️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧰️number-taken/🦀️.rs"]
mod tests_number_taken;
#[cfg(test)]
#[path = "🧪️tests/🧯️blank-number/🦀️.rs"]
mod tests_blank_number;
#[cfg(test)]
#[path = "🧪️tests/🧮️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🧭️custom-paper-too-small/🦀️.rs"]
mod tests_custom_paper_too_small;
#[cfg(test)]
#[path = "🧪️tests/🧩️custom-paper-too-large/🦀️.rs"]
mod tests_custom_paper_too_large;
#[cfg(test)]
#[path = "🧪️tests/🧨️date-unreadable/🦀️.rs"]
mod tests_date_unreadable;
//#endregion 🔖️Leaves
