//! 🔨️ `set-wall-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🧲️restacks/🦀️.rs"]
mod tests_restacks;
#[cfg(test)]
#[path = "🧪️tests/🧩️both-fields/🦀️.rs"]
mod tests_both_fields;
#[cfg(test)]
#[path = "🧪️tests/📭️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🕳️empty-layers/🦀️.rs"]
mod tests_empty_layers;
#[cfg(test)]
#[path = "🧪️tests/📉️thin-layer/🦀️.rs"]
mod tests_thin_layer;
#[cfg(test)]
#[path = "🧪️tests/👻️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
//#endregion 🔖️Leaves
