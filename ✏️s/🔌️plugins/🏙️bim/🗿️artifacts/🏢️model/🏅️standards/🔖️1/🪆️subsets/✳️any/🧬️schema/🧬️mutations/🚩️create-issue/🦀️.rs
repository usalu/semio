//! 🚩️ `create-issue` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️raises-an-issue/🦀️.rs"]
mod tests_raises_an_issue;
#[cfg(test)]
#[path = "🧪️tests/🧲️raises-one-from-a-clash/🦀️.rs"]
mod tests_raises_one_from_a_clash;
#[cfg(test)]
#[path = "🧪️tests/🧵️raises-one-with-a-section-box/🦀️.rs"]
mod tests_raises_one_with_a_section_box;
#[cfg(test)]
#[path = "🧪️tests/🧴️raises-a-bare-issue/🦀️.rs"]
mod tests_raises_a_bare_issue;
#[cfg(test)]
#[path = "🧪️tests/🧳️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/🧱️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧰️blank-title/🦀️.rs"]
mod tests_blank_title;
#[cfg(test)]
#[path = "🧪️tests/🧯️blank-author/🦀️.rs"]
mod tests_blank_author;
#[cfg(test)]
#[path = "🧪️tests/🧮️unreadable-date/🦀️.rs"]
mod tests_unreadable_date;
#[cfg(test)]
#[path = "🧪️tests/🧭️label-twice/🦀️.rs"]
mod tests_label_twice;
#[cfg(test)]
#[path = "🧪️tests/🧩️element-missing/🦀️.rs"]
mod tests_element_missing;
#[cfg(test)]
#[path = "🧪️tests/🧨️clash-set-missing/🦀️.rs"]
mod tests_clash_set_missing;
#[cfg(test)]
#[path = "🧪️tests/🧧️clash-with-itself/🦀️.rs"]
mod tests_clash_with_itself;
#[cfg(test)]
#[path = "🧪️tests/🧦️camera-without-distance/🦀️.rs"]
mod tests_camera_without_distance;
#[cfg(test)]
#[path = "🧪️tests/🧥️inverted-section-box/🦀️.rs"]
mod tests_inverted_section_box;
#[cfg(test)]
#[path = "🧪️tests/🧤️isolated-element-missing/🦀️.rs"]
mod tests_isolated_element_missing;
//#endregion 🔖️Leaves
