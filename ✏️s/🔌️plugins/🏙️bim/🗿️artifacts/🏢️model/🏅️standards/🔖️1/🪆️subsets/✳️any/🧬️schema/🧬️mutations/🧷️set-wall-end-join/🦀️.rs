//! 🧷️ `set-wall-end-join` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️butts-the-start/🦀️.rs"]
mod tests_butts_the_start;
#[cfg(test)]
#[path = "🧪️tests/🌀️miters-the-end/🦀️.rs"]
mod tests_miters_the_end;
#[cfg(test)]
#[path = "🧪️tests/🌟️frees-the-end/🦀️.rs"]
mod tests_frees_the_end;
#[cfg(test)]
#[path = "🧪️tests/🚛️back-to-automatic/🦀️.rs"]
mod tests_back_to_automatic;
#[cfg(test)]
#[path = "🧪️tests/🧲️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🕳️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves
