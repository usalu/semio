//! 🏁️ `set-issue` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️retitles/🦀️.rs"]
mod tests_retitles;
#[cfg(test)]
#[path = "🧪️tests/🧲️assigns/🦀️.rs"]
mod tests_assigns;
#[cfg(test)]
#[path = "🧪️tests/🧵️resolves/🦀️.rs"]
mod tests_resolves;
#[cfg(test)]
#[path = "🧪️tests/🧴️raises-the-priority/🦀️.rs"]
mod tests_raises_the_priority;
#[cfg(test)]
#[path = "🧪️tests/🧳️relabels/🦀️.rs"]
mod tests_relabels;
#[cfg(test)]
#[path = "🧪️tests/🧱️adds-an-element/🦀️.rs"]
mod tests_adds_an_element;
#[cfg(test)]
#[path = "🧪️tests/🧰️sets-the-viewpoint/🦀️.rs"]
mod tests_sets_the_viewpoint;
#[cfg(test)]
#[path = "🧪️tests/🧯️clears-the-viewpoint/🦀️.rs"]
mod tests_clears_the_viewpoint;
#[cfg(test)]
#[path = "🧪️tests/🧮️links-a-clash/🦀️.rs"]
mod tests_links_a_clash;
#[cfg(test)]
#[path = "🧪️tests/🧭️clears-the-clash/🦀️.rs"]
mod tests_clears_the_clash;
#[cfg(test)]
#[path = "🧪️tests/🧩️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧨️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧧️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧦️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧥️blank-title/🦀️.rs"]
mod tests_blank_title;
#[cfg(test)]
#[path = "🧪️tests/🧤️unreadable-date/🦀️.rs"]
mod tests_unreadable_date;
#[cfg(test)]
#[path = "🧪️tests/🧣️element-missing/🦀️.rs"]
mod tests_element_missing;
#[cfg(test)]
#[path = "🧪️tests/🧢️clash-set-missing/🦀️.rs"]
mod tests_clash_set_missing;
//#endregion 🔖️Leaves
