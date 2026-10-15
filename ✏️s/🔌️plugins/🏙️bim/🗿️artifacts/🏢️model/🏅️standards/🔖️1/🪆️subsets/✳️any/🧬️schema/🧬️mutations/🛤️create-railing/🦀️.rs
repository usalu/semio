//! 🛤️ `create-railing` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧱️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/📏️path-too-short/🦀️.rs"]
mod tests_path_too_short;
#[cfg(test)]
#[path = "🧪️tests/💥️non-positive-height/🦀️.rs"]
mod tests_non_positive_height;
#[cfg(test)]
#[path = "🧪️tests/🚧️non-positive-post-spacing/🦀️.rs"]
mod tests_non_positive_post_spacing;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧮️adds-balusters-and-a-glass-infill/🦀️.rs"]
mod tests_adds_balusters_and_a_glass_infill;
#[cfg(test)]
#[path = "🧪️tests/🧯️baluster-spacing-not-positive/🦀️.rs"]
mod tests_baluster_spacing_not_positive;
#[cfg(test)]
#[path = "🧪️tests/🧰️infill-thickness-not-positive/🦀️.rs"]
mod tests_infill_thickness_not_positive;
#[cfg(test)]
#[path = "🧪️tests/🧲️rail-profile-degenerate/🦀️.rs"]
mod tests_rail_profile_degenerate;
#[cfg(test)]
#[path = "🧪️tests/🧳️adds-a-railing-hosted-by-a-ramp/🦀️.rs"]
mod tests_adds_a_railing_hosted_by_a_ramp;
#[cfg(test)]
#[path = "🧪️tests/🧴️adds-a-railing-hosted-by-a-stair/🦀️.rs"]
mod tests_adds_a_railing_hosted_by_a_stair;
#[cfg(test)]
#[path = "🧪️tests/🧶️host-missing/🦀️.rs"]
mod tests_host_missing;
#[cfg(test)]
#[path = "🧪️tests/🧷️hosted-railing-with-a-path/🦀️.rs"]
mod tests_hosted_railing_with_a_path;
#[cfg(test)]
#[path = "🧪️tests/🧸️negative-host-inset/🦀️.rs"]
mod tests_negative_host_inset;
#[cfg(test)]
#[path = "🧪️tests/🧺️slab-edge-missing/🦀️.rs"]
mod tests_slab_edge_missing;
//#endregion 🔖️Leaves
