//! 🗺️ `set-site` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves-the-site/🦀️.rs"]
mod tests_moves_the_site;
#[cfg(test)]
#[path = "🧪️tests/🧲️outlines-the-boundary/🦀️.rs"]
mod tests_outlines_the_boundary;
#[cfg(test)]
#[path = "🧪️tests/🧵️clears-the-boundary/🦀️.rs"]
mod tests_clears_the_boundary;
#[cfg(test)]
#[path = "🧪️tests/🚫️latitude-out-of-range/🦀️.rs"]
mod tests_latitude_out_of_range;
#[cfg(test)]
#[path = "🧪️tests/💥️longitude-out-of-range/🦀️.rs"]
mod tests_longitude_out_of_range;
#[cfg(test)]
#[path = "🧪️tests/🔒️open-boundary/🦀️.rs"]
mod tests_open_boundary;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🔁️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🪜️raises-the-absolute-levels/🦀️.rs"]
mod tests_raises_the_absolute_levels;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
//#endregion 🔖️Leaves
