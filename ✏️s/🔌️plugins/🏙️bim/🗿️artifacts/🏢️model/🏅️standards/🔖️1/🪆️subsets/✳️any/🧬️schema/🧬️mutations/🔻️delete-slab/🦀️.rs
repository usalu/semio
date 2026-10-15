//! 🔻️ `delete-slab` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️removes/🦀️.rs"]
mod tests_removes;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️removes-its-data/🦀️.rs"]
mod tests_removes_its_data;
#[cfg(test)]
#[path = "🧪️tests/🧮️takes-its-hosted-railing-with-it/🦀️.rs"]
mod tests_takes_its_hosted_railing_with_it;
#[cfg(test)]
#[path = "🧪️tests/🧯️attached-by-the-top-of-a-wall/🦀️.rs"]
mod tests_attached_by_the_top_of_a_wall;
#[cfg(test)]
#[path = "🧪️tests/🧰️attached-by-the-base-of-a-wall/🦀️.rs"]
mod tests_attached_by_the_base_of_a_wall;
//#endregion 🔖️Leaves
